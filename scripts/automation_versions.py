#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **Versions** dock end to end.

The dock reads a row's past out of routine backups and the in-project history
log, and compares one version with the one before it. None of that is visible
to a unit test: the timeline is filesystem work against real archives, and the
comparison is rendered as Djot that only becomes underline and strikeout once a
real document parses it.

    scripts/automation_versions.py PROJECT.skrib

It needs backups of that project to exist in the default backup root
(`~/.local/share/Skribisto/backups`) — with no past there is nothing to show and
the honest empty state is all this would prove.

Checks:
  1. The Versions dock is reachable from the **trailing** rail and renders.
  2. Its scope bar offers both Synopsis and Text — a timeline is meaningless
     until you say which content it is of.
  3. With backups present, the list carries dated rows rather than the empty
     state, and each row names where it came from.
  4. Selecting a row produces a comparison: a summary sentence saying what
     moved, and the two controls that make a long diff navigable.
  5. The comparison reaches a real document — the pane's text is prose, not
     leaked `{+…+}` markup, which is the one thing the escaping has to get right
     and the one thing a string test cannot prove.
  6. The date filter narrows what is **drawn**, not just what the selection
     resolves against. The list once ran its own filter beside the view-model's,
     and the two disagreed the moment a second filter existed: with a date range
     set, list position *n* and timeline index *n* named different versions, so
     the row a writer clicked was not the one that got diffed — nor the one
     Restore would have written back.
  7. The "didn't exist yet" boundary names a moment the row was **proved**
     absent, which is necessarily before anything the dock has a version of. It
     used to name the first *sighting* instead, asserting non-existence across a
     gap that nothing examined could speak for.

Checks 6 and 7 are data-dependent — 6 needs a backup older than the window, 7
needs a row whose creation was actually observed. Each says out loud when it did
not run, rather than passing quietly on a project that cannot exercise it.

