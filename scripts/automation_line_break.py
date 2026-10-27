#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""**A line break inside a paragraph** (issue #217), end to end.

The headless tests prove each layer on its own: teksilo's editor turns
Shift+Enter into one line separator, `FormatViewModel` sends the menu row to the
same call, text-document saves the separator as Djot's hard break. What none of
them sees is the real window, where the key has to reach the editor through the
global shortcut router, the menu row runs with focus on the menu overlay, and
the find bar's own Shift+Enter ("previous match") has to keep working beside
the editor's. So this drives the app, in English, and proves six things:

  1. Shift+Enter in a scene starts a new line in the same paragraph, and Home,
     Up and End move along the lines of that paragraph (End stops before the
     break, or the words typed there would start the next line);
  2. Format ▸ Insert line break does the same from the menu, and gives the
     keyboard back to the editor;
  3. in the find bar, Shift+Enter goes to the previous match and writes nothing
     into the prose;
  4. under "Always forward", Shift+Enter still makes a line break, and the menu
     row goes in after a selection instead of replacing it;
  5. a save writes each of those paragraphs as one Djot paragraph holding a hard
     break (`\\` and a newline), read back from the saved bundle itself;
  6. reopened, each paragraph is still one paragraph with its two lines.

Run: `python3 scripts/automation_line_break.py`
"""

import base64
import json
import os
import re
import select
import subprocess
import sys
import tempfile
import time
import zipfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")
SHOTS = os.environ.get("SHOT_DIR", tempfile.mkdtemp(prefix="skribisto_line_break_"))
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# Each paragraph this probe writes, as its two lines. The Djot a save must hold
# for it is the two lines joined by a hard break.
STANZA = ("Roses are red, oh", "And violets are blue.")
FROM_MENU = ("Line typed before the menu", "line typed after it")
IN_GAME = ("Written in the game", "and its next line")
# Under the game, with "stay" selected when the menu row runs: the row goes in after
# the selection, as the key does, and takes no word away.
IN_GAME_MENU = ("Selected words stay", "after the break")
PARAGRAPHS = (STANZA, FROM_MENU, IN_GAME, IN_GAME_MENU)


def fail(msg, s=None):
    print("FAIL:", msg)
    if s is not None:
        try:
            print("--- app log tail ---")
            print("\n".join(open(s.log).read().splitlines()[-25:]))
        except Exception:
            pass
        s.close()
    sys.exit(1)


class Session:
    """One launched app plus the automation bridge, trimmed to what this needs."""

    def __init__(self, argv, env):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(
            argv, stdout=open(self.log, "w"), stderr=subprocess.STDOUT, env=env
        )
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=90)
        except RuntimeError as e:
            fail(str(e), self)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            self.mcp = subprocess.Popen(
                fixture.mcp_argv(bridge, MCP),
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=open(mcp_err, "w"),
                text=True,
                bufsize=1,
            )
            self._send(
                "initialize",
                {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "line-break", "version": "1"},
                },
            )
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect the automation bridge", self)
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

    def _recv(self, timeout=25, fatal=True):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                break
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
            if not r:
                continue
            line = self.mcp.stdout.readline()
            if not line:
                break
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if "id" in msg or "result" in msg or "error" in msg:
                return msg
        if fatal:
            fail("the bridge stopped answering", self)
        return None

    def call(self, tool, args=None):
        self._send("tools/call", {"name": tool, "arguments": args or {}})
        result = (self._recv() or {}).get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(
                c.get("text", "") for c in result.get("content", []) if c.get("type") == "text"
            ).strip()
            payload = json.loads(text) if (text.startswith("{") or text.startswith("[")) else {}
        return result, payload

    def settle(self, times=3):
        for _ in range(times):
            self.call("settle")

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", []) if isinstance(p, dict) else []

    def find(self, label, role=None, contains=False, timeout=10.0):
        end = time.time() + timeout
        while time.time() < end:
            for n in self.nodes():
                lab = n.get("label") or ""
                if role and n.get("role") != role:
                    continue
                if (label in lab) if contains else (lab == label):
                    return n
            time.sleep(0.4)
        return None

    def activate(self, node):
        """Fire the AccessKit action rather than a synthetic click: a press at a
        menu entry's centre does not open its submenu into the AT tree."""
        res, _ = self.call("invoke_action", {"node": node["id"], "action": "Click"})
        return not (isinstance(res, dict) and res.get("isError"))

    def click(self, node):
        b = node.get("bounds") or {}
        if "x" not in b:
            return False
        self.call(
            "inject_pointer",
            {
                "x": b["x"] + b.get("width", 0) / 2,
                "y": b["y"] + b.get("height", 0) / 2,
                "button": "primary",
            },
        )
        self.settle()
        return True

    def key(self, key, **mods):
        """A key press to the focused widget. `inject_key` takes no node: a call
        naming one is refused, and a probe that ignored the refusal would report
        on a key that never arrived."""
        res, _ = self.call("inject_key", {"key": key, **mods})
        if not res or res.get("isError"):
            fail(f"inject_key {key} {mods} was refused: {res}", self)
        self.settle(2)

    def type(self, node_id, text):
        self.call("type_text", {"node": node_id, "text": text})
        self.settle(3)

    def prose_editor(self, ns=None):
        """The manuscript editor, re-resolved every time: a tab rebuild re-mints
        node ids, so a cached one silently resolves to nothing. The editor with
        the most text is the manuscript, never the synopsis beside it."""
        ns = ns if ns is not None else self.nodes()
        eds = [
            n
            for n in ns
            if n.get("role") == "MultilineTextInput" and "set_value" in (n.get("actions") or [])
        ]
        return max(eds, key=lambda e: len(self.subtree_text(e["id"], ns)), default=None)

    @staticmethod
    def subtree_text(node_id, ns):
        """A RichTextEditor carries its prose in child nodes, not in `value`
        (the same walk as automation_formatting_marks.py)."""
        by_id = {n["id"]: n for n in ns if "id" in n}
        parts = []

        def walk(n):
            for k in ("label", "value", "name"):
                if n.get(k):
                    parts.append(str(n[k]))
                    break
            for cid in n.get("children") or []:
                child = by_id.get(cid)
                if child:
                    walk(child)

        node = by_id.get(node_id)
        if node:
            walk(node)
        return " ".join(parts)

    def prose_text(self):
        ns = self.nodes()
        ed = self.prose_editor(ns)
        return self.subtree_text(ed["id"], ns) if ed else ""

    def click_into_prose(self):
        """Put a real caret in the manuscript: `focus_node` gives focus but no
        caret, so only a pointer click does both (automation_writing_games.py
        explains the scroll)."""
        ed = self.prose_editor()
        if ed is None:
            fail("no manuscript editor on screen", self)
        for _ in range(6):
            n = next((x for x in self.nodes() if x.get("id") == ed["id"]), None)
            if ((n or {}).get("bounds") or {}).get("y", 0.0) <= 200.0:
                break
            self.call("scroll", {"node": ed["id"], "dx": 0, "dy": -400})
            self.settle(2)
        n = next((x for x in self.nodes() if x.get("id") == ed["id"]), ed)
        b = n.get("bounds") or {}
        self.call(
            "inject_pointer",
            {
                "x": b.get("x", 0.0) + min(b.get("width", 200.0) / 2, 200.0),
                "y": max(b.get("y", 0.0) + 24.0, 24.0),
                "button": "primary",
            },
        )
        self.settle(3)
        return self.prose_editor()

    def new_paragraph_at_end(self):
        """Caret to the end of the scene, then a paragraph of its own, so what
        this probe writes never runs into the example's prose."""
        self.click_into_prose()
        self.key("End", command=True)
        self.key("Enter")
        return self.prose_editor()

    def shot(self, name):
        path = os.path.join(SHOTS, name)
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")
                return path
        return None

    def close(self):
        for p in (getattr(self, "mcp", None), getattr(self, "app", None)):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=10)
        except Exception:
            self.app.kill()


