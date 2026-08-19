#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **Go to…** jump popup (Ctrl+G).

Flow: load a scratch copy of an example, open a scene, then:

  1. Ctrl+G opens the popup, with its search field focused;
  2. typing narrows the binder tree to matching rows;
  3. Down + Enter jumps to the highlighted row and closes the popup;
  4. Escape closes the popup and, inside distraction-free mode, does so
     without also leaving the mode — one keystroke must not do two things.

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

# Resolved from this script's own location, not hardcoded to the main
# checkout — this feature was built in a worktree, which has its own `target/`.
REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
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



def labels(s):
    return [(n.get("label") or "") for n in s.nodes()]


def has(s, sub):
    return any(sub.lower() in l.lower() for l in labels(s))


def popup_open(s):
    """Is the Go-to popover up?

    Asked through `get_overlays`, not by looking for the search field's
    placeholder: a placeholder is not an accessible label, so matching on it
    reports "closed" for a popover that is plainly on screen."""
    _, p = s.call("get_overlays")
    return int(p.get("count", 0)) > 0


def wait_for(pred, timeout=10):
    end = time.time() + timeout
    while time.time() < end:
        if pred():
            return True
        time.sleep(0.4)
    return False


def editors(s):
    return [n for n in s.nodes() if n.get("role") == "MultilineTextInput"
            and "set_value" in (n.get("actions") or [])]


print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-goto-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=30):
    fail("the example work did not load", s.app, s.mcp, s.log)

print("== open a scene so there is somewhere to jump FROM ==")
row = next((n for n in s.nodes() if (n.get("label") or "").strip() == "Prologue"), None)
if not row:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
for _ in range(3):
    click(s, row)
    if wait_for(lambda: len(editors(s)) > 0, timeout=8):
        break
if not editors(s):
    fail("no writing editor after opening Prologue", s.app, s.mcp, s.log)

# ── 1. Ctrl+G opens it. This is the whole point of `open_action`: the
#      shortcut is nowhere near the button, and before the framework gained
#      that mechanism a popover could only be opened by clicking its trigger.
print("== Ctrl+G opens the popup ==")
s.call("inject_key", {"key": "g", "ctrl": True})
if not wait_for(lambda: popup_open(s), timeout=10):
    s.shot("/tmp/goto-not-open.png")
    fail("Ctrl+G did not open the Go to popup", s.app, s.mcp, s.log)
print("  popup open (search field present)")
s.shot("/tmp/goto-open.png")

# ── 2. Typing narrows the tree, keeping ancestors.
print("== typing filters the binder ==")
# Two traps here, both hit for real:
#   * picking the first `TextInput` by role grabs the *binder dock's* search
#     field, so the query never reaches the popover and the keystrokes
#     type-ahead-select a row behind it;
#   * `inject_key` carries no text payload, so letters sent that way focus
#     nothing and insert nothing.
# So: locate the field inside the popover by position, then `type_text` it.
# The popover focuses its search field on open, so `focused` names it exactly —
# and asserting on it also pins that focus hand-off. Position alone is not a
# discriminator: the Inspector's own date field sits in the same corner.
fields = [n for n in s.nodes()
          if n.get("role") == "TextInput" and n.get("focused")]
if len(fields) != 1:
    s.shot("/tmp/goto-field-ambiguous.png")
    fail(f"expected the popover's search field to hold focus, found {len(fields)} "
         f"focused text inputs", s.app, s.mcp, s.log)
field = fields[0]
before = len([n for n in s.nodes() if n.get("role") == "TreeItem"])
s.call("type_text", {"node": field["id"], "text": "epi"})  # matches Epilogue only
time.sleep(1.2)
after = len([n for n in s.nodes() if n.get("role") == "TreeItem"])
s.shot("/tmp/goto-filtered.png")
if not (0 < after < before):
    fail(f"the query did not narrow the tree (before={before}, after={after})",
         s.app, s.mcp, s.log)
if not has(s, "epilogue"):
    fail("the surviving rows do not include the match", s.app, s.mcp, s.log)
print(f"  {before} rows -> {after} rows, Epilogue among them")

# ── 2b. Down highlights a result and Enter opens it, without ever leaving the
#       search field. This is the "one list, driven from the field" shape — the
#       reason the popup has no second suggestion dropdown over the tree.
print("== Down + Enter jumps ==")
s.call("inject_key", {"key": "ArrowDown"})
time.sleep(0.5)
s.call("inject_key", {"key": "Enter"})
if not wait_for(lambda: not popup_open(s), timeout=10):
    s.shot("/tmp/goto-enter-did-nothing.png")
    fail("Enter did not act on the highlighted row", s.app, s.mcp, s.log)
if not wait_for(lambda: has(s, "epilogue"), timeout=10):
    s.shot("/tmp/goto-enter-wrong-target.png")
    fail("Enter closed the popup without opening the match", s.app, s.mcp, s.log)
print("  jumped to Epilogue and closed")
s.shot("/tmp/goto-jumped.png")

# Reopen for the Escape checks below.
s.call("inject_key", {"key": "g", "ctrl": True})
if not wait_for(lambda: popup_open(s), timeout=10):
    fail("could not reopen the popup", s.app, s.mcp, s.log)

# ── 3. Escape closes the popup and nothing else.
print("== Escape closes the popup ==")
s.call("inject_key", {"key": "Escape"})
if not wait_for(lambda: not popup_open(s), timeout=10):
    fail("Escape did not close the popup", s.app, s.mcp, s.log)
print("  popup closed")

# ── 4. In distraction-free, Escape must close the popup WITHOUT also leaving
#      the mode — two things from one keystroke is the surprise to avoid.
print("== in distraction-free, Escape closes only the popup ==")
s.call("inject_key", {"key": "F11", "shift": True})
if not wait_for(lambda: has(s, "exit distraction-free"), timeout=10):
    fail("could not enter distraction-free", s.app, s.mcp, s.log)
s.call("inject_key", {"key": "g", "ctrl": True})
if not wait_for(lambda: popup_open(s), timeout=10):
    s.shot("/tmp/goto-df-not-open.png")
    fail("Ctrl+G did not open the popup inside distraction-free", s.app, s.mcp, s.log)
s.shot("/tmp/goto-df-open.png")
s.call("inject_key", {"key": "Escape"})
if not wait_for(lambda: not popup_open(s), timeout=10):
    fail("Escape did not close the popup in distraction-free", s.app, s.mcp, s.log)
time.sleep(0.6)
if not has(s, "exit distraction-free"):
    s.shot("/tmp/goto-df-escaped-out.png")
    fail("Escape closed the popup AND left distraction-free — one keystroke "
         "must not do two things", s.app, s.mcp, s.log)
print("  popup closed, still in distraction-free")
s.shot("/tmp/goto-df-after-escape.png")

print("\nPASS: Go to opens by shortcut, filters the binder, and its Escape is "
      "scoped to the popup even inside distraction-free.")
s.close()
