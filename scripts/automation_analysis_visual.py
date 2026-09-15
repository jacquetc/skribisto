#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the Analysis segment to a rendered report and screenshot it.

The Analysis panel's body is one `teksu!` block whose children are all helper
calls (`heading`, `note`, `wide_chart`, `empty_toggle`) and two conditionals.
Nothing in it is a stock widget, so a build that compiles proves very little:
if the children did not attach, the panel is simply empty. This probe opens the
book, runs the analysis, and asserts the block's own rows are in the
accessibility tree, then writes a PNG.

Run: python3 scripts/automation_analysis_visual.py [project.skrib]
"""
import base64
import json
import os
import select
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

MCP = fixture.mcp_binary()
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else fixture.repo_path(
    "resources/examples/le_tour_du_monde_en_80_jours/"
    "le-tour-du-monde-en-quatre-vingts-jours.skrib"))
OUT = os.environ.get("ANALYSIS_SHOT_DIR", tempfile.mkdtemp(prefix="analysis-vis-"))
#: Everything left of this is the binder; the container bar and the report are
#: right of it. Same constant, same reason, as `automation_corkboard`.
PANE_X = 300

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-20:]))
    try:
        et = open(mcp_err).read()
        if et.strip():
            print("--- mcp stderr ---")
            print("\n".join(et.splitlines()[-15:]))
    except Exception:
        pass
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


fixture.assert_no_running_instance()
env = fixture.isolated_config(locale="en-US", label="analysis-vis", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="analysis-vis")
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=pins),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT, env=env)
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
except RuntimeError as e:
    die(str(e), app)
print(f"bridge up: {bridge.endpoint}")

mcp = None
_id = [0]


def send(method, params=None, notif=False):
    m = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        m["params"] = params
    if not notif:
        _id[0] += 1
        m["id"] = _id[0]
    mcp.stdin.write(json.dumps(m) + "\n")
    mcp.stdin.flush()


def recv(timeout=30, fatal=True):
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


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    res = recv().get("result", {})
    if res.get("isError"):
        txt = "".join(c.get("text", "") for c in res.get("content", [])
                      if c.get("type") == "text")
        print(f"  !! {name}: {txt[:160]}")
    payload = res.get("structuredContent")
    if payload is None:
        txt = "".join(c.get("text", "") for c in res.get("content", [])
                      if c.get("type") == "text")
        payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
    return res, payload


deadline = time.time() + 25
init = None
while time.time() < deadline and init is None:
    mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                           stdout=subprocess.PIPE, stderr=open(mcp_err, "w"),
                           text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "analysis-vis", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None:
        if mcp.poll() is None:
            mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP", app)
send("notifications/initialized", notif=True)


def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def find(label, timeout=8.0, minx=None, contains=False):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            lb = n.get("label") or ""
            hit = (label.lower() in lb.lower()) if contains else (lb == label)
            if hit and (minx is None or (n.get("bounds") or {}).get("x", 0) >= minx):
                return n
        settle()
    return None


def click(label, what="", minx=None, timeout=8.0):
    n = find(label, timeout=timeout, minx=minx)
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
                                "y": b["y"] + b.get("height", 0) / 2,
                                "action": "click"})
    settle()
    time.sleep(0.5)
    return True


def shot(name):
    os.makedirs(OUT, exist_ok=True)
    path = os.path.join(OUT, f"{name}.png")
    res, _ = call("screenshot")
    b64 = None
    for c in res.get("content", []):
        if c.get("type") == "image" and isinstance(c.get("data"), str):
            b64 = c["data"]
    if b64:
        open(path, "wb").write(base64.b64decode(b64))
        print(f"  shot: {path}")
    return path


fixture.wait_for_load(nodes, ["Binder"], timeout=60)
print("work loaded")

# The Analysis segment is gated to a Folder/Book, and on this fixture the book
# is the binder root. A single click selects the container and raises its
# segment bar; no double click is needed.
# The Analysis segment is gated to a `Folder/Book` binder item. On this fixture
# that is the novel itself, not the "Manuscrit" root above it, which is the
# manuscript container: clicking the root selects it and opens nothing.
BOOK = "Le Tour du monde en quatre-vingts jours"
row = None
for n in nodes():
    if n.get("role") == "TreeItem" and BOOK.lower() in (n.get("label") or "").lower():
        row = n
        break
if row is None:
    die(f"{BOOK!r} not in the binder", app, mcp)
b = row.get("bounds") or {}
cx = b.get("x", 0) + b.get("width", 0) / 2
cy = b.get("y", 0) + b.get("height", 0) / 2
# The binder activates on a single click (`ActivateOn::SingleClick`).
call("inject_pointer", {"x": cx, "y": cy, "action": "click"})
settle()
# Park the pointer off the row: its hover tooltip otherwise sits over the
# editor area and swallows the next synthetic click.
call("inject_pointer", {"x": 600, "y": 300, "action": "move"})
settle()
time.sleep(1.5)
print(f"opened {BOOK!r}")

# The container bar is a SegmentedControl, and on this window width it
# overflows: only Book / Full Book / Full Synopsis / Pace fit, and Analysis
# sits behind the trailing chevron. Open the overflow first if it is not
# already visible.
if find("Analysis", timeout=3.0, minx=PANE_X) is None:
    chev = None
    for n in nodes():
        bb = n.get("bounds") or {}
        lb = (n.get("label") or "").lower()
        if bb.get("x", 0) >= PANE_X and bb.get("y", 0) < 200 and (
                "more" in lb or "overflow" in lb or "show all" in lb):
            chev = n
            break
    if chev is not None:
        print(f"opening segment overflow: {chev.get('label')!r}")
        call("invoke_action", {"node": chev["id"], "action": "click"})
    else:
        # No named chevron in the AT tree: click where the bar ends.
        print("opening segment overflow by position")
        call("inject_pointer", {"x": 830, "y": 108, "action": "click"})
    settle()
    time.sleep(1.0)

if not click("Analysis", "(segment)", minx=PANE_X):
    print("--- nodes right of the binder ---")
    for n in nodes():
        bb = n.get("bounds") or {}
        if bb.get("x", 0) >= PANE_X and (n.get("label") or "").strip():
            print(f"    {n.get('role','?'):16} x={bb.get('x',0):4.0f} {n.get('label')[:50]!r}")
    shot("analysis-segment-missing")
    die("no Analysis segment in the container bar", app, mcp)
print("analysis segment open")

# The report is computed on demand, so the block under test renders nothing
# until this runs.
if click("Run analysis", "(button)", minx=PANE_X):
    print("analysis running")
    end = time.time() + 90
    while time.time() < end:
        if find("Words per scene", timeout=2.0, minx=PANE_X, contains=True):
            break
        settle()

# The rows the migrated `teksu!` body produces, each from a bare helper call:
# `heading(..)`, `note(..)` and the `if`/`else` pair around `wide_chart(..)`.
WANT = ["Words per scene", "Dialogue"]
found = {}
for w in WANT:
    n = find(w, timeout=25, minx=PANE_X, contains=True)
    found[w] = n is not None
    print(f"  {'OK  ' if n else 'MISS'} {w}")

shot("analysis-report")

# The Arrivals view is the block's sibling: same shape, six more bare children
# and the `if total == 0` empty state.
if click("Arrivals", "(sub-segment)", minx=PANE_X):
    time.sleep(1.5)
    for w in ("Arrivals",):
        n = find(w, timeout=15, minx=PANE_X, contains=True)
        found[f"arrivals/{w}"] = n is not None
        print(f"  {'OK  ' if n else 'MISS'} arrivals/{w}")
    shot("analysis-arrivals")

ok = all(found.values())
print("\nRESULT:", "the migrated block rendered its rows"
      if ok else "ROWS MISSING - the bare-expression children did not attach")
print(f"screenshot dir: {OUT}")
print(f"app log: {log}")
app.terminate()
mcp.terminate()
sys.exit(0 if ok else 1)
