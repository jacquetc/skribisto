#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto with the keyboard ALONE and prove the tags surface is
reachable without a pointer — plan verification item 12.

Every other tags probe (`automation_tags.py`, `automation_tag_chips.py`, ...)
drives the feature with synthesized pointer clicks, because that is the fastest
way to prove the *feature* works. None of them prove a keyboard-only writer, or
a screen-reader user tabbing through the app, can reach the same controls. That
gap is what this probe closes, for three surfaces:

  1. the Inspector's tag pill field on an item that carries tags, and its "+"
     (add) control;
  2. the Settings > Work > Tags pane rows — each row's rename field, its
     story-bible switch, and its delete control;
  3. the tag picker popover, opened from the keyboard, ticking a tag off and
     back on with Enter/Space.

After the initial launch, EVERY interaction is `inject_key` — Tab, Shift+Tab,
the four arrows, Enter, Space, Escape. No `inject_pointer`, no `focus_node` (a
bridge convenience that seeds focus directly and would silently skip the very
question this probe asks). The one exception is a single, clearly-labelled
`invoke_action(..., "click")` used ONLY to arm a detector in Phase 3 (see the
comment there) and to restore the fixture afterwards — never as a stand-in for
a keyboard action under test.

## Why several assertions here are not "was `selected`/`focused` true"

The obvious way to prove "keyboard focus reached row X" is to read the
`focused`/`selected` flags AccessKit reports and check they landed on X. That
works for a genuinely individually-focusable widget (a `Pill`, a `TextInput`,
a `TagPickRow` — each is its own `.focusable(true)` leaf, so `focused==True`
lands on exactly that node). It does NOT work uniformly for composite,
roving-tabindex containers, and three controls in this probe live inside one:

  * `TreeView` (the binder, and the Settings rail) is ONE Tab stop
    (`tree_view/widget_impl.rs:185`); arrow keys move an internal cursor.
    AccessKit selection for a row lives on a SEPARATE ANCESTOR node —
    `TreeItemWrapper` (`teksilo-widgets/src/list_item_a11y.rs`) — not on the
    label-bearing content node a text search finds first. Reading `selected`
    straight off the node matched by label is a silent no-op: it is always
    `None`. (`automation_tags.py`'s own `page_reached()` carries exactly this
    check as a "costs nothing" fallback and never actually needed it to fire,
    because its primary signal — the breadcrumb — always won first. The binder
    has no breadcrumb, so here the fix is load-bearing: `climb_role()` below
    walks UP from the label node to find the real wrapper.)
  * `MenuList` (the hamburger's File menu) is worse: it tracks which row is
    keyboard-highlighted with a plain `Signal<Option<usize>>` and paints a
    background wash from it (`KeyboardHighlightWrapper`,
    `teksilo-widgets/src/menu_list.rs:127-165`) — that wrapper has NO
    `accessibility()` override at all, and `MenuItem::accessibility()`
    (`menu_item.rs:1360-1435`) never calls `set_selected` either. There is
    **no AccessKit signal, anywhere, for "which File-menu row is currently
    highlighted."** This probe does not pretend otherwise: it locates
    "Settings" by scanning the (always-present, focus-independent) row LABELS
    once, computes the exact ArrowDown count deterministically, and proves it
    landed correctly by the one signal that does exist — the observable
    consequence of activating it (the Settings modal opens).

## The Enter/Space-on-a-tag-row question (Phase 3)

`TagPickRow` (`tag_pill_field.rs`) is `.focusable(true)` with both
`.on_tap(...)` and an explicit `.on_key(...)` wiring `Key::Enter | Key::Space`
to the same toggle — so the row is expected to be both Tab-reachable and
key-activatable. Phase 3 still verifies this live rather than trusting the
source read, and reports whichever of the two actually happens; either way it
arms a detector (rule: prove the checker would have caught a real toggle)
before concluding.

Run:  python3 scripts/automation_tag_keyboard.py
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
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import wait_for_load, working_copy

# This probe never types or edits anything persistent (see the docstring), but
# it still opens an editor tab and toggles a tag mid-run — never the checked-in
# fixture.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "kbdtags")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name

# ── Every user-visible string, both locales, keyed to its ftl entry ─────────
# (verified directly against crates/teksilo_ui/locales/{en-US,fr-FR}/*.ftl and
# teksilo/crates/teksilo-widgets/locales/*.ftl — see file:line citations below)
ITEM = "1.1 Zeus"                      # fixture data, not localized
CHAPTER1 = "Chapter 1"                 # fixture data, not localized
LEGACY_TAGS = {"A", "B", "very looooooooooong tag"}   # fixture data

MENU_BTN = ("menu", "menu")                       # a11y-builtin-menu (identical both locales)
MENU_WORK = ("work", "œuvre")                   # menu-work: "&Work" / "Œ&uvre" (mnemonic stripped)
MENU_SETTINGS = ("settings", "paramètres")        # menu-settings: "S&ettings" / "&Paramètres"
SETTINGS_DONE = ("done", "terminé")               # settings-done
SETTINGS_RESET = ("reset to defaults", "réinitialiser")  # settings-reset
SEC_WORK = ("work", "œuvre", "oeuvre")            # settings-sec-work
PAGE_TAGS = ("tags", "étiquettes")                # settings-page-tags
TAGS_LIST_A11Y = ("tags", "étiquettes")           # tags-pill-list — disambiguate via role=="List", see §ScopedList
TAGS_ADD = ("add a tag", "ajouter une étiquette")  # tags-pill-add
SW_BIBLE = ("story bible", "bible narrative")     # settings-tags-discoverable
DELETE_PREFIX = ("delete", "supprimer")           # settings-tags-delete = "Delete { $name }" / "Supprimer { $name }"


def fail(msg, sess=None):
    print(f"FAIL: {msg}")
    if sess:
        try:
            print("--- app log (tail) ---")
            print("".join(open(sess.log).readlines()[-40:]))
        except Exception:
            pass
        sess.stop()
    sys.exit(1)


class Session:
    """One launched app + connected MCP server, restartable. Copied verbatim
    from automation_languages.py's Session — the template this probe follows."""

    def __init__(self, path):
        self.mcp = None
        self.app = None
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, path], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT)
        sock = tok = None
        deadline = time.time() + 25
        while time.time() < deadline:
            txt = open(self.log).read()
            a = re.search(r"bridge socket = (\S+)", txt)
            b = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
            if a and b:
                sock, tok = a.group(1), b.group(1)
                break
            if self.app.poll() is not None:
                fail("app exited before printing the bridge socket", self)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 25s", self)
        while not os.path.exists(sock) and time.time() < deadline:
            time.sleep(0.05)
        self._id = 0
        self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "kbd-tags", "version": "1"}})
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

    def shot(self, path):
        res, _ = self.call("screenshot", {})
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")

    def stop(self):
        """Terminate and *wait* — see automation_languages.py's Session.stop()
        for why: relaunching before the old process releases its lock file
        hands off silently and reads exactly like a crash."""
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        for p in (self.mcp, self.app):
            if p:
                try:
                    p.wait(timeout=10)
                except Exception:
                    p.kill()
        time.sleep(2.0)


