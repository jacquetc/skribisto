#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The **margin lane**, on screen, in a real window.

Everything else about this feature is headless, and headless has been generous
with it: eight defects so far were invisible to inspection and silent at runtime,
five of them presenting as a strip of exactly the right width with nothing in it.
Widths and node counts are the two things a `WidgetTree` test can prove and a
person cannot check; what a person can check is whether the thing is *there*, and
that is what this is for.

It opens a scene, finds the lane and asserts the two widths the settings promise
— twelve pixels for the marks, plus the texture column and its divider when that
is on — and screenshots each state so the result can be looked at rather than
believed.

**What it deliberately does not check: the marks themselves.** Where a mark lands
is covered headlessly, against real prose laid out in a real widget tree, and far
more precisely than a screenshot could. What no headless test can answer is
whether the strip is on screen and the right size at all, which is exactly the
half that kept going wrong.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/Starforgers.skrib")

# The lane's own accessible name, from `margin-lane-name` in en-US.
LANE_LABEL = "Margin marks"
TARGET = "Prologue"
# `DEFAULT_LANE_WIDTH`, and the texture column plus its one-pixel divider.
MARK_WIDTH = 12.0
TEXTURE_WIDTH = 28.0
DIVIDER = 1.0

fixture.assert_no_running_instance(SKRIBISTO)

sandbox = tempfile.mkdtemp(prefix="lane-probe-")
project = os.path.join(sandbox, "Starforgers.skrib")
shutil.copyfile(EXAMPLE, project)
env = dict(os.environ)
env.update(
    fixture.isolated_config(
        locale="en-US",
        label="lane",
        show_welcome=False,
        # Pinned rather than assumed: the defaults say the lane is on, and a probe
        # that trusted them would go green on a build where they had changed.
        pins={
            "editor.margin_lane.enabled": True,
            "editor.margin_lane.surface.editor": True,
            "editor.margin_lane.texture": False,
        },
    )
)

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
app = subprocess.Popen([SKRIBISTO, project], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
mcp = None


def die(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    try:
        print("\n".join(open(log).read().splitlines()[-30:]))
    except OSError:
        pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


sock = tok = None
deadline = time.time() + 25
while time.time() < deadline:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        die("the app exited before printing the bridge socket")
    time.sleep(0.2)
if not sock:
    die("no bridge socket within 25s")

_id = [0]


def send(method, params=None, notif=False):
    msg = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        msg["params"] = params
    if not notif:
        _id[0] += 1
        msg["id"] = _id[0]
    mcp.stdin.write(json.dumps(msg) + "\n")
    mcp.stdin.flush()


def recv(timeout=20, fatal=True):
    end = time.time() + timeout
    while time.time() < end:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, end - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        if line.strip():
            return json.loads(line.strip())
    if fatal:
        die("no MCP response within timeout")
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = recv().get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(c.get("text", "") for c in result.get("content", [])
                       if c.get("type") == "text")
        payload = json.loads(text) if text.strip().startswith("{") else {}
    return result, payload


deadline = time.time() + 25
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "margin-lane-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect the automation bridge")
send("notifications/initialized", notif=True)
print("connected")


def nodes():
    _, payload = call("snapshot_tree")
    return payload.get("nodes", [])


def shot(name):
    path = os.path.join(sandbox, name)
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot -> {path}")
            return path
    return None


def lane(ns):
    for n in ns:
        if (n.get("label") or "").strip() == LANE_LABEL:
            return n
    return None


def width_of(node):
    b = node.get("bounds") or {}
    return b.get("width", 0.0) if isinstance(b, dict) else b[2]


# ── open a scene ─────────────────────────────────────────────────────────────
if not fixture.wait_for_load(nodes, [TARGET], timeout=40):
    # The documented trap, and it costs half an hour every time: both feature sets
    # build to `target/debug/skribisto`, so any `cargo …  --features mocks` leaves the
    # **mocks** binary there and every probe afterwards drives fabricated data.
    if any("Mock" in (n.get("label") or "") for n in nodes()):
        die("this is the MOCKS binary — it opens a fabricated project, not the fixture.\n"
            "  Rebuild without the feature:  cargo build -p teksilo_ui --bin skribisto")
    shot("load-failure.png")
    die(f"the fixture never showed '{TARGET}'")
ns = nodes()
target = next((n for n in ns if (n.get("label") or "").strip() == TARGET), None)
if target is None:
    die(f"'{TARGET}' vanished between the wait and the snapshot")

if target.get("actions"):
    call("invoke_action", {"node": target["id"]})
time.sleep(1.5)
ns = nodes()
if lane(ns) is None:
    # The AT action may select without opening; fall back to a click, as
    # `automation_explore` does.
    b = target.get("bounds") or {}
    if b:
        call("inject_pointer", {"x": b.get("x", 0) + b.get("width", 0) / 2,
                                "y": b.get("y", 0) + b.get("height", 0) / 2,
                                "kind": "click"})
        time.sleep(1.5)
        ns = nodes()

failures = []


def check(ok, msg):
    print(("  PASS  " if ok else "  FAIL  ") + msg)
    if not ok:
        failures.append(msg)


# ── 1. the strip is on the page, at its declared width ───────────────────────
print("\n1. the lane, marks only")
strip = lane(ns)
check(strip is not None, f"a node labelled '{LANE_LABEL}' is in the accessibility tree")
if strip is None:
    shot("no-lane.png")
    die("no lane on the page — nothing further can be checked")
w = width_of(strip)
check(abs(w - MARK_WIDTH) < 1.5, f"it is {w:.1f} px wide (expected {MARK_WIDTH})")
h = (strip.get("bounds") or {}).get("height", 0.0)
check(h > 100.0, f"and {h:.1f} px tall, so it spans the page")
shot("lane-marks.png")

# ── 2. the texture column takes real width ───────────────────────────────────
print("\n2. the texture column")
print("  re-launching with editor.margin_lane.texture = true")
for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
        p.wait(timeout=10)
time.sleep(1.0)

env2 = dict(os.environ)
env2.update(
    fixture.isolated_config(
        locale="en-US", label="lane-tex", show_welcome=False,
        pins={
            "editor.margin_lane.enabled": True,
            "editor.margin_lane.surface.editor": True,
            "editor.margin_lane.texture": True,
        },
    )
)
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, project], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env2)
sock = tok = None
deadline = time.time() + 25
while time.time() < deadline:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        die("the app exited before printing the bridge socket (texture run)")
    time.sleep(0.2)
