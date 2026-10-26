#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: a menu opened with the pointer leaves the caret in the text.

Since teksilo 0.21 the title bar's menu button takes no focus from a click
(`focus_on_click(false)`): the menu opened with it gives the focus back to the
editor the writer was typing in however it closes, and a row's command is sent
from that editor. Before, the click moved focus to the button, where it stayed,
and the writer had to click back into the prose after every trip to a menu.

Asserts, against the running app:

  1. a scene opens and its prose editor holds the focus;
  2. a pointer click on the menu button opens the menu (whose rows then hold the
     keyboard, so arrow keys move through them);
  3. Escape closes the menu, and the focus is back in the editor;
  4. in a menu opened the same way, Edit offers Paste and Select all and greys
     Cut and Copy (nothing is selected), answering for the editor rather than
     for the open menu; Edit ▸ Select all then selects the editor's text (the
     row's intent went to the editor) and leaves the focus there, after which
     Edit ▸ Copy is offered.

Not on macOS: there the menu is the system menu bar, which the bridge does not
open, and the in-window menu button is not shown.

Run: python3 scripts/automation_menu_keeps_focus.py [project.skrib]
"""
import os
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

if fixture.IS_MACOS:
    # What a Mac writer uses is the system menu bar, which takes no focus at all
    # and is checked by hand; the in-window copy exists there only for the probes.
    fixture.skip("on macOS the writer's menu is the system menu bar, which the bridge does not open")

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
PROJECT = fixture.working_copy(SRC, label="menu-focus")
ENV = fixture.isolated_config(locale="en-US", label="menu-focus", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "en-US", "ui.show_welcome": False},
                                label="menu-focus")

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


def text_of(n):
    return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))


def label(n):
    # Mnemonics are an `&` in the ftl value, so the rendered label carries one.
    return (n.get("label") or "").replace("&", "")


def wait_for(pred, what, timeout=90.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(session.nodes())
        if hit:
            return hit
        time.sleep(0.5)
    fail(f"timed out waiting for {what}")


def click(n):
    b = n["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2,
                                    "y": b["y"] + b["height"] / 2, "button": "primary"})
    session.settle(4)


def prose_editors(ns):
    return [n for n in ns if n.get("role") == "MultilineTextInput"
            and "set_value" in (n.get("actions") or [])]


def focused(ns):
    return [n for n in ns if n.get("focused")]


def editor_has_focus(ns):
    """The focused node is one of the prose editors."""
    ids = {e["id"] for e in prose_editors(ns)}
    return any(n["id"] in ids for n in focused(ns))


def describe_focus(ns):
    return [f"{n.get('role')} {label(n)!r}" for n in focused(ns)] or ["nothing"]


def menu_items(ns):
    return [n for n in ns if n.get("role") == "MenuItem"]


def open_menu_with_the_pointer():
    button = [n for n in session.nodes() if n.get("label") == "Menu"]
    if not button:
        fail("no menu button in the title bar")
    click(button[0])
    wait_for(lambda ns: ns if any(label(i) == "Edit" for i in menu_items(ns)) else None,
             "the menu to open")
    # Ids are snapshot-scoped and a menu that is still being built re-mints them:
    # let it finish, then hand back a snapshot whose rows can be addressed.
    return settled_nodes()


def settled_nodes():
    time.sleep(0.9)
    session.settle(4)
    return session.nodes()


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "menu-keeps-focus-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    # 1. A scene's prose editor holds the focus.
    row = wait_for(lambda ns: [n for n in ns if n.get("role") == "TreeItem"
                               and text_of(n).strip().startswith("Prologue")],
                   "the binder to list the Prologue")[0]
    # Opening a scene from the binder puts the caret in its prose. Not a click in
    # the text: the pointer resting there raises a tooltip, which the tree then
    # reports as focused.
    click(row)
    wait_for(lambda ns: ns if editor_has_focus(ns) else None,
             "the scene's prose editor to take the focus")
    print("OK  1. the prose editor holds the focus")

    # 2. A pointer click on the menu button opens the menu.
    open_menu_with_the_pointer()
    print("OK  2. a pointer click on the menu button opened the menu")

    # 3. Escape closes it and leaves the focus in the editor.
    session.call("inject_key", {"key": "Escape"})
    session.settle(4)
    ns = wait_for(lambda ns: ns if not menu_items(ns) else None, "the menu to close")
    if not editor_has_focus(ns):
        fail(f"after Escape the focus is not in the editor: {describe_focus(ns)}")
    print("OK  3. Escape closed the menu, and the editor holds the focus")

    # 4. Edit ▸ Select all reaches the editor and leaves it focused. Before it,
    #    with nothing selected, Paste and Select all are offered and Cut and Copy
    #    are not: the rows answer for the editor, not for the open menu.
    ns = open_menu_with_the_pointer()
    edit = [i for i in menu_items(ns) if label(i) == "Edit"][0]
    session.call("invoke_action", {"action": "click", "node": edit["id"]})
    wait_for(lambda ns: [i for i in menu_items(ns) if label(i) == "Select all"],
             "Edit ▸ Select all")
    rows = {label(i): i for i in menu_items(settled_nodes())}
    greyed = sorted(name for name in ("Cut", "Copy", "Paste", "Select all")
                    if rows[name].get("disabled"))
    if greyed != ["Copy", "Cut"]:
        fail(f"with nothing selected in the editor, exactly Cut and Copy must be greyed "
             f"out, got {greyed}")
    session.call("invoke_action", {"action": "click", "node": rows["Select all"]["id"]})
    session.settle(4)
    ns = wait_for(lambda ns: ns if not menu_items(ns) else None, "the menu to close")
    if not editor_has_focus(ns):
        fail(f"after Select all the focus is not in the editor: {describe_focus(ns)}")
    ns = open_menu_with_the_pointer()
    edit = [i for i in menu_items(ns) if label(i) == "Edit"][0]
    session.call("invoke_action", {"action": "click", "node": edit["id"]})
    wait_for(lambda ns: [i for i in menu_items(ns) if label(i) == "Copy"], "Edit ▸ Copy")
    copy = [i for i in menu_items(settled_nodes()) if label(i) == "Copy"][0]
    if copy.get("disabled") or copy.get("enabled") is False:
        fail(f"Edit ▸ Copy is greyed out after Select all: {copy}")
    session.call("inject_key", {"key": "Escape"})
    session.settle(2)
    session.call("inject_key", {"key": "Escape"})
    session.settle(2)
    print("OK  4. Edit ▸ Select all reached the editor, which kept the focus")
except fixture.McpError as e:
    fail(str(e))

session.close()
app.terminate()
try:
    app.wait(timeout=30)
except subprocess.TimeoutExpired:
    app.kill()
print("\nALL PASS")
