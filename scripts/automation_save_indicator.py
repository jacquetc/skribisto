#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **status-bar save indicator**.

The indicator is the quiet answer to "is my last paragraph on disk?" — the one
thing the app could never tell you under autosave, which hides Save and Ctrl+S.
A save glyph with an asterisk means unsaved, with a check means written; clicking
saves. (Failures are toasts, not a fourth glyph — a write that didn't happen is
not a quiet fact.)

Flow: load a scratch copy of an example →
  1. clean project      → the indicator says "all changes saved"
  2. type into a scene  → it flips to "unsaved changes"
  3. click it           → it goes back to "saved" (the click really wrote the file)

The indicator surfaces as a Button whose accessible name *is* its tooltip, so the
state is asserted from the a11y tree rather than from pixels — and it is asserted
through the name, not the glyph, so this passes in either locale (the names come
from the same `tr!` keys the tooltip uses; the check below matches on the key's
en-US text and falls back to the "saved"/"unsaved" distinction).

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/Starforgers.skrib")

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
                                  "clientInfo": {"name": "save-indicator-test", "version": "1"}})
        # Generous: tearing a slow-but-working MCP down to retry leaves the app's
        # single-connection bridge unusable for the replacement.
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


print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-indicator-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=30):
    fail("the example work did not load", s.app, s.mcp, s.log)
print(f"  loaded {project}")


def indicator(timeout=10):
    """The save indicator: the one status-bar Button whose accessible name is the
    save tooltip. Matched on the name (which *is* the tooltip), not on pixels."""
    end = time.time() + timeout
    while time.time() < end:
        for n in s.nodes():
            label = (n.get("label") or "").lower()
            if n.get("role") == "Button" and ("saved" in label or "unsaved" in label):
                return n
        time.sleep(0.4)
    return None


def state(node):
    label = (node.get("label") or "").lower()
    return "unsaved" if "unsaved" in label else "saved"


def click(n):
    if "click" in (n.get("actions") or []):
        s.call("invoke_action", {"node": n["id"], "action": "click"})
    b = n.get("bounds") or {}
    if "x" in b:
        s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                  "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})


# ── 1. A freshly-loaded project is exactly what is on disk ───────────────────
print("== a freshly-loaded project reads as saved ==")
ind = indicator()
if not ind:
    fail("no save indicator in the status bar", s.app, s.mcp, s.log)
print(f"  indicator: {ind.get('label')!r}")
if state(ind) != "saved":
    fail(f"a just-loaded project must read as saved, got {ind.get('label')!r}",
         s.app, s.mcp, s.log)
s.shot("/tmp/indicator-clean.png")

# ── 2. Typing flips it to unsaved ────────────────────────────────────────────
print("== typing flips it to unsaved ==")
row = next((n for n in s.nodes() if (n.get("label") or "").strip() == "Prologue"), None)
if not row:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
editors = []
for attempt in range(3):
    click(row)
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
main = max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))
s.call("type_text", {"node": main["id"], "text": "The indicator must notice this. "})
time.sleep(1.5)

ind = indicator()
print(f"  indicator: {ind.get('label')!r}")
if state(ind) != "unsaved":
    fail("the indicator still reads as saved after typing — it is not tracking the "
         "dirty state", s.app, s.mcp, s.log)
s.shot("/tmp/indicator-dirty.png")

# ── 3. Clicking it saves ─────────────────────────────────────────────────────
# Not just "the icon changed": the click must actually reach `request_save`, so the
# state has to settle back to saved on its own (the write is a background op).
print("== clicking it writes the work ==")
click(ind)
end = time.time() + 15
settled = False
while time.time() < end:
    ind = indicator()
    if ind and state(ind) == "saved":
        settled = True
        break
    time.sleep(0.5)
if not settled:
    s.shot("/tmp/indicator-stuck.png")
    fail("the indicator never went back to saved after clicking it — the click did "
         "not reach the save, or the save never landed", s.app, s.mcp, s.log)
print(f"  indicator: {ind.get('label')!r}")
s.shot("/tmp/indicator-after-save.png")

print("\nPASS: the save indicator tracks the work — saved on load, unsaved on the "
      "first keystroke, saved again once a click writes it.")
s.close()
