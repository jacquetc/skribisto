#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto Launcher via the teksilo automation MCP bridge and
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

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

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
            t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
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


def list_items(s):
    # `ListView` wraps each row in a ListItemWrapper, surfaced as a
    # `ListBoxOption` under the view's `ListBox`; the row's own text is on a
    # `GenericContainer` child (that's where `access_label_literal` lands).
    return [n for n in s.nodes() if n.get("role") == "ListBoxOption"]


def selected_rows(s):
    """LIST ROWS the AT tree reports as selected.

    Scoped to `ListBoxOption` on purpose: the nav `TabBar` also publishes a
    `selected` tab ("Works"), and counting that as a hit turns this whole check
    into a false pass.
    """
    return [(n.get("id"), n.get("selected")) for n in list_items(s) if n.get("selected") is True]


def selected_index(s):
    """Row *position* of the cursor (rows come back in visual order), or None."""
    for i, n in enumerate(list_items(s)):
        if n.get("selected") is True:
            return i
    return None


failures = []

# ── Phase 0: Enter, straight off the launch, opens the highlighted project ──
# The decisive test of "focus is already on the list": no click, no Tab, no
# focus_node — just the key. A preselected row you cannot act on is a tease, and
# an AT `focused` flag is a weaker claim than the app actually doing the thing.
# (Alpha is the most recent, so it is the row under the cursor.)
print("== Phase 0: Enter at launch opens the top recent (no click, no Tab) ==")
s0 = Session([])
if not s0.wait_label("welcome sections"):
    fail("the Launcher window did not appear", s0.app, s0.mcp, s0.log)
if not s0.wait_label("alpha", timeout=8):
    fail("the seeded recents never appeared", s0.app, s0.mcp, s0.log)
lb0 = next((n for n in s0.nodes() if n.get("role") == "ListBox"), None)
print(f"AT `focused` on the recents ListBox at open: {lb0.get('focused') if lb0 else 'no ListBox'}")

s0.key("Enter")
opened_at_launch = s0.wait_label("binder", timeout=15)
loaded_alpha = s0.wait_label("alpha", timeout=10) if opened_at_launch else False
s0.shot("/tmp/sk-kbd-enter-at-launch.png")
if not opened_at_launch:
    failures.append("FOCUS: pressing Enter right after the Launcher opens did nothing — the "
                    "recents list does not hold keyboard focus, so the highlighted project "
                    "cannot be opened without Tabbing to the list first")
elif not loaded_alpha:
    failures.append("FOCUS: Enter at launch opened a project, but not the highlighted top "
                    "recent (Alpha)")
else:
    print("PASS: Enter at launch opened the highlighted top recent (Alpha) — the list is "
          "focused from the start")
s0.close()

print("\n== Launcher: keyboard highlight in the Welcome lists ==")
s = Session([])
if not s.wait_label("welcome sections"):
    fail("the Launcher window did not appear", s.app, s.mcp, s.log)

# ── Recent Works ────────────────────────────────────────────────────────────
if not s.wait_label("alpha", timeout=8):
    fail("the seeded recents never appeared in the Works pane", s.app, s.mcp, s.log)
rows = list_items(s)
print(f"recent rows (ListBoxOption): {[(n.get('id'), n.get('selected')) for n in rows]}")
if len(rows) < 3:
    fail(f"expected the 3 seeded recents as rows, got {len(rows)}", s.app, s.mcp, s.log)

# Focus the list without clicking a row (a click would OPEN the project).
listbox = next((n for n in s.nodes() if n.get("role") == "ListBox"), None)
if not listbox:
    fail("no ListBox node for the recents list", s.app, s.mcp, s.log)
# The launcher preselects the top recent, so a cursor exists before any input.
if selected_index(s) != 0:
    failures.append("RECENTS: the top recent is not preselected on open — the launcher "
                    "should open with a visible cursor (Enter resumes your last project)")

print(f"AT `focused` on the recents ListBox at open: {listbox.get('focused')}")

s.call("focus_node", {"node": listbox["id"]})
time.sleep(0.4)
s.shot("/tmp/sk-kbd-recents-focused.png")
print(f"selected row index after focus, before any arrow: {selected_index(s)}")

s.key("Down")
one_png = s.shot("/tmp/sk-kbd-recents-down1.png")
after_one = selected_index(s)
print(f"selected row index after 1x Down: {after_one}")
# The cursor steps from the *preselected* row 0 to row 1. It must NOT jump to
# row 2: that would mean the view ignored the visible selection and navigated
# from an invisible anchor, skipping a row.
if after_one != 1:
    failures.append(f"RECENTS: the first ArrowDown landed on row {after_one}, expected row 1 "
                    "(step from the preselected top row, skipping nothing)")
s.key("Down")
two_png = s.shot("/tmp/sk-kbd-recents-down2.png")
after_two = selected_index(s)
print(f"selected row index after 2x Down: {after_two}")

if after_two is None:
    failures.append("RECENTS: after 2x ArrowDown no row is reported `selected` in the "
                    "AccessKit tree — a screen reader cannot announce the cursor row")
elif after_two != 2:
    failures.append(f"RECENTS: the second ArrowDown landed on row {after_two}, expected row 2")
# Compare the two ARROW steps, not focus-vs-arrow: the first key press also
# turns on `focus_visible` (keyboard modality), which lights the ListView's
# whole-view container ring — a pixel delta that says nothing about a per-row
# cursor. Between Down #1 and Down #2 the ring is already lit and identical, so
# any difference must be the cursor moving from row 0 to row 1.
if one_png == two_png:
    failures.append("RECENTS: the window pixels are IDENTICAL between ArrowDown #1 and #2 "
                    "— the cursor row does not move on screen "
                    "(/tmp/sk-kbd-recents-down1.png vs /tmp/sk-kbd-recents-down2.png)")
else:
    print("recents: pixels changed between the two arrow steps (a cursor is visible)")

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
    # Examples are a browse list, so nothing is preselected here. With no cursor
    # yet, the first ArrowDown must land ON row 0 — landing on row 1 would skip
    # the first example (and with one bundled example, select nothing at all).
    s2.key("Down")
    idx2 = selected_index(s2)
    s2.shot("/tmp/sk-kbd-examples-down.png")
    print(f"examples selected row index after Down: {idx2}")
    if idx2 is None:
        failures.append("EXAMPLES: after ArrowDown no row is reported `selected` in the AT tree")
    elif idx2 != 0:
        failures.append(f"EXAMPLES: the first ArrowDown landed on row {idx2}, expected row 0 "
                        "(with no cursor yet, Down must land on the first row, not skip it)")
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
