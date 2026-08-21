#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **Timeline** band end to end.

The band is the whole project's past: a bar chart of every recorded moment, a
slider addressing the same bars, a date filter, and beside them a list of what
differs between the selected moment and now. None of that is reachable from a
unit test — the moments come from real archives on disk, the bars are painted by
`teksilo_charts`, and a bar's hover tooltip and screen-reader description are
built by the chart, not by this app.

    scripts/automation_timeline.py PROJECT.skrib

It needs recorded versions of that project to exist — backups beside the file or
in the backup root, or an in-project history log. With no past there is nothing
to draw and the honest empty state is all this would prove; the probe says so
rather than passing quietly.

Checks:
  1. The Timeline dock is reachable from the **bottom** rail and draws a band
     with real width, not a collapsed strip.
  2. The coverage sentence names how many moments there are and how far back
     they reach — the one line most writers will ever read here.
  3. **Every bar names what it measures.** The chart's series was anonymous, so
     `BarChart` built its hover tooltip as "{series}: {category} = {value}" and
     its screen-reader description as "{series}, {category}: {value}" with an
     empty first field: a writer hovering March saw a leading colon, and a
     screen reader announced a leading comma. The a11y description is the one
     place a probe can read either string back.
  4. The change list renders against the selected moment and names the date it
     is comparing with.
  5. **An edited row opens on a comparison, and can be switched out of it.**
     "Edited since" is a label; the question behind it is *edited how*, so
     opening such a row sets the recorded text against what the row says now,
     struck through and underlined. But someone who has found the scene they cut
     in March wants the paragraph as they wrote it, so a Changes/Text control
     gives back the recorded prose clean enough to select and copy.
  6. **A prose-only record says so.** Against an in-project history-log moment
     the list can only ever report "edited", because the log holds text and
     nothing else. Unsaid, a writer reads that silence as a fact about their
     book. This checks the sentence appears for a log moment and does not appear
     for a backup.

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

failures = []
skipped = []


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-25:]))
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


subprocess.run(["pkill", "-x", "skribisto"], check=False)
time.sleep(0.4)
env = dict(os.environ)
xdg = os.environ.get("PROBE_XDG")
if xdg:
    os.makedirs(os.path.join(xdg, "config"), exist_ok=True)
    os.makedirs(os.path.join(xdg, "data"), exist_ok=True)
    env["XDG_CONFIG_HOME"] = os.path.join(xdg, "config")
    env["XDG_DATA_HOME"] = os.path.join(xdg, "data")
    # A layout persisted by an earlier run can have the bottom side dragged to
    # zero height, and a dock with no height is indistinguishable from one that
    # failed to open.
    try:
        os.remove(os.path.join(xdg, "config", "skribisto", "workspace.toml"))
    except FileNotFoundError:
        pass
    print(f"isolated XDG root: {xdg}")
app = subprocess.Popen([SKRIBISTO] + ([project] if project else []),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT, env=env)

sock = tok = None
end = time.time() + 25
while time.time() < end:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt)
    t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
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


