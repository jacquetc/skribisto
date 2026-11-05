#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: every way into distraction-free mode leaves the keyboard somewhere.

Pressing F11 used to leave a screen reader on nothing, with every key going nowhere.

Asserts, against the running app:

  1. F11 from a scene's text lands in the text of the same document, on the same caret;
  2. typing there changes that document;
  3. F11 again, and the Exit button, give the keyboard back to the text;
  4. F11 from the binder lands in the text;
  5. the title bar's button and the View menu's row land in the text;
  6. on a page with no text (a book on Pace) it lands in the page;
  7. with nothing open it lands on the strip and says nothing can be typed.

No screen reader takes part: what one makes of this is measured on the Windows machine.

Run: python3 scripts/automation_distraction_free_focus.py [project.skrib]
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
PROJECT = fixture.working_copy(SRC, label="df-focus")
ENV = fixture.isolated_config(locale="en-US", label="df-focus", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "en-US", "ui.show_welcome": False},
                                label="df-focus")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=ENV)
session = None
seq = 0
CARET = 5


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


def focused(ns):
    return [n for n in ns if n.get("focused")]


def describe_focus(ns):
    return [f"{n.get('role')} {text_of(n)[:50]!r}" for n in focused(ns)] or ["nothing"]


def wait_for(pred, what, timeout=30.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(session.nodes())
        if hit:
            return hit
        time.sleep(0.25)
    fail(f"timed out waiting for {what}; focus is on {describe_focus(session.nodes())}")


def document(node):
    return node.get("text", {}).get("document_text", "")


def prose_focus(ns):
    return [n for n in focused(ns) if n.get("role") == "MultilineTextInput"]


def stays(pred, what, seconds=1.5):
    """`pred` holds now and is still holding `seconds` later: a focus that lands and is
    then parked passes a plain wait."""
    wait_for(pred, what)
    time.sleep(seconds)
    session.settle(3)
    if not pred(session.nodes()):
        fail(f"{what} did not last; focus is on {describe_focus(session.nodes())}")


def heard():
    global seq
    session.settle(3)
    items = session.call("pull_announcements", {"since_seq": seq})
    if isinstance(items, dict):
        items = items.get("announcements", [])
    if items:
        seq = max(a["seq"] for a in items)
    return [a["text"] for a in items]


def key(name, **kw):
    session.call("inject_key", {"key": name, **kw})
    session.settle(2)


def ctrl_shift(letter):
    key(letter, command=True, shift=True)


def click(node):
    b = node["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                    "button": "primary"})


def binder_row(ns, title):
    return [n for n in ns if n.get("role") == "TreeItem" and text_of(n).strip().startswith(title)]


def in_mode(ns):
    return [n for n in ns if n.get("role") == "Button"
            and text_of(n).strip() == "Exit distraction-free mode"]


def enter_with_f11():
    key("F11")
    wait_for(in_mode, "the mode's strip")


def leave_with_f11():
    key("F11")
    stays(prose_focus, "the keyboard in the text after leaving the mode")


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "df-focus-probe", mcp=os.environ.get("TEKSILO_MCP_BIN"))
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    row = wait_for(lambda ns: binder_row(ns, "Prologue"), "the binder to list the Prologue")[0]
    click(row)
    wait_for(prose_focus, "the Prologue's text to take the focus")

    print("== 1. F11 from the text ==")
    # Off the start of the document, so that the caret proves something.
    key("Home", command=True)
    for _ in range(CARET):
        key("ArrowRight")
    session.settle(3)
    before = prose_focus(session.nodes())[0]
    enter_with_f11()
    stays(prose_focus, "the keyboard in the mode's text")
    inside = prose_focus(session.nodes())[0]
    print("  ok:", describe_focus(session.nodes()))
    if document(before) != document(inside):
        fail(f"another document: {document(before)[:40]!r} and {document(inside)[:40]!r}")
    print("  ok: the same document")

    print("== 2. Typing goes into it, at the same caret ==")
    session.call("type_text", {"node": inside["id"], "text": "Zq"})
    session.settle(3)
    now = prose_focus(session.nodes())
    if not now or "Zq" not in document(now[0]):
        fail(f"the typing went nowhere: {describe_focus(session.nodes())}")
    if document(now[0]).index("Zq") != CARET:
        fail(f"typed at {document(now[0]).index('Zq')}, the caret was at {CARET}")
    print("  ok: the text took it at", CARET)

    print("== 3. The ways out ==")
    leave_with_f11()
    print("  ok: F11")
    enter_with_f11()
    stays(prose_focus, "the keyboard in the mode's text")
    click(in_mode(session.nodes())[0])
    stays(prose_focus, "the keyboard in the text after the Exit button")
    print("  ok: the Exit button")

    print("== 4. F11 from the binder ==")
    ctrl_shift("b")
    wait_for(lambda ns: focused(ns) and not prose_focus(ns), "the binder to take the focus")
    print("  in the binder:", describe_focus(session.nodes()))
    enter_with_f11()
    stays(prose_focus, "the keyboard in the mode's text")
    print("  ok:", describe_focus(session.nodes()))
    leave_with_f11()

    print("== 5. The other doors ==")
    button = [n for n in session.nodes() if "istraction" in text_of(n) and n.get("role") == "Button"]
    if not button:
        fail("no distraction-free button in the title bar")
    click(button[0])
    wait_for(in_mode, "the mode's strip")
    stays(prose_focus, "the keyboard in the mode's text")
    print("  ok: the title bar's button")
    leave_with_f11()

    menu = [n for n in session.nodes() if text_of(n).strip() == "Menu"][0]
    click(menu)
    time.sleep(1.0)
    session.settle(3)
    view = wait_for(lambda ns: [n for n in ns if n.get("role") == "MenuItem"
                                and text_of(n).replace("&", "").strip() == "View"], "the menu")[0]
    click(view)
    time.sleep(1.0)
    session.settle(3)
    row = wait_for(lambda ns: [n for n in ns if n.get("role") == "MenuItemCheckBox"
                               and "istraction" in text_of(n)], "the row")[0]
    click(row)
    wait_for(in_mode, "the mode's strip")
    stays(prose_focus, "the keyboard in the mode's text")
    print("  ok: the View menu's row")
    leave_with_f11()

    print("== 6. A page with no text ==")
    click(binder_row(session.nodes(), "Starforgers")[0])
    pace = wait_for(lambda ns: [n for n in ns if n.get("role") == "RadioButton"
                                and text_of(n).strip() == "Pace"], "the book's views")[0]
    session.call("invoke_action", {"node": pace["id"], "action": "click"})
    time.sleep(1.0)
    session.settle(3)
    enter_with_f11()
    stays(lambda ns: [n for n in focused(ns) if not in_mode([n])],
          "the keyboard in the page")
    print("  ok:", describe_focus(session.nodes()))
    key("F11")
    time.sleep(1.0)
    session.settle(3)

    print("== 7. Nothing open ==")
    for _ in range(30):
        ns = session.nodes()
        if not [n for n in ns if n.get("role") == "Tab"]:
            break
        key("F4", command=True)
        time.sleep(0.2)
    heard()
    enter_with_f11()
    stays(focused, "the keyboard on the strip")
    print("  ok:", describe_focus(session.nodes()))
    said = heard()
    if not any("no text to type in" in t for t in said):
        fail(f"the mode did not say there is nothing to type in: {said}")
    print("  ok: said", said)
    key("F11")
    wait_for(lambda ns: not in_mode(ns), "the mode to end")
    stays(focused, "the keyboard somewhere after leaving with nothing open")
    print("  ok: leaving:", describe_focus(session.nodes()))

    print("PASS")
finally:
    if session:
        session.close()
    if app.poll() is None:
        app.terminate()
