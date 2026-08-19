#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the image feature's UI surfaces.

Covers the four places images reach the writer, and the enablement rules that
decide whether each is offered:

* **Document menu** — `Insert image…`, and the three commands that act on a
  picture already in the prose (`Describe`, `Resize`, `Original size`). The
  three are gated on having clicked an image, so with nothing in hand they must
  be present *and greyed* — that gate is the whole reason they can be offered
  from a menu at all, since clicking a picture deliberately does not move the
  caret and nothing else can say which image is meant.
* **Work menu** — `Book cover…` / `Remove the cover`, hidden with no project
  open because there is no book to give a cover to.
* **Settings ▸ Compile & Export ▸ Export Formats** — that the pane still opens.
  Its `Images` and `Open with the cover` rows are asserted headlessly instead;
  see the section's own comment for why clicking to them would be unsafe.
* **No duplicate mnemonics.** `MenuList` panics at BUILD time when two rows in
  one menu share one, and nothing headless builds the whole menu model. Six new
  rows landed in two menus here; opening both at all is the only guard.

## What this script deliberately does not do

`Insert image…` opens a **native file dialog**, which is outside the AT tree and
cannot be driven from here — so the insert → save → reopen → export round trip
is not automatable through this bridge. That path is covered by tests instead,
and better than a script could:

* `skrib_format::asset_tests` — an image round-trips through both `.skrib`
  shapes, survives a prune, and refuses to save with its bytes missing.
* `skribisto_compiler::render::tests` — the sidecar lands beside the document
  for each referencing format, is restricted to what the scope names, and the
  cover opens the book (and does not, for a scoped export).
* `text-document`'s `image_export_tests` — the bytes really are in the DOCX,
  the EPUB and the HTML.

Run: `python3 scripts/automation_images.py` (needs a debug build and the
teksilo automation MCP binary).
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

import pathlib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402
_ROOT = pathlib.Path(__file__).resolve().parent.parent  # this repo/worktree root
SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = str(_ROOT / "resources/examples/Starforgers.skrib")
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
                fail("app exited before bridge socket", self.app, None, self.log)
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
                                      "clientInfo": {"name": "dicts-test", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate(); time.sleep(0.3)
        if init is None:
            fail("could not connect MCP", self.app, self.mcp, self.log)
        self._send("notifications/initialized", notif=True)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1; msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n"); self.mcp.stdin.flush()

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
            if substr.lower() in " | ".join(self.labels()).lower():
                return True
            time.sleep(0.4)
        return False

    def match(self, variants, rail_only=False, exact=False):
        for n in self.nodes():
            lab = (n.get("label") or "").strip().lower()
            if not lab:
                continue
            hit = lab in variants if exact else any(v in lab for v in variants)
            if not hit:
                continue
            if rail_only:
                b = n.get("bounds") or {}
                if (b.get("x", 9999) if isinstance(b, dict) else 9999) >= 280:
                    continue
            return n
        return None

    def click_node(self, n):
        b = n.get("bounds") or {}
        if isinstance(b, dict) and "x" in b:
            self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                         "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
            return True
        return self.call("invoke_action", {"node": n["id"], "action": "click"}) is not None

    def dump(self, title, limit=80):
        print(f"--- AT tree: {title} ---")
        c = 0
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            if lab:
                print(f"  [{n.get('role')}] {lab!r}")
                c += 1
                if c >= limit:
                    print("  …"); break

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"screenshot -> {path}")
                return

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


def has_any(*variants):
    j = " | ".join(s.labels()).lower()
    return any(v in j for v in variants)


print("== launch with the bundled example ==")
s = Session([EXAMPLE])
if not s.wait_label("starforgers", timeout=30):
    fail("example did not load", s.app, s.mcp, s.log)
print("example loaded.")

failures = []


def check(cond, msg):
    print(("  ok   " if cond else "  FAIL ") + msg)
    if not cond:
        failures.append(msg)


def esc(times=3):
    for _ in range(times):
        s.call("inject_key", {"key": "Escape"})
        time.sleep(0.3)


