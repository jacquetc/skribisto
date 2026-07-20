#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and check what the Settings > Work > Tags CSV
Import.../Export... feature can actually be verified through a GUI-automation
bridge -- which, after tracing the whole chain from the button to the OS, is
NOT the CSV round trip the plan originally asked for.

Both buttons (`crates/bastyde_ui/src/settings/panes/work_tags.rs:236,267`)
build a `FileDialogRequest` and hand it to `ctx.pick_file`/`ctx.save_file`.
That opens a real OS file chooser (on this Linux/KDE session, the
`org.freedesktop.portal.FileChooser` D-Bus service) -- a window that belongs
to a different process entirely. Tracing why that is unreachable, not assumed:

  * `bastyde-app/src/app.rs::install_file_dialog` always registers the real
    `RfdAsyncBackend`. The test-only `MemoryFileDialog` backend exists in
    `bastyde-platform/src/file_dialog.rs`, but Skribisto's `main.rs` never
    wires it in -- there is no automation-mode swap.
  * `bastyde-automation/src/mcp_schema.rs` lists every tool this bridge
    exposes (`snapshot_tree`, `inject_pointer`, `inject_key`, ...). None of
    them is a file-dialog tool, and there is no scripted-result queue like
    `MemoryFileDialog::enqueue` reachable from outside the process.
  * `bastyde-automation/src/executor.rs` marks `ListWindows` as "served by
    the host (window manager / headless shim)" -- i.e. bastyde-owned windows
    only. There is no OS-level input path anywhere in the stack (no
    XTest/uinput/enigo), so even a located dialog window could not be typed
    into or clicked.
  * `inject_key` (rule of this house, learned the hard way) reaches the
    FOCUSED BASTYDE WIDGET, not whatever window the compositor currently
    gives keyboard focus to -- so not even Escape can dismiss a portal
    dialog once it is open.
  * The view-model's own test author already hit this exact wall:
    `crates/bastyde_ui/src/view_models/tags.rs`, doc comment on
    `an_exported_file_is_readable_back_from_disk`, says outright "the UI path
    around it goes through a native file dialog, which no automation probe
    can drive, so this is the furthest out the export can be checked at all."
  * `scripts/automation_unsaved_guard.py` independently notes that New Work's
    confirmation is "the only one [door] whose confirmation isn't a native OS
    file dialog the bridge cannot see" -- i.e. every other probe in this
    directory already treats a file dialog as a hard stop, not a target.

Given that wall, clicking Import.../Export... here would not just fail to
verify anything -- it would pop a real, un-dismissable file-chooser window on
the desktop this happens to run on and leave it there after the probe exits,
because no teardown path this bridge has can close a window it cannot even
enumerate. That is a worse outcome than a probe that declines to try. So:

    THIS PROBE NEVER INVOKES EITHER BUTTON.

What it verifies instead, live, without opening the dialog:

  1. the fixture's baseline palette in the Settings > Work > Tags pane is
     exactly the three fixture tags with their real colours (#FFFAFA /
     #FF0000 / #000000) and the pane's own "3 tags"/"3 étiquettes"
     count pill agrees -- the data half of the plan's assertion 1, read
     directly rather than through a file no click can ever produce;
  2. the Import.../Export... buttons exist, are each found EXACTLY ONCE
     inside the pane (not the title bar's unrelated "Export" split-button,
     not a hypothetical Export command-palette entry sharing the word), are
     enabled, and are not clipped past the pane's right edge -- i.e. the
     wiring the plan's assertions 1-5 would ride on, if the dialog were not
     in the way.

None of the plan's five CSV-round-trip assertions (write + parse the export,
edit externally, re-import with a case-insensitive skip, 3 -> 5 -> one-undo
-> 3) are exercised here. They cannot be, by any GUI-automation probe against
this binary, for the reasons above. Every place below where this probe stops
short says so explicitly, with the assertion number it would have covered --
this script must never be mistaken for a green light on the CSV round trip.
The CSV format and the case-insensitive dedup themselves already have
`#[cfg(test)]` coverage that does not need a window at all:
`crates/bastyde_ui/src/view_models/tags.rs::tests::csv_round_trips` and
`::an_exported_file_is_readable_back_from_disk`. Driving the round trip and
the single-undo behaviour through the real `TagsViewModel` (bypassing only
the dialog, the same boundary that existing test already sits on) is a Rust
test, not a Python probe -- there is no live-app substitute for it.

