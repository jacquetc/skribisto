#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: a quit nobody can answer saves the project first.

`SIGTERM` (a service manager stopping the session, `kill`) and `SIGHUP` reach the
app through teksilo as a quit that overrules every close guard, so a "Save
changes?" question would never be seen. The project window's guard therefore
saves without asking (`QuitSequencer::close_without_asking`), blocking until the
write has landed, and lets the window close.

Asserts, against the running app, with autosave off (the setting under which a
plain close would have asked):

  1. a scene opens and takes typed text that is not yet in the file;
  2. `SIGTERM` ends the app, with exit status 0;
  3. the file on disk now holds the typed text.

Before teksilo 0.21 the signal killed the process outright, and step 3 failed.

Unix only: Windows has no signal to send a windowed app (its equivalent quit,
the session ending, cannot be started from a probe without ending the session).

Run: python3 scripts/automation_quit_without_asking.py [project.skrib]
"""
import os
import signal
import subprocess
import sys
import tempfile
import time
import zipfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

if fixture.IS_WINDOWS:
    fixture.skip("Windows has no termination signal for a windowed app to send")

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
PROJECT = fixture.working_copy(SRC, label="quit-signal")
ENV = fixture.isolated_config(locale="en-US", label="quit-signal", show_welcome=False,
                              pins={"editor.autosave": False})
PINS = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.show_welcome": False, "editor.autosave": False},
    label="quit-signal")
MARKER = "Saved before the signal 4815"

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


def editor_text(node, ns):
    by_id = {n["id"]: n for n in ns if "id" in n}
    parts = []

    def walk(n):
        for k in ("label", "value", "name"):
            if n.get(k):
                parts.append(str(n[k]))
                break
        for child in n.get("children") or []:
            if child in by_id:
                walk(by_id[child])

    walk(node)
    return " ".join(parts)


def prose_editor(ns):
    editors = [n for n in ns if n.get("role") == "MultilineTextInput"
               and "set_value" in (n.get("actions") or [])]
    return max(editors, key=lambda e: len(editor_text(e, ns)), default=None)


def file_holds(path, text):
    with zipfile.ZipFile(path) as bundle:
        return any(text.encode("utf-8") in bundle.read(name) for name in bundle.namelist())


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "quit-without-asking-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    # 1. A scene takes text that is not in the file yet.
    row = wait_for(lambda ns: [n for n in ns if n.get("role") == "TreeItem"
                               and text_of(n).strip().startswith("Prologue")],
                   "the binder to list the Prologue")[0]
    click(row)
    ns = wait_for(lambda ns: ns if prose_editor(ns) is not None
                  and len(editor_text(prose_editor(ns), ns)) > 40 else None,
                  "a prose editor with text")
    editor = prose_editor(ns)
    b = editor["bounds"]
    session.call("inject_pointer", {"x": b["x"] + min(b["width"] / 2, 200.0),
                                    "y": max(b["y"] + 24.0, 24.0), "button": "primary"})
    session.settle(3)
    session.call("type_text", {"node": prose_editor(session.nodes())["id"], "text": MARKER})
    session.settle(3)
    ns = session.nodes()
    if MARKER not in editor_text(prose_editor(ns), ns):
        fail("the typed text did not reach the editor")
    if file_holds(PROJECT, MARKER):
        fail("the text is already in the file before the signal (is autosave really off?)")
    print("OK  1. the scene holds text the file does not")
except fixture.McpError as e:
    fail(str(e))

# 2. SIGTERM ends the app cleanly.
session.close()
session = None
app.send_signal(signal.SIGTERM)
try:
    code = app.wait(timeout=90)
except subprocess.TimeoutExpired:
    fail("the app was still running 90 s after SIGTERM")
if code != 0:
    fail(f"the app ended with status {code} on SIGTERM, not 0")
print("OK  2. SIGTERM ended the app with status 0")

# 3. The file holds what was typed.
if not file_holds(PROJECT, MARKER):
    fail("the typed text is not in the file: the quit did not save it")
if "could not be saved" in open(log, encoding="utf-8", errors="replace").read():
    fail("the app reported that the project could not be saved")
print("OK  3. the file on disk holds the typed text")

print("\nALL PASS")
