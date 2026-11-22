#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: a click in the page margin beside a scene's text puts the caret
on the line at that height, drawn there.

The headless tests run on a text layout whose lines end before their trailing
space, so the end of one line and the start of the next are two positions there.
In the real app, with real fonts, each wrapped line keeps its trailing space and
the two are one position: only the caret's affinity says which line it is drawn
on. A click in the left margin beside a paragraph's second line put the caret at
the end of its first line, which no headless test could see.

The text cannot show it either: typing at that one position gives the same text
whichever line the caret is drawn on. So after each click this asks the caret's
line itself, with a key that moves along it:

  * left of a wrapped line, End and then a marker: the marker must land at the
    end of the clicked line, not the end of the line above;
  * right of a wrapped line, Home and then a marker: it must land at the start
    of the clicked line, not the start of the line below.

Each marker is undone before the next click. Lines are read from the editor's
`TextRun` children, one per line, with their bounds and their text.

Run: python3 scripts/automation_page_caret.py [project.skrib]
"""
import base64
import os
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
PROJECT = fixture.working_copy(SRC, label="page-caret")
ENV = fixture.isolated_config(locale="en-US", label="page-caret", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "en-US", "ui.show_welcome": False},
                                label="page-caret")
SHOTS = os.environ.get("SHOT_DIR", tempfile.mkdtemp(prefix="skribisto_page_caret_"))
MARK = "¤"

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=ENV)
session = None


def fail(message):
    print("FAIL:", message)
    print("--- app log tail ---")
    print("\n".join(open(log, encoding="utf-8", errors="replace").read().splitlines()[-25:]))
    if session:
        session.close()
    if app.poll() is None:
        app.kill()
    sys.exit(1)


def wait_for(pred, what, timeout=90.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(session.nodes())
        if hit:
            return hit
        time.sleep(0.5)
    fail(f"timed out waiting for {what}")


def shot(name):
    result = session.request("tools/call", {"name": "screenshot", "arguments": {}})
    for c in result.get("content", []):
        if c.get("type") == "image" and isinstance(c.get("data"), str):
            path = os.path.join(SHOTS, f"{name}.png")
            with open(path, "wb") as f:
                f.write(base64.b64decode(c["data"]))
            print(f"  screenshot -> {path}")
            return


def press(x, y, kind="mouse"):
    session.call("inject_pointer", {"x": x, "y": y, "button": "primary", "kind": kind})
    session.settle(4)
    # Apart from the next press, so two never make a double-click.
    time.sleep(0.6)


def key(name, command=False):
    session.call("inject_key", {"key": name, "command": command})
    session.settle(2)


def from_the_top():
    """Send the caret to the start of the text, away from every line checked, so
    a check only passes when its own press moved the caret."""
    key("Home", command=True)


def prose(ns):
    """The scene's prose editor and its lines, top to bottom."""
    editors = [n for n in ns if n.get("role") == "MultilineTextInput"
               and n.get("label") == "Text"]
    if not editors:
        return None
    editor = editors[0]
    by_id = {n["id"]: n for n in ns}
    lines = [by_id[c] for c in editor.get("children", []) if c in by_id
             and by_id[c].get("role") == "TextRun"]
    return editor, lines


def text_of(editor):
    return (editor.get("text") or {}).get("document_text") or ""


def line_starts(editor, lines):
    """Where each line starts in the editor's text.

    The runs are the lines in order and hold their text with its trailing
    space; between two paragraphs the text holds a newline the runs do not.
    """
    text = text_of(editor)
    starts, at = [], 0
    for line in lines:
        value = line.get("value") or ""
        found = text.find(value, at)
        if found < 0:
            fail(f"line {value!r} is not in the editor's text after {at}")
        starts.append(found)
        at = found + len(value)
    return starts


