#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the real Skribisto and check the **Scratchpad** dock end to end.

Each binder item has a private scratchpad, shown in a dock on the trailing rail
(Ctrl+Shift+A). It follows the item in front of the writer and, in a Full Chapter
stream, the row the caret was last in. It is saved with the project, never
exported, and read-only on a trashed item. The view-model and the model have
their headless tests; what only a live window shows is that the dock, the editor,
the formatting commands and the save are wired to each other:

  a. Ctrl+Shift+A shows the dock over scene A; its header
     names A; its editor (MultilineTextInput, in the trailing area) is empty.
  b. Typing into it keeps the note with A: opening scene B shows B's own, empty
     scratchpad; B takes its note; back on A the editor reads A's.
  c. Ctrl+B with the caret in the scratchpad formats the scratchpad, not the
     manuscript: A's text, and A's Djot once saved, carry no new markup.
  d. In a chapter's Full Chapter view the dock follows the row the caret is in
     (the chapter's own row, then a scene row), and clicking into the scratchpad
     leaves it on that row.
  e. Ctrl+S, quit, and relaunch on the same file: A's scratchpad still holds its
     note, and the bundle holds a `*.scratchpad.djot` with it (and no scene file
     holds it).
  f. A scene moved to the Trash and shown from the Trash dock has a read-only
     scratchpad: typing into it changes nothing, and it says why.
  g. With every tab closed the dock shows its no-item message.

The window is driven through the automation bridge: snapshot_tree, invoke_action,
inject_pointer, inject_key, type_text, settle. The real backend is used (a build
with `--features mocks` loads "Mock Project" and this fails at the first step), on
a working copy of the bundled Starforgers example in a private settings sandbox,
with `--new-instance`, so it never meets the operator's own Skribisto.

Run (after `cargo build -p teksilo_ui`):

    python3 scripts/automation_scratchpad.py [project.skrib]

Exit 0 when every check passes and 1 on the first failure (with the app log tail). Set `SHOT_DIR` to keep screenshots.
"""
import os
import re
import signal
import subprocess
import sys
import tempfile
import time
import zipfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
SHOTS = os.environ.get("SHOT_DIR", tempfile.mkdtemp(prefix="skribisto_scratchpad_"))

# The example's scenes, by title. A and B are two ordinary text items.
A, B = "Prologue", "Chapter 1"
# A chapter turned into a folder for the Full Chapter check, and its two scenes.
CHAPTER = "Chapter 22"
NEW_SCENE = "New Scene"
NOTE_A, NOTE_B, NOTE_C = "alpha note", "beta note", "gamma note"

# en-US `main.ftl`.
HEADER = "Scratchpad · "
NO_ITEM = "Open an item to keep your notes beside it."
TRASHED = "This item is in the Trash, so its notes are read-only."

PROJECT = fixture.working_copy(SRC, label="scratchpad")
ENV = fixture.isolated_config(locale="en-US", label="scratchpad", show_welcome=False,
                              pins={"editor.autosave": False})
PINS = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.show_welcome": False, "editor.autosave": False},
    label="scratchpad")
LOG = ""

app = None
session = None
launches = 0


# ---------------------------------------------------------------------------
# Plumbing
# ---------------------------------------------------------------------------

def stop():
    global app, session
    if session is not None:
        session.close()
        session = None
    if app is not None and app.poll() is None:
        app.kill()
        app.wait(timeout=15)


def fail(message):
    print("FAIL:", message)
    if session is not None:
        shot("scratchpad-failure")
    print(f"--- app log tail ({LOG}) ---")
    print("\n".join(open(LOG, encoding="utf-8", errors="replace").read().splitlines()[-25:]))
    stop()
    sys.exit(1)


def check(ok, what, detail=""):
    if ok:
        print("PASS:", what)
    else:
        fail(what + (f"\n  {detail}" if detail else ""))


def launch():
    """Start the app on the working copy and attach the bridge."""
    global app, session, launches
    global LOG
    launches += 1
    # A log of its own per launch: the bridge announce of the first would answer the second.
    LOG = tempfile.NamedTemporaryFile(prefix=f"scratchpad-{launches}-", suffix=".log",
                                      delete=False).name
    log = open(LOG, "w")
    app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), env=ENV,
                           stdout=log, stderr=subprocess.STDOUT)
    try:
        bridge = fixture.wait_for_bridge(LOG, app, timeout=90)
        session = fixture.McpSession(bridge, "scratchpad-probe")
    except (RuntimeError, fixture.McpError) as e:
        fail(str(e))


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
    time.sleep(0.3)


def shot(name):
    import base64
    try:
        result = session.request("tools/call", {"name": "screenshot", "arguments": {}})
    except fixture.McpError:
        return None
    for c in result.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            os.makedirs(SHOTS, exist_ok=True)
            path = os.path.join(SHOTS, name + ".png")
            with open(path, "wb") as fh:
                fh.write(base64.b64decode(c["data"]))
            print(f"  screenshot -> {path}")
            return path
    return None


def label(n):
    """What a node says: its label, else its value (a Label keeps its text in `value`)."""
    return str(n.get("label") or n.get("value") or "").strip()


def bounds(n):
    return n.get("bounds") or {}


def wait_for(pred, what, timeout=20.0):
    """The first truthy `pred(nodes)`, polling; fails the run on timeout."""
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(nodes())
        if hit:
            return hit
        time.sleep(0.4)
    fail(f"timed out waiting for {what}")


def maybe(pred, timeout=3.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(nodes())
        if hit:
            return hit
        time.sleep(0.3)
    return None


def key(name, **mods):
    """One key to the focused widget. `command`, never `ctrl`: the accelerator."""
    call("inject_key", {"key": name, **mods})
    settle(2)


def click_node(n):
    if "click" in (n.get("actions") or []):
        call("invoke_action", {"node": n["id"], "action": "click"})
    else:
        b = bounds(n)
        call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                "button": "primary"})
    settle()


def click_in(n, dx=40.0, dy=10.0):
    """A pointer click inside an editor, near its top-left, so the caret lands in it."""
    b = bounds(n)
    call("inject_pointer", {"x": b["x"] + min(dx, b["width"] / 2),
                            "y": b["y"] + min(dy, b["height"] / 2), "button": "primary"})
    settle()


def text_of(n, ns):
    """Everything an editor's subtree says: what a screen reader reads."""
    by_id = {x["id"]: x for x in ns if "id" in x}
    parts = []

    def walk(x):
        for k in ("label", "value", "name"):
            if x.get(k):
                parts.append(str(x[k]))
                break
        for cid in x.get("children") or []:
            if cid in by_id:
                walk(by_id[cid])

    # The root's own label is its accessible name ("Scratchpad"), not its text.
    for cid in n.get("children") or []:
        if cid in by_id:
            walk(by_id[cid])
    return " ".join(parts)


# ---------------------------------------------------------------------------
# The window's parts
# ---------------------------------------------------------------------------

def trailing(ns):
    """The bounds of the trailing panel (`Complementary` 'Right panel')."""
    for n in ns:
        if n.get("role") == "Complementary" and label(n) == "Right panel":
            return bounds(n)
    return None


def inside(b, outer):
    cx, cy = b.get("x", 0) + b.get("width", 0) / 2, b.get("y", 0) + b.get("height", 0) / 2
    return (outer["x"] <= cx <= outer["x"] + outer["width"]
            and outer["y"] <= cy <= outer["y"] + outer["height"])


def header(ns):
    """The dock's header line ("Scratchpad · Prologue"), or None."""
    area = trailing(ns)
    for n in ns:
        if n.get("role") == "Label" and label(n).startswith(HEADER) and (
                area is None or inside(bounds(n), area)):
            return label(n)
    return None


def note_line(ns, text):
    area = trailing(ns)
    return next((n for n in ns if n.get("role") == "Label" and label(n) == text
                 and (area is None or inside(bounds(n), area))), None)


def scratch_editor(ns):
    """The scratchpad's editor: an editable `MultilineTextInput`, or a read-only
    `Document`, labelled Scratchpad, in the trailing panel."""
    area = trailing(ns)
    if area is None:
        return None
    return next((n for n in ns if n.get("role") in ("MultilineTextInput", "Document")
                 and label(n) == "Scratchpad" and inside(bounds(n), area)), None)


def scratch_text(ns=None):
    ns = ns if ns is not None else nodes()
    ed = scratch_editor(ns)
    return text_of(ed, ns).strip() if ed else None


def manuscript_editors(ns):
    """Text editors outside the trailing panel, top to bottom."""
    area = trailing(ns)
    eds = [n for n in ns if n.get("role") in ("MultilineTextInput", "Document")
           and label(n) != "Scratchpad" and area is not None and not inside(bounds(n), area)
           and bounds(n).get("x", 0) > 330]
    return sorted(eds, key=lambda e: bounds(e).get("y", 0))


def prose_text(ns=None):
    """The text of the item's own editor: the one holding the most."""
    ns = ns if ns is not None else nodes()
    eds = manuscript_editors(ns)
    return max((text_of(e, ns) for e in eds), key=len, default="")


def tree_item(name, ns=None):
    ns = ns if ns is not None else nodes()
    return next((n for n in ns if n.get("role") == "TreeItem" and label(n) == name
                 and bounds(n).get("x", 0) < 330), None)


def open_item(name):
    n = wait_for(lambda ns: tree_item(name, ns), f"the binder row {name!r}", 30)
    click_node(n)
    wait_for(lambda ns: next((t for t in ns if t.get("role") == "Tab" and label(t) == name
                              and bounds(t).get("y", 99) < 80 and bounds(t).get("x", 0) > 300),
                             None), f"a tab named {name!r}")
    wait_for(lambda ns: manuscript_editors(ns), f"{name}'s editor")
    time.sleep(0.6)
    settle()


def context_menu(item_name, *path):
    """Right-click a binder row and walk its menu: `expand` the submenus, `click` the last."""
    n = wait_for(lambda ns: tree_item(item_name, ns), f"the binder row {item_name!r}")
    call("invoke_action", {"node": n["id"], "action": "show_context_menu"})
    settle()
    for i, step in enumerate(path):
        row = wait_for(lambda ns, s=step: next(
            (m for m in reversed(ns) if (m.get("role") or "").startswith("MenuItem")
             and label(m) == s), None), f"the menu row {step!r}", 8)
        last = i == len(path) - 1
        call("invoke_action", {"node": row["id"], "action": "click" if last else "expand"})
        settle()
    time.sleep(0.8)
    settle()


def reveal_scratchpad():
    key("a", command=True, shift=True)
    return wait_for(lambda ns: header(ns) and scratch_editor(ns), "the Scratchpad dock", 15)


def type_in_scratchpad(text):
    ed = scratch_editor(nodes())
    click_in(ed)
    call("type_text", {"node": ed["id"], "text": text})
    settle(4)


# ---------------------------------------------------------------------------
# The project file
# ---------------------------------------------------------------------------

def bundle_files(path):
    """name -> bytes for every file in the bundle, zip or exploded folder."""
    out = {}
    if os.path.isdir(path):
        for root, _dirs, files in os.walk(path):
            for f in files:
                full = os.path.join(root, f)
                with open(full, "rb") as fh:
                    out[os.path.relpath(full, path).replace(os.sep, "/")] = fh.read()
    else:
        with zipfile.ZipFile(path) as z:
            for name in z.namelist():
                if not name.endswith("/"):
                    out[name] = z.read(name)
    return out


def save():
    """Ctrl+S, and wait until the project file changed on disk."""
    before = os.path.getmtime(PROJECT) if os.path.exists(PROJECT) else 0
    key("s", command=True)
    end = time.time() + 30
    while time.time() < end:
        try:
            if os.path.getmtime(PROJECT) > before:
                time.sleep(0.5)
                return
        except OSError:
            pass
        time.sleep(0.5)
    fail("Ctrl+S did not write the project file")


# ---------------------------------------------------------------------------
# The run
# ---------------------------------------------------------------------------

print(f"working copy: {PROJECT}")
fixture.assert_no_running_instance(fixture.skribisto_binary())
original = bundle_files(PROJECT)
launch()

# The real build opens a project whose binder lists the example's items.
wait_for(lambda ns: tree_item(A, ns), "the example's binder (is this a --features mocks build?)",
         60)

# --- a. reveal ---------------------------------------------------------------
open_item(A)
check(header(nodes()) is None, "a. before the chord the Scratchpad dock is not showing")
prose_a_before = prose_text()
check(len(prose_a_before) > 200, f"a. {A} opens with its text", repr(prose_a_before[:80]))
reveal_scratchpad()
ns = nodes()
check(header(ns) == HEADER + A, f"a. Ctrl+Shift+A reveals the dock, its header names {A}",
      f"header = {header(ns)!r}")
ed = scratch_editor(ns)
check(ed is not None and ed.get("role") == "MultilineTextInput" and "set_value" in (ed.get("actions") or []),
      "a. the scratchpad editor is an editable MultilineTextInput in the trailing area",
      f"editor = {ed and (ed.get('role'), ed.get('actions'))}")
check(scratch_text(ns) == "", f"a. {A}'s scratchpad starts empty", f"text = {scratch_text(ns)!r}")

# --- b. each item has its own ------------------------------------------------
type_in_scratchpad(NOTE_A)
check(scratch_text() == NOTE_A, f"b. {A}'s scratchpad takes {NOTE_A!r}", repr(scratch_text()))
check(prose_text() == prose_a_before, f"b. typing in the scratchpad left {A}'s own text alone")
open_item(B)
wait_for(lambda ns: header(ns) == HEADER + B, f"the dock to name {B}")
check(header(nodes()) == HEADER + B, f"b. opening {B}: the dock names {B}")
check(scratch_text() == "", f"b. {B}'s scratchpad is empty (not {A}'s)", repr(scratch_text()))
type_in_scratchpad(NOTE_B)
check(scratch_text() == NOTE_B, f"b. {B}'s scratchpad takes {NOTE_B!r}", repr(scratch_text()))
open_item(A)
wait_for(lambda ns: header(ns) == HEADER + A, f"the dock to name {A} again")
check(scratch_text() == NOTE_A, f"b. back on {A}: the scratchpad reads {NOTE_A!r}",
      repr(scratch_text()))

# --- c. Ctrl+B goes to the scratchpad ------------------------------------------
prose_a_before = prose_text()
click_in(scratch_editor(nodes()))
key("a", command=True)
key("b", command=True)
check(scratch_text() == NOTE_A, "c. Ctrl+A, Ctrl+B in the scratchpad: it still reads its note",
      repr(scratch_text()))
check(prose_text() == prose_a_before, f"c. the same keys left {A}'s text unchanged")
check(header(nodes()) == HEADER + A, "c. the dock still names " + A)

# --- d. the Full Chapter row the caret was last in -----------------------------
# A chapter folder with two scenes, made through the binder's own menus.
context_menu(CHAPTER, "Convert to", "Chapter folder")
for _ in range(2):
    context_menu(CHAPTER, "Add", "Scene")
open_item(CHAPTER)
full = wait_for(lambda ns: next((n for n in ns if n.get("role") == "RadioButton"
                                 and label(n) == "Full Chapter"), None),
                f"the Full Chapter segment on {CHAPTER}")
click_node(full)
rows = wait_for(lambda ns: len(manuscript_editors(ns)) >= 3 and manuscript_editors(ns),
                "three rows in the Full Chapter stream (the chapter's own and two scenes)")
time.sleep(1.0)


def stream_rows():
    return manuscript_editors(nodes())


def scroll_stream():
    """The stream's scroll column: the tallest one right of the binder."""
    cols = [n for n in nodes() if n.get("role") == "ScrollView" and bounds(n).get("x", 0) > 300
            and bounds(n).get("x", 0) < 400 and bounds(n).get("height", 0) > 300]
    return cols[0] if cols else None


def bring_on_screen(index):
    """Scroll the stream until row `index` is within the visible column, and return it."""
    col = scroll_stream()
    for _ in range(40):
        rows = stream_rows()
        row = rows[index]
        b = bounds(row)
        top, bottom = bounds(col)["y"], bounds(col)["y"] + bounds(col)["height"]
        if top + 5 <= b["y"] and b["y"] + 30 <= bottom:
            return row
        step = b["y"] - (top + 60) if b["y"] > bottom - 30 else b["y"] - (top + 60)
        call("scroll", {"node": col["id"], "dy": max(-3000.0, min(3000.0, step))})
        settle(2)
    fail(f"could not scroll stream row {index} into view")


last = len(stream_rows()) - 1
check(last >= 2, "d. the Full Chapter view lists the chapter and its two scenes",
      f"{len(stream_rows())} text rows")
click_in(bring_on_screen(last))
wait_for(lambda ns: header(ns) == HEADER + NEW_SCENE, "the dock to follow the caret into a scene row")
check(header(nodes()) == HEADER + NEW_SCENE,
      f"d. caret in the last row: the dock names that scene ({NEW_SCENE})", repr(header(nodes())))
type_in_scratchpad(NOTE_C)
check(header(nodes()) == HEADER + NEW_SCENE and scratch_text() == NOTE_C,
      f"d. clicking into the scratchpad and typing: the dock still names {NEW_SCENE}",
      f"header={header(nodes())!r} text={scratch_text()!r}")
click_in(bring_on_screen(0))
wait_for(lambda ns: header(ns) == HEADER + CHAPTER, "the dock to follow the caret to the chapter's own row")
check(header(nodes()) == HEADER + CHAPTER and scratch_text() == "",
      f"d. caret back in the chapter's own row: the dock names {CHAPTER}, with its own scratchpad",
      f"header={header(nodes())!r} text={scratch_text()!r}")
click_in(bring_on_screen(last))
wait_for(lambda ns: header(ns) == HEADER + NEW_SCENE, "the dock to return to the scene row")
check(scratch_text() == NOTE_C, "d. and the scene's scratchpad still holds its note",
      repr(scratch_text()))

# --- e. saved with the project -------------------------------------------------
save()
files = bundle_files(PROJECT)
pads = {n: d for n, d in files.items() if n.endswith(".scratchpad.djot")}
check(any(NOTE_A.encode() in d for d in pads.values()),
      "e. the saved bundle holds a *.scratchpad.djot with " + repr(NOTE_A),
      f"scratchpad files: {sorted(pads)}")
check(any(NOTE_B.encode() in d for d in pads.values()) and any(NOTE_C.encode() in d for d in pads.values()),
      "e. and one for each of the other two notes", f"scratchpad files: {sorted(pads)}")
# (`history/` blobs are the project's recorded past, which keeps scratchpads too.)
leaks = [n for n, d in files.items() if not n.endswith(".scratchpad.djot")
         and not n.startswith("history/")
         and re.search(rb"(alpha|beta|gamma) note", d)]
check(not leaks, "e. no other file of the bundle holds a scratchpad note (it is not manuscript text)",
      f"found in {leaks}")
# c, in the file: the bold went to the scratchpad. The scene's Djot has no new strong run.
bold_pad = next((d for d in pads.values() if NOTE_A.encode() in d), b"")
check(re.search(rb"\*\s*" + NOTE_A.encode() + rb"\s*\*", bold_pad) is not None,
      "c. the saved scratchpad carries the bold (the formatting went where the caret was)",
      repr(bold_pad))
new_strong = [n for n, d in files.items() if n.endswith(".scene.djot")
              and d.count(b"*") > original.get(n, d).count(b"*")]
check(not new_strong, "c. no scene file gained a strong run (the manuscript took no bold)",
      f"changed: {new_strong}")

session.close()
session = None
app.send_signal(signal.SIGTERM)
try:
    code = app.wait(timeout=90)
except subprocess.TimeoutExpired:
    fail("the app was still running 90 s after SIGTERM")
check(code == 0, "e. the app quit cleanly", f"exit status {code}")
launch()
wait_for(lambda ns: tree_item(A, ns), "the binder after the relaunch", 60)
open_item(A)
if header(nodes()) is None:
    reveal_scratchpad()
wait_for(lambda ns: header(ns) == HEADER + A and scratch_text(ns) == NOTE_A,
         f"{A}'s scratchpad to read {NOTE_A!r} after the relaunch", 15)
check(scratch_text() == NOTE_A, f"e. relaunched: {A}'s scratchpad reads {NOTE_A!r}",
      repr(scratch_text()))
open_item(B)
wait_for(lambda ns: header(ns) == HEADER + B, f"the dock to name {B}")
check(scratch_text() == NOTE_B, f"e. relaunched: {B}'s scratchpad reads {NOTE_B!r}",
      repr(scratch_text()))

# --- f. a trashed item's scratchpad is read-only --------------------------------
context_menu(B, "Move to Trash")
trash_tab = wait_for(lambda ns: next((n for n in ns if n.get("role") == "Tab" and label(n) == "Trash"
                                      and bounds(n).get("x", 99) < 100), None), "the Trash rail tab")
click_node(trash_tab)

def newest_trashed(ns):
    """The Trash dock lists the newest first. A numbered chapter has no title of its
    own, so its row there is blank: take the first row rather than look it up by name."""
    rows = sorted((n for n in ns if n.get("role") == "TreeItem" and bounds(n).get("x", 0) < 330
                   and bounds(n).get("y", 0) > 90), key=lambda n: bounds(n).get("y", 0))
    return rows[0] if rows else None


trashed = wait_for(newest_trashed, "the trashed item in the Trash dock")
print(f"  (the Trash dock row for {B} is labelled {label(trashed)!r})")
click_node(trashed)
wait_for(lambda ns: note_line(ns, TRASHED), "the dock to say it is read-only", 20)
# A trashed chapter is no longer numbered, so "Chapter 1" reads "Chapter" there.
check((header(nodes()) or "").startswith(HEADER + B.split()[0]),
      f"f. the dock still names the item once it is in the Trash ({B} reads as 'Chapter' there)",
      repr(header(nodes())))
ns = nodes()
ed = scratch_editor(ns)
check(note_line(ns, TRASHED) is not None, "f. the trashed item's dock says its notes are read-only")
check(ed is not None and ed.get("role") == "Document" and "set_value" not in (ed.get("actions") or []),
      "f. the scratchpad editor is read-only (a Document, no set_value)",
      f"editor = {ed and (ed.get('role'), ed.get('actions'))}")
before = scratch_text(ns)
check(before == NOTE_B, f"f. a trashed item's note is still shown ({NOTE_B!r})", repr(before))
click_in(ed)
call("type_text", {"node": ed["id"], "text": "zzz"})
settle(3)
for name, mods in [("Backspace", {}), ("Delete", {}), ("b", {"command": True}),
                   ("x", {"command": True}), ("v", {"command": True})]:
    key(name, **mods)
check(scratch_text() == before, "f. typing, Backspace, Delete, Ctrl+B, Ctrl+X, Ctrl+V change nothing",
      f"{before!r} -> {scratch_text()!r}")

# --- g. no item -----------------------------------------------------------------
tab = wait_for(lambda ns: next((n for n in ns if n.get("role") == "Tab"
                                and bounds(n).get("y", 99) < 80 and 300 < bounds(n).get("x", 0) < 1000
                                and bounds(n).get("width", 0) > 100), None),
               "an open document tab")
call("invoke_action", {"node": tab["id"], "action": "show_context_menu"})
settle()
close_all = wait_for(lambda ns: next((m for m in ns if (m.get("role") or "").startswith("MenuItem")
                                      and label(m).replace("&", "").lower() == "close all"), None),
                     "'Close all' in the tab menu", 8)
call("invoke_action", {"node": close_all["id"], "action": "click"})
settle(4)
wait_for(lambda ns: note_line(ns, NO_ITEM), "the no-item message", 15)
ns = nodes()
check(note_line(ns, NO_ITEM) is not None, "g. with every tab closed the dock says " + repr(NO_ITEM))
check(scratch_editor(ns) is None and header(ns) is None, "g. and shows no editor and no item name")

print("\nALL PASS")
stop()
