#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the mocks build and check the **Resources** feature where a writer meets it.

A resource is a file or a web link attached to binder items. The view-model and the
model have their headless tests; what only a live app shows is that each surface is
wired to them, with the labels the writer reads:

  1. **The Resources panel** opens with Ctrl+Shift+X and lists the resources of the
     item in front, and only those: Chapter Two carries the village map and the
     harbour records, and opening Scene 1 instead switches the list to the
     lighthouse photograph.
  2. **A resource's menu** (the button at the end of its line) offers Open, Rename
     and describe…, Attach to items…, Detach from this item and Remove from the
     project…. The menu is only opened and dismissed: nothing is detached or removed.
  3. **A Book folder's Resources view** (the segment named Resources) lists the
     resources of the folder and of everything in it, each once, and switches
     between List and Grid.
  4. **Settings ▸ Work ▸ Resources** lists the two resources attached to nothing.
  5. **Global search** has its Resources switch, named by its tooltip
     (`search-tip-resource`), and the switch turns on and off.

Mocks only: the rows come from the `mocks` `imp` of
`crates/teksilo_ui/src/models/resources_list_model.rs` over the mock binder (ids
101 Book One, 104 Chapter Two, 201 Scene 1, 203 Scene 3, 105 Confrontation, 106
Character sketch). Run bare after `cargo build -p teksilo_ui --features mocks`; it
skips (exit 77) on a build without the mocks' Mock Project.

    python3 scripts/automation_resources.py

The app runs in a private settings sandbox with the locale pinned to en-US, and
`--new-instance`, so it never meets the operator's own Skribisto. At the end it
stops the one process it started, by its pid, and nothing else.
"""
import json
import os
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

# The mock rows, by title (`resources_list_model.rs`, the `mocks` `imp`).
CHAPTER_TWO = {"Village map", "Harbour records"}
SCENE_ONE = {"Lighthouse at dusk"}
# Book One's subtree: the map (on the Book and Chapter Two, listed once), the
# photograph (Scene 1), the recording (Scene 3), the moved tide tables
# (Confrontation) and the web page (Chapter Two).
BOOK_SUBTREE = {"Village map", "Lighthouse at dusk", "Interview with the keeper",
                "Tide tables 1887", "Harbour records"}
UNATTACHED = {"Keeper's cottage", "Lighthouse history"}
ALL_TITLES = BOOK_SUBTREE | UNATTACHED

# en-US `resources.ftl`.
MENU_LABELS = ["Open", "Rename and describe…", "Attach to items…",
               "Detach from this item", "Remove from the project…"]
# en-US `main.ftl`, `search-tip-resource`: the switch's accessible name is its tooltip.
SEARCH_TIP = ("Resources: search the titles, descriptions, file names and web addresses "
              "of the files and links attached to your items. Their matches are listed so "
              "you can find them, and a replace never changes them.")

fixture.assert_no_running_instance(SKRIBISTO)
env = fixture.isolated_config(locale="en-US", label="resources", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="resources")
log = tempfile.NamedTemporaryFile(prefix="resources-", suffix=".log", delete=False,
                                  dir=fixture.SCRATCH if os.path.isdir(fixture.SCRATCH)
                                  else None).name
app = subprocess.Popen(fixture.launch_argv(None, pins=pins), env=env,
                       stdout=open(log, "w"), stderr=subprocess.STDOUT)
session = None
failures = []


def stop():
    """Stop what this probe started, and only that: the MCP client, then the app by pid."""
    if session is not None:
        session.close()
    if app.poll() is None:
        app.terminate()
        try:
            app.wait(timeout=15)
        except subprocess.TimeoutExpired:
            app.kill()
            app.wait(timeout=15)


def die(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log, errors="replace").read().splitlines()[-25:]))
    stop()
    sys.exit(1)


def check(ok, what):
    print(("PASS: " if ok else "FAIL: ") + what)
    if not ok:
        failures.append(what)
    return ok


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
    session = fixture.McpSession(bridge, "resources-probe", MCP)
except (RuntimeError, fixture.McpError) as e:
    die(str(e))
print(f"bridge up: {bridge.endpoint}")


def call(tool, args=None):
    try:
        return session.call(tool, args)
    except fixture.McpError as e:
        die(str(e))


def nodes():
    return call("snapshot_tree").get("nodes", [])


def settle():
    call("settle")
    time.sleep(0.3)


def label(n):
    return (n.get("label") or "").strip()


def bounds(n):
    return n.get("bounds") or {}


def click(n):
    """A click the way the writer gives it: the node's own action when it has one,
    else the pointer at its centre."""
    if "click" in (n.get("actions") or []):
        call("invoke_action", {"node": n["id"], "action": "click"})
    else:
        b = bounds(n)
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    settle()


def wait_for(pred, timeout=10.0):
    """The first node `pred` accepts, polling until `timeout`; None if none came."""
    end = time.time() + timeout
    while time.time() < end:
        for n in nodes():
            if pred(n):
                return n
        time.sleep(0.4)
    return None


def tree_item(title):
    return wait_for(lambda n: n.get("role") == "TreeItem" and label(n) == title)


def resource_nodes(ns, on_items):
    """The lines and tiles named like a resource: "<title>, <kind>, <detail>", plus
    ", on <items>" on a folder's page (`resources-row-name-attached`). Returns
    {title: [node, …]}.

    A line is a list option (or a grid cell); the same name also sits on the
    line's own content, an `Unknown` node, which is skipped so a resource shown
    once counts once."""
    out = {}
    for n in ns:
        name = label(n)
        if n.get("role") == "Unknown":
            continue
        for title in ALL_TITLES:
            if name.startswith(title + ", ") and ((", on " in name) == on_items):
                out.setdefault(title, []).append(n)
    return out


def dock_titles(timeout=8.0, want=None):
    """The titles the Resources panel lists, waiting until they read `want`."""
    end = time.time() + timeout
    got = set()
    while time.time() < end:
        got = set(resource_nodes(nodes(), on_items=False))
        if want is None or got == want:
            return got
        time.sleep(0.4)
    return got


def text_present(text):
    """True when `text` is drawn somewhere: a plain text is not an accessible node
    of its own, so this looks through the layout tree's resolved texts too."""
    for n in nodes() + call("layout_tree").get("nodes", []):
        for field in ("label", "value", "text", "resolved_text"):
            if (n.get(field) or "") == text:
                return True
    return False