if not sock:
    die("no bridge socket within 25s (texture run)")

init = None
deadline = time.time() + 25
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "margin-lane-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not reconnect for the texture run")
send("notifications/initialized", notif=True)

if not fixture.wait_for_load(nodes, [TARGET], timeout=40):
    die("the fixture never reloaded")
ns = nodes()
target = next((n for n in ns if (n.get("label") or "").strip() == TARGET), None)
if target and target.get("actions"):
    call("invoke_action", {"node": target["id"]})
time.sleep(1.5)
ns = nodes()
if lane(ns) is None:
    b = (target or {}).get("bounds") or {}
    if b:
        call("inject_pointer", {"x": b.get("x", 0) + b.get("width", 0) / 2,
                                "y": b.get("y", 0) + b.get("height", 0) / 2,
                                "kind": "click"})
        time.sleep(1.5)
        ns = nodes()

strip = lane(ns)
check(strip is not None, "the lane is still there with the texture on")
if strip is not None:
    w = width_of(strip)
    expected = MARK_WIDTH + TEXTURE_WIDTH + DIVIDER
    check(abs(w - expected) < 1.5,
          f"it is {w:.1f} px wide (expected {expected}: marks + texture + divider)")
    shot("lane-texture.png")

# ── 3. a search puts marks on the strip ──────────────────────────────────────
#
# The one check that exercises the whole chain in a live window: the arbiter hears
# the query, the provider re-derives the hits against the document, `locate` turns
# each offset into a fraction through the editor's own laid-out geometry, and the
# widget resolves it to a node assistive technology can reach. Everything before
# this ran against a `WidgetTree` with no window behind it.
print("\n3. marks for a search")


def lane_marks(ns, strip):
    """The lane's mark nodes.

    Found by **where they are**, not by a parent link: they are synthetic nodes the
    lane pushes into the tree itself, and the snapshot does not report a parent for
    them. Their bounds and their role are the honest identity anyway: a mark is a
    `GraphicsObject` sitting inside the strip, which the page furniture beside it
    (a Split editor button whose centre happens to fall in the same column) is not.
    """
    b = (strip or {}).get("bounds") or {}
    if not b:
        return []
    x0, x1 = b.get("x", 0.0), b.get("x", 0.0) + b.get("width", 0.0)
    inside = []
    for n in ns:
        if n.get("id") == strip.get("id"):
            continue
        nb = n.get("bounds") or {}
        if not nb:
            continue
        cx = nb.get("x", 0.0) + nb.get("width", 0.0) / 2
        if (n.get("role") == "GraphicsObject"
                and x0 - 1 <= cx <= x1 + 1
                and nb.get("height", 0.0) <= b.get("height", 0.0)):
            inside.append(n)
    return inside


before = len(lane_marks(ns, strip)) if strip else 0
print(f"  marks before searching: {before}")

