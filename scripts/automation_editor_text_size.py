#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe for **editor text size** (Ctrl+Wheel and Ctrl+= / Ctrl+- / Ctrl+0).

The headless tests already dispatch a real `WidgetEvent::Scroll` with
`Modifiers::CTRL` through a real `writing_column`, so the wheel arithmetic and
the preview-pass interception are pinned there. What only the running app can
show is the rest of the chain: that the chords are registered globally and are
not eaten by the editor, that they resolve the editor the writer is *in* through
the format registry, that the new size reaches the mounted `RichTextEditor`, and
that the readout appears without stacking one toast per keystroke.

Asserts, against the running app:

  1. a scene opens and its editor takes focus;
  2. Ctrl+= raises the manuscript size and says so — the toast names
     *Manuscript*, not another of the six surfaces;
  3. repeating it does not stack toasts: still exactly one live readout;
  4. Ctrl+- brings it back down;
  5. Ctrl+0 restores the compile-time default (100%);
  6. an **unmodified** wheel over the prose changes nothing — the guard that the
     gesture has not swallowed ordinary scrolling;
  7. Ctrl+Wheel **up** grows the text, and
  8. Ctrl+Wheel **down** shrinks it again.

7 and 8 need `scroll`'s `ctrl` argument, added to teksilo's automation bridge
for this (`c658b4a4`) — it hardcoded `Modifiers::NONE`, so a modifier-held wheel
was the one input the bridge could describe but not perform. Without it a probe
can only ever send the plain wheel, and "scrolled when it should have zoomed" and
"zoomed when it should have scrolled" are indistinguishable.

Run: python3 scripts/automation_editor_text_size.py [project.skrib]
"""
import json
import os
import re
import select
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import isolated_config, working_copy  # noqa: E402

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = os.path.join(HERE, "target/debug/skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
SRC = os.path.abspath(
    sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "resources/examples/Starforgers.skrib")
)
# Never open the checked-in fixture: autosave is real, and this probe writes
# settings besides.
PROJECT = working_copy(SRC)
# en-US so the toast assertions read the strings this probe knows.
ENV = isolated_config(locale="en-US", label="textsize", show_welcome=False)

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = mcp = None


def fail(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    try:
        print("\n".join(open(log).read().splitlines()[-25:]))
    except OSError:
        pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


app = subprocess.Popen(
    [SKRIBISTO, PROJECT], stdout=open(log, "w"), stderr=subprocess.STDOUT, env=ENV
)

sock = tok = None
deadline = time.time() + 90
while time.time() < deadline:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        fail("app exited before printing the bridge socket")
    time.sleep(0.2)
if not sock:
    fail("no bridge socket within 90s")

_id = [0]


def send(method, params=None):
    msg = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        msg["params"] = params
    _id[0] += 1
    msg["id"] = _id[0]
    mcp.stdin.write(json.dumps(msg) + "\n")
    mcp.stdin.flush()


def recv(timeout=25, fatal=True):
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
        fail("no MCP response within timeout")
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = recv().get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(
            c.get("text", "") for c in result.get("content", []) if c.get("type") == "text"
        )
        payload = json.loads(text) if text.strip().startswith("{") else {}
    return result, payload


deadline = time.time() + 30
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen(
        [MCP, "--connect", sock, "--token", tok],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=open(mcp_err, "w"),
        text=True,
        bufsize=1,
    )
    send(
        "initialize",
        {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "editor-text-size-probe", "version": "1"},
        },
    )
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    fail("could not connect MCP")


def nodes():
    _r, payload = call("snapshot_tree")
    return payload.get("nodes", [])


def settle(times=3):
    for _ in range(times):
        call("settle")


def text_of(n):
    return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))


def wait_for(pred, what, timeout=90.0):
    end = time.time() + timeout
    while time.time() < end:
        hit = pred(nodes())
        if hit:
            return hit
        time.sleep(0.5)
    fail(f"timed out waiting for {what}")


def readouts():
    """Every live size-readout toast, by the wording only they carry.

    Counted by the toast's `Status` node, not by every node whose text matches:
    one toast surfaces as a `Status` **and** the `Label` inside it, so matching
    on text alone reports two of everything and a working dedup looks broken.
    """
    return [
        text_of(n) for n in nodes() if n.get("role") == "Status" and "text size" in text_of(n).lower()
    ]


def click_node(n):
    b = n.get("bounds")
    if not b:
        fail(f"node {text_of(n)!r} carries no bounds to click")
    call(
        "inject_pointer",
        {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2, "button": "left"},
    )
    settle(5)


# ── 1. Open a scene and focus its editor ────────────────────────────────────
#
# A scene leaf, not a chapter heading: a chapter tab stacks title + synopsis
# above its prose, so a click would land on padding rather than in the text.
wait_for(
    lambda ns: [
        n for n in ns if n.get("role") == "TreeItem" and text_of(n).strip().startswith("Prologue")
    ],
    "the binder to list the Prologue",
)
row = [
    n for n in nodes() if n.get("role") == "TreeItem" and text_of(n).strip().startswith("Prologue")
][0]
click_node(row)

def editors(ns):
    return [
        n
        for n in ns
        if n.get("role") == "MultilineTextInput" and "set_value" in (n.get("actions") or [])
    ]


def editor_text(node_id, ns):
    """An editor's prose — carried by its descendants, not the node itself."""
    by_id = {n["id"]: n for n in ns if "id" in n}
    parts = []

    def walk(n):
        for k in ("label", "value", "name"):
            if n.get(k):
                parts.append(str(n[k]))
                break
        for cid in n.get("children") or []:
            if (child := by_id.get(cid)) is not None:
                walk(child)

    if (node := by_id.get(node_id)) is not None:
        walk(node)
    return " ".join(parts)


