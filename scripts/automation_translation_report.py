#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe: Help ▸ Report a Translation Error…, from the menu to the link.

Runs in French, over a project, with `SKRIBISTO_LINK_SINK` set so the link the app
would hand to the browser is written to a file instead (a debug-build facility of
`shared::external_link`). Asserts, against the running app:

  1. Aide ▸ Signaler une erreur de traduction… opens the report window;
  2. typing in Trouver le texte lists the matches, and the field keeps every
     character typed (a field rebuilt under the fingers keeps only the last one);
  3. picking "Annuler" shows its card and prefills Meilleure traduction;
  4. with a kind chosen, Continuer sur GitHub writes one link to the sink: the
     translation form, every field the report fills, the machine block, and a
     sign-in address within the 4,000-character budget GitHub accepts;
  5. Copier le lien and Signaler autre chose then appear;
  6. a 1,000-character Japanese suggestion still yields a link within the budget,
     with the overflow note in place of the suggestion;
  7. Signaler autre chose clears the search for the next report.

After 4 and 6 it also walks the window's layout tree, in the real fonts, and fails
if anything paints past the window's right edge (a combo box sized to a long kind
once made the whole report column wider than the window).

Run: python3 scripts/automation_translation_report.py [project.skrib]
"""
import os
import subprocess
import sys
import tempfile
import time
import urllib.parse

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

NBSP = " "
WINDOW = "translation-report"
BUDGET = 4000
SIGN_IN = "https://github.com/login?return_to="
UNRESERVED = "-._~"

SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1
    else fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib"))
PROJECT = fixture.working_copy(SRC, label="translation-report")
SINK = tempfile.NamedTemporaryFile(suffix=".links", delete=False).name
ENV = fixture.isolated_config(locale="fr-FR", label="translation-report", show_welcome=False)
ENV["SKRIBISTO_LINK_SINK"] = SINK
PINS = fixture.config_pins_file({"ui.locale": "fr-FR", "ui.show_welcome": False},
                                label="translation-report")

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


def editor_text(n):
    """A multiline editor's text: the bridge reports it as its document, not a value."""
    return ((n or {}).get("text") or {}).get("document_text") or (n or {}).get("value") or ""


def text_of(n):
    return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))


def windows():
    rows = session.call("list_windows")
    return rows if isinstance(rows, list) else rows.get("windows", [])


def window(name):
    return next((w for w in windows() if w.get("label") == name), None)


def project_window():
    return next((w for w in windows() if w.get("label") not in ("help", "launcher", WINDOW)),
                None)


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


def click(node, window_id=None):
    """The node's click action; the pointer at its centre where the node refuses it
    (a combo box's dropdown row lives in an overlay that takes pointer input only)."""
    try:
        session.call("invoke_action", {"action": "click", "node": node["id"]})
    except fixture.McpError as e:
        if "UNHANDLED_ACTION" not in str(e):
            raise
        b = node["bounds"]
        args = {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                "button": "primary"}
        if window_id is not None:
            args["window_id"] = window_id
        session.call("inject_pointer", args)
    session.settle(4)


def sign_in_length(url):
    rest = url.split("://", 1)[1]
    path_and_query = rest[rest.index("/"):]
    return len(SIGN_IN) + len(urllib.parse.quote(path_and_query, safe=UNRESERVED))


def wider_than_the_window(window_id):
    """Active widgets that paint past the window's right edge, in the real fonts."""
    tree = session.call("layout_tree", {"window_id": window_id})
    nodes = tree.get("nodes", [])
    roots = set(tree.get("roots", []))
    width = max((n["bounds"]["width"] for n in nodes if n.get("id") in roots), default=0)
    if len(nodes) < 50 or width < 500:
        fail(f"the layout tree is not the report window's: {len(nodes)} nodes, {width} wide")
    return [f"{n.get('type_name', '?').rsplit('::', 1)[-1]} "
            f"{n['bounds']['x']:.0f}..{n['bounds']['x'] + n['bounds']['width']:.0f}"
            for n in nodes
            if n.get("active", True) and n["bounds"]["width"] > 0
            and n["bounds"]["x"] + n["bounds"]["width"] > width + 0.5]


def check_fits(window_id, state):
    wide = wider_than_the_window(window_id)
    if wide:
        fail(f"{state}: {len(wide)} widgets paint past the window: {wide[:8]}")
    print(f"OK     nothing paints past the window ({state})")


def links():
    with open(SINK, encoding="utf-8") as handle:
        return [line.strip() for line in handle if line.strip()]


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "translation-report-probe")
except (RuntimeError, fixture.McpError) as e:
    fail(str(e))

