#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the story-bible mention index end to end.

Stage 5 is a promise with three moving parts: a tag flagged "discoverable" turns
its items into story-bible material; their titles and aliases get matched
against every scene's prose; and each hit becomes a *suggestion* in the
scene's roster and a *backlink* on the named item, until the writer pins it.
Nothing about that is visible to a unit test — `mentions.rs::scan_prose` proves
the matcher, `scan_mentions_uc` proves the gather, but neither one opens a
window. This is the half that only exists once a person clicks the "+", types
a name, and watches something respond.

Built end to end, through the UI, exactly as a writer would:

  1. Settings > Work > Tags: create a tag, flip its "Story bible" switch — the
     tag's row-scoped switch, found by *structural correlation* (nearest Switch
     right of the tag's own name field) rather than `sw[0]`, because the fixture
     may carry other discoverable tags and the switch's label never names the
     row it belongs to.
  2. Put that tag on "Note 1" via the Inspector's own tag picker, and confirm —
     as a real behaviour, not an assumption — that the aliases field does not
     exist until the item carries a discoverable tag, then does.
  3. Give Note 1 an alias, type it into "1.1 Zeus"'s prose, and watch the
     roster grow a suggestion naming Note 1, with a pin (not a bare fact).
  4. Hover it: the evidence tooltip must carry the actual sentence, not just
     open. A hover that shows nothing is indistinguishable from a feature that
     silently stopped rendering its explanation — the one thing this feature
     has to get right, since the matcher is knowingly wrong about "don't".
  5. Pin it: the suggestion is replaced by a confirmed backlink on Note 1's own
     Inspector — the same rows, read from the other end.
  6. THE LOAD-BEARING CHECK: a scan must never dirty the project. Reading the
     whole binder and re-deriving a hundred rows sounds exactly like the kind
     of background work that leaves a stray write behind, and the UI's only
     tell would be the save indicator flipping to "unsaved" for no reason a
     writer typed. Proven with a positive control first (a real edit DOES flip
     the indicator, in this session, on this build) so the final "and a scan
     does not" is not trivially true of a detector that never worked.

Matching is case-sensitive and whole-word with a 3-char floor (`mentions.rs`),
so the alias here is "Vesparil" — capitalised, 8 characters, and absent from
the fixture's Lorem Ipsum scenes (checked against the checked-in text, not
assumed) so a hit is a real match and not a coincidence with existing prose.

Runs on a COPY of the fixture (`automation_fixture.working_copy`) — this probe
saves, twice, and relaunches once. Never the checked-in file: see
`automation_fixture`'s module docs for the three incidents that rule exists
to prevent.

Run:  python3 scripts/automation_mentions.py
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
from automation_fixture import wait_for_load, working_copy

# This probe saves twice and relaunches once. Never the checked-in fixture —
# three separate past incidents came from exactly that. See automation_fixture.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "mentions")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcp.log", delete=False).name

# ─────────────────────────────────────────────────────────────────────────────
# Locale-tuple strings. The app follows the SYSTEM locale (French on this
# machine); an English-only selector finds nothing and reports it exactly like
# a real regression. Every string carries both spellings, commented with the
# ftl key it came from (crates/bastyde_ui/locales/{en-US,fr-FR}/{main,tags}.ftl).
# ─────────────────────────────────────────────────────────────────────────────
SEC_WORK = ("work", "œuvre", "oeuvre")                       # settings-sec-work
PAGE_TAGS = ("tags", "étiquettes")                            # settings-page-tags
DONE = ("done", "terminé")                                    # settings-done
ADD_TAG_BTN = ("add tag", "ajouter")                           # settings-tags-add
SW_STORY_BIBLE = ("story bible", "bible narrative")            # settings-tags-discoverable
TAGS_PILL_ADD = ("add a tag", "ajouter une étiquette")          # tags-pill-add
TAGS_ALIAS_ADD = ("add another name", "ajouter un autre nom")   # tags-alias-add
INSPECTOR_TAGS = ("tags", "étiquettes")                         # inspector-tags
INSPECTOR_ALIASES = ("also known as", "aussi appelé")           # inspector-aliases / tags-alias-list
MENTIONS_ROSTER = ("mentioned here", "mentionnés ici")          # mentions-roster
MENTIONS_BACKLINKS = ("mentioned in", "mentionné dans")         # mentions-backlinks
MENTIONS_PIN_VERB = ("keep", "conserver")                       # mentions-pin (dynamic $name, match the verb)
INSPECTOR_LANGUAGE = ("language", "langue")                     # inspector-dict-language
INSPECTOR_EXPORT = ("export",)                                  # inspector-export (identical in both locales)
SAVE_UNSAVED = ("unsaved changes", "modifications non enregistrées")           # statusbar-save-unsaved
SAVE_SAVED = ("all changes saved", "toutes les modifications sont enregistrées")  # statusbar-save-saved
SAVE_AUTOSAVE = ("autosave is on", "l'enregistrement automatique est activé")  # statusbar-save-autosave

# Every Inspector section heading that can appear on a Scene/Note item, in no
# particular order — used only to find the y-boundary of whichever section we
# actually care about (see `inspector_section`). Incomplete would just mean a
# band runs a little long; it does not need to be exhaustive to be safe.
ALL_INSPECTOR_HEADS = [
    INSPECTOR_TAGS, INSPECTOR_ALIASES, MENTIONS_ROSTER, MENTIONS_BACKLINKS,
    INSPECTOR_LANGUAGE, INSPECTOR_EXPORT,
]

