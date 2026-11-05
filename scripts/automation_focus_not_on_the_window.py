#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: Ctrl+G and F9 leave a screen reader somewhere, and tell it.

Measured with NVDA on Windows, both of these left the writer on the bare window,
hearing its title: Enter in Go to, and F9 while the binder held the focus. Go to's
results were not spoken at all, and showing the side again said nothing.

Asserts, against the running app:

  1. typing in Go to announces how many rows match and the first of them;
  2. Down and Up announce the row reached and its place among the matches, and an
     arrow at the end of the list says the same row again;
  3. Enter closes the popup and leaves the focus in the opened item's text;
  4. F9 with the focus in the binder moves it to the text and says the left panel hid;
  5. F9 again says it is shown and leaves the focus in the text;
  6. F9 with the focus in the text hides the side and leaves the focus there;
  7. on a page with no text (a book on Pace), F9 lands in the page; so does Go to from a
     scene's text, while Go to from the binder stays in the binder; and the View menu's row
     behaves as F9 does;
  8. the status bar's right-panel button does the same as F9;
  9. entering distraction-free mode puts the keyboard in its text, and leaving it gives
     the keyboard back to the text underneath (`automation_distraction_free_focus.py`
     has the other ways in).

The announcements are what the bridge captures from the live regions, as a platform
adapter is handed them. No screen reader takes part: what one makes of them is measured
on the Windows machine, not here.

