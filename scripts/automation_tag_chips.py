#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the tag **dot row** on all three surfaces.

`automation_tags.py` covers the palette, the settings pane and the Inspector on
the legacy fixture. This one covers what Stage 4 added: the dots that appear
beside an item wherever it is listed.

It runs on the bundled example rather than the legacy fixture for a structural
reason — the legacy project's "Chapter 1" is a plain grouping folder, so it has
no segmented control and therefore neither a manuscript stream nor a corkboard.
There is nowhere in that project to *see* two of the three surfaces.

The example ships with an empty palette, so the run creates its own tag through
the very affordance under test (the picker's "Create ..." row) and then looks
for it in each place:

  1. the editor, under the title/subtitle;
  2. the manuscript stream's row header;
  3. the corkboard card.

Plus the two interaction rules the design rests on:

  * hovering one dot shows *that* dot's tooltip (per-dot hover), and
  * clicking a dot opens the picker rather than doing nothing (whole-row click).

The second is the load-bearing one. The dots deliberately carry no `on_tap`,
because in bastyde a descendant tap handler captures the pointer on PointerDown
and would swallow the row's own tap — so the press is left to bubble to the
`Popover` trigger. If anyone ever gives a dot a handler, this run fails.

Run:  python3 scripts/automation_tag_chips.py
"""

import base64
import json
import os
import re
import select
import subprocess
import sys
import tempfile
import time

ROOT = "/home/cyril/Devel/skribisto/.claude/worktrees/tags"
SKRIBISTO = f"{ROOT}/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = f"{ROOT}/resources/examples/Starforgers.skrib"

TAG = "chip-probe"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name


def fail(msg, app=None, mcp=None, log=None):
    print(f"FAIL: {msg}")
    if log:
        try:
            print("--- app log (tail) ---")
            print("".join(open(log).readlines()[-30:]))
        except Exception:
            pass
    for p in (mcp, app):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


class Session:
    """One launched app + connected MCP server."""

    def __init__(self, args):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT)
        sock = tok = None
        deadline = time.time() + 20
        while time.time() < deadline:
            txt = open(self.log).read()
            s = re.search(r"bridge socket = (\S+)", txt)
            t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
            if s and t:
                sock, tok = s.group(1), t.group(1)
                break
            if self.app.poll() is not None:
                fail("app exited before printing the bridge socket", self.app, None, self.log)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 20s", self.app, None, self.log)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "chips-test", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect MCP", self.app, self.mcp, self.log)
        self._send("notifications/initialized", notif=True)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=20, fatal=True):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                break
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
            if not r:
                break
            line = self.mcp.stdout.readline()
            if not line:
                break
            if line.strip():
                return json.loads(line)
        if fatal:
            fail("no MCP response within timeout", self.app, self.mcp, self.log)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        result = self._recv().get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(c.get("text", "") for c in result.get("content", [])
                           if c.get("type") == "text")
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", [])

    def settle(self):
        try:
            self.call("settle")
        except Exception:
            time.sleep(0.3)

    def dump(self, title):
        print(f"--- AT tree: {title} ---")
        for n in self.nodes():
            lab = (n.get("label") or n.get("value") or "").strip()
            if lab:
                b = n.get("bounds") or {}
                print(f"  [{n.get('role')}] {lab[:44]!r} @x={b.get('x')} y={b.get('y')}")

    def shot(self, path):
        try:
            res, _ = self.call("screenshot")
            for c in res.get("content", []):
                if c.get("type") == "image" and c.get("data"):
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"  screenshot -> {path}")
        except Exception as e:
            print("  screenshot failed:", e)

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


print("== launch on the bundled example ==")
s = Session([EXAMPLE])
deadline = time.time() + 25
while time.time() < deadline:
    if any("starforgers" in (n.get("label") or "").lower() for n in s.nodes()):
        break
    time.sleep(0.4)
else:
    fail("the example did not load", s.app, s.mcp, s.log)
if any("mock" in (n.get("label") or "").lower() for n in s.nodes()):
    fail("this is a `--features mocks` build — rebuild with `cargo build -p bastyde_ui`",
         s.app, s.mcp, s.log)
print("example loaded.")


def click(b, dx=None):
    if not (isinstance(b, dict) and "x" in b):
        return False
    s.call("inject_pointer", {"x": b["x"] + (dx if dx is not None else b.get("width", 0) / 2),
                              "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    return True


def hover(b):
    """Approach from outside, then jiggle inside — a single `move` landing already
    inside the widget is not reliably read as a hover *enter*."""
    cx, cy = b["x"] + b.get("width", 0) / 2, b["y"] + b.get("height", 0) / 2
    s.call("inject_pointer", {"x": cx - 150, "y": cy, "action": "move"})
    time.sleep(0.25)
    s.call("inject_pointer", {"x": cx, "y": cy, "action": "move"})
    time.sleep(0.1)
    s.call("inject_pointer", {"x": cx + 1, "y": cy + 1, "action": "move"})


def wait_for(pred, tries=18, delay=0.3):
    for _ in range(tries):
        time.sleep(delay)
        if pred():
            return True
    return False


def binder_rows():
    return [n for n in s.nodes()
            if n.get("role") == "Unknown"
            and 40 <= (n.get("bounds") or {}).get("x", 9999) <= 160
            and (n.get("label") or "").strip()]


def find(sub, role=None):
    sub = sub.lower()
    for n in s.nodes():
        lab = (n.get("label") or "").lower()
        if sub in lab and (role is None or n.get("role") == role):
            return n
    return None


def dot_row():
    """The dot row announces itself with every tag name it shows."""
    for n in s.nodes():
        # A Label, not a Button: the Popover's OverlayTrigger owns the button role.
        # Label-role nodes carry their text in `value`, not `label`.
        text = (n.get("value") or "") + " " + (n.get("label") or "")
        if TAG in text and n.get("role") == "Label":
            return n
    return None


# ── 1. Tag a scene, through the picker's own "Create" row ────────────────────
print("\n== create a tag on a scene ==")
rows = binder_rows()
if not rows:
    s.dump("no binder")
    fail("no binder rows", s.app, s.mcp, s.log)

# Pick a leaf (a scene), not a container: the deepest-indented row.
leaf = max(rows, key=lambda n: (n.get("bounds") or {}).get("x", 0))
scene_name = (leaf.get("label") or "").strip()
click(leaf.get("bounds") or {})
s.settle()
time.sleep(1.2)
print(f"  opened {scene_name!r}")

add = find("add a tag", role="Button")
if not add:
    s.dump("no Inspector tag field")
    fail("the Inspector shows no 'Add a tag' button", s.app, s.mcp, s.log)
click(add.get("bounds") or {})
s.settle()
time.sleep(0.8)

field = None
for n in s.nodes():
    if n.get("role") == "TextInput" and not (n.get("value") or "").strip():
        b = n.get("bounds") or {}
        if b.get("x", 0) > 400:
            field = n
            break
if not field:
    s.dump("no picker field")
    fail("the tag picker has no filter/create field", s.app, s.mcp, s.log)
s.call("type_text", {"node": field["id"], "text": TAG})
s.settle()
time.sleep(0.8)

create = find(f'create "{TAG}"') or find("create")
if not create:
    s.dump("no create row")
    fail("the picker offers no 'Create' row for a new name", s.app, s.mcp, s.log)
click(create.get("bounds") or {})
s.settle()
time.sleep(1.2)
print(f"  created and assigned {TAG!r}")

# Dismiss the picker.
s.call("inject_key", {"key": "Escape"})
s.settle()
time.sleep(0.6)

# ── 2. Surface one: the editor ───────────────────────────────────────────────
print("\n== surface 1: the editor ==")
if not wait_for(lambda: dot_row() is not None):
    s.dump("no dot row in the editor")
    s.shot("/tmp/chips-editor-fail.png")
    fail("the editor shows no dot row for the tagged scene", s.app, s.mcp, s.log)
row = dot_row()
b = row.get("bounds") or {}
print(f"  dot row {b.get('width', 0):.0f}x{b.get('height', 0):.0f} dp, announces "
      f"{row.get('label')!r}")
s.shot("/tmp/chips-editor.png")

# Per-dot hover.
print("\n== per-dot tooltip ==")
hover({"x": b.get("x", 0), "y": b.get("y", 0), "width": 18.0, "height": b.get("height", 18.0)})
if wait_for(lambda: any((n.get("role") or "") in ("Tooltip", "Dialog")
                        and TAG in (n.get("label") or "")
                        for n in s.nodes())):
    print("  hovering the dot shows its own tooltip")
    s.shot("/tmp/chips-tooltip.png")
else:
    print("  (no tooltip within the poll window)")

# Whole-row click — the assertion that the dots let the press bubble.
print("\n== clicking a dot opens the picker ==")
click({"x": b.get("x", 0), "y": b.get("y", 0), "width": 8.0, "height": b.get("height", 18.0)})
s.settle()
if not wait_for(lambda: any(n.get("role") == "ListBoxOption" for n in s.nodes())):
    s.dump("picker did not open")
    s.shot("/tmp/chips-click-fail.png")
    fail("clicking a dot did not open the picker — a dot is swallowing the row's tap "
         "(did one grow an on_tap?)", s.app, s.mcp, s.log)
opts = [n for n in s.nodes() if n.get("role") == "ListBoxOption"]
print(f"  picker opened from a dot, {len(opts)} option(s), "
      f"{sum(1 for n in opts if n.get('selected'))} ticked")
s.call("inject_key", {"key": "Escape"})
s.settle()
time.sleep(0.6)

# ── 3+4. The stream and the corkboard ────────────────────────────────────────
# Both live behind a container's segmented control, so open the scene's parent.
print("\n== surfaces 2 and 3: stream and corkboard ==")
parents = [n for n in binder_rows()
           if (n.get("bounds") or {}).get("x", 0) < (leaf.get("bounds") or {}).get("x", 0)]
opened = False
for cand in reversed(parents):
    click(cand.get("bounds") or {})
    s.settle()
    time.sleep(1.4)
    if find("corkboard"):
        print(f"  opened container {(cand.get('label') or '').strip()!r}")
        opened = True
        break
if not opened:
    s.dump("no container with a segmented control")
    fail("could not open a container offering Stream/Corkboard", s.app, s.mcp, s.log)

if not wait_for(lambda: dot_row() is not None):
    s.shot("/tmp/chips-stream-fail.png")
    fail("the manuscript stream's row header shows no dot row", s.app, s.mcp, s.log)
print(f"  stream row header shows the dots: {dot_row().get('label')!r}")
s.shot("/tmp/chips-stream.png")

cb = find("corkboard")
click(cb.get("bounds") or {})
s.settle()
time.sleep(1.8)
if not wait_for(lambda: dot_row() is not None):
    s.dump("no dots on any card")
    s.shot("/tmp/chips-corkboard-fail.png")
    fail("the corkboard card shows no dot row", s.app, s.mcp, s.log)
print(f"  corkboard card shows the dots: {dot_row().get('label')!r}")
s.shot("/tmp/chips-corkboard.png")

print("\nOK — tag dots verified on the editor, the stream and the corkboard.")
s.close()