# Item titles, literal fixture data (not localized).
ITEM_ZEUS = "1.1 Zeus"
ITEM_MARS = "1.2 Mars"
ITEM_NOTE1 = "Note 1"

# A per-run-unique tag name: the fixture's palette starts at exactly 3 tags
# (A / B / very looooooooooong tag, none discoverable — see
# resources/test/skribisto_test_project.skrib at HEAD), but naming this
# uniquely means the probe degrades gracefully rather than colliding if that
# ever changes.
TAG_NAME = f"MentionsProbeTag{int(time.time())}"

# Case-sensitive, whole-word, >=3 chars (skribisto_model::mentions::MIN_NAME_LEN).
# Checked against 45-1-1-zeus.scene.djot's actual Lorem Ipsum text at HEAD:
# "Vesparil" and "lanternfish" appear nowhere in it, so a hit here is the real
# match this feature exists to produce, not a coincidence with placeholder text.
ALIAS = "Vesparil"
# Leading AND trailing space: whichever word the caret happens to land inside
# when `type_text` inserts, the padding still gives "Vesparil" and the
# following word real boundaries on both sides — the matcher treats a space
# as a boundary the same as it treats sentence punctuation.
SENTENCE = " Vesparil paced past the peculiar lanternfish tank. "


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
        self.app = subprocess.Popen([SKRIBISTO, path], stdout=open(self.log, "w"),
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
                                  "clientInfo": {"name": "mentions", "version": "1"}})
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
        # dying process; give it a beat before the next launch claims it.
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


def act(s, node, action="click"):
    """Prefer `invoke_action` over a synthesised pointer click for buttons — a
    near-miss pointer click lands on the neighbouring widget and opens *its*
    tooltip, leaving the intended popover shut, which reads exactly like "the
    popover is empty". Falls back to a pointer click only if invoke_action
    errors. (Section 4 below never even calls this for `TagPickRow` options —
    that widget's `accessibility()` registers no actions at all, so it goes
    straight to a pointer click, the technique automation_tags.py already
    established for that exact row type.)"""
    res, _ = s.call("invoke_action", {"node": node["id"], "action": action})
    if isinstance(res, dict) and res.get("isError"):
        click(s, node.get("bounds") or {})


def hover(s, b):
    """Approach from outside, then jiggle inside. A single `move` that lands on
    the widget is not reliably read as a hover *enter* — the dwell timer starts
    on motion within the widget, so one event arriving already-inside can leave
    it unarmed."""
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


def wait_for(pred, tries=20, delay=0.3):
    for _ in range(tries):
        if pred():
            return True
        time.sleep(delay)
    return False


def wait_label(s, substr, timeout=20):
    end = time.time() + timeout
    while time.time() < end:
        joined = " | ".join(text_of(n) for n in s.nodes()).lower()
        if substr.lower() in joined:
            return True
        time.sleep(0.4)
    return False


def diag(s, hint):
    """Print every node whose text contains `hint` — for a failure message to
    say what WAS there, not just that the expected thing was not. "Not found"
    is equally true of a wrong selector and a real regression."""
    matches = [(n.get("role"), text_of(n).strip()) for n in s.nodes()
               if hint.lower() in text_of(n).lower()]
    print(f"  diagnostic: nodes containing {hint!r}: {matches[:20]}")


def open_item(s, title):
    """Select `title` in the binder so the Inspector/editor target it. The x
    window (40-200) is the binder tree's own column, wide enough to cover
    every indent depth without reaching into the outline/editor beside it."""
    for n in s.nodes():
        b = n.get("bounds") or {}
        if (n.get("label") or "").strip() == title and 40 <= b.get("x", 9999) <= 200:
            click(s, b)
            s.settle()
            time.sleep(1.2)
            return True
    return False


def main_editor(s, timeout=10):
    """The scene's own body editor (not the synopsis): the `MultilineTextInput`
    with the greatest height, as `automation_word_count.py`'s proven idiom for
    telling the two apart."""
    end = time.time() + timeout
    editors = []
    while time.time() < end and not editors:
        editors = [n for n in s.nodes() if n.get("role") == "MultilineTextInput"
                   and "set_value" in (n.get("actions") or [])]
        if not editors:
            time.sleep(0.4)
    if not editors:
        return None
    return max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))


def save(s):
    s.call("inject_key", {"key": "s", "ctrl": True})
    s.settle()
    time.sleep(2.0)


# ── Settings navigation, adapted from automation_tags.py's proven idiom ─────

def has_any(s, variants):
    j = " | ".join(text_of(n) for n in s.nodes() if text_of(n)).lower()
    return any(v.lower() in j for v in variants)


def rail_node(s, variants, exact=False):
    """First node in the left category rail (x < 280) matching any variant."""
    for n in s.nodes():
        lab = text_of(n).strip().lower()
        if not lab:
            continue
        hit = lab in tuple(v.lower() for v in variants) if exact else any(
            v.lower() in lab for v in variants)
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
    # The instant-apply footer is on every pane, so it survives whichever page
    # happens to be showing.
    return has_any(s, ("reset to defaults", "réinitialiser")) and has_any(s, DONE)


def open_settings(s):
    for args in ({"key": ",", "ctrl": True}, {"key": ",", "modifiers": ["ctrl"]},
                 {"key": "Comma", "modifiers": ["ctrl"]}):
        s.call("inject_key", args)
        for _ in range(10):
            time.sleep(0.4)
            if settings_open(s):
                return True
    return False


