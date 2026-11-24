#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe for **dimming the text around the caret** (F8).

The headless tests pin the chain from the settings bundle to
`EditorHandle::get_caret_dim`, and teksilo and text-typeset test what the
typesetter draws. What only the running app can show is the whole path at
once: that F8 is a global shortcut the focused editor does not eat, that it
writes the setting every surface reads, that the title bar and the View menu
follow it, and above all that the fade reaches the pixels of a mounted editor,
follows the caret, steps aside for a selection and goes away again.

The fade is drawn by the editor's own typesetter and never reaches the
accessibility tree, so the pixel steps read screenshots: the darkest ink in
each paragraph's band of the window, on the light theme.

Asserts, against the running app:

  1. a scene with several paragraphs (the Prologue) opens and its editor takes
     focus, with the caret in its first paragraph;
  2. the title-bar toggle carries its "off" name, and the View menu row "Fade
     the rest of the text" is there and not checked;
  3. F8 (no modifier, from inside the editor) turns the fade on: the toggle
     takes its "on" name, "The rest of the text is faded" is announced,
     and the editor keeps the focus;
  4. in pixels, the first two paragraphs carry the same ink before F8; after
     it the caret's paragraph is unchanged and the other one is clearly
     lighter;
  5. moving the caret into the second paragraph (Down arrows) swaps which
     paragraph is light;
  6. a selection (Shift+Right) lifts the fade: both paragraphs at full ink;
     collapsing it brings the fade back;
  7. F8 again turns it off: full ink everywhere, the toggle back to its "off"
     name, and "All the text is at full strength" announced;
  8. the switch persisted: the sandbox's `general.toml` held
     `editor.dim_enabled = true` after the first F8 and `false` after the
     last.

