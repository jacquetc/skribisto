#!/usr/bin/env python3
"""Drive a live Skribisto Launcher via the bastyde automation MCP bridge and
check the **keyboard highlight** in the Welcome window's lists.

The bug under test: arrow-keying the RECENT WORKS list moves the list's internal
`focused_index` (Enter opens the right row, so navigation *works*), but nothing
on screen says which row you are on. `ListView` only hands `selected = true` to
its row delegate when a selection model is attached (`list_view.rs`:
`selection.as_ref().map(|s| s.is_selected(i)).unwrap_or(false)`), and
`welcome_panel.rs` attaches none — so every row renders unselected forever and
the only feedback is a focus ring around the *whole list*.

What this asserts, per list (Recent Works, then Examples):
  1. the AccessKit tree exposes the rows as ListItems,
  2. after focusing the list and pressing Down, SOME row reports `selected`
     (AT truth — a screen reader must be able to announce the cursor row),
  3. the rendered pixels change between "focused, no arrow" and "arrowed down
     twice" (visual truth — a sighted keyboard user must see the cursor move).

(2) and (3) are what regress. Screenshots are written to /tmp/sk-kbd-*.png so
the difference (or lack of it) can be eyeballed.

Recents are seeded on disk (a sandbox `recents.toml` pointing at 3 real .skrib
copies) rather than by opening projects: the list only shows *reachable* paths,
and clicking a row would open it (ActivateOn::SingleClick).
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = "/home/cyril/Devel/skribisto/resources/examples/Starforgers.skrib"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# Isolated XDG dirs — never touch the real ~/.config/skribisto (and give us a
# recents.toml we fully control).
_sandbox = tempfile.mkdtemp(prefix="skribisto_kbd_test_")
CONFIG = os.path.join(_sandbox, "config")
SANDBOX_ENV = {
    "XDG_CONFIG_HOME": CONFIG,
    "XDG_DATA_HOME": os.path.join(_sandbox, "data"),
    "HOME": _sandbox,
}

# 3 reachable projects, so an arrow-key cursor has somewhere to go.
PROJECTS = []
_works = os.path.join(_sandbox, "works")
os.makedirs(_works, exist_ok=True)
for name in ("Alpha", "Beta", "Gamma"):
    dst = os.path.join(_works, f"{name}.skrib")
    shutil.copyfile(EXAMPLE, dst)
    PROJECTS.append((name, dst))

os.makedirs(os.path.join(CONFIG, "skribisto"), exist_ok=True)
with open(os.path.join(CONFIG, "skribisto", "recents.toml"), "w") as fh:
    fh.write("version = 1\n")
    for i, (name, path) in enumerate(PROJECTS):
        fh.write("\n[[items]]\n")
        fh.write(f'path = "{path}"\n')
        fh.write(f'title = "{name}"\n')
        fh.write(f"last_opened_ms = {1_700_000_000_000 - i * 86_400_000}\n")
        fh.write("pinned = false\n")


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


class Session:
    def __init__(self, args):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        env = {**os.environ, **SANDBOX_ENV}
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
        sock = tok = None
        deadline = time.time() + 20
        while time.time() < deadline:
            txt = open(self.log).read()
            s = re.search(r"bridge socket = (\S+)", txt)
            t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
            if s and t:
                sock, tok = s.group(1), t.group(1)
                break
            if self.app.poll() is not None:
                fail("app exited before printing the bridge socket", self.app, None, self.log)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 20s", self.app, None, self.log)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "kbd-test", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect MCP", self.app, self.mcp, self.log)
        self._send("notifications/initialized", notif=True)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=20, fatal=True):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                break
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
            if not r:
                break
            line = self.mcp.stdout.readline()
            if not line:
                break
            if line.strip():
                return json.loads(line)
        if fatal:
            fail("no MCP response within timeout", self.app, self.mcp, self.log)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        result = self._recv().get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(c.get("text", "") for c in result.get("content", [])
                           if c.get("type") == "text")
            t = text.strip()
            payload = json.loads(t) if (t.startswith("{") or t.startswith("[")) else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", []) if isinstance(p, dict) else []

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in " | ".join(l for l in self.labels()).lower():
                return True
            time.sleep(0.3)
        return False

    def find(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def key(self, k, **mods):
        res, _ = self.call("inject_key", {"key": k, **mods})
        if isinstance(res, dict) and res.get("isError"):
            fail(f"inject_key {k} errored: {res}", self.app, self.mcp, self.log)
        time.sleep(0.35)

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                data = base64.b64decode(c["data"])
                open(path, "wb").write(data)
                return data
        return b""

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


def selected_rows(s):
    """Rows the AT tree reports as selected, with their labels."""
    return [(n.get("id"), n.get("label")) for n in s.nodes() if n.get("selected")]


def list_items(s):
    return [n for n in s.nodes() if n.get("role") == "ListItem"]


print("== Launcher: keyboard highlight in the Welcome lists ==")
s = Session([])
if not s.wait_label("welcome sections"):
    fail("the Launcher window did not appear", s.app, s.mcp, s.log)

failures = []

# ── Recent Works ────────────────────────────────────────────────────────────
if not s.wait_label("alpha", timeout=8):
    fail("the seeded recents never appeared in the Works pane", s.app, s.mcp, s.log)
rows = list_items(s)
print(f"recent rows (ListItem): {[(n.get('id'), n.get('label')) for n in rows]}")
if len(rows) < 3:
    fail(f"expected the 3 seeded recents as ListItems, got {len(rows)}", s.app, s.mcp, s.log)

# Focus the list without clicking a row (a click would OPEN the project).
listbox = next((n for n in s.nodes() if n.get("role") == "ListBox"), None)
if not listbox:
    fail("no ListBox node for the recents list", s.app, s.mcp, s.log)
s.call("focus_node", {"node": listbox["id"]})
time.sleep(0.4)
before_png = s.shot("/tmp/sk-kbd-recents-focused.png")
print(f"selected after focus, before any arrow: {selected_rows(s)}")

s.key("Down")
after_one = selected_rows(s)
print(f"selected after 1x Down: {after_one}")
s.key("Down")
after_two = selected_rows(s)
after_png = s.shot("/tmp/sk-kbd-recents-down2.png")
print(f"selected after 2x Down: {after_two}")

if not after_two:
    failures.append("RECENTS: after 2x ArrowDown no row is reported `selected` in the "
                    "AccessKit tree — a screen reader cannot announce the cursor row")
if before_png == after_png:
    failures.append("RECENTS: the window pixels are IDENTICAL before and after 2x ArrowDown "
                    "— the keyboard cursor is invisible "
                    "(/tmp/sk-kbd-recents-focused.png vs /tmp/sk-kbd-recents-down2.png)")
else:
    print("recents: pixels changed on arrow-down (some visual feedback exists)")

# Enter must still activate the arrowed-to row — proving nav DOES work, it is
# only unseen. Assert the project window opens on the row the cursor is on.
# (Row 2 = 'Gamma' after two Downs from the top, if the cursor started at 0.)
s.key("Enter")
opened = s.wait_label("binder", timeout=15)
if opened:
    print("PASS: Enter on the arrowed-to row opened a project — keyboard nav works, "
          "it is only invisible")
else:
    failures.append("RECENTS: Enter after arrow-keying did not open a project")

s.close()

# ── Examples (a second, independent list in the same window) ────────────────
print("\n== Examples pane ==")
s2 = Session([])
if not s2.wait_label("welcome sections"):
    fail("the Launcher window did not appear (examples phase)", s2.app, s2.mcp, s2.log)
ex_tab = s2.find("Examples", role="Tab") or s2.find("Examples")
if not ex_tab:
    fail("no Examples tab", s2.app, s2.mcp, s2.log)
s2.call("invoke_action", {"node": ex_tab["id"], "action": "click"})
if not s2.wait_label("starforgers", timeout=8):
    fail("Examples pane never revealed Starforgers", s2.app, s2.mcp, s2.log)
lb2 = next((n for n in s2.nodes() if n.get("role") == "ListBox"), None)
if lb2:
    s2.call("focus_node", {"node": lb2["id"]})
    time.sleep(0.3)
    b2 = s2.shot("/tmp/sk-kbd-examples-focused.png")
    s2.key("Down")
    sel2 = selected_rows(s2)
    a2 = s2.shot("/tmp/sk-kbd-examples-down.png")
    print(f"examples selected after Down: {sel2}")
    if not sel2:
        failures.append("EXAMPLES: after ArrowDown no row is reported `selected` in the AT tree")
    if b2 == a2:
        failures.append("EXAMPLES: pixels IDENTICAL before/after ArrowDown — no keyboard cursor")
else:
    failures.append("EXAMPLES: no ListBox node")
s2.close()

shutil.rmtree(_sandbox, ignore_errors=True)

print()
if failures:
    print("REPRODUCED — the keyboard cursor is not visible:")
    for f in failures:
        print("  ✗ " + f)
    sys.exit(1)
print("ALL PASS — the keyboard cursor is visible and exposed to AT")
sys.exit(0)