def wait_text(text, timeout=4.0):
    end = time.time() + timeout
    while time.time() < end:
        if text_present(text):
            return True
        time.sleep(0.4)
    return False


def dump(ns, why):
    print(f"  --- nodes ({why}) ---")
    for n in ns:
        if label(n):
            print(f"  {n.get('role')!s:<14} {label(n)[:110]!r} {bounds(n)}")


# ── reach the Mock Project ───────────────────────────────────────────────────
row = wait_for(lambda n: label(n) == "Mock Project", timeout=20)
if row is None:
    dump(nodes(), "Launcher")
    stop()
    fixture.skip("not a `--features mocks` build: the Launcher has no Mock Project")
click(row)
if not tree_item("Chapter Two"):
    die("clicking 'Mock Project' never brought up a binder holding Chapter Two")
print("the Mock Project is open")

# ── 1. the panel lists the item in front's resources ─────────────────────────
print("\n[1] the Resources panel")
click(tree_item("Chapter Two"))
time.sleep(0.8)
call("inject_key", {"key": "X", "command": True, "shift": True})
settle()
got = dock_titles(want=CHAPTER_TWO)
if not check(got == CHAPTER_TWO,
             f"Ctrl+Shift+X opens the panel on Chapter Two's resources "
             f"{sorted(CHAPTER_TWO)} (saw {sorted(got)})"):
    dump(nodes(), "after Ctrl+Shift+X on Chapter Two")

scene = tree_item("Scene 1")
if scene is None:
    die("no Scene 1 in the binder")
click(scene)
got = dock_titles(want=SCENE_ONE)
check(got == SCENE_ONE,
      f"with Scene 1 in front the panel lists {sorted(SCENE_ONE)} (saw {sorted(got)})")

# ── 2. a resource's menu ─────────────────────────────────────────────────────
print("\n[2] a resource's menu")
line = resource_nodes(nodes(), on_items=False).get("Lighthouse at dusk", [None])[0]
if line is None:
    die("the panel shows no line for Lighthouse at dusk")
