#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and prove that near-black / near-white tag dots stay
visible in BOTH themes, by sampling actual rendered pixels rather than trusting
the accessibility tree.

Tag colours are user data and constant across themes (`contrast.rs`'s own
module doc says so), which is exactly the setup that breaks: a near-black tag
is invisible on a dark card, a near-white one vanishes on a light card, and
where tags render as dots (`tag_chip.rs`) there is no text to fall back on at
all. The fix (`contrast::outline_on`) draws every dot's hairline from the
fill's own best-contrast extreme rather than a themed border token — the
`BorderRole::Default` token that shipped first was 1.26:1 against a white
card, i.e. present and still invisible. `contrast.rs` has five unit tests
pinning that arithmetic. None of them can see whether the *rasterizer* — text
shaping, anti-aliasing, corner radius, the render target's actual sRGB
encoding — agrees with the arithmetic. That gap is what this probe closes.

The fixture is the case the feature exists for: legacy tag "A" is `#FFFAFA`
(near-white) and "very looooooooooong tag" is `#000000` (near-black), both
attached to binder item "1.1 Zeus" (`automation_tags.py` proved this reachable
end to end already, which is why this probe does not re-derive that path).

What "proof" means here, concretely: locate the tag-dots row via its
accessibility bounds, screenshot *just that node* (so the crop is the
server's job, in already-scaled physical pixels, not a guess about DPI on
this end), decode the PNG with nothing but `zlib` and a hand-rolled chunk
walk (Pillow is used only as an optional cross-check on the decoder, never as
the thing being tested), and for each dot compute the actual WCAG contrast
ratio between every pixel in its cell and a sampled corner of the surface
behind it. That is SC 1.4.11 applied to a screenshot, not to a colour value
read out of the widget tree.

Two things stop this from quietly proving nothing:

  * an ARMED DETECTOR (assertion 4): the LIGHT and DARK screenshots' own card
    luminance must differ by a wide margin. Without this, a `set_theme` call
    that reported success in the AT tree but never actually repainted the
    window would make the "DARK" screenshot secretly still light — and the
    near-black tag would then pass on ~21:1 contrast against a card that was
    never dark, proving nothing about the border at all.
  * a POSITIVE CONTROL (assertion 5): dot "B" (`#FF0000`) needs no border to
    clear 3:1 against either card. If it ever fails, the fault is in this
    harness's crop/decode/sampling, never in the product — B's contrast does
    not run through `contrast::outline_on`.

Only once both of those hold do the two REGRESSION assertions (6 and 7) mean
what they claim: "A" clears 3:1 on its own LIGHT card, and "very looooooooooong
tag" clears 3:1 on its own DARK card. Two more (assertion 8) cover the
opposite-extreme pairing for completeness, labelled as expected to pass even
without the border, so a reader is not alarmed if only 6/7 regress.

Honesty note, up front: if pixel sampling turns out not to be achievable on
this machine (no readable screenshot, a PNG shape the manual decoder does not
understand), this script says so loudly and falls back to the weakest thing
that still means something — that the AT tree's own outline colour differs
from the fill — rather than printing a green line for nothing. See the
`decode_png` failure path.

Run:  python3 scripts/automation_tag_contrast.py
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
import zlib

ROOT = "/home/cyril/Devel/skribisto/.claude/worktrees/tags"
SKRIBISTO = f"{ROOT}/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import wait_for_load, working_copy

# This probe saves nothing on purpose (no Ctrl+S anywhere below) but it still
# opens the project live, and a legacy load can itself rewrite the file on
# disk during migration — never the checked-in fixture. See `automation_fixture`
# for the three incidents that rule exists to prevent.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "tagcontrast")

try:
    from PIL import Image  # noqa: F401 -- optional cross-check only, see pil_cross_check()
    HAVE_PIL = True
except ImportError:
    HAVE_PIL = False

# ── Locale-robust strings, each tied to its ftl key ─────────────────────────
# The app follows the SYSTEM locale (French on this machine); an English-only
# selector finds nothing and that reads exactly like a real regression. Every
# user-visible string matched below carries both spellings.
SETTINGS_RESET = ("reset to defaults", "réinitialiser")            # main.ftl `settings-reset`
SETTINGS_DONE = ("done", "terminé")                                 # main.ftl `settings-done`
SEC_APPEARANCE_BEHAVIOUR = ("appearance & behaviour",
                             "apparence et comportement")           # `settings-sec-appearance-behaviour`
PAGE_APPEARANCE = ("appearance", "apparence")                       # main.ftl `settings-page-appearance`
# The accessible NAME on the Theme ComboBox itself comes from bastyde-widgets'
# own bundle (`theme_switcher_label`, set in `ThemeSwitcher::build`), not the
# app's `settings-field-app-theme` FormLayout label -- FormLayout field labels
# are decorative, not AccessKit labels (automation_settings.py's own comment).
# Both keys resolve to the identical string in both locales, so this is safe
# either way.
THEME_COMBO_NAME = ("theme", "thème")                                # `theme-switcher-label`
THEME_LIGHT = ("light", "clair")                                    # `theme-switcher-light`
THEME_DARK = ("dark", "sombre")                                     # `theme-switcher-dark`

ITEM = "1.1 Zeus"
# Alphabetical (case-insensitive), matching `sort_rows` in
# `work_tags_list_model.rs` -- this IS the order the dot row announces them
# in, and it is asserted, not assumed (see assertion 1 below).
LEGACY_TAGS = ("A", "B", "very looooooooooong tag")

