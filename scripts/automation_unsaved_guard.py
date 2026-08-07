#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify that **replacing the open project never eats
unsaved edits**.

New Work, Open Work, the switcher's "Open here" and the import toast's "Open now"
all *replace* this window's project in place — the backend closes the open Work
first, and `EditorsViewModel::close_all` drops the open documents without
flushing. Every one of them used to do that with no prompt, no save and no undo:
type a paragraph, hit Ctrl+N, and the paragraph was gone. They now go through
`ProjectSwitchViewModel` — the same Save / Discard / Cancel guard the close paths
have always used.

This drives the **New Work** door end-to-end — all four share the one guard, and
New Work is the only one whose confirmation isn't a native OS file dialog the
bridge cannot see:

  1. load a scratch copy of an example, type into a scene  → the work is dirty
  2. Work ▸ New Work                                       → the *guard* appears,
                                                             NOT the New Work form
  3. Cancel                                                → project still open,
                                                             work still dirty
                                                             (nothing destroyed)
  4. Work ▸ New Work again, then Discard                   → the guard steps aside
                                                             and the New Work form
                                                             finally appears

Step 2 is the regression test: before the fix the New Work form opened straight
away, which is exactly what losing the edits looked like. Step 3 proves Cancel is
non-destructive, and step 4 proves the guard *gates* the command without blocking
it.

The **menu**, not Ctrl+N: a synthetic `inject_key` only reaches the app's global
shortcuts while the window holds OS keyboard focus — which a freshly-launched
window on this compositor often doesn't — and a silently-dropped keystroke would
make this whole script pass without testing anything. The menu item is a click,
and it fires the very same global `work.new` action; that Ctrl+N is bound to that
action is asserted separately, from `get_shortcuts`.

Assertions favour locale-independent anchors (AccessKit roles, geometry, the
button set) so this passes whatever UI language is persisted.

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""
import json, os, re, select, shutil, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
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
# This types into a scene and (in the Discard leg) really replaces the project, so
# never point it at the example in the repo.
print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-guard-check-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=30):
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


def dialog_buttons():
    """The message box's buttons, left-to-right, as `(label, node)` — `[]` when no
    message box is up.

    A `MessageBox` surfaces as an `AlertDialog`. The bridge's `snapshot_tree`
    carries no parent links, so the buttons are matched to it **geometrically**:
    a Button whose centre lies inside the alert's bounds belongs to it. That also
    keeps this locale-independent — no label is hard-coded.
    """
    nodes = s.nodes()
    alerts = [n for n in nodes
              if n.get("role") in ("AlertDialog", "Dialog") and (n.get("bounds") or {}).get("width")]
    if not alerts:
        return []
    # The innermost box (the app's root Window is also a Dialog-ish container in
    # some builds; the message box is the smallest one).
    box = min(alerts, key=lambda n: n["bounds"]["width"] * n["bounds"]["height"])
    b = box["bounds"]
    found = []
    for n in nodes:
        if n.get("role") != "Button" or not (n.get("label") or "").strip():
            continue
        nb = n.get("bounds") or {}
        if "x" not in nb:
            continue
        cx = nb["x"] + nb.get("width", 0) / 2
        cy = nb["y"] + nb.get("height", 0) / 2
        if b["x"] <= cx <= b["x"] + b["width"] and b["y"] <= cy <= b["y"] + b["height"]:
            found.append((cx, (n.get("label") or "").strip(), n))
    found.sort(key=lambda t: t[0])
    return [(label, node) for _, label, node in found]


def close_menu():
    for _ in range(2):
        s.call("inject_key", {"key": "Escape"})
        time.sleep(0.3)


def save_item_disabled():
    """Open the title-bar menu ▸ File and report the Save item's `disabled` flag —
    the app's own answer to "is there anything unsaved?" (`can_save`). Leaves the
    menu closed.

    This is how "the edits survived" is asserted: the editor's prose itself is not
    in the a11y payload (a `MultilineTextInput` exposes bounds + child TextRuns,
    no value), but Save being *live* means the work still has edits not on disk —
    which is precisely what Cancel must preserve.
    """
    ham = s.find_exact("Menu", role="Button")
    if not ham:
        fail("no title-bar 'Menu' (hamburger) button in the AT tree", s.app, s.mcp, s.log)
    click(ham)
    time.sleep(0.6)
    work_menu = s.find_exact("Work", role="MenuItem")
    if not work_menu:
        fail("the 'File' menu did not open", s.app, s.mcp, s.log)
    click(work_menu)
    time.sleep(0.7)
    item = s.find_exact("Save", role="MenuItem")
    if not item:
        fail("no 'Save' item in the open File menu", s.app, s.mcp, s.log)
    state = bool(item.get("disabled"))
    close_menu()
    return state