def open_scene(s, title="Prologue"):
    """Open a scene from the binder. `load_work` is slow in a debug build, so the
    wait is generous; near the top of the tree on purpose, since `inject_pointer`
    aims at the node's bounds and a row below the fold is a click into nothing."""
    row = None
    end = time.time() + 120
    while time.time() < end and row is None:
        row = next(
            (
                n
                for n in s.nodes()
                if n.get("role") == "TreeItem" and (n.get("label") or "").strip() == title
            ),
            None,
        )
        if row is None:
            time.sleep(1.0)
    if row is None:
        fail(f"the binder never showed {title!r}", s)
    s.click(row)
    end = time.time() + 30
    while time.time() < end:
        if s.prose_editor() is not None:
            return
        time.sleep(0.8)
    fail(f"opening {title!r} showed no writing editor", s)


def write_two_lines(s, lines, between):
    """Type the first line, run `between` (which must make the line break), type
    the second.

    The second line's first character is a key to *whatever holds focus*, not to
    the editor by name: `type_text` focuses the node it is given, so a `between`
    that left focus on a menu would pass unnoticed if everything were typed with it.
    """
    ed = s.new_paragraph_at_end()
    s.type(ed["id"], lines[0])
    between()
    first = lines[1][0]
    s.key(first, text=first)
    ed = s.prose_editor()
    s.type(ed["id"], lines[1][1:])


