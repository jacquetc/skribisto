#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the worktree build to verify the note-template feature end to end:

the Settings > Work > Templates pane (empty state, preset menu, a preset applied),
and the Document menu (its five binder rows plus the two template rows, gated on
whether the caret is in a note).

The mnemonic clash that took the app down on first launch is exactly the class of bug
only a real run catches: `MenuList` panics at build time on two items sharing a
mnemonic, and no headless test builds the whole menu model. Opening the Document menu
here is that regression guard.

Deliberately NOT covered here: that the two template rows are *enabled* when the caret
sits in a note's prose. Staging that state through the bridge proved unreliable — a note
created from the binder is selected but its tab is not opened, so the probe kept
asserting against whatever chapter happened to be open and failing for a reason that had
nothing to do with the gate. A check that red-flags its own staging is worse than none.
The gate's real hazard — that it must survive the focus loss of opening the menu, since
`FormatViewModel::surface` drops to `None` there — is pinned by
`note_focused_survives_the_focus_loss_of_opening_a_menu` and
`a_real_change_of_surface_still_clears_the_gate` in `view_models/format.rs`."""
import base64, json, os, re, select, subprocess, sys, tempfile, time

import pathlib
_ROOT = pathlib.Path(__file__).resolve().parent.parent  # this repo/worktree root
SKRIBISTO = str(_ROOT / "target/debug/skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
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


def settings_open():
    return has_any("reset to defaults", "réinitialiser") and has_any("done", "terminé")


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


def esc():
    for _ in range(4):
        s.call("inject_key", {"key": "Escape"})
        time.sleep(0.3)


def open_menu(title):
    """Open the title-bar hamburger and then one of its menus.

    Menu titles live in an overlay whose reported bounds can be negative, so they are
    activated through `invoke_action` rather than a pointer click.
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


# ---------------------------------------------------------------- Document menu
# Opening it at all is half the point. `MenuList` panics at BUILD time when two items
# share a mnemonic, and nothing headless builds the whole menu model — so this is the
# only guard against that. It fired for real: `&Indent` collided with `&Insert
# template`, and `Move to &Trash` with `Save as &template`, in en-US; French had a
# three-way clash. The app died on launch.
print("== Document menu ==")
check(open_menu("document"), "Document menu opens (no duplicate-mnemonic panic)")
rows = {
    "Rename": None,
    "Duplicate": None,
    "Indent": None,
    "Outdent": None,
    "Insert template": None,
    "Save as template\u2026": None,
    "Move to Trash": None,
}
for n in s.nodes():
    lab = (n.get("label") or "").strip()
    if lab in rows:
        rows[lab] = n
for lab, n in rows.items():
    check(n is not None, f"Document menu offers {lab!r}")
s.shot("/tmp/templates-document-menu.png")

# With no binder selection and a chapter open, the item rows are greyed and the
# template rows are greyed for a different reason: a chapter is not a note.
# Templates are not restricted to notes any more — a scene, a synopsis, a corkboard
# card all qualify — so the gate is simply "is there an editor to act on"
# (`FormatViewModel::has_target`, which is also sticky, so it survives opening this
# menu). With a project open and a tab restored, that is satisfied.
sat = rows.get("Save as template\u2026")
check(sat is not None and sat.get("disabled") is not True,
      "Save as template is enabled with an editor open, whatever kind it is")
s.call("inject_key", {"key": "Escape"})
time.sleep(0.6)

# Selecting a binder row enables the five item commands, and only those.
ch = s.match(["chapter 3"])
if ch:
    s.click_node(ch)
    time.sleep(1.5)
if open_menu("document"):
    state = {}
    for n in s.nodes():
        lab = (n.get("label") or "").strip()
        if lab in rows:
            state[lab] = n.get("disabled")
    for lab in ("Rename", "Duplicate", "Indent", "Outdent", "Move to Trash"):
        check(state.get(lab) is not True, f"{lab} is enabled with a binder selection")
    # The loosening, stated directly: a *chapter* is a scene, not a note, and the row
    # is live all the same. This check is the inverse of the one it replaced.
    check(state.get("Save as template\u2026") is not True,
          "Save as template is enabled over a chapter — no longer note-only")
    s.shot("/tmp/templates-document-menu-selected.png")
    s.call("inject_key", {"key": "Escape"})
    time.sleep(0.5)

# ------------------------------------------------------------------- Settings
# The other crash this caught: the settings `Switcher` indexes its children BY pane
# discriminant, and the Templates pane was first inserted next to Tags (where it reads
# naturally) rather than at its own discriminant. Opening Settings panicked.
print("== Settings ==")
for args in ({"key": ",", "ctrl": True}, {"key": ",", "modifiers": ["ctrl"]}):
    s.call("inject_key", args)
    time.sleep(2.5)
    if has_any("reset to defaults", "r\u00e9initialiser"):
        break
check(has_any("reset to defaults", "r\u00e9initialiser"),
      "Settings opens (no Switcher discriminant panic)")
check(s.match(["templates", "mod\u00e8les"], exact=True) is not None,
      "a Templates page is present in the settings tree")
s.shot("/tmp/templates-settings-tree.png")

print()
if failures:
    print(f"FAILED {len(failures)} check(s):")
    for f in failures:
        print("  -", f)
    s.close()
    sys.exit(1)
print("all checks passed.")
s.close()
print("done.")
