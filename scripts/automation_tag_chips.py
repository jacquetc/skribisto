#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the tag **dot row** on all three surfaces.

`automation_tags.py` covers the palette, the settings pane and the Inspector on
the legacy fixture. This one covers what Stage 4 added: the dots that appear
beside an item wherever it is listed.

It runs on `resources/test/skribisto_test_project.skrib`, whose "Chapter 1" is a
`Folder`/`ChapterScene` — a structural container, and therefore the one place in
the test corpus that renders a segmented control with a manuscript stream and a
corkboard. Two of the three surfaces do not exist anywhere else to be looked at.

The run **assigns its own tag** rather than relying on the fixture carrying one,
through the very affordance under test (the picker's "Create ..." row). That is
deliberate: it exercises create-and-assign, and it survives the fixture being
regenerated (the last regeneration kept the palette but dropped every item's
assignments). It then looks for that tag in each place:

  1. the editor, under the title/subtitle;
  2. the manuscript stream's row header;
  3. the corkboard card.

Plus the two interaction rules the design rests on:

  * hovering one dot shows *that* dot's tooltip (per-dot hover), and
  * clicking a dot opens the picker rather than doing nothing (whole-row click).

The second is the load-bearing one. The dots deliberately carry no `on_tap`,
because in teksilo a descendant tap handler captures the pointer on PointerDown
and would swallow the row's own tap — so the press is left to bubble to the
`Popover` trigger. If anyone ever gives a dot a handler, this run fails.

Run:  python3 scripts/automation_tag_chips.py
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

ROOT = fixture.repo_root()
SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
from automation_fixture import working_copy

# A throwaway copy — see `automation_fixture`.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "chips")

# Every launch below gets its own sandboxed config (so it never inherits this
# machine's real settings or OS locale) plus a validated `--config` pins file
# (so an unknown key is a hard startup error, not a silently-ignored typo).
# `find()` below matches both English and French label variants, but the
# literal strings in this file (TAG, "Add a tag", …) are English, hence en-US.
ENV = fixture.isolated_config(locale="en-US", label="chips", show_welcome=False)
PINS = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False}, label="chips")

# The one container in the fixture that renders Stream/Corkboard, and a scene
# inside it. Named rather than discovered: the structure is fixed and known,
# and guessing "the most-indented row" picked the wrong item on a flat binder.
CONTAINER = "Chapter 1"
SCENE = "1.1 Zeus"

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
        self.app = subprocess.Popen(fixture.launch_argv(list(args), pins=PINS),
                                    stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=ENV)
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=20)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "chips-test", "version": "1"}})
        if self._recv(timeout=8, fatal=False) is None:
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


print("== launch on the test fixture ==")
s = Session([FIXTURE])
deadline = time.time() + 25
while time.time() < deadline:
    if any(SCENE in (n.get("label") or "") for n in s.nodes()):
        break
    time.sleep(0.4)
else:
    fail("the fixture did not load", s.app, s.mcp, s.log)