Navigation is copied from `scripts/automation_tags.py`'s proven
`open_settings`/`rail_node`/`select_page`/`page_reached` (handles the Work
section's collapsed start state, the Tags row landing below the rail's
scroll viewport, and both locales). `Session`, `fail`, `text_of`, `find` and
`click` are copied from `scripts/automation_languages.py`.

Run:  python3 scripts/automation_tag_csv.py
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

# This probe launches the app on a copy and never the checked-in fixture, even
# though it does not itself save -- opening the real file at all risks a
# silent format-migration write on load. See `automation_fixture` for the
# three incidents that rule comes from.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "tagcsv")

# The app follows the SYSTEM locale (French on this machine). Every
# user-visible string matched below is a tuple of (english, french), each
# commented with the ftl key it came from -- an English-only selector finds
# nothing on this desktop and that reads exactly like a missing control.
SEC_WORK = ("work", "œuvre", "oeuvre")          # settings-sec-work
PAGE_TAGS = ("tags", "étiquettes")               # settings-page-tags
IMPORT_LABEL = ("import…", "importer…")     # settings-tags-import
EXPORT_LABEL = ("export…", "exporter…")     # settings-tags-export
COUNT_3 = ("3 tags", "3 étiquettes")             # settings-tags-count (n=3, "other" arm)

# The fixture's three legacy tags (see the module docstring in
# automation_tags.py for provenance): name -> real colour, lowercased because
# every text comparison below works on lowercased AT-tree strings.
FIXTURE_TAGS = ("a", "b", "very looooooooooong tag")
FIXTURE_COLORS = ("#fffafa", "#ff0000", "#000000")

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
    """One launched app + connected MCP server, restartable.

    Copied verbatim from automation_languages.py (the template this task was
    told to copy from) -- no changes beyond the client name in `initialize`.
    """

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
                                  "clientInfo": {"name": "tag-csv", "version": "1"}})
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
        exit immediately -- which surfaces as "app exited before printing the
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


# ── Navigation, copied (signatures adapted to take `s` explicitly, matching ──
# ── the languages.py convention) from the proven scripts/automation_tags.py ──

def joined(s):
    """Every label and value on screen, lowercased and space-joined.

    Values matter as much as labels: a tag's colour lives in a `ColorWell`'s
    `value`, not any `label` -- a labels-only search would find the pane's
    chrome and none of the fixture's actual palette content.
    """
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


def exact_texts(s):
    """Every label/value as an exact, stripped, lowercased string.

    Substring matching is unusable here: two of the fixture's tags are named
    "A" and "B", and "a" occurs in virtually every sentence of chrome on
    screen, so tag presence has to be matched whole, not as a substring.
    """
    out = set()
    for n in s.nodes():
        for key in ("label", "value"):
            v = n.get(key)
            if isinstance(v, str) and v.strip():
                out.add(v.strip().lower())
    return out