lb = bounds(line)


def inside(n, b):
    nb = bounds(n)
    if "x" not in nb or "x" not in b:
        return False
    cx, cy = nb["x"] + nb.get("width", 0) / 2, nb["y"] + nb.get("height", 0) / 2
    return (b["x"] <= cx <= b["x"] + b.get("width", 0)
            and b["y"] <= cy <= b["y"] + b.get("height", 0))


def within(n, b):
    """Wholly inside `b`. In Settings this matters: the Resources panel behind the
    dialog lists its own lines, and their centres can fall inside the dialog's box."""
    nb = bounds(n)
    return ("x" in nb and "x" in b and nb["x"] >= b["x"] and nb["y"] >= b["y"]
            and nb["x"] + nb.get("width", 0) <= b["x"] + b.get("width", 0)
            and nb["y"] + nb.get("height", 0) <= b["y"] + b.get("height", 0))


menu_button = next((n for n in nodes() if label(n) == "Actions" and inside(n, lb)), None)
if menu_button is None:
    dump(nodes(), "looking for the line's Actions button")
    die("the Lighthouse at dusk line has no Actions button at its end")
click(menu_button)
time.sleep(0.5)
ns = nodes()
offered = [label(n) for n in ns if str(n.get("role")).startswith("MenuItem")]
missing = [m for m in MENU_LABELS if m not in offered]
if not check(not missing,
             "the line's menu offers " + ", ".join(MENU_LABELS)
             + (f" (missing {missing})" if missing else "")):
    dump(ns, "with the menu open")
call("inject_key", {"key": "Escape"})
settle()

# ── 3. a Book folder's Resources view ────────────────────────────────────────
print("\n[3] Book One's Resources view")
click(tree_item("Book One"))
bar = wait_for(lambda n: n.get("role") == "RadioGroup" and label(n) == "Views", timeout=10)
if bar is None:
    dump(nodes(), "Book One open")
    die("Book One's page has no bar of views")
segment = wait_for(lambda n: n.get("role") == "RadioButton" and label(n) == "Resources",
                   timeout=2)
if segment is None:
    # A Book has more views than a narrow page shows: the rest are behind the bar's
    # own More options button, as menu entries under the same names.
    more = next((n for n in nodes() if label(n) == "More options" and inside(n, bounds(bar))),
                None)
    if more is None:
        dump(nodes(), "Book One open")
        die("Book One's bar of views shows no Resources segment and no More options")
    click(more)
    segment = wait_for(lambda n: str(n.get("role")).startswith("MenuItem")
                       and label(n) == "Resources",
                       timeout=5)
if segment is None:
    dump(nodes(), "Book One open, its views' overflow open")
    die("Book One's page has no view named Resources")
click(segment)


def folder_titles(want, timeout=8.0):
    end = time.time() + timeout
    found = {}
    while time.time() < end:
        found = resource_nodes(nodes(), on_items=True)
        if set(found) == want:
            break
        time.sleep(0.4)
    return found


found = folder_titles(BOOK_SUBTREE)
once = all(len(v) == 1 for v in found.values())
if not check(set(found) == BOOK_SUBTREE and once,
             f"the List shows Book One's subtree resources {sorted(BOOK_SUBTREE)}, "
             f"each once (saw { {k: len(v) for k, v in found.items()} })"):
    dump(nodes(), "Book One ▸ Resources")
check(wait_text("5 resources"), "its header counts 5 resources")
map_line = found.get("Village map", [{}])[0]
check(all(item in label(map_line) for item in ("Book One", "Chapter Two")),
      f"the map's line names both items it is on ({label(map_line)!r})")


def view_segment(name):
    return wait_for(lambda n: n.get("role") == "RadioButton" and label(n) == name, timeout=5)


def selected(n):
    return str(n.get("toggled")).lower() == "true"


list_seg, grid_seg = view_segment("List"), view_segment("Grid")
if list_seg is None or grid_seg is None:
    dump(nodes(), "looking for List / Grid")
    die("the Resources view has no List / Grid switch")