def saved_djot(project):
    """Every Djot member of the saved bundle, concatenated. Read from the file on
    disk: the probe is about what a save writes, not about what the editor holds."""
    with zipfile.ZipFile(project) as z:
        return "\n".join(
            z.read(name).decode("utf-8") for name in z.namelist() if name.endswith(".djot")
        )


def hard_break(lines):
    return f"{lines[0]}\\\n{lines[1]}"


def editor_lines(s):
    """The manuscript as the accessibility tree reads it: one child of the editor
    per line, a line that ends in a line break carrying the U+2028 at its end."""
    ns = s.nodes()
    ed = s.prose_editor(ns)
    if ed is None:
        return []
    by_id = {n["id"]: n for n in ns if "id" in n}
    return [s.subtree_text(c, ns) for c in (by_id[ed["id"]].get("children") or [])]


def check_line_break(s, lines, what):
    """Fail unless `lines` read as two lines of one paragraph: the first ends in a
    line separator, and the second follows it. Two paragraphs would end the first
    line with no separator. The line nodes are joined first, so a soft wrap that
    splits a line across two nodes changes nothing. Retried for a few seconds: a
    reopened document reaches the accessibility tree after its editor mounts."""
    got = []
    end = time.time() + 10
    while time.time() < end:
        got = editor_lines(s)
        if lines[0] + "\u2028" + lines[1] in "".join(got):
            return
        time.sleep(0.5)
    near = [line for line in got if lines[0] in line or lines[1] in line]
    s.shot("line-break-wrong.png")
    fail(f"{what}: {lines!r} are not two lines of one paragraph; the editor reads {near!r}", s)


def find_count(s):
    """The find bar's "N of M" readout (`find-count`), as (N, M), or None. Fluent
    may wrap each number in isolation marks, which are dropped before matching."""
    for n in s.nodes():
        text = (n.get("label") or n.get("value") or "").replace("⁨", "").replace("⁩", "")
        m = re.fullmatch(r"(\d+) of (\d+)", text.strip())
        if m:
            return int(m.group(1)), int(m.group(2))
    return None


def tab_text_inputs(s):
    """The one-line fields inside the tab, right of the leading dock: opening the
    find banner adds exactly one (automation_find_banner_probe.py)."""
    return [
        n
        for n in s.nodes()
        if n.get("role") == "TextInput" and (n.get("bounds") or {}).get("x", 0) > 330
    ]


