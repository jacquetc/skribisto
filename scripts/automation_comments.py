#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **comments** feature end to end.

Comments are anchored notes on prose: a range comment on a selection, a
paragraph comment on the block at the caret. They surface in two docks — a
project-wide one on the leading rail and a this-document one on the trailing
rail — and as an underline in the editor itself.

    scripts/automation_comments.py               # mocks build
    scripts/automation_comments.py PROJECT.skrib # a real project

Checks:
  1. Both comment docks are reachable from their rails and render their panel
     (filter chips + either threads or an honest empty state).
  2. The orphan filter chip is present **even at zero** — the always-visible
     contract. A filter that only appears once something is wrong is how a lone
     orphan goes unnoticed, which is the Google-Docs/Confluence failure this
     feature is designed against.
  3. The editor's context menu offers "Comment on this paragraph" on prose, and
     offers "Add comment" only once there is a selection to anchor to.
  4. `Ctrl+Alt+Shift+M` actually reaches `comments.add_paragraph` and a thread
     appears in the margin — the menu items existing says nothing about whether
     the chords are wired to them.
  5. That margin is **flush** against the writing column, not pinned to the far
     edge of the pane with a run of dead space between each card and its text.
  6. The Overview carries its Comments column.

The mocks build fabricates a comment set that deliberately covers an open
thread, a resolved one and an orphan, so 1 and 2 exercise every row shape
rather than only the happy one.

Reuses the launch + Launcher→Mock-Project scaffolding from
automation_overview_segment.py.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = os.environ.get(
    "SKRIBISTO_BIN",
    os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                 "target", "debug", "skribisto"))
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
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
app = subprocess.Popen([SKRIBISTO] + ([project] if project else []),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT)

sock = tok = None
end = time.time() + 25
while time.time() < end:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt)
    t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
    if s and t:
        sock, tok = s.group(1), t.group(1)
        break
    if app.poll() is not None:
        die("app exited early", app)
    time.sleep(0.2)
if not sock:
    die("no bridge socket", app)

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
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "overview-segment", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None:
        if mcp.poll() is None:
            mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP", app, mcp)
send("notifications/initialized", notif=True)
print(f"bridge up: {sock}")


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


def want(label, why, minx=None):
    """Assert a label is somewhere in the AccessKit tree."""
    if has_label(label, minx=minx):
        print(f"  {label!r} present \u2713")
        return True
    failures.append(f"{why}: {label!r} missing")
    return False


# ── 1. The create actions are on the editor's context menu ───────────────────
#
# FIRST, before any dock is visited: the comment docks share the leading rail
# with the binder, so opening one *replaces* the binder tree and every later
# binder-row lookup would fail. (Learned the hard way — this probe's first
# version checked the docks first and then reported the binder as missing.)
print("\n=== editor context menu ===")


# The layout-tree kinds of the two widgets that wrap a main prose column: with
# comments wired (every real document) and without (the widget tests, and any
# document whose Content has no id).
WRITING_COLUMN_KINDS = ("comments::pane::ColumnWithMargin",
                        "editor::CenterColumnFlowing")


def editor_point():
    """A point genuinely inside the writing column's **prose**.

    Taken from the writing column's own laid-out box in the layout tree, near its
    top-left, rather than from the panel geometry: a scene is only a few lines
    long, so the middle of the *pane* is blank space well below the last line —
    and a right-click there opens no menu at all. That is how this probe's second
    version "passed" while never reaching the editor. It is also why the point is
    the column's top rather than its centre: the column's box includes the empty
    run under short prose.
    """
    best = None
    for n in all_widgets():
        blob = json.dumps(n.get("kind") or n.get("type") or "")
        if not any(k in blob for k in WRITING_COLUMN_KINDS):
            continue
        b = n.get("bounds") or {}
        if b.get("width", 0) <= 0 or b.get("height", 0) <= 0:
            continue
        # The last one laid out is the visible tab's; earlier ones belong to tabs
        # left open behind it.
        best = b
    if best is None:
        return None
    return (best["x"] + best["width"] * 0.25, best["y"] + 12.0)


# A NON-trashed scene: the mock binder's "Scene at dawn" is in the Trash, and a
# trashed item opens behind a banner rather than as an ordinary writing surface.
scene = find("Scene 1", timeout=8) or find("Scene 2", timeout=8)
if scene is None:
    failures.append("context menu: no prose row in the binder")
else:
    b = scene.get("bounds") or {}
    if "x" in b:
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
        settle()
        pt = editor_point()
        if pt is None:
            failures.append("context menu: could not locate the writing column")
        else:
            # `action` + `button`, not `kind`: `inject_pointer` ignores unknown
            # fields, so the original `"kind": "right_click"` was serialised away
            # and injected an ordinary LEFT click. The menu never opened, and the
            # check below reported the editor as unreachable for the one reason
            # that had nothing to do with the editor.
            call("inject_pointer",
                 {"x": pt[0], "y": pt[1], "action": "click", "button": "secondary"})
            settle()
            # Prove a menu opened at all before judging its contents: the
            # "'Add comment' is absent" assertion below passes trivially if the
            # right-click missed the editor.
            if not has_label("Cut"):
                failures.append(
                    "context menu: no menu opened at the right-click point")
            else:
                want("Comment on this paragraph", "context menu")
                if has_label("Add comment"):
                    failures.append(
                        "context menu: 'Add comment' offered with no selection — "
                        "a zero-width range has nothing to anchor to")
                else:
                    print("  'Add comment' correctly withheld with no selection \u2713")
            shot("comments-context-menu")
            call("inject_key", {"key": "Escape"})
            settle()