# Into the prose first: Ctrl+F acts on the editor the caret is in.
editors = [n for n in ns
           if n.get("role") == "MultilineTextInput"
           and "set_value" in (n.get("actions") or [])]
if editors:
    biggest = max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))
    call("focus_node", {"node": biggest["id"]})
    time.sleep(0.4)
    ns = nodes()
def text_inputs(ns):
    return [n for n in ns if n.get("role") == "TextInput"]


fields_before = len(text_inputs(ns))
call("inject_key", {"key": "f", "ctrl": True})
time.sleep(0.4)
# Pump frames: the banner's entrance is animated, and an idle app leaves it queued
# but never scheduled — the same reason `automation_find_banner_probe` does this.
for i in range(6):
    call("inject_pointer", {"x": 700.0 + i, "y": 400.0, "action": "move"})
    time.sleep(0.12)
time.sleep(0.6)
ns = nodes()
check(len(text_inputs(ns)) > fields_before, "Ctrl+F mounted the find banner")

# A word the Prologue repeats, typed into the banner's own field rather than into
# the prose — typing into the manuscript would edit it, and would not search.
banner = max(text_inputs(ns), key=lambda n: (n.get("bounds") or {}).get("width", 0))
call("focus_node", {"node": banner["id"]})
time.sleep(0.3)
# Addressed to the node: `type_text` with no target goes wherever focus happens to
# be, which for a freshly mounted banner is not reliably its own field.
call("type_text", {"node": banner["id"], "text": "the"})
time.sleep(0.5)
for i in range(6):
    call("inject_pointer", {"x": 700.0 + i, "y": 400.0, "action": "move"})
    time.sleep(0.12)
time.sleep(0.8)
ns = nodes()
strip = lane(ns)
after = len(lane_marks(ns, strip)) if strip else 0
print(f"  marks after searching:  {after}")
check(strip is not None, "the lane survived the find banner opening")
check(after > before, f"searching put marks on the strip ({before} -> {after})")
if after:
    print("  a sample of what the strip now says:")
    for n in lane_marks(ns, strip)[:3]:
        print(f"    {n.get('label')!r}")
shot("lane-search.png")

# ── 4. a Full Book stream: the map must hold still ───────────────────────────
#
# The bug this section exists for, reported from a window: revealing a row deep in a
# Full Book made that row's part of the lane expand, and its bars go from merged to
# one-per-paragraph. A map that rescales as you read it is not a map.
#
# The cause was not in the lane. Until an editor has been through a frame on screen
# `content_height()` is 0, so every row below the fold claimed the same `min_lines`
# floor whatever it held — and the lane, which draws the page's own extent, drew that
# settling. `RichTextEditor::estimate_height_before_layout` is what fixes it; this is
# what says so.
print("\n4. a Full Book holds its shape when a row is revealed")

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
        p.wait(timeout=10)
time.sleep(1.0)

env3 = dict(os.environ)
env3.update(
    fixture.isolated_config(
        locale="en-US", label="lane-book", show_welcome=False,
        pins={
            "editor.margin_lane.enabled": True,
            "editor.margin_lane.surface.stream": True,
            "editor.margin_lane.texture": True,
        },
    )
)
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, project], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env3)
sock = tok = None
deadline = time.time() + 25
while time.time() < deadline:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        die("the app exited before printing the bridge socket (book run)")
    time.sleep(0.2)
if not sock:
    die("no bridge socket within 25s (book run)")
init = None
deadline = time.time() + 25
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "margin-lane-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not reconnect for the book run")
send("notifications/initialized", notif=True)

if not fixture.wait_for_load(nodes, ["Chapter 1"], timeout=45):
    die("the fixture never reloaded (book run)")
ns = nodes()
book = next((n for n in ns
             if n.get("role") == "TreeItem" and (n.get("label") or "").strip() == "Starforgers"), None)
check(book is not None, "the Book is in the binder")
if book is None:
    die("no Book to open")
# `click`, explicitly: the default action on a TreeItem is `collapse`, which folds the
# binder instead of opening a tab.
call("invoke_action", {"node": book["id"], "action": "click"})
time.sleep(2.5)
ns = nodes()
seg = next((n for n in ns if (n.get("label") or "").strip() == "Full Book"), None)
check(seg is not None, "the Book tab offers a Full Book segment")
if seg is None:
    die("no Full Book segment")
call("invoke_action", {"node": seg["id"], "action": "click"})
time.sleep(4.0)

before = shot("book-before.png")
ns = nodes()
rows = sorted((n for n in ns if n.get("role") == "MultilineTextInput"),
              key=lambda n: (n.get("bounds") or {}).get("y", 0))
check(len(rows) > 5, f"the stream built several rows (got {len(rows)})")