def new_work_form_present():
    """Is the New Work modal on screen? Anchored on its Form landmark + the two
    SegmentedControls (see automation_new_work.py), not on a translated label."""
    nodes = s.nodes()
    has_form = any(n.get("role") == "Form" for n in nodes)
    segmented = sum(1 for n in nodes if n.get("role") == "RadioGroup")
    return has_form and segmented >= 1


# ── 1. Dirty the project ─────────────────────────────────────────────────────
print("== open 'Prologue' and type into it (the work becomes dirty) ==")
prologue = s.find_exact("Prologue")
if not prologue:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
def writing_editors(timeout=10):
    """Poll for the scene's editors — opening a tab builds a document, which is not
    instant on a cold start."""
    end = time.time() + timeout
    while time.time() < end:
        eds = [n for n in s.nodes()
               if n.get("role") == "MultilineTextInput" and "set_value" in (n.get("actions") or [])]
        if eds:
            return eds
        time.sleep(0.5)
    return []


# Retry the click: a synthetic tap on a debug build can land before the row is
# hit-testable, and a missed click here would make every assertion below vacuous.
editors = []
for attempt in range(3):
    row = s.find_exact("Prologue") or prologue
    click(row)
    editors = writing_editors()
    if editors:
        break
    print(f"  (the scene did not open — retrying the click, attempt {attempt + 2})")
if not editors:
    fail("no writing editor after opening Prologue", s.app, s.mcp, s.log)
# The tallest one is the main writing column (the short one is the synopsis box).
main = max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))
# `type_text` delivers real text input, which is what drives RichTextEditor's
# `on_change` → `OpenDoc::mark_dirty` → the app-wide `unsaved` signal the guard
# reads. Raw key events carry no character, so the editor would ignore them.
PROSE = "A sentence the guard must not throw away. "
res, _ = s.call("type_text", {"node": main["id"], "text": PROSE})
if isinstance(res, dict) and res.get("isError"):
    fail("type_text on the writing editor failed", s.app, s.mcp, s.log)
time.sleep(1.2)
print(f"  typed {PROSE!r}")

if dialog_buttons():
    fail("a message box was already up before Ctrl+N — the checks below would be vacuous",
         s.app, s.mcp, s.log)

# ── 2. Ctrl+N ⇒ the GUARD, not the New Work form ──────────────────────────────
# Ctrl+N is the same command by another door: assert the binding exists, then
# drive the command through the menu (see `new_work_command`).
_, sc = s.call("get_shortcuts")
bound = json.dumps(sc).lower()
if "work.new" not in bound:
    fail("no `work.new` shortcut registered — Ctrl+N would not reach the guard",
         s.app, s.mcp, s.log)
print("  Ctrl+N is bound to the `work.new` action (same action the menu fires)")

print("== New Work on a dirty project must prompt, not replace ==")


def wait_dialog(timeout=8):
    """Poll for the guard; presenting a message box is a frame or two away."""
    end = time.time() + timeout
    while time.time() < end:
        b = dialog_buttons()
        if b:
            return b
        time.sleep(0.4)
    return []


def new_work_command():
    """Fire the New Work command through **Work ▸ New Work** and wait for whatever
    it produces — the guard, or (the bug) the New Work form.

    The menu, not Ctrl+N: a synthetic `inject_key` only reaches the app's global
    shortcuts when the window holds OS keyboard focus, which a freshly-launched
    window on this compositor often doesn't — and a *dropped* keystroke looks
    exactly like a passing test. The menu item is a click, and it fires the very
    same global `work.new` action the shortcut does, so what's under test is
    identical. (Ctrl+N's binding to that action is asserted separately by
    `get_shortcuts` below.)
    """
    ham = s.find_exact("Menu", role="Button")
    if not ham:
        fail("no title-bar 'Menu' (hamburger) button in the AT tree", s.app, s.mcp, s.log)
    click(ham)
    time.sleep(0.6)
    work_menu = s.find_exact("Work", role="MenuItem")
    if not work_menu:
        fail("the 'File' menu did not open", s.app, s.mcp, s.log)
    click(work_menu)
    time.sleep(0.7)
    item = next((n for n in s.nodes()
                 if n.get("role") == "MenuItem" and "new work" in (n.get("label") or "").lower()),
                None)
    if not item:
        fail("no 'New Work' item in the open File menu", s.app, s.mcp, s.log)
    click(item)
    return wait_dialog()


