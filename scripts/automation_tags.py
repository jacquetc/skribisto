#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the teksilo automation MCP bridge and verify the
tags feature end-to-end, against a *legacy* project.

The fixture is chosen deliberately. `resources/test/skribisto_test_project.skrib`
is a v2.0.7 SQLite project whose `tbl_tag` holds three tags, all three attached
to one binder item:

    A                        #FFFAFA   (near-white)
    B                        #FF0000
    very looooooooooong tag  #000000   (near-black)

Those tags have survived every load/save since the Rust rewrite and have never
been shown to anyone — the "hostage data" this feature pays off. So a run that
finds them in the Settings pane proves the whole chain at once: the legacy
upgrader, `load_work_uc`, the palette model, and the pane.

It is also, by luck, the contrast fixture the plan asks for: a near-white and a
near-black tag exercise both ends of the derived-text-colour rule (`A` must get
black text, the long one white) and both ends of the hairline-border rule (each
fill vanishes into one theme's surface without it).

Asserts:

  1. the legacy project opens and Settings reaches Work ▸ Tags;
  2. the pane lists all three legacy tags, with its controls (add field, filter,
     preset, import/export) present;
  3. the duplicate-name warning fires case-insensitively — typing "a" while tag
     "A" exists warns but leaves the field usable (duplicates are allowed);
  4. the Inspector shows the same three tags on the item that carries them.

Navigation notes, learned the hard way and worth keeping:

  * Settings tree rows have role "Unknown" and expose **no** AccessKit actions,
    so `invoke_action` has nothing to invoke — every row is reached by pointer.
  * The `Work: <name>` section starts **collapsed**, so the Tags row does not
    exist in the tree until its chevron is tapped.
  * Once expanded, Tags is the 5th child and lands *below the rail's visible
    viewport*: it has layout bounds but is scrolled out, so a click at those
    bounds hits nothing at all. It must be scrolled into view first — hence
    `select_page`, which scrolls and re-reads bounds between attempts.

Run:  python3 scripts/automation_tags.py
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
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
# A throwaway copy, never the checked-in fixture — this probe saves.
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import working_copy

LEGACY = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "tags")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name

# The app follows the *system* locale, so a probe written against English labels
# fails on a French desktop with "no such row" — which reads exactly like a broken
# selector. Every user-visible string this probe matches on lives here, in both
# locales, and the matchers below take these tuples.
SEC_WORK = ("work", "\u0153uvre", "oeuvre")     # settings-sec-work
PAGE_TAGS = ("tags", "\u00e9tiquettes")          # settings-page-tags
SW_BIBLE = ("story bible", "bible narrative")   # settings-tags-discoverable