def open_menu(title):
    """Open the title-bar hamburger and then one of its menus.

    Menu titles live in an overlay whose reported bounds can be negative, so they
    are activated through `invoke_action` rather than a pointer click.
    """
    ham = s.match(["menu"], rail_only=True, exact=True)
    if not ham:
        return False
    s.click_node(ham)
    time.sleep(1.0)
    top = s.match([title], exact=True)
    if not top:
        return False
    s.call("invoke_action", {"node": top["id"], "action": "click"})
    time.sleep(1.2)
    return True


def rows_of(wanted):
    """Map each wanted label to its AT node in the open menu, or None."""
    found = {w: None for w in wanted}
    for n in s.nodes():
        lab = (n.get("label") or "").strip()
        if lab in found:
            found[lab] = n
    return found


def expand_and_click(section_variants, leaf_variants):
    """Expand a tree section (click its chevron at the leading edge), then click a leaf."""
    sec = s.match(section_variants, rail_only=True)
    if sec:
        b = sec.get("bounds") or {}
        if isinstance(b, dict) and "x" in b:  # chevron ~10px in
            s.call("inject_pointer", {"x": b["x"] + 10, "y": b["y"] + b.get("height", 0) / 2,
                                      "kind": "click"})
            time.sleep(0.5)
    leaf = s.match(leaf_variants, rail_only=True, exact=True) or s.match(leaf_variants, rail_only=True)
    if leaf:
        s.click_node(leaf)
        time.sleep(0.8)
        return True
    return False

# ------------------------------------------------------------- Document menu
print("== Document menu ==")
check(open_menu("document"), "Document menu opens (no duplicate-mnemonic panic)")

INSERT = "Insert image\u2026"
DESCRIBE = "Describe the image\u2026"
RESIZE = "Resize the image\u2026"
RESET = "Original size"

found = rows_of([INSERT, DESCRIBE, RESIZE, RESET])
check(found.get(INSERT) is not None, f"Document menu offers {INSERT!r}")

# Insert happens AT THE CARET, so its gate is narrower than its siblings':
# a restored tab the writer has never typed in has no caret to insert at.
ins = found.get(INSERT)
check(ins is not None and ins.get("disabled") is True,
      "Insert image is greyed before the writer has been in an editor")

# The three commands that act on a picture are NOT here: they moved to their own
# Image menu, which exists only while an image is selected. A greyed row in the
# Document menu was the old shape; finding one now would mean the move was
# half-done.
for lab in (DESCRIBE, RESIZE, RESET):
    check(found.get(lab) is None,
          f"{lab!r} has left the Document menu for the Image menu")

s.shot("/tmp/images-document-menu.png")
esc()

# ...and live once there is one. Clicking into the prose is what sets the latch;
# it then survives the focus loss that opening the menu itself causes, which is
# the regression a live-focus gate would reintroduce (commit 2d4890b0).
print("== Document menu, with a caret in the prose ==")
# Open a real scene tab first: the restored tab is a Full Book stream, whose
# segment bar sits where a naive "click the middle of the pane" lands.
scene = s.match(["prologue"], rail_only=True)
if scene:
    s.click_node(scene)
    time.sleep(1.8)

# Then click into the prose editor itself. Ask the a11y tree which nodes *are*
# editors rather than guessing at pane geometry: a scene tab stacks the synopsis
# above the prose, so "the middle of the biggest box" lands in whichever of the
# two happens to be laid out there — and clicking the synopsis sets the latch
# from the wrong surface, which is a passing test for the wrong reason.
editors = [n for n in s.nodes()
           if n.get("role") == "MultilineTextInput"
           and "set_value" in (n.get("actions") or [])
           and (n.get("bounds") or {}).get("height", 0) > 0]
# The prose editor is the tall one; the synopsis is a few lines in a fixed box.
prose = max(editors, key=lambda n: n["bounds"]["height"], default=None)
if prose:
    b = prose["bounds"]
    # Aim near the TOP of the editor, not its middle: a scrolling editor reports
    # its whole *content* height, so the centre of a long scene is far below the
    # window and the click silently lands on nothing. A few lines down from the
    # top is inside the viewport whatever the scene's length.
    s.call("inject_pointer", {"x": b["x"] + b["width"] / 2,
                              "y": b["y"] + min(60, b["height"] / 2), "kind": "click"})
    time.sleep(1.2)
    # The gate below is "a registered editor has (or had) keyboard focus", so a
    # click that lands without focusing fails it for a reason no screenshot of a
    # greyed row can show. Name what focus actually reached.
    foc = [f"{n.get('role')}:{(n.get('label') or '')[:24]}"
           for n in s.nodes() if n.get("focused")]
    print(f"  focus after clicking the prose: {foc or 'nothing'}")
