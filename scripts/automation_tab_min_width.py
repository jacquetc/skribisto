#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Editor tabs must stay wide enough to still name their document.

teksilo's stock `min_tab_width` is 96 dp, which fits a chapter icon, five
characters and an ellipsis. In a writing project that is a strip of identical
"Chap…" pills the moment half a dozen documents are open — the tab bar stops
naming anything, which is its one job. `app.rs`'s `MIN_EDITOR_TAB_WIDTH` raises
the floor to 160 dp: a "Chapter 12"-class title plus its hover close button,
with the bar scrolling (arrows + overflow dropdown) rather than squeezing past
it.

Both editor panes are built by one `build_pane_tabs`, so this drives BOTH —
several documents in the main pane, several more in the side pane (Ctrl+Enter
on the outline = `outline.open_to_side`) — and asserts every tab header the
accessibility tree reports is at least that wide. The side pane matters on its
own: the splitter lets it shrink to 320 dp, the narrowest place a tab is ever
laid out.
"""
import json, os, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

#: Keep in step with `MIN_EDITOR_TAB_WIDTH` in `crates/teksilo_ui/src/app.rs`.
MIN_TAB_WIDTH = 160.0
#: Documents opened in each pane. Distinct sets, so a tab's label says which
#: pane it belongs to — the strips overflow and scroll, so x positions do not.
MAIN_DOCS = ["Prologue", "Chapter 1", "Chapter 2", "Chapter 3", "Chapter 4"]
SIDE_DOCS = ["Chapter 10", "Chapter 11", "Chapter 12"]

sandbox = tempfile.mkdtemp(prefix="skribisto_tab_width_")
env = {**os.environ, "XDG_CONFIG_HOME": os.path.join(sandbox, "config"),
       "XDG_DATA_HOME": os.path.join(sandbox, "data"), "HOME": sandbox}
# This probe rolls its own sandbox (it isolates HOME/XDG_DATA_HOME too, not
# just XDG_CONFIG_HOME), so `write_settings` — not `isolated_config`, which
# would build its own separate directory — writes the settings into it. The
# tab/row labels asserted on below are document titles, not UI chrome, so the
# locale itself is not load-bearing here; it is still pinned for the same
# reason every other probe pins it: an unset `ui.locale` follows this
# machine's OS language, which is one more thing to vary between runs.
fixture.write_settings(os.path.join(sandbox, "config"), locale="en-US", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="tabwidth")
work = os.path.join(sandbox, "Starforgers.skrib")
shutil.copyfile(EXAMPLE, work)


def die(msg, log=None):
    print("FAIL:", msg)
    if log:
        print("\n".join(open(log).read().splitlines()[-20:]))
    shutil.rmtree(sandbox, ignore_errors=True)
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(work, pins=pins), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=40)
except RuntimeError as e:
    die(str(e), log)

mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, bufsize=1)
_id = 0


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
                                       "clientInfo": {"name": "tab-min-width", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


def row(label):
    """The binder row whose label is exactly `label`, or None."""
    for n in nodes():
        if (n.get("label") or "").strip() == label:
            return n
    return None


def select_row(label):
    """Click a binder row — the outline's selection effect opens it in the
    focused pane — and leave the outline focused for a following Ctrl+Enter."""
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


# The legacy fixture is migrated on open and the binder populates some way after
# the window first answers — wait for its rows rather than sleeping and hoping.
# `load_work` is slow in a debug build: a cold open of the bundled example takes
# the better part of a minute, so this budget is generous on purpose.
end = time.time() + 90
while time.time() < end:
    if row("Chapter 5"):
        break
    time.sleep(0.5)
else:
    die("the binder never populated with the example's chapters", log)

for title in MAIN_DOCS:
    if not select_row(title):
        die(f"no binder row named {title!r}", log)
    time.sleep(0.6)

# Ctrl+Enter on the outline = `outline.open_to_side`: opens the split and puts
# the selected document in the side pane.
for title in SIDE_DOCS:
    if not select_row(title):
        die(f"no binder row named {title!r}", log)
    time.sleep(0.4)
    call("inject_key", {"key": "Enter", "ctrl": True})
    time.sleep(0.8)
time.sleep(1.5)

# `Role::Tab` is not enough on its own: the docking activity rails are tab
# strips too ("Binder", "Search", "Inspector", …), and those are deliberately
# 40 dp icon-only pills owned by teksilo's dock panel. Only the documents this
# probe opened are editor tabs.
opened = set(MAIN_DOCS + SIDE_DOCS)
tabs = {}
for n in nodes():
    label = (n.get("label") or "").strip()
    if (n.get("role") or "") == "Tab" and label in opened:
        b = n.get("bounds") or {}
        if "width" in b:
            tabs[label] = b["width"]

for p in (mcp, app):
    if p.poll() is None:
        p.terminate()
shutil.rmtree(sandbox, ignore_errors=True)

if not tabs:
    die("no tab headers in the accessibility tree — nothing opened?")

missing = [t for t in MAIN_DOCS + SIDE_DOCS if t not in tabs]
if missing:
    die(f"documents that never became tabs: {missing} (saw {sorted(tabs)})")

# A dp of slack: widths come back as floats through the bridge.
narrow = {t: w for t, w in tabs.items() if w < MIN_TAB_WIDTH - 1.0}
for title, width in sorted(tabs.items()):
    pane = "side" if title in SIDE_DOCS else "main"
    print(f"  [{pane}] {title!r}: {width:.1f} dp")

if narrow:
    print(f"FAIL: {len(narrow)} tab(s) squeezed below {MIN_TAB_WIDTH:.0f} dp: "
          f"{ {t: round(w, 1) for t, w in narrow.items()} } — at that width a "
          "chapter title truncates to 'Chap…' and the strip names nothing")
    sys.exit(1)
print(f"PASS: all {len(tabs)} editor tabs across both panes are >= "
      f"{MIN_TAB_WIDTH:.0f} dp wide")