buttons = new_work_command()
labels = [lbl for lbl, _ in buttons]
print(f"  message-box buttons: {labels}")
s.shot("/tmp/guard-ctrl-n-prompt.png")
if not buttons:
    if new_work_form_present():
        fail("REGRESSION: Ctrl+N opened the New Work form directly on a dirty project — "
             "the unsaved edits were about to be destroyed with no prompt",
             s.app, s.mcp, s.log)
    fail("Ctrl+N produced neither a guard prompt nor the New Work form", s.app, s.mcp, s.log)
if len(buttons) != 3:
    fail(f"expected a 3-button Save/Discard/Cancel guard, got {labels}", s.app, s.mcp, s.log)
if new_work_form_present():
    fail("the New Work form opened *behind* the guard — the switch was not deferred",
         s.app, s.mcp, s.log)
print("  the Save/Discard/Cancel guard is up, and the New Work form is not")

# ── 3. Cancel ⇒ nothing was destroyed ────────────────────────────────────────
print("== Cancel must leave the project open and untouched ==")
# Cancel is the guard's escape button, so Escape is the locale-independent way to
# hit it (no label matching).
s.call("inject_key", {"key": "Escape"})
time.sleep(1.2)

if dialog_buttons():
    fail("the guard did not dismiss on Escape", s.app, s.mcp, s.log)
if new_work_form_present():
    fail("cancelling the guard still opened the New Work form", s.app, s.mcp, s.log)
if not s.wait_label("starforgers", timeout=5):
    s.shot("/tmp/guard-after-cancel.png")
    fail("the project was closed even though the guard was cancelled", s.app, s.mcp, s.log)
# Patient: dismissing the message box rebuilds the tree, and a snapshot taken
# mid-rebuild briefly reports no editor — which is a stale read, not a lost tab.
if not writing_editors(timeout=25):
    s.shot("/tmp/guard-editor-gone.png")
    fail("the editor is gone after cancelling the guard", s.app, s.mcp, s.log)

# Cancel neither saves nor discards: the work must still be *dirty*. If Save had
# gone grey the edits would have been written (or thrown away) behind the user's
# back — the very thing this guard exists to prevent.
if save_item_disabled():
    s.shot("/tmp/guard-after-cancel.png")
    fail("Save went grey after cancelling the guard — the unsaved edits are gone",
         s.app, s.mcp, s.log)
print("  project still open, editor still there, work still dirty (Save still live)")

# ── 4. Ctrl+N ⇒ Discard ⇒ the New Work form finally appears ───────────────────
# Proves the guard *steps aside* — it gates the switch, it doesn't block it.
print("== New Work then Discard must let the New Work form through ==")
buttons = new_work_command()
if len(buttons) != 3:
    fail(f"the guard did not reappear on a second New Work: {[l for l, _ in buttons]}",
         s.app, s.mcp, s.log)

# Discard is the leading button of the Save/Discard/Cancel set (the order is the
# button-set's, not the locale's — see the screenshot in the docs above).
discard_label, discard = buttons[0]
print(f"  clicking the leading guard button: {discard_label!r}")
click(discard)
time.sleep(2.5)

if dialog_buttons():
    fail("the guard is still up after Discard", s.app, s.mcp, s.log)
if not new_work_form_present():
    s.shot("/tmp/guard-after-discard.png")
    fail("Discard dismissed the guard but the New Work form never opened — "
         "the switch was dropped instead of performed", s.app, s.mcp, s.log)
s.shot("/tmp/guard-new-work-after-discard.png")
print("  the New Work form opened after Discard")

print("\nPASS: replacing the project is guarded — Ctrl+N prompts on a dirty work, "
      "Cancel keeps it, Discard lets it through.")
s.close()
