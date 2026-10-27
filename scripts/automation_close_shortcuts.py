#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: Ctrl+W closes Settings and the Help window, and F1 works after.

Settings is a dialog inside the project window, so the project window's own
Ctrl+W (Close Work) cannot reach into it, and until the panel registered one of
its own the chord did nothing there. The Help window is a window of its own,
whose registry held no Ctrl+W at all. And on Windows, F1 was once seen doing
nothing right after Settings closed.

Asserts, against the running app, for each way out of Settings (Escape, Ctrl+W,
the header's close button, Done):

  1. Ctrl+, opens Settings over a project;
  2. the way out closes it, and the project stays open with no question asked;
  3. F1 then opens the Help window;
  4. Ctrl+W in the Help window closes it, and the project window is still there;

then that Ctrl+W in the project window is Close Work again: it closes the project.

`command`, never `ctrl`: a shortcut declared on Ctrl is the Command chord on macOS.

Run: python3 scripts/automation_close_shortcuts.py [project.skrib]
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
PROJECT = fixture.working_copy(SRC, label="close-shortcuts")
ENV = fixture.isolated_config(locale="en-US", label="close-shortcuts", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "en-US", "ui.show_welcome": False},
                                label="close-shortcuts")

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


def windows():
    rows = session.call("list_windows")
    return rows if isinstance(rows, list) else rows.get("windows", [])


def window(label):
    return next((w for w in windows() if w.get("label") == label), None)


def project_window():
    return next((w for w in windows() if w.get("label") not in ("help", "launcher")), None)


def nodes(window_id):
    return session.call("snapshot_tree", {"window_id": window_id}).get("nodes", [])


def labels(window_id):
    return {n.get("label") or "" for n in nodes(window_id)}


def wait_for(pred, what, timeout=30.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred()
        if hit:
            return hit
        time.sleep(0.4)
    fail(f"timed out waiting for {what}")


def key(window_id, name, **mods):
    session.call("inject_key", {"key": name, "window_id": window_id, **mods})
    session.settle(3)


def settings_open(window_id):
    # The instant-apply footer is on every page, whichever one opens first.
    return {"Reset to defaults", "Done"} <= labels(window_id)


def asks_to_save(window_id):
    return any("save changes" in text_of(n).lower() for n in nodes(window_id))


def click_label(window_id, label):
    hit = [n for n in nodes(window_id) if n.get("label") == label]
    if not hit:
        fail(f"nothing labelled {label!r} in Settings")
    session.call("invoke_action", {"action": "click", "node": hit[-1]["id"]})
    session.settle(3)


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "close-shortcuts-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    win = wait_for(project_window, "the project window")["id"]
    wait_for(lambda: any(n.get("role") == "TreeItem" and text_of(n).strip().startswith("Prologue")
                         for n in nodes(win)), "the binder to list the Prologue", timeout=90)

    routes = {
        "Escape": lambda: key(win, "Escape"),
        "Ctrl+W": lambda: key(win, "w", command=True),
        "the close button": lambda: click_label(win, "Close"),
        "Done": lambda: click_label(win, "Done"),
    }
    for route, close in routes.items():
        # 1. Ctrl+, opens Settings.
        key(win, ",", command=True)
        wait_for(lambda: settings_open(win), "Settings to open", timeout=10)

        # 2. The way out closes it and leaves the project open.
        close()
        wait_for(lambda: not settings_open(win), f"{route} to close Settings", timeout=10)
        if asks_to_save(win) or project_window() is None:
            fail(f"{route} in Settings went to the project window: it asked to close the work")

        # 3. F1 opens Help right after.
        key(win, "F1")
        help_win = wait_for(lambda: window("help"), f"F1 to open Help after {route}",
                            timeout=10)["id"]

        # 4. Ctrl+W closes Help, not the project.
        key(help_win, "w", command=True)
        wait_for(lambda: window("help") is None, "Ctrl+W to close the Help window",
                 timeout=10)
        if project_window() is None:
            fail("Ctrl+W in the Help window closed the project window too")
        print(f"OK  Settings closed by {route}; F1 opened Help, Ctrl+W closed it")

    # 5. Back on the page, Ctrl+W is Close Work again.
    key(win, "w", command=True)
    wait_for(lambda: project_window() is None or asks_to_save(win),
             "Ctrl+W in the project window to close the work", timeout=15)
    print("OK  Ctrl+W in the project window closes the work again")
    print("PASS: close shortcuts verified")
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