Run: python3 scripts/automation_focus_not_on_the_window.py [project.skrib]
"""
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
PROJECT = fixture.working_copy(SRC, label="focus-window")
ENV = fixture.isolated_config(locale="en-US", label="focus-window", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "en-US", "ui.show_welcome": False},
                                label="focus-window")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=ENV)
session = None
seq = 0


def fail(message):
    print("FAIL:", message)
    print("--- app log tail ---")
    print("\n".join(open(log, encoding="utf-8", errors="replace").read().splitlines()[-25:]))
    if session:
        session.close()
    if app.poll() is None:
        app.kill()
    sys.exit(1)


def text_of(n):
    return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))


def wait_for(pred, what, timeout=90.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(session.nodes())
        if hit:
            return hit
        time.sleep(0.25)
    fail(f"timed out waiting for {what}; focus is on {describe_focus(session.nodes())}")


def focused(ns):
    return [n for n in ns if n.get("focused")]


def describe_focus(ns):
    return [f"{n.get('role')} {text_of(n)[:50]!r}" for n in focused(ns)] or ["nothing"]


def prose_has_focus(ns):
    return any(n.get("role") == "MultilineTextInput" for n in focused(ns))


def heard():
    """What has been announced since the last call."""
    global seq
    session.settle(3)
    items = session.call("pull_announcements", {"since_seq": seq})
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
        if any(needle in t for t in got):
            print(f"  ok: {what}: {[t for t in got if needle in t]}")
            return got
        time.sleep(0.2)
    fail(f"{what}: expected {needle!r} among what was announced: {got}")


def key(name, **kw):
    session.call("inject_key", {"key": name, **kw})
    session.settle(2)


def ctrl_shift(letter):
    session.call("inject_key", {"key": letter, "command": True, "shift": True})
    session.settle(2)


def binder_row(ns, title):
    return [n for n in ns if n.get("role") == "TreeItem" and text_of(n).strip().startswith(title)]


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "focus-window-probe", mcp=os.environ.get("TEKSILO_MCP_BIN"))
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    # A scene is open with the caret in its text: somewhere to jump from.
    row = wait_for(lambda ns: binder_row(ns, "Prologue"), "the binder to list the Prologue")[0]
    b = row["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                    "button": "primary"})
    wait_for(prose_has_focus, "the Prologue's text to take the focus")
    heard()

    # 1. Typing says how many match and which comes first.
    print("== Go to: typing ==")
    key("g", command=True)
    field = wait_for(lambda ns: [n for n in ns if n.get("focused") and "Search the binder" in text_of(n)],
                     "Go to's search field to take the focus")[0]
    session.call("type_text", {"node": field["id"], "text": "e"})
    expect_heard("Matches:", "typing announces the matches")
    session.call("type_text", {"node": field["id"], "text": "pi"})
    expect_heard("Epilogue", "a narrower search names its first match")
    session.call("type_text", {"node": field["id"], "text": "zzqq"})
    expect_heard("No item matches", "a search with no match says so")
    for _ in range(4):
        key("Backspace")
    heard()

    # 2. The arrows say the row and its place. The focus stays in the field.
    print("== Go to: arrows ==")
    key("ArrowDown")
    got = expect_heard("1 of ", "Down names the first match")
    print("     ", got)
    key("ArrowDown")
    expect_heard("1 of ", "Down at the end of one match says it again")
    if not [n for n in focused(session.nodes()) if "Search the binder" in text_of(n)]:
        fail(f"the arrows took the focus out of the field: {describe_focus(session.nodes())}")

    # 3. Enter opens it with the caret in its text.
    print("== Go to: Enter ==")
    key("Enter")
    wait_for(lambda ns: prose_has_focus(ns) and not [n for n in ns if "Search the binder" in text_of(n)],
             "the opened item's text to take the focus")
    if not any("Epilogue" in text_of(n) and n.get("selected") for n in session.nodes()):
        print("  note: no selected row or tab named Epilogue in the tree")
    print("  ok: the focus is in the text:", describe_focus(session.nodes()))

    # 4. F9 from the binder.
    print("== F9 from inside the binder ==")
    heard()
    ctrl_shift("b")
    wait_for(lambda ns: focused(ns) and not prose_has_focus(ns), "the binder to take the focus")
    print("  focus in the binder:", describe_focus(session.nodes()))
    heard()
    key("F9")
    expect_heard("Left panel hidden", "F9 says the side hid")
    wait_for(prose_has_focus, "the focus to reach the text")
    print("  ok: the focus is in the text:", describe_focus(session.nodes()))

    # 5. F9 again: shown, and the caret has not moved.
    print("== F9 to show it again ==")
    key("F9")
    expect_heard("Left panel shown", "F9 says the side is back")
    time.sleep(0.6)
    if not prose_has_focus(session.nodes()):
        fail(f"showing the side moved the focus: {describe_focus(session.nodes())}")
    print("  ok: the focus stayed in the text")

    # 6. F9 from the text: it stays.
    print("== F9 from the text ==")
    key("F9")
    expect_heard("Left panel hidden", "F9 says the side hid")
    time.sleep(0.6)
    if not prose_has_focus(session.nodes()):
        fail(f"hiding a side the focus was not in moved it: {describe_focus(session.nodes())}")
    key("F9")
    expect_heard("Left panel shown", "F9 says the side is back")
    print("  ok: the focus stayed in the text")

    # 7. A page with no text: a book on Pace.
    print("== A page with no text ==")
    b = binder_row(session.nodes(), "Starforgers")[0]["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                    "button": "primary"})
    pace = wait_for(lambda ns: [n for n in ns if n.get("role") == "RadioButton"
                                and text_of(n).strip() == "Pace"], "the book's views")[0]
    session.call("invoke_action", {"node": pace["id"], "action": "click"})
    time.sleep(1.0)
    session.settle(3)
    ctrl_shift("b")
    time.sleep(0.5)
    session.settle(3)
    heard()
    key("F9")
    expect_heard("Left panel hidden", "F9 says the side hid")
    views = wait_for(lambda ns: [n for n in focused(ns) if "Views" in text_of(n)],
                     "the focus to reach the bar of views")
    print("  ok: F9 lands in the view:", describe_focus(session.nodes()))
    key("F9")
    heard()

    # Go to from a scene's text to that book: the editor that held the keyboard is parked
    # by the tab coming up, and the book has no text to take it.
    b = binder_row(session.nodes(), "Prologue")[0]["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                    "button": "primary"})
    wait_for(prose_has_focus, "the Prologue's text to take the focus")
    key("g", command=True)
    field = wait_for(lambda ns: [n for n in ns if n.get("focused") and "Search the binder" in text_of(n)],
                     "Go to's search field")[0]
    session.call("type_text", {"node": field["id"], "text": "Starforgers"})
    expect_heard("Starforgers", "the book is the match")
    key("Enter")
    wait_for(lambda ns: [n for n in focused(ns) if "Views" in text_of(n)],
             "the focus to reach the bar of views after Go to")
    print("  ok: Go to lands in the view:", describe_focus(session.nodes()))

    # From the binder, the guide's rule: it stays in the binder.
    ctrl_shift("b")
    time.sleep(0.5)
    session.settle(3)
    key("g", command=True)
    field = wait_for(lambda ns: [n for n in ns if n.get("focused") and "Search the binder" in text_of(n)],
                     "Go to's search field")[0]
    session.call("type_text", {"node": field["id"], "text": "Starforgers"})
    time.sleep(0.5)
    key("Enter")
    time.sleep(1.0)
    session.settle(3)
    if not [n for n in focused(session.nodes()) if n.get("role") == "Tree"]:
        fail(f"Go to from the binder should stay there: {describe_focus(session.nodes())}")
    print("  ok: from the binder it stays in the binder")

    # 7b. The View menu's row, a door that closes a menu first.
    print("== The View menu ==")
    b = binder_row(session.nodes(), "Prologue")[0]["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                    "button": "primary"})
    wait_for(prose_has_focus, "the Prologue's text to take the focus")
    ctrl_shift("b")
    time.sleep(0.5)
    session.settle(3)
    heard()
    menu = [n for n in session.nodes() if text_of(n).strip() == "Menu"][0]["bounds"]
    session.call("inject_pointer", {"x": menu["x"] + menu["width"] / 2,
                                    "y": menu["y"] + menu["height"] / 2, "button": "primary"})
    # Ids and bounds are good only once the menu has finished opening.
    time.sleep(1.0)
    session.settle(3)
    view = wait_for(lambda ns: [n for n in ns if n.get("role") == "MenuItem"
                                and text_of(n).replace("&", "").strip() == "View"], "the menu")[0]["bounds"]
    session.call("inject_pointer", {"x": view["x"] + view["width"] / 2,
                                    "y": view["y"] + view["height"] / 2, "button": "primary"})
    time.sleep(1.0)
    session.settle(3)
    row = wait_for(lambda ns: [n for n in ns if n.get("role") == "MenuItemCheckBox"
                               and "Left panel" in text_of(n).replace("&", "")], "the row")[0]["bounds"]
    session.call("inject_pointer", {"x": row["x"] + row["width"] / 2,
                                    "y": row["y"] + row["height"] / 2, "button": "primary"})
    expect_heard("Left panel hidden", "the row says the side hid")
    wait_for(prose_has_focus, "the focus to reach the text after the menu")
    print("  ok: the row lands in the text")
    key("F9")
    heard()

    # 8. The right panel's command.
    print("== Right panel ==")
    ctrl_shift("i")
    time.sleep(0.6)
    session.settle(3)
    print("  focus in the right panel:", describe_focus(session.nodes()))
    heard()
    # No default chord: the status bar's button fires the same command.
    button = [n for n in session.nodes() if text_of(n).strip() == "Toggle the inspector"]
    if not button:
        fail("no inspector toggle in the status bar")
    session.call("invoke_action", {"node": button[0]["id"], "action": "click"})
    expect_heard("Right panel hidden", "the status bar's button says the side hid")
    time.sleep(1.0)
    session.settle(3)
    print("  focus afterwards:", describe_focus(session.nodes()))
    wait_for(prose_has_focus, "the focus to reach the text from the right panel")
    session.call("invoke_action", {"node": [n for n in session.nodes() if text_of(n).strip()
                                            == "Toggle the inspector"][0]["id"], "action": "click"})
    expect_heard("Right panel shown", "the button says the side is back")

    # 9. Leaving distraction-free mode: the surface is gone, the text takes the keyboard.
    print("== Distraction-free ==")
    key("F11")
    time.sleep(2.0)
    session.settle(3)
    wait_for(prose_has_focus, "the focus to reach the text in the mode")
    print("  in the mode:", describe_focus(session.nodes()))
    key("F11")
    wait_for(prose_has_focus, "the focus to reach the text after leaving the mode")
    print("  ok: the text has the focus again")

    print("PASS")
finally:
    if session:
        session.close()
    if app.poll() is None:
        app.terminate()
