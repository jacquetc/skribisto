#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify **which chrome distraction-free mode keeps**.

The headless tests pin the pieces (`tab_bar_policy`, the strip's `VisibleWhen`
gates, and — in bastyde — that a bound `TabBarVisibility` flips the strip in
place). What they cannot see is the assembled window: that entering the mode
really does reach `TabWidget::bar_visibility`, and that the editor tab strip
disappears while the Exit button stays.

Flow: load a scratch copy of an example →
  1. open a scene → an editor tab strip (Role::TabList) exists
  2. Shift+F11    → the tab strip is GONE and the strip's Exit button is present
  3. Shift+F11    → the tab strip is BACK

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

# Resolved from this script's own location, not hardcoded to the main
# checkout — this feature was built in a worktree, which has its own `target/`.
REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = os.path.join(REPO, "target/debug/skribisto")
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = os.path.join(REPO, "resources/examples/Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


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
        self._id = 0
        while not os.path.exists(sock) and time.time() < deadline:
            time.sleep(0.05)
        self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "df-chrome-test", "version": "1"}})
        if self._recv(timeout=15, fatal=False) is None:
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
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", [])

    def wait_label(self, substr, timeout=30):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(n.get("label") or "" for n in self.nodes()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image":
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot → {path}")

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


def click(s, n):
    if "click" in (n.get("actions") or []):
        s.call("invoke_action", {"node": n["id"], "action": "click"})
    b = n.get("bounds") or {}
    if "x" in b:
        s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                  "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})


def tab_lists(s):
    """Every Role::TabList in the window. The editor panes are the only tab
    strips in a project window, so a non-empty result means the editor tab
    strip is mounted."""
    return [n for n in s.nodes() if n.get("role") == "TabList"]


def has_label(s, substr):
    return any(substr.lower() in (n.get("label") or "").lower() for n in s.nodes())


def wait_for(pred, timeout=10):
    end = time.time() + timeout
    while time.time() < end:
        if pred():
            return True
        time.sleep(0.4)
    return False


print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-df-chrome-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=30):
    fail("the example work did not load", s.app, s.mcp, s.log)
print(f"  loaded {project}")

# ── 1. Open a scene so an editor tab (and therefore a tab strip) exists ──────
print("== open a scene ==")
row = next((n for n in s.nodes() if (n.get("label") or "").strip() == "Prologue"), None)
if not row:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
editors = []
for _ in range(3):
    click(s, row)
    end = time.time() + 10
    while time.time() < end and not editors:
        editors = [n for n in s.nodes() if n.get("role") == "MultilineTextInput"
                   and "set_value" in (n.get("actions") or [])]
        if not editors:
            time.sleep(0.5)
    if editors:
        break
if not editors:
    fail("no writing editor after opening Prologue", s.app, s.mcp, s.log)

if not wait_for(lambda: len(tab_lists(s)) > 0):
    fail("no editor tab strip before entering the mode — the rest of this "
         "script would pass vacuously", s.app, s.mcp, s.log)
before = len(tab_lists(s))
print(f"  {before} tab strip(s) present outside the mode")
s.shot("/tmp/df-chrome-before.png")

# ── 2. Enter distraction-free → the tab strip goes, Exit stays ───────────────
print("== Shift+F11 enters distraction-free ==")
s.call("inject_key", {"key": "F11", "shift": True})
if not wait_for(lambda: len(tab_lists(s)) == 0, timeout=10):
    s.shot("/tmp/df-chrome-still-there.png")
    fail(f"the editor tab strip survived entering the mode "
         f"({len(tab_lists(s))} TabList node(s) remain)", s.app, s.mcp, s.log)
print("  tab strip hidden")

if not wait_for(lambda: has_label(s, "exit"), timeout=10):
    s.shot("/tmp/df-chrome-no-exit.png")
    fail("no Exit button in the distraction-free strip — the one control that "
         "must never be absent", s.app, s.mcp, s.log)
print("  Exit button present")
s.shot("/tmp/df-chrome-during.png")

# ── 3. Leave the mode → the tab strip comes back ────────────────────────────
print("== Shift+F11 leaves distraction-free ==")
s.call("inject_key", {"key": "F11", "shift": True})
if not wait_for(lambda: len(tab_lists(s)) == before, timeout=10):
    s.shot("/tmp/df-chrome-not-restored.png")
    fail(f"the tab strip did not come back on leaving the mode "
         f"(want {before}, got {len(tab_lists(s))})", s.app, s.mcp, s.log)
print("  tab strip restored")
s.shot("/tmp/df-chrome-after.png")

# ── 4. The settings surface exists and is on the right page ─────────────────
# The four checkboxes are the only way a writer changes any of this, so a pane
# that silently failed to grow them would leave the feature unreachable even
# though every gate below it works.
print("== Settings ▸ Editor ▸ Editor Behavior carries the four checkboxes ==")


def settings_open():
    """The instant-apply footer is on every pane, so it is a page-agnostic
    "the Settings window is open" signal."""
    return has_label(s, "reset to defaults") and has_label(s, "done")


opened = None
for args in ({"key": ",", "ctrl": True}, {"key": "Comma", "ctrl": True}):
    res, _ = s.call("inject_key", args)
    if not (isinstance(res, dict) and res.get("isError")):
        time.sleep(0.8)
        if settings_open():
            opened = args
            break
if not opened:
    fail("could not open the Settings window", s.app, s.mcp, s.log)

page = next((n for n in s.nodes()
             if (n.get("label") or "").strip().lower() == "editor behavior"), None)
if not page:
    fail("no 'Editor Behavior' page row in the settings category rail", s.app, s.mcp, s.log)
if "click" in (page.get("actions") or []):
    s.call("invoke_action", {"node": page["id"], "action": "click"})
else:
    b = page.get("bounds") or {}
    s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                              "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
time.sleep(0.8)

WANT = ["keep the editor tabs", "keep the word count",
        "keep the writing session", "keep the previous and next buttons"]
missing = [w for w in WANT if not has_label(s, w)]
s.shot("/tmp/df-chrome-settings.png")
if missing:
    fail(f"the Editor Behavior pane is missing {missing}", s.app, s.mcp, s.log)
print("  all four checkboxes present")

# Exit is promised to have no checkbox — a settings row that could take it away
# would defeat the strip's whole reason for being always-visible.
if has_label(s, "keep the exit"):
    fail("an Exit checkbox appeared — Exit must never be optional",
         s.app, s.mcp, s.log)
print("  and no Exit checkbox, as promised")

print("\nPASS: distraction-free hides the editor tab strip, keeps Exit, restores "
      "the strip on the way out, and its four toggles are reachable in Settings.")
s.close()
