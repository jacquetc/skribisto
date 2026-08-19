#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the teksilo automation MCP bridge and verify that the
"Save" affordances are disabled while there is nothing to save.

A freshly-loaded project is clean, so Work ▸ Save must be greyed out (and the
`editor.save` action / Ctrl+S inert). Typing one character into a scene marks the
work unsaved, so the very same menu item must go live. Then Ctrl+S saves and it
must fall back to disabled.

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
                                      "clientInfo": {"name": "save-enabled-test", "version": "1"}})
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

    def find_exact(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def find_contains(self, substr, role=None):
        sub = substr.lower()
        for n in self.nodes():
            if sub in (n.get("label") or "").lower() and (role is None or n.get("role") == role):
                return n
        return None

    def shot(self, path):
        try:
            res, _ = self.call("screenshot")
            for c in res.get("content", []):
                if c.get("type") == "image" and c.get("data"):
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"  screenshot -> {path}")
        except Exception as e:
            print("  screenshot failed:", e)

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


# ── Launch with a *copy* of the bundled example ───────────────────────────────
# The last step really presses Ctrl+S, which really writes to disk — so never point
# it at the example in the repo (Welcome is suppressed when a work is given).
print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-save-check-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=15):
    fail("the example work did not load", s.app, s.mcp, s.log)
print(f"  loaded {project}")


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
                              "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
    return True


def save_item_disabled(shot=None):
    """Open the title-bar menu ▸ File and report the Save item's `disabled` flag.
    Leaves the menu closed."""
    ham = s.find_exact("Menu", role="Button")
    if not ham:
        fail("no title-bar 'Menu' (hamburger) button in the AT tree", s.app, s.mcp, s.log)
    click(ham)
    time.sleep(0.6)
    work_menu = s.find_exact("Work", role="MenuItem")
    if not work_menu:
        print("  labels:", " | ".join(x for x in s.labels() if x)[:400])
        fail("the 'File' menu did not open", s.app, s.mcp, s.log)
    click(work_menu)
    time.sleep(0.7)
    item = s.find_exact("Save", role="MenuItem")
    if not item:
        print("  Work menu labels:",
              " | ".join(n.get("label") or "" for n in s.nodes()
                         if n.get("role") == "MenuItem")[:400])
        fail("no 'Save' item in the open File menu", s.app, s.mcp, s.log)
    if shot:
        s.shot(shot)
    state = bool(item.get("disabled"))
    close_menu()
    return state


def close_menu():
    for _ in range(2):
        s.call("inject_key", {"key": "Escape"})
        time.sleep(0.3)


def block_count(node_id):
    """The writing editor's block count, read straight from the a11y tree — a
    document-level fact, independent of whatever the dirty flag believes."""
    _, n = s.call("read_node", {"node": node_id})
    return len(n.get("children") or [])


# ── 1. Freshly loaded ⇒ clean ⇒ Save must be disabled ─────────────────────────
print("== clean project: Work ▸ Save must be greyed out ==")
disabled = save_item_disabled(shot="/tmp/save-item-clean.png")
print(f"  Save item: disabled={disabled}")
if not disabled:
    fail("Save is enabled on a freshly loaded (clean) project", s.app, s.mcp, s.log)

# ── 2. Edit a scene ⇒ unsaved ⇒ the same item must go live ────────────────────
print("== open 'Prologue' and edit its text ==")
prologue = s.find_exact("Prologue")
if not prologue:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
click(prologue)
time.sleep(1.2)

editors = [n for n in s.nodes()
           if n.get("role") == "MultilineTextInput" and "set_value" in (n.get("actions") or [])]
if not editors:
    fail("no writing editor after opening Prologue", s.app, s.mcp, s.log)
# The tallest one is the main writing column (the short one is the synopsis box).
main = max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))
# `type_text` (not `inject_key`): it focuses the node and delivers real text input,
# which is what drives RichTextEditor's `on_change` → OpenDoc::mark_dirty. Raw key
# events carry no character, so the editor would ignore them.
res, _ = s.call("type_text", {"node": main["id"], "text": "Typed by the automation check. "})
if isinstance(res, dict) and res.get("isError"):
    fail("type_text on the writing editor failed", s.app, s.mcp, s.log)
time.sleep(1.2)

print("== dirty project: Work ▸ Save must be enabled ==")
disabled = save_item_disabled(shot="/tmp/save-item-dirty.png")
print(f"  Save item: disabled={disabled}")
if disabled:
    fail("Save is still greyed out after an edit — the dirty signal never reached the menu",
         s.app, s.mcp, s.log)

# ── 3. Ctrl+S ⇒ written to disk ⇒ back to disabled ────────────────────────────
print("== Ctrl+S, then Save must fall back to disabled ==")
s.call("inject_key", {"key": "s", "ctrl": True})
time.sleep(2.5)  # save_work is a long operation
disabled = save_item_disabled()
print(f"  Save item: disabled={disabled}")
if not disabled:
    fail("Save stayed enabled after a successful save", s.app, s.mcp, s.log)

# ── 4. A *structural* edit must dirty the work too ────────────────────────────
# A lone Enter changes the block count but inserts no character, a case that
# has previously been missed and left Save unreachable for that edit. Assert
# the document really changed (block count, straight from the a11y tree) *and*
# that Save went live, so a null result can't pass silently.
print("== a lone Enter (block split, no character typed) must enable Save ==")
s.call("invoke_action", {"node": main["id"], "action": "focus"})
time.sleep(0.5)
before = block_count(main["id"])
s.call("inject_key", {"key": "Enter"})
time.sleep(1.5)
after = block_count(main["id"])
print(f"  blocks: {before} -> {after}")
if after == before:
    fail("the injected Enter never reached the editor — the check below would be vacuous",
         s.app, s.mcp, s.log)
disabled = save_item_disabled()
print(f"  Save item: disabled={disabled}")
if disabled:
    fail("a structural edit (Enter) did not mark the work unsaved — it is now UNSAVABLE; "
         "RichTextEditor's on_change is suppressing block-count changes again",
         s.app, s.mcp, s.log)

print("\nPASS: Save is disabled while there is nothing to save, enabled after an edit "
      "(typed text *and* a structural block split), and disabled again once written to disk.")
s.close()
shutil.rmtree(scratch, ignore_errors=True)
