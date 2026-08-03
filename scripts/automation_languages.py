#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the language pill field end to end.

This is the live half of Stage 6, where `dict_language` stopped being a
space-separated string and became a `Vec<String>`. Unit tests already cover
the format, the migration, the resolver and the parse-time tolerance for
pre-v4 files — all of which are reachable without a window. What none of them
touch is the path a writer actually takes: open a project, add a language to
an item, remove one, and have the change survive a save and a reload.

That gap is the whole point of running this. The refactor's riskiest edges are
exactly the ones a unit test cannot see:

  * the pill field writes a **list** now, through `UpdateBinderItemDto`, and the
    Inspector's mirror-signal has to round-trip it without echoing;
  * an item with no languages must *inherit* the work's, and inheritance is
    decided by `has_tags`, which treats `[""]` as empty — a distinction that
    only exists because a list can hold a blank and a string could not;
  * a save writes the list to RON and the next open parses it back.

Asserts:

  1. the fixture opens and its item shows the language it was saved with;
  2. adding a second language leaves BOTH pills, not one replacing the other
     (the failure mode a string-to-list bug produces);
  3. the added language survives Ctrl+S and a full relaunch;
  4. removing it leaves the first, and that also survives a reload;
  5. removing the last one leaves the field empty rather than holding a blank
     entry — the `[""]`-is-not-empty invariant, checked where it is observable.

Run:  python3 scripts/automation_languages.py
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
from automation_fixture import working_copy

# This probe saves. Never the checked-in fixture — see `automation_fixture`.
FIXTURE = working_copy(f"{ROOT}/resources/test/skribisto_test_project.skrib", "langs")

# The app follows the system locale, so every matched string carries both
# spellings, keyed to its ftl entry.
LANG_SECTION = ("language", "langue")            # main.ftl `language`
ADD_LANG = ("add a language", "ajouter une langue")  # lang-pill-add
ITEM = "1.1 Zeus"

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
                                  "clientInfo": {"name": "langs", "version": "1"}})
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
    """Select `title` in the binder so the Inspector targets it."""
    for n in s.nodes():
        b = n.get("bounds") or {}
        if (n.get("label") or "").strip() == title and 40 <= b.get("x", 9999) <= 200:
            click(s, b)
            s.settle()
            time.sleep(1.2)
            return True
    return False


def language_pills(s):
    """The languages on the focused item, read from the pills' remove controls.

    Not from the pill `ListItem` itself: its accessible label is the *mute*
    action ("Turn spell-checking off for X"), and the language name lives in a
    child `Label`'s `value`. Reading the ListItem label reported the mute
    sentence as though it were a language.

    The remove control is a better handle anyway — `lang-pill-remove` is
    "Remove { $name }", so there is exactly one per pill and the name is the
    remainder. Anchored on the `lang-pill-list` List node rather than the
    section heading, so an unscoped match cannot pull in the settings page's
    language list or the spellcheck status button.
    """
    lst = next((n for n in s.nodes()
                if n.get("role") == "List"
                and text_of(n).strip().lower() in ("languages", "langues")), None)
    if not lst:
        return None
    lb = lst.get("bounds") or {}
    out = []
    for n in s.nodes():
        b = n.get("bounds") or {}
        t = text_of(n).strip()
        low = t.lower()
        if not (low.startswith("remove ") or low.startswith("retirer ")):
            continue
        # Inside the pill list's row band, not some other section's remove.
        # The list's own box, not a band around it: the Tags section sits in the
        # same column with its own "Retirer X" controls, and a loose band read
        # three tag names as languages.
        top, bot = lb.get("y", 0) - 4, lb.get("y", 0) + lb.get("height", 0) + 4
        if b.get("x", 0) >= lb.get("x", 0) - 20 and top <= b.get("y", -999) <= bot:
            out.append(t.split(" ", 1)[1].strip())
    return out


def add_language(s, code):
    """Open the '+' popover and pick `code`."""
    btn = find(s, ADD_LANG, role="Button")
    if not btn:
        fail(f"no add-language button (looked for {ADD_LANG})", s)
    # `invoke_action` rather than a synthesised pointer click: a click has to land
    # as press+release on the right widget, and a near miss lands on the language
    # pill beside it — which opens that pill's *tooltip* and leaves the popover
    # shut, looking exactly like "the popover has no options".
    res, _ = s.call("invoke_action", {"node": btn["id"], "action": "click"})
    if isinstance(res, dict) and res.get("isError"):
        click(s, btn.get("bounds") or {})
    s.settle()
    time.sleep(1.5)
    for n in s.nodes():
        if code.lower() in text_of(n).lower() and n.get("role") in (
                "ListItem", "ListBoxOption", "MenuItem", "Button"):
            click(s, n.get("bounds") or {})
            s.settle()
            time.sleep(1.2)
            return True
    # Say what the popover *did* offer. "Could not pick en-US" is equally true of
    # a popover that never opened and one that offers different codes, and the
    # two need opposite fixes.
    offered = [f"{n.get('role')}:{text_of(n).strip()}" for n in s.nodes()
               if n.get("role") in ("ListItem", "ListBoxOption", "MenuItem")
               and text_of(n).strip()]
    print(f"  popover offered {len(offered)} option(s):")
    for o in offered[:15]:
        print(f"    {o}")
    return False


