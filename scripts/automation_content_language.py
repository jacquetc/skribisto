#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: the prose is read in its own language, the interface in the window's.

Opens the bundled English example (its project language is `en_US`) in a French
interface and reads, from the running app's accessibility tree:

  1. the scene's prose editor is declared `en-US`: a screen reader that follows
     the language reads the manuscript as English, not with the French voice;
  2. the interface around it declares nothing of its own (the menu button reads in
     the window's French);
  3. Help ▸ Report a Translation Error…, with Japanese picked: the "Meilleure
     traduction" field is declared `ja-JP`, the explanation beside it nothing.

Run: python3 scripts/automation_content_language.py [project.skrib]
"""
import os
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

NBSP = " "
SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
PROJECT = fixture.working_copy(SRC, label="content-language")
ENV = fixture.isolated_config(locale="fr-FR", label="content-language", show_welcome=False)
PINS = fixture.config_pins_file({"ui.locale": "fr-FR", "ui.show_welcome": False},
                                label="content-language")

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


def label(n):
    return (n.get("label") or "").replace("&", "")


def windows():
    rows = session.call("list_windows")
    return rows if isinstance(rows, list) else rows.get("windows", [])


def nodes(window_id):
    return session.call("snapshot_tree", {"window_id": window_id}).get("nodes", [])


def wait_for(pred, what, timeout=30.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred()
        if hit:
            return hit
        time.sleep(0.4)
    fail(f"timed out waiting for {what}")


def node_labelled(window_id, wanted, role=None):
    return next((n for n in nodes(window_id) if label(n) == wanted
                 and (role is None or n.get("role") == role)), None)


def click(node):
    session.call("invoke_action", {"action": "click", "node": node["id"]})
    session.settle(4)


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "content-language-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    win = wait_for(lambda: next((w for w in windows()
                                 if w.get("label") not in ("help", "launcher")), None),
                   "the project window", timeout=90)["id"]

    # 1. The prose of an English scene, in a French window.
    row = wait_for(lambda: next((n for n in nodes(win) if n.get("role") == "TreeItem"
                                 and label(n).startswith("Prologue")), None),
                   "the Prologue row", timeout=60)
    click(row)
    session.call("inject_key", {"key": "Enter", "window_id": win})
    session.settle(6)
    editor = wait_for(lambda: next((n for n in nodes(win)
                                    if n.get("role") == "MultilineTextInput"
                                    and label(n) == "Texte"), None),
                      "the scene's prose editor", timeout=20)
    language = (editor.get("text") or {}).get("language")
    if language != "en-US":
        fail(f"the prose is read in {language!r}, not the project's en-US: {editor.get('text')}")
    print("OK  1. the English scene's prose is read in en-US inside a French window")

    # 2. The interface around it stays the window's.
    menu = wait_for(lambda: node_labelled(win, "Menu"), "the menu button")
    if menu.get("language") not in (None, "fr-FR"):
        fail(f"the menu button declares {menu.get('language')!r}")
    print("OK  2. the menu button declares nothing of its own (read in fr-FR)")

    # 3. The report window: the suggestion in the reported language.
    click(menu)
    click(wait_for(lambda: node_labelled(win, "Aide", "MenuItem"), "the Aide menu"))
    click(wait_for(lambda: node_labelled(win, "Signaler une erreur de traduction…",
                                         "MenuItem"), "the report row"))
    report = wait_for(lambda: next((w for w in windows()
                                    if w.get("label") == "translation-report"), None),
                      "the report window")["id"]
    picker = wait_for(lambda: node_labelled(report, "Langue"), "the language picker")
    session.call("invoke_action", {"action": "focus", "node": picker["id"]})
    session.settle(2)
    # Open the list, filter it by Japanese's own name, choose it.
    session.call("inject_key", {"key": "Enter", "window_id": report})
    session.settle(2)
    search = wait_for(lambda: next((n for n in nodes(report)
                                    if n.get("role") in ("SearchInput", "TextInput")
                                    and label(n) != "Trouver le texte"), None),
                      "the language list's filter", timeout=10)
    session.call("type_text", {"node": search["id"], "text": "日本"})
    session.settle(3)
    # The filter leaves one option, not yet highlighted: Down highlights it, Enter
    # chooses it, as a keyboard user does.
    session.call("inject_key", {"key": "ArrowDown", "window_id": report})
    session.settle(2)
    session.call("inject_key", {"key": "Enter", "window_id": report})
    session.settle(4)

    def editor_language(name):
        n = node_labelled(report, name)
        return ((n or {}).get("text") or {}).get("language") if n else "missing"

    wait_for(lambda: editor_language("Meilleure traduction") == "ja-JP",
             "Meilleure traduction to be declared ja-JP", timeout=10)
    explain = next((label(n) for n in nodes(report) if label(n).startswith("Expliquer")), None)
    if explain is None:
        fail("no explanation field")
    if editor_language(explain) not in (None, "fr-FR"):
        fail(f"the explanation declares {editor_language(explain)!r}; it is in any language")
    print("OK  3. Meilleure traduction is read in ja-JP; the explanation in the window's")
    print("PASS: each text is declared in its own language, and the interface in the window's")
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