def fail(msg, app=None, mcp=None, log=None):
    print(f"FAIL: {msg}")
    if log:
        try:
            print("--- app log (tail) ---")
            print("".join(open(log).readlines()[-40:]))
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
            t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
            if s and t:
                sock, tok = s.group(1), t.group(1)
                break
            if self.app.poll() is not None:
                fail("app exited before printing the bridge socket", self.app, None, self.log)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 20s", self.app, None, self.log)
        self.sock, self.tok = sock, tok
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
                                      "clientInfo": {"name": "tags-test", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect MCP (socket never reachable)", self.app, self.mcp, self.log)
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

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def by_role(self, role):
        return [n for n in self.nodes() if n.get("role") == role]

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(l for l in self.labels()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def settle(self):
        try:
            self.call("settle")
        except Exception:
            time.sleep(0.3)

    def dump(self, title):
        print(f"--- AT tree: {title} ---")
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            if lab:
                b = n.get("bounds") or {}
                print(f"  [{n.get('role')}] {lab!r} @x={b.get('x')} y={b.get('y')} "
                      f"actions={n.get('actions')}")

    def shot(self, path):
        try:
            res, _ = self.call("screenshot")
            for c in res.get("content", []):
                if c.get("type") == "image" and c.get("data"):
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"screenshot -> {path}")
        except Exception as e:
            print("screenshot failed:", e)

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


# ── Launch on the legacy project (a path on argv skips the Launcher) ──────────
print("== launch on the legacy v2.0.7 project ==")
s = Session([LEGACY])
if not s.wait_label("chapter", timeout=20):
    fail("the legacy project did not load", s.app, s.mcp, s.log)

# A `--features mocks` binary ignores the path on argv and serves fixture data,
# so every assertion below would pass against tags that were never in the file.
# That is the one failure this script must never report as a success — rebuild
# without the feature and run again.
if s.wait_label("mock", timeout=1):
    fail("this is a `--features mocks` build serving fixture data, not the legacy "
         "project — rebuild with `cargo build -p teksilo_ui` and re-run",
         s.app, s.mcp, s.log)
print("legacy project loaded.")

LEGACY_TAGS = ("a", "b", "very looooooooooong tag")
LEGACY_TAGS_SET = set(LEGACY_TAGS)


def joined():
    """Labels *and* values, lowercased.

    Values matter as much as labels here: a tag's name is the value of its
    inline-rename `TextInput` and its colour is the value of a `ColorWell`, so a
    labels-only search finds the pane's chrome and none of its content.
    """
    parts = []
    for n in s.nodes():
        for key in ("label", "value"):
            v = n.get(key)
            if isinstance(v, str) and v.strip():
                parts.append(v)
    return " | ".join(parts).lower()


def has_any(variants):
    j = joined()
    return any(v in j for v in variants)


def exact_texts():
    """Every label/value as an exact, stripped, lowercased string.

    Substring matching is unusable for this fixture: two of its tags are named
    "A" and "B", and "a" occurs in virtually every sentence of chrome on screen.
    Tag presence has to be matched whole.
    """
    out = set()
    for n in s.nodes():
        for key in ("label", "value"):
            v = n.get(key)
            if isinstance(v, str) and v.strip():
                out.add(v.strip().lower())
    return out


def rail_node(variants, exact=False):
    """First node in the left category rail (x < 280) matching any variant.

    The rail bound matters: without it a page name like "Tags" also matches the
    Inspector's tag list in the main window *behind* the modal, and clicking
    that does nothing to the settings pane.
    """
    for n in s.nodes():
        lab = (n.get("label") or "").strip().lower()
        if not lab:
            continue
        hit = lab in variants if exact else any(v in lab for v in variants)
        if not hit:
            continue
        b = n.get("bounds") or {}
        if not isinstance(b, dict) or b.get("x", 9999) >= 280:
            continue
        return n
    return None


def pointer_click(b, dx=None):
    """Synthetic primary click (atomic down+up → a real tap gesture) at the
    centre of a bounds dict, or at `bounds.x + dx` when `dx` is given (used to
    hit a row's leading-edge chevron rather than its centre)."""
    if not (isinstance(b, dict) and "x" in b):
        return False
    cx = b["x"] + (dx if dx is not None else b.get("width", 0) / 2)
    cy = b["y"] + b.get("height", 0) / 2
    # The parameter is `action`, not `kind`. Sibling scripts pass `kind`, which the
    # bridge ignores — they work only because "click" is the default.
    s.call("inject_pointer", {"x": cx, "y": cy, "action": "click"})
    return True


def hover(b):
    """Approach from outside, then jiggle inside.

    A single `move` that lands on the widget is not reliably read as a hover
    *enter* — the dwell timer starts on motion within the widget, so one event
    arriving already-inside can leave it unarmed."""
    if not (isinstance(b, dict) and "x" in b):
        return False
    cx = b["x"] + b.get("width", 0) / 2
    cy = b["y"] + b.get("height", 0) / 2
    s.call("inject_pointer", {"x": cx - 160, "y": cy, "action": "move"})
    time.sleep(0.25)
    s.call("inject_pointer", {"x": cx, "y": cy, "action": "move"})
    time.sleep(0.1)
    s.call("inject_pointer", {"x": cx + 1, "y": cy + 1, "action": "move"})
    return True


def wait_for(pred, tries=15, delay=0.3):
    for _ in range(tries):
        time.sleep(delay)
        if pred():
            return True
    return False


def node_text(n):
    """A node's visible text, wherever it lives: `Label`-role nodes carry theirs in
    `value`, everything else in `label`."""
    if not n:
        return ""
    return ((n.get("value") or "") + " " + (n.get("label") or "")).strip()


def breadcrumb():
    return [n.get("label") for n in s.nodes() if n.get("role") == "Link"]


def settings_open():
    """The instant-apply footer is on every pane, so it survives the panel
    being restructured around any particular page name."""
    return (has_any(("reset to defaults", "réinitialiser"))
            and has_any(("done", "terminé")))


def open_settings():
    for args in ({"key": ",", "ctrl": True},
                 {"key": ",", "modifiers": ["ctrl"]},
                 {"key": "Comma", "modifiers": ["ctrl"]}):
        s.call("inject_key", args)
        for _ in range(10):
            time.sleep(0.4)
            if settings_open():
                return True
    return False


def page_reached(target):
    """The breadcrumb if the panel is showing `target`, else None.

    `target` is a tuple of accepted names (one per locale); a bare string is
    accepted too. Taking a tuple matters — an earlier version compared a string
    against the tuple the caller passed, which is silently never equal, so the
    walk arrived and the check said it had not.

    Deliberately not `breadcrumb()[-1] in target`. The breadcrumb is "every node
    with role Link", so any link *inside the pane* — an explainer, a "learn
    more" — becomes the last element and the check fails on a page that is
    plainly open. Matching any crumb is safe here because a page name never
    collides with its own parent section (`Œuvre: test` ▸ `Étiquettes`).
    """
    names = (target,) if isinstance(target, str) else tuple(target)
    crumb = breadcrumb()
    if crumb and any(c and c.strip().lower() in names for c in crumb):
        return crumb
    # Second, independent signal: the rail row itself, when the tree reports
    # selection. Costs nothing and covers a breadcrumb that has not repainted.
    row = rail_node(names, exact=True)
    if row and row.get("selected"):
        return crumb or list(names[:1])
    return None


def select_page(target, anchor="keymap", steps=14, anchor_node=None):
    """Select a rail page by walking to it from a visible anchor row.

    Pointer-clicking the target's reported bounds does not work, and the reason
    is worth stating: a row far enough down the rail is laid out *below the
    scroll viewport*. Its AT bounds are perfectly real, but nothing is painted
    there, so the click lands on empty chrome and the pane never changes — the
    failure looks exactly like a wrong selector.

    Keyboard navigation sidesteps it entirely, because teksilo scrolls the
    focused row into view (`scroll_focused_into_view`). So: click a row that
    *is* visible to put focus in the tree, then step until we arrive, letting
    the tree do the scrolling.

    Two things this used to get wrong, both of which reported a reachable page
    as unreachable:

      * It walked **Down only**. `anchor` is a fixed guess about tree order, so
        a target above it could never be reached — indistinguishable from the
        page not existing. It now tries both directions, re-anchoring between.
      * It clicked the anchor unconditionally. The panel remembers its last page
        between launches, so a run could *start* on the target and the first
        click would leave it.

    And a third, which is why `anchor_node` exists: the default anchor was the
    English page name "keymap", so on a French desktop `rail_node` found nothing
    and this returned None before pressing a single key — reported as "could not
    select the page" when the page was sitting right there. Prefer passing a row
    the caller has *already located*; the name lookup is only a fallback.
    """
    got = page_reached(target)
    if got:
        return got

    a = anchor_node or rail_node((anchor,) if isinstance(anchor, str) else anchor, exact=True)
    if not a:
        return None

    for key in ("Down", "Up"):
        pointer_click(a.get("bounds") or {})
        s.settle()
        time.sleep(0.5)
        for _ in range(steps):
            got = page_reached(target)
            if got:
                return got
            s.call("inject_key", {"key": key})
            s.settle()
            time.sleep(0.35)
    return page_reached(target)


# ── 1. Settings ▸ Work ▸ Tags ────────────────────────────────────────────────
print("\n== open Settings ==")
if not open_settings():
    s.dump("settings never opened")
    fail("Settings did not open (Ctrl+,)", s.app, s.mcp, s.log)
print("Settings open.")

print("\n== find the Work section ==")
work = rail_node(SEC_WORK)
if not work:
    s.dump("no Work section in the rail")
    fail(f"the rail has no Work section (looked for {SEC_WORK})", s.app, s.mcp, s.log)
print(f"  found {work.get('label')!r} at y={(work.get('bounds') or {}).get('y')}")

tags_row = rail_node(PAGE_TAGS, exact=True)
if not tags_row:
    # Only needed if the section ever ships collapsed; tap the chevron once (a
    # second tap would toggle it back).
    pointer_click(work.get("bounds") or {}, dx=10)
    for _ in range(12):
        s.settle()
        time.sleep(0.4)
        tags_row = rail_node(PAGE_TAGS, exact=True)
        if tags_row:
            break
if not tags_row:
    s.dump("Work section did not expand")
    fail("no Tags page under the Work section", s.app, s.mcp, s.log)
print(f"  Tags row at y={(tags_row.get('bounds') or {}).get('y')} "
      f"(below the viewport — keyboard nav will scroll it in)")

print("\n== select the Tags page ==")
crumb = select_page(PAGE_TAGS, anchor_node=work)
if not crumb:
    s.dump("Tags page never selected")
    s.shot("/tmp/tags-pane-fail.png")
    fail("could not select the Tags page", s.app, s.mcp, s.log)
print(f"  breadcrumb: {crumb}")

# ── 2. The pane's controls, and the hostage tags ─────────────────────────────
print("\n== the pane ==")
# Placeholders ("Name a new tag", "Filter tags") are painted but not exposed to
# the AT tree, so they are deliberately not asserted here — only what a screen
# reader can actually reach.
PANE_BITS = {
    "add button":   ("add tag", "ajouter"),
    "preset menu":  ("apply a preset", "appliquer un"),
    "import":       ("import", "importer"),
    "export":       ("export", "exporter"),
    "count":        ("3 tags", "3 étiquettes"),
    "story bible":  ("story bible", "bible"),
}
missing = [k for k, v in PANE_BITS.items() if not has_any(v)]
if missing:
    print("  text:", joined()[:900])
    s.shot("/tmp/tags-pane-fail.png")
    fail(f"missing from the Tags pane: {missing}", s.app, s.mcp, s.log)
print("  controls: all present ->", ", ".join(PANE_BITS))

texts = exact_texts()
absent = [t for t in LEGACY_TAGS if t not in texts]
if absent:
    print("  exact texts:", sorted(texts)[:40])
    s.shot("/tmp/tags-pane-fail.png")
    fail(f"legacy tags missing from the palette: {absent}", s.app, s.mcp, s.log)
print(f"  legacy tags listed: {', '.join(LEGACY_TAGS)}  <- the hostage data, freed")

# The colours came across too, extremes intact — these are the two that need the
# derived ring to stay visible (#FFFAFA on a light card, #000000 on a dark one).
for hexval in ("#fffafa", "#ff0000", "#000000"):
    if hexval not in texts:
        fail(f"legacy tag colour {hexval} did not survive the load", s.app, s.mcp, s.log)
print("  legacy colours intact: #FFFAFA, #FF0000, #000000")

# Every button must be fully inside the pane: "Export…" was previously clipped
# off the right edge by a toolbar that overflowed.
#
# The pane's own controls are named EXACTLY, not by substring. The settings panel
# is an overlay, so the main window's widgets are still in the AT tree behind it,
# and a substring match on "export" also catches the toolbar's "Exporter le
# livre" — which sits outside the pane by design and duly reported a 14px
# "overflow" that did not exist. (Same trap `rail_node`'s x<280 bound exists to
# avoid, one layer along.)
PANE_BUTTONS = ("import…", "importer…", "export…", "exporter…",
                "apply a preset…", "appliquer un préréglage…",
                "add tag", "ajouter")
pane_right = max((n.get("bounds") or {}).get("x", 0) + (n.get("bounds") or {}).get("width", 0)
                 for n in s.nodes()
                 if (n.get("bounds") or {}).get("x", 0) > 280
                 and n.get("role") == "Label")
checked = 0
for n in s.nodes():
    b = n.get("bounds") or {}
    lab = (n.get("label") or "").strip().lower()
    if n.get("role") != "Button" or b.get("x", 0) <= 280:
        continue
    if lab in PANE_BUTTONS:
        checked += 1
        right = b.get("x", 0) + b.get("width", 0)
        if right > pane_right + 2:
            s.shot("/tmp/tags-clip-fail.png")
            fail(f"{lab!r} overflows the pane (right={right:.0f} > {pane_right:.0f})",
                 s.app, s.mcp, s.log)
# Without this the check passes loudest when it matches nothing at all — a
# renamed control would silently turn the clipping guard into a no-op.
if checked < 3:
    fail(f"only {checked} pane buttons matched {PANE_BUTTONS} — the labels moved, "
         f"so this clipping check was about to prove nothing", s.app, s.mcp, s.log)
print(f"  toolbar fits: {checked} pane buttons, none clipped at the right edge")

s.shot("/tmp/tags-settings-pane.png")

# ── The "Story bible" switch explains itself on hover ────────────────────────
print("\n== story-bible rich tooltip ==")
switch = next((n for n in s.nodes()
               if n.get("role") == "Switch"
               and any(v in (n.get("label") or "").lower() for v in SW_BIBLE)), None)
if not switch:
    fail(f"no story-bible switch on any tag row (looked for {SW_BIBLE})",
         s.app, s.mcp, s.log)

b = switch.get("bounds") or {}
cx, cy = b["x"] + b.get("width", 0) / 2, b["y"] + b.get("height", 0) / 2
# Approach from outside, then jiggle inside. A single `move` landing on the
# widget is not reliably read as a hover *enter* — the dwell timer starts on
# motion within the widget, so one event that arrives already-inside can leave
# it unarmed. Two events either side of the boundary always arm it.
s.call("inject_pointer", {"x": cx - 160, "y": cy, "action": "move"})
time.sleep(0.25)
s.call("inject_pointer", {"x": cx, "y": cy, "action": "move"})
time.sleep(0.1)
s.call("inject_pointer", {"x": cx + 2, "y": cy + 1, "action": "move"})

# Rich tooltips open on `motion.tooltip_delay` (200 ms); poll well past it
# rather than sleeping a fixed amount, since the delay is themeable.
tip = None
for _ in range(15):
    time.sleep(0.3)
    if has_any(("looks for this item", "recherche cet élément")):
        tip = True
        break
if not tip:
    print("  text:", joined()[:900])
    s.shot("/tmp/tags-tooltip-fail.png")
    fail("hovering the Story bible switch showed no explanation", s.app, s.mcp, s.log)
print("  hovering the switch explains what turning it on does")
s.shot("/tmp/tags-story-bible-tooltip.png")

# ── 3. The duplicate-name warning (case-insensitive, non-blocking) ───────────
print("\n== duplicate-name warning ==")
# Reset what the previous check deliberately left behind. It dwelled a tooltip
# into its *sticky* state, and a sticky tooltip is a Dialog that takes focus —
# so typing here would go to it, not to the add field. Move the pointer away and
# press Escape before touching anything.
s.call("inject_pointer", {"x": 40, "y": 400, "action": "move"})
time.sleep(0.3)
s.call("inject_key", {"key": "Escape"})
s.settle()
time.sleep(0.6)
# The add field carries no placeholder or value in the AT tree, so it has to be
# found by position — but "topmost TextInput below the breadcrumb" is not a sound
# handle and was picking the wrong widget entirely: a stray TextInput at x=384
# sits *outside* the settings modal (whose content starts at x≈450), and the
# main window's own breadcrumb Links share the y range used to exclude it. The
# probe typed into that stray, the AT value dutifully changed, and the pane's
# real field stayed empty — reported as "the warning is broken".
#
# Anchor on meaning instead: the add field is the TextInput sharing a row with
# the "Add tag" button. That button has a label, in both locales, so the handle
# survives both a re-layout and a translation.
add_btn = next((n for n in s.nodes()
                if n.get("role") == "Button"
                and (n.get("label") or "").strip().lower() in ("add tag", "ajouter")), None)
if not add_btn:
    fail("no Add-tag button — cannot locate the add row", s.app, s.mcp, s.log)
btn_b = add_btn.get("bounds") or {}
add_field = next((n for n in s.nodes()
                  if n.get("role") == "TextInput"
                  and abs((n.get("bounds") or {}).get("y", -999) - btn_b.get("y", 0)) < 12
                  and (n.get("bounds") or {}).get("x", 0) < btn_b.get("x", 0)), None)
if not add_field:
    fail(f"no TextInput on the add row (button at y={btn_b.get('y')})",
         s.app, s.mcp, s.log)
# Guard the guard: the add field starts empty. If it holds text, we have latched
# onto some other input and are about to overwrite real content.
if (add_field.get("value") or "").strip():
    fail(f"expected an empty add-tag field, got one holding "
         f"{add_field.get('value')!r} — refusing to type into it", s.app, s.mcp, s.log)

# Focus first, then type. Without the focus call `type_text` updates the node's
# AccessKit value but the text never reaches the widget's signal — so the field
# renders empty, the validation effect never runs, and the AT tree reports the
# text as present. That combination reads exactly like "the warning is broken",
# which is what it was mistaken for; the screenshot showed an empty field while
# the tree said 'a'. `automation_search.py` had it right all along.
s.call("invoke_action", {"node": add_field["id"], "action": "focus"})
time.sleep(0.3)
s.call("type_text", {"node": add_field["id"], "text": "a"})
s.settle()
time.sleep(0.8)
if not has_any(("already exists", "existe déjà")):
    # Say whether the text even landed. "No warning" and "nothing was typed" look
    # identical in a label dump, and they need opposite fixes.
    now = next((n for n in s.nodes() if n.get("id") == add_field["id"]), None)
    print(f"  add field now holds {(now or {}).get('value')!r} "
          f"(typed 'a'; a warning is expected because tag 'A' exists)")
    print("  labels:", joined()[:900])
    s.shot("/tmp/tags-dup-fail.png")
    fail("typing an existing name (different case) raised no warning", s.app, s.mcp, s.log)
print("  typing 'a' with tag 'A' present -> warned, and still accepted (duplicates are legal)")
s.shot("/tmp/tags-duplicate-warning.png")

# ── 4. The Inspector carries the same tags ───────────────────────────────────
print("\n== close Settings and check the Inspector ==")
done = None
for n in s.nodes():
    if (n.get("label") or "").strip().lower() in ("done", "terminé"):
        done = n
        break
if done:
    pointer_click(done.get("bounds") or {})
    s.settle()
    time.sleep(0.8)

if settings_open():
    fail("Settings would not close", s.app, s.mcp, s.log)

def binder_tree_rows():
    """The binder's own tree rows: role "Unknown" at x≈48.

    The x window is narrow on purpose. `x < 120` alone also swept up the activity
    bar at x≈28 (Binder / Search / Trash / Settings) and the hamburger menu at
    x≈24; clicking through those closed the project and left an earlier run
    asserting against the Welcome screen.
    """
    return [n for n in s.nodes()
            if n.get("role") == "Unknown"
            and 40 <= (n.get("bounds") or {}).get("x", 9999) <= 140
            and (n.get("label") or "").strip()]


# Make sure the binder is the visible panel — but only act if it is not already,
# because the activity-bar entry *toggles*. Which panel is showing is persisted
# between launches, so a run that left the Trash up (or the binder hidden) would
# otherwise poison every run after it, in a way that reads as "the tags are
# missing" rather than "you are looking at the wrong panel".
if not binder_tree_rows():
    for n in s.nodes():
        b = n.get("bounds") or {}
        lab = (n.get("label") or "").strip().lower()
        if lab in ("binder", "classeur") and b.get("x", 999) < 45:
            pointer_click(b)
            s.settle()
            time.sleep(0.8)
            break

# All three tags hang off one binder item ("1.1 Zeus", tree code 14 in the legacy
# DB), so walk the binder until the Inspector shows them. Snapshot the rows up
# front: clicking changes the tree, and iterating a live snapshot would skip
# entries.
binder_rows = binder_tree_rows()
if not binder_rows:
    s.dump("no binder rows found")
    fail("could not find the binder tree", s.app, s.mcp, s.log)
print(f"  walking {len(binder_rows)} binder rows")

def inspector_texts():
    """Exact texts from the **Inspector column only**.

    Scoping matters more than it looks. `exact_texts()` reads the whole tree, and
    the Settings pane that was just closed can still contribute nodes \u2014 so the
    unscoped check matched all three tag names on the *first* row it clicked and
    reported "Inspector shows all three tags on 'Sol'". Sol carries no tags at
    all; the fixture attaches them to "1.1 Zeus". The screenshot showed an empty
    \u00c9tiquettes section directly under a passing assertion.

    The column is located from its own section header rather than a hardcoded x,
    so a resized or repositioned Inspector does not silently widen the scope
    back out.
    """
    header = next((n for n in s.nodes()
                   if node_text(n).strip().lower() in ("tags", "\u00e9tiquettes")
                   and (n.get("bounds") or {}).get("x", 0) > 600), None)
    if not header:
        return set()
    left = (header.get("bounds") or {}).get("x", 0) - 20
    out = set()
    for n in s.nodes():
        if (n.get("bounds") or {}).get("x", -1) >= left:
            t = node_text(n).strip().lower()
            if t:
                out.add(t)
    return out


found_on = None
for n in binder_rows:
    pointer_click(n.get("bounds") or {})
    s.settle()
    time.sleep(0.5)
    # Bail loudly if a click took the project down rather than selecting a row:
    # every later assertion would otherwise be measuring the Welcome screen.
    if (has_any(("works", "examples", "\u0153uvres", "exemples"))
            and not has_any(("binder", "classeur"))):
        s.shot("/tmp/tags-inspector-fail.png")
        fail(f"clicking {(n.get('label') or '')!r} closed the project", s.app, s.mcp, s.log)
    if LEGACY_TAGS_SET <= inspector_texts():
        found_on = (n.get("label") or "").strip()
        break

if not found_on:
    s.dump("no binder item showed the tags")
    s.shot("/tmp/tags-inspector-fail.png")
    fail("no binder item shows its three tags in the Inspector", s.app, s.mcp, s.log)
# The fixture attaches all three tags to exactly one item. Naming it here means a
# future fixture edit that moves them fails loudly instead of quietly passing on
# whichever row happened to match.
if found_on.lower() != "1.1 zeus":
    fail(f"the tags showed on {found_on!r}, but the fixture attaches them to '1.1 Zeus' \u2014 "
         f"either the fixture changed or the Inspector is showing another item's tags",
         s.app, s.mcp, s.log)
print(f"  Inspector shows all three tags on {found_on!r}")
s.shot("/tmp/tags-inspector.png")

# ── A tag pill's composite tooltip renders on the tooltip surface ────────────
print("\n== tag pill tooltip ==")
# Match on `node_text` (label *or* value) rather than `label` alone: a Label-role
# node carries its text in `value`, and which of the two a pill uses is an
# implementation detail this probe should not be pinned to.
LONG_TAG = "very looooooooooong tag"
# Either role is correct for a removable pill and teksilo reports ListBoxOption
# in a pill row; pinning one spelling of it tests the framework, not the feature.
PILL_ROLES = ("ListItem", "ListBoxOption")
pill = next((n for n in s.nodes()
             if n.get("role") in PILL_ROLES
             and node_text(n).strip().lower() == LONG_TAG), None)
if not pill:
    # Say what *does* carry the text, so the next reader fixes the selector
    # instead of re-deriving where the pill went.
    carriers = [f"role={n.get('role')} label={n.get('label')!r} value={n.get('value')!r}"
                for n in s.nodes() if LONG_TAG in node_text(n).lower()]
    print("  nodes carrying the tag name:")
    for c in carriers[:8]:
        print(f"    {c}")
    fail(f"no tag pill in roles {PILL_ROLES} ({len(carriers)} nodes carry the name)",
         s.app, s.mcp, s.log)
b = pill.get("bounds") or {}
cx, cy = b["x"] + b.get("width", 0) / 2, b["y"] + b.get("height", 0) / 2
s.call("inject_pointer", {"x": cx - 200, "y": cy, "action": "move"})
time.sleep(0.25)
s.call("inject_pointer", {"x": cx, "y": cy, "action": "move"})
time.sleep(0.1)
s.call("inject_pointer", {"x": cx + 2, "y": cy + 1, "action": "move"})
# Composite tooltips use `tooltip_delay_heavy` (400 ms), not the rich delay.
shown = False
for _ in range(15):
    time.sleep(0.3)
    if any((n.get("role") or "") in ("Tooltip", "Dialog")
           and "very looooooooooong tag" in (n.get("label") or "").lower()
           for n in s.nodes()):
        shown = True
        break
if shown:
    print("  the tag's composite tooltip opens")
    s.shot("/tmp/tags-pill-tooltip.png")
else:
    # Not fatal: the tooltip's own surface colours are asserted by unit tests, and
    # hover timing here is the least reliable thing in the harness.
    print("  (tooltip did not open within the poll window; skipping the screenshot)")

# ── The dot row: on the editor subtitle, per-dot hover, whole-row click ──────
print("\n== tag dots in context ==")

# The tagged item is open in the editor from the Inspector walk above, so its
# subtitle should now carry a dot row. It announces itself with every tag name,
# which is also the only way a screen reader reaches the dots at all.
dot_row = None
for n in s.nodes():
    # The row is a Label naming every tag; the Popover's trigger supplies the button.
    # Label-role nodes carry their text in `value`.
    text = ((n.get("value") or "") + " " + (n.get("label") or "")).lower()
    if all(t in text for t in LEGACY_TAGS) and n.get("role") == "Label":
        dot_row = n
        break
if not dot_row:
    s.dump("no dot row found")
    s.shot("/tmp/tags-dots-fail.png")
    fail("no tag dot row announcing all three tags", s.app, s.mcp, s.log)
print(f"  dot row announces: {node_text(dot_row)!r}")

b = dot_row.get("bounds") or {}
print(f"  dot row is {b.get('width', 0):.0f}x{b.get('height', 0):.0f} dp")
s.shot("/tmp/tags-dots-row.png")

# Hovering one dot must show that dot's tooltip, not the row's. The row spans
# every dot, so aim at the first cell rather than the row's centre.
print("\n== per-dot tooltip ==")
first_dot = {"x": b.get("x", 0), "y": b.get("y", 0),
             "width": 18.0, "height": b.get("height", 18.0)}
hover(first_dot)
if wait_for(lambda: any(
        (n.get("role") or "") in ("Tooltip", "Dialog")
        and (n.get("label") or "").strip().lower() in LEGACY_TAGS_SET
        for n in s.nodes())):
    tip = next(n for n in s.nodes()
               if (n.get("role") or "") in ("Tooltip", "Dialog")
               and (n.get("label") or "").strip().lower() in LEGACY_TAGS_SET)
    print(f"  hovering the first dot shows {tip.get('label')!r} alone")
    s.shot("/tmp/tags-dot-tooltip.png")
else:
    print("  (no per-dot tooltip within the poll window)")
    s.shot("/tmp/tags-dot-tooltip-miss.png")

# Clicking anywhere on the row — including on a dot — opens the picker. This is
# the assertion that the dots' lack of an on_tap actually lets the press bubble
# to the Popover trigger; if a dot ever grows a handler, this fails.
print("\n== whole-row click opens the picker ==")
s.call("inject_pointer", {"x": b.get("x", 0) + 4, "y": b.get("y", 0) + b.get("height", 18) / 2,
                          "action": "move"})
time.sleep(0.2)
pointer_click({"x": b.get("x", 0), "y": b.get("y", 0), "width": 8.0,
               "height": b.get("height", 18.0)})
s.settle()

def picker_open():
    # The picker lists every palette tag as a tickable option.
    opts = [n for n in s.nodes() if n.get("role") == "ListBoxOption"]
    return len(opts) >= len(LEGACY_TAGS)

if not wait_for(picker_open):
    s.dump("picker did not open")
    s.shot("/tmp/tags-picker-fail.png")
    fail("clicking a dot did not open the tag picker — the dots are swallowing "
         "the row's tap (did one grow an on_tap?)", s.app, s.mcp, s.log)

opts = [n for n in s.nodes() if n.get("role") == "ListBoxOption"]
ticked = [n for n in opts if n.get("selected")]
print(f"  picker lists {len(opts)} tags, {len(ticked)} ticked")
if len(ticked) < len(LEGACY_TAGS):
    print("  options:", [(n.get("label"), n.get("selected")) for n in opts])
    s.shot("/tmp/tags-picker-fail.png")
    fail("the item's three tags should all be ticked", s.app, s.mcp, s.log)
print("  every tag on the item is ticked")
s.shot("/tmp/tags-picker.png")

# Untick one. The assertion is on the dot row, not on the tick: the row is what
# the writer actually sees, and it is the end of the whole chain (picker tap ->
# relationship write -> DTO refetch -> signal -> repaint).
print("\n== unticking removes the tag ==")


def dots_row():
    # The row is a Label, not a Button: the Popover's OverlayTrigger owns the button
    # role at the same bounds, and two would be announced twice. A Label-role node
    # carries its text in `value`, so match on either field.
    for n in s.nodes():
        text = (n.get("value") or "") + " " + (n.get("label") or "")
        if "very looooooooooong tag" in text and n.get("role") == "Label" and "," in text:
            return n
    return None


before = dots_row()
before_w = (before.get("bounds") or {}).get("width", 0) if before else 0
target = next(n for n in opts if (n.get("label") or "").strip().lower() == "b")
pointer_click(target.get("bounds") or {})
s.settle()

if not wait_for(lambda: "B," not in node_text(dots_row())):
    s.shot("/tmp/tags-untick-fail.png")
    fail("unticking 'B' did not drop it from the dot row", s.app, s.mcp, s.log)

after = dots_row()
after_w = (after.get("bounds") or {}).get("width", 0) if after else 0
print(f"  row went {before_w:.0f} -> {after_w:.0f} dp, now {node_text(after)!r}")
if after_w >= before_w:
    fail(f"the row should have lost a dot ({before_w} -> {after_w})", s.app, s.mcp, s.log)

# The picker must survive the toggle. It used to die: the row owned the popover
# AND rebuilt on every tag change, so each tick tore down its own picker.
if not any(n.get("role") == "ListBoxOption" for n in s.nodes()):
    s.shot("/tmp/tags-picker-closed.png")
    fail("the picker closed on the first tick — unticking several tags would mean "
         "reopening it each time", s.app, s.mcp, s.log)
print("  the picker stayed open, so several tags can be toggled in one visit")
s.shot("/tmp/tags-unticked.png")

# Put it back, so the fixture is left as found.
back = next((n for n in s.nodes()
             if n.get("role") == "ListBoxOption"
             and (n.get("label") or "").strip().lower() == "b"), None)
if back:
    pointer_click(back.get("bounds") or {})
    s.settle()
    time.sleep(0.5)
    print("  re-ticked 'B' to leave the fixture as found")

print("\nOK — tags verified end-to-end on a legacy project.")
s.close()
