#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The Format dock's "nothing focused" hint must wrap, at every dock width.

    scripts/automation_format_empty_state.py resources/examples/Starforgers.skrib

With no formattable surface focused the dock hides every group and shows one
sentence instead. That sentence was laid out by `Center`, which measures its
child with an *unbounded* proposal — so `TextWidget`, which wraps only against a
bounded width, measured as a single 592px line. `Center` then placed that line at
half its overhang, a negative x, and the hint ran off the dock on the left and
the right at once.

The trailing side is a splitter the writer drags, so one width proves nothing.
This drives the real app and, at three widths across that range, asserts:

  1. the dock is in its empty state — the group buttons are gone;
  2. the hint's box stays inside the dock's padded inset, both edges;
  3. it is *wrapped*, not clipped — more than one line tall.

Read against the layout tree rather than AccessKit: the placeholder reaches the
AT tree as painted text, not as a node label, so only the arena can answer where
its box actually is.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import isolated_config, working_copy

SKRIBISTO = os.environ.get(
    "SKRIBISTO_BIN", "/home/cyril/Devel/skribisto/target/debug/skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
OUT = "/tmp/format-empty-state"
os.makedirs(OUT, exist_ok=True)

# A private config dir keeps `workspace.toml` out of the operator's own: the desk
# is keyed by the Work's uid, so a previous run's dragged splitter would decide
# this run's starting width.
env = isolated_config(locale="en-US", label="fmt-empty", show_welcome=False)
project = working_copy(os.path.abspath(sys.argv[1]), "fmt-empty") \
    if len(sys.argv) > 1 else None

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
argv = [SKRIBISTO] + ([project] if project else [])
app = subprocess.Popen(argv, env=env, stdout=open(log, "w"), stderr=subprocess.STDOUT)
mcp = None


def die(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


sock = tok = None
end = time.time() + 30
while time.time() < end:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        die("the app exited before printing its bridge socket")
    time.sleep(0.2)
if not sock:
    die("no bridge socket within 30s")

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
    stop = time.time() + timeout
    while time.time() < stop:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, stop - time.time()))
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


init = None
end = time.time() + 30
while time.time() < end and init is None:
    while not os.path.exists(sock) and time.time() < end:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "format-empty-state", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect the automation MCP")
send("notifications/initialized", notif=True)
print("connected")


def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def labels():
    return {n.get("label") for n in nodes() if n.get("label")}


def click_node(n):
    b = n.get("bounds") or {}
    if "x" not in b:
        return False
    call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                            "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    settle()
    time.sleep(0.4)
    return True


def click(label, what=""):
    n = find(label)
    if not n:
        print(f"  !! no node labelled {label!r} {what}")
        return False
    return click_node(n)


def find(label, timeout=6.0):
    stop = time.time() + timeout
    while time.time() < stop:
        for n in nodes():
            if n.get("label") == label:
                return n
        settle()
    return None


def shot(name):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            path = os.path.join(OUT, f"{name}.png")
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot → {path}")
            return path
    return None


# ── load the project ─────────────────────────────────────────────────────────
if project is None:
    if not click("Mock Project", "(needs a --features mocks build)"):
        die("no 'Mock Project' recent row and no project argument")
# `load_work` is SLOW in a debug build (~20 s for the bundled example).
end = time.time() + 120
while time.time() < end:
    if "Binder" in labels():
        break
    settle()
else:
    die("the project never finished loading")
settle()
print("project loaded")


def reveal_format_dock():
    """Click the Format dock's activity-rail entry — picked by shape, because
    the menu bar carries a "Format" menu under the same accessible name."""
    rail = [n for n in nodes()
            if n.get("label") == "Format"
            and 0 < (n.get("bounds") or {}).get("width", 0) < 60
            and 0 < (n.get("bounds") or {}).get("height", 0) < 60]
    return bool(rail) and click_node(rail[0])


if not reveal_format_dock():
    shot("00-no-dock")
    die("the Format dock's activity-rail entry could not be found")
settle()


def dock_box():
    """The `FormatDock` widget's own bounds, from the arena."""
    for w in widgets():
        if w.get("type", "").endswith("::FormatDock") and w.get("active", True):
            return w
    return None


def hint_box(dock):
    """The one live `TextWidget` under the dock — the empty state's sentence."""
    by_id = {w["id"]: w for w in widgets()}
    out, stack = [], [dock["id"]]
    while stack:
        w = by_id.get(stack.pop())
        if not w:
            continue
        if w.get("type", "").endswith("::TextWidget") \
                and w.get("active", True) \
                and (w.get("bounds") or {}).get("width", 0) > 0:
            out.append(w)
        stack.extend(w.get("children", []))
    return out