check(selected(list_seg) and not selected(grid_seg), "the view opens as a List")
list_roles = {n.get("role") for v in found.values() for n in v}
click(grid_seg)
time.sleep(0.6)
grid_seg = view_segment("Grid")
check(grid_seg is not None and selected(grid_seg), "choosing Grid selects it")
tiles = folder_titles(BOOK_SUBTREE)
grid_roles = {n.get("role") for v in tiles.values() for n in v}
# The grid itself is named Resources (`resources-grid-label`) and holds the tiles; the
# panel on the right is named Resources too, so the name alone would prove nothing.
tile = next(iter(tiles.values()), [{}])[0]
grid = wait_for(lambda n: label(n) == "Resources" and n.get("role") != "TabPanel"
                and within(tile, bounds(n)), timeout=3)
if not check(set(tiles) == BOOK_SUBTREE and grid is not None
             and list_roles == {"ListBoxOption"} and grid_roles == {"GridCell"},
             f"the Grid shows the same five resources as tiles of a grid named Resources "
             f"(tiles {sorted(tiles)}, grid {grid.get('role') if grid else None}, "
             f"roles {sorted(map(str, list_roles))} → {sorted(map(str, grid_roles))})"):
    dump(nodes(), "Grid")
click(view_segment("List"))
time.sleep(0.6)
back = folder_titles(BOOK_SUBTREE)
check(set(back) == BOOK_SUBTREE
      and {n.get("role") for v in back.values() for n in v} == list_roles,
      "choosing List brings the list back")

# ── 4. Settings ▸ Work ▸ Resources ───────────────────────────────────────────
print("\n[4] Settings ▸ Work ▸ Resources")
call("inject_key", {"key": ",", "command": True})
if wait_for(lambda n: label(n) == "Reset to defaults", timeout=15) is None:
    die("Ctrl+, opened no Settings")


def settings_row(title):
    return next((n for n in nodes() if n.get("role") == "TreeItem" and label(n) == title), None)


page = settings_row("Resources")
if page is None:
    work = next((n for n in nodes() if n.get("role") == "TreeItem"
                 and label(n).startswith("Work: ")), None)
    if work is None:
        dump(nodes(), "Settings")
        die("Settings has no Work: <title> section")
    call("invoke_action", {"node": work["id"], "action": "expand"})
    settle()
    page = settings_row("Resources")
if page is None:
    die("Settings ▸ Work has no Resources page")
click(page)
dialog = wait_for(lambda n: n.get("role") == "Dialog" and label(n) == "Settings", timeout=3)
if dialog is None:
    die("Settings is not a dialog named Settings")



end = time.time() + 8
loose = {}
while time.time() < end:
    loose = resource_nodes([n for n in nodes() if within(n, bounds(dialog))],
                           on_items=False)
    if set(loose) == UNATTACHED:
        break
    time.sleep(0.4)
if not check(set(loose) == UNATTACHED,
             f"the page lists the two resources attached to nothing {sorted(UNATTACHED)} "
             f"(saw {sorted(loose)})"):
    dump(nodes(), "Settings ▸ Work ▸ Resources")
check(wait_text("Attached to nothing") and wait_text("2 resources"),
      "under its Attached to nothing heading, counted as 2 resources")
call("inject_key", {"key": "Escape"})
settle()
time.sleep(0.6)

# ── 5. the search switch ─────────────────────────────────────────────────────
print("\n[5] the Resources switch in global search")
call("inject_key", {"key": "F", "command": True, "shift": True})
switch = wait_for(lambda n: label(n).startswith("Resources:"), timeout=10)
if switch is None:
    dump(nodes(), "after Ctrl+Shift+F")
    die("the search panel has no switch named Resources:…")
check(label(switch) == SEARCH_TIP,
      "the search options have a Resources switch named by `search-tip-resource`")
before = selected(switch)
click(switch)
after_node = wait_for(lambda n: n["id"] == switch["id"], timeout=3)
after = selected(after_node) if after_node else before
check(after != before, f"clicking it turns it {'off' if before else 'on'} "
      f"(toggled {before} → {after})")
click(after_node or switch)

print()
stop()
if failures:
    print(f"FAIL: {len(failures)} check(s) failed:")
    for f in failures:
        print(" -", f)
    sys.exit(1)
print("PASS: every Resources check passed")