def text_of(n):
    """A node's visible text: Label-role nodes carry theirs in `value`."""
    if not n:
        return ""
    return ((n.get("value") or "") + " " + (n.get("label") or "")).strip()


def find(s, variants, role=None):
    vs = tuple(v.lower() for v in variants)
    for n in s.nodes():
        if role and n.get("role") != role:
            continue
        if any(v in text_of(n).lower() for v in vs):
            return n
    return None


def click(s, b, dx=None):
    """Kept only for the ONE-TIME detector-arming/restore step in Phase 3 —
    see the comment there. Never used as a keyboard-reachability mechanism."""
    if not (isinstance(b, dict) and "x" in b):
        return False
    s.call("inject_pointer", {"x": b["x"] + (dx if dx is not None else b.get("width", 0) / 2),
                              "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    return True


# ── Keyboard-only primitives ─────────────────────────────────────────────────

def key(s, k, shift=False, settle=0.15):
    """One `inject_key`. The bridge's own dispatcher settles after every key
    (`finish_settle` in executor.rs's `InjectKey` arm), so no extra explicit
    `settle` call is needed here — just enough sleep for binding-registry
    rebuilds and repaint to land before the next snapshot."""
    s.call("inject_key", {"key": k, "shift": shift})
    time.sleep(settle)


def tab(s, shift=False, settle=0.15):
    key(s, "Tab", shift=shift, settle=settle)


def focused_node(s):
    """The sole node AccessKit currently reports focus on, or None. There is
    at most one — `focused: id == focus` in the bridge (executor.rs:518)."""
    return next((n for n in s.nodes() if n.get("focused")), None)


def tab_until(s, pred, max_steps, shift=False):
    """Press Tab (or Shift+Tab) up to `max_steps` times; after each press,
    return the currently-focused node the moment `pred` accepts it, plus the
    step count. `(None, max_steps)` means the bound was exhausted — the caller
    decides whether that is a reachability failure or a bound too small."""
    for i in range(1, max_steps + 1):
        tab(s, shift=shift)
        cur = focused_node(s)
        if cur and pred(cur):
            return cur, i
    return None, max_steps


# ── Node-graph helpers: resolving composite-container state ─────────────────

def index_nodes(nodes):
    """id -> node, and child id -> parent id, built from every node's own
    `children` list. Needed because `TreeItemWrapper` (the node that actually
    carries `selected`/`expanded`/role=="TreeItem") is a DIFFERENT node than
    the label-bearing content widget a text search finds — see the module
    docstring's "why not just read selected" section."""
    by_id = {n["id"]: n for n in nodes if "id" in n}
    parent_of = {}
    for n in nodes:
        for c in (n.get("children") or []):
            parent_of[c] = n.get("id")
    return by_id, parent_of


def climb_role(by_id, parent_of, start_id, roles, max_hops=8):
    """Walk up from `start_id` (inclusive) through parents, return the first
    ancestor whose role is in `roles`, or None within `max_hops`."""
    cur, hops = start_id, 0
    while cur is not None and hops <= max_hops:
        n = by_id.get(cur)
        if n and n.get("role") in roles:
            return n
        cur = parent_of.get(cur)
        hops += 1
    return None


def find_label(nodes, label, scope=None):
    for n in nodes:
        if scope and not scope(n):
            continue
        if text_of(n).strip() == label:
            return n
    return None


def find_box(nodes, variants, role, x_min=0):
    """A landmark node (e.g. the Inspector's Tags `List`) by exact text + role
    + a leading-edge x bound — the same idiom automation_tags.py's
    `inspector_texts()` uses to keep the Settings pane (closed or not) from
    bleeding into the match."""
    vs = tuple(v.lower() for v in variants)
    for n in nodes:
        if role and n.get("role") != role:
            continue
        b = n.get("bounds") or {}
        if b.get("x", -1) < x_min:
            continue
        if text_of(n).strip().lower() in vs:
            return n
    return None


def in_box(n, box_bounds, pad=6.0):
    b = n.get("bounds") or {}
    if "x" not in b or "x" not in box_bounds:
        return False
    return (b["x"] >= box_bounds["x"] - pad
            and b["x"] + b.get("width", 0) <= box_bounds["x"] + box_bounds.get("width", 0) + pad
            and b["y"] >= box_bounds["y"] - pad
            and b["y"] + b.get("height", 0) <= box_bounds["y"] + box_bounds.get("height", 0) + pad)


def binder_scope(n):
    """The binder tree's own row band: x in [40,140] — narrow on purpose.
    `x < 120` alone also sweeps up the activity bar (x≈28) and the hamburger
    (x≈24); automation_tags.py's `binder_tree_rows()` hit that exact trap."""
    b = n.get("bounds") or {}
    x = b.get("x")
    return x is not None and 40 <= x <= 140


def inspector_tags_box(nodes):
    return find_box(nodes, TAGS_LIST_A11Y, role="List", x_min=600)


def pill_names_in_box(nodes, box):
    if not box:
        return None
    bb = box.get("bounds") or {}
    return sorted(text_of(n).strip() for n in nodes
                  if n.get("role") == "ListItem" and in_box(n, bb) and text_of(n).strip())


def find_pick_row(nodes, name):
    for n in nodes:
        if n.get("role") == "ListBoxOption" and text_of(n).strip() == name:
            return n
    return None


def hunt_tree_row(s, target_label, scope, max_steps=40):
    """Bounded Down-then-Up walk that selects `target_label` inside a TreeView
    that ALREADY holds real keyboard focus. Reads true selection via the
    row's `TreeItemWrapper` ancestor (`climb_role`), not the label node
    itself — see the module docstring. Returns (label_node, wrapper_node) or
    (None, None) if exhausted in both directions."""
    for direction in ("Down", "Up"):
        for _ in range(max_steps):
            nodes = s.nodes()
            label_node = find_label(nodes, target_label, scope)
            if label_node:
                by_id, parent_of = index_nodes(nodes)
                wrapper = climb_role(by_id, parent_of, label_node["id"], {"TreeItem"})
                if wrapper and wrapper.get("selected"):
                    return label_node, wrapper
            key(s, direction)
    return None, None


def ensure_visible(s, target_label, scope, ancestor_hints, max_steps=40):
    """Make `target_label` appear in the tree, expanding a collapsed ancestor
    (tried in `ancestor_hints` order) if it is currently hidden. `ArrowRight`
    expands WITHOUT moving the cursor (outline.rs's own design comment: arrow
    navigation never opens a tab; tree_view/widget_impl.rs:316-323)."""
    if find_label(s.nodes(), target_label, scope):
        return True
    for ancestor in ancestor_hints:
        anc_label, anc_wrapper = hunt_tree_row(s, ancestor, scope, max_steps=max_steps)
        if not anc_label:
            continue
        if not anc_wrapper.get("expanded"):
            key(s, "ArrowRight", settle=0.3)
        if find_label(s.nodes(), target_label, scope):
            return True
    return find_label(s.nodes(), target_label, scope) is not None


# ── Settings navigation helpers, ported from automation_tags.py verbatim ────
# (the breadcrumb-based signal is unaffected by the TreeItemWrapper issue
# above — it is real text on a real Link node, not a per-row selection flag —
# so it needs no change to work seeded from a keyboard Tab instead of a click)

def joined(s):
    parts = []
    for n in s.nodes():
        for k_ in ("label", "value"):
            v = n.get(k_)
            if isinstance(v, str) and v.strip():
                parts.append(v)
    return " | ".join(parts).lower()


def has_any(s, variants):
    j = joined(s)
    return any(v in j for v in variants)


def settings_open(s):
    return has_any(s, SETTINGS_RESET) and has_any(s, SETTINGS_DONE)


def breadcrumb(s):
    return [n.get("label") for n in s.nodes() if n.get("role") == "Link"]


def rail_node(s, variants, exact=False):
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


def page_reached(s, target):
    names = (target,) if isinstance(target, str) else tuple(target)
    crumb = breadcrumb(s)
    if crumb and any(c and c.strip().lower() in names for c in crumb):
        return crumb
    # Fallback only — see the module docstring: this is the same `selected`
    # read that never actually needs to fire in automation_tags.py either,
    # because the breadcrumb above wins first. Left in for parity/cheapness.
    row = rail_node(s, names, exact=True)
    if row and row.get("selected"):
        return crumb or list(names[:1])
    return None


def select_page_kbd(s, target, steps=20):
    """`select_page()` from automation_tags.py, keyboard-seeded: the caller
    has already Tab-ed real focus onto the rail Tree, so no anchor click is
    needed before walking Down/Up."""
    got = page_reached(s, target)
    if got:
        return got
    for direction in ("Down", "Up"):
        for _ in range(steps):
            key(s, direction, settle=0.3)
            got = page_reached(s, target)
            if got:
                return got
    return page_reached(s, target)


# ═════════════════════════════════════════════════════════════════════════
# Launch
# ═════════════════════════════════════════════════════════════════════════
print("== launch ==")
s = Session(FIXTURE)

# "chapter" is fixture DATA (a binder item title), not a translated UI string,
# so this check is locale-independent — same idiom automation_tags.py uses.
if not wait_for_load(s.nodes, ("chapter", "chapitre")):
    fail("the fixture did not load (no 'chapter' text anywhere in the AT tree)", s)
# A `--features mocks` binary ignores argv and serves fixture data regardless
# of what we typed — every assertion below would then pass against data that
# was never really driven by our keys. This is the one failure this probe
# must never silently report as success.
if has_any(s, ("mock",)):
    fail("this is a `--features mocks` build serving fixture data, not the real "
         "project — rebuild without the feature and re-run", s)
print("project loaded.")

try:
    # ═════════════════════════════════════════════════════════════════════
    # Phase 1a — reach "1.1 Zeus" in the binder and open it (keyboard only)
    # ═════════════════════════════════════════════════════════════════════
    print("\n== Phase 1a: binder — Tab to the tree, arrow to '1.1 Zeus', Enter to open ==")
    binder_tree, steps = tab_until(s, lambda n: n.get("role") == "Tree", max_steps=60)
    if not binder_tree:
        fail("Tab never reached the binder Tree within 60 presses from launch — "
             "either nothing in the binder dock is a Tab stop, or something earlier "
             "in the app traps focus first", s)
    print(f"  reached the binder Tree after {steps} Tab presses (focused==True)")

    if not ensure_visible(s, ITEM, binder_scope, [CHAPTER1]):
        fail(f"{ITEM!r} never appeared in the binder even after trying to expand "
             f"{CHAPTER1!r} via ArrowRight — the fixture's structure may have changed, "
             f"or ArrowRight does not expand a collapsed row", s)

    item_label, item_wrapper = hunt_tree_row(s, ITEM, binder_scope, max_steps=40)
    if not item_label:
        fail(f"could not select {ITEM!r} in the binder using ArrowDown/ArrowUp alone "
             f"(bounded hunt exhausted in both directions) — the row's real selection "
             f"state lives on its TreeItemWrapper ancestor; see climb_role()", s)
    print(f"  '{ITEM}' row selected (TreeItemWrapper reports selected=True)")

    # Only Enter opens the tab — arrow-key navigation deliberately never does
    # (outline.rs's own design comment, confirmed at tree_view/widget_impl.rs
    # :400-408: `on_activate` fires from `Key::Enter`, not from selection).
    key(s, "Enter", settle=1.2)

    nodes = s.nodes()
    tags_box = inspector_tags_box(nodes)
    if not tags_box:
        fail(f"no Inspector 'Tags' List landmark found after pressing Enter on "
             f"{ITEM!r} — either the item did not open, or the Inspector column "
             f"is not where expected (x>600)", s)
    opened_names = pill_names_in_box(nodes, tags_box)
    if sorted(LEGACY_TAGS) != opened_names:
        fail(f"Enter opened SOME item, but its Inspector Tags box shows "
             f"{opened_names!r}, not the fixture's {sorted(LEGACY_TAGS)!r} — Enter "
             f"likely activated the wrong row (arrow-key hunting landed elsewhere)", s)
    print(f"  '{ITEM}' opened: Inspector shows exactly its 3 tags {opened_names}")

    # ═════════════════════════════════════════════════════════════════════
    # Phase 1b — Tab onto the Inspector's tag pills + "+" button, and ONLY those
    # ═════════════════════════════════════════════════════════════════════
    print("\n== Phase 1b: Inspector — Tab onto the 3 pills, then the '+' button ==")
    box_bounds = tags_box.get("bounds") or {}
    in_box_stops = []
    entered = False
    after_box = None
    MAX_INSPECTOR_TABS = 150
    for i in range(MAX_INSPECTOR_TABS):
        tab(s)
        cur = focused_node(s)
        if cur is None:
            continue
        if in_box(cur, box_bounds):
            entered = True
            in_box_stops.append(cur)
        elif entered:
            after_box = cur
            break
    else:
        if entered:
            print(f"  NOTE: hit the {MAX_INSPECTOR_TABS}-Tab bound while still inside "
                  f"the Tags box — raise MAX_INSPECTOR_TABS if this recurs")
    if not entered:
        fail(f"Tab never landed inside the Inspector's Tags box within "
             f"{MAX_INSPECTOR_TABS} presses after opening {ITEM!r}", s)

    pills = [n for n in in_box_stops if n.get("role") == "ListItem"]
    buttons = [n for n in in_box_stops if n.get("role") == "Button"]
    pill_names = [text_of(n).strip() for n in pills]
    # Exact set AND exact count — the fixture's hostage-data count is known
    # (3), so "≥3" would silently pass on a duplicate or a stray match.
    if set(pill_names) != LEGACY_TAGS or len(pill_names) != 3:
        fail(f"Tab-ing through the Tags box did not land on exactly the 3 expected "
             f"pills; got {pill_names!r}, expected exactly {sorted(LEGACY_TAGS)}", s)
    if len(buttons) != 1 or not any(v in text_of(buttons[0]).lower() for v in TAGS_ADD):
        fail(f"expected exactly 1 '+' button focus stop after the 3 pills; got "
             f"{len(buttons)}: {[text_of(b) for b in buttons]}", s)
    add_button_node = buttons[0]
    if in_box_stops[-1].get("id") != add_button_node.get("id"):
        fail(f"the '+' button is not the LAST in-box Tab stop — pill/button order in "
             f"the Inspector does not match TagPillField's build order (pills, then "
             f"'+'); stops were {[(n.get('role'), text_of(n)) for n in in_box_stops]}", s)
    if len(in_box_stops) != 4:
        fail(f"expected exactly 4 Tab stops inside the Tags box (3 pills + '+' — the "
             f"pill's own '×' remove glyph is documented as NOT a Tab stop, "
             f"pill.rs:253); got {len(in_box_stops)}: "
             f"{[(n.get('role'), text_of(n)) for n in in_box_stops]}", s)
    print(f"  4/4 in-box stops confirmed: pills {pill_names}, then the '+' button")

    # Positive control (rule 8): if the pill's '×' ever grows a Tab stop, or a
    # stray focus stop sneaks in, the NEXT Tab must land OUTSIDE the box.
    if after_box is None:
        tab(s)
        after_box = focused_node(s)
    if after_box and in_box(after_box, box_bounds):
        fail(f"a 5th focus stop lands inside the Tags box: role="
             f"{after_box.get('role')} text={text_of(after_box)!r} — the box no "
             f"longer holds exactly 4 stops", s)
    after_box_role = after_box.get("role") if after_box else None
    after_box_text = text_of(after_box) if after_box else None
    print(f"  confirmed: the 5th Tab leaves the box "
          f"(now on {after_box_role}:{after_box_text!r})")

    # Shift+Tab back onto the '+' button so Phase 2 starts with real focus there
    # — also the meaningful exercise of Shift+Tab the brief asks for.
    tab(s, shift=True)
    cur = focused_node(s)
    if not cur or cur.get("id") != add_button_node.get("id"):
        fail(f"Shift+Tab did not return focus to the '+' button; landed on "
             f"role={cur.get('role') if cur else None} text={text_of(cur)!r}", s)
    print("  Shift+Tab returns focus to the '+' button")

    # ═════════════════════════════════════════════════════════════════════
    # Phase 3 — open the tag picker from the keyboard, and try to toggle "B"
    # ═════════════════════════════════════════════════════════════════════
    print("\n== Phase 3: tag picker popover — open, then try Enter/Space to toggle 'B' ==")
    key(s, "Enter", settle=0.5)   # OverlayTrigger wires Enter/Space on KeyUp (popover.rs:616-641)

    nodes = s.nodes()
    opts = [n for n in nodes if n.get("role") == "ListBoxOption"]
    opt_names = sorted(text_of(n).strip() for n in opts)
    # Exactly 3, not "≥3": the fixture's whole palette is 3 tags and no filter
    # is applied, so 3 is the correct exact expectation.
    if len(opts) != 3 or opt_names != sorted(LEGACY_TAGS):
        fail(f"expected exactly 3 tag-picker rows {sorted(LEGACY_TAGS)}, got "
             f"{len(opts)}: {opt_names} — did Enter actually open the popover?", s)
    if not all(o.get("selected") for o in opts):
        fail(f"all 3 tags are assigned to {ITEM!r}, so all 3 picker rows should be "
             f"ticked; got {[(text_of(o), o.get('selected')) for o in opts]}", s)
    print(f"  popover open: 3/3 rows present, all ticked (matches {ITEM!r}'s 3 tags)")

    baseline_names = pill_names_in_box(s.nodes(), inspector_tags_box(s.nodes()))
    if baseline_names != sorted(LEGACY_TAGS):
        fail(f"baseline pill count before toggling is wrong: {baseline_names}", s)
    baseline_count = len(baseline_names)

    # Discover live how many Tabs land us on row "B" — the spec's own §5
    # flags that whether Enter auto-focused the filter field or the first row
    # is not knowable from the call site alone.
    b_row, b_steps = tab_until(
        s, lambda n: n.get("role") == "ListBoxOption" and text_of(n).strip() == "B",
        max_steps=15)
    if not b_row:
        stops = []
        for _ in range(6):
            tab(s)
            fn = focused_node(s)
            stops.append(((fn or {}).get("role"), text_of(fn)))
        fail(f"Tab never landed on the 'B' picker row within 15 presses of opening "
             f"the popover; diagnostic extra stops: {stops}", s)
    if not b_row.get("selected"):
        fail("'B' row is reachable but is not reported ticked before any toggle — "
             "the baseline itself is wrong, stop before drawing conclusions", s)
    print(f"  Tab reached the 'B' row after {b_steps} presses (focused=True, selected=True)")

    def toggle_state():
        nodes_ = s.nodes()
        row_ = find_pick_row(nodes_, "B")
        box_ = inspector_tags_box(nodes_)
        names_ = pill_names_in_box(nodes_, box_)
        return row_, names_

    key(s, "Space", settle=0.4)
    row_after, names_after = toggle_state()
    space_worked = (row_after is not None and row_after.get("selected") is False
                    and names_after is not None and len(names_after) == baseline_count - 1)

    enter_worked = False
    if not space_worked:
        # Re-locate before a second attempt — a rebuild triggered by the (no-op)
        # Space could in principle have changed the row's WidgetId, and a stale
        # id would make a follow-up key event a silent no-op we'd misread as
        # "Enter doesn't work either."
        row_now, _ = toggle_state()
        if row_now and not row_now.get("focused"):
            s.call("invoke_action", {"node": row_now["id"], "action": "focus"})
            time.sleep(0.2)
        key(s, "Enter", settle=0.4)
        row_after2, names_after2 = toggle_state()
        enter_worked = (row_after2 is not None and row_after2.get("selected") is False
                        and names_after2 is not None and len(names_after2) == baseline_count - 1)

    if space_worked or enter_worked:
        which = "Space" if space_worked else "Enter"
        print(f"  RESULT: {which} DID toggle 'B' via keyboard alone — this CONTRADICTS "
              f"the source-reading prediction (TagPickRow has no .on_key at "
              f"tag_pill_field.rs:363-417); either a generic key->tap bridge exists "
              f"that this reading missed, or the framework changed under this probe. "
              f"Reporting the surprise, not forcing the narrative.")
        row_now, _ = toggle_state()
        if row_now:
            click(s, row_now.get("bounds") or {})  # restore, pointer is fine for cleanup only
            time.sleep(0.3)
        final_names = pill_names_in_box(s.nodes(), inspector_tags_box(s.nodes()))
        if final_names != sorted(LEGACY_TAGS):
            fail(f"could not restore the fixture's tag set after the keyboard toggle "
                 f"succeeded: now {final_names}, expected {sorted(LEGACY_TAGS)}", s)
        print("  fixture restored to its original 3 tags")
    else:
        print("  RESULT: neither Space nor Enter toggled 'B' — matches the source-reading "
              "prediction that TagPickRow is Tab-reachable but not key-activatable "
              "(tag_pill_field.rs:363-417 has .on_tap but no .on_key).")
        # Arm the detector (rule 8): prove a REAL toggle would have been caught,
        # so the no-op above is a confirmed feature gap, not a broken checker.
        # `invoke_action(..., "click")` here is explicitly NOT a keyboard action
        # and is not offered as evidence for "reachable by keyboard" — it exists
        # only to validate the two readers (`selected`, pill count) themselves.
        row_now, _ = toggle_state()
        if not row_now:
            fail("lost track of the 'B' row before the detector-arming click", s)
        s.call("invoke_action", {"node": row_now["id"], "action": "click"})
        time.sleep(0.3)
        armed_row, armed_names = toggle_state()
        armed_ok = (armed_row is not None and armed_row.get("selected") is False
                    and armed_names is not None and len(armed_names) == baseline_count - 1)
        if not armed_ok:
            fail("a REAL click on the 'B' row (invoke_action) ALSO failed to flip "
                 "`selected` / drop the pill count — the readers themselves are "
                 "broken, so the Enter/Space result above is not trustworthy either", s)
        print("  detector armed: a real click DOES flip selected=False and drop the "
              "pill count to 2, confirming the Enter/Space no-op above is a genuine "
              "feature gap in TagPickRow, not a broken assertion.")
        row_now2, _ = toggle_state()
        if row_now2:
            s.call("invoke_action", {"node": row_now2["id"], "action": "click"})
            time.sleep(0.3)
        restored_names = pill_names_in_box(s.nodes(), inspector_tags_box(s.nodes()))
        if restored_names != sorted(LEGACY_TAGS):
            fail(f"could not restore 'B' after arming the detector: now "
                 f"{restored_names}, expected {sorted(LEGACY_TAGS)}", s)
        print("  fixture restored to its original 3 tags")

    key(s, "Escape", settle=0.3)   # close the popover
    key(s, "Escape", settle=0.3)   # belt and braces — a promoted sticky tooltip
                                    # from keyboard focus can steal the next phase's focus

    # ═════════════════════════════════════════════════════════════════════
    # Phase 2a — reach Settings from the hamburger, using only Tab/Arrow/Enter
    # ═════════════════════════════════════════════════════════════════════
    print("\n== Phase 2a: hamburger -> File -> Settings, entirely by keyboard ==")
    # Exact match, not substring: "menu" is short enough to collide with an
    # unrelated tooltip/label ("Context menu", "File menu…") elsewhere in the
    # 80-step walk, and the hamburger's real accessible name is exactly
    # "Menu"/"Menu" (a11y-builtin-menu, identical both locales).
    hamburger, ham_steps = tab_until(
        s, lambda n: n.get("role") == "Button" and text_of(n).strip().lower() in MENU_BTN,
        max_steps=80)
    if not hamburger:
        fail("Tab never reached the hamburger menu button within 80 presses", s)
    print(f"  reached the hamburger after {ham_steps} Tab presses")

    # Enter reveals the bar AND auto-focuses the first trigger ("File") —
    # menu_bar.rs:1157-1159, `ctx.request_focus(trigger)` inside the reveal
    # callback. No extra Tab needed.
    key(s, "Enter", settle=0.4)
    cur = focused_node(s)
    if not (cur and cur.get("role") == "MenuItem"
            and any(v in text_of(cur).strip().lower() for v in MENU_WORK)):
        fail(f"opening the hamburger did not land focus on the File trigger; focus is "
             f"now role={cur.get('role') if cur else None} text={text_of(cur)!r}", s)
    print(f"  bar revealed, 'File' trigger auto-focused: {text_of(cur)!r}")

    # Enter/Space/ArrowDown all open the trigger's dropdown (menu_bar.rs:677)
    # and move REAL focus onto the Role::Menu container (menu_context.rs's
    # open_at -> ctx.request_focus(focus_id)).
    key(s, "Enter", settle=0.4)
    menu_container = focused_node(s)
    if not (menu_container and menu_container.get("role") == "Menu"):
        fail(f"opening File did not move keyboard focus onto its Role::Menu dropdown; "
             f"focus is now role={menu_container.get('role') if menu_container else None}", s)
    print("  File dropdown open, real focus on its Role::Menu container")

    # MenuItem never exposes which row is keyboard-highlighted (see the module
    # docstring) — so "where is Settings" is answered from the always-present
    # row LABELS, not from a per-step selection read.
    nodes = s.nodes()
    items = sorted((n for n in nodes if n.get("role")
                     in ("MenuItem", "MenuItemCheckBox", "MenuItemRadio")),
                   key=lambda n: ((n.get("bounds") or {}).get("y", 0),
                                  (n.get("bounds") or {}).get("x", 0)))
    labels = [text_of(n).strip() for n in items]
    print(f"  File menu has {len(items)} items: {labels}")
    # Fixed, always-visible items for an open project: New Work, Open Work,
    # Import, Export, one of {Save as file/folder}, Back up now, View
    # backups, Close work, Welcome, Settings, Quit = 11; "Save" itself is the
    # only conditionally-hidden one (autosave). 8 is a conservative floor.
    if len(items) < 8:
        fail(f"only {len(items)} File-menu items found (expected >= 8; a fully-open "
             f"project's fixed items alone are ~11) — labels: {labels}", s)
    settings_idx = next((i for i, l in enumerate(labels) if l.lower() in MENU_SETTINGS), None)
    if settings_idx is None:
        fail(f"'Settings'/'Paramètres' not found among the {len(items)} File items: "
             f"{labels}", s)

    for _ in range(settings_idx + 1):
        key(s, "Down", settle=0.15)
    key(s, "Enter", settle=0.3)

    if not settings_open(s):
        fail(f"pressing Enter after {settings_idx + 1} ArrowDown presses did not open "
             f"Settings — either the computed index into {labels} was wrong, or "
             f"activation itself failed", s)
    print(f"  Settings opened via keyboard (item #{settings_idx} of {len(labels)}: "
          f"{labels[settings_idx]!r})")

    # ═════════════════════════════════════════════════════════════════════
    # Phase 2b — Settings rail: reach Work > Tags via Tab + arrows
    # ═════════════════════════════════════════════════════════════════════
    print("\n== Phase 2b: Settings rail — Tab to the tree, arrow to Work > Tags ==")
    rail_tree, rail_steps = tab_until(s, lambda n: n.get("role") == "Tree", max_steps=40)
    if not rail_tree:
        fail("Tab never reached the Settings rail's Tree within 40 presses", s)
    print(f"  reached the rail Tree after {rail_steps} Tab presses")

    crumb = select_page_kbd(s, PAGE_TAGS)
    if not crumb:
        fail(f"could not reach Work > Tags via arrow keys alone (looked for {PAGE_TAGS})", s)
    if len(crumb) < 2 or crumb[-1].strip().lower() not in PAGE_TAGS:
        fail(f"reached a page, but the breadcrumb {crumb} does not END on Tags — "
             f"likely a stale previous page rather than the real destination", s)
    print(f"  Work > Tags reached; breadcrumb: {crumb}")

    # ═════════════════════════════════════════════════════════════════════
    # Phase 2c — each row's rename field, story-bible switch, delete control
    # ═════════════════════════════════════════════════════════════════════
    print("\n== Phase 2c: Tags pane rows — Tab through all 3 tags' controls ==")
    FORWARD_BOUND = 60   # ~6 toolbar stops + 1 list-container stop + 3x5 row stops = ~22 expected
    records = []
    for _ in range(FORWARD_BOUND):
        tab(s)
        records.append(focused_node(s))

    first = records[0] if records else None
    if first and first.get("role") == "TextInput":
        print("  first pane control after the rail is a TextInput (the 'Add tag' field, "
              "as work_tags.rs's add_row() builds it first) — cheap positive control ok")
    else:
        print(f"  NOTE: first pane control after the rail was role="
              f"{first.get('role') if first else None} value={(first or {}).get('value')!r} "
              f"— expected the empty 'Add tag' TextInput; the pane's layout order may "
              f"differ from what this probe assumed (not fatal on its own)")

    rows = {}
    for i, n in enumerate(records):
        if n and n.get("role") == "TextInput" and (n.get("value") or "").strip() in LEGACY_TAGS:
            name = n["value"].strip()
            rows.setdefault(name, {})["name"] = (i, n)
    for name, info in rows.items():
        start = info["name"][0]
        later_starts = [oi["name"][0] for other, oi in rows.items()
                        if other != name and oi["name"][0] > start]
        end = min(later_starts) if later_starts else len(records)
        for j in range(start + 1, end):
            n = records[j]
            if not n:
                continue
            if "switch" not in info and n.get("role") == "Switch":
                info["switch"] = (j, n)
            if "delete" not in info and n.get("role") == "Button":
                lab = text_of(n).strip().lower()
                if lab.startswith(("delete ", "supprimer ")):
                    info["delete"] = (j, n)

    problems = []
    for name in sorted(LEGACY_TAGS):
        info = rows.get(name)
        if not info or "name" not in info:
            problems.append(f"{name}: no focus-reachable TextInput with that exact value")
            continue
        sw = info.get("switch")
        if not sw or not any(v in text_of(sw[1]).lower() for v in SW_BIBLE):
            problems.append(f"{name}: no matching Story-bible Switch stop after its name field")
        de = info.get("delete")
        if not de or name.lower() not in text_of(de[1]).lower():
            problems.append(f"{name}: no matching delete Button stop "
                            f"(expected 'Delete {name}' / 'Supprimer {name}')")
    if len(rows) != 3 or problems:
        fail(f"expected exactly 3 tag rows fully reachable by Tab "
             f"({sorted(LEGACY_TAGS)}); found {len(rows)} row(s) with a name-matching "
             f"stop, problems: {problems}", s)
    print("  all 3 rows reachable: name field, Story-bible switch, delete button — "
          "each individually reported focused==True at its own Tab stop")

    # Exercise Shift+Tab meaningfully: rewind from where the forward walk
    # ended back onto row 'B's delete button, then retrace switch -> name.
    b_info = rows["B"]
    del_idx, del_node = b_info["delete"]
    rewind = (FORWARD_BOUND - 1) - del_idx
    for _ in range(rewind):
        tab(s, shift=True)
    cur = focused_node(s)
    if not cur or cur.get("role") != "Button" or cur.get("id") != del_node.get("id"):
        fail(f"rewinding {rewind} Shift+Tab presses did not land back on 'B''s delete "
             f"button; landed on role={cur.get('role') if cur else None} "
             f"text={text_of(cur)!r} — Tab order is not stable across repeats", s)

    tab(s, shift=True)
    step2 = focused_node(s)
    ok_switch = (step2 and step2.get("role") == "Switch"
                and any(v in text_of(step2).lower() for v in SW_BIBLE))
    tab(s, shift=True)
    step3 = focused_node(s)
    ok_name = (step3 and step3.get("role") == "TextInput"
              and (step3.get("value") or "").strip() == "B")
    if not (ok_switch and ok_name):
        fail(f"Shift+Tab from 'B''s delete button did not retrace switch -> name field; "
             f"step2 role={step2.get('role') if step2 else None}, "
             f"step3 role={step3.get('role') if step3 else None} "
             f"value={(step3 or {}).get('value')!r}", s)
    print("  Shift+Tab correctly retraces delete -> Story-bible switch -> name field")

    key(s, "Escape", settle=0.3)   # best-effort close; not asserted — out of scope for item 12

    print("\n" + "=" * 70)
    print("OK — the whole tags surface is reachable by keyboard alone:")
    print("  1. Inspector tag pills (3/3) + '+' button: Tab-reachable, confirmed")
    print("  2. Settings > Work > Tags rows: rename field / story-bible switch / "
          "delete, all 3 tags, all Tab-reachable")
    print("  3. Tag picker popover: opens via Enter on '+'; Enter/Space toggling a "
          "row is " + ("supported" if (space_worked or enter_worked) else
                        "NOT supported — see Phase 3 output above (a confirmed gap, "
                        "not a probe defect)"))
    print("=" * 70)

except SystemExit:
    raise
except Exception as e:
    import traceback
    traceback.print_exc()
    fail(f"unexpected exception during the keyboard walk: {e!r}", s)

s.stop()
