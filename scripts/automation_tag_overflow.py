#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the tag dot row's "+N" overflow cell.

`tag_chip.rs` caps how many dots a row paints before the rest collapse into one
overflow cell: 4 on the corkboard, 5 on the stream, 8 in the editor. The
fixture's tagged item ("1.1 Zeus") only ever carries 3-4 tags, so the
collapse, the "+N" cell, and the tooltip naming what it hides have never been
rendered by a real app — a `#[cfg(test)]` unit
(`the_overflow_cell_names_exactly_the_hidden_tags`) covers the arithmetic but
cannot reach a real tooltip on a real hover, which is what this script does.

This probe **creates its own tags**, through the "+" popover's own
create-and-assign row, to push Zeus to six: `Chip-Extra-1/2/3` beside the
fixture's `A`, `B` and `very looooooooooong tag`. The names are chosen to sort
(case-insensitively — `sort_rows` is a plain `.to_lowercase()` compare) between
`B` and `very looooooooooong tag`, so the final palette order is fixed and
known ahead of the run:

    A, B, Chip-Extra-1, Chip-Extra-2, Chip-Extra-3, very looooooooooong tag

Six tags is deliberately over BOTH the corkboard cap (4) and the stream cap
(5), and under the editor cap (8) — one item, three different outcomes,
proving the cap is read per surface rather than off one shared constant.

Asserts, precisely (no "is something shown", always "is exactly this shown"):

  1. baseline — Zeus starts with exactly the fixture's three tags;
  2. after creating three more, Zeus carries exactly six, by name;
  3. the EDITOR row (cap 8) paints all 6 dots, its accessible name is the full
     6-name join, and NO "+N" cell exists anywhere inside its own bounds —
     this negative check is proven capable of firing by the identical positive
     checks in 4 and 5 below (the same cell-finder function, not a different
     one written to always pass);
  4. the CORKBOARD row (cap 4) is exactly 5 hit-cells wide (4 dots + 1
     overflow), the "+2" cell exists strictly inside the row's own box, and
     hovering it opens a tooltip whose body names EXACTLY `Chip-Extra-3` and
     `very looooooooooong tag` — not all 6, not just 1;
  5. the STREAM row (cap 5) is exactly 6 hit-cells wide (5 dots + 1 overflow),
     the "+1" cell is found the same way, and its tooltip names EXACTLY
     `very looooooooooong tag` alone (no comma — proves one hidden tag, not
     more).

The row is located by width, in multiples of the 18dp hit cell (`HIT` in
`tag_chip.rs`) — not height, since a stream row header stretches to its own
line height while the dots inside it stay round, so pinning height rejects a
row that is rendering correctly.

A second structural fact this script leans on: `TagChipRow`'s accessible name
(`row_label()`) joins *every* assigned tag, not just the ones painted as
dots — which is what lets one locator (a `Label` node containing a marker tag
name) work identically on all three surfaces, and is itself worth asserting
once (a screen reader always hears the complete set, cap or no cap).

Run:  python3 scripts/automation_tag_overflow.py
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

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

ROOT = fixture.repo_root()
MCP = fixture.mcp_binary()
from automation_fixture import wait_for_load, working_copy

# This probe creates tags and saves nothing explicitly, but the app autosaves
# on its own timer — never the checked-in fixture.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "overflow")

# A private config dir, pinned to English (this probe's own selector tuples
# tolerate French too, but the item/container titles it plants — "1.1 Zeus",
# "Chapter 1" — are fixture DATA, never translated, so the interface language
# doesn't matter to what is being proven; English keeps the printed labels
# readable). Mirrored into a `--config` pins file so the app validates the
# keys rather than silently ignoring a typo'd one.
PROBE_ENV = fixture.isolated_config(locale="en-US", label="overflow", show_welcome=False)
PINS = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="overflow")

