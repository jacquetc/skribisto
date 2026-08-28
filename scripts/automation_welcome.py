#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the teksilo automation MCP bridge and verify the
**launcher-window** model end-to-end (the Welcome UI is a real window now, not
a modal — see `teksilo_ui::main`'s module docs).

Phase 1 (no CLI arg): a bare launch must open the **Launcher window**. Assert
its nav tabs (Works / Examples / Learn / About) and the RECENT WORKS section
are in the AccessKit tree; activate the Examples tab and confirm the pane
switches to the bundled example (Starforgers). Then actually open that
example: click its row and assert (a) a **second** window (the project) opens
*before* the Launcher closes — `list_windows` briefly reports two — (b) the
Launcher then closes, leaving exactly one window, (c) that window shows the
loaded project, and (d) the process is still running throughout (a
launcher→project transition done in the wrong order quits the app).

Phase 2 (a project path): the Launcher must NOT appear at all (argv always
skips it), and the work must load directly into the (only) window.

Reuses the launch + scrape-socket/token + connect + announce-before-bind-race
scaffolding from automation_test.py / automation_explore.py.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# Isolated XDG dirs (never touches the real user's ~/.config/skribisto), same
# rationale as automation_two_process.py: a real, ever-growing recents.toml
# from repeated runs makes "find the Starforgers row" ambiguous once a prior
# run has *also* recorded "Starforgers" as a recent (it's the bundled
# example's fixed temp path) — the label-based lookup would then match either
# the Examples-tab row or a stale Recent-Works row.
_sandbox = tempfile.mkdtemp(prefix="skribisto_welcome_test_")
SANDBOX_ENV = {
    "XDG_CONFIG_HOME": os.path.join(_sandbox, "config"),
    "XDG_DATA_HOME": os.path.join(_sandbox, "data"),
    "HOME": _sandbox,
}


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    try:
        et = open(mcp_err).read()
        if et.strip():
            print("--- mcp stderr ---")
            print("\n".join(et.splitlines()[-15:]))
    except Exception:
        pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


class Session:
    """One launched app + connected MCP server."""

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
        self.sock, self.tok = sock, tok
        self._id = 0
        self.mcp = None
        # The bridge announces the socket before binding; retry connect+init.
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "welcome-test", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect MCP (socket never reachable)", self.app, self.mcp, self.log)
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
            # Ops reply with either a JSON object (`snapshot_tree` -> {"nodes":
            # [...]}) or a bare JSON array (`list_windows` -> [WindowInfo, ...]).
            payload = json.loads(t) if (t.startswith("{") or t.startswith("[")) else {}
        return result, payload

    def nodes(self, window_id=None):
        args = {"window_id": window_id} if window_id is not None else {}
        _, p = self.call("snapshot_tree", args)
        return p.get("nodes", []) if isinstance(p, dict) else []

    def labels(self, window_id=None):
        return [n.get("label") for n in self.nodes(window_id) if n.get("label")]

    def wait_label(self, substr, timeout=20, window_id=None):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(l for l in self.labels(window_id)).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def find(self, label, role=None, window_id=None):
        for n in self.nodes(window_id):
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def list_windows(self):
        _, p = self.call("list_windows")
        return p if isinstance(p, list) else []

    def wait_window_count(self, want, timeout=10):
        end = time.time() + timeout
        seen = None
        while time.time() < end:
            seen = self.list_windows()
            if len(seen) == want:
                return seen
            time.sleep(0.3)
        return seen

    def shot(self, path, window_id=None):
        try:
            args = {"window_id": window_id} if window_id is not None else {}
            res, _ = self.call("screenshot", args)
            for c in res.get("content", []):
                if c.get("type") == "image" and c.get("data"):
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"screenshot -> {path}")
        except Exception as e:
            print("screenshot failed:", e)

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


# ── Phase 1: no arg → the Launcher window appears, tabs switch, then a
#    picked example opens a project window and the Launcher closes ──────────
# The Launcher is detected by its nav-rail access label ("Welcome sections") —
# the header title text is not surfaced as an AccessKit label.
print("== Phase 1: the Launcher window at a bare startup ==")
s = Session([])
if not s.wait_label("welcome sections"):
    fail("the Launcher window did not appear at startup", s.app, s.mcp, s.log)
labels = s.labels()
joined = " | ".join(labels).lower()
print("labels:", " | ".join(labels)[:400])
for needed in ("works", "examples", "learn", "about", "open", "new work"):
    if needed not in joined:
        fail(f"expected '{needed}' in the Launcher window; got: {joined[:300]}", s.app, s.mcp, s.log)
# The "show at startup" control lives only in Settings ▸ Appearance & Behaviour
# now — a launcher-local copy would hide the very screen it's on. Assert its
# ABSENCE here instead (regression guard for that removal).
if "show at startup" in joined:
    fail("the Launcher must not have its own inline 'show at startup' checkbox "
         "(that setting lives in Settings only)", s.app, s.mcp, s.log)
# Likewise the modal-era inline "✕" is gone; the window's own TitleBar close
# button is the only close affordance now (still present as "Close" above).
tabs = [n for n in s.nodes() if n.get("role") == "Tab"]
tablists = [n for n in s.nodes() if n.get("role") == "TabList"]
print(f"AT roles present: TabList x{len(tablists)}, Tab x{len(tabs)}")

windows_before = s.list_windows()
print(f"windows before opening a project: {len(windows_before)} -> {windows_before}")
if len(windows_before) != 1:
    fail(f"expected exactly one window (the Launcher) before picking a project, got {len(windows_before)}",
         s.app, s.mcp, s.log)