How a paragraph's ink is measured: the editor reports one `TextRun` per
drawn line, with its bounds, and a scene is set with no space between
paragraphs and a first-line indent, so a paragraph starts at each line that
begins right of the editor's left edge. Its ink is the 2nd percentile of grey
over its line boxes (only the lines wholly inside the editor's scroll view):
the cores of the glyphs, which neither the width of a band nor the length of
its last line moves. On the light theme full-strength text measures about 14
and text faded at Medium about 211 (the renderer blends in linear light, so
the 0.35 opacity lands well above the 171 an sRGB blend would give). The
thresholds below (two paragraphs "the same" within 25 levels, "clearly
lighter" by at least 50) therefore sit far from both antialiasing noise and
the effect itself, and still hold at Light (about 180).

The announcements are what the bridge captures from the live regions
(`pull_announcements`), as a platform adapter is handed them; no screen reader
takes part.

The View menu row is read by opening Menu > View with the pointer, as
`automation_focus_not_on_the_window.py` does, and closed with Escape; only its
state before F8 is checked, since the row is reflect-only and the title-bar
toggle already shows the setting changed.

The screenshots go to `target/dim-probe/` in this checkout; their paths are
printed.

Run: python3 scripts/automation_dim_text.py [project.skrib]
"""
import base64
import io
import os
import subprocess
import sys
import tempfile
import time
import tomllib

from PIL import Image

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
fixture.assert_no_running_instance()
# Never open the checked-in fixture: autosave is real.
PROJECT = fixture.working_copy(SRC, label="dim-text")
# en-US so the names and announcements read the strings this probe knows; the
# light theme so "ink" means dark pixels on a pale page. The band behind the
# caret is pinned off so the only thing that changes between two shots is the
# fade, and typewriter scrolling off so moving the caret moves no paragraph.
PINS_DICT = {
    "ui.locale": "en-US",
    "ui.dark": False,
    "ui.show_welcome": False,
    "editor.dim_enabled": False,
    "editor.dim_scope": "Paragraph",
    "editor.dim_strength": "Medium",
    "editor.highlight_scope": "None",
    "editor.typewriter_scroll": False,
}
ENV = fixture.isolated_config(locale="en-US", label="dim-text", show_welcome=False)
PINS = fixture.config_pins_file(PINS_DICT, label="dim-text")
GENERAL_TOML = os.path.join(fixture.app_config_dir(ENV), "general.toml")
SHOTS = fixture.repo_path("target", "dim-probe")
os.makedirs(SHOTS, exist_ok=True)

NAME_OFF = "Click to fade the rest of the text (F8)"
NAME_ON = ("The rest of the text is faded. "
           "Click to show all of it at full strength (F8)")
MENU_ROW = "Fade the rest of the text"
SAID_ON = "The rest of the text is faded"
SAID_OFF = "All the text is at full strength"

#: Two paragraphs at full strength differ by less than this (grey levels).
SAME = 25
#: A faded paragraph is at least this much lighter than the caret's.
LIGHTER = 50

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=ENV)
session = None
seq = 0


def stop():
    if session:
        session.close()
    if app.poll() is None:
        app.terminate()
        try:
            app.wait(timeout=15)
        except subprocess.TimeoutExpired:
            app.kill()


def fail(message):
    print("FAIL:", message)
    print("--- app log tail ---")
    try:
        print("\n".join(open(log, encoding="utf-8", errors="replace").read().splitlines()[-25:]))
    except OSError:
        pass
    stop()
    sys.exit(1)


def call(tool, args=None):
    try:
        return session.call(tool, args)
    except fixture.McpError as e:
        fail(str(e))


def nodes():
    return call("snapshot_tree").get("nodes", [])


def settle(times=3):
    for _ in range(times):
        call("settle")


def text_of(n):
    return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))


def wait_for(pred, what, timeout=60.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(nodes())
        if hit:
            return hit
        time.sleep(0.3)
    fail(f"timed out waiting for {what}; focus is on {describe_focus(nodes())}")


def focused(ns):
    return [n for n in ns if n.get("focused")]


def describe_focus(ns):
    return [f"{n.get('role')} {text_of(n)[:50]!r}" for n in focused(ns)] or ["nothing"]


def prose_has_focus(ns):
    return any(n.get("role") == "MultilineTextInput" for n in focused(ns))


def heard():
    """What has been announced since the last call."""
    global seq
    settle(2)
    items = call("pull_announcements", {"since_seq": seq})
    if isinstance(items, dict):
        items = items.get("announcements", [])
    if items:
        seq = max(a["seq"] for a in items)
    return [a["text"] for a in items]


def expect_heard(needle, what):
    end = time.time() + 5
    got = []
    while time.time() < end:
        got += heard()
        if any(needle == t.strip() for t in got):
            print(f"OK  {what}: {needle!r}")
            return
        time.sleep(0.2)
    fail(f"{what}: expected {needle!r} among what was announced: {got}")


def key(name, times=1, **mods):
    for _ in range(times):
        call("inject_key", {"key": name, **mods})
        call("settle")
    settle(2)


def toggle_names(ns):
    """The accessible names of the title-bar dim toggle (either state)."""
    return [text_of(n).strip() for n in ns if n.get("role") == "Button"
            and text_of(n).strip() in (NAME_ON, NAME_OFF)]


def expect_toggle(name, what):
    got = wait_for(lambda ns: (lambda t: t if t == [name] else None)(toggle_names(ns)),
                   f"the title-bar toggle to read {name!r}", timeout=10)
    print(f"OK  {what}: {got[0]!r}")


def general_toml_value(want, what, timeout=10.0):
    """Wait until `general.toml` holds `editor.dim_enabled = want`."""
    end = time.time() + timeout
    got = None
    while time.time() < end:
        try:
            with open(GENERAL_TOML, "rb") as fh:
                data = tomllib.load(fh)
            got = data.get("editor", {}).get("dim_enabled")
        except (OSError, tomllib.TOMLDecodeError):
            got = None
        if got is want:
            print(f"OK  {what}: general.toml holds editor.dim_enabled = {str(want).lower()}")
            return
        time.sleep(0.25)
    fail(f"{what}: general.toml ({GENERAL_TOML}) holds editor.dim_enabled = {got!r}, "
         f"expected {want!r}")


# ── Pixels ──────────────────────────────────────────────────────────────────


def shot(name):
    """The window as the renderer draws it, saved under `name`, returned grey."""
    # Let a pending repaint land before grabbing: the fade is drawn on the next
    # frame after the setting changes.
    settle(3)
    time.sleep(0.4)
    try:
        res = session.request("tools/call", {"name": "screenshot", "arguments": {}})
    except fixture.McpError as e:
        fail(str(e))
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            data = base64.b64decode(c["data"])
            path = os.path.join(SHOTS, name)
            with open(path, "wb") as fh:
                fh.write(data)
            print(f"    screenshot -> {path}")
            return Image.open(io.BytesIO(data)).convert("L")
    fail("the bridge returned no screenshot")


def window_scale(img, ns):
    """Screenshot pixels per logical unit: the screenshot is the window in
    physical pixels, the tree is logical, and the widest node is the window."""
    widest = max((n.get("bounds", {}).get("width", 0) for n in ns), default=0)
    return img.width / widest if widest else 1.0


def paragraph_lines(ns):
    """The prose editor's first two paragraphs, as lists of line boxes.

    The editor reports one `TextRun` per drawn line, in document order, with
    its bounds. Scenes are set with no space between paragraphs and a
    first-line indent, so a paragraph starts at every line that begins right
    of the editor's left edge.
    """
    ed = prose_editor(ns)
    if ed is None:
        fail("no prose editor to measure")
    left = ed["bounds"]["x"]
    by_id = {n["id"]: n for n in ns if "id" in n}
    runs = [by_id[c] for c in ed.get("children") or []
            if c in by_id and by_id[c].get("role") == "TextRun" and by_id[c].get("bounds")]
    paras = []
    for r in runs:
        if r["bounds"]["x"] > left + 12 or not paras:
            if len(paras) == 2:
                break
            paras.append([])
        paras[-1].append(r)
    if len(paras) < 2:
        fail(f"need two paragraphs in the editor, found {len(paras)}")
    return paras


def pixel_boxes(img, ns, lines):
    """The line boxes of one paragraph in screenshot pixels, keeping only the
    lines wholly inside the visible part of the editor's scroll view."""
    s = window_scale(img, ns)
    ed = prose_editor(ns)["bounds"]
    views = [n.get("bounds") or {} for n in ns if n.get("role") == "ScrollView"]
    holding = [v for v in views
               if v.get("x", 0) <= ed["x"] + 1 and ed["x"] < v.get("x", 0) + v.get("width", 0)
               and v.get("y", 0) <= max(ed["y"], 0) + 1 < v.get("y", 0) + v.get("height", 0)]
    clip_top, clip_bottom = 0.0, img.height / s
    if holding:
        v = min(holding, key=lambda v: v.get("width", 0) * v.get("height", 0))
        clip_top, clip_bottom = v["y"], v["y"] + v["height"]
    boxes = []
    for r in lines:
        b = r["bounds"]
        if b["y"] < clip_top or b["y"] + b["height"] > clip_bottom:
            continue
        boxes.append((round(b["x"] * s), round(b["y"] * s),
                      round((b["x"] + b["width"]) * s), round((b["y"] + b["height"]) * s)))
    return boxes


def ink(img, boxes):
    """The 2nd percentile of grey over a paragraph's line boxes: the cores of
    its glyphs."""
    hist = [0] * 256
    for box in boxes:
        for v, n in enumerate(img.crop(box).histogram()):
            hist[v] += n
    target = sum(hist) * 0.02
    seen = 0
    for v, n in enumerate(hist):
        seen += n
        if seen >= target:
            return v
    return 255


def measure(name):
    """Screenshot the window and return the ink of the first two paragraphs.

    The line boxes are read from the tree right after the screenshot, so a
    paragraph is measured where it is drawn even if something scrolled.
    """
    img = shot(name)
    ns = nodes()
    paras = paragraph_lines(ns)
    boxes = [pixel_boxes(img, ns, p) for p in paras]
    for i, bx in enumerate(boxes):
        if not bx:
            fail(f"paragraph {i + 1} has no line on screen")
    return ink(img, boxes[0]), ink(img, boxes[1]), boxes


# ── The editor ──────────────────────────────────────────────────────────────


def editor_text(node_id, ns):
    by_id = {n["id"]: n for n in ns if "id" in n}
    parts = []

    def walk(n):
        for k in ("label", "value", "name"):
            if n.get(k):
                parts.append(str(n[k]))
                break
        for cid in n.get("children") or []:
            if (child := by_id.get(cid)) is not None:
                walk(child)

    if (node := by_id.get(node_id)) is not None:
        walk(node)
    return " ".join(parts)


def prose_editor(ns):
    """The manuscript editor: the editable one holding the most text."""
    best, best_len = None, -1
    for e in ns:
        if e.get("role") != "MultilineTextInput" or "set_value" not in (e.get("actions") or []):
            continue
        n = len(editor_text(e["id"], ns))
        if n > best_len:
            best, best_len = e, n
    return best if best_len > 200 else None


def menu_row_state():
    """Open Menu > View and return the "Fade the rest of the text" row's
    `toggled`, then close the menu with Escape."""
    ns = nodes()
    menu = [n for n in ns if n.get("role") == "Button" and text_of(n).strip() == "Menu"]
    if not menu:
        fail("no Menu button in the title bar")
    b = menu[0]["bounds"]
    call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                            "button": "primary"})
    # Ids and bounds are good only once the menu has finished opening.
    time.sleep(1.0)
    settle(3)
    view = wait_for(lambda ns: [n for n in ns if n.get("role") == "MenuItem"
                                and text_of(n).replace("&", "").strip() == "View"],
                    "the View menu")[0]["bounds"]
    call("inject_pointer", {"x": view["x"] + view["width"] / 2,
                            "y": view["y"] + view["height"] / 2, "button": "primary"})
    time.sleep(1.0)
    settle(3)
    row = wait_for(lambda ns: [n for n in ns if n.get("role", "").startswith("MenuItem")
                               and text_of(n).replace("&", "").strip().startswith(MENU_ROW)],
                   f"the View menu row {MENU_ROW!r}")[0]
    role, toggled = row.get("role"), str(row.get("toggled")).lower()
    for _ in range(3):
        call("inject_key", {"key": "Escape"})
        settle(2)
        if not [n for n in nodes() if n.get("role", "").startswith("MenuItem")]:
            break
    return role, toggled