def save(s):
    s.call("inject_key", {"key": "s", "ctrl": True})
    s.settle()
    time.sleep(2.0)


# ── 1. Open, and read the item's language ───────────────────────────────────
print("== launch ==")
s = Session(FIXTURE)
if not any(ITEM in (n.get("label") or "") for n in s.nodes()):
    fail(f"the fixture did not load ({ITEM} not in the binder)", s)
print("project loaded.")

if not open_item(s, ITEM):
    fail(f"could not select {ITEM!r} in the binder", s)
print(f"  opened {ITEM!r}")

before = language_pills(s)
if before is None:
    s.shot("/tmp/langs-no-section.png")
    fail(f"no Language section in the Inspector (looked for {LANG_SECTION})", s)
print(f"  languages on the item: {before}")
s.shot("/tmp/langs-initial.png")

# ── 2. Add a second language: BOTH must remain ──────────────────────────────
print("\n== add a second language ==")
# The popover offers display names ("English (United States)"); the pill and the
# stored value carry the code. Pick by the former, assert on the latter.
if any("fr-fr" in p.lower() for p in before):
    NEW, NEW_LABEL = "en-US", "English (United States)"
else:
    NEW, NEW_LABEL = "fr-FR", "Français"
if not add_language(s, NEW_LABEL):
    s.shot("/tmp/langs-add-fail.png")
    fail(f"could not pick {NEW_LABEL!r} from the add-language popover", s)

after = language_pills(s)
print(f"  now: {after}")
if not any(NEW.lower() in p.lower() for p in after):
    fail(f"{NEW} was not added (field shows {after})", s)
# The point of the whole stage. A string-shaped bug replaces rather than appends.
if len(after) <= len(before):
    s.shot("/tmp/langs-replaced.png")
    fail(f"adding {NEW} did not grow the list ({before} -> {after}) — a list must "
         f"append, and this is exactly what a string-shaped write looks like", s)
print(f"  both kept: {len(before)} -> {len(after)} pills")

# ── 3. It survives a save and a full relaunch ───────────────────────────────
print("\n== save and relaunch ==")
save(s)
s.stop()

s = Session(FIXTURE)
if not open_item(s, ITEM):
    fail(f"could not reselect {ITEM!r} after relaunch", s)
reloaded = language_pills(s)
print(f"  after reload: {reloaded}")
if sorted(x.lower() for x in reloaded) != sorted(x.lower() for x in after):
    fail(f"the language list did not survive save+reload: {after} -> {reloaded}", s)
print("  the list round-tripped through RON unchanged")
s.shot("/tmp/langs-reloaded.png")

# ── 4. Remove one: the other stays ──────────────────────────────────────────
print("\n== remove the added language ==")
rm = None
for n in s.nodes():
    t = text_of(n).lower()
    if ("retirer" in t or "remove" in t) and NEW.lower() in t:
        rm = n
        break
if not rm:
    fail(f"no remove control for {NEW}", s)
click(s, rm.get("bounds") or {})
s.settle()
time.sleep(1.2)

left = language_pills(s)
print(f"  now: {left}")
if any(NEW.lower() in p.lower() for p in left):
    fail(f"{NEW} was not removed (field shows {left})", s)
if sorted(x.lower() for x in left) != sorted(x.lower() for x in before):
    fail(f"removing {NEW} should restore the original list {before}, got {left}", s)
print("  removal left the original language untouched")

# ── 5. Save the removal too, and confirm no blank entry survives ────────────
print("\n== the removal survives a reload, with no blank left behind ==")
save(s)
s.stop()

s = Session(FIXTURE)
if not open_item(s, ITEM):
    fail(f"could not reselect {ITEM!r} after the second relaunch", s)
final = language_pills(s)
print(f"  after reload: {final}")
if sorted(x.lower() for x in final) != sorted(x.lower() for x in before):
    fail(f"the removal did not persist: expected {before}, got {final}", s)
# `[""]` renders as a pill with no text. A list can hold a blank where a
# space-separated string could not, and `has_tags` exists precisely because a
# blank must not count as "has a language" — this is where that is observable.
blanks = [p for p in final if not p.strip()]
if blanks:
    s.shot("/tmp/langs-blank.png")
    fail(f"the field holds {len(blanks)} blank entr(y/ies) — a removed language must "
         f"leave no empty string behind, or the item stops inheriting the work's", s)
print("  no blank entry left behind")
s.shot("/tmp/langs-final.png")

s.stop()
print("\nOK — the language list adds, removes, and round-trips through save+reload.")