# WCAG SC 1.4.11's floor for a graphical object's boundary, and the same
# constant `contrast.rs::GRAPHICAL_OBJECT_MIN` uses -- the live check and the
# unit test assert the identical number, not two different opinions about it.
GRAPHICAL_OBJECT_MIN = 3.0
# `contrast.rs::outline_guarantees_a_visible_boundary` pins the worst
# theoretical case at "> 3.5". A live measurement landing between 3.0 and 3.5
# is technically a pass but close enough to the floor that it is worth an
# eyeball on the saved PNG rather than a silent green line.
CLOSE_MARGIN = 0.5

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
                                  "clientInfo": {"name": "tagcontrast", "version": "1"}})
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

    def shot(self, path, node=None):
        """Screenshot the whole window, or (with `node`) crop to one node's
        bounds -- the server does the logical-dp -> physical-px conversion via
        `device_scale_factor` before capturing, so this never has to guess the
        display's scale factor."""
        res, _ = self.call("screenshot", {"node": node} if node is not None else {})
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                data = base64.b64decode(c["data"])
                open(path, "wb").write(data)
                print(f"  screenshot -> {path}")
                return data
        return None

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


def open_item(s, title):
    """Select `title` in the binder so its tab opens and the Inspector targets it."""
    for n in s.nodes():
        b = n.get("bounds") or {}
        if (n.get("label") or "").strip() == title and 40 <= b.get("x", 9999) <= 200:
            click(s, b)
            s.settle()
            time.sleep(1.2)
            return True
    return False


# ── Settings navigation ──────────────────────────────────────────────────────

def click_node(s, n):
    """Prefer `invoke_action(click)` over a synthesised pointer click (a
    near-miss pointer click lands on the neighbouring widget and opens ITS
    tooltip, leaving a popover shut in a way that looks like "the popover is
    empty") -- but gate on the node actually ADVERTISING the action first,
    rather than trying it and reacting to an error.

    Read `dispatch_action_and_settle` in bastyde-automation's executor: it
    dispatches whatever action is asked for to whatever node id is asked for
    with no check that the node supports it, and reports success either way.
    A Settings-rail row (role "Unknown", no actions at all --
    `automation_tags.py` documents this) would make `invoke_action(click)`
    return no error while doing precisely nothing, and a "fall back only on
    error" policy would then never fall back -- it would silently no-op on
    every rail row. Checking `"click" in actions` up front is what
    `automation_settings.py`'s own `click_node` does, and is the only way to
    tell "this will really do something" from "this will be silently
    swallowed" ahead of time.
    """
    if not n:
        return False
    if "click" in (n.get("actions") or []):
        res, _ = s.call("invoke_action", {"node": n["id"], "action": "click"})
        if not (isinstance(res, dict) and res.get("isError")):
            return True
        # Advertised but dispatch still errored (e.g. a transient overlay
        # race) -- the pointer fallback below is the last resort either way.
    return click(s, n.get("bounds") or {})


