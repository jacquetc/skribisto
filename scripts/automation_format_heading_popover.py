#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The Format dock must survive its own heading popover.

    scripts/automation_format_heading_popover.py resources/examples/Starforgers.skrib

Every button in the Format dock is `focusable(false)` so that pressing one never
blurs the editor out from under itself — but the heading picker is a
`PopoverIconButton`, and a popover is not a button: it moves keyboard focus into
its list (it has to, or the levels would be unreachable from the keyboard). The
resolver then reports "nothing focused", which is exactly what it reports when
you click into the binder, and the dock answered by hiding every group — the
picker's own group included, whose subtree going dormant took the just-opened
list with it. Pressing Heading level blanked the whole dock.

Asserts, against the live app, with a scene open and the caret in its prose:

  1. the Format dock shows its groups — Bold / Undo / Heading level and friends;
  2. clicking **Heading level** opens the list — all seven levels are reachable;
  3. and the dock is *still standing behind it*: every one of those buttons;
  4. picking a level applies it and leaves the dock live, i.e. focus came back.

Step 3 is the regression. Steps 1–2 are there so a failure at 3 cannot be
confused with "the dock was never up" or "the click missed".
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import isolated_config, working_copy

import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
OUT = "/tmp/format-heading-popover"
os.makedirs(OUT, exist_ok=True)

# Both halves of the isolation matter here. A private `XDG_CONFIG_HOME` keeps
# `workspace.toml` out of it — the desk is keyed by the Work's uid, so a
# *previous run of this very probe* would otherwise hand the next one a closed
# Format dock and a different splitter, and the run would fail at "the caret
# never landed" for a reason that has nothing to do with the popover. A private
# copy of the project keeps the heading this probe applies out of the operator's
# example file.
env = isolated_config(locale="en-US", label="fmt-heading", show_welcome=False)
project = working_copy(os.path.abspath(sys.argv[1]), "fmt-heading") \
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
                        "clientInfo": {"name": "format-heading-popover", "version": "1"}})
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


def all_widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def find(label, timeout=6.0):
    stop = time.time() + timeout
    while time.time() < stop:
        for n in nodes():
            if n.get("label") == label:
                return n
        settle()
    return None


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


def shot(name):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            path = os.path.join(OUT, f"{name}.png")
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot → {path}")
            return path
    return None


# ── reach a scene with the caret in its prose ────────────────────────────────
if project is None:
    if not click("Mock Project", "(needs a --features mocks build)"):
        die("no 'Mock Project' recent row and no project argument")
# `load_work` is SLOW in a debug build (~20 s for the bundled example) — without
# this wait every check below runs against the empty shell.
end = time.time() + 120
while time.time() < end:
    if "Binder" in labels():
        break
    settle()
else:
    die("the project never finished loading")
settle()
print("project loaded")

def editor_area():
    """The editor pane's box: the widest `TabPanel` in the tree.

    A rich-text editor exposes no distinguishing AccessKit role here (its prose
    arrives as child Label nodes, and the docks' panels carry the same roles),
    so the pane is located geometrically and the caret is placed by clicking
    into it. Whether that worked is then read off the dock itself — if the Bold
    button is up, the caret is in prose. That is the property the test needs,
    and it beats any amount of widget archaeology.
    """
    best = None
    for n in nodes():
        if n.get("role") != "TabPanel":
            continue
        b = n.get("bounds") or {}
        if "x" not in b or b.get("height", 0) < 200:
            continue
        if best is None or b["width"] > best["width"]:
            best = b
    return best


# Open a scene, so the caret lands in manuscript prose rather than in whatever
# the restored desk happened to leave open.
for row in ("Prologue", "Chapter 1", "Chapter 2"):
    if click(row, "(a binder row)"):
        break
else:
    die("no scene row to open in the binder")

def reveal_format_dock():
    """Click the Format dock's activity-rail entry.

    Picked by shape, not by label alone: the menu bar has a "Format" menu with
    the same accessible name, and clicking *that* opens a menu instead of the
    dock — which then fails three steps later as "the caret never landed", the
    one failure this probe must never confuse with the bug under test.
    """
    rail = [n for n in nodes()
            if n.get("label") == "Format"
            and 0 < (n.get("bounds") or {}).get("width", 0) < 60
            and 0 < (n.get("bounds") or {}).get("height", 0) < 60]
    return bool(rail) and click_node(rail[0])


