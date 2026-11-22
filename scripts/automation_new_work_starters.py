#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: the New Work starter pickers say what they create, and lead back to none.

On the wizard's last page, Organization, the Tags, Workflow and Note templates pickers each show
a line under the list naming what the current choice lays down. "None" used to be
only the placeholder of the Tags, Note templates and Book structure pickers, so
once the writer had picked something there was no way back to it; it is now the
first item of each list. Workflow has no "none" answer, and shows its default as
its value rather than as a placeholder, which a screen reader announced as no
value at all.

Asserts, against the running app (Ctrl+N over a project, en-US):

  1. on the Organization page, Tags reads "No tags", Note templates "No templates" and
     Workflow "Drafting", and only the Stages line is shown;
  2. picking Fantasy, Plume Creator and Essentials shows each choice's names;
  3. picking "No tags", "Drafting" and "No templates" again works, and hides the
     Tags and Templates lines;
  4. on Language & structure, Book structure offers "No structure" as an item,
     and choosing it after a preset sticks.

Nothing is created: the probe cancels the wizard.

Run: python3 scripts/automation_new_work_starters.py [project.skrib]
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
PROJECT = fixture.working_copy(SRC, label="new-work-starters")
ENV = fixture.isolated_config(locale="en-US", label="new-work-starters", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "en-US", "ui.show_welcome": False},
                                label="new-work-starters")

LINE_PREFIXES = ("Tags:", "Stages:", "Templates:")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=ENV)
session = None


def stop():
    if session:
        session.close()
    if app.poll() is None:
        app.kill()


def fail(message):
    print("FAIL:", message)
    print("--- app log tail ---")
    print("\n".join(open(log, encoding="utf-8", errors="replace").read().splitlines()[-25:]))
    stop()
    sys.exit(1)


def nodes():
    # The project window is the only one, so no call names a window: the bridge
    # targets it by default. (`list_windows` answers with a bare list, which
    # `McpSession.call` reads as an empty payload.)
    return session.nodes()


def wait_for(pred, what, timeout=30.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred()
        if hit:
            return hit
        time.sleep(0.4)
    fail(f"timed out waiting for {what}")


def combo(label):
    hit = [n for n in nodes() if n.get("role") == "ComboBox" and n.get("label") == label]
    if not hit:
        fail(f"no combo box labelled {label!r}")
    return hit[0]


def click_button(label):
    hit = [n for n in nodes() if n.get("role") == "Button" and n.get("label") == label]
    if not hit:
        fail(f"no button labelled {label!r}")
    session.call("invoke_action", {"action": "click", "node": hit[-1]["id"]})
    session.settle(5)


def pick(label, option):
    """Open `label`'s list and click `option` in it.

    By pointer, not focus + Enter: an option advertises only `focus`, and Enter
    goes to the wizard, where it means Next (Create Work on the last page).
    """
    session.call("invoke_action", {"action": "click", "node": combo(label)["id"]})
    session.settle(5)
    offered = [n for n in nodes() if n.get("role") == "ListBoxOption"]
    hit = [n for n in offered if n.get("label") == option]
    if not hit:
        fail(f"{label} does not offer {option!r}; it offers "
             f"{[n.get('label') for n in offered]}")
    b = hit[0]["bounds"]
    session.call("inject_pointer", {"x": b["x"] + b["width"] / 2,
                                    "y": b["y"] + b["height"] / 2,
                                    "action": "click"})
    session.settle(5)
    value = combo(label).get("value")
    if value != option:
        fail(f"{label} reads {value!r} after picking {option!r}")


def lines():
    return {str(n.get("value")).split(":", 1)[0]: str(n.get("value"))
            for n in nodes() if n.get("role") == "Label"
            and str(n.get("value") or "").startswith(LINE_PREFIXES)}


def expect_lines(wanted, step):
    got = lines()
    if set(got) != set(wanted):
        fail(f"{step}: lines shown {sorted(got)}, expected {sorted(wanted)}: {got}")
    for prefix, names in wanted.items():
        for name in names:
            if name not in got[prefix]:
                fail(f"{step}: the {prefix} line {got[prefix]!r} lacks {name!r}")


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "new-work-starters-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    wait_for(lambda: any(n.get("role") == "TreeItem" for n in nodes()),
             "the binder to list the project", timeout=90)

    session.call("inject_key", {"key": "n", "command": True})
    session.settle(5)
    name = wait_for(lambda: next((n for n in nodes() if n.get("role") == "TextInput"), None),
                    "the New Work form")
    session.call("set_value", {"node": name["id"], "value": "Starter Probe"})
    session.settle(3)
    for _ in range(3):
        click_button("Next")
    wait_for(lambda: any(n.get("role") == "ComboBox" and n.get("label") == "Tags"
                         for n in nodes()), "the Organization page")

    # 1. What an untouched page shows.
    start = {label: combo(label).get("value") for label in ("Tags", "Workflow", "Note templates")}
    if start != {"Tags": "No tags", "Workflow": "Drafting", "Note templates": "No templates"}:
        fail(f"the pickers open on {start}")
    expect_lines({"Stages": ["To do", "Draft", "Revised", "Final"]}, "untouched")
    print("PASS 1: the pickers open on No tags, Drafting and No templates")

    # 2. A choice in each, and the names it lays down.
    pick("Tags", "Fantasy")
    pick("Workflow", "Plume Creator")
    pick("Note templates", "Essentials")
    expect_lines({"Tags": ["character", "place", "creature", "magic system"],
                  "Stages": ["1st draft", "Proofread", "Finished"],
                  "Templates": ["Character sheet", "Location", "Research note"]}, "picked")
    print("PASS 2: each choice lists what it creates")

    # 3. And back to none.
    pick("Tags", "No tags")
    pick("Workflow", "Drafting")
    pick("Note templates", "No templates")
    expect_lines({"Stages": ["To do", "Final"]}, "back to none")
    print("PASS 3: No tags and No templates can be chosen again, and hide their lines")

    # 4. Book structure, two pages before.
    click_button("Back")
    click_button("Back")
    wait_for(lambda: any(n.get("role") == "ComboBox" and n.get("label") == "Book structure"
                         for n in nodes()), "the Language & structure page")
    pick("Book structure", "US trade novel")
    pick("Book structure", "No structure")
    print("PASS 4: Book structure offers No structure as an item")

    click_button("Cancel")
except fixture.McpError as e:
    fail(str(e))

stop()
print("ALL PASS")