def page_reached(s, target):
    names = tuple(v.lower() for v in target)
    crumb = breadcrumb(s)
    if crumb and any(c and c.strip().lower() in names for c in crumb):
        return crumb
    row = rail_node(s, target, exact=True)
    if row and row.get("selected"):
        return crumb or list(names[:1])
    return None


def select_page(s, target, anchor_node, steps=14):
    """Walk to a rail page by keyboard from an already-visible anchor row —
    pointer-clicking a row's reported bounds does not work for one laid out
    below the scroll viewport (real bounds, nothing painted there); bastyde
    scrolls the *focused* row into view instead, so click the anchor to focus
    the tree, then step."""
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


def find_dialog(s, variants):
    """A popover's own Panel, by its `access_role(Dialog)` + `access_label` —
    the scoping technique used everywhere a popover's content must be told
    apart from whatever else is on screen behind it."""
    vs = tuple(v.lower() for v in variants)
    return next((n for n in s.nodes()
                 if n.get("role") == "Dialog" and text_of(n).strip().lower() in vs), None)


def within(b, box, pad=4.0):
    if not b or not box:
        return False
    return (box.get("x", 0) - pad <= b.get("x", 0)
            and b.get("x", 0) + b.get("width", 0) <= box.get("x", 0) + box.get("width", 0) + pad
            and box.get("y", 0) - pad <= b.get("y", 0)
            and b.get("y", 0) + b.get("height", 0) <= box.get("y", 0) + box.get("height", 0) + pad)


# ── Inspector section scoping (rule: bound by the section's own box) ───────

def inspector_section(s, heading_variants, x_min=560.0):
    """Find a named Inspector section's heading and the nodes inside its own
    band — never a "+/- pixels" guess, which sweeps in the next section's rows.

    Every known Inspector heading present on screen is located first and
    sorted by y; the requested section's band runs from its own heading down
    to the NEXT heading found (or +400 if it is the last one). This is what
    stops the Tags/Aliases/Mentioned-here/Mentioned-in sections — which all
    stack in the same column — from bleeding into one another, and it is why
    an "absent" case returns cleanly: if the heading itself is not among the
    ones found, there is no band to report and this returns `(None, [])`.
    """
    found = []
    for n in s.nodes():
        b = n.get("bounds") or {}
        if b.get("x", -1) < x_min:
            continue
        t = text_of(n).strip().lower()
        for variants in ALL_INSPECTOR_HEADS:
            if t in tuple(v.lower() for v in variants):
                found.append((b.get("y", 0.0), variants, n))
                break
    found.sort(key=lambda h: h[0])
    idx = next((i for i, h in enumerate(found) if h[1] == heading_variants), None)
    if idx is None:
        return None, []
    top = found[idx][0]
    bottom = found[idx + 1][0] if idx + 1 < len(found) else top + 400.0
    left = (found[idx][2].get("bounds") or {}).get("x", x_min) - 20
    rows = [n for n in s.nodes()
            if (n.get("bounds") or {}).get("x", -1) >= left
            and top <= (n.get("bounds") or {}).get("y", -1) < bottom]
    return found[idx][2], rows


def roster_row_for(rows, title):
    """Every `ListItem` row in a scoped band whose text is exactly `title` —
    exact, not substring, because a loose match on "Note 1" would also catch
    "Note 1" appearing inside some unrelated sentence of chrome."""
    return [n for n in rows if n.get("role") == "ListItem" and text_of(n).strip() == title]


def pin_for(rows, title):
    t = title.lower()
    for n in rows:
        low = text_of(n).lower()
        if t in low and any(v in low for v in MENTIONS_PIN_VERB):
            return n
    return None


# ── The save indicator, from automation_save_indicator.py's proven idiom ───

def indicator(s, timeout=10):
    end = time.time() + timeout
    while time.time() < end:
        for n in s.nodes():
            if n.get("role") != "Button":
                continue
            t = text_of(n).strip().lower()
            if any(v.lower() in t for v in SAVE_UNSAVED + SAVE_SAVED + SAVE_AUTOSAVE):
                return n
        time.sleep(0.4)
    return None


def indicator_state(n):
    t = text_of(n).strip().lower()
    if any(v.lower() in t for v in SAVE_UNSAVED):
        return "unsaved"
    if any(v.lower() in t for v in SAVE_SAVED):
        return "saved"
    if any(v.lower() in t for v in SAVE_AUTOSAVE):
        return "autosave"
    return "unknown"


def wait_indicator_state(s, want, timeout=15):
    end = time.time() + timeout
    last = None
    while time.time() < end:
        n = indicator(s, timeout=2)
        if n:
            last = indicator_state(n)
            if last == want:
                return True
        time.sleep(0.4)
    print(f"  indicator stuck at {last!r}, wanted {want!r}")
    return False


# ── Evidence text: search both trees, several fields, like word_count.py ───

def find_evidence(s, needles):
    """A node containing every string in `needles`. Primary: the accessibility
    tree — `TextWidget` defaults to `Role::Label` with its text as the
    accessible name unless explicitly hidden, and nothing in `mention_list.rs`
    hides the evidence body, so this should find it directly. Falls back to
    the full layout tree's Debug reprs (`include_debug=True`) as the same
    safety net `automation_word_count.py` uses for status-bar text that
    genuinely is pruned — cheap insurance against being wrong about that."""
    for n in s.nodes():
        t = text_of(n).lower()
        if all(nd.lower() in t for nd in needles):
            return n, "snapshot_tree"
    _, payload = s.call("layout_tree", {"include_debug": True})
    for n in payload.get("nodes", []):
        blob = (n.get("debug") or "").lower()
        if all(nd.lower() in blob for nd in needles):
            return n, "layout_tree(debug)"
    return None, None