def rail_node(s, variants, exact=False):
    """First node in the left category rail (x < 280) matching any variant.

    The rail bound matters: without it a page name like "Tags" also matches
    the Inspector's tag list in the main window *behind* the modal, and
    clicking that does nothing to the settings pane.
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
    """The instant-apply footer is on every pane, so it survives the panel
    being restructured around any particular page name."""
    return (has_any(s, ("reset to defaults", "réinitialiser"))
            and has_any(s, ("done", "terminé")))


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


def page_reached(s, target):
    """The breadcrumb if the panel is showing `target`, else None."""
    names = (target,) if isinstance(target, str) else tuple(target)
    crumb = breadcrumb(s)
    if crumb and any(c and c.strip().lower() in names for c in crumb):
        return crumb
    row = rail_node(s, names, exact=True)
    if row and row.get("selected"):
        return crumb or list(names[:1])
    return None


def select_page(s, target, anchor="keymap", steps=14, anchor_node=None):
    """Select a rail page by walking to it from a visible anchor row.

    Pointer-clicking the target's reported bounds does not work for a row
    below the rail's scroll viewport -- its AT bounds are real but nothing is
    painted there. Keyboard navigation sidesteps it: bastyde scrolls the
    focused row into view, so click a visible row for focus, then step.
    """
    got = page_reached(s, target)
    if got:
        return got

    a = anchor_node or rail_node(s, (anchor,) if isinstance(anchor, str) else anchor, exact=True)
    if not a:
        return None

    for key in ("Down", "Up"):
        click(s, a.get("bounds") or {})
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


def node_enabled(s, node_id):
    """Fresh enabled/disabled read via `read_node` (bypasses whatever a stale
    snapshot_tree copy says)."""
    _, p = s.call("read_node", {"node": node_id})
    if isinstance(p, dict):
        if p.get("disabled") is True:
            return False
        if "enabled" in p:
            return bool(p["enabled"])
    return True


def pane_button(s, variants):
    """The Button-role node with an EXACT (not substring) label match inside
    the settings pane (x > 280, past the rail), or a diagnostic list of every
    candidate if the exactly-one invariant doesn't hold.

    Exact match, not substring: `crates/bastyde_ui/src/app/commands/export.rs`
    registers an unrelated global shortcut named literally "Export…" (no
    ellipsis-locale variance, no relation to the CSV feature), and the
    title-bar Export split-button's own label ("Export"/"Export Scene", no
    ellipsis) sits in the same x>280 region. A substring match on "export"
    would happily return either. Requiring the ellipsis and requiring exactly
    one hit turns a coincidental label collision into a loud failure instead
    of a silently wrong node.
    """
    vs = tuple(v.lower() for v in variants)
    hits = []
    for n in s.nodes():
        if n.get("role") != "Button":
            continue
        b = n.get("bounds") or {}
        if not isinstance(b, dict) or b.get("x", 0) <= 280:
            continue
        lab = (n.get("label") or "").strip().lower()
        if lab in vs:
            hits.append(n)
    if len(hits) != 1:
        print(f"  ambiguous match for {variants}: {len(hits)} candidate(s)")
        for n in hits:
            print(f"    {n.get('role')} {n.get('label')!r} @ {n.get('bounds')}")
        return None
    return hits[0]


# ── 1. Launch and reach Settings > Work > Tags ───────────────────────────────
print("== launch ==")
s = Session(FIXTURE)
# "chapter" is project content (a binder item title stored in the fixture),
# not UI chrome, so it is the same string regardless of system locale -- the
# same load-confirmation string automation_tags.py uses against this fixture.
if not wait_for_load(s.nodes, ("chapter", "chapitre")):
    fail("the fixture did not load (no 'chapter' binder item found)", s)
print("project loaded.")

print("\n== open Settings ==")
if not open_settings(s):
    fail("Settings did not open (Ctrl+,)", s)
print("Settings open.")

print("\n== find the Work section, expand it if needed ==")
work = rail_node(s, SEC_WORK)
if not work:
    fail(f"the rail has no Work section (looked for {SEC_WORK})", s)
tags_row = rail_node(s, PAGE_TAGS, exact=True)
if not tags_row:
    # The Work section starts collapsed; tap its chevron once. A second tap
    # would toggle it back shut, so this is deliberately not looped.
    click(s, work.get("bounds") or {}, dx=10)
    for _ in range(12):
        s.settle()
        time.sleep(0.4)
        tags_row = rail_node(s, PAGE_TAGS, exact=True)
        if tags_row:
            break
if not tags_row:
    fail("no Tags page under the Work section", s)
print(f"  Tags row located at y={(tags_row.get('bounds') or {}).get('y')}")

print("\n== select the Tags page ==")
crumb = select_page(s, PAGE_TAGS, anchor_node=work)
if not crumb:
    s.shot("/tmp/tagcsv-nav-fail.png")
    fail("could not select the Tags page", s)
print(f"  breadcrumb: {crumb}")

# ── 2. Baseline palette: the data half of the plan's assertion 1 ────────────
# This is read straight from the pane's own AT nodes (name = a row's inline
# TextInput value, colour = its ColorWell value), NOT from an exported file --
# no button click in this probe ever produces one. It stands in for "the
# export contains all three fixture tags with their real colours" by proving
# the SOURCE data the export would have serialised is exactly that.
print("\n== baseline palette (plan assertion 1's data, read live) ==")
texts = exact_texts(s)
missing_names = [t for t in FIXTURE_TAGS if t not in texts]
if missing_names:
    print("  exact texts seen:", sorted(texts)[:40])
    s.shot("/tmp/tagcsv-palette-fail.png")
    fail(f"fixture tags missing from the palette: {missing_names} -- an export "
         f"built from this palette would have been short, so it must never be "
         f"reported as containing all three", s)
print(f"  all 3 fixture tags present: {', '.join(FIXTURE_TAGS)}")

missing_colors = [c for c in FIXTURE_COLORS if c not in texts]
if missing_colors:
    fail(f"fixture tag colours missing from the palette: {missing_colors}", s)
print(f"  all 3 real colours present: {', '.join(FIXTURE_COLORS)}")

if not has_any(s, COUNT_3):
    fail(f"the pane's own count pill does not read {COUNT_3} -- palette size "
         f"disagrees with 3, and this exact pill is what a live probe would "
         f"have re-checked after import to catch a fourth copy of a duplicate", s)
print(f"  count pill agrees: {COUNT_3}")
s.shot("/tmp/tagcsv-baseline.png")

# ── 3. Import…/Export… buttons: present, unique, enabled, unclipped ─────────
# This is as far into the plan's assertions as a live probe can honestly go:
# it proves the wiring the click would ride on, and stops at the click itself.
print("\n== Import…/Export… button wiring (stops short of clicking) ==")
import_btn = pane_button(s, IMPORT_LABEL)
if not import_btn:
    s.shot("/tmp/tagcsv-buttons-fail.png")
    fail(f"no unique Import button in the pane (looked for {IMPORT_LABEL})", s)
export_btn = pane_button(s, EXPORT_LABEL)
if not export_btn:
    s.shot("/tmp/tagcsv-buttons-fail.png")
    fail(f"no unique Export button in the pane (looked for {EXPORT_LABEL})", s)
print(f"  found exactly one Import button: {import_btn.get('label')!r}")
print(f"  found exactly one Export button: {export_btn.get('label')!r}")

for name, btn in (("Import", import_btn), ("Export", export_btn)):
    if not node_enabled(s, btn["id"]):
        fail(f"the {name} button is disabled with a full palette open -- "
             f"a writer with tags to export could never reach the dialog", s)
print("  both buttons enabled")

# Neither button may overflow the pane's right edge -- "Export…" specifically
# was previously clipped off by a toolbar that overflowed (see
# automation_tags.py's PANE_BUTTONS clipping check, same technique reused
# here). `pane_right` is the rightmost edge of any Label-role node inside the
# pane; there is always at least one (the section description text), so an
# empty max() would itself be a sign the pane never rendered.
pane_labels_right = [
    (n.get("bounds") or {}).get("x", 0) + (n.get("bounds") or {}).get("width", 0)
    for n in s.nodes()
    if n.get("role") == "Label" and (n.get("bounds") or {}).get("x", 0) > 280
]
if not pane_labels_right:
    fail("no Label-role content found inside the pane at all -- the pane "
         "region this clipping check relies on is empty, which means the "
         "check above would prove nothing", s)
pane_right = max(pane_labels_right)
for name, btn in (("Import", import_btn), ("Export", export_btn)):
    b = btn.get("bounds") or {}
    right = b.get("x", 0) + b.get("width", 0)
    if right > pane_right + 2:
        s.shot("/tmp/tagcsv-clip-fail.png")
        fail(f"{name}… overflows the pane (right={right:.0f} > {pane_right:.0f})", s)
print(f"  neither button is clipped (pane right edge ~{pane_right:.0f}px)")
s.shot("/tmp/tagcsv-buttons.png")

# ── 4. What this probe deliberately does NOT do, and why ────────────────────
# Spelled out per assertion so a reader of the transcript (not just the
# docstring) sees exactly what was and was not checked.
print("\n== NOT covered here (native file dialog unreachable by this bridge) ==")
print("  plan assertion 1 (export file exists, header, all 3 rows via CSV parse):")
print("    BLOCKED -- export_to() only runs after ctx.save_file()'s dialog")
print("    resolves with a path; the bridge cannot open or confirm that dialog,")
print("    so clicking Export would never produce a file to parse. The palette")
print("    DATA that export would have written was verified directly above.")
print("  plan assertion 2 (edit the CSV externally, re-include one name cased):")
print("    MOOT -- no file is ever produced by this probe to edit.")
print("  plan assertion 3 (import gains exactly 2, count 3 -> 5, dup skipped):")
print("    BLOCKED -- import_from() only runs after ctx.pick_file()'s dialog")
print("    resolves; same wall, opposite button.")
print("  plan assertion 4 (import summary reports added/skipped):")
print("    BLOCKED -- no import ever fires, so no summary toast is ever shown.")
print("  plan assertion 5 (one Ctrl+Z reverts the whole import, back to 3):")
print("    BLOCKED -- nothing was ever imported through the live app to undo.")
print("  These are covered at the only reachable boundary -- TagsViewModel::")
print("  export_to/import_from called directly, bypassing only the dialog --")
print("  by crates/bastyde_ui/src/view_models/tags.rs::tests (csv_round_trips,")
print("  an_exported_file_is_readable_back_from_disk), and would need a new")
print("  Rust-level test, not a Python probe, to cover the 3->5->undo->3 shape.")

s.stop()
print("\nOK (partial, by design) -- Tags settings pane reachable, baseline "
      "palette (3 tags, real colours) verified, Import…/Export… wiring "
      "verified present/unique/enabled/unclipped. The CSV round trip itself "
      "is architecturally unreachable from this bridge; see above.")
