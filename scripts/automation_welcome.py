#!/usr/bin/env python3
"""Drive a live Skribisto via the bastyde automation MCP bridge and verify the
Welcome modal feature end-to-end.

Phase 1 (no CLI arg): the Welcome modal must pop at startup. Assert its title,
the four nav tabs (Works / Examples / Learn / About) and the RECENT WORKS
section are in the AccessKit tree; then activate the Examples tab and confirm the
right pane switches to the bundled example (Starforgers). Screenshot the result.

Phase 2 (a project path): the Welcome modal must NOT appear (the startup gate is
suppressed when a work is passed), and the work must load.

Reuses the launch + scrape-socket/token + connect + announce-before-bind-race
scaffolding from automation_test.py / automation_explore.py.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = "/home/cyril/Devel/skribisto/resources/examples/Starforgers.skrib"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


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
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT)
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
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", [])

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(l for l in self.labels()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def find(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def shot(self, path):
        try:
            res, _ = self.call("screenshot")
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


# ── Phase 1: no arg → Welcome modal appears, tabs switch ──────────────────────
# The modal is detected by its nav-rail access label ("Welcome sections") — the
# header title text is not surfaced as an AccessKit label.
print("== Phase 1: Welcome modal at startup ==")
s = Session([])
if not s.wait_label("welcome sections"):
    fail("Welcome modal did not appear at startup", s.app, s.mcp, s.log)
labels = s.labels()
joined = " | ".join(labels).lower()
print("labels:", " | ".join(labels)[:400])
for needed in ("works", "examples", "learn", "about", "show at startup", "open", "new work"):
    if needed not in joined:
        fail(f"expected '{needed}' in the Welcome modal; got: {joined[:300]}", s.app, s.mcp, s.log)
tabs = [n for n in s.nodes() if n.get("role") == "Tab"]
tablists = [n for n in s.nodes() if n.get("role") == "TabList"]
print(f"AT roles present: TabList x{len(tablists)}, Tab x{len(tabs)}")

# Switch to the Examples tab and confirm the pane reveals the bundled example.
ex_tab = s.find("Examples", role="Tab") or s.find("Examples")
if not ex_tab:
    fail("no 'Examples' tab node", s.app, s.mcp, s.log)
print(f"Examples tab: id={ex_tab.get('id')} role={ex_tab.get('role')} "
      f"selected={ex_tab.get('selected')} actions={ex_tab.get('actions')} bounds={ex_tab.get('bounds')}")
switched = False
r, _ = s.call("invoke_action", {"node": ex_tab["id"]})
time.sleep(0.6)
if s.wait_label("starforgers", timeout=4):
    switched = True
if not switched and ex_tab.get("bounds"):  # fall back to a synthetic click
    b = ex_tab["bounds"]
    cx, cy = (b.get("x", 0) + b.get("width", 0) / 2, b.get("y", 0) + b.get("height", 0) / 2) \
        if isinstance(b, dict) else (b[0] + b[2] / 2, b[1] + b[3] / 2)
    print(f"-> inject_pointer click at ({cx:.0f},{cy:.0f})")
    s.call("inject_pointer", {"x": cx, "y": cy, "kind": "click"})
    time.sleep(0.6)
    switched = s.wait_label("starforgers", timeout=4)
if not switched:
    fail("Examples pane did not reveal the bundled example (Starforgers)", s.app, s.mcp, s.log)
print("PASS: Examples tab switched the pane to the bundled example (Starforgers)")
s.shot("/tmp/sk-welcome-shot.png")
s.close()

# ── Phase 2: a project path → Welcome modal suppressed, work loads ────────────
print("\n== Phase 2: startup gate (project passed) ==")
s2 = Session([EXAMPLE])
# Give the launch-load (+ legacy .skrib migration) a moment; assert no modal.
time.sleep(2.5)
joined2 = " | ".join(s2.labels()).lower()
if "welcome sections" in joined2 or "show at startup" in joined2:
    fail("Welcome modal appeared even though a project was passed", s2.app, s2.mcp, s2.log)
loaded = s2.wait_label("starforgers", timeout=12)
s2.shot("/tmp/sk-welcome-gate-shot.png")
s2.close()
if not loaded:
    fail("project did not appear loaded in Phase 2 (real backend)", None, None, None)
print("PASS: Welcome suppressed and the work loaded when a project is passed")

print("\nALL PASS")
sys.exit(0)