try:
    win = wait_for(project_window, "the project window", timeout=90)["id"]

    # 1. The menu row opens the window.
    click(wait_for(lambda: node_labelled(win, "Menu"), "the menu button"))
    click(wait_for(lambda: node_labelled(win, "Aide", "MenuItem"), "the Aide menu"))
    click(wait_for(lambda: node_labelled(win, "Signaler une erreur de traduction…", "MenuItem"),
                   "Aide ▸ Signaler une erreur de traduction…"))
    report = wait_for(lambda: window(WINDOW), "the report window")["id"]
    print("OK  1. Aide ▸ Signaler une erreur de traduction… opened the report window")

    # 2. Typing lists the matches and keeps every character.
    field = wait_for(lambda: node_labelled(report, "Trouver le texte"), "the search field")
    session.call("type_text", {"node": field["id"], "text": "annuler"})
    session.settle(6)
    typed = node_labelled(report, "Trouver le texte")
    if (typed or {}).get("value") != "annuler":
        fail(f"the search field holds {(typed or {}).get('value')!r}, not 'annuler'")
    row = wait_for(lambda: next((n for n in nodes(report) if label(n) == "Annuler"
                                 and n.get("role") != "TextInput"), None),
                   "a result reading Annuler")
    print("OK  2. typing listed the matches, and the field kept 'annuler'")

    # 3. Picking it shows the card and prefills the suggestion.
    click(row)
    wait_for(lambda: node_labelled(report, "Quel endroit" + NBSP + "?"), "the card's place picker")
    # The editor's accessible text can trail its document by a frame or two.
    wait_for(lambda: "nnuler" in editor_text(node_labelled(report, "Meilleure traduction")),
             "Meilleure traduction to be prefilled", timeout=10)
    print("OK  3. picking Annuler showed its card and prefilled the suggestion")

    # 4. Continue writes one link within the budget, with every field.
    # The kind, from the keyboard, as a screen-reader user picks it: focus the picker,
    # Enter opens its list on the first row, Enter again chooses that row. (The
    # bridge's click opens the list without moving the keyboard focus, which a real
    # pointer click would.)
    picker = wait_for(lambda: node_labelled(report, "Qu’est-ce qui ne va pas" + NBSP + "?"),
                      "the kind picker")
    session.call("invoke_action", {"action": "focus", "node": picker["id"]})
    session.settle(3)
    for name in ("Enter", "Enter"):
        session.call("inject_key", {"key": name, "window_id": report})
        session.settle(3)
    kind = wait_for(lambda: (lambda k: k if "Le sens est faux" in text_of(k or {}) else None)(
        node_labelled(report, "Qu’est-ce qui ne va pas" + NBSP + "?")),
        "the kind to read Le sens est faux", timeout=10)
    go = wait_for(lambda: node_labelled(report, "Continuer sur GitHub"), "Continue")
    if go.get("disabled") or go.get("enabled") is False:
        fail(f"Continuer sur GitHub is still disabled with a kind and a suggestion: {go}")
    click(go)
    url = wait_for(lambda: (links() or [None])[-1], "a link in the sink")
    query = urllib.parse.parse_qs(urllib.parse.urlsplit(url).query)
    for key in ("template", "title", "language", "message", "kind", "report-data",
                "suggestion", "current", "english"):
        if key not in query:
            fail(f"the link has no {key} field: {url[:300]}")
    if query["template"] != ["translation-error.yml"] or query["language"] != ["fr-FR"]:
        fail(f"wrong template or language: {query['template']} {query['language']}")
    if "skribisto-translation-report: 1" not in query["report-data"][0]:
        fail(f"no machine block: {query['report-data']}")
    if sign_in_length(url) > BUDGET:
        fail(f"the sign-in address would be {sign_in_length(url)} characters")
    print(f"OK  4. Continue wrote the link ({sign_in_length(url)} characters once re-encoded)")
    check_fits(report, "a card and a kind")

    # 5. Copy link and Report another appear.
    wait_for(lambda: node_labelled(report, "Copier le lien"), "Copier le lien")
    wait_for(lambda: node_labelled(report, "Signaler autre chose"), "Signaler autre chose")
    print("OK  5. Copier le lien and Signaler autre chose appeared")

    # 6. A long Japanese suggestion overflows to the clipboard and keeps the budget.
    editor = node_labelled(report, "Meilleure traduction")
    session.call("type_text", {"node": editor["id"], "text": "語" * 1000})
    session.settle(6)
    before = len(links())
    click(node_labelled(report, "Continuer sur GitHub"))
    wait_for(lambda: len(links()) > before, "a second link")
    long_url = links()[-1]
    long_query = urllib.parse.parse_qs(urllib.parse.urlsplit(long_url).query)
    if sign_in_length(long_url) > BUDGET:
        fail(f"the long report's sign-in address would be {sign_in_length(long_url)}")
    if "did not fit" not in long_query.get("suggestion", [""])[0]:
        fail("the long suggestion was not replaced by the overflow note")
    print(f"OK  6. the long report kept the budget ({sign_in_length(long_url)}) with the note; "
          f"fields {sorted(long_query)}")
    check_fits(report, "a 1,000-character suggestion")
    # Node ids are snapshot-scoped: look the button up again before using it.
    click(wait_for(lambda: node_labelled(report, "Signaler autre chose"), "Signaler autre chose"))
    wait_for(lambda: not ((node_labelled(report, "Trouver le texte") or {}).get("value") or ""),
             "Signaler autre chose to clear the search", timeout=10)
    print("OK  7. Signaler autre chose cleared the search for the next report")
    print("PASS: the translation report goes from the menu to a link within GitHub's limits")
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
    try:
        os.unlink(SINK)
    except OSError:
        pass
