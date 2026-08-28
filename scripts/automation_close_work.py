#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify that **closing a dirty work saves before it
closes** — the deferred-close path, end to end.

Closing is not synchronous. `work.close` (Work ▸ Close Work, Ctrl+W, the window's
X) runs the unsaved-changes guard; choosing **Save** does not close anything — it
arms `pending_exit`, asks `EditorsViewModel` for a disk save, and remembers the
edit *sequence* that save will cover. The close performs only once a
`LongOperation::Completed` reports that sequence on disk, via
`view_models::save_queue::resume_deferred`.

That indirection is what makes the flow worth a script. The failure modes are
both silent and both bad:

  * close *before* the write lands → the store is torn down and the last edits
    exist in neither the file nor memory;
  * never close at all → Ctrl+W / the X do nothing, forever, with no error.

Neither shows up in a unit test (the decision is pure and tested, but the wiring
to a real async save is not), and no other automation script exercised it — the
sibling automation_unsaved_guard.py drives the *switch* door (New Work), which
shares the guard but not the deferred close.

  1. load a scratch copy of an example, type into a scene → the work is dirty
  2. Work ▸ Close Work                                    → the guard appears
  3. Save                                                 → the close is armed,
                                                            not performed
  4. wait                                                 → the Launcher replaces
                                                            the project window
  5. reopen the file                                      → the typed prose is in
                                                            it (the save really
                                                            covered the edits)

Step 5 is the point: reaching the Launcher only proves *something* closed. Only
re-reading the file proves the close waited for the right write.

The **menu**, not Ctrl+W: a synthetic `inject_key` only reaches the app's global
shortcuts while the window holds OS keyboard focus — which a freshly-launched
window on this compositor often doesn't — and a silently-dropped keystroke would
make this script pass without testing anything. The menu item is a click and
fires the very same global `work.close` action.

Assertions favour locale-independent anchors (AccessKit roles, geometry, the
button set) so this passes whatever UI language is persisted.

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""

import json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

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
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "unsaved-guard-test", "version": "1"}})
            # Generous: the app's bridge accepts one connection, and tearing a slow
            # -but-working MCP down to retry leaves the socket unusable for the
            # replacement (every later call then dies with a broken pipe).
            init = self._recv(timeout=15, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.5)
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

    def find_exact(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def shot(self, path):
        try:
            res, _ = self.call("screenshot")
            import base64
            for c in res.get("content", []):
                if c.get("type") == "image":
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"  screenshot → {path}")
                    return
        except Exception as e:
            print(f"  (screenshot failed: {e})")

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


# ── Launch with a *copy* of the bundled example ───────────────────────────────
# This types into a scene and really saves, so never point it at the repo's own
# example.
print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-close-work-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=30):
    fail("the example work did not load", s.app, s.mcp, s.log)
print(f"  loaded {project}")

# A marker distinctive enough that finding it in the saved bundle proves *these*
# edits were written, not merely that a save happened at some point.
MARKER = "Deferred close probe alpha7."


def click(n):
    """Click a node — its AccessKit `click` action when it has one, else a
    synthetic pointer tap at its centre."""
    if "click" in (n.get("actions") or []):
        res, _ = s.call("invoke_action", {"node": n["id"], "action": "click"})
        if not (isinstance(res, dict) and res.get("isError")):
            return True
    b = n.get("bounds") or {}
    if "x" not in b:
        return False
    s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                              "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    return True


def menu_item(label_substr, timeout=8):
    """The first open MenuItem whose label contains `label_substr` (case-insensitive)."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        for n in s.nodes():
            if n.get("role") == "MenuItem" and label_substr in (n.get("label") or "").lower():
                return n
        time.sleep(0.3)
    return None


# ── 1. Dirty the work ─────────────────────────────────────────────────────────
s.wait_label("prologue", timeout=20)
row = next((n for n in s.nodes()
            if "prologue" in (n.get("label") or "").lower()
            and (n.get("bounds") or {}).get("width")), None)
if not row:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
click(row)
time.sleep(1.0)

editors = [n for n in s.nodes() if n.get("role") == "MultilineTextInput"]
if not editors:
    fail("no editor after opening Prologue", s.app, s.mcp, s.log)
# The tallest input is the scene body; the shorter one is the synopsis.
main = max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))
res, _ = s.call("type_text", {"node": main["id"], "text": MARKER})
if isinstance(res, dict) and res.get("isError"):
    fail(f"could not type into the scene: {res}", s.app, s.mcp, s.log)
time.sleep(0.8)
print("  typed into the scene — the work is now dirty")

# ── 2. Work ▸ Close Work → the guard must appear ──────────────────────────────
ham = s.find_exact("Menu", role="Button")
if not ham:
    fail("no title-bar 'Menu' (hamburger) button in the AT tree", s.app, s.mcp, s.log)
click(ham)
time.sleep(0.6)
fm = s.find_exact("Work", role="MenuItem")
if not fm:
    fail("the 'File' menu did not open", s.app, s.mcp, s.log)
click(fm)
time.sleep(0.7)
item = menu_item("close")
if not item:
    fail("no 'Close Work' item in the open File menu", s.app, s.mcp, s.log)
click(item)
time.sleep(1.0)

# ── 3. Choose Save → arms the deferred close ──────────────────────────────────
save_btn = None
deadline = time.time() + 8
while time.time() < deadline:
    save_btn = next((n for n in s.nodes()
                     if n.get("role") == "Button"
                     and (n.get("label") or "").strip().lower() == "save"), None)
    if save_btn:
        break
    time.sleep(0.4)
if not save_btn:
    print("  labels on screen:", [l for l in s.labels() if l.strip()][:30])
    fail("closing a dirty work did not raise the unsaved-changes guard "
         "(it must never close without asking)", s.app, s.mcp, s.log)
print("  the guard prompted; choosing Save arms the deferred close")
click(save_btn)

# ── 4. The Launcher replaces the project window once the save lands ───────────
reached = False
deadline = time.time() + 25
while time.time() < deadline:
    time.sleep(0.5)
    labels = " ".join(s.labels()).lower()
    if "recent works" in labels or "new work" in labels:
        reached = True
        break
if not reached:
    print("  labels on screen:", [l for l in s.labels() if l.strip()][:30])
    fail("the deferred close never performed — Close Work saved but never closed",
         s.app, s.mcp, s.log)
print("PASS: closing a dirty work saved first, then returned to the Launcher")

# ── 5. The edits actually reached the file ────────────────────────────────────
# Reaching the Launcher only proves *something* closed. Only the marker being on
# disk proves the close waited for the write that carried these edits.
found = False
if os.path.isdir(project):
    for root, _dirs, files in os.walk(project):
        for f in files:
            try:
                with open(os.path.join(root, f), "r", encoding="utf-8", errors="ignore") as fh:
                    if MARKER in fh.read():
                        found = True
                        break
            except OSError:
                pass
        if found:
            break
else:
    import zipfile
    try:
        with zipfile.ZipFile(project) as z:
            for name in z.namelist():
                try:
                    if MARKER.encode() in z.read(name):
                        found = True
                        break
                except KeyError:
                    pass
    except zipfile.BadZipFile:
        fail(f"{project} is neither a folder nor a readable zip after the save",
             s.app, s.mcp, s.log)

if not found:
    fail("the close performed but the typed prose is not in the saved file — "
         "the close did not wait for the write that covered these edits",
         s.app, s.mcp, s.log)
print("PASS: the typed prose is in the saved bundle — the close awaited its own write")

s.close()
shutil.rmtree(scratch, ignore_errors=True)
print("\nALL CHECKS PASSED")
