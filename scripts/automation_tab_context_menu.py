#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The editor tab strip's context menu, driven end to end.

Everything interesting about this menu is a claim about a **live** widget tree
that no headless test can make:

  1. **It acts on the tab that was right-clicked, not the selected one.**
     Teksilo returns early on a successful Secondary `PointerDown`
     (`event_dispatch_impl.rs`), so a right-click neither selects nor focuses the
     tab it lands on, and `TapRecognizer` is primary-only. The unit tests call
     `row_label_and_action` directly and so can only prove the view-model agrees;
     only a real right-click proves the *event path* does. This is the crown-jewel
     assertion: select tab A, right-click tab B, fire "Close others", and B must
     be what survives.
  2. **A pinned tab really loses its close button and really sorts first.**
     Both are teksilo rendering decisions (`bar.rs` builds `on_close` only when
     closable), reachable only from a laid-out strip.
  3. **A pinned tab is still closable from this menu.** `closable(false)` removes
     the cross, the middle-click close and the Delete key together, so the Close
     row is the *only* way out. A regression here makes a pinned tab permanent.
  4. **A pin survives a restart, presentation and all.** The workspace restore
     reopens a remembered tab as an ordinary one and then marks it pinned; if it
     only marks it, the tab comes back with a close cross, a live middle-click
     close and a live Delete key while the menu and both bulk closes still spare
     it — and one careless click destroys the pin for good. Asserted the way the
     writer would find out: middle-click the restored tab and require it to still
     be there.
  5. **"Move into a new window" really opens one**, and the tab really leaves.

Runs against a throwaway copy of the bundled example in an isolated XDG sandbox
(see `automation_fixture`'s module doc for why a probe never opens a checked-in
fixture), so autosave and format migration cannot touch the repo.