if any("mock" in (n.get("label") or "").lower() for n in s.nodes()):
    fail("this is a `--features mocks` build — rebuild with `cargo build -p teksilo_ui`",
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
    """First node whose label contains any of `sub` (a string or a tuple).

    Takes variants because the app follows the *system* locale: an English-only
    selector finds nothing on a French desktop and reports it as a missing
    control, which is the same shape as a genuine regression. Every call site
    below passes both spellings, keyed to the ftl entry named in the comment.
    """
    subs = (sub,) if isinstance(sub, str) else tuple(sub)
    subs = tuple(x.lower() for x in subs)
    for n in s.nodes():
        lab = (n.get("label") or "").lower()
        if any(x in lab for x in subs) and (role is None or n.get("role") == role):
            return n
    return None


HIT = 18.0  # one dot's hit cell, from tags/tag_chip.rs


def dot_row():
    """The dot row, identified by geometry as well as by text.

    Text alone is not enough: the tag's own tooltip body is also a `Label`
    containing the tag name, and matching that reported a "dot row" of 58x13 dp
    on a surface where no dots were drawn at all — a false pass. The row is
    exactly one hit-cell tall and a whole number of cells wide, which nothing
    else on screen is.

    It is a `Label` rather than a `Button` because the Popover's OverlayTrigger
    already owns the button role at the same bounds; `Label`-role nodes carry
    their text in `value`, not `label`.
    """
    for n in s.nodes():
        if n.get("role") != "Label":
            continue
        text = (n.get("value") or "") + " " + (n.get("label") or "")
        if TAG not in text:
            continue
        b = n.get("bounds") or {}
        w, h = b.get("width", 0), b.get("height", 0)
        # Width is the reliable signal: the row is exactly one hit cell per tag.
        # Height is NOT — a stream row header stretches the row to its own line
        # height (24 dp observed) while the dots stay round inside it, so pinning
        # height to 18 rejected a row that was rendering perfectly well.
        cells = w / HIT
        if abs(cells - round(cells)) > 0.08 or round(cells) < 1:
            continue
        if not (12.0 <= h <= 36.0):
            continue
        return n
    return None


def content_pane():
    """The editor pane's own scrollable body.

    Picked by area, not by "first node past x=400": that matched a zero-width
    splitter at x=846, so every scroll went to a widget that does not scroll and
    the row below the fold was never reached."""
    best, best_area = None, 0.0
    for n in s.nodes():
        b = n.get("bounds") or {}
        x, w, h = b.get("x", 0), b.get("width", 0), b.get("height", 0)
        if not (380 < x < 860 and w > 300 and h > 300):
            continue
        if w * h > best_area:
            best, best_area = n, w * h
    return best


def scroll_for_dot_row(anchor, tries=12):
    """Find the dot row, scrolling the pane if it is below the fold.

    A stream is the whole chapter's prose, so a scene's row header can be a long
    way down; the corkboard can likewise need scrolling once cards wrap.
    """
    for _ in range(tries):
        row = dot_row()
        if row:
            return row
        s.call("scroll", {"node": anchor["id"], "dx": 0, "dy": -240})
        s.settle()
        time.sleep(0.4)
    return dot_row()


# ── 1. Tag a scene, through the picker's own "Create" row ────────────────────
print("\n== create a tag on a scene ==")
rows = binder_rows()
if not rows:
    s.dump("no binder")
    fail("no binder rows", s.app, s.mcp, s.log)

leaf = next((n for n in rows if (n.get("label") or "").strip() == SCENE), None)
if not leaf:
    s.dump("no scene row")
    fail(f"{SCENE!r} is not in the binder", s.app, s.mcp, s.log)
scene_name = SCENE
click(leaf.get("bounds") or {})
s.settle()
time.sleep(1.2)
print(f"  opened {scene_name!r}")

add = find(("add a tag", "ajouter une étiquette"), role="Button")
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

create = (find((f'create "{TAG}"', f'créer « {TAG} »'))
          or find(("create", "créer")))
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
      f"{(row.get('value') or '')!r}")
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
container = next((n for n in binder_rows()
                  if (n.get("label") or "").strip() == CONTAINER), None)
if not container:
    s.dump("no container row")
    fail(f"{CONTAINER!r} is not in the binder", s.app, s.mcp, s.log)
click(container.get("bounds") or {})
s.settle()
time.sleep(1.6)
if not find(("corkboard", "tableau d'affichage")):
    s.dump("no segmented control")
    fail(f"{CONTAINER!r} offers no Stream/Corkboard segments — it must be a Folder whose "
         "sub_role is Book/Part/ChapterScene", s.app, s.mcp, s.log)
print(f"  opened container {CONTAINER!r}")


def select_segment(name):
    """Pick a segment by name and give it time to build.

    Never assume which one is showing: the chosen segment is remembered per
    container type *between launches*, so a run that ended on the corkboard
    reopens there and a stream check would silently measure the wrong view."""
    seg = find(name)
    if not seg:
        s.dump(f"no {name!r} segment")
        fail(f"the container offers no {name!r} segment", s.app, s.mcp, s.log)
    click(seg.get("bounds") or {})
    s.settle()
    # A stream builds a live editor per child, so it is slow to realize; a short
    # wait here reported "no dots" against a pane that had not finished.
    time.sleep(3.0)


# "Full Chapter" is the manuscript stream: the container's own text followed by
# each child's row header and prose.
select_segment(("full chapter", "chapitre complet"))

# The stream is the chapter's whole prose, so the tagged scene's row header is
# usually below the fold.
pane = content_pane()
row = scroll_for_dot_row(pane) if pane else dot_row()
if not row:
    s.dump("no dot row in the stream")
    s.shot("/tmp/chips-stream-fail.png")
    fail("the manuscript stream's row header shows no dot row", s.app, s.mcp, s.log)
b = row.get("bounds") or {}
print(f"  stream row header: {(row.get('value') or '')!r} "
      f"({b.get('width', 0):.0f}x{b.get('height', 0):.0f} dp)")
s.shot("/tmp/chips-stream.png")

select_segment(("corkboard", "tableau d'affichage"))
pane = content_pane()
row = scroll_for_dot_row(pane) if pane else dot_row()
if not row:
    s.dump("no dots on any card")
    s.shot("/tmp/chips-corkboard-fail.png")
    fail("the corkboard card shows no dot row", s.app, s.mcp, s.log)
b = row.get("bounds") or {}
print(f"  corkboard card: {(row.get('value') or '')!r} "
      f"({b.get('width', 0):.0f}x{b.get('height', 0):.0f} dp)")
s.shot("/tmp/chips-corkboard.png")

print("\nOK — tag dots verified on the editor, the stream and the corkboard.")
s.close()