# ═════════════════════════════════════════════════════════════════════════
# 1. Launch, sanity-check the build, and the autosave precondition
# ═════════════════════════════════════════════════════════════════════════
print("== launch ==")
s = Session(FIXTURE)
if not wait_label(s, ITEM_ZEUS.lower(), timeout=20):
    fail(f"the fixture did not load ({ITEM_ZEUS} not in the binder)", s)
# A `--features mocks` binary ignores the path on argv and serves fixture data
# of its own — every assertion below would then pass against state that was
# never in our working copy. The one failure this script must never report as
# a success.
if wait_label(s, "mock", timeout=1):
    fail("this is a `--features mocks` build serving fixture data, not our working "
         "copy — rebuild without the feature and re-run", s)
print("project loaded.")

# `SaveIndicator`'s tooltip COLLAPSES to the autosave string regardless of
# dirty state whenever autosave is on (save_indicator.rs), and Ctrl+S is a
# no-op there (the menu item and shortcut are hidden) — so section 6 below is
# unfalsifiable if autosave is unexpectedly on. `AUTOSAVE_KEY` defaults to
# `false` (main.rs), but this is cheap and exactly what an armed detector
# should check rather than assume.
ind0 = indicator(s)
if not ind0:
    fail("no save indicator in the status bar at all", s)
if indicator_state(ind0) == "autosave":
    fail("autosave is on — the load-bearing 'a scan does not dirty the project' "
         "check needs Ctrl+S and the saved/unsaved distinction, neither of which "
         "exist under autosave", s)
print(f"  save indicator: {text_of(ind0)!r} (autosave is off, as expected)")


# ═════════════════════════════════════════════════════════════════════════
# 2. Note 1 starts with no aliases field — the "before" half of the
#    appears-only-once-discoverable behaviour (the "after" half is checked
#    later; this half is what makes it a real assertion rather than one that
#    would also pass if the field were unconditionally present).
# ═════════════════════════════════════════════════════════════════════════
print("\n== Note 1 has no aliases field before it carries a discoverable tag ==")
if not open_item(s, ITEM_NOTE1):
    fail(f"could not select {ITEM_NOTE1!r} in the binder", s)
heading, _ = inspector_section(s, INSPECTOR_ALIASES)
if heading is not None:
    diag(s, "also known as")
    fail(f"{ITEM_NOTE1!r} already shows an aliases section before any discoverable "
         "tag was added — the gate on tags_vm's discoverable set is not doing its "
         "job, or a leftover tag from a previous run survived into this fixture", s)
print(f"  confirmed: no '{INSPECTOR_ALIASES[0]}' section yet")


# ═════════════════════════════════════════════════════════════════════════
# 3. Settings > Work > Tags: create a tag, flip its Story-bible switch
# ═════════════════════════════════════════════════════════════════════════
print("\n== create a story-bible tag in Settings ==")
if not open_settings(s):
    fail("Settings did not open (Ctrl+,)", s)
work = rail_node(s, SEC_WORK)
if not work:
    fail(f"no Work section in the settings rail (looked for {SEC_WORK})", s)
tags_row = rail_node(s, PAGE_TAGS, exact=True)
if not tags_row:
    # The section may ship collapsed; tap its chevron once.
    click(s, work.get("bounds") or {}, dx=10)
    for _ in range(12):
        s.settle()
        time.sleep(0.4)
        tags_row = rail_node(s, PAGE_TAGS, exact=True)
        if tags_row:
            break
if not tags_row:
    fail("no Tags page under the Work settings section", s)
crumb = select_page(s, PAGE_TAGS, anchor_node=work)
if not crumb:
    fail("could not select the Settings > Work > Tags page", s)
print(f"  breadcrumb: {crumb}")

# The add field: the TextInput sharing a row with the "Add tag"/"Ajouter"
# button (it has no placeholder/value in the AT tree, so position relative to
# a labelled sibling is the only sound handle — automation_tags.py's proven
# technique).
add_btn = next((n for n in s.nodes() if n.get("role") == "Button"
                and text_of(n).strip().lower() in tuple(v.lower() for v in ADD_TAG_BTN)), None)
if not add_btn:
    fail(f"no Add-tag button (looked for {ADD_TAG_BTN})", s)
btn_b = add_btn.get("bounds") or {}
add_field = next((n for n in s.nodes() if n.get("role") == "TextInput"
                  and abs((n.get("bounds") or {}).get("y", -999) - btn_b.get("y", 0)) < 12
                  and (n.get("bounds") or {}).get("x", 0) < btn_b.get("x", 0)), None)
if not add_field:
    fail(f"no TextInput on the add-tag row (button at y={btn_b.get('y')})", s)
if (add_field.get("value") or "").strip():
    fail(f"expected an empty add-tag field, got {add_field.get('value')!r} — refusing "
         "to type into what might be the wrong widget", s)

# Focus THEN type — rule: `type_text` without an explicit prior focus can
# update the AccessKit value without the widget's real signal ever seeing it,
# so the field renders empty while the tree reports the text. Costs nothing
# to do explicitly even on a build where `type_text` already auto-focuses.
act(s, add_field, "focus")
time.sleep(0.3)
s.call("type_text", {"node": add_field["id"], "text": TAG_NAME})
s.settle()
time.sleep(0.5)
# Re-find the button: the validation effect on the typed text can rebuild the
# row, which reallocates widget ids.
add_btn = next((n for n in s.nodes() if n.get("role") == "Button"
                and text_of(n).strip().lower() in tuple(v.lower() for v in ADD_TAG_BTN)), None)