print("== A line break inside a paragraph ==")
env = fixture.isolated_config(locale="en-US", label="line-break", show_welcome=False)
project = fixture.working_copy(EXAMPLE, "line-break")
s = Session(fixture.launch_argv(project), env)

try:
    open_scene(s)
    print("Prologue open.")

    # ── 1. Shift+Enter, and moving along the lines ───────────────────────────
    def shift_enter():
        s.key("Enter", shift=True)

    ed = s.new_paragraph_at_end()
    s.type(ed["id"], "Roses are red,")
    shift_enter()
    ed = s.prose_editor()
    s.type(ed["id"], "violets are blue.")
    # Home goes to the start of the *line*, Up to the line above, End to the end
    # of that line, before the break: the two insertions below land in the right
    # line only if all three treat the break as a line end and not as a paragraph
    # end or as an ordinary character.
    s.key("Home")
    ed = s.prose_editor()
    s.type(ed["id"], "And ")
    s.key("Up")
    s.key("End")
    ed = s.prose_editor()
    s.type(ed["id"], " oh")
    check_line_break(s, STANZA, "Shift+Enter, Home, Up and End")
    s.shot("line-break-stanza.png")
    print("1. Shift+Enter made a second line; Home, Up and End moved along the lines.")

    # ── 2. Format ▸ Insert line break ────────────────────────────────────────
    def from_menu():
        # The project window folds its menu bar into a title-bar hamburger, so
        # Format is two clicks in (see automation_formatting_marks.py).
        ham = s.find("Menu", role="Button", timeout=10)
        if ham is None or not s.click(ham):
            fail("no title-bar 'Menu' (hamburger) button", s)
        time.sleep(0.7)
        fmt = s.find("Format", role="MenuItem", timeout=6)
        if fmt is None or not s.activate(fmt):
            fail("the Format menu would not open", s)
        time.sleep(0.7)
        row = s.find("Insert line break", role="MenuItem", contains=True, timeout=6)
        if row is None:
            seen = [n.get("label") for n in s.nodes() if n.get("role") == "MenuItem"]
            s.shot("line-break-menu.png")
            fail(f"no 'Insert line break' row in Format; saw {seen}", s)
        if not s.activate(row):
            fail("the Insert line break row would not activate", s)
        time.sleep(0.8)
        s.settle(3)

    write_two_lines(s, FROM_MENU, from_menu)
    check_line_break(s, FROM_MENU, "Format ▸ Insert line break")
    print("2. Format ▸ Insert line break made a second line from the menu.")

    # ── 3. The find bar keeps its own Shift+Enter ────────────────────────────
    s.click_into_prose()
    prose_before = s.prose_text()
    fields_before = {n["id"] for n in tab_text_inputs(s)}
    s.key("f", command=True)
    # The banner slides in on an animation that needs frames to run; a few
    # pointer moves give it them, as the find-banner probe does.
    for i in range(6):
        s.call("inject_pointer", {"x": 700.0 + i, "y": 400.0, "action": "move"})
        time.sleep(0.12)
    s.settle(3)
    field = next((n for n in tab_text_inputs(s) if n["id"] not in fields_before), None)
    if field is None:
        s.shot("line-break-find.png")
        fail("Ctrl+F opened no find field", s)
    s.type(field["id"], "the")
    s.key("Enter")
    s.key("Enter")
    forward = find_count(s)
    if forward is None or forward[0] < 2 or forward[1] < 3:
        s.shot("line-break-find.png")
        fail(f"the find bar shows no usable count after two Enters: {forward}", s)
    s.key("Enter", shift=True)
    back = find_count(s)
    if back is None or back[0] != forward[0] - 1 or back[1] != forward[1]:
        s.shot("line-break-find.png")
        fail(f"Shift+Enter in the find bar did not go back one match: {forward} -> {back}", s)
    s.key("Escape")
    if s.prose_text() != prose_before:
        fail("Shift+Enter in the find bar changed the prose", s)
    print(f"3. In the find bar, Shift+Enter went back a match ({forward} -> {back}).")

    # ── 4. "Always forward" keeps the line break ─────────────────────────────
    def text_of(n):
        return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))

    def toggle_game():
        node = next(
            (
                n
                for n in s.nodes()
                if n.get("role") in ("CheckBox", "Switch", "ToggleButton")
                and "forward" in text_of(n).lower()
                and n.get("bounds")
            ),
            None,
        )
        if node is None:
            fail("no 'Always forward' toggle", s)
        s.click(node)
        s.settle(3)

    def playing():
        # The dock's own status line, which reads the signal every frozen editor
        # reads; "deleting is disabled" is only on the playing line.
        return any("deleting is disabled" in text_of(n).lower() for n in s.nodes())

    # A pointer click on the rail tab: an activity-rail tab's a11y click does not
    # switch the side's visible panel (automation_writing_games.py).
    tab = next(
        (
            n
            for n in s.nodes()
            if n.get("role") == "Tab" and "Writing games" in text_of(n)
        ),
        None,
    )
    if tab is None or not s.click(tab):
        fail("no 'Writing games' rail tab", s)
    s.settle(3)
    toggle_game()
    if not playing():
        s.shot("line-break-game.png")
        fail("the game did not start", s)
    write_two_lines(s, IN_GAME, shift_enter)
    check_line_break(s, IN_GAME, "Shift+Enter under Always forward")

    def select_last_word_then_menu():
        for _ in IN_GAME_MENU[0].split()[-1]:
            s.key("Left", shift=True)
        from_menu()

    write_two_lines(s, IN_GAME_MENU, select_last_word_then_menu)
    check_line_break(s, IN_GAME_MENU, "the menu row over a selection under Always forward")
    toggle_game()
    if playing():
        fail("the game did not stop", s)
    # The binder back on the rail, as a writer leaves it: the reopened project
    # restores this layout, and step 6 opens the scene from the binder.
    tabs = [n for n in s.nodes() if n.get("role") == "Tab"]
    binder_tab = next((n for n in tabs if text_of(n).strip() == "Binder"), None)
    if binder_tab is None or not s.click(binder_tab):
        fail(f"no 'Binder' rail tab; saw {[text_of(n) for n in tabs]}", s)
    print(
        "4. Under Always forward, Shift+Enter made a second line, and the menu row"
        " went in after a selection."
    )

    # ── 5. The save ──────────────────────────────────────────────────────────
    s.click_into_prose()
    s.key("s", command=True)
    djot = ""
    end = time.time() + 30
    while time.time() < end:
        try:
            djot = saved_djot(project)
        except (OSError, zipfile.BadZipFile, KeyError):
            djot = ""
        if all(hard_break(p) in djot for p in PARAGRAPHS):
            break
        time.sleep(1.0)
    missing = [hard_break(p) for p in PARAGRAPHS if hard_break(p) not in djot]
    if missing:
        near = [
            djot[max(0, djot.find(p[1]) - 80) : djot.find(p[1]) + 40]
            for p in PARAGRAPHS
            if p[1] in djot
        ]
        fail(f"the save holds no hard break for {missing!r}; around them: {near!r}", s)
    print("5. The save wrote each paragraph with a Djot hard break.")
    s.close()

    # ── 6. Reopened ──────────────────────────────────────────────────────────
    s = Session(fixture.launch_argv(project), env)
    open_scene(s)
    for lines in PARAGRAPHS:
        check_line_break(s, lines, "reopened")
    s.shot("line-break-reopened.png")
    print("6. Reopened, each paragraph still holds its two lines.")

    print("\nPASS: Shift+Enter and the menu row make a line break that survives a save.")
finally:
    s.close()
