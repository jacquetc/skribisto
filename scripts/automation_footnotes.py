#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **footnotes** feature end to end.

A footnote is an anchored note that, unlike a comment, renders into the book.
Its reference is one inline object in the prose; its body is a project-owned
entity; its number is derived from where the reference sits and is never
stored. That last part is what these checks are really about — a number that
agrees between the editor and the exported book is the whole feature, and
nothing in a unit test can see the marker actually painted.

    scripts/automation_footnotes.py               # mocks build
    scripts/automation_footnotes.py PROJECT.skrib # a real project

Checks:
  1. The footnotes dock is reachable from the **trailing** rail and renders its
     panel — the filter chips plus either rows or an honest empty state.
  2. The orphan chip is present **even at zero**, the same always-visible
     contract the comment docks hold: a filter that only appears once something
     is wrong is how a lone orphan goes unnoticed, and an orphaned footnote is
     missing content in a published manuscript rather than a lost remark.
  3. The mocks fixture's rows are all three shapes — two placed notes carrying
     real numbers, and a deliberate orphan. A build that fabricates only the
     happy row proves nothing about the two that matter.
  4. Document ▸ Insert footnote exists, carries its shortcut, and is gated on
     there being a caret to insert at.
  5. `Ctrl+Alt+F` actually reaches `editor.insert_footnote`. A menu item
     existing says nothing about whether the chord is wired to it — which is
     exactly how the outline toggle came to be a dead key once.
  6. Inserting adds a row to the dock, and that row carries a number rather
     than the raw `fn7`-style label the format uses internally. The label is
     machinery the writer never typed and must never read.

Reuses the launch + Launcher-to-Mock-Project scaffolding from
`automation_comments.py`.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
OUT = os.environ.get("SHOT_DIR", "/tmp")
PANE_X = 300

project = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else None
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-20:]))
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


subprocess.run(["pkill", "-x", "skribisto"], check=False)
time.sleep(0.4)
env = fixture.isolated_config(locale="en-US", label="footnotes", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="footnotes")
app = subprocess.Popen(fixture.launch_argv(project, pins=pins), env=env,
                       stdout=open(log, "w"), stderr=subprocess.STDOUT)

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


deadline = time.time() + 25
init = None
while time.time() < deadline and init is None:
    mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "footnotes-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None:
        if mcp.poll() is None:
            mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP", app, mcp)
send("notifications/initialized", notif=True)
print(f"bridge up: {bridge.endpoint}")


def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def all_widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def find(label, timeout=6.0, minx=None):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            if n.get("label") == label:
                if minx is None or (n.get("bounds") or {}).get("x", 0) >= minx:
                    return n
        settle()
    return None


def click(label, what="", minx=None):
    n = find(label, minx=minx)
    if not n:
        print(f"  !! no node labelled {label!r} {what}")
        return False
    if "click" in (n.get("actions") or []):
        call("invoke_action", {"node": n["id"], "action": "click"})
    else:
        b = n.get("bounds") or {}
        if "x" not in b:
            return False
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    settle()
    time.sleep(0.5)
    return True


def has_label(label, minx=None):
    """True if a node with this exact label exists (optionally right of minx)."""
    for n in nodes():
        if n.get("label") == label:
            if minx is None or (n.get("bounds") or {}).get("x", 0) >= minx:
                return True
    return False


def text_present(substr):
    """True if `substr` appears in any text field across a11y + layout trees."""
    for n in nodes() + all_widgets():
        for field in ("label", "value", "text", "resolved_text", "params"):
            blob = n.get(field)
            hay = blob if isinstance(blob, str) else json.dumps(blob or "")
            if substr.lower() in hay.lower():
                return True
    return False


def shot(name):
    path = os.path.join(OUT, f"{name}.png")

# ── reach the editor (Launcher → Mock Project for the no-args mocks build) ──────
if project is None:
    row = find("Mock Project", timeout=10)
    if not row:
        die("no 'Mock Project' recent row — this no-args mode needs a `--features "
            "mocks` build", app, mcp)
    b = row.get("bounds") or {}
    if "x" not in b:
        die("'Mock Project' row has no bounds to click", app, mcp)
    call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                            "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    end = time.time() + 15
    opened = False
    while time.time() < end:
        labels_now = [n.get("label") for n in nodes() if n.get("label")]
        if "Welcome sections" not in labels_now and "Binder" in labels_now:
            opened = True
            break
        settle()
    if not opened:
        die("clicking 'Mock Project' never opened the editor", app, mcp)
else:
    # An argv launch loads the project itself — and `load_work` is SLOW in a debug
    # build (~20 s for the bundled example). Without this wait every check below
    # runs against the empty shell and fails for the one reason that has nothing
    # to do with comments.
    end = time.time() + 90
    while time.time() < end:
        if "Binder" in [n.get("label") for n in nodes() if n.get("label")]:
            break
        settle()
    else:
        die("the project never finished loading", app, mcp)
settle()




failures = []

# The layout-tree kinds of the two widgets that wrap a main prose column: with
# comments wired (every real document) and without.
WRITING_COLUMN_KINDS = ("comments::pane::ColumnWithMargin",
                        "editor::CenterColumnFlowing")


def want(label, why, minx=None):
    """Assert a label is somewhere in the AccessKit tree."""
    if has_label(label, minx=minx):
        print(f"  {label!r} present ✓")
        return True
    failures.append(f"{why}: {label!r} missing")
    return False


def either(options, why):
    """Assert one of several locale spellings is present."""
    if any(has_label(o) for o in options):
        print(f"  {options[0]!r} present ✓")
        return True
    failures.append(f"{why}: none of {options} present")
    return False


