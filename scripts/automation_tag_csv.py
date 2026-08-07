#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and check what the Settings > Work > Tags CSV
Import.../Export... feature can actually be verified through a GUI-automation
bridge -- which is NOT the CSV round trip the plan originally asked for.

Both buttons build a `FileDialogRequest` and hand it to
`ctx.pick_file`/`ctx.save_file`, which opens a real OS file chooser
(`org.freedesktop.portal.FileChooser` on this Linux/KDE session) belonging to
a different process. Skribisto wires no test-only file-dialog backend, the
automation bridge exposes no file-dialog tool, and `inject_key` only reaches
the focused teksilo widget, not whatever window the compositor gives
keyboard focus to -- so this bridge cannot see, type into, or even dismiss
that dialog. Clicking Import.../Export... here would pop a real,
un-dismissable file-chooser window with no way to close it. So:

    THIS PROBE NEVER INVOKES EITHER BUTTON.

What it verifies instead, live, without opening the dialog:

  1. the fixture's baseline palette in the Settings > Work > Tags pane is
     exactly the three fixture tags with their real colours (#FFFAFA /
     #FF0000 / #000000) and the pane's own "3 tags"/"3 étiquettes" count
     pill agrees;
  2. the Import.../Export... buttons exist, are each found EXACTLY ONCE
     inside the pane, are enabled, and are not clipped past the pane's right
     edge -- the wiring the CSV round trip would ride on, if the dialog were
     not in the way.

None of the plan's five CSV-round-trip assertions (write + parse the export,
edit externally, re-import with a case-insensitive skip, 3 -> 5 -> one-undo
-> 3) are exercised here; they cannot be, by any GUI-automation probe against
this binary. That coverage instead lives in
`crates/teksilo_ui/src/view_models/tags.rs::tests::csv_round_trips` and
`::an_exported_file_is_readable_back_from_disk`, which drive the real
`TagsViewModel` directly, bypassing only the dialog.

Navigation is copied from `scripts/automation_tags.py`'s
`open_settings`/`rail_node`/`select_page`/`page_reached`. `Session`, `fail`,
`text_of`, `find` and `click` are copied from `scripts/automation_languages.py`.

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
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import wait_for_load, working_copy

# This probe launches the app on a copy and never the checked-in fixture, even
# though it does not itself save -- opening the real file at all risks a
# silent format-migration write on load.
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

        Relaunching on the same path while the old process still holds its
        open-registry lock hands the new launch off to it and exits it
        immediately -- surfacing as "app exited before printing the bridge
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
    painted there. Keyboard navigation sidesteps it: teksilo scrolls the
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

    Exact match, not substring: an unrelated global "Export…" shortcut and
    the title-bar Export split-button's own label ("Export", no ellipsis)
    both sit in the same x>280 region, so a substring match on "export"
    would happily return either.
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

# Neither button may overflow the pane's right edge. `pane_right` is the
# rightmost edge of any Label-role node inside the pane; there is always at
# least one (the section description text), so an empty max() would itself
# be a sign the pane never rendered.
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
print("  by crates/teksilo_ui/src/view_models/tags.rs::tests (csv_round_trips,")
print("  an_exported_file_is_readable_back_from_disk), and would need a new")
print("  Rust-level test, not a Python probe, to cover the 3->5->undo->3 shape.")

s.stop()
print("\nOK (partial, by design) -- Tags settings pane reachable, baseline "
      "palette (3 tags, real colours) verified, Import…/Export… wiring "
      "verified present/unique/enabled/unclipped. The CSV round trip itself "
      "is architecturally unreachable from this bridge; see above.")