# ── Locale-safe strings, each keyed to its ftl entry ─────────────────────────
# The app follows the *system* locale (French on this machine). An
# English-only selector finds nothing on a French desktop and reports it as a
# missing control, which is indistinguishable from a real regression.
ADD_A_TAG = ("add a tag", "ajouter une étiquette")           # tags-pill-add
TAGS_PILL_LIST = ("tags", "étiquettes")                       # tags-pill-list (exact match only)
CORKBOARD_SEG = ("corkboard", "tableau d'affichage")          # `corkboard`, main.ftl —
# NOT `settings-page-corkboard` ("Tableau de liège"): same English spelling,
# different French string, wrong control entirely.
STREAM_SEG = ("full chapter", "chapitre complet")             # `full-chapter`, main.ftl
# tags-chip-more, both plural forms this run needs (n=1 and n=2).
TAGS_CHIP_MORE = {
    1: ("1 more tag", "1 étiquette de plus"),
    2: ("2 more tags", "2 étiquettes de plus"),
}

ITEM = "1.1 Zeus"
CONTAINER = "Chapter 1"

# The fixture's original three (case-folded, for set comparison).
BASE_TAGS = {"a", "b", "very looooooooooong tag"}

# Sorts (case-insensitively) between "B" and "very looooooooooong tag" — chosen
# so the final palette order is fixed and known before the run, not discovered
# by reading it back. Verified against `sort_rows` in work_tags_list_model.rs,
# a plain `.to_lowercase()` compare with no secondary key that would reorder these.
NEW_TAGS = ["Chip-Extra-1", "Chip-Extra-2", "Chip-Extra-3"]

# The full palette on Zeus, in the order `sort_rows` produces and therefore the
# order `TagChipRow::row_label()` announces on every surface.
ALL_TAGS_SORTED = ["A", "B", "Chip-Extra-1", "Chip-Extra-2", "Chip-Extra-3",
                   "very looooooooooong tag"]
FULL_LABEL = ", ".join(ALL_TAGS_SORTED)

# `split_at_cap` (tag_chip.rs) hides the TAIL of the sorted list, never the
# head and never resorted — asserted directly by the Rust unit test this probe
# is the live counterpart of.
HIDDEN_CORKBOARD = ["Chip-Extra-3", "very looooooooooong tag"]   # cap 4: shown A,B,C1,C2
HIDDEN_STREAM = ["very looooooooooong tag"]                       # cap 5: shown A,B,C1,C2,C3

HIT = 18.0  # one dot's hit cell, from tags/tag_chip.rs — the reliable width unit.

# Marker used to locate the dot row on any surface: unique to this run (no
# fixture data or app chrome contains it), and present in the row's full
# accessible name regardless of which surface's cap hid it from view.
TAG_MARKER = "Chip-Extra-1"

OVERFLOW_RE = re.compile(r"^\+\d+$")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name


def fail(msg, sess=None):
    print(f"FAIL: {msg}")
    if sess:
        try:
            print("--- app log (tail) ---")
            print("".join(open(sess.log).readlines()[-30:]))
        except Exception:
            pass
        sess.stop()
    sys.exit(1)


class Session:
    """One launched app + connected MCP server, restartable."""

    def __init__(self, path):
        # Set before anything that can fail, so `fail()`'s teardown never trips
        # over a half-built Session and hides the real error.
        self.mcp = None
        self.app = None
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(fixture.launch_argv(path, pins=PINS),
                                    stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=PROBE_ENV)
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=40)
        except RuntimeError as e:
            fail(str(e), self)
        self.bridge = bridge
        self._id = 0
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "overflow", "version": "1"}})
        if self._recv(timeout=8) is None:
            fail("MCP did not initialize", self)
        self._send("notifications/initialized", notif=True)
        time.sleep(3)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                return None
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
            if not r:
                return None
            line = self.mcp.stdout.readline()
            if not line:
                return None
            if line.strip():
                return json.loads(line)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        res = (self._recv() or {}).get("result", {})
        p = res.get("structuredContent")
        if p is None:
            txt = "".join(c.get("text", "") for c in res.get("content", [])
                          if c.get("type") == "text")
            p = json.loads(txt) if txt.strip().startswith("{") else {}
        return res, p

    def nodes(self):
        return self.call("snapshot_tree")[1].get("nodes", [])

    def settle(self):
        self.call("settle")

    def dump(self, title):
        print(f"--- AT tree: {title} ---")
        for n in self.nodes():
            lab = (n.get("value") or n.get("label") or "").strip()
            if lab:
                b = n.get("bounds") or {}
                print(f"  [{n.get('role')}] {lab[:60]!r} @x={b.get('x')} y={b.get('y')} "
                      f"w={b.get('width')} h={b.get('height')}")

    def shot(self, path):
        res, _ = self.call("screenshot", {})
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")

    def stop(self):
        """Terminate and *wait*.

        Relaunching on the same path while the old process still holds its
        open-registry lock hands the new launch off to it and exits it
        immediately — surfacing as "app exited before printing the bridge
        socket", not as the timeout it actually is.
        """
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        for p in (self.mcp, self.app):
            if p:
                try:
                    p.wait(timeout=10)
                except Exception:
                    p.kill()
        # The lock is released on exit, but the registry file is touched by the
        # dying process; give it a beat before the next launch claims it.
        time.sleep(2.0)