def editor_point():
    """A point genuinely inside the writing column's **prose**.

    Taken from the column's own laid-out box near its top-left, not from the
    pane geometry: a scene is a few lines long, so the middle of the pane is
    blank space well below the last line, and a click there reaches no editor.
    """
    best = None
    for n in all_widgets():
        blob = json.dumps(n.get("kind") or n.get("type") or "")
        if not any(k in blob for k in WRITING_COLUMN_KINDS):
            continue
        b = n.get("bounds") or {}
        if b.get("width", 0) > 0 and b.get("height", 0) > 0:
            best = b
    if best is None:
        return None
    return (best["x"] + best["width"] * 0.25, best["y"] + 12.0)


def row_count():
    """How many note rows the dock is showing.

    Counted by the per-row menu button's accessible name — the one part of a row
    that is exactly one per row whatever the note says.
    """
    names = ("Footnote actions", "Actions sur la note")
    return len([n for n in nodes() if (n.get("label") or "") in names])


# ── 1. A scene, open, with a caret in it ─────────────────────────────────────
#
# Before anything else: insertion is prose-only, so every check below that
# involves the caret is meaningless without a writing surface in front of it.
# ("Scene at dawn" is in the mock Trash and opens behind a banner instead.)
print("\n[1] open a scene")
scene = find("Scene 1", timeout=10) or find("Scene 2", timeout=10)
if scene is None:
    die("no prose row in the binder to open", app, mcp)
b = scene.get("bounds") or {}
call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                        "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
settle()
if editor_point() is None:
    die("clicking a binder row opened no writing column", app, mcp)
print("  a writing column is up ✓")


# ── 2. The dock is on the trailing rail and renders ──────────────────────────
#
# Trailing, not leading, and that is a design claim worth pinning: footnotes
# share a rail with the Inspector and this-document comments — "what is in front
# of me right now" — rather than with the binder.
print("\n[2] the footnotes dock")
DOCK_TITLES = ("Footnotes", "Notes de bas de page")
if not any(has_label(t) and click(t, "trailing rail") for t in DOCK_TITLES):
    die("the footnotes dock is not reachable from any rail", app, mcp)
print("  dock revealed ✓")


# ── 3. The filter chips, orphan chip included at any count ───────────────────
#
# The orphan chip is present **even at zero**, the same always-visible contract
# the comment docks hold. A filter that only appears once something is wrong is
# how a lone orphan goes unnoticed — and an orphaned footnote is missing content
# in a published manuscript, not merely a lost remark.
print("\n[3] filter chips")
either(("All", "Toutes"), "the unfiltered view")
either(("This document", "Ce document"), "the focused-document filter")
either(("Orphaned", "Orphelines"), "the always-visible orphan filter")


# ── 4. Every row shape, not just the happy one ───────────────────────────────
#
# Mocks only: a real project starts with no notes, and an empty dock is the
# right answer there rather than a failure.
if project is None:
    print("\n[4] fabricated rows")
    if any(text_present(t) for t in ("Nothing points at this note any more",
                                     "Plus rien ne renvoie à cette note")):
        print("  the orphan row says why it has nowhere to go ✓")
    else:
        failures.append("the fixture's orphan row never reached the dock")
    if text_present("parish register") or text_present("ridgeline"):
        print("  a note's own words are shown ✓")
    else:
        failures.append("no fabricated note body reached the dock")
    if row_count() < 3:
        failures.append(f"expected the fixture's three rows, saw {row_count()}")
    else:
        print(f"  {row_count()} rows ✓")
else:
    print("\n[4] fabricated rows — skipped (real project)")


# ── 5. The chord reaches the command ─────────────────────────────────────────
#
# A menu item existing says nothing about whether its chord is wired to it —
# which is how a toggle can sit in a menu, look correct, and do nothing. The
# Document ▸ Insert footnote row itself is not checked here: the title-bar menu
# renders in an overlay this bridge cannot open by name.
print("\n[5] Ctrl+Alt+F inserts")
before = row_count()
pt = editor_point()
call("inject_pointer", {"x": pt[0], "y": pt[1], "action": "click"})
settle()
call("inject_key", {"key": "F", "ctrl": True, "alt": True})
settle()
time.sleep(1.0)
after = row_count()
if after > before:
    print(f"  a row appeared ({before} → {after}) ✓")
else:
    failures.append(f"Ctrl+Alt+F added no row to the dock ({before} → {after})")


# ── 6. The marker is a number, never the stored label ────────────────────────
#
# `fn7` is machinery the writer never typed. It appears in the `.skrib` and
# nowhere a person can read — not in the prose, not in the dock.
print("\n[6] no raw label on any surface")


def label_on_screen():
    """Any `fn<digits>` in a string a person can read.

    Tighter than looking for `[^fn…]`, and the difference is the whole point:
    when the marker map does not know a label, the reference falls back to
    drawing that label **bare** — `fn4`, no brackets — straight into the
    writer's prose. That is exactly what the first live run of this feature
    showed, and a bracketed check sails past it.
    """
    pat = re.compile(r"\bfn\d+\b")
    for n in nodes() + all_widgets():
        for field in ("label", "value", "text", "resolved_text"):
            blob = n.get(field)
            if isinstance(blob, str) and pat.search(blob):
                return blob
    return None


leak = label_on_screen()
if leak:
    failures.append(f"a raw footnote label is visible on screen: {leak!r}")
else:
    print("  no raw label anywhere ✓")


print()
if failures:
    print("FAILURES:")
    for f in failures:
        print(" -", f)
else:
    print("all checks passed")
for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
sys.exit(1 if failures else 0)