# ── 2. The create SHORTCUT, and the margin it opens ─────────────────────────
#
# The context-menu check above proves the actions exist; it says nothing about
# whether the chords reach them. `Ctrl+Alt+Shift+M` is the paragraph one, which
# needs no selection — so a bare caret in the prose is enough to fire it.
print("\n=== the paragraph shortcut and the margin ===")


def turn_count():
    """How many conversation turns are rendered in the margin right now."""
    return sum(1 for n in all_widgets()
               if "comments::card::Turn" in json.dumps(n.get("kind") or n.get("type") or ""))


def margin_boxes():
    """(prose column, margin) bounds of every laid-out writing column with cards.

    Read through `ColumnWithMargin`'s **children**, not off its own box: the pane
    widget spans the whole width, so comparing the margin against *it* measures
    nothing (and reports a flush margin as 328 px of overlap). Its two children
    are the capped writing column and the margin, in that order.
    """
    by_id = {n["id"]: n for n in all_widgets() if "id" in n}
    out = []
    for n in by_id.values():
        kind = json.dumps(n.get("type") or n.get("kind") or "")
        if "comments::pane::ColumnWithMargin" not in kind:
            continue
        kids = [by_id[k] for k in (n.get("children") or []) if k in by_id]
        margin = next((k for k in kids
                       if "comments::margin::CommentMargin"
                       in json.dumps(k.get("type") or "")), None)
        column = next((k for k in kids if k is not margin), None)
        if margin is not None and column is not None:
            out.append((column.get("bounds") or {}, margin.get("bounds") or {}))
    return out


pt = editor_point()
if pt is None:
    failures.append("shortcut: could not locate the writing column")
else:
    before = turn_count()
    call("inject_pointer", {"x": pt[0], "y": pt[1], "action": "click"})
    settle()
    call("inject_key", {"key": "M", "ctrl": True, "alt": True, "shift": True})
    settle()
    time.sleep(0.8)
    after = turn_count()
    if after <= before:
        failures.append(
            "Ctrl+Alt+Shift+M: no comment appeared in the margin — the chord "
            "never reached `comments.add_paragraph`")
    else:
        print(f"  Ctrl+Alt+Shift+M opened a thread ({before} → {after} turns) ✓")

    # And the margin sits **flush** against the prose. A margin pinned to the far
    # edge of the pane strands every card at the end of a long empty leader, which
    # is what the placement rules in `comments::layout::place_pane` exist to stop.
    boxes = [(c, m) for c, m in margin_boxes() if m.get("width", 0) > 0]
    if not boxes:
        failures.append("margin: nothing claimed a column after the shortcut")
    else:
        col, mar = boxes[-1]
        gap = mar["x"] - (col["x"] + col["width"])
        if abs(gap) > 1.0:
            failures.append(
                f"margin: {gap:.0f} px of dead space between the prose and its cards")
        else:
            print("  the margin is flush against the writing column ✓")
    shot("comments-margin")

# ── 3. The Overview carries its Comments column ──────────────────────────
print("\n=== Overview \u2192 Comments column ===")
if click("Chapter Two", "(binder row)") and click("Overview", "(segment)", minx=PANE_X):
    settle()
    want("Total comments", "Overview", minx=PANE_X)
    shot("comments-overview-column")

# ── 4. Both docks render, and the orphan chip is always there ───────────────
#
# Last, because opening either one hides the binder. "Orphaned" is the assertion
# that carries weight: All/Open/Resolved are words that occur elsewhere in the
# shell, so matching them proves little on its own.
for dock_label, why in (("Comments", "leading (project-wide) dock"),
                        ("This document", "trailing (per-document) dock")):
    print(f"\n=== {why} ===")
    if not click(dock_label, "(rail tab)"):
        failures.append(f"{why}: no rail tab labelled {dock_label!r}")
        continue
    settle()
    want("Orphaned", why)
    shot(f"comments-dock-{dock_label.split()[0].lower()}")

# ── verdict ─────────────────────────────────────────────────
print()
if failures:
    print(f"FAIL \u2014 {len(failures)} problem(s):")
    for f in failures:
        print("  \u2022", f)
    die("comments probe failed", app, mcp)
print("PASS \u2014 the context-menu actions, the paragraph shortcut and its flush "
      "margin, the Overview column and both docks (with their always-on orphan "
      "chip) all check out")
for p in (app, mcp):
    try:
        p.terminate()
    except Exception:
        pass
