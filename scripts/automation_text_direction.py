#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: a text field reads in the direction of what is typed into it.

Since teksilo 0.27 every text field follows its text (`TextBaseDirection::Auto`):
the first strongly directional character, then the field's declared language, then
the window. This checks the three cases a writer meets, against the running app's
accessibility tree, in the search dock's query field:

  * French interface: the empty field reads left to right; Arabic typed into it
    reads right to left, keeps every character, and has a place on screen (before
    text-typeset 3.2 it was laid out at an infinite x and never drawn); cleared, it
    is left to right again.
  * Arabic and Hebrew interfaces: the empty field reads right to left, from the
    window; Latin typed into it reads left to right; cleared, right to left again.

Run: python3 scripts/automation_text_direction.py [project.skrib]
"""
import math
import os
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

RTL = "right_to_left"
ARABIC = "مرحبا بالعالم"
LATIN = "Hello world"

# (interface, what the empty field reads, what is typed, what it then reads)
CASES = [
    ("fr-FR", "ltr", ARABIC, "rtl"),
    ("ar-SA", "rtl", LATIN, "ltr"),
    ("he-IL", "rtl", LATIN, "ltr"),
]

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))


class Failure(Exception):
    pass


def reads(node):
    """`rtl` when the node declares a right-to-left base direction, else `ltr`."""
    return "rtl" if ((node or {}).get("text") or {}).get("direction") == RTL else "ltr"


def field_text(node):
    return ((node or {}).get("text") or {}).get("document_text") or (node or {}).get("value") or ""


def run(locale, empty_reads, typed, typed_reads):
    project = fixture.working_copy(SRC, label=f"text-direction-{locale}")
    env = fixture.isolated_config(locale=locale, label=f"text-direction-{locale}",
                                  show_welcome=False)
    pins = fixture.config_pins_file({"ui.locale": locale, "ui.show_welcome": False},
                                    label=f"text-direction-{locale}")
    log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
    app = subprocess.Popen(fixture.launch_argv(project, pins=pins), stdout=open(log, "w"),
                           stderr=subprocess.STDOUT, env=env)
    session = None
    try:
        bridge = fixture.wait_for_bridge(log, app, timeout=90)
        session = fixture.McpSession(bridge, f"text-direction-{locale}")

        def windows():
            rows = session.call("list_windows")
            return rows if isinstance(rows, list) else rows.get("windows", [])

        def wait_for(pred, what, timeout=60.0):
            end = time.time() + timeout
            while time.time() < end:
                hit = pred()
                if hit:
                    return hit
                time.sleep(0.4)
            raise Failure(f"timed out waiting for {what}")

        win = wait_for(lambda: next((w for w in windows()
                                     if w.get("label") not in ("help", "launcher")), None),
                       "the project window", timeout=90)["id"]

        def nodes():
            return session.call("snapshot_tree", {"window_id": win}).get("nodes", [])

        # Wait for the binder, so the dock's shortcut has a window to act in.
        wait_for(lambda: any(n.get("role") == "TreeItem" for n in nodes()), "the binder")
        session.call("inject_key", {"key": "f", "command": True, "shift": True,
                                    "window_id": win})
        session.settle(6)

        def query_field():
            # The search field is one node, a SearchInput, that carries its text.
            return next((n for n in nodes() if n.get("role") in ("SearchInput", "TextInput")),
                        None)

        field = wait_for(query_field, "the search dock's query field", timeout=20)
        if reads(field) != empty_reads:
            raise Failure(f"the empty field reads {reads(field)}, not {empty_reads}: "
                          f"{field.get('text')}")
        print(f"OK  {locale}: the empty field reads {empty_reads}")

        session.call("invoke_action", {"action": "focus", "node": field["id"]})
        session.call("type_text", {"node": field["id"], "text": typed})
        session.settle(6)
        field = wait_for(lambda: (lambda f: f if field_text(f) == typed else None)(query_field()),
                         f"the field to hold {typed!r}", timeout=10)
        if reads(field) != typed_reads:
            raise Failure(f"{typed!r} reads {reads(field)}, not {typed_reads}: {field.get('text')}")
        bounds = field.get("bounds") or {}
        if not all(math.isfinite(bounds.get(k, math.nan)) for k in ("x", "y", "width", "height")):
            raise Failure(f"the field holding {typed!r} has no place on screen: {bounds}")
        if not (field.get("text") or {}).get("has_geometry", False):
            raise Failure(f"the text {typed!r} reports no geometry: {field.get('text')}")
        print(f"OK  {locale}: {typed!r} reads {typed_reads}, whole, with its geometry")

        # Clear it the way a writer does: select everything, then delete.
        session.call("inject_key", {"key": "a", "command": True, "window_id": win})
        session.call("inject_key", {"key": "Backspace", "window_id": win})
        session.settle(6)
        field = wait_for(lambda: (lambda f: f if not field_text(f) else None)(query_field()),
                         "the field to empty", timeout=10)
        if reads(field) != empty_reads:
            raise Failure(f"cleared, the field reads {reads(field)}, not {empty_reads}")
        print(f"OK  {locale}: cleared, it reads {empty_reads} again")
    except (Failure, fixture.McpError, RuntimeError) as e:
        print(f"FAIL {locale}: {e}")
        print("--- app log tail ---")
        print("\n".join(open(log, encoding="utf-8", errors="replace").read().splitlines()[-20:]))
        return False
    finally:
        if session:
            session.close()
        if app.poll() is None:
            app.terminate()
            try:
                app.wait(timeout=10)
            except subprocess.TimeoutExpired:
                app.kill()
    return True


if __name__ == "__main__":
    results = [run(*case) for case in CASES]
    if all(results):
        print("PASS: a text field reads in the direction of its text, in every interface")
    else:
        sys.exit(1)