# **Read the whole book, then come back.** This is the reported action, and getting
# it wrong is why an earlier version of this test passed against a broken build:
# it *focused* a row instead, which reveals one row without ever painting the ones
# between — so nothing settled and the lane could not move.
#
# The scroll has to be addressed to the **ScrollView**, not to an editor inside it.
# Addressed to a row's editor it silently does nothing, and the probe then reports a
# pass on a page that never moved.
view = max((n for n in ns if n.get("role") == "ScrollView"),
           key=lambda n: (n.get("bounds") or {}).get("height", 0), default=None)
check(view is not None, "the stream has a scroll view to drive")
if view is None:
    die("no ScrollView on the page")

for _ in range(120):
    call("scroll", {"node": view["id"], "dx": 0.0, "dy": -700.0})
    time.sleep(0.05)
call("settle")
time.sleep(2.0)
shot("book-bottom.png")

for _ in range(200):
    call("scroll", {"node": view["id"], "dx": 0.0, "dy": 700.0})
    time.sleep(0.04)
call("settle")
time.sleep(2.0)
after = shot("book-after.png")

# Compare the strip itself. The **boundary marks** are the measurement: one per
# mapped document, so their positions are the row extents made visible. If a row's
# slice changed, its rule moved and every rule below it moved with it.
try:
    from PIL import Image
except ImportError:
    print("  (Pillow not installed — screenshots saved, comparison skipped)")
else:
    def rule_rows(path, lane, scale):
        b = lane["bounds"]
        im = Image.open(path).convert("RGB")
        # Accessibility bounds are **logical**; a screenshot is physical pixels, and
        # on a scaled display the two differ by the display factor. Derived from the
        # window rather than assumed, or the crop lands on the prose instead.
        box = (int((b["x"] + b["width"] * 0.55) * scale), int(b["y"] * scale),
               int((b["x"] + b["width"]) * scale), int((b["y"] + b["height"]) * scale))
        crop = im.crop(box)
        px = crop.load()
        out = []
        for y in range(crop.height):
            for x in range(crop.width):
                r, g, bl = px[x, y]
                if r > 150 and bl > 100 and g < 120:
                    out.append(y)
                    break
        return out

    lane_node = lane(nodes())
    # The display scale: the widest node in the tree is the window, and the
    # screenshot is that window in physical pixels.
    widest = max((n.get("bounds") or {}).get("width", 0.0) for n in nodes())
    scale = Image.open(before).width / widest if before and widest else 1.0
    print(f"  display scale {scale:.2f}")
    if lane_node is None or before is None or after is None:
        print("  (no lane or no screenshots — comparison skipped)")
    else:
        ra, rb = rule_rows(before, lane_node, scale), rule_rows(after, lane_node, scale)
        common = len(set(ra) & set(rb))
        share = common / max(1, len(ra))
        print(f"  boundary rules: {len(ra)} before, {len(rb)} after, {common} unmoved")
        check(
            share > 0.9,
            f"the map holds still after reading the whole book ({share:.0%} of its rules unmoved)",
        )

        # **And the bars, which is what was actually reported.** The rules holding
        # still says the row *slots* did not move; it says nothing about whether the
        # texture inside them reorganised, and "one paragraph per line instead of the
        # merged way" was the complaint. Counted from the drawn column rather than
        # through an API, because merging happens before the widget sees the bars and
        # there is nothing to ask.
        def bar_count(path, lane, scale):
            b = lane["bounds"]
            im = Image.open(path).convert("RGB")
            box = (int(b["x"] * scale), int(b["y"] * scale),
                   int((b["x"] + b["width"] * 0.5) * scale),
                   int((b["y"] + b["height"]) * scale))
            crop = im.crop(box)
            px = crop.load()
            # A bar is a run of rows that have ink; count the runs, not the rows.
            bars, inside = 0, False
            for y in range(crop.height):
                ink = any(sum(px[x, y]) < 690 for x in range(crop.width))
                if ink and not inside:
                    bars += 1
                inside = ink
            return bars

        bars_a = bar_count(before, lane_node, scale)
        bars_b = bar_count(after, lane_node, scale)
        print(f"  texture bars: {bars_a} before, {bars_b} after")
        # A little movement is honest — the row that was revealed does get its exact
        # height — but the reported bug was a whole region going from merged to
        # one-per-paragraph, which doubles a book's bar count or worse.
        drift = abs(bars_b - bars_a) / max(1, bars_a)
        check(
            drift < 0.15,
            f"the texture does not reorganise after reading the whole book ({bars_a} -> {bars_b} bars)",
        )

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()

print()
if failures:
    print(f"FAIL: {len(failures)} check(s) failed")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("ALL PASS")
print(f"screenshots in {sandbox}")