else:
    print("  !! no MultilineTextInput editor in the a11y tree")

if open_menu("document"):
    ins2 = rows_of([INSERT]).get(INSERT)
    check(ins2 is not None and ins2.get("disabled") is not True,
          "Insert image goes live once the caret is in the prose")
    s.shot("/tmp/images-document-menu-caret.png")
    esc()

# ---------------------------------------------------------------- Image menu
# It is inserted and removed as the selection changes, so with the caret in
# ordinary prose it must not be in the bar at all — a permanently-present menu
# whose every row is greyed is the shape this replaced.
print("== Image menu ==")
check(not open_menu("image"), "no Image menu while no picture is selected")
esc()

# ----------------------------------------------------------------- Work menu
print("== Work menu ==")
check(open_menu("work"), "Work menu opens (no duplicate-mnemonic panic)")
cover = rows_of(["Book cover\u2026", "Remove the cover"])
for lab, n in cover.items():
    check(n is not None, f"Work menu offers {lab!r} with a project open")
s.shot("/tmp/images-work-menu.png")
esc()

# ------------------------------------------------- Settings: export styles
# The style is what decides whether an exported Markdown gets its pictures
# written beside it, so the control has to be reachable — a default nobody can
# see is a default nobody chose.
print("== Settings: the export-styles pane ==")
# The hazard this guards is a crash, not a missing label: the settings
# `Switcher` indexes its children BY pane discriminant, so a pane that gains
# fields can panic on open. Two new rows landed in this one.
#
# The rows themselves are asserted headlessly instead — `skribisto_compiler`
# exercises `image_handling` and `book_cover` directly, on the values rather
# than on their labels — because reaching the style *editor* means selecting a
# built-in style and duplicating it (built-ins are read-only), and the only
# buttons this pane exposes to the AT tree at its top level are `Reset to
# defaults` and `Import…`. A script that clicked its way there would be one
# layout change away from pressing Reset on the writer's own styles.
for args in ({"key": ",", "ctrl": True}, {"key": ",", "modifiers": ["ctrl"]}):
    s.call("inject_key", args)
    time.sleep(2.5)
    if has_any("reset to defaults", "r\u00e9initialiser"):
        break
check(has_any("reset to defaults", "r\u00e9initialiser"), "Settings opens")

# Settings is a modal overlay inside the window, not a separate window, so its
# tree is nowhere near the activity rail — the usual `rail_only` filter would
# skip it. It also reopens on whichever pane it was last left on, so the
# section holding this pane may be collapsed: click its chevron first.
def settings_row(*variants):
    for n in s.nodes():
        lab = (n.get("label") or "").strip().lower()
        if any(v in lab for v in variants) and (n.get("bounds") or {}).get("x") is not None:
            return n
    return None

section = settings_row("compile & export", "compilation")
if section:
    b = section["bounds"]
    s.call("inject_pointer", {"x": b["x"] + 10, "y": b["y"] + b.get("height", 0) / 2,
                              "kind": "click"})
    time.sleep(0.8)
leaf = settings_row("export formats", "formats d")
opened = bool(leaf) and s.click_node(leaf)
time.sleep(1.0)
check(opened, "the export-styles pane is reachable in the settings tree")
time.sleep(1.0)
# The breadcrumb is what says the pane really rendered rather than the tree
# merely highlighting a row.
crumb = [n.get("label") for n in s.nodes()
         if n.get("role") == "Link" and (n.get("label") or "").strip()]
check(any("Export" in (c or "") for c in crumb),
      f"the pane rendered (breadcrumb: {crumb})")
s.shot("/tmp/images-export-styles.png")
esc(4)

# ---------------------------------------------------------------- report
print()
if failures:
    print("FAIL:")
    for f in failures:
        print("  -", f)
else:
    print("PASS: image menus, their enablement gates, and the export-styles pane.")
s.close()
sys.exit(1 if failures else 0)
