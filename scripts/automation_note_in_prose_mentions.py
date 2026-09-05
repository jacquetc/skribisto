#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""**The In prose reading's mention counter, driven on screen.**

Two things about this feature are invisible to every headless test, and both were
wrong when it first shipped.

1. **The matcher.** `SubjectHighlight` marks where the entry is named. A first cut
   compared names byte for byte while the rest of the app folds diacritics, so an
   entry called "Élise Laroche" marked nothing at all and her ASCII alias "Claire"
   marked three times. A unit test with names the author typed themselves cannot
   catch that — the author types the same spelling on both sides. Only a real
   project written in French does.

2. **The frame.** Stepping writes the counter's two signals, and the label binds a
   *derived* signal over them, which the binding registry picks up by polling on
   the next frame it runs. A press that scrolled nothing requested no frame, so the
   counter showed the previous press's answer: "129 mentions" after the first Next,
   "1 of 129" after the second. Every unit test passed — the walk was right the
   whole time — and the page said so one interaction late. Nothing but a live
   window can see a repaint that never happened.

So this probe presses the chevrons and reads the label back, which is exactly the
loop a writer performs.

Usage: `python3 scripts/automation_note_in_prose_mentions.py <project.skrib> [entry]`
The project must hold a story-bible entry (default "Elise") declared present at
least 35 times across the reading. The walk below presses Next 35 times in a row
before pressing Previous once and checking the ordinal it lands on; `SubjectWalk`
wraps round the whole reading once every mention has been visited, so with fewer
than 35 mentions the 35th press has already wrapped and every assertion after it
reads a wrapped position, not a bug.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else "/tmp/p02/Starforgers.skrib")
ENTRY = sys.argv[2] if len(sys.argv) > 2 else "Elise"

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
app = mcp = None


def fail(msg):
    print("FAIL:", msg)
    try:
        print("--- app log tail ---")
        pl = [l for l in open(log).readlines() if "PROBE" in l]
        print(f"PROBE lines: {len(pl)}")
        print("".join(pl[:3])); print("..."); print("".join(pl[-4:]))
    except Exception:
        pass
    for p in (mcp, app):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


env = fixture.isolated_config(locale="en-US", label="in-prose-mentions", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="in-prose-mentions")
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=pins), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=40)
except RuntimeError as e:
    fail(str(e))

mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=open(mcp_err, "w"), text=True, bufsize=1)
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


def recv(timeout=25, want=None):
    """Read until the response carrying `want` arrives.

    Id-matched deliberately: a slow reply consumed by the *next* read shifts every
    later one by one, and the symptom is an empty tree rather than an error.
    """
    e = time.time() + timeout
    while time.time() < e:
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, e - time.time()))
        if not r:
            return None
        line = mcp.stdout.readline()
        if not line:
            return None
        if not line.strip():
            continue
        msg = json.loads(line)
        if want is None or msg.get("id") == want:
            return msg
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    res = (recv(want=_id[0]) or {}).get("result", {})
    return res, (res.get("structuredContent") or {})


send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                    "clientInfo": {"name": "in-prose-mentions", "version": "1"}})
if recv(10, want=1) is None:
    fail("MCP did not initialize")
send("notifications/initialized", notif=True)
time.sleep(4)


def nodes(tries=12):
    """The first snapshot after launch comes back empty — the window is up before
    the tree is. Retry rather than treat that as the answer."""
    for _ in range(tries):
        _, p = call("snapshot_tree")
        if p.get("nodes"):
            return p["nodes"]
        time.sleep(1.0)
    return []


def text_of(n):
    return ((n.get("value") or "") + " " + (n.get("label") or "")).strip()


def shot(path):
    res, _ = call("screenshot", {})
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print("  screenshot →", path)


ns = nodes()
if not ns:
    fail("no accessibility tree within 12 tries")
print(f"connected — {len(ns)} nodes")

# The entry, in the binder tree (x < 340 is the leading dock's own column).
rows = [n for n in ns
        if n.get("role") == "TreeItem" and ENTRY.lower() in text_of(n).lower()]
if not rows:
    fail(f"no binder row for {ENTRY!r}")
b = rows[0].get("bounds") or {}
call("inject_pointer", {"x": b["x"] + 60, "y": b["y"] + b.get("height", 12) / 2,
                        "action": "double_click"})
time.sleep(2.5)

# Its In prose segment.
ns = nodes()
chip = [n for n in ns if text_of(n).strip().lower() in ("in prose", "dans le texte")]
if not chip:
    fail("the entry's tab has no In prose segment (is it carrying a discoverable tag?)")
call("invoke_action", {"node": chip[0]["id"], "action": "click"})
time.sleep(3.0)
ns = nodes()


