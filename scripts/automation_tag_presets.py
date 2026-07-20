#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the bastyde automation MCP bridge and verify that
tag presets apply, translate, dedupe, and (attempt to) undo in one step.

This is plan-verification item 9. `crates/bastyde_ui/src/tags/presets.rs`
builds every preset row IN CODE, through `tr!()`, precisely so a French
project gets French tag names ("personnage", "lieu", "statut/brouillon", ...)
rather than an English wordlist imported once at authoring time. Its own
module doc says so outright: "That is the whole point ... a data file would
have to pick one language at authoring time." presets.rs and
models/work_tags_list_model.rs already carry unit tests for the pure logic —
every genre preset extends Basic, no preset repeats a name, the `status/`
prefix clusters when the rows are sorted, `import_tags` skips names already
present. None of them can see what a writer running a FRENCH build of the app
actually reads on screen when they open a brand-new, empty-palette project
and click "Appliquer un préréglage…" twice.

That gap is the whole point of running this live. Three ways the wiring
could break the pure logic's promises without failing a single headless
test:

  * `tr!()` could resolve at the wrong locale, or a stale English literal
    could slip back into `work_tags.rs` — invisible to a test that never
    boots the i18n runtime;
  * the palette `ListView` is virtualised (its own module doc: "renders only
    the rows visible in its viewport, plus a buffer") and sits inside a
    `Switcher` whose inactive page is not even *built* until selected
    (`switcher.rs`: "Pending slots ... contribute zero work to the arena
    until visited") — so the empty-to-Basic transition exercises a
    lazy-mount + virtualised-scroll path no headless layout test walks;
  * `import_tags`'s skip-on-duplicate logic runs through the same
    undo/redo command whether it is handed 10 rows against an empty palette
    or 3 rows against Basic's 10 already there — a live re-apply is the only
    way to see the *rendered* list still reads 13 distinct rows, not 23 or a
    palette with "vaisseau" listed twice.

Asserts (every expected string/count is read from `presets.rs` and both
`tags.ftl` files, not guessed — see the constants below):

  1. a NEW project — created through the Launcher, never the checked-in
     fixture, since this probe both creates and mutates one — starts with
     an EMPTY palette: "Aucune étiquette pour le moment." AND zero rows,
     checked independently so a broken `Switcher` branch can't hide behind
     the other;
  2. applying "Basique" adds exactly its 10 rows, all in FRENCH, confirmed
     both by the toast's own numbers (10 added, 0 skipped) and by reading
     every row back off its delete control — the assertion that proves
     presets are translated at apply time, not shipped as English data;
  3. those 10 rows form a run of exactly 4 `statut/…` names, ADJACENT in
     alphabetical order — the live proof of presets.rs's own
     `the_status_prefix_clusters_when_sorted` unit test, this time through
     the real `ListView` + `WorkTagsListModel::sort_rows`, not the pure
     function it tests headlessly;
  4. applying "Science-fiction" afterwards adds ONLY its 3 extras (vaisseau,
     planète, organisation) — 13 rows total, no name repeated, Basic's 10
     exactly as they were;
  5. NOT asserted pass/fail — see the note below and the KNOWN GAP block
     printed at the end of the run.

A note on item 5, "ONE Ctrl+Z reverts the entire preset apply": the backend
guarantee is real — `tag_management/src/use_cases/import_tags_uc.rs` pushes
one `UndoRedoCommand` per `import_tags` call (snapshot/restore), and its own
header comment is explicit that this "must be a single Ctrl+Z". What is NOT
real, as of this probe, is a way to *reach* that undo from the keyboard. A
repo-wide search of `bastyde_ui/src` for `KeyStroke::ctrl(Key::Z)` (`app.rs`,
every file under `app/commands/`, `docks/outline.rs`, `panels/welcome.rs`)
returns zero hits, and `docks/search_replace_flow.rs`'s own module doc says
so outright: "App-level undo is not wired to any keystroke in `bastyde_ui`
... `Ctrl+Z` reaches only a focused editor's own local buffer undo." The two
preset-apply toasts in `work_tags.rs` (the add-row's and the empty-state's,
both `Toast::info(...).id("tags.preset")`) carry no `.action(...)` — unlike
`search_replace_flow.rs` and `view_models/trash.rs`, which both attach a real
Undo action to their own completion toast. So Ctrl+Z is, today, a dead key
for this feature: sending it and finding "nothing changed" would be
indistinguishable from a real regression, and this probe must not report
that as a pass. It DOES send Ctrl+Z anyway, with focus moved off every text
field first (so this cannot be confused with a `TextInput`'s own local
Ctrl+Z/Y — a real, separate binding, registered in the same breath as
Ctrl+A: `primitives/text_input_field.rs`), and prints what happened as a
diagnostic. The fix is a one-line `.action(...)` add to `work_tags.rs`'s two
toasts (mirroring `trash.rs`); this probe does not make that product change
unasked.

Run:  python3 scripts/automation_tag_presets.py
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
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import SCRATCH, assert_no_running_instance  # noqa: E402

# Before anything else: a live instance would swallow this launch (see the
# helper's docstring) and every later failure would name the wrong cause.
assert_no_running_instance()

# No `.skrib` fixture is opened here — this probe CREATES a project, so
# `working_copy()` doesn't apply. It owns and cleans up its own scratch
# subdirectory instead, following the same "never inside the repo" rule
# `automation_fixture` exists to enforce (belt-and-braces check below).
LOCATION_DIR = os.path.join(
    SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir(),
    f"probe-preset-newwork-{os.getpid()}",
)
os.makedirs(LOCATION_DIR, exist_ok=True)
_REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
assert not os.path.abspath(LOCATION_DIR).startswith(_REPO + os.sep), (
    f"the new project's location resolved inside the repo ({LOCATION_DIR}) — refusing"
)

WORK_NAME = "Preset Test"
WORK_SLUG = "preset-test"  # NewWorkViewModel::slugify("Preset Test")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name

# ── Locale-aware string table ────────────────────────────────────────────────
# The app follows the SYSTEM locale, which on this machine is FRENCH. Every
# user-visible string this probe matches on is a tuple of (english, french)
# spellings, with a comment naming the ftl key it came from
# (crates/bastyde_ui/locales/{en-US,fr-FR}/{main,tags}.ftl) — all read in
# full, not guessed.
WELCOME_NAV = "welcome sections"  # panels/welcome.rs: access_label_literal("Welcome sections") — not tr!, same in both locales
NEW_WORK_BTN = ("new work", "nouvelle œuvre", "nouvelle oeuvre")  # welcome-new-work
SEC_WORK = ("work", "œuvre", "oeuvre")  # settings-sec-work
PAGE_TAGS = ("tags", "étiquettes")  # settings-page-tags
TAGS_EMPTY = ("no tags yet", "aucune étiquette pour le moment")  # settings-tags-empty
APPLY_PRESET_BTN = ("apply a preset", "appliquer un préréglage")  # settings-tags-apply-preset (without the trailing ellipsis, to dodge glyph encoding)
BASIC_LABEL = ("basic", "basique")  # tags-preset-basic
SCIFI_LABEL = ("science fiction", "science-fiction")  # tags-preset-scifi — fr has a hyphen, en does not
# settings-tags-preset-applied is a flat interpolation, not a plural-select,
# so it is safe to match exactly for a specific (added, skipped) pair.
BASIC_TOAST = ("added 10, skipped 0 already present",
               "10 ajoutée(s), 0 déjà présente(s) ignorée(s)")
SCIFI_TOAST = ("added 3, skipped 10 already present",
               "3 ajoutée(s), 10 déjà présente(s) ignorée(s)")
TAGS_COUNT_10 = ("10 tags", "10 étiquettes")  # settings-tags-count [other]
TAGS_COUNT_13 = ("13 tags", "13 étiquettes")  # settings-tags-count [other]
DELETE_PREFIXES = (("delete ", "Delete "), ("supprimer ", "Supprimer "))  # settings-tags-delete = Delete { $name } / Supprimer { $name }

# Basic's 10 rows, French, in the EXACT order `WorkTagsListModel::sort_rows`
# renders them (case-insensitive compare, then exact — see
# models/work_tags_list_model.rs). Hand-derived from fr-FR/tags.ftl, not
# guessed: 'à'/'é' (U+00E0/U+00E9) sort after every ASCII letter, which is
# what pulls "statut/à relire" to the end of its own run and "vérifier la
# continuité" to the very end of the list.
EXPECTED_BASIC_ORDER = [
    "lieu", "objet", "personnage", "point d'intrigue", "recherches à faire",
    "statut/brouillon", "statut/plan", "statut/terminé", "statut/à relire",
    "vérifier la continuité",
]
EXPECTED_BASIC_SET = set(EXPECTED_BASIC_ORDER)

# The 13-row order after Sci-fi is applied on top: "organisation" slots
# between objet/personnage, "planète" between personnage/point d'intrigue,
# "vaisseau" immediately before "vérifier..." ('a' < 'é').
EXPECTED_SCIFI_ORDER = [
    "lieu", "objet", "organisation", "personnage", "planète",
    "point d'intrigue", "recherches à faire",
    "statut/brouillon", "statut/plan", "statut/terminé", "statut/à relire",
    "vaisseau", "vérifier la continuité",
]
SCIFI_EXTRA = {"vaisseau", "planète", "organisation"}


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
    """One launched app + connected MCP server."""

    def __init__(self, args):
        # Set before anything that can fail, so `fail()`'s teardown never trips
        # over a half-built Session and hides the real error.
        self.mcp = None
        self.app = None
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT)
        sock = tok = None
        deadline = time.time() + 25
        while time.time() < deadline:
            txt = open(self.log).read()
            a = re.search(r"bridge socket = (\S+)", txt)
            b = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
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
                                  "clientInfo": {"name": "tag-presets", "version": "1"}})
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
            t = txt.strip()
            # Most ops return a JSON object (`snapshot_tree` -> {"nodes": [...]});
            # `list_windows` returns a bare JSON array. Accept either.
            p = json.loads(t) if (t.startswith("{") or t.startswith("[")) else {}
        return res, p

    def nodes(self):
        p = self.call("snapshot_tree")[1]
        return p.get("nodes", []) if isinstance(p, dict) else []

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(l for l in self.labels()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def wait_role(self, role, timeout=20):
        """Poll until a node of `role` exists. Returns True, or False on timeout.

        A modal is NOT queryable the instant its opening click returns. Measured
        on the New Work modal: `invoke_action(click)` answers in 0.0s, then
        `snapshot_tree` times out for ~4-6s while the overlay animates, and only
        then reports the modal — 196 nodes including the `Form` landmark.

        A one-shot check after a fixed `sleep(0.8)` therefore reports "the modal
        did not open" for a modal that opens perfectly a few seconds later. That
        cost a full diagnostic detour chasing a `Form`-landmark regression that
        does not exist. Same failure shape `wait_for_load` exists to prevent, one
        level in: wait for the thing, never sleep-and-hope.
        """
        end = time.time() + timeout
        while time.time() < end:
            if any(n.get("role") == role for n in self.nodes()):
                return True
            time.sleep(0.5)
        return False

    def list_windows(self):
        _, p = self.call("list_windows")
        return p if isinstance(p, list) else []

    def settle(self):
        try:
            self.call("settle")
        except Exception:
            time.sleep(0.3)

    def shot(self, path):
        res, _ = self.call("screenshot", {})
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")

    def stop(self):
        """Terminate and *wait*.

        Waiting is not politeness. Skribisto is one process per project, guarded
        by an open-registry lock file: relaunching on the same path while the
        old process still holds the lock makes the new one hand off to it and
        exit immediately — which surfaces as "app exited before printing the
        bridge socket" and reads like a crash.
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
        # dying process; give it a beat before anything else claims it.
        time.sleep(2.0)


def text_of(n):
    """A node's visible text: Label-role nodes carry theirs in `value`."""
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
    if not (isinstance(b, dict) and "x" in b):
        return False
    s.call("inject_pointer", {"x": b["x"] + (dx if dx is not None else b.get("width", 0) / 2),
                              "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    return True


def node_text(n):
    """A node's visible text, wherever it lives: `Label`-role nodes carry
    theirs in `value`, everything else in `label`."""
    if not n:
        return ""
    return ((n.get("value") or "") + " " + (n.get("label") or "")).strip()


def val(n):
    """`TextInput`/preview text lives in `value` (AccessKit), not `label`."""
    return "" if n is None or n.get("value") is None else str(n.get("value"))


def joined(s):
    """Every label *and* value in the tree, lowercased — a labels-only scan
    misses a tag's own name (the value of its rename `TextInput`) and the
    toast's interpolated text (the value of a plain `TextWidget`)."""
    parts = []
    for n in s.nodes():
        for key in ("label", "value"):
            v = n.get(key)
            if isinstance(v, str) and v.strip():
                parts.append(v)
    return " | ".join(parts).lower()


def has_any(s, variants):
    j = joined(s)
    return any(v.lower() in j for v in variants)


def node_enabled(s, node_id):
    _, p = s.call("read_node", {"node": node_id})
    if isinstance(p, dict):
        if p.get("disabled") is True:
            return False
        if "enabled" in p:
            return bool(p["enabled"])
    return True


def dump(s, title):
    print(f"--- AT tree: {title} ---")
    for n in s.nodes():
        lab = (n.get("label") or "").strip()
        v = n.get("value")
        if lab or v:
            b = n.get("bounds") or {}
            print(f"  [{n.get('role')}] label={lab!r} value={v!r} "
                  f"@x={b.get('x')} y={b.get('y')}")


def binder_tree_rows(s):
    """The binder's own tree rows: role "Unknown" at x≈48.

    The x window is narrow on purpose (learned on `automation_tags.py`):
    `x < 120` alone also sweeps up the activity bar (Binder / Search / Trash /
    Settings) and the hamburger menu — clicking through those by accident
    would close the project and every later assertion would silently be
    measuring the Launcher instead.
    """
    return [n for n in s.nodes()
            if n.get("role") == "Unknown"
            and 40 <= (n.get("bounds") or {}).get("x", 9999) <= 140
            and (n.get("label") or "").strip()]


def rail_node(s, variants, exact=False):
    """First node in the left settings rail (x < 280) matching any variant.

    The rail bound matters: without it a page name like "Tags" also matches
    the Inspector's own tag section in the main window behind the modal.
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


def breadcrumb(s):
    return [n.get("label") for n in s.nodes() if n.get("role") == "Link"]


def settings_open(s):
    # The instant-apply footer is on every pane, so it survives whichever
    # page happens to be showing.
    return (has_any(s, ("reset to defaults", "réinitialiser"))
            and has_any(s, ("done", "terminé")))


def open_settings(s):
    for args in ({"key": ",", "ctrl": True}, {"key": "Comma", "ctrl": True}):
        s.call("inject_key", args)
        for _ in range(10):
            time.sleep(0.4)
            if settings_open(s):
                return True
    return False


def page_reached(s, target):
    names = (target,) if isinstance(target, str) else tuple(target)
    crumb = breadcrumb(s)
    if crumb and any(c and c.strip().lower() in names for c in crumb):
        return crumb
    row = rail_node(s, names, exact=True)
    if row and row.get("selected"):
        return crumb or list(names[:1])
    return None


def select_page(s, target, anchor_node, steps=14):
    """Select a rail page by walking to it (keyboard) from a visible anchor
    row — a row far enough down the rail is laid out below the scroll
    viewport, so a pointer click at its reported bounds lands on empty
    chrome. `bastyde` scrolls the focused row into view on arrow-key
    navigation, so: click a row that IS visible to seed focus, then step.
    Tries both directions since `anchor_node`'s position relative to the
    target is not known ahead of time.
    """
    got = page_reached(s, target)
    if got:
        return got
    if not anchor_node:
        return None
    for key in ("Down", "Up"):
        click(s, anchor_node.get("bounds") or {})
        s.settle()
        time.sleep(0.5)
        for _ in range(steps):
            got = page_reached(s, target)
            if got:
                return got
            s.call("inject_key", {"key": key})
            s.settle()
            time.sleep(0.35)
    return page_reached(s, target)


def read_all_tag_rows(s, max_passes=10):
    """Every tag name in the palette, read off each row's own delete control.

    Not the row's name `TextInput` or the swatch: the delete `IconButton`'s
    tooltip DOUBLES AS its accessible name (`IconButton::tooltip`'s own doc:
    "the tooltip text doubles as the AT name for icon-only buttons"), it is
    exactly `Delete { $name }` / `Supprimer { $name }`, and there is exactly
    one per row — so the remainder after the fixed prefix is the tag name,
    no further disambiguation needed. This is the same trick
    `automation_languages.py` uses for a pill's "Remove X" control rather
    than the pill's own (differently-worded) label.

    The list is virtualised (`list_view.rs`'s own module doc: renders only
    the rows visible in its viewport, plus a small buffer) and, before the
    first tag exists, is not even MOUNTED: the palette card wraps it in a
    `Switcher` whose inactive page "contribute[s] zero work to the arena
    until visited" (`switcher.rs`). So: no `ListBox` node at all means a
    genuinely empty, not-yet-mounted list — return `[]` rather than
    crashing on a missing node. Once it exists, scroll to the very top, then
    walk down in less-than-a-viewport steps, collecting every row's name as
    it enters the realized window; a `seen` dict makes re-reading the same
    row across overlapping windows harmless, and two consecutive passes with
    nothing new is the signal that the rest has already been seen (the step
    is smaller than the viewport, so a genuinely new row can never be
    skipped between two reads).
    """
    lst = next((n for n in s.nodes() if n.get("role") == "ListBox"), None)
    if not lst:
        return []
    # Scroll to the top first. `automation_tag_chips.py`'s `scroll_for_dot_row`
    # established live that a NEGATIVE dy reveals content further down the
    # pane — so a large POSITIVE dy walks back up to the very start.
    for _ in range(4):
        s.call("scroll", {"node": lst["id"], "dx": 0, "dy": 2000})
        s.settle()
        time.sleep(0.2)

    seen = {}
    stall = 0
    for _ in range(max_passes):
        before = len(seen)
        for n in s.nodes():
            if n.get("role") != "Button":
                continue
            lab = (n.get("label") or "").strip()
            low = lab.lower()
            name = None
            for low_prefix, cased_prefix in DELETE_PREFIXES:
                if low.startswith(low_prefix):
                    name = lab[len(cased_prefix):]
                    break
            if name and name not in seen:
                b = n.get("bounds") or {}
                seen[name] = b.get("y", 0)
        if len(seen) == before:
            stall += 1
            if stall >= 2:
                break
        else:
            stall = 0
        # LIST_MIN_HEIGHT is 320dp (work_tags.rs); a step smaller than that
        # keeps consecutive windows overlapping, so nothing between two reads
        # is skipped.
        s.call("scroll", {"node": lst["id"], "dx": 0, "dy": -200})
        s.settle()
        time.sleep(0.2)
    return [name for name, _y in sorted(seen.items(), key=lambda kv: kv[1])]


def wait_toast(s, variants, timeout=4):
    end = time.time() + timeout
    while time.time() < end:
        if has_any(s, variants):
            return True
        time.sleep(0.2)
    return False


def apply_preset(s, label_variants, toast_variants, what):
    """Open the "Apply a preset…" popover, pick the item matching
    `label_variants`, and wait for its confirmation toast."""
    btn = find(s, APPLY_PRESET_BTN, role="Button")
    if not btn:
        dump(s, "no Apply-a-preset button")
        fail(f"no 'Apply a preset…' button (looked for {APPLY_PRESET_BTN})", s)
    # `invoke_action` rather than a synthesised pointer click: a near-miss
    # click lands on a neighbouring control and opens ITS tooltip, leaving
    # the popover shut — which reads exactly like "the popover is empty".
    res, _ = s.call("invoke_action", {"node": btn["id"], "action": "click"})
    if isinstance(res, dict) and res.get("isError"):
        click(s, btn.get("bounds") or {})
    s.settle()
    time.sleep(0.8)

    item = find(s, label_variants, role="MenuItem")
    if not item:
        offered = [f"{n.get('role')}:{text_of(n).strip()}" for n in s.nodes()
                   if n.get("role") == "MenuItem" and text_of(n).strip()]
        print(f"  popover offered {len(offered)} item(s): {offered}")
        fail(f"no {what} item in the preset popover (looked for {label_variants})", s)

    res, _ = s.call("invoke_action", {"node": item["id"], "action": "click"})
    if isinstance(res, dict) and res.get("isError"):
        click(s, item.get("bounds") or {})
    s.settle()
    time.sleep(1.0)

    if not wait_toast(s, toast_variants, timeout=4):
        statuses = [(n.get("role"), node_text(n)) for n in s.nodes()
                    if n.get("role") in ("Status", "Alert") and node_text(n).strip()]
        fail(f"applying {what} never showed the toast {toast_variants!r} — "
             f"status/alert nodes present: {statuses}", s)
    print(f"  {what} applied: toast confirms {toast_variants[1]!r}")


def find_value_contains(s, substr, timeout=6):
    end = time.time() + timeout
    while time.time() < end:
        for n in s.nodes():
            if substr.lower() in val(n).lower():
                return n
        time.sleep(0.3)
    return None


# ── 0. Launch bare, create a NEW project through the Launcher ───────────────
print("== launch (bare, to reach the Launcher) ==")
s = Session([])
if not s.wait_label(WELCOME_NAV):
    fail(f"the Launcher window did not appear at startup (looked for {WELCOME_NAV!r})", s)
print("Launcher window is up.")

print("\n== open New Work from the Launcher ==")
new_work_btn = find(s, NEW_WORK_BTN, role="Button")
if not new_work_btn:
    dump(s, "no New Work button")
    fail(f"no Launcher 'New Work' button (looked for {NEW_WORK_BTN})", s)
res, _ = s.call("invoke_action", {"node": new_work_btn["id"], "action": "click"})
if isinstance(res, dict) and res.get("isError"):
    click(s, new_work_btn.get("bounds") or {})
if not s.wait_role("Form", timeout=30):
    dump(s, "New Work modal did not open")
    fail("the New Work modal did not open (no Form landmark)", s)
print("  New Work modal open.")

# The modal must open with the Work-name field focused (`NewWorkPanel`'s own
# `initial_focus_hint`). Reading it off `focus` is also how the name field is
# IDENTIFIED — the Launcher's own "Search works" box sits behind the modal
# and is an empty TextInput too, so a first-match scan would risk it instead.
_, snap = s.call("snapshot_tree")
focus_id = snap.get("focus") if isinstance(snap, dict) else None
name_field = next((n for n in s.nodes() if n.get("id") == focus_id), None)
if name_field is None or name_field.get("role") != "TextInput":
    fail(f"New Work should open with the Work name field focused, got "
         f"{(name_field or {}).get('role')!r} (id={focus_id})", s)
# Defensive per the harness's rule 4, even though this field is already
# focused: `invoke_action(focus)` first, always, before typing.
s.call("invoke_action", {"node": name_field["id"], "action": "focus"})
time.sleep(0.2)

# Location is the OTHER TextInput holding a path — excluding the (empty)
# focused name field is what stops the Launcher's own search box, sitting
# behind the modal, from being mistaken for it.
text_fields = [n for n in s.nodes() if n.get("role") in ("TextInput", "TextField")]
location_field = next((n for n in text_fields
                       if "/" in val(n) and n.get("id") != name_field["id"]), None)
if location_field is None:
    fail("could not identify the Location field (no other TextInput holds a path)", s)
print(f"  name field id={name_field['id']}; default location={val(location_field)!r}")

res, _ = s.call("type_text", {"node": name_field["id"], "text": WORK_NAME})
if isinstance(res, dict) and res.get("isError"):
    fail(f"type_text into the name field errored: {res}", s)
time.sleep(0.6)

# `type_text` synthesises real keystrokes at the caret — it does NOT replace
# the field's content. Appending onto the default $HOME would point Create
# at a folder that was never made writable/existent for this run, so clear
# first: Ctrl+A (a real bound shortcut in every TextInput) then Backspace.
s.call("invoke_action", {"node": location_field["id"], "action": "focus"})
time.sleep(0.2)
# Settle BETWEEN the two keys, not just after: firing Backspace before the
# Ctrl+A selection has been dispatched and applied would delete one
# character instead of the whole field (`automation_welcome_search.py`'s
# proven Ctrl+A/Backspace idiom settles after each key for the same reason).
s.call("inject_key", {"key": "a", "ctrl": True})
s.settle()
time.sleep(0.2)
s.call("inject_key", {"key": "Backspace"})
s.settle()
time.sleep(0.2)
res, _ = s.call("type_text", {"node": location_field["id"], "text": LOCATION_DIR})
if isinstance(res, dict) and res.get("isError"):
    fail(f"type_text into the location field errored: {res}", s)
time.sleep(0.6)

loc_now = next((n for n in s.nodes() if n.get("id") == location_field["id"]), None)
if val(loc_now) != LOCATION_DIR:
    fail(f"Location field should read {LOCATION_DIR!r} after clear+type, got "
         f"{val(loc_now)!r} — the Ctrl+A/Backspace may have landed on the "
         f"wrong focused widget", s)
print(f"  location set to {LOCATION_DIR!r}")

preview = find_value_contains(s, ".skrib", timeout=6)
if preview is None or not val(preview).lower().endswith(f"{WORK_SLUG}.skrib"):
    fail(f"path preview should end with '…/{WORK_SLUG}.skrib' (proves the typed "
         f"name reached the VM's signal, not just the AccessKit value), got "
         f"{val(preview) if preview else None!r}", s)
print(f"  path preview: {val(preview)!r}")

create_btn = None
for n in s.nodes():
    lab = (n.get("label") or "").lower()
    if any(w in lab for w in ("create work", "créer", "creer")) and not any(
            w in lab for w in ("cancel", "annuler")):
        create_btn = n
        break
if not create_btn:
    fail("no 'Create Work' button", s)
# Non-vacuity: if Create is disabled the click below is a silent no-op and
# every assertion downstream would be measuring nothing.
if not node_enabled(s, create_btn["id"]):
    fail("Create Work should be ENABLED with a valid name and an existing, "
         "writable location", s)
print("  Create Work is enabled.")
s.shot("/tmp/tag-presets-new-work.png")

res, _ = s.call("invoke_action", {"node": create_btn["id"], "action": "click"})
if isinstance(res, dict) and res.get("isError"):
    click(s, create_btn.get("bounds") or {})
print("  clicked Create Work — waiting for the project window …")

# Creating from the Launcher opens a SECOND (project) window carrying the
# deferred `PendingAction::New`, THEN closes the Launcher
# (`NewWorkViewModel::create`, the `Some(factory)` branch). Poll for the
# transition the way `automation_welcome.py` proved live for opening an
# existing project: the Launcher's own nav landmark disappears and a real
# binder row shows up — locale-independent, unlike an English-only "binder"
# substring check, which would be a locale bug on this French machine.
end = time.time() + 20
transitioned = False
while time.time() < end:
    if s.app.poll() is not None:
        fail("the app process exited while creating the work from the "
             "Launcher (a wrong open/close window ordering quits it)", s)
    joined_now = " | ".join(l for l in s.labels()).lower()
    if WELCOME_NAV not in joined_now and binder_tree_rows(s):
        transitioned = True
        break
    time.sleep(0.4)
if not transitioned:
    dump(s, "creation never transitioned")
    fail("creating the work never transitioned into the project window "
         "(no binder rows appeared once the Launcher nav was gone)", s)

windows = s.list_windows()
tries = 0
while len(windows) != 1 and tries < 15:
    time.sleep(0.3)
    windows = s.list_windows()
    tries += 1
if len(windows) != 1:
    fail(f"expected exactly one window once the Launcher closes, got {windows}", s)
print(f"  project window is up ({len(binder_tree_rows(s))} binder row(s)); "
      f"windows settled to {len(windows)}")
s.shot("/tmp/tag-presets-project.png")

# ── Settings ▸ Work ▸ Tags ───────────────────────────────────────────────────
print("\n== open Settings ▸ Work ▸ Tags ==")
if not open_settings(s):
    dump(s, "Settings never opened")
    fail("Settings did not open (Ctrl+,)", s)
work = rail_node(s, SEC_WORK)
if not work:
    dump(s, "no Work section")
    fail(f"the rail has no Work section (looked for {SEC_WORK})", s)
tags_row = rail_node(s, PAGE_TAGS, exact=True)
if not tags_row:
    # Only needed if the section ever ships collapsed; tap the chevron once.
    click(s, work.get("bounds") or {}, dx=10)
    for _ in range(12):
        s.settle()
        time.sleep(0.4)
        tags_row = rail_node(s, PAGE_TAGS, exact=True)
        if tags_row:
            break
if not tags_row:
    dump(s, "Work section did not expand")
    fail("no Tags page under the Work section", s)
crumb = select_page(s, PAGE_TAGS, anchor_node=work)
if not crumb:
    dump(s, "Tags page never selected")
    s.shot("/tmp/tag-presets-select-fail.png")
    fail("could not select the Tags page", s)
print(f"  Tags page open (breadcrumb: {crumb})")

# ── 1. A NEW project starts with an EMPTY palette ────────────────────────────
print("\n== 1. a new project starts with an EMPTY palette ==")
if not has_any(s, TAGS_EMPTY):
    dump(s, "no empty-palette note")
    fail(f"expected the empty-palette note (looked for {TAGS_EMPTY})", s)
rows0 = read_all_tag_rows(s)
if rows0:
    fail(f"a brand-new project should have 0 tags, found {len(rows0)}: {rows0}", s)
print(f"  confirmed: {TAGS_EMPTY[1]!r} shown, and independently, 0 rows read")
s.shot("/tmp/tag-presets-empty.png")

# ── 2. Applying "Basique" adds its 10 tags, in FRENCH ────────────────────────
print("\n== 2. applying 'Basique' adds its 10 tags, in FRENCH ==")
apply_preset(s, BASIC_LABEL, BASIC_TOAST, "Basic/Basique")
rows_basic = read_all_tag_rows(s)
print(f"  rows after Basic: {rows_basic}")
if len(rows_basic) != 10:
    fail(f"expected exactly 10 rows after Basic, got {len(rows_basic)}: {rows_basic}", s)
got_set = {r.lower() for r in rows_basic}
if got_set != EXPECTED_BASIC_SET:
    fail(f"Basic's tag set does not match — missing={EXPECTED_BASIC_SET - got_set} "
         f"unexpected={got_set - EXPECTED_BASIC_SET}", s)
if not has_any(s, TAGS_COUNT_10):
    fail(f"toolbar count did not update to 10 (looked for {TAGS_COUNT_10})", s)
print(f"  all 10 French names present: {sorted(got_set)}")
s.shot("/tmp/tag-presets-basic.png")

# ── 3. The four "statut/…" rows are adjacent, alphabetically ────────────────
print("\n== 3. the four 'statut/…' rows are adjacent, alphabetically ==")
if len(rows_basic) != 10:
    fail(f"cannot check adjacency: expected 10 rows, have {len(rows_basic)}", s)
if rows_basic != EXPECTED_BASIC_ORDER:
    fail("Basic's rendered order does not match the expected alphabetical "
         f"order.\n  got:      {rows_basic}\n  expected: {EXPECTED_BASIC_ORDER}", s)
status_slice = rows_basic[5:9]
if not all(name.startswith("statut/") for name in status_slice):
    fail(f"rows[5:9] should be the four 'statut/…' rows, got {status_slice}", s)
if any(name.startswith("statut/") for name in rows_basic[:5] + rows_basic[9:]):
    fail(f"a 'statut/…' row exists outside the contiguous run: {rows_basic}", s)
print(f"  statut/… run is contiguous at positions 6-9: {status_slice}")

# ── 4. Applying "Science-fiction" adds ONLY its 3 extras ────────────────────
print("\n== 4. applying 'Science-fiction' adds ONLY its 3 extras ==")
apply_preset(s, SCIFI_LABEL, SCIFI_TOAST, "Sci-fi/Science-fiction")
rows_scifi = read_all_tag_rows(s)
print(f"  rows after Sci-fi: {rows_scifi}")
if len(rows_scifi) != 13:
    fail(f"expected exactly 13 rows after Sci-fi, got {len(rows_scifi)}: {rows_scifi}", s)
if len(rows_scifi) != len(set(r.lower() for r in rows_scifi)):
    dupes = sorted({r for r in rows_scifi if rows_scifi.count(r) > 1})
    fail(f"re-applying produced a duplicate — {dupes}", s)
added = {r.lower() for r in rows_scifi} - EXPECTED_BASIC_SET
if added != SCIFI_EXTRA:
    fail(f"Sci-fi's added set should be exactly {SCIFI_EXTRA}, got {added}", s)
if rows_scifi != EXPECTED_SCIFI_ORDER:
    fail("Sci-fi's rendered order does not match.\n"
         f"  got:      {rows_scifi}\n  expected: {EXPECTED_SCIFI_ORDER}", s)
if not has_any(s, TAGS_COUNT_13):
    fail(f"toolbar count did not update to 13 (looked for {TAGS_COUNT_13})", s)
print(f"  exactly the 3 extras were added: {sorted(SCIFI_EXTRA)}; Basic's 10 untouched")
s.shot("/tmp/tag-presets-scifi.png")

# ── 5. Ctrl+Z — KNOWN GAP, see the module docstring ──────────────────────────
print("\n== 5. ONE Ctrl+Z reverting the preset apply — KNOWN GAP ==")
print(
    "  Not asserted pass/fail: bastyde_ui registers zero KeyStroke::ctrl(Key::Z)\n"
    "  anywhere, and docks/search_replace_flow.rs's own module doc says outright\n"
    "  that app-level undo is not wired to any keystroke. work_tags.rs's two\n"
    "  preset-apply toasts carry no .action(...), unlike trash.rs and\n"
    "  search_replace_flow.rs, which both attach a real Undo action to their\n"
    "  own toast. The backend guarantee (import_tags_uc.rs: one UndoRedoCommand\n"
    "  per apply) is real; there is simply no UI path to invoke it yet. The fix\n"
    "  is a one-line .action(...) add to work_tags.rs mirroring trash.rs — a\n"
    "  product change this probe does not make unasked. What follows is a live\n"
    "  demonstration of the current (dead-key) behaviour, not a test of it."
)
# Move focus off every field first, so this cannot be mistaken for a
# TextInput's own local Ctrl+Z (a real, separate binding — see the module
# docstring). Clicking the (non-interactive) toolbar count label moves focus
# there without activating anything.
count_lbl = next((n for n in s.nodes() if n.get("role") == "Label"
                  and any(v in node_text(n).lower() for v in TAGS_COUNT_13)), None)
if count_lbl:
    click(s, count_lbl.get("bounds") or {})
    s.settle()
    time.sleep(0.3)
before_ctrl_z = read_all_tag_rows(s)
s.call("inject_key", {"key": "z", "ctrl": True})
s.settle()
time.sleep(1.0)
after_ctrl_z = read_all_tag_rows(s)
if sorted(after_ctrl_z) == sorted(before_ctrl_z):
    print(f"  observed live: Ctrl+Z left the palette unchanged ({len(after_ctrl_z)} "
          f"rows) — matches the static-analysis finding above; this is the "
          f"product gap to file, not a probe failure")
else:
    # If this ever fires, the KNOWN GAP note above is stale — something changed
    # (for better or worse). Flag it loudly either way rather than silently
    # accepting a different row count as fine.
    fail(f"UNEXPECTED: the palette changed after Ctrl+Z ({len(before_ctrl_z)} -> "
         f"{len(after_ctrl_z)} rows: {before_ctrl_z} -> {after_ctrl_z}) — the "
         f"KNOWN GAP note in this script's docstring may now be stale; "
         f"re-verify whether Ctrl+Z undo is wired before trusting this either "
         f"as a regression or as the feature landing", s)
if not settings_open(s):
    print("  note: Settings closed or changed after Ctrl+Z — worth checking "
          "nothing unrelated fired on that key")

s.stop()
print(
    "\nOK — presets apply, translate to French, and dedupe on re-apply, all "
    "verified live against the real Settings ▸ Work ▸ Tags pane on a "
    "freshly-created project. Item 5 (Ctrl+Z) is a documented product gap, "
    "not asserted pass/fail — see the KNOWN GAP block above and the module "
    "docstring."
)