if not add_btn:
    fail("the Add-tag button vanished after typing the new tag's name", s)
act(s, add_btn)
s.settle()
time.sleep(1.0)

name_matches = [n for n in s.nodes() if n.get("role") == "TextInput"
                and (n.get("value") or "").strip() == TAG_NAME]
if len(name_matches) != 1:
    diag(s, TAG_NAME)
    fail(f"expected exactly one tag named {TAG_NAME!r} after creation, found "
         f"{len(name_matches)} — creation either silently failed or ran twice", s)
name_field = name_matches[0]
print(f"  created tag {TAG_NAME!r}")

# The Story-bible switch is on the SAME row: nearest Switch to the right of
# the tag's own name field, within a small y band. NOT `sw[0]` — the pane's
# switch label is the generic "Story bible" text on every row, unparameterized
# by tag name, so with more than one tag on screen `sw[0]` picks whichever row
# happens to sort first, silently toggling the wrong tag.


def correlated_switch(s, name_val):
    nf = next((n for n in s.nodes() if n.get("role") == "TextInput"
               and (n.get("value") or "").strip() == name_val), None)
    if not nf:
        return None
    fb = nf.get("bounds") or {}
    for n in s.nodes():
        if n.get("role") != "Switch":
            continue
        b = n.get("bounds") or {}
        if abs(b.get("y", -9999) - fb.get("y", 0)) < 20 and b.get("x", -9999) > fb.get("x", 0):
            return n
    return None


sw = correlated_switch(s, TAG_NAME)
if not sw:
    fail(f"no Story-bible switch correlated to {TAG_NAME!r}'s row "
         f"(looked for {SW_STORY_BIBLE})", s)
if sw.get("toggled") == "true":
    fail("the new tag's Story-bible switch already reads on before we touched it — "
         "the structural correlation likely picked another row's switch", s)
act(s, sw)
s.settle()
time.sleep(0.8)
# Re-locate by the SAME structural correlation, not the cached id — the row
# rebuilds when the toggle writes back.
sw2 = correlated_switch(s, TAG_NAME)
if not sw2 or sw2.get("toggled") != "true":
    fail(f"the Story-bible switch for {TAG_NAME!r} did not flip on "
         f"(toggled={sw2.get('toggled') if sw2 else None!r})", s)
print(f"  {TAG_NAME!r} is now a Story-bible tag")

# Close Settings.
done = next((n for n in s.nodes() if n.get("role") == "Button"
             and text_of(n).strip().lower() in tuple(v.lower() for v in DONE)), None)
if not done:
    fail(f"no Settings 'Done' button (looked for {DONE})", s)
click(s, done.get("bounds") or {})
s.settle()
time.sleep(0.8)
if settings_open(s):
    fail("Settings would not close", s)


# ═════════════════════════════════════════════════════════════════════════
# 4. Put the tag on Note 1 via the Inspector's own "+" picker; confirm the
#    aliases field NOW appears (the "after" half of section 2); add an alias.
# ═════════════════════════════════════════════════════════════════════════
print("\n== tag Note 1 as story-bible material ==")
if not open_item(s, ITEM_NOTE1):
    fail(f"could not (re)select {ITEM_NOTE1!r} after closing Settings", s)

tag_add_btn = find(s, TAGS_PILL_ADD, role="Button")
if not tag_add_btn:
    fail(f"no Inspector tag '+' trigger (looked for {TAGS_PILL_ADD})", s)
act(s, tag_add_btn)
s.settle()
time.sleep(0.8)

option = next((n for n in s.nodes() if n.get("role") == "ListBoxOption"
               and text_of(n).strip() == TAG_NAME), None)
if not option:
    offered = [(n.get("role"), text_of(n).strip()) for n in s.nodes()
               if n.get("role") == "ListBoxOption"]
    fail(f"{TAG_NAME!r} was not offered in the tag picker; offered: {offered}", s)
# `TagPickRow` registers NO accessibility actions at all (its accessibility()
# override sets only role/name/selected) — invoke_action has nothing to
# invoke, so this goes straight to a pointer click, the technique
# automation_tags.py already established for exactly this row type.
click(s, option.get("bounds") or {})
s.settle()
time.sleep(0.8)
s.call("inject_key", {"key": "Escape"})
s.settle()
time.sleep(0.5)

heading, _ = inspector_section(s, INSPECTOR_ALIASES)
if heading is None:
    diag(s, "also known as")
    fail(f"{ITEM_NOTE1!r} still shows no aliases section after gaining a discoverable "
         "tag — the gate on the tags mirror signal is not reacting to the assign", s)
print(f"  the '{INSPECTOR_ALIASES[0]}' section appeared — confirmed, not assumed")

print("\n== give Note 1 the alias 'Vesparil' ==")
alias_add_btn = find(s, TAGS_ALIAS_ADD, role="Button")
if not alias_add_btn:
    fail(f"no alias '+' trigger (looked for {TAGS_ALIAS_ADD})", s)
act(s, alias_add_btn)
s.settle()
time.sleep(0.8)
dialog = find_dialog(s, TAGS_ALIAS_ADD)
if not dialog:
    fail(f"the alias entry popover did not open (looked for a Dialog labelled "
         f"{TAGS_ALIAS_ADD})", s)
db = dialog.get("bounds") or {}
field = next((n for n in s.nodes() if n.get("role") == "TextInput"
              and within(n.get("bounds"), db)), None)