#: "129 mentions" — the bare total, before anyone has stepped into it.
TOTAL_SHAPE = re.compile(r"^\s*\d+\s+mentions?\s*$", re.I)
#: "6 of 129" — the ordinal after a step. Matched by *shape* rather than by the
#: joining word, so the probe reads the same in either locale without this file
#: having to carry a translation of its own.
#:
#: Anchored to the whole label, not searched inside it: a French manuscript's own
#: prose says "Lundi 9 juillet 2007" and an unanchored three-token shape happily
#: reads that as an ordinal.
AT_SHAPE = re.compile(r"^\s*(\d+)\s+\w+\s+(\d+)\s*$")


def counter(ns):
    """The counter reads as a bare total until the reader steps into it, and as an
    ordinal after — so match both shapes."""
    return [text_of(n) for n in ns
            if TOTAL_SHAPE.search(text_of(n)) or AT_SHAPE.search(text_of(n))]


def chevron(ns, which):
    keys = {"next": ("next mention", "mention suivante"),
            "prev": ("previous mention", "mention précédente")}[which]
    hits = [n for n in ns if any(k in text_of(n).lower() for k in keys)]
    return hits[0] if hits else None


start = counter(ns)
nxt, prv = chevron(ns, "next"), chevron(ns, "prev")
print("counter:", start)
if not start:
    shot("/tmp/in_prose_no_counter.png")
    fail("no mention counter on the page — the entry is named nowhere the reading shows")
if not (nxt and prv):
    fail("the counter has no chevrons")

def at_is(labels, current, total):
    """Does the counter say the reader is on `current` of `total`?"""
    for t in labels:
        m = AT_SHAPE.search(t)
        if m and (int(m.group(1)), int(m.group(2))) == (current, total):
            return True
    return False


total = int(TOTAL_SHAPE.search(start[0]).group(0).split()[0])
if total < 35:
    fail("the fixture must name the entry at least 35 times so the walk below "
         f"(35 presses of Next, then one Previous) cannot wrap round the reading "
         f"before its assertions run; found {total}")

# **One press, one answer.** The regression: the label used to lag a press behind.
seen = []
for i in range(1, 6):
    call("invoke_action", {"node": nxt["id"], "action": "click"})
    time.sleep(1.0)
    now = counter(nodes())
    print(f"  next #{i} → {now}")
    seen.append(now[0] if now else "")
    if not at_is(now, i, total):
        shot("/tmp/in_prose_lagging.png")
        fail(f"after {i} press(es) the counter reads {now}, expected {i} of {total} — "
             "the label is a frame behind the walk")

# **The viewport follows.** A reveal walked from the chevron climbs out through the
# pinned bar and never meets the scrolling page; a reveal through a *dormant* editor
# (the same scene also open in a background tab) requests nothing at all. Either way
# the match is selected, the counter moves, and the page stays exactly where it was.
#
# Measured on a real node's own `y`: the first prose line of the reading. If the page
# scrolled, it moved up.
def first_prose_y():
    """The topmost text node inside the reading column, and where it sits."""
    best = None
    for n in nodes():
        b = n.get("bounds") or {}
        if b.get("x", 0) < 400 or b.get("x", 0) > 1400:
            continue
        if not text_of(n) or b.get("y", 0) < 150:
            continue
        if best is None or b["y"] < best[1]:
            best = (text_of(n)[:30], b["y"])
    return best


before = first_prose_y()
print("  top of the reading before:", before)
for _ in range(30):
    call("invoke_action", {"node": nxt["id"], "action": "click"})
    time.sleep(0.25)
time.sleep(1.2)
print("  after 30 more:", counter(nodes()))
after = first_prose_y()
print("  top of the reading after :", after)
if before is None or after is None:
    # first_prose_y() found no qualifying node either time — the scroll was never
    # observed, so it cannot be reported as followed. Say so plainly rather than
    # falling through to a PASS that would claim a check that did not happen.
    viewport_note = " (viewport-follow check skipped: no reading node found to measure)"
    print("SKIP: could not locate a reading node to measure the scroll against")
elif abs(before[1] - after[1]) < 20.0 and before[0] == after[0]:
    shot("/tmp/in_prose_no_scroll.png")
    fail(f"the viewport did not follow: 35 mentions on and the reading still starts at "
         f"{after} — a reveal that reaches no scroll container, or reaches a dormant editor")
else:
    viewport_note = " (the page followed)"

call("invoke_action", {"node": prv["id"], "action": "click"})
time.sleep(1.0)
back = counter(nodes())
print("  prev  →", back)
if not at_is(back, 34, total):
    fail(f"stepping back reads {back}, expected 34 of {total}")

shot("/tmp/in_prose_mentions.png")
print(f"PASS: {total} mentions, stepped 1..35{viewport_note} and back to 34")
for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