# ── 1. Open the Prologue and put the caret in its first paragraph ───────────
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "dim-text-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    row = wait_for(lambda ns: [n for n in ns if n.get("role") == "TreeItem"
                               and text_of(n).strip().startswith("Prologue")],
                   "the binder to list the Prologue", timeout=90)[0]["bounds"]
    call("inject_pointer", {"x": row["x"] + row["width"] / 2, "y": row["y"] + row["height"] / 2,
                            "button": "primary"})
    ns = wait_for(lambda ns: ns if prose_editor(ns) else None, "the Prologue's prose editor")
    wait_for(prose_has_focus, "the Prologue's text to take the focus")
    # Ctrl+Home: the caret at the very start of the first paragraph, whatever
    # the restored caret was.
    key("Home", command=True)
    if not prose_has_focus(nodes()):
        fail(f"the editor lost the focus: {describe_focus(nodes())}")
    print("OK  the Prologue is open with the caret in its first paragraph")

    # ── 2. The off state, on both surfaces ──────────────────────────────────
    expect_toggle(NAME_OFF, "the title-bar toggle starts off")
    role, toggled = menu_row_state()
    if role != "MenuItemCheckBox":
        fail(f"the View menu row should be a check box, it is a {role}")
    if toggled == "true":
        fail("the View menu row is checked while the fade is off")
    print(f"OK  View > {MENU_ROW!r} is a check box, not checked (toggled={toggled})")
    # The menu took the focus; give it back to the text, still at its start.
    wait_for(prose_has_focus, "the focus to come back to the text after the menu", timeout=10)
    key("Home", command=True)
    heard()  # drop anything said so far

    # ── 4a. Before: both paragraphs at full ink ─────────────────────────────
    p1, p2, boxes = measure("dim-0-off.png")
    first_lines = len(paragraph_lines(nodes())[0])
    print(f"    paragraph 1: {len(boxes[0])} lines on screen of {first_lines}, "
          f"y {boxes[0][0][1]}-{boxes[0][-1][3]} px; "
          f"paragraph 2: {len(boxes[1])} lines on screen, y {boxes[1][0][1]}-{boxes[1][-1][3]} px")
    print(f"    off:              first {p1:3d}   second {p2:3d}")
    if abs(p1 - p2) > SAME:
        fail(f"before F8 the two paragraphs should carry the same ink: {p1} vs {p2}")
    print("OK  before F8 both paragraphs are at full ink")

    # ── 3. F8 turns it on, from inside the editor ───────────────────────────
    key("F8")
    expect_toggle(NAME_ON, "F8 turned the title-bar toggle on")
    expect_heard(SAID_ON, "F8 is announced")
    if not prose_has_focus(nodes()):
        fail(f"F8 took the focus out of the text: {describe_focus(nodes())}")
    print("OK  the text keeps the focus after F8")
    general_toml_value(True, "after the first F8")

    # ── 4b. After: the caret's paragraph unchanged, the other lighter ───────
    a1, a2, _ = measure("dim-1-on-first.png")
    print(f"    on, caret in 1st: first {a1:3d}   second {a2:3d}")
    if abs(a1 - p1) > SAME:
        fail(f"the caret's paragraph changed with the fade: {p1} -> {a1}")
    if a2 - a1 < LIGHTER:
        fail(f"the other paragraph is not clearly lighter: caret's {a1}, other {a2}")
    print("OK  the caret's paragraph keeps its ink, the next one fades")

    # ── 5. Into the second paragraph: the fade follows ──────────────────────
    key("Down", times=first_lines)
    if not prose_has_focus(nodes()):
        fail(f"Down took the focus out of the text: {describe_focus(nodes())}")
    b1, b2, _ = measure("dim-2-on-second.png")
    print(f"    on, caret in 2nd: first {b1:3d}   second {b2:3d}")
    if abs(b2 - p2) > SAME:
        fail(f"the second paragraph, now the caret's, is not at full ink: {p2} -> {b2}")
    if b1 - b2 < LIGHTER:
        fail(f"the first paragraph did not fade once the caret left it: first {b1}, second {b2}")
    print("OK  moving the caret into the second paragraph swaps which one is light")

    # ── 6. A selection lifts the fade ───────────────────────────────────────
    key("Right", times=4, shift=True)
    c1, c2, _ = measure("dim-3-selection.png")
    print(f"    selection up:     first {c1:3d}   second {c2:3d}")
    if abs(c1 - p1) > SAME or abs(c2 - p2) > SAME:
        fail(f"a selection should lift the fade: first {p1} -> {c1}, second {p2} -> {c2}")
    print("OK  a selection lifts the fade")
    key("Left")  # collapse
    d1, d2, _ = measure("dim-4-collapsed.png")
    print(f"    collapsed:        first {d1:3d}   second {d2:3d}")
    if d1 - d2 < LIGHTER:
        fail(f"collapsing the selection should bring the fade back: first {d1}, second {d2}")
    print("OK  collapsing the selection brings the fade back")

    # ── 7. F8 again turns it off ────────────────────────────────────────────
    key("F8")
    expect_toggle(NAME_OFF, "F8 turned the title-bar toggle off again")
    expect_heard(SAID_OFF, "the second F8 is announced")
    if not prose_has_focus(nodes()):
        fail(f"F8 took the focus out of the text: {describe_focus(nodes())}")
    e1, e2, _ = measure("dim-5-off-again.png")
    print(f"    off again:        first {e1:3d}   second {e2:3d}")
    if abs(e1 - p1) > SAME or abs(e2 - p2) > SAME:
        fail(f"with the fade off both paragraphs should be at full ink: {e1}, {e2}")
    print("OK  F8 again brings every paragraph back to full ink")

    # ── 8. The switch persisted ─────────────────────────────────────────────
    general_toml_value(False, "after the last F8")

    print(f"\nscreenshots in {SHOTS}")
    print("All dim-text assertions passed.")
finally:
    stop()