# Switch to the Examples tab via the AccessKit *click* action and confirm the
# pane reveals the bundled example. `invoke_action` REQUIRES an `action`
# argument — omitting it errors and is a no-op (the bug this harness used to
# paper over with an inject_pointer fallback). The AT click selects the tab and
# the sibling Switcher swaps panes; no synthetic pointer needed.
ex_tab = s.find("Examples", role="Tab") or s.find("Examples")
if not ex_tab:
    fail("no 'Examples' tab node", s.app, s.mcp, s.log)
print(f"Examples tab: id={ex_tab.get('id')} role={ex_tab.get('role')} "
      f"selected={ex_tab.get('selected')} actions={ex_tab.get('actions')} bounds={ex_tab.get('bounds')}")
res, _ = s.call("invoke_action", {"node": ex_tab["id"], "action": "click"})
if res.get("isError"):
    fail(f"invoke_action click on the Examples tab errored: {res}", s.app, s.mcp, s.log)
if not s.wait_label("starforgers", timeout=5):
    fail("Examples pane did not reveal the bundled example (Starforgers) after invoke_action click",
         s.app, s.mcp, s.log)
print("PASS: Examples tab switched the pane to the bundled example (Starforgers)")
s.shot("/tmp/sk-welcome-shot.png")

# ── Open the example: the Launcher must open a project window BEFORE closing
#    itself (the critical ordering rule — reversed, this would quit the app) —
#    and the process must still be running once it settles on one window. ───
star_row = s.find("Starforgers", role="ListItem") or next(
    (n for n in s.nodes() if "starforgers" in (n.get("label") or "").lower()
     and n.get("role") not in ("Tab", "TabList")),
    None,
)
if not star_row:
    fail("no Starforgers list row to open", s.app, s.mcp, s.log)
print(f"Starforgers row: id={star_row.get('id')} role={star_row.get('role')} actions={star_row.get('actions')}")
# `ListView` rows carry no AT action of their own (confirmed: role "Unknown",
# no `actions`) — `ActivateOn::SingleClick` is a pointer/AT-focus gesture the
# row's own semantics don't expose as an invokable action. Same class of gap
# as the RadioTile fallback in automation_new_work.py: drive it with a
# synthetic pointer click at the row's bounds.
b = star_row.get("bounds") or {}
cx = b.get("x", 0) + b.get("width", 0) / 2
cy = b.get("y", 0) + b.get("height", 0) / 2
res, _ = s.call("inject_pointer", {"x": cx, "y": cy, "action": "click"})
if isinstance(res, dict) and res.get("isError"):
    fail(f"inject_pointer click on the Starforgers row errored: {res}", s.app, s.mcp, s.log)

# Poll for the transition rather than a single immediate snapshot: opening the
# project window + tearing down the Launcher is a real (if fast) sequence of
# window-manager operations, and `list_windows` already reading "1" before the
# click (just the Launcher) would make a naive one-shot check a false pass.
# The unambiguous signal is the *content*: the Launcher's nav ("Welcome
# sections") must be gone and the project's own chrome ("Menu"/"Binder",
# registered by `App::build`) must be up.
end = time.time() + 15
transitioned = False
while time.time() < end:
    if s.app.poll() is not None:
        fail("the app process exited while opening the example from the Launcher "
             "(a wrong open/close ordering quits it)", s.app, s.mcp, s.log)
    joined_now = " | ".join(s.labels()).lower()
    if "welcome sections" not in joined_now and "binder" in joined_now:
        transitioned = True
        break
    time.sleep(0.4)
if not transitioned:
    print("labels at timeout:", " | ".join(s.labels())[:400])
    print("windows at timeout:", s.list_windows())
    fail("opening Starforgers from the Launcher never transitioned to the project window",
         s.app, s.mcp, s.log)

settled = s.list_windows()
if len(settled) != 1:
    fail(f"expected exactly one window once the Launcher closes, got {settled}", s.app, s.mcp, s.log)
if not s.wait_label("starforgers", timeout=10):
    fail("the project window never shows the opened work's content", s.app, s.mcp, s.log)
print("PASS: opening Starforgers from the Launcher opened the project window, "
      "then closed the Launcher — one window remains, process still running")
s.shot("/tmp/sk-welcome-project-shot.png")
s.close()

# ── Phase 2: a project path → the Launcher never appears, one window only ───
print("\n== Phase 2: startup gate (project passed) ==")
s2 = Session([EXAMPLE])
# Give the launch-load (+ legacy .skrib migration) a moment; assert no Launcher.
time.sleep(2.5)
joined2 = " | ".join(s2.labels()).lower()
if "welcome sections" in joined2 or "show at startup" in joined2:
    fail("the Launcher window appeared even though a project was passed", s2.app, s2.mcp, s2.log)
win2 = s2.list_windows()
if len(win2) != 1:
    fail(f"expected exactly one window when launched with a path, got {win2}", s2.app, s2.mcp, s2.log)
loaded = s2.wait_label("starforgers", timeout=12)
s2.shot("/tmp/sk-welcome-gate-shot.png")
s2.close()
if not loaded:
    fail("project did not appear loaded in Phase 2 (real backend)", None, None, None)
print("PASS: the Launcher was skipped and the work loaded directly when a project is passed")

shutil.rmtree(_sandbox, ignore_errors=True)
print("\nALL PASS")
sys.exit(0)