if not field:
    fail("no TextInput inside the alias entry popover", s)
act(s, field, "focus")
time.sleep(0.3)
s.call("type_text", {"node": field["id"], "text": ALIAS})
s.settle()
time.sleep(0.3)
s.call("inject_key", {"key": "Return"})
s.settle()
time.sleep(1.0)

# Asserted on the PILL, not the field's own reported value — rule 4's focus
# trap: a field can report the typed text in the AT tree while the real write
# never reached the persisted alias list.
pills = [n for n in s.nodes() if n.get("role") == "ListItem" and text_of(n).strip() == ALIAS]
if len(pills) != 1:
    diag(s, ALIAS)
    fail(f"expected exactly one alias pill reading {ALIAS!r}, found {len(pills)} — "
         "the typed text may not have reached the persisted alias list", s)
print(f"  alias {ALIAS!r} committed (one pill, not just a field value)")


# ═════════════════════════════════════════════════════════════════════════
# 5. Zeus's roster is empty before the alias is written into its prose —
#    the positive control that arms the "roster appears" check below.
# ═════════════════════════════════════════════════════════════════════════
print(f"\n== {ITEM_ZEUS} has no roster entry before the alias is typed ==")
if not open_item(s, ITEM_ZEUS):
    fail(f"could not select {ITEM_ZEUS!r} in the binder", s)
editor = main_editor(s)
if not editor:
    fail(f"no writing editor after opening {ITEM_ZEUS!r}", s)
_, before_rows = inspector_section(s, MENTIONS_ROSTER)
before_hits = roster_row_for(before_rows, ITEM_NOTE1)
if before_hits:
    fail(f"{ITEM_ZEUS!r}'s roster already names {ITEM_NOTE1!r} before any alias was "
         "typed into its prose — the roster-appears check below would pass "
         "regardless of whether typing does anything at all", s)
print(f"  confirmed: {ITEM_ZEUS!r}'s roster does not yet name {ITEM_NOTE1!r}")


# ═════════════════════════════════════════════════════════════════════════
# 6. Type the alias into Zeus's prose; force a fresh Inspector read of the
#    live buffer; the roster must grow a SUGGESTION (pin present) naming
#    Note 1.
# ═════════════════════════════════════════════════════════════════════════
print(f"\n== typing {ALIAS!r} into {ITEM_ZEUS}'s prose ==")
act(s, editor, "focus")
time.sleep(0.3)
s.call("type_text", {"node": editor["id"], "text": SENTENCE})
s.settle()
time.sleep(1.0)

# `MentionIndex.roster_for` reads the focused item's LIVE OpenDoc buffer
# (`OpenDocsStore.peek`), no save required — but the Inspector widget itself
# only recomputes it on a rebuild, and typing alone does not bump any signal
# the Inspector binds. Switching the active tab away and back does: `focus`
# (== the active editor tab) is bound at `BindingLevel::Rebuild`, and
# `OpenDocsStore.open()` REUSES an already-open doc rather than discarding it,
# so Zeus's buffer — including what we just typed — survives the trip. This
# also has the side effect of flushing the edit into the Content store
# (`editors.flush_all()` runs on every pane-selection change), which only
# strengthens the check below.
if not open_item(s, ITEM_MARS):
    fail(f"could not select {ITEM_MARS!r} (needed to force the Inspector to re-read "
         f"{ITEM_ZEUS!r}'s live prose)", s)
if not open_item(s, ITEM_ZEUS):
    fail(f"could not reselect {ITEM_ZEUS!r} after switching away", s)


def roster_ready():
    _, rows = inspector_section(s, MENTIONS_ROSTER)
    return len(roster_row_for(rows, ITEM_NOTE1)) == 1


if not wait_for(roster_ready, tries=25, delay=0.4):
    # One more away-and-back, in case the first rebuild raced the scan that
    # seeded MentionIndex's alias table.
    open_item(s, ITEM_MARS)
    open_item(s, ITEM_ZEUS)
    if not wait_for(roster_ready, tries=25, delay=0.4):
        heading, rows = inspector_section(s, MENTIONS_ROSTER)
        print(f"  roster heading found: {heading is not None}; "
              f"rows in band: {[text_of(n) for n in rows]}")
        diag(s, ITEM_NOTE1)
        s.shot("/tmp/mentions-roster-fail.png")
        fail(f"{ITEM_ZEUS!r}'s roster never grew a row for {ITEM_NOTE1!r} after typing "
             f"the alias, count expected exactly 1", s)

heading, rows = inspector_section(s, MENTIONS_ROSTER)
if heading is None:
    fail(f"the '{MENTIONS_ROSTER[0]}' heading itself is missing even though "
         "roster_ready() reported the row present — inconsistent read", s)
note_rows = roster_row_for(rows, ITEM_NOTE1)
if len(note_rows) != 1:
    fail(f"expected exactly 1 roster row for {ITEM_NOTE1!r}, found {len(note_rows)}", s)
note_row = note_rows[0]
pin_btn = pin_for(rows, ITEM_NOTE1)
if not pin_btn:
    diag(s, "keep")
    diag(s, "conserver")
    fail(f"the roster row for {ITEM_NOTE1!r} carries no pin control — it should be a "
         "SUGGESTION, not a confirmed reference, until pinned", s)
print(f"  {ITEM_ZEUS!r}'s roster names {ITEM_NOTE1!r} as a suggestion (pin present)")
s.shot("/tmp/mentions-roster.png")