def rail_node(s, variants, exact=False):
    """First node in the left category rail (x < 280) matching any variant.

    The bound matters: without it a page name like "Appearance" can also match
    something in the main window behind the modal, and clicking that does
    nothing to the settings pane (rule: scope every "is X shown" check to the
    panel that owns X).
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


def joined(s):
    parts = []
    for n in s.nodes():
        for key in ("label", "value"):
            v = n.get(key)
            if isinstance(v, str) and v.strip():
                parts.append(v)
    return " | ".join(parts).lower()


def has_any(s, variants):
    j = joined(s)
    return any(v in j for v in variants)


def settings_open(s):
    """The instant-apply footer (Reset to defaults + Done) is on every pane,
    so it survives the panel being restructured around any particular page."""
    return has_any(s, SETTINGS_RESET) and has_any(s, SETTINGS_DONE)


def open_settings(s):
    for args in ({"key": ",", "ctrl": True},
                 {"key": ",", "modifiers": ["ctrl"]},
                 {"key": "Comma", "modifiers": ["ctrl"]}):
        s.call("inject_key", args)
        for _ in range(10):
            time.sleep(0.4)
            if settings_open(s):
                return True
    return False


def click_done(s):
    for n in s.nodes():
        if (n.get("role") == "Button"
                and (n.get("label") or "").strip().lower() in SETTINGS_DONE
                and "click" in (n.get("actions") or [])):
            click_node(s, n)
            s.settle()
            time.sleep(0.6)
            return True
    return False


def theme_combo(s):
    """The (should-be-unique) Theme ComboBox on the Appearance pane. Returns
    the list of matches rather than the first one, so the caller can enforce
    "exactly one" instead of silently trusting whichever the loop found
    first (rule: non-vacuity -- a count-of-1 claim has to be checked, not
    assumed)."""
    return [n for n in s.nodes()
            if n.get("role") == "ComboBox"
            and any(v in (n.get("label") or "").lower() for v in THEME_COMBO_NAME)]


def open_appearance_pane(s):
    """Reach Settings ▸ Appearance ▸ Theme. Cheap fast path first (the page is
    almost certainly already reachable, since `settings.rs` expands the
    Appearance & Behaviour section unconditionally -- `tree.expand(ab)` runs
    with no guard, unlike the Work/Spelling/Backup sections which start
    collapsed); a keyboard-walk fallback in case the row's bounds sit below
    the rail's scroll viewport, mirroring `automation_tags.py`'s
    `select_page` -- clicking a row's reported bounds does nothing when
    nothing is actually painted there.
    """
    if theme_combo(s):
        return True
    row = rail_node(s, PAGE_APPEARANCE, exact=True)
    if row and click_node(s, row):
        s.settle()
        time.sleep(0.6)
        if theme_combo(s):
            return True
    anchor = rail_node(s, SEC_APPEARANCE_BEHAVIOUR, exact=True)
    if not anchor:
        return False
    for key in ("Down", "Up"):
        click_node(s, anchor)
        s.settle()
        time.sleep(0.4)
        for _ in range(10):
            if theme_combo(s):
                return True
            s.call("inject_key", {"key": key})
            s.settle()
            time.sleep(0.3)
    return theme_combo(s)


THEME_TOUCHED = [False]  # mutable box so nested functions can flip it
ORIGINAL_THEME_VALUE = [None]
# Reentrancy guard: `restore_theme_best_effort` calls `set_theme` to put the
# ORIGINAL value back. If THAT call hits one of `set_theme`'s own failure
# branches, calling `fail_restoring` again would call
# `restore_theme_best_effort` again, which would call `set_theme` again for
# the identical, presumably identically-failing, restore -- unbounded
# recursion instead of a clean exit. `bail()` below checks this flag and
# drops to a plain `fail()` (no further restore attempt) whenever a restore
# is already in flight further up the call stack.
RESTORING = [False]


def set_theme(s, target_variants, label):
    """Pick `label` (one of `target_variants`, e.g. THEME_LIGHT) in the Theme
    ComboBox. Requires Settings ▸ Appearance to already be open. Returns the
    combo's own freshly-read `value` after the click, which is what assertion
    3 checks -- not "no error", since a near-miss click landing on a
    neighbouring row is a real, previously-seen failure shape that reports no
    error at all."""

    def bail(msg):
        # `fail_restoring`, not `fail`, in the common case: this function runs
        # both before AND after the theme has changed from ORIGINAL, and
        # `restore_theme_best_effort` is a safe no-op when nothing has been
        # touched yet (it checks `THEME_TOUCHED[0]` itself). But see
        # `RESTORING` above -- if we are already inside a restore attempt,
        # fall back to a plain `fail()` rather than recursing into another one.
        if RESTORING[0]:
            fail(msg, s)
        else:
            fail_restoring(msg)

    combos = theme_combo(s)
    if len(combos) != 1:
        names = [(n.get("label"), n.get("value")) for n in s.nodes() if n.get("role") == "ComboBox"]
        bail(f"expected exactly 1 Theme ComboBox (matched on {THEME_COMBO_NAME}), "
             f"found {len(combos)}. Every ComboBox on screen: {names}")
    combo = combos[0]
    current = (combo.get("value") or "").strip()
    if current.lower() in target_variants:
        print(f"  theme already {label} ({current!r}) — nothing to click")
        return current

    if not click_node(s, combo):
        bail(f"could not open the Theme dropdown for {label} — invoke_action(click) "
             f"errored and the pointer-click fallback also reported nothing")
    s.settle()
    time.sleep(0.6)

    # The dropdown floats in a child overlay of the modal -- search the main
    # tree first (proven to work for Sec 3b's regression check in
    # automation_settings.py), then the overlay layer as a fallback.
    opt = None
    for n in s.nodes():
        if (n.get("role") in ("ListBoxOption", "ListItem", "MenuItem", "Button")
                and (n.get("label") or "").strip().lower() in target_variants):
            opt = n
            break
    if not opt:
        _, ov = s.call("get_overlays")
        pool = ov.get("overlays") or ov.get("nodes") or []
        for grp in pool:
            for n in ([grp] + (grp.get("nodes") or grp.get("children") or [])):
                if (n.get("label") or "").strip().lower() in target_variants:
                    opt = n
                    break
            if opt:
                break
    if not opt:
        # Say what the dropdown DID offer -- "could not pick Dark" is equally
        # true of a popover that never opened and one offering different
        # labels, and the two need opposite fixes.
        offered = [f"{n.get('role')}:{(n.get('label') or '').strip()}" for n in s.nodes()
                   if n.get("role") in ("ListBoxOption", "ListItem", "MenuItem")
                   and (n.get("label") or "").strip()]
        print(f"  dropdown offered {len(offered)} option(s):")
        for o in offered[:15]:
            print(f"    {o}")
        bail(f"could not find {label!r} among the Theme dropdown options "
             f"(looked for {target_variants})")

    if not click_node(s, opt):
        bail(f"clicking the {label!r} option in the Theme dropdown failed "
             f"(both invoke_action and the pointer fallback)")
    # Mark it touched as soon as a click was actually dispatched into the
    # dropdown, BEFORE checking whether it landed where intended. If the
    # confirmation below fails, the live theme may still have changed to
    # *something* (a near-miss can land on a different row, not just fail
    # outright) -- and if this is the very first click of the run, leaving
    # `THEME_TOUCHED` False until a successful confirmation would make
    # `restore_theme_best_effort` skip the restore entirely, on the one
    # failure path where it is most likely to be needed.
    THEME_TOUCHED[0] = True
    s.settle()
    time.sleep(0.8)

    fresh = next((n for n in s.nodes() if n.get("id") == combo.get("id")), None)
    now = (fresh.get("value") or "").strip() if fresh else ""
    if now.lower() not in target_variants:
        bail(f"selected {label!r} but the combo now reports {now!r} — this is the "
             f"known near-miss shape: a click landed on the neighbouring row "
             f"instead of the one picked")
    print(f"  theme -> {now!r}")
    return now


def restore_theme_best_effort(s):
    """Put the global theme back to what it was before this probe touched it,
    even on a failure path.

    Selecting a theme here is instant-apply (no OK/Apply button on the
    footer) and, per every other instant-apply setting in this panel,
    persists to the writer's real settings file immediately -- not just in
    this process's memory. A probe that dies mid-run without this would leave
    the *developer's actual desktop* switched to whichever theme this run
    happened to fail on. Same hygiene class `automation_fixture.py` exists to
    prevent for the project file; this is its equivalent for global settings.
    Best-effort and silent-on-failure: we are already failing, and a second
    failure here must not mask the first one.
    """
    if not THEME_TOUCHED[0] or ORIGINAL_THEME_VALUE[0] is None:
        return
    if RESTORING[0]:
        # Already attempting a restore further up the call stack (a failure
        # inside `set_theme`'s own restore attempt bounced back here) --
        # do not start a second, identically-doomed one.
        return
    RESTORING[0] = True
    try:
        if not settings_open(s):
            open_settings(s)
        if open_appearance_pane(s):
            set_theme(s, (ORIGINAL_THEME_VALUE[0].strip().lower(),),
                      f"ORIGINAL ({ORIGINAL_THEME_VALUE[0]!r})")
            click_done(s)
            THEME_TOUCHED[0] = False
            print(f"  (best-effort restore: theme set back to {ORIGINAL_THEME_VALUE[0]!r})")
        else:
            print("  (best-effort restore FAILED: could not reach the Appearance pane -- "
                  "the desktop theme may be left changed; check Settings > Appearance by hand)")
    except SystemExit:
        raise
    except Exception as e:
        print(f"  (best-effort restore also failed: {e!r} -- the desktop theme may be left "
              f"changed; check Settings > Appearance by hand)")
    finally:
        RESTORING[0] = False


def clear_hover(s):
    """Move the pointer to a neutral corner and settle, so no stray hover ring
    (a tooltip, a hover-revealed affordance) contaminates the crop."""
    s.call("inject_pointer", {"x": 5, "y": 5, "action": "move"})
    s.settle()
    time.sleep(0.3)


def find_dot_row(s, context, bail):
    """The Label-role node naming every tag on the current item.

    `TagChipRow::accessibility` (`tag_chip.rs`) sets exactly `Role::Label` +
    the joined tag names as the node's name, and its own doc comment says it
    emits no per-dot nodes -- so this is structurally the only place all
    three tag names co-occur in one node. Enforcing "exactly one match" (not
    "at least one") is the same non-vacuity discipline `automation_tags.py`
    needed for its Inspector check: a stray match would mean either the item
    is no longer open/selected, or (per that probe's own war story) a
    leftover node from a panel that has since closed is being swept in.
    """
    matches = [n for n in s.nodes()
               if n.get("role") == "Label"
               and all(t.lower() in (n.get("value") or n.get("label") or "").lower()
                       for t in LEGACY_TAGS)]
    if len(matches) != 1:
        carriers = [f"role={n.get('role')} value={n.get('value')!r} label={n.get('label')!r}"
                    for n in s.nodes()
                    if any(t.lower() in text_of(n).lower() for t in LEGACY_TAGS)]
        print(f"  [{context}] nodes mentioning any of {LEGACY_TAGS}:")
        for c in carriers[:12]:
            print(f"    {c}")
        bail(f"[{context}] expected exactly 1 dot-row Label naming all three tags, found "
             f"{len(matches)} — either {ITEM!r} is no longer open/selected, or a stray node "
             f"from another panel is being matched (see the carriers list above)")
    return matches[0]


# ── PNG decode: zlib + a hand-rolled chunk walk, stdlib only ────────────────

def _paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    if pb <= pc:
        return b
    return c


def decode_png(data):
    """Decode an 8-bit RGBA, non-interlaced PNG to (width, height, rgba_bytes).

    This is the exact shape the bridge always emits: `encode_png` in
    `bastyde-app/src/automation_bridge.rs` hardcodes
    `png::ColorType::Rgba` + `png::BitDepth::Eight` with no interlacing set.
    If a future bridge change ever emits something else, this fails loudly
    naming the actual header fields rather than silently misreading pixels —
    the "assert the strongest thing achievable, honestly" rule this whole
    probe is built around.
    """
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("not a PNG (bad signature) — the screenshot tool returned something "
                          "else, or no image at all")
    pos = 8
    width = height = bit_depth = color_type = interlace = None
    idat = bytearray()
    n = len(data)
    while pos + 8 <= n:
        length = int.from_bytes(data[pos:pos + 4], "big")
        ctype = data[pos + 4:pos + 8]
        pos += 8
        chunk = data[pos:pos + length]
        pos += length + 4  # skip the trailing CRC — the bridge is a trusted local source
        if ctype == b"IHDR":
            width = int.from_bytes(chunk[0:4], "big")
            height = int.from_bytes(chunk[4:8], "big")
            bit_depth = chunk[8]
            color_type = chunk[9]
            interlace = chunk[12]
        elif ctype == b"IDAT":
            idat.extend(chunk)
        elif ctype == b"IEND":
            break
    if width is None:
        raise ValueError("PNG had no IHDR chunk")
    if bit_depth != 8 or color_type != 6 or interlace != 0:
        raise ValueError(f"unexpected PNG shape: bit_depth={bit_depth} color_type={color_type} "
                          f"interlace={interlace} (expected 8/6/0 = 8-bit RGBA, non-interlaced) "
                          f"— the bridge's encoder must have changed; this decoder needs updating, "
                          f"not silently misreading pixels")

    raw = zlib.decompress(bytes(idat))
    bpp = 4  # RGBA @ 8 bits/channel
    stride = width * bpp
    pixels = bytearray(height * stride)
    prev_row = bytearray(stride)
    pos = 0
    for y in range(height):
        filt = raw[pos]
        pos += 1
        row = raw[pos:pos + stride]
        pos += stride
        out = bytearray(stride)
        for x in range(stride):
            a = out[x - bpp] if x >= bpp else 0
            b = prev_row[x]
            c = prev_row[x - bpp] if x >= bpp else 0
            rv = row[x]
            if filt == 0:
                v = rv
            elif filt == 1:
                v = (rv + a) & 0xFF
            elif filt == 2:
                v = (rv + b) & 0xFF
            elif filt == 3:
                v = (rv + (a + b) // 2) & 0xFF
            elif filt == 4:
                v = (rv + _paeth(a, b, c)) & 0xFF
            else:
                raise ValueError(f"unknown PNG scanline filter type {filt} on row {y}")
            out[x] = v
        pixels[y * stride:(y + 1) * stride] = out
        prev_row = out
    return width, height, bytes(pixels)


def get_px(pixels, width, x, y):
    i = (y * width + x) * 4
    return pixels[i], pixels[i + 1], pixels[i + 2]


def pil_cross_check(path, width, height, pixels):
    """Optional, non-authoritative: if Pillow happens to be importable, decode
    the same PNG with it and compare a handful of sample points against the
    manual decoder above. This is a cross-check on the *decoder*, never a
    substitute for it — every assertion in this file runs against the manual
    decode regardless of whether Pillow is present, so its absence never
    changes what gets proven, only how loudly a decoder bug would be caught.
    """
    if not HAVE_PIL:
        print("  (Pillow not importable — skipping the decoder cross-check; the manual "
              "zlib/PNG path above is what every assertion in this run actually relies on)")
        return
    try:
        im = Image.open(path).convert("RGBA")
        if im.size != (width, height):
            print(f"  Pillow cross-check: size mismatch ({im.size} vs manual "
                  f"{width}x{height}) — skipping")
            return
        pts = [(0, 0), (width // 2, height // 2), (width - 1, height - 1), (width // 3, height // 2)]
        bad = [(x, y) for (x, y) in pts if get_px(pixels, width, x, y) != im.getpixel((x, y))[:3]]
        if bad:
            print(f"  Pillow cross-check: DISAGREES with the manual decoder at {bad} — "
                  f"treat the pixel results below with suspicion")
        else:
            print("  Pillow cross-check: agrees with the manual decoder on 4 sample points")
    except Exception as e:
        print(f"  Pillow cross-check errored ({e!r}) — non-fatal, manual decode is unaffected")


# ── WCAG contrast math, re-implemented from bastyde-tokens/src/color.rs ────

def _linearize(c):
    c = c / 255.0
    return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4


def relative_luminance(rgb):
    r, g, b = rgb
    return 0.2126 * _linearize(r) + 0.7152 * _linearize(g) + 0.0722 * _linearize(b)


def contrast_ratio(rgb1, rgb2):
    a, b = relative_luminance(rgb1), relative_luminance(rgb2)
    lighter, darker = (a, b) if a >= b else (b, a)
    return (lighter + 0.05) / (darker + 0.05)


def analyze_cell(pixels, width, height, x0, x1):
    """For one dot's cell (a vertical slice `[x0, x1)` of the row), sample an
    inset corner patch as the "surface behind the dot" reference, then find
    the pixel in the cell with the HIGHEST contrast against that reference.

    The corner is inset rather than taken from the exact edge because the
    node crop's own boundary can carry a sliver of anti-aliasing that belongs
    to neither the dot nor a clean sample of the surface. The dot itself
    (`FixedDot`, `tag_chip.rs`) is centred within an 18dp cell at a painted
    size of at most 14dp (fill + ring), so an inset corner patch is reliably
    clear of it regardless of the display's scale factor — no dp/px
    conversion needed here at all, only a fraction of the cell we already
    have in pixels.
    """
    cw = x1 - x0
    dim = min(cw, height)
    inset = max(1, int(dim * 0.12))
    patch = max(1, int(dim * 0.15))
    xs = range(x0 + inset, min(x0 + inset + patch, x1))
    ys = range(inset, min(inset + patch, height))
    samples = [get_px(pixels, width, x, y) for y in ys for x in xs]
    if not samples:
        samples = [get_px(pixels, width, min(x0 + 1, x1 - 1), min(1, height - 1))]
    ref = tuple(sum(s[i] for s in samples) / len(samples) for i in range(3))

    best_ratio, best_at = 0.0, None
    for y in range(height):
        for x in range(x0, x1):
            px = get_px(pixels, width, x, y)
            r = contrast_ratio(px, ref)
            if r > best_ratio:
                best_ratio, best_at = r, (x, y, px)
    return ref, best_ratio, best_at


def analyze_row_png(data, theme_label):
    """Decode one dot-row screenshot and return `{tag_name: {ref, ratio, at}}`
    for all three tags, splitting the row into three equal-width cells.

    Proportional thirds, not a dp measurement: the row is
    `HStack::new().spacing(0.0)` (`tag_chip.rs`) of exactly 3 cells at
    `HIT=18.0` dp each (3 tags ≤ `MAX_VISIBLE_EDITOR=8`, so no overflow "+N"
    cell exists to throw the count off), and cells abut with zero gap. Any
    uniform scale factor divides that into three equal PNG-pixel thirds, so
    there is nothing here that depends on knowing the display's DPI.
    """
    w, h, pixels = decode_png(data)
    print(f"  [{theme_label}] decoded {w}x{h} px")
    if w < 3 or h < 1:
        # `fail_restoring`, not `fail` -- this runs after the theme has already
        # been switched away from ORIGINAL, so a bare `fail()` here would exit
        # with the writer's real desktop left on Light/Dark and the app process
        # still up (see the module docstring's hygiene note on assertion 9).
        fail_restoring(f"[{theme_label}] decoded a {w}x{h} image from the dot row — too small "
                        f"to contain 3 dot cells; the node crop likely returned the wrong node "
                        f"or nothing")
    cells = [(round(i * w / 3), round((i + 1) * w / 3)) for i in range(3)]
    out = {}
    for name, (x0, x1) in zip(LEGACY_TAGS, cells):
        if x1 <= x0:
            fail_restoring(f"[{theme_label}] the cell computed for {name!r} is zero-width "
                            f"({x0}..{x1}) out of a {w}px row")
        ref, ratio, at = analyze_cell(pixels, w, h, x0, x1)
        note = ""
        if GRAPHICAL_OBJECT_MIN <= ratio < GRAPHICAL_OBJECT_MIN + CLOSE_MARGIN:
            note = "  (close to the 3.0 floor — worth an eyeball on the saved PNG)"
        print(f"  [{theme_label}] {name!r}: cell=[{x0}:{x1}) ref=rgb{tuple(round(c) for c in ref)} "
              f"max_ratio={ratio:.2f}:1 at {at}{note}")
        out[name] = {"ref": ref, "ratio": ratio, "at": at}
    return w, h, pixels, out


# =============================================================================
print("== launch ==")
s = Session(FIXTURE)

# A `--features mocks` build ignores the path on argv and serves fixture data
# baked into the binary, so every assertion below would "pass" against tags
# that were never read from this file. This must never be silently green.
if find(s, ("mock",)):
    fail("this looks like a `--features mocks` build (found the word 'mock' in the tree) — "
         "rebuild without that feature and re-run, or every result below is meaningless", s)

if not wait_for_load(s.nodes, ITEM):
    fail(f"the fixture did not load ({ITEM!r} not in the binder)", s)
print("project loaded.")

if not open_item(s, ITEM):
    fail(f"could not select {ITEM!r} in the binder", s)
print(f"  opened {ITEM!r}")

# ── Assertion 1: the dot row announces exactly these 3 tags, in this order ──
print("\n== the dot row (before touching the theme) ==")
row0 = find_dot_row(s, "initial read", lambda m: fail(m, s))
row0_text = (row0.get("value") or row0.get("label") or "").strip()
parts = [p.strip() for p in row0_text.split(",")]
print(f"  row announces: {row0_text!r}")
if parts != list(LEGACY_TAGS):
    fail(f"expected the dot row to name exactly {list(LEGACY_TAGS)} in that order (this is "
         f"also what makes the alphabetical A/B/very-tag -> cell-index mapping below safe — "
         f"it is asserted here, not assumed), got {parts}", s)
print(f"  order confirmed: {LEGACY_TAGS} (matches sort_rows' case-insensitive alpha order)")

# ── Reach Settings ▸ Appearance ▸ Theme, and record the untouched value ─────
print("\n== Settings ▸ Appearance ▸ Theme ==")
if not open_settings(s):
    fail("Settings did not open (Ctrl+,)", s)
if not open_appearance_pane(s):
    fail("could not reach Settings ▸ Appearance (no Theme ComboBox ever appeared)", s)
combos = theme_combo(s)
if len(combos) != 1:
    fail(f"expected exactly 1 Theme ComboBox on the Appearance pane, found {len(combos)}", s)
ORIGINAL_THEME_VALUE[0] = (combos[0].get("value") or "").strip()
if not ORIGINAL_THEME_VALUE[0]:
    fail("the Theme ComboBox reports no current value — cannot safely record an original "
         "to restore later, refusing to touch the theme at all", s)
print(f"  ORIGINAL theme: {ORIGINAL_THEME_VALUE[0]!r} (will be restored at the end)")


def fail_restoring(msg):
    """Like `fail`, but restores the theme first — every call site after this
    point has potentially left the writer's real desktop theme changed."""
    restore_theme_best_effort(s)
    fail(msg, s)


try:
    # ── LIGHT ────────────────────────────────────────────────────────────────
    print("\n== switch to LIGHT ==")
    set_theme(s, THEME_LIGHT, "Light")
    if not click_done(s):
        fail_restoring("the Done button did not close Settings after selecting Light")
    if settings_open(s):
        fail_restoring("Settings window did not close after Done (Light)")
    clear_hover(s)
    s.shot("/tmp/tagcontrast-light-full.png")
    row = find_dot_row(s, "LIGHT", fail_restoring)
    light_png = s.shot("/tmp/tagcontrast-light-dots.png", node=row["id"])
    if not light_png:
        fail_restoring("the node-cropped screenshot of the LIGHT dot row returned no image data")

    # ── DARK ─────────────────────────────────────────────────────────────────
    print("\n== switch to DARK ==")
    if not open_settings(s):
        fail_restoring("Settings did not reopen for the Dark switch")
    if not open_appearance_pane(s):
        fail_restoring("could not reach Settings ▸ Appearance for the Dark switch")
    set_theme(s, THEME_DARK, "Dark")
    if not click_done(s):
        fail_restoring("the Done button did not close Settings after selecting Dark")
    if settings_open(s):
        fail_restoring("Settings window did not close after Done (Dark)")
    clear_hover(s)
    s.shot("/tmp/tagcontrast-dark-full.png")
    row = find_dot_row(s, "DARK", fail_restoring)
    dark_png = s.shot("/tmp/tagcontrast-dark-dots.png", node=row["id"])
    if not dark_png:
        fail_restoring("the node-cropped screenshot of the DARK dot row returned no image data")

    # ── Decode both, honestly ────────────────────────────────────────────────
    print("\n== decoding the dot-row screenshots ==")
    try:
        lw, lh, lpixels, light = analyze_row_png(light_png, "LIGHT")
        dw, dh, dpixels, dark = analyze_row_png(dark_png, "DARK")
    except ValueError as e:
        # The honesty fallback: pixel sampling did not work on this machine.
        # Say so loudly and prove the weakest thing that still means
        # something, instead of printing a green line for nothing.
        print(f"\n  PIXEL SAMPLING NOT ACHIEVABLE: {e}")
        print("  Falling back to the weakest claim this probe can still honestly make: that "
              "the dot row still names both extreme tags via the AT tree in the theme this "
              "process is currently in (DARK — the last one selected). This does NOT prove the "
              "rendered boundary is visible; it only proves the row still exists and still "
              "names them.")
        # NOT a substring `has_any()` scan of the whole tree: one of this fixture's own tag
        # names is the single letter "A", which — as automation_tags.py's own war story spells
        # out — is a substring of virtually every sentence of chrome on screen, so a bare
        # substring check would read "found" no matter what was on screen. Reuse the same
        # "exactly one Label names all three tags" predicate assertion 1 already proved
        # non-vacuous, scoped to a single node's content rather than the whole tree's text.
        fallback_matches = [n for n in s.nodes()
                            if n.get("role") == "Label"
                            and all(t.lower() in (n.get("value") or n.get("label") or "").lower()
                                    for t in LEGACY_TAGS)]
        if len(fallback_matches) != 1:
            fail_restoring(f"even the fallback text check failed: expected exactly 1 dot-row "
                            f"Label naming all three tags in the current theme, found "
                            f"{len(fallback_matches)} — cannot prove anything about visibility "
                            f"this run")
        fb_text = (fallback_matches[0].get("value") or fallback_matches[0].get("label") or "").strip()
        print(f"  fallback dot-row text: {fb_text!r}")
        print("  FALLBACK CHECK ONLY (not a pixel proof): the dot row still names both extreme "
              "tags ('A' and 'very looooooooooong tag') in the current theme. NOTHING about the "
              "border's actual RENDERED visibility was proven this run.")
        restore_theme_best_effort(s)
        s.settle()
        if not click_done(s) and settings_open(s):
            fail("could not close Settings during the fallback path's restore", s)
        s.stop()
        print("\nDONE (degraded) — pixel sampling was not achievable; see the note above. "
              "This run proves nothing about SC 1.4.11 visibility and must not be read as a pass.")
        sys.exit(2)

    pil_cross_check("/tmp/tagcontrast-light-dots.png", lw, lh, lpixels)
    pil_cross_check("/tmp/tagcontrast-dark-dots.png", dw, dh, dpixels)

    # ── Assertion 4: ARMED DETECTOR — the theme genuinely re-rendered ───────
    print("\n== assertion 4: the DARK screenshot is actually dark ==")
    light_lum = sum(relative_luminance(light[t]["ref"]) for t in LEGACY_TAGS) / len(LEGACY_TAGS)
    dark_lum = sum(relative_luminance(dark[t]["ref"]) for t in LEGACY_TAGS) / len(LEGACY_TAGS)
    gap = light_lum - dark_lum
    print(f"  mean card luminance: LIGHT={light_lum:.3f}  DARK={dark_lum:.3f}  gap={gap:.3f}")
    if gap < 0.20:
        fail_restoring(f"ARMED-DETECTOR FAILURE: the LIGHT card ({light_lum:.3f}) is not >= 0.20 "
                        f"brighter than the DARK card ({dark_lum:.3f}). Either the theme switch "
                        f"never actually repainted the window, or both screenshots were taken in "
                        f"the same theme — either way, the DARK screenshot cannot be trusted, and "
                        f"a pass on assertion 7 below would be meaningless: a near-black tag "
                        f"trivially clears 3:1 against a card that is secretly still light")
    print("  PASS: the two screenshots are genuinely different themes — the checks below mean "
          "what they claim to")

    # ── Assertion 5: POSITIVE CONTROL — the sampling pipeline can see a dot ──
    print("\n== assertion 5: positive control (dot 'B', #FF0000, needs no border) ==")
    for theme_label, results in (("LIGHT", light), ("DARK", dark)):
        b_ratio = results["B"]["ratio"]
        if b_ratio < GRAPHICAL_OBJECT_MIN:
            fail_restoring(f"POSITIVE-CONTROL FAILURE: dot 'B' only reached {b_ratio:.2f}:1 "
                            f"against its own {theme_label} card. This can ONLY be a harness bug "
                            f"(a bad crop, an off-by-one in the cell split, a PNG-decoder mistake) "
                            f"— B's contrast never runs through contrast::outline_on at all, so "
                            f"the product code is not a suspect here")
    print("  PASS: dot 'B' clears 3:1 unaided in both themes — the crop/decode/sample pipeline "
          "can see a real boundary, so a failure below would be about the product, not the harness")

    # ── Assertions 6 & 7: the CORE regression checks ─────────────────────────
    print("\n== assertions 6 & 7: the two extremes, each on the card that swallows it ==")
    a_light = light["A"]["ratio"]
    if a_light < GRAPHICAL_OBJECT_MIN:
        fail_restoring(f"CORE REGRESSION: 'A' (#FFFAFA, near-white) reached only {a_light:.2f}:1 "
                        f"against its own LIGHT card. This is the exact case contrast.rs's own "
                        f"test names ('1.03:1 against a white card on its own') — the derived "
                        f"ring (contrast::outline_on) is missing, or was reverted to a themed "
                        f"BorderRole token (BorderRole::Default measures 1.26:1 against white, "
                        f"which is also below the 3:1 floor and would look like 'it has SOME "
                        f"border' while still failing this exact check)")
    print(f"  PASS: 'A' (near-white) on its LIGHT card reaches {a_light:.2f}:1 >= "
          f"{GRAPHICAL_OBJECT_MIN}:1 — the border is doing real work")

    very_dark = dark["very looooooooooong tag"]["ratio"]
    if very_dark < GRAPHICAL_OBJECT_MIN:
        fail_restoring(f"CORE REGRESSION: 'very looooooooooong tag' (#000000, near-black) "
                        f"reached only {very_dark:.2f}:1 against its own DARK card — the "
                        f"mirror-image of the 'A' regression above")
    print(f"  PASS: 'very looooooooooong tag' (near-black) on its DARK card reaches "
          f"{very_dark:.2f}:1 >= {GRAPHICAL_OBJECT_MIN}:1")

    # ── Assertion 8: coverage on the opposite-extreme pairing ───────────────
    print("\n== assertion 8: coverage (opposite-extreme pairing) ==")
    for desc, val in (("'A' (near-white) on its DARK card", dark["A"]["ratio"]),
                       ("'very looooooooooong tag' (near-black) on its LIGHT card",
                        light["very looooooooooong tag"]["ratio"])):
        if val < GRAPHICAL_OBJECT_MIN:
            fail_restoring(f"{desc} only reached {val:.2f}:1. Unexpected: this pairing is the "
                            f"FILL alone against the opposite theme's card (e.g. white ink on a "
                            f"dark card), which should clear the floor by a wide margin without "
                            f"the border doing any work — its failure would suggest something "
                            f"more broadly wrong than the border logic")
        print(f"  PASS (expected even without the border): {desc} reaches {val:.2f}:1")

    print("\n== all pixel assertions passed — 'A' and 'very looooooooooong tag' are each "
          "distinguishable from their own card in both themes, proven from rendered pixels, "
          "not from the widget tree's reported colours ==")

    # ── Assertion 9: restore, and prove it stuck ─────────────────────────────
    print("\n== restore the original theme ==")
    if not settings_open(s):
        if not open_settings(s):
            fail("could not reopen Settings to restore the original theme", s)
    if not open_appearance_pane(s):
        fail("could not reach Settings ▸ Appearance to restore the original theme", s)
    restored = set_theme(s, (ORIGINAL_THEME_VALUE[0].strip().lower(),),
                          f"ORIGINAL ({ORIGINAL_THEME_VALUE[0]!r})")
    THEME_TOUCHED[0] = False
    if restored.strip().lower() != ORIGINAL_THEME_VALUE[0].strip().lower():
        fail(f"failed to restore the original theme: expected {ORIGINAL_THEME_VALUE[0]!r}, "
             f"combo now reports {restored!r} — the desktop theme is left changed", s)
    if not click_done(s):
        fail("the Done button did not close Settings after restoring the theme", s)
    if settings_open(s):
        fail("Settings window would not close after restoring the theme", s)
    print(f"  PASS: theme restored to {ORIGINAL_THEME_VALUE[0]!r}, and Settings closed")

except SystemExit:
    raise
except Exception as e:
    # An unhandled bug in this script, not a `fail()` call — still must not
    # leave the desktop theme changed or the app process dangling.
    import traceback
    print(f"\nFAIL: unexpected error during the theme/contrast checks: {e!r}")
    traceback.print_exc()
    restore_theme_best_effort(s)
    try:
        s.stop()
    except Exception:
        pass
    sys.exit(1)

s.stop()
print("\nOK — 'A' (near-white) and 'very looooooooooong tag' (near-black) both stay pixel-"
      "distinguishable from their surface in the LIGHT and DARK themes, proven by sampling the "
      "rendered dot row, with an armed re-render detector and a positive sampling control both "
      "green. The original theme was restored.")