def prose_editor(ns):
    """The manuscript editor: the one holding the most text.

    Re-resolved from a fresh snapshot every time — a tab rebuild re-mints a11y
    ids, and a cached one silently resolves to nothing.
    """
    best, best_len = None, -1
    for e in editors(ns):
        n = len(editor_text(e["id"], ns))
        if n > best_len:
            best, best_len = e, n
    return best, best_len


ns = wait_for(
    lambda ns: ns if (prose_editor(ns)[1] or 0) > 40 else None,
    "a prose editor with text",
)
editor = prose_editor(ns)[0]
# Click near the TOP of the editor, with a capped x — the prose editor's bounds
# run the whole document's height (1700px+ here), so its geometric centre is far
# below the viewport and a centre-click lands on nothing. Focus staying on the
# binder tree is exactly what that looks like, and the size commands then
# correctly no-op, which reads as "the feature is broken" rather than "the probe
# missed".
b = editor["bounds"]
call(
    "inject_pointer",
    {"x": b["x"] + min(b["width"] / 2, 200.0), "y": max(b["y"] + 24.0, 24.0), "button": "left"},
)
settle(4)
if not any(n.get("focused") for n in nodes()):
    fail("clicking the prose did not focus anything")

# ── 2. Ctrl+= raises the manuscript size, and names the right surface ───────
call("inject_key", {"key": "=", "ctrl": True})
settle(3)
time.sleep(0.4)
t = readouts()
if not t:
    fail("Ctrl+= produced no size readout")
if not any("Manuscript" in x for x in t):
    fail(f"the readout must name the Manuscript surface, got: {t}")
if not any("105%" in x for x in t):
    fail(f"one step from 100% must read 105%, got: {t}")
print("OK  Ctrl+= raised the manuscript size ->", t[0])

# ── 3. Repeating must update one toast, never stack a queue of them ─────────
for _ in range(4):
    call("inject_key", {"key": "=", "ctrl": True})
    call("settle", {})
time.sleep(0.4)
t = readouts()
if len(t) != 1:
    fail(f"a burst must update one readout, not stack {len(t)}: {t}")
if "125%" not in t[0]:
    fail(f"five steps from 100% must read 125%, got: {t}")
print("OK  a burst of five keeps exactly one readout ->", t[0])

# ── 4. Ctrl+- comes back down ───────────────────────────────────────────────
for _ in range(3):
    call("inject_key", {"key": "-", "ctrl": True})
    call("settle", {})
time.sleep(0.4)
t = readouts()
if not t or "110%" not in t[0]:
    fail(f"three steps down from 125% must read 110%, got: {t}")
print("OK  Ctrl+- lowered it ->", t[0])

# ── 5. Ctrl+0 restores this bundle's own default ────────────────────────────
call("inject_key", {"key": "0", "ctrl": True})
settle(3)
time.sleep(0.4)
t = readouts()
if not t or "100%" not in t[0]:
    fail(f"Ctrl+0 must restore the 100% default, got: {t}")
print("OK  Ctrl+0 reset it ->", t[0])

# ── 6. An unmodified wheel must not resize anything ─────────────────────────
before = t[0]
call("scroll", {"node": prose_editor(nodes())[0]["id"], "dx": 0.0, "dy": -240.0})
settle(3)
time.sleep(0.4)
after = [x for x in readouts() if x != before]
if after:
    fail(f"a plain wheel must not resize: {after}")
print("OK  a plain wheel scrolls without resizing")

# ── 7. Ctrl+Wheel UP grows the text ─────────────────────────────────────────
#
# The gesture itself, end to end through the real dispatcher. Note the sign:
# teksilo's `ScrollDelta` is a scroll *offset* delta, so wheel-up is NEGATIVE
# dy. A notch is 48px on the pixel path, so -240 is five notches.
call(
    "scroll",
    {"node": prose_editor(nodes())[0]["id"], "dx": 0.0, "dy": -240.0, "ctrl": True},
)
settle(3)
time.sleep(0.4)
t = readouts()
if not t:
    fail("Ctrl+Wheel produced no size readout")
if "125%" not in t[0]:
    fail(f"five notches up from 100% must read 125%, got: {t}")
print("OK  Ctrl+Wheel up grew the text ->", t[0])

# ── 8. …and Ctrl+Wheel DOWN shrinks it, on the same editor ──────────────────
call(
    "scroll",
    {"node": prose_editor(nodes())[0]["id"], "dx": 0.0, "dy": 144.0, "ctrl": True},
)
settle(3)
time.sleep(0.4)
t = readouts()
if not t or "110%" not in t[0]:
    fail(f"three notches down from 125% must read 110%, got: {t}")
print("OK  Ctrl+Wheel down shrank it ->", t[0])

call("screenshot", {"path": "/tmp/skribisto-text-size.png"})

print("\nAll editor-text-size assertions passed.")
for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