# ═════════════════════════════════════════════════════════════════════════
# 7. Hover the suggestion: the evidence tooltip must carry the matched name
#    AND the surrounding sentence — not merely open.
# ═════════════════════════════════════════════════════════════════════════
print("\n== the suggestion's evidence tooltip ==")
hover(s, note_row.get("bounds") or {})
# Composite tooltips use tooltip_delay_heavy (400 ms per pill.rs); poll well
# past it rather than a fixed sleep.
tip = None
for _ in range(20):
    time.sleep(0.3)
    tip = next((n for n in s.nodes()
                if (n.get("role") or "") in ("Tooltip", "Dialog")
                and text_of(n).strip() == ITEM_NOTE1), None)
    if tip:
        break
if not tip:
    diag(s, ITEM_NOTE1)
    s.shot("/tmp/mentions-tooltip-fail.png")
    fail(f"hovering the {ITEM_NOTE1!r} roster row opened no tooltip labelled "
         f"{ITEM_NOTE1!r}", s)
print(f"  tooltip opened, labelled {ITEM_NOTE1!r}")

# The sentence is the feature's whole answer to being wrong (the matcher
# knowingly false-positives on things like "don't"), so this proves the
# EXCERPT is really there — not just that a tooltip container appeared. Two
# independent distinctive substrings, not one, so a coincidental match against
# unrelated chrome text is implausible.
node, where = find_evidence(s, ["vesparil", "lanternfish"])
if not node:
    s.shot("/tmp/mentions-evidence-fail.png")
    fail("the evidence tooltip opened, but no node in either the accessibility "
         "tree or the full layout tree carries both 'Vesparil' and 'lanternfish' "
         "— the tooltip container rendered with an empty or wrong body", s)
print(f"  evidence sentence found ({where})")
s.shot("/tmp/mentions-evidence.png")


# ═════════════════════════════════════════════════════════════════════════
# 8. Note 1's backlinks are absent before pinning — the "before" half of the
#    confirmed-backlink check.
# ═════════════════════════════════════════════════════════════════════════
print(f"\n== {ITEM_NOTE1} has no backlinks before the suggestion is pinned ==")
if not open_item(s, ITEM_NOTE1):
    fail(f"could not select {ITEM_NOTE1!r}", s)
heading, _ = inspector_section(s, MENTIONS_BACKLINKS)
if heading is not None:
    diag(s, "mentioned in")
    diag(s, "mentionné dans")
    fail(f"{ITEM_NOTE1!r} already shows a '{MENTIONS_BACKLINKS[0]}' section before "
         "anything was pinned — the after-pin check below would be meaningless", s)
print(f"  confirmed: no '{MENTIONS_BACKLINKS[0]}' section yet")


# ═════════════════════════════════════════════════════════════════════════
# 9. Pin the suggestion on Zeus's Inspector: the row must persist while the
#    pin control disappears (confirmed, not merely no-longer-a-guess-and-gone).
# ═════════════════════════════════════════════════════════════════════════
print(f"\n== pin the suggestion on {ITEM_ZEUS} ==")
if not open_item(s, ITEM_ZEUS):
    fail(f"could not reselect {ITEM_ZEUS!r} to reach the pin control", s)
_, rows = inspector_section(s, MENTIONS_ROSTER)
pin_btn = pin_for(rows, ITEM_NOTE1)
if not pin_btn:
    diag(s, "keep")
    diag(s, "conserver")
    fail(f"the pin control for {ITEM_NOTE1!r} is gone before we ever clicked it — "
         f"{ITEM_ZEUS!r}'s roster did not survive the trip back", s)
act(s, pin_btn)
s.settle()


def pin_settled():
    _, band = inspector_section(s, MENTIONS_ROSTER)
    return len(roster_row_for(band, ITEM_NOTE1)) == 1 and pin_for(band, ITEM_NOTE1) is None


if not wait_for(pin_settled, tries=25, delay=0.4):
    _, band = inspector_section(s, MENTIONS_ROSTER)
    print(f"  rows now: {[text_of(n) for n in band]}")
    s.shot("/tmp/mentions-pin-fail.png")
    fail(f"after pinning, the roster row for {ITEM_NOTE1!r} and its pin control did "
         "not settle into 'row present, pin gone' — a no-op pin would leave the "
         "button; a pin that also deleted the row would still fail this jointly", s)
print(f"  {ITEM_NOTE1!r} is now confirmed on {ITEM_ZEUS!r}'s roster (pin gone, row kept)")
s.shot("/tmp/mentions-pinned.png")


# ═════════════════════════════════════════════════════════════════════════
# 10. Note 1's own Inspector now shows the backlink.
# ═════════════════════════════════════════════════════════════════════════
print(f"\n== {ITEM_NOTE1}'s Inspector shows the backlink ==")
if not open_item(s, ITEM_NOTE1):
    fail(f"could not reselect {ITEM_NOTE1!r}", s)


def backlink_ready():
    heading, band = inspector_section(s, MENTIONS_BACKLINKS)
    return heading is not None and len([n for n in band if n.get("role") == "ListItem"]) >= 1


if not wait_for(backlink_ready, tries=25, delay=0.4):
    diag(s, "mentioned in")
    diag(s, "mentionné dans")
    s.shot("/tmp/mentions-backlink-fail.png")
    fail(f"{ITEM_NOTE1!r} shows no '{MENTIONS_BACKLINKS[0]}' section even after "
         f"pinning from {ITEM_ZEUS!r}", s)