def text_of(n):
    """A node's visible text: Label-role nodes carry theirs in `value`."""
    return ((n.get("value") or "") + " " + (n.get("label") or "")).strip()


def find(s, variants, role=None):
    vs = tuple(v.lower() for v in variants) if not isinstance(variants, str) else (variants.lower(),)
    for n in s.nodes():
        if role and n.get("role") != role:
            continue
        if any(v in text_of(n).lower() for v in vs):
            return n
    return None


def click(s, b, dx=None):
    if not (isinstance(b, dict) and "x" in b):
        return False
    s.call("inject_pointer", {"x": b["x"] + (dx if dx is not None else b.get("width", 0) / 2),
                              "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    return True


def hover(b):
    """Approach from outside, then jiggle inside — a single `move` landing
    already inside the widget is not reliably read as a hover *enter* (the
    dwell timer arms on motion within the widget, and one event that arrives
    already-inside can leave it unarmed)."""
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


def open_item(title):
    """Select `title` in the binder so the Inspector and editor target it."""
    for n in s.nodes():
        b = n.get("bounds") or {}
        if n.get("role") == "Unknown" and (n.get("label") or "").strip() == title \
                and 40 <= b.get("x", 9999) <= 160:
            click(s, b)
            s.settle()
            time.sleep(1.2)
            return True
    return False


def binder_rows():
    return [n for n in s.nodes()
            if n.get("role") == "Unknown"
            and 40 <= (n.get("bounds") or {}).get("x", 9999) <= 160
            and (n.get("label") or "").strip()]


def dot_row():
    """The dot row, identified by geometry as well as by text.

    Text alone is not enough: a tag's own per-dot tooltip body is also a
    `Label` containing the tag name, and the overflow cell's tooltip body
    contains it too once hovered. The row is exactly one hit-cell tall and a
    whole number of cells wide, which nothing else sharing the marker text is.

    A `Label`, not a `Button`: the Popover's `OverlayTrigger` already owns the
    button role at the same bounds (tag_chip.rs's own comment on
    `TagChipRow::accessibility`), so `Label`-role nodes carry their text in
    `value`, not `label`.
    """
    for n in s.nodes():
        if n.get("role") != "Label":
            continue
        if TAG_MARKER not in ((n.get("value") or "") + " " + (n.get("label") or "")):
            continue
        b = n.get("bounds") or {}
        w, h = b.get("width", 0), b.get("height", 0)
        cells = w / HIT
        # Width is the reliable signal — a whole number of 18dp hit cells.
        # Height is NOT: a stream row header stretches to its own line height
        # (24dp observed) while the dots stay round inside it, so pinning
        # height would reject a row that renders correctly.
        if abs(cells - round(cells)) > 0.08 or round(cells) < 1:
            continue
        if not (12.0 <= h <= 36.0):
            continue
        return n
    return None


def content_pane():
    """The editor/corkboard/stream pane's own scrollable body.

    Picked by area, not by "first node past x=400": on this window that once
    matched a zero-width splitter, so every scroll went to a widget that does
    not scroll and a row below the fold was never reached.
    """
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

    A stream is the whole chapter's prose, so a scene's row header can be a
    long way down; the corkboard can likewise need scrolling once cards wrap.
    """
    for _ in range(tries):
        row = dot_row()
        if row:
            return row
        if anchor:
            s.call("scroll", {"node": anchor["id"], "dx": 0, "dy": -240})
        s.settle()
        time.sleep(0.4)
    return dot_row()


def select_segment(name):
    """Pick a segment by name and give it time to build.

    Never assume which one is showing: the chosen segment is remembered per
    container type *between launches*, so this always clicks rather than
    checking first — a run that happened to start on the target segment is
    harmless to re-click, but skipping the click on a wrong guess is not.
    """
    for attempt in range(3):
        seg = find(s, name)
        if not seg:
            s.dump(f"no {name!r} segment")
            fail(f"the container offers no {name!r} segment "
                 f"({CONTAINER!r} must be a Folder/ChapterScene)", s)
        click(s, seg.get("bounds") or {})
        s.settle()
        # A stream builds a live editor per child and the corkboard lays out
        # cards; a short wait here reported "no dots" against a pane that had not
        # finished building.
        time.sleep(3.0)
        if segment_selected(name):
            return
        print(f"    (segment {name[0]!r} not selected after attempt {attempt + 1}, retrying)")

    # Verifying matters: a silently-failed click leaves the pane showing
    # whatever segment a previous run remembered, and measuring that pane's
    # dot row would misreport a wrong-pane failure as a cap regression.
    s.dump(f"segment {name!r} would not select")
    s.shot("/tmp/overflow-segment-fail.png")
    fail(f"could not select the {name[0]!r} segment after 3 attempts; segment "
         f"states now: {segment_states()}", s)


def segment_states():
    """Every node whose text matches one of the known segment names, with
    whatever selection flag it carries — for diagnostics when a click is lost."""
    known = [SEG for SEG in (CORKBOARD_SEG, STREAM_SEG) if SEG]
    out = []
    for n in s.nodes():
        t = text_of(n).strip()
        low = t.lower()
        if any(any(v in low for v in variants) for variants in known):
            out.append((t, n.get("role"), n.get("selected"), n.get("toggled")))
    return out


def segment_selected(name):
    """True when the segment named `name` reports itself selected.

    A `SegmentedControl` marks the active segment with the AT `selected` flag;
    some roles use `toggled` instead, so accept either rather than pinning one
    and silently never matching.
    """
    for n in s.nodes():
        low = text_of(n).strip().lower()
        if any(v in low for v in name):
            if n.get("selected") is True or n.get("toggled") is True:
                return True
    return False


def tag_pill_names():
    """The tag names on the currently-open item, read from the Inspector's
    Tags List — from the pills' `Remove …` controls, not the pill itself.

    `tags-pill-remove` is word-for-word identical to `tags-alias-remove`, so
    an unscoped search for "Remove …" also pulls in the Aliases section's
    pills; anchored on the `tags-pill-list` `List` node's own bounding box.
    """
    lst = next((n for n in s.nodes()
                if n.get("role") == "List"
                and text_of(n).strip().lower() in TAGS_PILL_LIST
                and (n.get("bounds") or {}).get("x", 0) > 400), None)
    if not lst:
        return None
    lb = lst.get("bounds") or {}
    top, bot = lb.get("y", 0) - 4, lb.get("y", 0) + lb.get("height", 0) + 4
    out = []
    for n in s.nodes():
        b = n.get("bounds") or {}
        t = text_of(n).strip()
        low = t.lower()
        if not (low.startswith("remove ") or low.startswith("retirer ")):
            continue
        if b.get("x", 0) >= lb.get("x", 0) - 20 and top <= b.get("y", -999) <= bot:
            out.append(t.split(" ", 1)[1].strip())
    return out


def picker_field_now():
    """The picker's filter field as it exists *right now*, or None.

    Always re-locate; never cache the id. The `query` signal is bound at
    `BindingLevel::Rebuild`, so every keystroke replaces the whole `TagPicker`
    subtree and the previous id refers to a destroyed node.
    """
    dlg = next((n for n in s.nodes()
                if n.get("role") == "Dialog"
                and any(v in (n.get("label") or "").lower() for v in ADD_A_TAG)), None)
    if not dlg:
        return None
    db = dlg.get("bounds") or {}
    x0, y0 = db.get("x", 0), db.get("y", 0)
    x1, y1 = x0 + db.get("width", 0), y0 + db.get("height", 0) + 400
    for n in s.nodes():
        if n.get("role") != "TextInput":
            continue
        b = n.get("bounds") or {}
        if x0 - 8 <= b.get("x", -1) <= x1 and y0 - 8 <= b.get("y", -1) <= y1:
            return n
    return None


def create_tag(name):
    """Type `name` into the "+" popover's filter field and click its
    "Create …" row.

    The popover does NOT close after a create — `vm.create()` both makes the
    palette row and appends its id to the item's tags in the same click, and
    only the query text resets. Each call re-locates the field and the create
    row fresh, because the whole `TagPicker` subtree rebuilds and the
    previous call's widget ids are no longer valid.
    """
    # Locate the filter field INSIDE the picker's own Dialog box: "any
    # TextInput at x>400" also matches other Inspector fields. The popover is
    # a `Dialog` whose label is the add button's own name, so scoping to its
    # bounds is unambiguous.
    field = None
    for _ in range(15):
        field = picker_field_now()
        if field:
            break
        time.sleep(0.3)
    if not field:
        s.dump(f"no picker field for {name!r}")
        fail(f"the tag picker Dialog has no filter field to type {name!r} into", s)

    stale = (field.get("value") or "").strip()
    if stale:
        s.call("invoke_action", {"node": field["id"], "action": "focus"})
        time.sleep(0.2)
        for _ in range(len(stale) + 4):
            s.call("inject_key", {"key": "Backspace"})
        s.settle()
        time.sleep(0.4)

    # Focus first, then type: without it, `type_text` sets the node's
    # AccessKit value but the text never reaches the widget's signal, so the
    # field renders empty and the "Create" row never appears.
    s.call("invoke_action", {"node": field["id"], "action": "focus"})
    time.sleep(0.3)
    s.call("type_text", {"node": field["id"], "text": name})
    s.settle()
    time.sleep(0.8)

    create = (find(s, (f'create "{name}"', f'créer « {name} »'))
              or find(s, ("create", "créer")))
    if not create:
        # Distinguish "the text never landed" from "the row is named something
        # else" -- these need opposite fixes. Re-LOCATE the field rather than
        # re-reading `field["id"]`: typing rebuilds the whole `TagPicker`
        # subtree, so the pre-typing id no longer exists.
        now = picker_field_now()
        print(f"    field now holds {(now or {}).get('value')!r} (typed {name!r})")
        dlg = next((n for n in s.nodes()
                    if n.get("role") == "Dialog"
                    and any(v in (n.get("label") or "").lower() for v in ADD_A_TAG)), None)
        if dlg:
            db = dlg.get("bounds") or {}
            x0, y0 = db.get("x", 0), db.get("y", 0)
            print("    dialog rows:")
            for n in s.nodes():
                b = n.get("bounds") or {}
                if b and x0 - 8 <= b.get("x", -1) <= x0 + db.get("width", 0) + 8 \
                   and y0 - 8 <= b.get("y", -1) <= y0 + 500:
                    lab = ((n.get("value") or "") + " " + (n.get("label") or "")).strip()
                    if lab:
                        print(f"      {n.get('role')}: {lab[:50]!r}")
        else:
            print("    (the picker Dialog is not open at all)")
        s.dump(f"no create row for {name!r}")
        fail(f"the picker offers no 'Create' row for {name!r} "
             f"(typed into field id={field['id']})", s)
    # A synthesised pointer click, NOT `invoke_action(click)`: the Create row
    # is an `HStack` with `access_role(Role::Button)` but no AccessKit Click
    # handler (teksilo only wires one for stock widgets), so `invoke_action`
    # would report success while doing nothing. The near-miss risk that
    # normally argues for `invoke_action` doesn't apply: the row is the full
    # width of the popover, with no neighbouring tag rows to miss into.
    before = tag_pill_names() or []
    click(s, create.get("bounds") or {})
    s.settle()
    time.sleep(1.0)

    # Verify against the INSPECTOR's pills, not the picker's rows: the
    # popover closes on a create, so reading the picker's own list would
    # report failure on a create that worked perfectly. `vm.create()` is
    # silent on failure, so this must be checked, not assumed.
    for _ in range(12):
        now = tag_pill_names() or []
        if any(n.lower() == name.lower() for n in now):
            return
        time.sleep(0.3)
    s.dump(f"create had no effect for {name!r}")
    fail(f"clicking Create for {name!r} did not put it on the item "
         f"(pills before={sorted(before)}, after={sorted(tag_pill_names() or [])})", s)


def overflow_cells_in_box(row):
    """Every `Label` node whose `value` matches `+<digits>` and whose bounds
    fall strictly inside `row`'s own box.

    Scoped to the row's box rather than a whole-tree scan, which risks
    matching stale nodes from an unrelated panel. Used both as the negative
    check (the editor must return an empty list) and the positive one
    (corkboard/stream) — the same function, so the negative result is only
    trustworthy because the positive calls prove it can find something when
    something is really there.
    """
    rb = row.get("bounds") or {}
    rx, ry = rb.get("x", 0), rb.get("y", 0)
    rw, rh = rb.get("width", 0), rb.get("height", 0)
    out = []
    for n in s.nodes():
        if n.get("role") != "Label":
            continue
        v = (n.get("value") or "").strip()
        if not OVERFLOW_RE.match(v):
            continue
        b = n.get("bounds") or {}
        if rx <= b.get("x", -1) < rx + rw and ry <= b.get("y", -1) < ry + rh:
            out.append(n)
    return out


def assert_cell_count(surface, row, expected):
    """The row's width, in whole 18dp hit cells, must equal `expected` exactly.

    Width alone cannot distinguish stream (5 dots + overflow) from editor (6
    dots, no overflow) — both render 6 cells at 108dp. This check only proves
    the cell count is right; `assert_overflow` is what tells them apart.
    """
    b = row.get("bounds") or {}
    w = b.get("width", 0)
    got = round(w / HIT)
    print(f"  {surface}: row is {w:.0f} dp wide -> {got} cell(s) (expecting {expected})")
    if got != expected:
        s.shot(f"/tmp/overflow-{surface.lower()}-width-fail.png")
        fail(f"{surface}: expected exactly {expected} hit-cells "
             f"({expected * HIT:.0f} dp), got {got} ({w:.0f} dp) — the row is "
             f"{FULL_LABEL!r}", s)


def assert_row_label(surface, row, expected):
    """The row's accessible name lists EVERY assigned tag, not just the ones
    painted as dots (`TagChipRow::row_label()`, tag_chip.rs:103-109). Worth
    asserting on its own: a screen reader always hears the complete set, cap
    or no cap, which is not obvious from looking at the screen."""
    got = (row.get("value") or "").strip()
    print(f"  {surface}: row announces {got!r}")
    if got != expected:
        fail(f"{surface}: the row's accessible name should be the full "
             f"6-tag join regardless of the visible cap; expected "
             f"{expected!r}, got {got!r}", s)


def assert_overflow(surface, row, n, hidden_names, shot_prefix):
    """The "+n" cell exists inside `row`'s own box, its tooltip announces
    itself via `tags-chip-more`, and the tooltip's body names EXACTLY
    `hidden_names` — no more, no fewer. This is the live counterpart of
    `tag_chip.rs`'s `the_overflow_cell_names_exactly_...` unit test, which
    cannot reach a real tooltip on a real hover.
    """
    expect = f"+{n}"
    cells = overflow_cells_in_box(row)
    cell = next((c for c in cells if (c.get("value") or "").strip() == expect), None)
    if not cell:
        found = [(c.get("value") or "").strip() for c in cells] or "none"
        rb = row.get("bounds") or {}
        s.dump(f"{surface}: no {expect!r} cell")
        s.shot(f"/tmp/overflow-{shot_prefix}-no-cell.png")
        fail(f"{surface}: expected a {expect!r} overflow cell inside the row's "
             f"own box ({rb.get('width', 0):.0f}x{rb.get('height', 0):.0f} dp at "
             f"x={rb.get('x')},y={rb.get('y')}) — found instead: {found}", s)
    print(f"  {surface}: found the {expect!r} overflow cell")

    hover(cell.get("bounds") or {})
    variants = TAGS_CHIP_MORE[n]
    if not wait_for(lambda: any(
            (nd.get("role") or "") in ("Tooltip", "Dialog")
            and (nd.get("label") or "").strip().lower() in (v.lower() for v in variants)
            for nd in s.nodes())):
        s.dump(f"{surface}: overflow tooltip never opened")
        s.shot(f"/tmp/overflow-{shot_prefix}-tooltip-fail.png")
        fail(f"{surface}: hovering the {expect!r} cell showed no tooltip "
             f"(looked for {variants})", s)
    print(f"  {surface}: hovering {expect!r} announces '{variants[0]}'")

    expected_body = ", ".join(hidden_names)
    body = next((nd for nd in s.nodes()
                 if nd.get("role") == "Label"
                 and (nd.get("value") or "").strip() == expected_body), None)
    if not body:
        # Not "not found" alone — say what Label nodes DO mention a hidden
        # name, so a wrong-count bug (names the whole palette, or just one) is
        # distinguishable from the tooltip never having opened at all.
        candidates = [(nd.get("value") or "").strip() for nd in s.nodes()
                      if nd.get("role") == "Label"
                      and any(h in (nd.get("value") or "") for h in hidden_names)]
        s.shot(f"/tmp/overflow-{shot_prefix}-body-fail.png")
        fail(f"{surface}: the overflow tooltip does not name exactly "
             f"{hidden_names} (expected value {expected_body!r}; Label nodes "
             f"mentioning a hidden name held: {candidates})", s)
    print(f"  {surface}: tooltip names exactly {hidden_names}")
    s.shot(f"/tmp/overflow-{shot_prefix}.png")

    # Move away and dismiss: a dwelled composite tooltip is promoted to a
    # sticky `Dialog` that can take focus, which can swallow the next typed
    # keystroke if left open.
    s.call("inject_pointer", {"x": 40, "y": 400, "action": "move"})
    time.sleep(0.3)
    s.call("inject_key", {"key": "Escape"})
    s.settle()
    time.sleep(0.5)


# ── 1. Launch on the fixture, open the tagged item ───────────────────────────
print("== launch ==")
s = Session(FIXTURE)
if not wait_for_load(s.nodes, ITEM):
    fail(f"the fixture did not load ({ITEM!r} not in the binder)", s)
if any("mock" in (n.get("label") or "").lower() for n in s.nodes()):
    fail("this is a `--features mocks` build serving fixture data, not the real "
         "project — rebuild with `cargo build -p teksilo_ui` and re-run", s)
print("project loaded.")

if not open_item(ITEM):
    fail(f"could not select {ITEM!r} in the binder", s)
print(f"  opened {ITEM!r}")

# ── 2. Baseline: exactly the fixture's three tags ────────────────────────────
print("\n== baseline: 3 tags on Zeus ==")
print(f"  expecting exactly 3 tags, the set {sorted(BASE_TAGS)}")
before = tag_pill_names()
if before is None:
    s.dump("no Tags List")
    s.shot("/tmp/overflow-no-tags-list.png")
    fail(f"no Tags List found in the Inspector for {ITEM!r} (looked for "
         f"{TAGS_PILL_LIST})", s)
if len(before) != 3:
    fail(f"expected exactly 3 tags on {ITEM!r} (the fixture's baseline), got "
         f"{len(before)}: {before}", s)
before_set = {t.lower() for t in before}
if before_set != BASE_TAGS:
    fail(f"expected the fixture's baseline {sorted(BASE_TAGS)}, got "
         f"{sorted(before_set)}", s)
print(f"  baseline confirmed: {before}")

# ── 3. Create three more tags, through the "+" popover ───────────────────────
print("\n== create 3 more tags on Zeus ==")
def picker_dialog():
    """The open tag-picker Dialog, or None."""
    return next((n for n in s.nodes()
                 if n.get("role") == "Dialog"
                 and any(v in (n.get("label") or "").lower() for v in ADD_A_TAG)), None)


def open_picker():
    """Open the Inspector's "+" tag popover, and CONFIRM it opened.

    Idempotent and verified, because the "+" is a toggle: clicking it while
    the popover is already up closes it, so a blind click-then-proceed can
    silently close a popover that was already open.
    """
    if picker_dialog():
        return
    for attempt in range(3):
        btn = find(s, ADD_A_TAG, role="Button")
        if not btn:
            s.dump("no Add-a-tag button")
            fail(f"the Inspector shows no 'Add a tag' button (looked for {ADD_A_TAG})", s)
        res, _ = s.call("invoke_action", {"node": btn["id"], "action": "click"})
        if isinstance(res, dict) and res.get("isError"):
            click(s, btn.get("bounds") or {})
        s.settle()
        for _ in range(10):
            time.sleep(0.3)
            if picker_dialog():
                return
    s.dump("picker never opened")
    fail("the tag picker never opened after three attempts", s)


# One open/close cycle PER tag: the popover does not survive a create.
for name in NEW_TAGS:
    open_picker()
    create_tag(name)
    # Close it so the next iteration starts from a known state; open_picker
    # tolerates either outcome.
    s.call("inject_key", {"key": "Escape"})
    s.settle()
    time.sleep(0.8)
    print(f"  created and assigned {name!r}")

s.call("inject_key", {"key": "Escape"})
s.settle()
time.sleep(0.6)

print(f"  expecting exactly 6 tags now, the full set {ALL_TAGS_SORTED}")
after = tag_pill_names()
if after is None or len(after) != 6:
    s.dump("tag count after creation")
    s.shot("/tmp/overflow-create-fail.png")
    fail(f"expected exactly 6 tags on {ITEM!r} after creating 3, got "
         f"{after}", s)
after_set = {t.lower() for t in after}
expected_set = {t.lower() for t in ALL_TAGS_SORTED}
if after_set != expected_set:
    fail(f"expected the tag set {sorted(expected_set)}, got {sorted(after_set)}", s)
print(f"  Zeus now carries all 6: {after}")
s.shot("/tmp/overflow-6-tags.png")

# ── 4. The editor (cap 8): all 6 dots, no overflow ───────────────────────────
print("\n== surface 1: the editor (cap 8, no overflow) ==")
if not wait_for(lambda: dot_row() is not None):
    s.dump("no dot row in the editor")
    s.shot("/tmp/overflow-editor-fail.png")
    fail(f"the editor shows no dot row for {ITEM!r} carrying {TAG_MARKER!r}", s)
row = dot_row()
assert_cell_count("editor", row, 6)
assert_row_label("editor", row, FULL_LABEL)
leftover = overflow_cells_in_box(row)
if leftover:
    s.shot("/tmp/overflow-editor-unexpected.png")
    fail(f"editor: found an overflow cell {[c.get('value') for c in leftover]} "
         f"inside the row's box, but the editor's cap (8) is over the item's "
         f"6 tags — there should be no overflow cell at all", s)
print("  editor: no overflow cell — all 6 dots shown, as expected under cap 8")
s.shot("/tmp/overflow-editor.png")

# ── 5. Navigate to the container that has Stream/Corkboard segments ──────────
print("\n== open the container (Chapter 1) ==")
container = next((n for n in binder_rows()
                  if (n.get("label") or "").strip() == CONTAINER), None)
if not container:
    s.dump("no container row")
    fail(f"{CONTAINER!r} is not in the binder", s)
click(s, container.get("bounds") or {})
s.settle()
time.sleep(1.6)
if not find(s, CORKBOARD_SEG):
    s.dump("no segmented control")
    fail(f"{CONTAINER!r} offers no Stream/Corkboard segments — it must be a "
         f"Folder whose sub_role is Book/Part/ChapterScene", s)
print(f"  opened container {CONTAINER!r}")

# ── 6. The corkboard (cap 4): 4 dots + "+2", naming exactly the tail two ────
print("\n== surface 2: the corkboard (cap 4) ==")
select_segment(CORKBOARD_SEG)
pane = content_pane()
row = scroll_for_dot_row(pane)
if not row:
    s.dump("no dot row on the corkboard")
    s.shot("/tmp/overflow-corkboard-fail.png")
    fail(f"no corkboard card shows a dot row for {ITEM!r} carrying "
         f"{TAG_MARKER!r}", s)
assert_cell_count("corkboard", row, 5)  # 4 dots + 1 overflow cell
assert_row_label("corkboard", row, FULL_LABEL)
assert_overflow("corkboard", row, 2, HIDDEN_CORKBOARD, "corkboard")

# ── 7. The stream (cap 5): 5 dots + "+1", naming exactly the last one ───────
print("\n== surface 3: the stream (cap 5) ==")
select_segment(STREAM_SEG)
pane = content_pane()
row = scroll_for_dot_row(pane)
if not row:
    s.dump("no dot row in the stream")
    s.shot("/tmp/overflow-stream-fail.png")
    fail(f"the manuscript stream's row header for {ITEM!r} shows no dot row "
         f"carrying {TAG_MARKER!r}", s)
assert_cell_count("stream", row, 6)  # 5 dots + 1 overflow cell
assert_row_label("stream", row, FULL_LABEL)
assert_overflow("stream", row, 1, HIDDEN_STREAM, "stream")

print("\nOK — the corkboard's +2 and the stream's +1 overflow cells both "
      "render, both name exactly what they hide, and the editor shows all 6 "
      "dots with no overflow — the cap is per-surface, and the collapse path "
      "works end to end.")
s.stop()