The locale is **set**, not inherited: every label matched below is written in
English, and an unset `ui.locale` is the operator's OS language, not English
(`startup.rs`'s `auto_detect_os_locale`). Without this the probe passes on an
English desktop and fails on a French one, reporting that the menu never opened.
"""
import base64, json, os, re, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

#: Opened in the main pane, in this order. Three is the fewest that can tell
#: "closed the others" apart from "closed everything but the selection".
DOCS = ["Chapter 1", "Chapter 2", "Chapter 3"]

sandbox = tempfile.mkdtemp(prefix="skribisto_tab_menu_")
env = {**os.environ, "XDG_CONFIG_HOME": os.path.join(sandbox, "config"),
       "XDG_DATA_HOME": os.path.join(sandbox, "data"), "HOME": sandbox}
fixture.write_settings(env["XDG_CONFIG_HOME"])
work = os.path.join(sandbox, "Starforgers.skrib")
shutil.copyfile(EXAMPLE, work)

app = mcp = None


def cleanup():
    for p in (mcp, app):
        if p is not None and p.poll() is None:
            p.terminate()
    shutil.rmtree(sandbox, ignore_errors=True)


def die(msg, log=None):
    print("FAIL:", msg)
    if log:
        print("\n".join(open(log).read().splitlines()[-25:]))
    cleanup()
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, work], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
sock = tok = None
end = time.time() + 40
while time.time() < end:
    t = open(log).read()
    s = re.search(r"bridge socket = (\S+)", t)
    k = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", t)
    if s and k:
        sock, tok = s.group(1), k.group(1)
        break
    if app.poll() is not None:
        die("app exited before announcing the bridge", log)
    time.sleep(0.2)
if not sock:
    die("no bridge socket", log)
time.sleep(1.0)

mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok], stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, bufsize=1)
_id = 0


def call_raw(name, args=None):
    """The full MCP result. `call` below returns only the parsed payload, which
    throws away the image content block a screenshot answers with."""
    global _id
    _id += 1
    mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": _id, "method": "tools/call",
                                "params": {"name": name, "arguments": args or {}}}) + "\n")
    mcp.stdin.flush()
    while True:
        line = mcp.stdout.readline()
        if not line:
            return {}
        m = json.loads(line)
        if m.get("id") == _id:
            return m.get("result", {})


def shot(path):
    for c in call_raw("screenshot").get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot -> {path}")
            return


def call(name, args=None):
    global _id
    _id += 1
    mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": _id, "method": "tools/call",
                                "params": {"name": name, "arguments": args or {}}}) + "\n")
    mcp.stdin.flush()
    while True:
        line = mcp.stdout.readline()
        if not line:
            return {}
        m = json.loads(line)
        if m.get("id") == _id:
            r = m.get("result", {})
            p = r.get("structuredContent")
            if p is None:
                txt = "".join(c.get("text", "") for c in r.get("content", [])
                              if c.get("type") == "text").strip()
                p = json.loads(txt) if txt.startswith(("{", "[")) else {}
            return p


mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                            "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                                       "clientInfo": {"name": "tab-context-menu", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


def windows():
    """`list_windows` answers with a bare list, not a `{windows: [...]}` object."""
    w = call("list_windows")
    return w if isinstance(w, list) else w.get("windows", []) if isinstance(w, dict) else []


def row(label, ns=None):
    for n in ns if ns is not None else nodes():
        if (n.get("label") or "").strip() == label:
            return n
    return None


def open_doc(label):
    """Click a binder row — the outline's selection effect opens it in the
    focused pane."""
    n = row(label)
    if not n:
        return False
    if "click" in (n.get("actions") or []):
        call("invoke_action", {"node": n["id"], "action": "click"})
    b = n.get("bounds") or {}
    if "x" in b:
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    return True


def editor_tabs(ns=None):
    """The editor tab headers, in left-to-right order.

    `role == "Tab"` alone is NOT enough: the docking activity rails are tab
    strips too ("Binder", "Search", "Inspector", …), and right-clicking one of
    those opens teksilo's own dock menu, whose labels are plausible English
    `lit!`s that a careless assertion would happily match. Filtering against the
    documents this probe opened is what keeps the two apart — the same trap
    `automation_tab_min_width.py` records.
    """
    opened = set(DOCS)
    found = []
    for n in ns if ns is not None else nodes():
        label = (n.get("label") or "").strip()
        if (n.get("role") or "") == "Tab" and label in opened:
            found.append(n)
    found.sort(key=lambda n: (n.get("bounds") or {}).get("x", 0.0))
    return found


def menu_labels(ns=None):
    ns = ns if ns is not None else nodes()
    return [str(n.get("label") or n.get("name") or "").strip()
            for n in ns if n.get("role") in ("MenuItem", "MenuListOption")]


def right_click_tab(label, timeout=15.0):
    """Right-click the editor tab named `label` and return once its menu is up.

    Three things make this deterministic rather than a sleep-and-hope:

    * **Dismiss whatever is open first.** A menu still on screen swallows the
      next press as its own dismissal, so the right-click that was meant to open
      a new menu opens nothing at all.
    * **Re-resolve the tab immediately before clicking.** A node id captured
      before the previous menu closed is a trap: `right_click` falls back to the
      node's last known bounds, and with the strip shorter than it was those
      coordinates land on the binder tree underneath — whose own context menu
      ("Add", "Convert to", "Rename"…) is plausible enough to be asserted
      against by mistake.
    * **Wait for the menu, not for a duration.** Debug builds are slow and
      uneven; polling for the rows is what makes this insensitive to that.
    """
    end = time.time() + timeout
    while time.time() < end:
        dismiss_menu()
        tab = next((t for t in editor_tabs()
                    if (t.get("label") or "").strip() == label), None)
        if tab is None:
            time.sleep(0.4)
            continue
        res = call("right_click", {"node": tab["id"]})
        if isinstance(res, dict) and res.get("isError"):
            time.sleep(0.4)
            continue
        inner = time.time() + 3.0
        while time.time() < inner:
            if "Close" in menu_labels():
                return True
            time.sleep(0.25)
    return False


def click_menu_row(text):
    """Fire the menu row whose label contains `text`. Returns False if absent."""
    for n in nodes():
        if n.get("role") in ("MenuItem", "MenuListOption") \
                and text in str(n.get("label") or n.get("name") or ""):
            call("invoke_action", {"node": n["id"], "action": "click"})
            return True
    return False


def dismiss_menu():
    call("inject_key", {"key": "Escape"})
    time.sleep(0.5)


# `load_work` is slow in a debug build — a cold open of the bundled example takes
# the better part of a minute — so wait for the binder rather than sleeping.
end = time.time() + 120
while time.time() < end:
    if row("Chapter 3"):
        break
    time.sleep(0.5)
else:
    die("the binder never populated with the example's chapters", log)

for title in DOCS:
    if not open_doc(title):
        die(f"no binder row named {title!r}", log)
    time.sleep(0.7)
time.sleep(1.2)

tabs = editor_tabs()
if len(tabs) != len(DOCS):
    die(f"expected {len(DOCS)} editor tabs, saw {[t.get('label') for t in tabs]}", log)

# ── STEP 1: the menu opens on a right-click, and names its rows ─────────────
if not right_click_tab("Chapter 2"):
    die("could not right-click the 'Chapter 2' tab — the menu is the only route "
        "to close others / pin / move a tab", log)
labels = menu_labels()
shot("/tmp/tab-menu.png")

EXPECTED = ["Close", "Close others", "Close all", "Open to the Side",
            "Move to the side", "Move into a new window", "Pin this tab"]
missing = [e for e in EXPECTED if not any(e == l for l in labels)]
if missing:
    die(f"the tab menu is missing rows {missing} — it offered {labels}", log)
print(f"  STEP 1 ok: the menu offers all {len(EXPECTED)} rows")

# ── STEP 2: it acts on the RIGHT-CLICKED tab, not the selected one ──────────
# "Chapter 3" is selected (it was opened last); the menu is open on "Chapter 2".
if not click_menu_row("Close others"):
    die("no 'Close others' row to fire", log)
time.sleep(1.4)
left = [(t.get("label") or "").strip() for t in editor_tabs()]
if left != ["Chapter 2"]:
    die("'Close others' followed the selection instead of the right-clicked tab: "
        f"expected ['Chapter 2'], got {left}", log)
print("  STEP 2 ok: the menu acted on the right-clicked tab, not the selected one")

# ── STEP 3: pinning sorts the tab first and takes its close button away ─────
for title in ["Chapter 1", "Chapter 3"]:
    if not open_doc(title):
        die(f"could not reopen {title!r}", log)
    time.sleep(0.7)
time.sleep(1.0)
if not right_click_tab("Chapter 3"):
    die(f"'Chapter 3' did not reopen; saw "
        f"{[t.get('label') for t in editor_tabs()]}", log)
if not click_menu_row("Pin this tab"):
    die("no 'Pin this tab' row to fire", log)
time.sleep(1.2)

tabs = editor_tabs()
order = [(t.get("label") or "").strip() for t in tabs]
if not order or order[0] != "Chapter 3":
    die(f"a pinned tab must sort to the head of its pane; order is {order}", log)
pinned_node = tabs[0]
# The tab keeps its NAME — the whole reason this uses `closable(false)` rather
# than teksilo's `TabInfo::pinned`, which renders icon-only at 32 dp with the
# title demoted to a tooltip. Three pinned scenes would then be three identical
# glyphs, the exact failure `MIN_EDITOR_TAB_WIDTH` exists to prevent.
if (pinned_node.get("label") or "").strip() != "Chapter 3":
    die("a pinned tab lost its title — it must stay readable in the strip", log)
shot("/tmp/tab-pinned.png")
# The pin's accident protection, asserted the way the writer meets it: teksilo
# builds the middle-click close from the same `on_close` as the cross, so a
# `closable(false)` tab has neither. If this closes the tab, the pin is
# decorative.
b = pinned_node.get("bounds") or {}
call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                        "y": b["y"] + b.get("height", 0) / 2,
                        "action": "click", "button": "middle"})
time.sleep(1.2)
if "Chapter 3" not in [(t.get("label") or "").strip() for t in editor_tabs()]:
    die("a freshly pinned tab was closed by a middle-click — `closable(false)` "
        "is not reaching the tab header", log)
print("  STEP 3 ok: the pinned tab sorts first, keeps its title, and ignores a "
      "middle-click")

# ── STEP 4: "Close all" spares the pin; the menu can still close it ─────────
others = [(t.get("label") or "").strip() for t in editor_tabs()
          if (t.get("label") or "").strip() != "Chapter 3"]
if not others:
    die("nothing left to close alongside the pinned tab", log)
if not right_click_tab(others[0]):
    die(f"could not right-click the {others[0]!r} tab", log)
if not click_menu_row("Close all"):
    die("no 'Close all' row to fire", log)
time.sleep(1.4)
left = [(t.get("label") or "").strip() for t in editor_tabs()]
if left != ["Chapter 3"]:
    die(f"'Close all' did not spare the pinned tab: {left}", log)
print("  STEP 4 ok: 'Close all' left the pinned tab open")

# A pinned tab has no cross, no middle-click close and no Delete key — this menu
# is its only way out. If the Close row failed here, a pin would be permanent.
if not right_click_tab("Chapter 3"):
    die("the pinned tab is gone or unreachable after 'Close all'", log)
labels = menu_labels()
if not any(l == "Unpin this tab" for l in labels):
    die(f"a pinned tab must be offered Unpin, not Pin; menu was {labels}", log)
if not click_menu_row("Close"):
    die("no 'Close' row on a pinned tab's menu — the pin would be permanent", log)
time.sleep(1.4)
if editor_tabs():
    die("the pinned tab could not be closed from its own menu — a pin is permanent",
        log)
print("  STEP 5 ok: a pinned tab is offered Unpin, and is still closable from the menu")

# ── STEP 6: a pin survives a restart WITH its presentation ─────────────────
# `Ctrl+S` first: `capture` writes `PaneLayout.pinned`, and it deliberately
# bails while the project is unsaved, so a pin toggled and never saved is not
# remembered — which is the app's design, not a bug to work around here.
for title in ["Chapter 1", "Chapter 2"]:
    if not open_doc(title):
        die(f"could not reopen {title!r}", log)
    time.sleep(0.7)
if not right_click_tab("Chapter 2"):
    die("could not right-click 'Chapter 2' to pin it", log)
if not click_menu_row("Pin this tab"):
    die("no 'Pin this tab' row to fire", log)
time.sleep(1.0)
# Dirty the manuscript before saving. A pin is *not* a document edit, so it
# leaves the project clean — and `editor.save` is gated on `can_save`
# (dirty && !backup mode), which makes a bare Ctrl+S here completely inert.
# `WorkspaceLayoutViewModel::capture` runs on save completion, so with no save
# there is no capture and nothing to restore. Typing one character is what
# turns the pin into something the desk is actually written down with.
editor = next((n for n in nodes()
               if n.get("role") == "MultilineTextInput"
               and "set_value" in (n.get("actions") or [])), None)
if editor is None:
    die("no editor to type into — cannot dirty the project for a save", log)
call("type_text", {"node": editor["id"], "text": "x"})
time.sleep(1.0)
call("inject_key", {"key": "s", "ctrl": True})
time.sleep(4.0)

for p in (mcp, app):
    if p.poll() is None:
        p.terminate()
time.sleep(2.0)

# Same file, same sandbox — so the same `workspace.toml`.
app = subprocess.Popen([SKRIBISTO, "--new-instance", work], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
sock = tok = None
end = time.time() + 60
while time.time() < end:
    t = open(log).read()
    sm = re.search(r"bridge socket = (\S+)", t)
    km = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", t)
    if sm and km:
        sock, tok = sm.group(1), km.group(1)
        break
    if app.poll() is not None:
        die("the app exited before announcing the bridge on restart", log)
    time.sleep(0.2)
if not sock:
    die("no bridge socket after restart", log)
time.sleep(1.0)
mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok], stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, bufsize=1)
_id = 0
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                            "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                                       "clientInfo": {"name": "tab-context-menu", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()

# The project has to finish loading before its desk can come back.
end = time.time() + 150
while time.time() < end:
    if row("Chapter 3"):
        break
    time.sleep(0.5)
else:
    die("the project never reloaded after the restart", log)

end = time.time() + 60
restored = []
while time.time() < end:
    restored = [(t.get("label") or "").strip() for t in editor_tabs()]
    if "Chapter 2" in restored:
        break
    time.sleep(0.5)
else:
    ns = nodes()
    tabs_seen = [(n.get("label") or "").strip() for n in ns
                 if (n.get("role") or "") == "Tab"]
    cfg = os.path.join(env["XDG_CONFIG_HOME"], "skribisto", "workspace.toml")
    saved = open(cfg).read() if os.path.exists(cfg) else "<no workspace.toml>"
    die(f"the desk did not restore its tabs; editor tabs {restored}, all Tab "
        f"nodes {tabs_seen}\n--- workspace.toml ---\n{saved[:1500]}", log)

if not restored or restored[0] != "Chapter 2":
    die(f"the restored pin did not sort to the head of the pane: {restored}", log)

# The regression itself: a pinned tab has no middle-click close. If the restore
# marked it pinned without repainting it, teksilo still has a live `on_close`
# for it and this closes the tab.
pinned_tab = editor_tabs()[0]
b = pinned_tab.get("bounds") or {}
call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                        "y": b["y"] + b.get("height", 0) / 2,
                        "action": "click", "button": "middle"})
time.sleep(1.2)
after_middle = [(t.get("label") or "").strip() for t in editor_tabs()]
if "Chapter 2" not in after_middle:
    die("a restored pin was closed by a middle-click — the restore marked it "
        f"pinned without repainting it, so its close affordances are all live; "
        f"tabs are now {after_middle}", log)
print("  STEP 6 ok: a pin survives a restart with its presentation, not just its flag")

# ── STEP 7: "Move into a new window" opens one, and the tab leaves ──────────
if not open_doc("Chapter 1"):
    die("could not reopen 'Chapter 1'", log)
time.sleep(1.2)
before = windows()
if not right_click_tab("Chapter 1"):
    die("'Chapter 1' did not reopen", log)
if not click_menu_row("Move into a new window"):
    die("no 'Move into a new window' row to fire", log)
time.sleep(3.0)

# The window really appears asynchronously — `open_window` is synchronous on
# the app side, but the AT tree the bridge reports catches up a beat later.
after = before
end = time.time() + 20
while time.time() < end:
    after = windows()
    if len(after) > len(before):
        break
    time.sleep(0.5)
if len(after) <= len(before):
    die(f"no second window opened: {len(before)} -> {len(after)}", log)
print(f"  STEP 7 ok: a second window opened ({len(before)} -> {len(after)}) and the "
      "tab moved into it")

cleanup()
print("PASS: the editor tab context menu closes, pins and tears off correctly, and "
      "acts on the tab the writer right-clicked")