if dock_box() is None:
    shot("00-no-dock-widget")
    die("no FormatDock in the layout tree — the rail click did not open it")

# Blur the editor: clicking a binder row moves focus out of any prose, which is
# exactly the state the placeholder exists for.
if not click("Binder", "(the binder dock's own header)"):
    for row in ("Manuscript", "Notes"):
        if click(row, "(a binder row)"):
            break
settle()

if "Bold" in labels():
    shot("00-still-focused")
    die("the editor still has focus — the dock is showing its groups, not the "
        "empty state")
print("nothing formattable focused: the dock is in its empty state")

# ── drag the trailing splitter and re-measure at each width ──────────────────
DOCK_PADDING = 8.0

failures = []
# The sentence's natural extent, learned from the widest dock in the sweep —
# where it does fit on one line, and should. Every narrower dock has less room
# than that, so there the hint must be more than one line tall or it was clipped
# rather than wrapped. Hardcoding a pixel threshold would instead make this a
# translation test: fr-FR's string is longer, and both are free to change.
natural = {"width": 0.0, "line": 0.0}


def measure(tag):
    dock = dock_box()
    if dock is None:
        failures.append(f"{tag}: the FormatDock vanished from the layout tree")
        return None
    db = dock["bounds"]
    hints = hint_box(dock)
    if len(hints) != 1:
        failures.append(f"{tag}: expected exactly one live label in the empty "
                        f"dock, found {len(hints)}")
        return None
    hb = hints[0]["bounds"]
    left = hb["x"] - db["x"]
    right = (db["x"] + db["width"]) - (hb["x"] + hb["width"])
    print(f"  {tag}: dock {db['width']:.0f}px wide, hint {hb['width']:.0f}x"
          f"{hb['height']:.0f} — insets L{left:.0f} R{right:.0f}")

    if left < DOCK_PADDING - 0.5 or right < DOCK_PADDING - 0.5:
        failures.append(f"{tag}: the hint escapes the dock's {DOCK_PADDING}px "
                        f"inset (left {left:.0f}, right {right:.0f})")

    room = db["width"] - 2 * DOCK_PADDING
    if natural["width"] and room < natural["width"] - 1.0 \
            and hb["height"] <= natural["line"] + 1.0:
        failures.append(
            f"{tag}: {room:.0f}px of room for a {natural['width']:.0f}px "
            f"sentence, yet the hint is still one {hb['height']:.0f}px line — "
            f"clipped, not wrapped")
    return hb


def drag_side_to(target_width):
    """Drag the trailing splitter until the dock is ~`target_width` wide.

    The gutter is the strip immediately left of the dock body; grabbing it a few
    pixels out and moving by the width delta is enough — the docking model
    clamps to its own floor, and the measurement below reads the width that
    actually resulted rather than the one asked for.
    """
    dock = dock_box()
    if dock is None:
        return
    db = dock["bounds"]
    delta = db["width"] - target_width
    x = db["x"] - 3
    y = db["y"] + db["height"] / 2
    call("inject_pointer", {"x": x, "y": y, "action": "move"})
    call("inject_pointer", {"x": x, "y": y, "action": "down"})
    # A few intermediate moves: a single jump can be read as a click.
    for step in (0.34, 0.67, 1.0):
        call("inject_pointer", {"x": x + delta * step, "y": y, "action": "move"})
    call("inject_pointer", {"x": x + delta, "y": y, "action": "up"})
    settle()
    time.sleep(0.4)


print("\n=== the hint at every width the splitter reaches ===")
# Widest first, and on its own: that is the measurement every later one is
# judged against, so it has to be taken before the narrow ones are.
drag_side_to(520.0)
wide = measure("widened")
shot("01-widened")
if wide is None:
    die("could not measure the hint at the widest dock; see " + OUT)
natural["width"], natural["line"] = wide["width"], wide["height"]

for target, tag in ((300.0, "as opened"), (130.0, "narrowed")):
    drag_side_to(target)
    measure(tag)
    shot(f"02-{tag.replace(' ', '-')}")

print()
if failures:
    for f in failures:
        print("  !!", f)
    die(f"{len(failures)} failure(s); screenshots in " + OUT)

print("PASS: the placeholder wraps inside the dock at every width")
for p in (app, mcp):
    if p and p.poll() is None:
        p.terminate()