deadline = time.time() + 40
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "timeline-probe", "version": "1"}})
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
    time.sleep(0.4)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def rendered_text():
    """Every string the widget tree resolved, from both trees.

    The a11y snapshot alone is not enough: its `SemanticNode` DTO carries
    `label` and `value` but not `description`, and a `StandardListItem`'s
    subtitle goes to `description`. So the layout tree has to be read too.
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


def check(ok, why):
    print(("  ok   " if ok else "  FAIL ") + why)
    if not ok:
        failures.append(why)
    return ok


def skip(why):
    print("  skip  " + why)
    skipped.append(why)


# ── 1. the band ────────────────────────────────────────────────────────────


def menu_items():
    return [n for n in nodes() if n.get("role") == "MenuItem"]


def menu_label(n):
    # Mnemonics are an `&` in the ftl value, so the rendered label carries one.
    return (n.get("label") or "").replace("&", "")


def open_timeline():
    """View ▸ Timeline — `view.rs` says it is the only door.

    Not a rail tab: the bottom side starts closed and mounts no icon until
    something reveals the dock, so there is nothing to click until this has run.
    And not a coordinate click on the menu button either — the title-bar menu is
    an overlay whose bounds are reported in its own space, so a pointer aimed at
    them lands outside the window and dismisses it. Focus plus Enter opens the
    bar; `invoke_action` addresses the rows by id, which needs no coordinates.
    """
    for _ in range(4):
        if any(menu_label(i) == "View" for i in menu_items()):
            break
        hamburger = [n for n in nodes() if n.get("label") == "Menu"]
        if not hamburger:
            return False
        call("focus_node", {"node": hamburger[0]["id"]})
        time.sleep(0.3)
        call("inject_key", {"key": "Enter"})
        time.sleep(0.9)
        settle()
    view = [i for i in menu_items() if menu_label(i) == "View"]
    if not view:
        return False
    call("invoke_action", {"action": "click", "node": view[0]["id"]})
    time.sleep(0.9)
    settle()
    row = [i for i in menu_items() if menu_label(i) in ("Timeline", "Chronologie")]
    if not row:
        return False
    call("invoke_action", {"action": "click", "node": row[0]["id"]})
    # The scan opens every archive the project has, which outruns `settle`.
    time.sleep(2.5)
    settle()
    time.sleep(2.5)
    return True


settle()
print("1. the Timeline dock draws a band")
if not open_timeline():
    die("could not reach View ▸ Timeline", app, mcp)
band = [n for n in widgets() if "TimelinePanel" in (n.get("type") or "")]
if not check(bool(band), "the band is on screen"):
    die("no TimelinePanel in the widget tree — the dock never opened", app, mcp)
b = band[0].get("bounds") or {}
w, h = b.get("width", 0), b.get("height", 0)
check(w > 200 and h > 40, f"the band has real size ({w:.0f}x{h:.0f} dp)")

text = rendered_text()
blob = "\n".join(text)

# ── 2. coverage ────────────────────────────────────────────────────────────

print("2. the coverage sentence")
if re.search(r"No version of this project has been recorded yet", blob):
    skip("this project has no recorded past — nothing below can be exercised")
    print("\n".join(f"  skipped: {s}" for s in skipped))
    sys.exit(0)
check(bool(re.search(r"versions? recorded, going back to \d{4}-\d{2}-\d{2}", blob)),
      "it says how many moments there are and how far back they reach")

# ── 3. every bar names what it measures ────────────────────────────────────


def bars():
    """The chart's marks, oldest first.

    Found by **role**, never by matching a date in the label:
    `timeline_axis::label_for` formats a bucketed bar four different ways
    (`2026-03`, `2026-03-14`, `03-14 09:00`) and an unbucketed one a fifth, so
    every regex over the label silently finds nothing on some histories — which
    is the failure mode a probe must not have, because it reads as a pass.
    `teksilo_charts::hit::emit_mark_node` gives each mark
    `accesskit::Role::GraphicsObject`, and nothing else in this window has it.
    """
    return [n for n in nodes() if n.get("role") == "GraphicsObject"]


print("3. a bar's screen-reader description")
# `teksilo_charts` names each mark "{series}, {category}: {value}". An anonymous
# series makes that start with a bare comma, and makes the hover tooltip start
# with a bare colon.
marks = bars()
if not marks:
    skip("no chart marks in the a11y tree — the chart drew no bars")
else:
    labels = [n.get("label") or "" for n in marks]
    lead = [m for m in labels if m.lstrip().startswith(",")]
    check(not lead, f"no bar announces itself with a bare comma ({len(marks)} bars)")
    named = [m for m in labels if m.split(",")[0].strip()]
    check(bool(named), f"a bar names its series, e.g. {named[0] if named else '—'!r}")

# ── 4. the change list ─────────────────────────────────────────────────────

print("4. the change list")
listed = re.search(r"items? differ between (.+?) and your project now", blob)
nothing = re.search(r"(Nothing|No text) has changed since then", blob)
check(bool(listed or nothing), "the list says what it is comparing, or that nothing is")
if listed:
    check(bool(re.search(r"\d{4}-\d{2}-\d{2}", listed.group(1))),
          f"and names the moment: {listed.group(1)!r}")

# ── 5. opening an edited row compares it with now ──────────────────────────

print("5. an edited row opens on a comparison")


def click_at(node):
    b = node.get("bounds") or {}
    if not b.get("width"):
        return False
    x, y = b["x"] + b["width"] / 2, b["y"] + b["height"] / 2
    call("inject_pointer", {"action": "move", "x": x, "y": y})
    time.sleep(0.15)
    call("inject_pointer", {"action": "click", "x": x, "y": y, "button": "left"})
    time.sleep(0.5)
    return True


# Select the oldest bar: the newest usually differs from nothing, and a list with
# no rows proves nothing about opening one.
marks = bars()
if marks:
    click_at(marks[0])
    settle()
    time.sleep(1.0)

rows = [n for n in nodes() if n.get("role") == "ListBoxOption" and (n.get("label") or "").strip()]
if not rows:
    skip("nothing differs between the oldest moment and now — no row to open")
else:
    # A row is opened by *selection* then Enter. Focus alone is not selection:
    # `ListView::on_activate` is handed the selected index, so a probe that only
    # focused each row in turn reopened the first one every time.
    found = None
    for r in rows[:12]:
        if not click_at(r):
            continue
        call("inject_key", {"key": "Enter"})
        time.sleep(1.4)
        settle()
        here = rendered_text()
        if any("Struck through" in t for t in here):
            found = r.get("label")
            break
        call("inject_key", {"key": "Escape"})
        time.sleep(0.6)
        settle()
    if found:
        check(True, f"{found!r} opens set against what it says now, with a legend")
        # …and the writer can get out of the comparison to the prose itself.
        # A `SegmentedControl` surfaces as an AccessKit RadioGroup whose children
        # are RadioButtons, and an AT click does not move the selection — the
        # segment has to be pressed at its own bounds.
        segs = [n for n in nodes() if n.get("role") == "RadioButton"]
        if len(segs) < 2:
            check(False, "a comparison offers no way back to the plain text")
        else:
            click_at(segs[1])
            settle()
            after = rendered_text()
            check(
                not any("Struck through" in t for t in after),
                "switching to the plain text drops the comparison's legend",
            )
            check(
                any(t.startswith("As it was on") and "set against" not in t for t in after),
                "and the stamp stops claiming a comparison it is no longer showing",
            )
    else:
        skip("no edited row among the first twelve — nothing to compare")
    call("inject_key", {"key": "Escape"})
    time.sleep(0.5)
    settle()

# ── 6. a prose-only record says so ─────────────────────────────────────────

print("6. a prose-only record says what it cannot show")
CAVEAT = "keeps text and nothing else"
seen = {}
for node in reversed(bars()[-8:]):
    if not click_at(node):
        continue
    settle()
    has = CAVEAT in "\n".join(rendered_text())
    seen.setdefault(has, node.get("label"))
    if len(seen) == 2:
        break
if True in seen:
    check(True, f"a prose-only moment says so: {seen[True]!r}")
else:
    skip("no history-log moment was recorded — the project has only backups")
if False in seen:
    check(True, f"and a backup does not: {seen[False]!r}")
else:
    skip("no backup moment was selectable — the caveat could not be shown to lift")

# ── verdict ────────────────────────────────────────────────────────────────

print()
for s in skipped:
    print(f"skipped: {s}")
# The app is left running on purpose (see the module docstring), but the MCP
# client is ours and nothing else will reap it.
if mcp and mcp.poll() is None:
    mcp.terminate()
if failures:
    print(f"\n{len(failures)} check(s) failed:")
    for f in failures:
        print(f"  - {f}")
    sys.exit(1)
print("all checks passed (app left running)")