The app is deliberately **left running** so a screenshot can be taken of the
state these checks left it in.
"""
import json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

project = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else None
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
print(f"app log: {log}")


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-25:]))
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


fixture.assert_no_running_instance()
# Run against an isolated config/data root when asked, so the probe reads the
# *default* backup location and never touches the real one. `SHOT_XDG=<dir>`.
#
# Deliberately **not** `fixture.isolated_config` here: this probe's whole point,
# absent that override, is to read backups out of the *operator's own* default
# backup root (see the module docstring) — an isolated sandbox with nothing in
# it would make every check below report "no dated version rows" honestly, but
# uselessly. `--config` writes into whatever `XDG_CONFIG_HOME` the process
# resolves (`cli::apply_config_pins`'s own doc comment says so), so pinning
# settings without `PROBE_XDG` set would permanently rewrite the operator's
# real `general.toml` — the file this run was explicitly trying not to touch.
# So the pins file is built only inside the isolated branch, where writing to
# `XDG_CONFIG_HOME` is this run's own scratch directory.
env = dict(os.environ)
xdg = os.environ.get("PROBE_XDG")
pins_file = None
if xdg:
    os.makedirs(os.path.join(xdg, "config"), exist_ok=True)
    os.makedirs(os.path.join(xdg, "data"), exist_ok=True)
    env["XDG_CONFIG_HOME"] = os.path.join(xdg, "config")
    env["XDG_DATA_HOME"] = os.path.join(xdg, "data")
    # Start from the default desk. A layout persisted by an earlier run can have
    # the trailing panel dragged to zero width, and a dock with no width is
    # indistinguishable from a dock that failed to open.
    for stale in ("workspace.toml",):
        try:
            os.remove(os.path.join(xdg, "config", "skribisto", stale))
        except FileNotFoundError:
            pass
    pins_file = fixture.config_pins_file(
        {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
        label="versions")
    print(f"isolated XDG root: {xdg}")
app = subprocess.Popen(fixture.launch_argv(project, pins=pins_file),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT, env=env)

try:
    bridge = fixture.wait_for_bridge(log, app, timeout=60)
except RuntimeError as e:
    die(str(e), app)

_id = [0]
mcp = None


def send(method, params=None, notif=False):
    m = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        m["params"] = params
    if not notif:
        _id[0] += 1
        m["id"] = _id[0]
    mcp.stdin.write(json.dumps(m) + "\n")
    mcp.stdin.flush()


def recv(timeout=25, fatal=True):
    e = time.time() + timeout
    while time.time() < e:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, e - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        if line.strip():
            return json.loads(line)
    if fatal:
        die("no MCP response", app, mcp)
    return None


def call(name, a=None):
    send("tools/call", {"name": name, "arguments": a or {}})
    res = recv().get("result", {})
    payload = res.get("structuredContent")
    if payload is None:
        txt = "".join(c.get("text", "") for c in res.get("content", []) if c.get("type") == "text")
        payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
    return res, payload


mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                       stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                       stderr=subprocess.DEVNULL, text=True, bufsize=1)
send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                    "clientInfo": {"name": "versions-probe", "version": "1"}})
if recv(timeout=20, fatal=False) is None:
    die("could not connect MCP", app, mcp)
send("notifications/initialized", notif=True)
print(f"bridge up: endpoint={bridge.endpoint} pid={bridge.pid}")


def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def rendered_text():
    """Every string the widget tree resolved, from both trees.

    The a11y snapshot alone is not enough: the bridge's `SemanticNode` DTO
    carries `label` and `value` but **not** `description`, and a
    `StandardListItem`'s subtitle goes to `description` (which a real screen
    reader does get). So a subtitle is invisible here unless the layout tree is
    read too.
    """
    out = []
    for n in nodes() + widgets():
        for field in ("label", "value", "text", "resolved_text", "params"):
            blob = n.get(field)
            if isinstance(blob, str) and blob.strip():
                out.append(blob)
            elif blob is not None and not isinstance(blob, str):
                out.append(json.dumps(blob))
    return out


def texts(ns):
    out = []
    for n in ns:
        for field in ("label", "value", "description"):
            v = n.get(field)
            if isinstance(v, str) and v.strip():
                out.append(v)
    return out


def find(label, timeout=6.0, minx=None):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            if n.get("label") == label:
                if minx is None or (n.get("bounds") or {}).get("x", 0) >= minx:
                    return n
        settle()
    return None


def activate(node):
    acts = node.get("actions") or []
    if "activate" in acts:
        call("invoke_action", {"node": node["id"], "action": "activate"})
    elif "click" in acts:
        call("invoke_action", {"node": node["id"], "action": "click"})
    else:
        b = node.get("bounds") or {}
        if "x" not in b:
            return False
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    settle()
    time.sleep(0.5)
    return True


def click(label, what="", minx=None):
    n = find(label, minx=minx)
    if not n:
        print(f"  !! no node labelled {label!r} {what}")
        return False
    return activate(n)


# Left edge of the trailing dock in a default window — enough to tell the
# Versions pane's controls from identically-labelled ones in the editor.
TRAILING_X = 800

failures = []


def want_text(substr, why):
    if any(substr.lower() in t.lower() for t in texts(nodes())):
        print(f"  {substr!r} present ✓")
        return True
    failures.append(f"{why}: no text containing {substr!r}")
    return False


# ── reach a loaded project ────────────────────────────────────────────────────
if project is None:
    die("this probe needs a real project with backups: pass a .skrib path", app, mcp)

end = time.time() + 120
while time.time() < end:
    if "Binder" in [n.get("label") for n in nodes() if n.get("label")]:
        break
    settle()
else:
    die("the project never finished loading", app, mcp)
settle()
print("project loaded")

# ── STEP 1: open a scene, so the trailing rail has something to be about ──────
# The dock follows the active editor tab's item, exactly as the Inspector and the
# per-document comments dock do. With no tab open there is no row to have a past.
rows = [n for n in nodes()
        if n.get("role") in ("TreeItem", "ListBoxOption")
        and (n.get("bounds") or {}).get("x", 999) < 320
        and (n.get("bounds") or {}).get("y", 0) > 100]
if not rows:
    die("no binder rows to open", app, mcp)
rows.sort(key=lambda n: ((n.get("label") or "") != "Chapter 5",
                         (n.get("bounds") or {}).get("y", 1e9)))
opened = False
for row in rows[:14]:
    activate(row)
    time.sleep(0.6)
    if any(n.get("role") == "MultilineTextInput" and
           (n.get("bounds") or {}).get("x", 0) > 320 for n in nodes()):
        opened = True
        print(f"opened {row.get('label')!r}")
        break
if not opened:
    die("clicking binder rows never opened a writing surface", app, mcp)
settle()

# ── STEP 2: the dock is on the trailing rail and renders ──────────────────────
# Trailing, not leading: versions of *this row* belong beside the Inspector and
# this-document comments — "whatever is in front of me right now".
def dock_showing():
    """The pin filter exists only inside the Versions panel.

    Checked rather than assumed, because the rail tab *toggles*: a workspace
    layout that already remembers the dock open turns the probe's click into a
    click that closes it, and every later step then fails for a reason that has
    nothing to do with what it was testing.
    """
    return any((n.get("label") or "") == "Show only pinned versions" for n in nodes())


if not dock_showing():
    if not click("Versions", "trailing rail"):
        die("the Versions dock is not reachable from any rail", app, mcp)
    settle()
    time.sleep(1.5)
if not dock_showing():
    die("the Versions dock did not open", app, mcp)
print("STEP 2 OK: the dock is showing")

# ── STEP 3: the scope bar names which content a timeline is of ────────────────
want_text("Synopsis", "STEP 3")
want_text("Text", "STEP 3")

# This example project keeps no synopses, so the default scope is honestly empty.
# Switch to the body, which is where its past actually is.
# The prose column has a section header labelled "Text" too, so require the
# trailing pane's x — otherwise which one is clicked is a race.
if not click("Text", "scope bar", minx=TRAILING_X):
    failures.append("STEP 3: the Text scope is not clickable")
time.sleep(1.5)
settle()

# ── STEP 4: with backups present, there are dated rows ────────────────────────
# The trailing pane starts around x=1000 in a default window; a date matched
# anywhere else would be the binder or the status bar.
ns = nodes()
dated = [t for t in texts(ns) if re.match(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$", t.strip())]
if not dated:
    failures.append("STEP 4: no dated version rows — the timeline found nothing "
                    "in the backup root, so nothing below this proves anything")
    print("  !! no dated rows")
else:
    print(f"STEP 4 OK: {len(dated)} version row(s) — e.g. {dated[:4]}")
    # Where a version lives is not decoration: "only in a backup on a drive you
    # last plugged in a month ago" is a different promise from "in your project
    # file", and a date alone cannot say which. It rides the row's a11y
    # description, so look there rather than at labels.
    # A row's *source* ("From a backup" / "From the project's own history")
    # rides the `StandardListItem` subtitle, which goes to the AccessKit `description`
    # field — and the automation bridge's `SemanticNode` DTO carries `label` and
    # `value` but **not** `description`, while `layout_tree` carries no text at
    # all. So the string itself is out of reach here; what is checkable is that
    # every row really does lay out a second line under its date, and that the
    # magnitude bar is there.
    rows_boxes = [n for n in widgets()
                  if n.get("type", "").endswith("standard_item::StandardListItem")
                  and 850 <= (n.get("bounds") or {}).get("x", -1) <= 1160]
    subtitled = 0
    for row in rows_boxes:
        rb = row["bounds"]
        texts_in = [n for n in widgets()
                    if n.get("type", "").endswith("text_widget::TextWidget")
                    and rb["y"] <= (n.get("bounds") or {}).get("y", -1) < rb["y"] + rb["height"]]
        if len(texts_in) >= 2:
            subtitled += 1
    if rows_boxes and subtitled == len(rows_boxes):
        print(f"  all {subtitled} row(s) carry a second line (the source) \u2713")
    else:
        failures.append(f"STEP 4: {len(rows_boxes) - subtitled} row(s) show only a date, "
                        "with nothing saying where the version came from")
    bars = [n for n in widgets()
            if n.get("type", "").endswith("progress_bar::ProgressBar")
            and 1050 <= (n.get("bounds") or {}).get("x", -1) <= 1160]
    if bars:
        print(f"  {len(bars)} magnitude bar(s) \u2713")
    elif len(rows_boxes) <= 1:
        # A magnitude is "how much this changed against the state below it", so
        # the *only* recorded state has none by design — drawing a full bar there
        # would claim an edit nobody ever observed. Reported rather than passed
        # over: a project with one version proves nothing about the bar.
        print("  !! only one recorded state, so no magnitude to draw \u2014 "
              "this check did not run")
    else:
        failures.append("STEP 4: no magnitude bar on any row")

# ── STEP 5: selecting a version produces a comparison ─────────────────────────
# The **oldest** row when a real restore is going to follow: it is the only one
# whose text differs from what the project already holds, so it is the only one
# whose landing can be observed at all.
dated_rows = [n for n in ns
              if re.match(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$", (n.get("label") or "").strip())
              and n.get("role") == "ListBoxOption"]
dated_rows.sort(key=lambda n: (n.get("label") or ""))
if not dated_rows:
    picked = None
elif os.environ.get("PROBE_RESTORE"):
    picked = dated_rows[0]
else:
    picked = dated_rows[-1]
if picked is None:
    failures.append("STEP 5: no version row to select")
else:
    activate(picked)
    time.sleep(1.2)
    settle()
    blob = " ".join(texts(nodes()))
    # One of: a real comparison, the earliest-on-record state, or a
    # formatting-only change. All three are answers; an empty pane is not.
    answered = any(s in blob for s in
                   ("words added", "words removed", "word added", "word removed",
                    "paragraph moved", "paragraphs moved",
                    "earliest version on record", "Only the formatting changed",
                    "Nothing changed in this part"))
    if answered:
        print("STEP 5 OK: the pane answered")
    else:
        failures.append("STEP 5: selecting a version produced no summary at all")
    # The controls that make a long comparison navigable — only meaningful when
    # there *is* a comparison. In restore mode the oldest version is selected on
    # purpose (it is the only one whose text differs from the project's), and it
    # has nothing older to be compared against; saying so is the right answer
    # there, and the right thing to assert.
    if os.environ.get("PROBE_RESTORE"):
        if "earliest version on record" in blob:
            print("  the earliest version says so rather than showing an empty diff ✓")
        else:
            failures.append("STEP 5: the earliest version did not name its own state")
    elif "earliest version on record" in blob:
        # This project keeps only one recorded state of the row — two backups
        # holding byte-identical prose collapse into the earliest — so there is
        # no comparison for these controls to navigate. Said out loud rather
        # than skipped quietly: a green run would otherwise imply they were
        # checked. Point this at a project with a real edit history to exercise
        # them.
        print("  !! the only recorded state is the earliest, so there is no "
              "comparison — the diff controls were not checked")
    else:
        for label in ("Show unchanged", "Hide unchanged"):
            if label in blob:
                print(f"  {label!r} present ✓")
                break
        else:
            failures.append("STEP 5: no show/hide-unchanged control")
        if "Next change" in blob:
            print("  'Next change' present ✓")
        else:
            failures.append("STEP 5: no jump-to-next-change control")

    # ── STEP 6a: pinning ──────────────────────────────────────────────────────
    # A pin protects a backup **file** from a retention sweep. Only a backup can
    # carry one — a log version is an entry inside the project, not a file — so
    # the control is absent on those rows rather than present and inert.
    #
    # Matched on "the backup", not on "this version": the tooltip used to say the
    # latter, which named the wrong thing. Pinning keeps the whole snapshot the
    # version was read out of, every other row's state in it included, and a
    # writer who read "Pin this version" had been told it protected one scene's
    # wording. `versions-pin` / `versions-unpin`.
    PIN, UNPIN = "Pin the backup", "Unpin the backup"

    def pin_controls(needle):
        return [n for n in nodes()
                if n.get("role") == "Button" and needle in (n.get("label") or "")]

    pins, unpins = pin_controls(PIN), pin_controls(UNPIN)
    if not pins and not unpins:
        failures.append("STEP 6a: no pin control on any backup row")
    else:
        print(f"  {len(pins) + len(unpins)} pin control(s) \u2713")
        before = len(unpins)
        if pins and activate(pins[0]):
            time.sleep(0.8)
            settle()
            after = len(pin_controls(UNPIN))
            if after == before + 1:
                print("  pinning a version flips its control \u2713")
            else:
                failures.append(f"STEP 6a: pinning changed nothing ({before} -> {after})")

    # ── STEP 6a2: and the panel says why the other half has none ──────────────
    # The absence of a control teaches nothing on its own, while the tooltip
    # beside it promises automatic cleanup will never delete "this version". Said
    # in words, on exactly the lists that mix the two sources.
    blob = " ".join(texts(nodes()))
    log_rows = [t for t in texts(nodes()) if "From the project's own history" in t]
    note = "Only versions from a backup can be pinned" in blob
    if log_rows and not note:
        failures.append("STEP 6a2: the list holds an unpinnable version and does "
                        "not say why it has no pin")
    elif log_rows:
        print("  the panel explains the missing pin column \u2713")
    else:
        print("  !! every row here is backup-sourced, so the pin note does not "
              "apply — it was not checked")

    # ── STEP 6b: the marks stayed marks ───────────────────────────────────────
    # The renderer splices escaped prose between `{+`/`{-` sigils. If the escaping
    # or the block-start guard were wrong, those sigils would be sitting in the
    # text where a reader can see them.
    leaked = [t for t in texts(nodes()) if "{+" in t or "+}" in t or "{-" in t or "-}" in t]
    if leaked:
        failures.append(f"STEP 6: diff markup leaked into the text: {leaked[:2]}")
    else:
        print("STEP 6b OK: no diff markup leaked into the rendered text")

    # ── STEP 7: restore is offered, and asks before it overwrites ─────────────
    # The confirmation is the whole guard. A restore that went straight through
    # would be the single most destructive thing in the app.
    if not click("Restore this version", "the diff pane"):
        failures.append("STEP 7: no restore control")
    else:
        time.sleep(1.0)
        settle()
        blob = " ".join(texts(nodes()))
        if "Replace this text with the version" in blob:
            print("STEP 7 OK: restoring asks first \u2713")
            if "Ctrl+Z" in blob:
                print("  the dialog says it is undoable \u2713")
            else:
                failures.append("STEP 7: the confirmation does not say it can be undone")
            if os.environ.get("PROBE_RESTORE"):
                # ── STEP 8: the restore actually lands ────────────────────────
                # Opt-in, because it rewrites the project it is pointed at. Point
                # it at a copy. Nothing short of this proves the write reaches
                # disk: the guards, the confirmation and the engine all pass their
                # own tests while the whole chain silently does nothing.
                marker = os.environ.get("PROBE_MARKER", "")
                if not click("OK", "confirmation"):
                    failures.append("STEP 8: could not confirm")
                else:
                    print("  confirmed; waiting for the safety copy, the write and the save")
                    for _ in range(40):
                        time.sleep(2)
                        settle()
                        if "Restored the version" in " ".join(texts(nodes())):
                            break
                    said = " ".join(texts(nodes()))
                    if "Restored the version" in said:
                        print("STEP 8 OK: the restore reported success \u2713")
                    elif "safety backup" in said:
                        failures.append("STEP 8: refused because the safety copy did not run")
                    else:
                        failures.append("STEP 8: the restore neither succeeded nor said why not")
                    if marker:
                        import zipfile
                        z = zipfile.ZipFile(project)
                        names = [n for n in z.namelist() if n.endswith(".scene.djot")]
                        landed = any(marker in z.read(n).decode("utf-8", "replace")
                                     for n in names)
                        if landed:
                            print("STEP 8 OK: the past text is on disk \u2713")
                        else:
                            failures.append("STEP 8: the restored text never reached the file")
            elif not click("Cancel", "confirmation"):
                # Leave the project untouched by default: this probe must not
                # rewrite the manuscript it was pointed at.
                failures.append("STEP 7: could not cancel the confirmation")
        elif "backup" in blob.lower() and "running" in blob.lower():
            print("STEP 7 OK: refused with a named reason (a backup was running)")
        else:
            failures.append("STEP 7: restoring neither confirmed nor said why not")

# ── STEP 9: the date filter narrows what is drawn ─────────────────────────────
# **The bug this guards.** `list` built its rows with its own filter, which knew
# about "pinned only" and not about the date range, while the view-model's
# `visible_indices` applied both. A projection with two definitions has none: the
# list drew rows the selection had already excluded, so clicking row *n* resolved
# to a different version — the one the diff pane showed and Restore would have
# written back.
DATE_ROW = re.compile(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$")


def version_rows():
    return [(n.get("label") or "").strip() for n in nodes()
            if n.get("role") == "ListBoxOption"
            and DATE_ROW.match((n.get("label") or "").strip())]


import datetime as _dt  # noqa: E402  (used only by this step)

before = version_rows()
window_start = (_dt.date.today() - _dt.timedelta(days=29)).isoformat()
outside = [r for r in before if r[:10] < window_start]
if not outside:
    print("STEP 9 SKIPPED: every version here is inside the last 30 days, so the "
          "preset cannot narrow anything — this check did not run")
elif not click("Last 30 days", "filter row"):
    failures.append("STEP 9: the 'Last 30 days' preset is not reachable")
else:
    time.sleep(1.0)
    settle()
    after = version_rows()
    still_there = [r for r in after if r[:10] < window_start]
    if still_there:
        failures.append(
            f"STEP 9: {still_there} are older than the window and still drawn. "
            "The list is filtering differently from the selection, so a click "
            "resolves a version other than the one on screen.")
    else:
        print(f"STEP 9 OK: the list narrowed, {len(before)} -> {len(after)} row(s)")
    if not click("Clear the filters", "filter row"):
        failures.append("STEP 9: no way out of a filter that can empty the list")
    else:
        time.sleep(1.0)
        settle()
        if len(version_rows()) != len(before):
            failures.append("STEP 9: clearing the filter did not restore the list")

# ── STEP 10: a boundary states only what was proved ───────────────────────────
blob = " ".join(rendered_text())
if re.search(r"Didn't exist before", blob):
    failures.append(
        "STEP 10: the old wording is on screen. It named the moment the row was "
        "first *seen*, which claims it did not exist right up to that point — "
        "across a gap in which it may well have.")
said = re.findall(r"Didn't exist yet on (\d{4}-\d{2}-\d{2})", blob)
rows = version_rows()
if not said:
    print("STEP 10 SKIPPED: no row here had its creation observed, so there is "
          "no boundary to check — this check did not run")
elif not rows:
    print("STEP 10 SKIPPED: no dated rows to compare the boundary against")
elif said[0] >= min(r[:10] for r in rows):
    failures.append(
        f"STEP 10: the boundary names {said[0]}, which is not before the oldest "
        f"version on record ({min(r[:10] for r in rows)}). A moment the row was "
        "proved absent cannot be one the dock holds a version from.")
else:
    print(f"STEP 10 OK: the boundary ({said[0]}) precedes every version on record")

print()
if failures:
    print("FAILURES:")
    for f in failures:
        print(" -", f)
else:
    print("all checks passed")
print("\nthe app is left running for a screenshot")
if mcp.poll() is None:
    mcp.terminate()
sys.exit(1 if failures else 0)