def marker_at(what):
    """Type the marker, read where it went in the text, and undo it."""
    editor, _ = prose(session.nodes())
    session.call("type_text", {"node": editor["id"], "text": MARK})
    session.settle(4)
    editor, _ = wait_for(lambda ns: (lambda p: p if p and MARK in text_of(p[0]) else None)(
        prose(ns)), f"the marker typed {what}", timeout=10)
    index = text_of(editor).index(MARK)
    session.call("inject_key", {"key": "z", "command": True})
    session.settle(4)
    wait_for(lambda ns: (lambda p: p if p and MARK not in text_of(p[0]) else None)(prose(ns)),
             "the marker to be undone", timeout=10)
    return index


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "page-caret-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    schema = next(t for t in session.request("tools/list").get("tools", [])
                  if t["name"] == "inject_pointer")
    print("inject_pointer takes:", sorted(schema["inputSchema"].get("properties", {})))

    # Maximized, so the page is wider than its column and has a margin on each
    # side. In the default window the column fills the page, and the right
    # "margin" is all under the page's scroll bar.
    maximize = wait_for(lambda ns: [n for n in ns if (n.get("label") or "") == "Maximize"],
                        "the title bar's Maximize button")[0]
    session.call("invoke_action", {"action": "click", "node": maximize["id"]})
    session.settle(6)
    time.sleep(1.0)

    row = wait_for(lambda ns: [n for n in ns if n.get("role") == "TreeItem"
                               and (n.get("label") or "").startswith("Prologue")],
                   "the binder to list the Prologue")[0]
    b = row["bounds"]
    press(b["x"] + b["width"] / 2, b["y"] + b["height"] / 2)
    editor, lines = wait_for(lambda ns: (lambda p: p if p and len(p[1]) > 4 else None)(
        prose(ns)), "the Prologue's prose and its lines")
    starts = line_starts(editor, lines)
    text = text_of(editor)
    eb = editor["bounds"]
    # Lines 1 to 3 are the first paragraph's second, third and fourth lines: each
    # starts where the line above ends, its trailing space included.
    wrapped = [i for i in (1, 2, 3) if text[starts[i] - 1] == " "]
    if len(wrapped) < 3:
        fail(f"the first paragraph must wrap onto four lines: {[l.get('value') for l in lines[:4]]}")
    # The page is the `ScrollView` holding the editor: the margins are what lies
    # between its edges and the editor's, narrow in a small window.
    ns = session.nodes()
    pages = [n for n in ns if n.get("role") == "ScrollView" and (n.get("bounds") or {})
             and n["bounds"]["x"] <= eb["x"] and n["bounds"]["x"] + n["bounds"]["width"]
             >= eb["x"] + eb["width"] and n["bounds"]["height"] < eb["height"]]
    if not pages:
        fail("no page scroll view around the prose editor")
    page = min(pages, key=lambda n: n["bounds"]["width"])["bounds"]
    left_x = (page["x"] + eb["x"]) / 2
    right_x = (eb["x"] + eb["width"] + page["x"] + page["width"]) / 2
    # Clear of the page's scroll bar on the right, which a press would page with.
    right_x = min(right_x, page["x"] + page["width"] - 20)
    if eb["x"] - page["x"] < 8 or right_x - (eb["x"] + eb["width"]) < 4:
        fail(f"no margin to press in: page {page}, editor {eb}")
    print(f"editor at x {eb['x']:.0f}..{eb['x'] + eb['width']:.0f}; pressing at "
          f"x {left_x:.0f} and {right_x:.0f}, in a page from {page['x']:.0f} to "
          f"{page['x'] + page['width']:.0f}")

    failures = []
    # Every wrapped line with the mouse, and the first with a finger, which
    # places the caret on release rather than on press.
    for i, kind in [(i, "mouse") for i in wrapped] + [(wrapped[0], "touch")]:
        lb = lines[i]["bounds"]
        y = lb["y"] + lb["height"] / 2
        by = f" ({kind})"
        # End goes past the line's trailing space: to the position the next line
        # starts at, drawn at the end of this one.
        end_of_line = starts[i] + len(lines[i].get("value") or "")

        from_the_top()
        press(left_x, y, kind)
        if i == 1 and kind == "mouse":
            shot("left-of-line-2")
        key("End")
        got = marker_at(f"after a click left of line {i + 1} and End")
        if got != end_of_line:
            failures.append(
                f"left of line {i + 1}{by}: End went to {got}, not the end of that line "
                f"({end_of_line}): the caret was drawn on another line")
        else:
            print(f"OK  left of line {i + 1}{by}: the caret is at its start (End reached its end)")

        from_the_top()
        press(right_x, y, kind)
        key("Home")
        got = marker_at(f"after a click right of line {i + 1} and Home")
        if got != starts[i]:
            failures.append(
                f"right of line {i + 1}{by}: Home went to {got}, not the start of that line "
                f"({starts[i]}): the caret was drawn on another line")
        else:
            print(f"OK  right of line {i + 1}{by}: the caret is at its end (Home reached its start)")

    # Beside the first line, where nothing wraps into it.
    lb = lines[0]["bounds"]
    press(left_x, lb["y"] + lb["height"] / 2)
    got = marker_at("after a click left of the first line")
    if got != 0:
        failures.append(f"left of the first line: the caret is at {got}, not 0")
    else:
        print("OK  left of the first line: the caret is at the start of the text")

    if failures:
        fail("; ".join(failures))
    print("PASS: a click beside a line puts the caret on that line, at the clicked side")
except fixture.McpError as e:
    fail(str(e))
finally:
    if session:
        session.close()
    if app.poll() is None:
        app.terminate()
        try:
            app.wait(timeout=10)
        except subprocess.TimeoutExpired:
            app.kill()
