#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the bastyde automation MCP bridge and verify the
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
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
LEGACY = f"{ROOT}/resources/test/skribisto_test_project.skrib"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name


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
            t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
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
         "project — rebuild with `cargo build -p bastyde_ui` and re-run",
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
    s.call("inject_pointer", {"x": cx, "y": cy, "kind": "click"})
    return True


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


def select_page(target, anchor="keymap", steps=12):
    """Select a rail page by walking down to it from a visible anchor row.

    Pointer-clicking the target's reported bounds does not work, and the reason
    is worth stating: a row far enough down the rail is laid out *below the
    scroll viewport*. Its AT bounds are perfectly real, but nothing is painted
    there, so the click lands on empty chrome and the pane never changes — the
    failure looks exactly like a wrong selector.

    Keyboard navigation sidesteps it entirely, because bastyde scrolls the
    focused row into view (`scroll_focused_into_view`). So: click a row that
    *is* visible to put focus in the tree, then press Down until the breadcrumb
    says we have arrived, letting the tree do the scrolling.
    """
    a = rail_node((anchor,), exact=True)
    if not a:
        return None
    pointer_click(a.get("bounds") or {})
    s.settle()
    time.sleep(0.5)
    for _ in range(steps):
        crumb = breadcrumb()
        if crumb and crumb[-1].strip().lower() == target:
            return crumb
        s.call("inject_key", {"key": "Down"})
        s.settle()
        time.sleep(0.35)
    crumb = breadcrumb()
    return crumb if crumb and crumb[-1].strip().lower() == target else None


# ── 1. Settings ▸ Work ▸ Tags ────────────────────────────────────────────────
print("\n== open Settings ==")
if not open_settings():
    s.dump("settings never opened")
    fail("Settings did not open (Ctrl+,)", s.app, s.mcp, s.log)
print("Settings open.")

print("\n== find the Work section ==")
work = rail_node(("work:",))
if not work:
    s.dump("no Work section in the rail")
    fail("the rail has no 'Work: <name>' section", s.app, s.mcp, s.log)
print(f"  found {work.get('label')!r} at y={(work.get('bounds') or {}).get('y')}")

tags_row = rail_node(("tags",), exact=True)
if not tags_row:
    # Only needed if the section ever ships collapsed; tap the chevron once (a
    # second tap would toggle it back).
    pointer_click(work.get("bounds") or {}, dx=10)
    for _ in range(12):
        s.settle()
        time.sleep(0.4)
        tags_row = rail_node(("tags",), exact=True)
        if tags_row:
            break
if not tags_row:
    s.dump("Work section did not expand")
    fail("no Tags page under the Work section", s.app, s.mcp, s.log)
print(f"  Tags row at y={(tags_row.get('bounds') or {}).get('y')} "
      f"(below the viewport — keyboard nav will scroll it in)")

print("\n== select the Tags page ==")
crumb = select_page("tags")
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
pane_right = max((n.get("bounds") or {}).get("x", 0) + (n.get("bounds") or {}).get("width", 0)
                 for n in s.nodes()
                 if (n.get("bounds") or {}).get("x", 0) > 280
                 and n.get("role") == "Label")
for n in s.nodes():
    b = n.get("bounds") or {}
    lab = (n.get("label") or "").strip().lower()
    if n.get("role") != "Button" or b.get("x", 0) <= 280:
        continue
    if any(v in lab for v in ("import", "export", "apply a preset", "add tag")):
        right = b.get("x", 0) + b.get("width", 0)
        if right > pane_right + 2:
            s.shot("/tmp/tags-clip-fail.png")
            fail(f"{lab!r} overflows the pane (right={right:.0f} > {pane_right:.0f})",
                 s.app, s.mcp, s.log)
print("  toolbar fits: no button clipped at the pane's right edge")

s.shot("/tmp/tags-settings-pane.png")

# ── The "Story bible" switch explains itself on hover ────────────────────────
print("\n== story-bible rich tooltip ==")
switch = next((n for n in s.nodes()
               if n.get("role") == "Switch"
               and "story bible" in (n.get("label") or "").lower()), None)
if not switch:
    fail("no Story bible switch on any tag row", s.app, s.mcp, s.log)

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
# The add field is the pane's topmost TextInput. It cannot be found by its
# placeholder (not exposed to AT) nor by its value (empty, like every details
# field), so position is the only stable handle.
#
# The bound must be the breadcrumb, not just `x > 280`: the main window is still
# behind the modal and its editor exposes a title TextInput at y≈99, which sorts
# first and is a *live rename field*. An earlier version of this script typed
# into it and silently renamed a binder item instead of testing anything.
crumb_y = min((n.get("bounds") or {}).get("y", 0)
              for n in s.nodes() if n.get("role") == "Link")
inputs = sorted((n for n in s.nodes()
                 if n.get("role") == "TextInput"
                 and (n.get("bounds") or {}).get("x", 0) > 280
                 and (n.get("bounds") or {}).get("y", 0) > crumb_y),
                key=lambda n: (n.get("bounds") or {}).get("y", 0))
add_field = inputs[0] if inputs else None
if not add_field:
    fail("no add-tag field to type into", s.app, s.mcp, s.log)
# Guard the guard: the add field starts empty. If it holds text, we have latched
# onto some other input and are about to overwrite real content.
if (add_field.get("value") or "").strip():
    fail(f"expected an empty add-tag field, got one holding "
         f"{add_field.get('value')!r} — refusing to type into it", s.app, s.mcp, s.log)

s.call("type_text", {"node": add_field["id"], "text": "a"})
s.settle()
time.sleep(0.5)
if not has_any(("already exists", "existe déjà")):
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

found_on = None
for n in binder_rows:
    pointer_click(n.get("bounds") or {})
    s.settle()
    time.sleep(0.5)
    # Bail loudly if a click took the project down rather than selecting a row:
    # every later assertion would otherwise be measuring the Welcome screen.
    if has_any(("works", "examples")) and not has_any(("binder", "classeur")):
        s.shot("/tmp/tags-inspector-fail.png")
        fail(f"clicking {(n.get('label') or '')!r} closed the project", s.app, s.mcp, s.log)
    if LEGACY_TAGS_SET <= exact_texts():
        found_on = (n.get("label") or "").strip()
        break

if not found_on:
    s.dump("no binder item showed the tags")
    s.shot("/tmp/tags-inspector-fail.png")
    fail("no binder item shows its three tags in the Inspector", s.app, s.mcp, s.log)
print(f"  Inspector shows all three tags on {found_on!r}")
s.shot("/tmp/tags-inspector.png")

# ── A tag pill's composite tooltip renders on the tooltip surface ────────────
print("\n== tag pill tooltip ==")
pill = next((n for n in s.nodes()
             if n.get("role") == "ListItem"
             and (n.get("label") or "").strip().lower() == "very looooooooooong tag"), None)
if not pill:
    fail("no tag pill to hover in the Inspector", s.app, s.mcp, s.log)
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

print("\nOK — tags verified end-to-end on a legacy project.")
s.close()