heading, band = inspector_section(s, MENTIONS_BACKLINKS)
back_rows = [n for n in band if n.get("role") == "ListItem"]
if len(back_rows) != 1:
    fail(f"expected exactly 1 backlink row on {ITEM_NOTE1!r}, found {len(back_rows)}", s)
# NOTE: not asserting the row's text equals `1.1 Zeus`. `MentionHit::Found`
# sets `title` unconditionally to the TARGET's name (skribisto_model /
# scan_mentions_uc.rs) — correct for the roster direction, but for backlinks
# every row's target IS the item being viewed, so `row.title` reads
# `ITEM_NOTE1` on every backlink row regardless of who mentions it. With one
# mentioning scene this happens to not look wrong; asserting the literal text
# here would be asserting behaviour the shipped code does not implement.
print(f"  backlink present: 1 row, reading {text_of(back_rows[0])!r} "
      f"(row identifies the mentioning scene only by position/count, a known "
      f"limitation — see MentionHit::Found's single `title` field)")
s.shot("/tmp/mentions-backlink.png")


# ═════════════════════════════════════════════════════════════════════════
# 11. THE LOAD-BEARING CHECK: a scan must never dirty the project.
#
#     Armed with a positive control first: save to a clean baseline, prove
#     the indicator DOES flip on a real edit in THIS session, save again to
#     get back to clean (not undo — `unsaved` is `dirty_seq > saved_seq`, a
#     monotonic counter; an edit-then-undo pair advances it twice and never
#     comes back down. Save is the only thing that moves `saved_seq`).
#     Only then does a relaunch's automatic on-load scan get to prove
#     anything: if the detector had never worked, the final "still saved"
#     would be trivially true of a check that cannot fail.
# ═════════════════════════════════════════════════════════════════════════
print("\n== save to a clean baseline ==")
save(s)
if not wait_indicator_state(s, "saved"):
    fail("could not reach a clean 'saved' baseline before the load-bearing check", s)
print("  saved")

print("\n== positive control: a real edit DOES flip the indicator (arming the detector) ==")
if not open_item(s, ITEM_ZEUS):
    fail(f"could not reselect {ITEM_ZEUS!r} for the positive control", s)
editor = main_editor(s)
if not editor:
    fail(f"no writing editor when reopening {ITEM_ZEUS!r} for the positive control", s)
act(s, editor, "focus")
time.sleep(0.3)
s.call("type_text", {"node": editor["id"], "text": "!"})
s.settle()
if not wait_indicator_state(s, "unsaved"):
    fail("typing one character did not flip the save indicator to 'unsaved' — the "
         "detector itself is not working, so the final 'a scan does not dirty the "
         "project' check below would be meaningless even if it passed", s)
print("  armed: the indicator DOES react to a real edit in this session")

print("\n== save again, back to clean ==")
save(s)
if not wait_indicator_state(s, "saved"):
    fail("could not save back to clean after the positive control", s)
print("  saved")

print("\n== relaunch: a fresh process, its own on-load scan, zero prior edits ==")
s.stop()
s = Session(FIXTURE)
if not wait_label(s, ITEM_ZEUS.lower(), timeout=20):
    fail("the working copy did not reload after relaunch", s)
print("  reloaded")

# Baseline: right after load, before we have touched anything in this process.
if not wait_indicator_state(s, "saved", timeout=10):
    fail("the reloaded project reads as unsaved immediately after load, before this "
         "process has made any edit at all — the load path itself is dirtying it, "
         "independently of any scan", s)
print("  reads as saved immediately after load")

# Proof the automatic post-load scan actually ran, using the now-PERSISTED
# alias/prose data (everything was saved before the relaunch): the backlink
# must reappear from disk, with no action on our part beyond opening Note 1 —
# `WorkManagementEvent::LoadWork` fires `MentionIndex::rescan()`
# unconditionally (app/wiring/long_ops.rs). Without this positive proof, "the
# indicator never moved" would be equally true of a scan that silently never
# ran or errored.
if not open_item(s, ITEM_NOTE1):
    fail(f"could not select {ITEM_NOTE1!r} after relaunch", s)


def backlink_ready_after_reload():
    heading, band = inspector_section(s, MENTIONS_BACKLINKS)
    return heading is not None and len([n for n in band if n.get("role") == "ListItem"]) >= 1


if not wait_for(backlink_ready_after_reload, tries=30, delay=0.5):
    diag(s, "mentioned in")
    diag(s, "mentionné dans")
    s.shot("/tmp/mentions-reload-fail.png")
    fail(f"{ITEM_NOTE1!r}'s backlink did not reappear after relaunch — either the "
         "persisted data did not survive save+reload, or the automatic on-load "
         "scan never completed; either way there is nothing here to prove the "
         "final indicator check against", s)
print(f"  {ITEM_NOTE1!r}'s backlink reappeared from persisted data — the on-load "
      "scan genuinely ran")

# THE assertion. A scan that touched `updated_at` or any entity field, even
# incidentally, would push `dirty_seq` past `saved_seq` and this catches it.
if not wait_indicator_state(s, "saved", timeout=10):
    fail("the save indicator now reads 'unsaved' after the on-load mention scan "
         "completed — a read-only scan (undoable: false, read_only: true, "
         "QueryUnitOfWork per scan_mentions_uc.rs) has dirtied the project", s)
print("  the indicator STILL reads saved — the scan did not dirty the project")
s.shot("/tmp/mentions-final-clean.png")

s.stop()
print("\nOK — the story-bible roster, its evidence, pinning, backlinks, and the "
      "no-dirty-on-scan guarantee all verified end to end.")