def place_caret():
    """Click down the editor column until the dock says the caret is in prose.

    The dock itself is the oracle — Bold up means a formattable surface has
    focus — rather than any widget-tree lookup for the editor. Its placeholder
    text is *not* usable for this: the empty state reaches AccessKit as painted
    text, not as a node label, so an "is the dock empty" check reads false
    whether the dock is empty or absent. A button that is either there or not
    cannot be ambiguous that way.

    Several depths, because the top of the pane is the title field and a
    container's own page puts an epigraph box above its prose.
    """
    area = editor_area()
    if area is None:
        return None
    for frac in (0.35, 0.5, 0.22, 0.65):
        x = area["x"] + area["width"] * 0.45
        y = area["y"] + area["height"] * frac
        call("inject_pointer", {"x": x, "y": y, "action": "click"})
        settle()
        time.sleep(0.4)
        if "Bold" in labels():
            return (x, y)
    return None


# The dock may already be up (it is on the trailing rail by default); only go
# looking for the rail entry if the caret alone did not bring the buttons out.
placed = place_caret()
if placed is None:
    if not reveal_format_dock():
        shot("00-no-dock")
        die("the Format dock is not showing and its activity-rail entry could "
            "not be found")
    settle()
    placed = place_caret()
if placed is None:
    shot("00-no-caret")
    die("clicking into the editor pane never put the caret in a formattable "
        "surface — the Format dock stayed empty")
print(f"scene open, caret in the prose at {placed[0]:.0f},{placed[1]:.0f}")

failures = []
BUTTONS = ["Undo", "Bold", "Italic", "Heading level", "Align left", "Bulleted list"]
LEVELS = ["Normal text", "Heading 1", "Heading 2", "Heading 3",
          "Heading 4", "Heading 5", "Heading 6"]

# ── 1. the dock is up and populated ──────────────────────────────────────────
print("\n=== 1. the dock shows its groups ===")
present = labels()
for b in BUTTONS:
    if b in present:
        print(f"  {b!r} present ✓")
    else:
        failures.append(f"before the click: {b!r} missing from the Format dock")
shot("01-dock-open")
if failures:
    for f in failures:
        print("  !!", f)
    die("the dock was not in a state worth testing; see " + OUT)

# ── 2. clicking Heading level opens the list ─────────────────────────────────
print("\n=== 2. clicking 'Heading level' opens the seven levels ===")
picker = find("Heading level")
if not picker or not click_node(picker):
    die("could not click the 'Heading level' button")
after = labels()
shot("02-popover-open")
missing = [l for l in LEVELS if l not in after]
if missing:
    failures.append(f"the popover did not open (levels missing: {missing})")
else:
    print("  all seven levels reachable ✓")

# ── 3. THE REGRESSION: the dock is still standing behind it ──────────────────
print("\n=== 3. the dock survives its own popover ===")
for b in BUTTONS:
    if b in after:
        print(f"  {b!r} still present ✓")
    else:
        failures.append(f"after the click: {b!r} VANISHED — the dock blanked itself")

# ── and picking a level puts the caret back ──────────────────────────────────
print("\n=== 4. picking a level leaves the dock live ===")
if click("Heading 2", "(a level in the open popover)"):
    settle()
    time.sleep(0.5)
    back = labels()
    shot("03-after-pick")
    for b in BUTTONS:
        if b not in back:
            failures.append(f"after picking a level: {b!r} missing — focus never "
                            "came back to the editor")
    if not [f for f in failures if "after picking" in f]:
        print("  the dock is live again after the pick ✓")

print()
if failures:
    for f in failures:
        print("FAIL:", f)
    print(f"\n{len(failures)} failure(s) — screenshots in {OUT}")
    code = 1
else:
    print(f"PASS — the Format dock survives its heading popover. Screenshots in {OUT}")
    code = 0

for p in (app, mcp):
    if p.poll() is None:
        p.terminate()
sys.exit(code)
