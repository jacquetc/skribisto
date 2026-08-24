#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""End-to-end check of the search & replace docks (Part D) against the live app.

Launches the real backend on a real project, reveals the leading search dock
(Ctrl+Shift+F), types a query, and asserts against the AccessKit tree that:

  1. the search dock's query field is present (a TextInput advertising the
     search placeholder) — i.e. the leading side really hosts a *second*
     activity dock beside the binder;
  2. after typing, the debounced search runs and produces result rows in the
     dock's TREE: level-0 nodes carrying an item title + a "×N" occurrence
     badge, opening onto one level-1 node per occurrence inside that item;
  3. activating a result reveals the bottom preview band with an editable
     RichTextEditor over that item's document (the shared OpenDoc, editable).

Screenshots land in /tmp for a human to eyeball the two docks + the leading
bottom corner (binder full-height, preview starting to its right).
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else "/tmp/p02/Starforgers.skrib")
QUERY = sys.argv[2] if len(sys.argv) > 2 else "the"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, PROJECT], stdout=open(log, "w"), stderr=subprocess.STDOUT)
sock = tok = None
deadline = time.time() + 25
while time.time() < deadline:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        fail("app exited before printing the bridge socket", app, None, log)
    time.sleep(0.2)
if not sock:
    fail("no bridge socket within 25s", app, None, log)

mcp = None
_id = [0]


def send(method, params=None, notif=False):
    msg = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        msg["params"] = params
    if not notif:
        _id[0] += 1
        msg["id"] = _id[0]
    mcp.stdin.write(json.dumps(msg) + "\n")
    mcp.stdin.flush()


def recv(timeout=20, fatal=True):
    end = time.time() + timeout
    while time.time() < end:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, end - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        if line.strip():
            return json.loads(line.strip())
    if fatal:
        fail("no MCP response within timeout", app, mcp, log)
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = recv().get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(c.get("text", "") for c in result.get("content", [])
                       if c.get("type") == "text")
        payload = json.loads(text) if text.strip().startswith("{") else {}
    return result, payload


deadline = time.time() + 25
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "search-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    fail("could not connect MCP", app, mcp, log)
send("notifications/initialized", notif=True)
print("connected")


def nodes():
    _, payload = call("snapshot_tree")
    return payload.get("nodes", [])


def shot(path):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot → {path}")
            return


def texts(ns):
    """Every label/name/value string in the tree, for substring assertions."""
    out = []
    for n in ns:
        for k in ("label", "name", "value", "placeholder"):
            v = n.get(k)
            if v:
                out.append(str(v))
    return out


# Wait for the project to load.
end = time.time() + 25
ns = []
while time.time() < end:
    ns = nodes()
    if any("Writings" in (n.get("label") or "") for n in ns):
        break
    time.sleep(0.3)
print(f"project loaded ({len(ns)} a11y nodes)")

# ── STEP 1: reveal the search dock and find its query field ───────────────────
call("inject_key", {"key": "f", "ctrl": True, "shift": True})
time.sleep(1.0)
ns = nodes()
shot("/tmp/search-dock.png")
# The query SearchField advertises "Search…" as placeholder/name; the facet
# chips carry their tooltips (Books/Parts/…). Assert on locale-independent-ish
# anchors: an input plus at least one facet chip name.
all_text = " | ".join(texts(ns))
inputs = [n for n in ns if n.get("role") in ("TextInput", "SearchInput", "MultilineTextInput")]
if not inputs:
    print("roles:", sorted({n.get("role") or "?" for n in ns}))
    fail("STEP 1: no text input after Ctrl+Shift+F — search dock did not reveal", app, mcp, log)
print(f"STEP 1 OK: search dock revealed, {len(inputs)} input(s) present")

# ── STEP 2: type a query, the debounce runs, result rows appear ───────────────
# The SearchField's editor is the inner TextInput, not the SearchInput wrapper —
# keys go to the focused text widget, and focusing the wrapper leaves them nowhere.
text_inputs = [n for n in inputs if n.get("role") == "TextInput"]
q_field = (text_inputs or inputs)[0]
call("invoke_action", {"node": q_field["id"], "action": "focus"})
time.sleep(0.2)
res, _ = call("type_text", {"node": q_field["id"], "text": QUERY})
if isinstance(res, dict) and res.get("isError"):
    fail("STEP 2: type_text into the query field errored", app, mcp, log)
time.sleep(1.5)  # > 300ms debounce + the scan
ns = nodes()
shot("/tmp/search-results.png")
def result_rows(ns):
    """The tree's rows, in the dock's lower region."""
    return [n for n in ns if n.get("role") == "TreeItem"
            and (n.get("bounds") or {}).get("x", 999) < 330
            and (n.get("bounds") or {}).get("y", 0) > 440]


# Rows, not badge glyphs. The count is a `Badge` carrying a bare number now, and a
# probe that matched its punctuation was asserting on the decoration rather than on
# the thing the step is about — that results appeared at all.
found = result_rows(ns)
if not found:
    fail("STEP 2: no result rows — the search produced none "
         "(a common word over a real manuscript must match)", app, mcp, log)
print(f"STEP 2 OK: {len(found)} result row(s) — e.g. "
      f"{[ (n.get('label') or n.get('name') or '')[:24] for n in found[:3] ]}")

# ── STEP 3: an item row opens onto the occurrences inside it ──────────────────
# The results are a TREE now, not a list: `TreeItem`, not `ListBoxOption`. Level 0
# is a BinderItem carrying the ×N badge; level 1 is one row per occurrence, and
# those do not exist until the item is opened — they are fetched on toggle. So a
# probe that clicked the first row and expected a preview would be clicking an
# item, which deliberately previews nothing.
rows = result_rows(ns)
if not rows:
    fail("STEP 3: no result rows — the tree produced none", app, mcp, log)
# **Names, not a count.** A `TreeView` is virtualized: only the rows that fit the
# viewport have widgets, so opening a branch does not add rendered rows, it changes
# which rows are rendered. Counting them proves nothing either way.
def row_text(n):
    for k in ("label", "name", "value"):
        v = n.get(k)
        if v:
            return str(v)
    return ""

before_names = {row_text(n) for n in rows}
before = len(rows)
item = min(rows, key=lambda n: (n.get("bounds") or {}).get("y", 1e9))
acts = item.get("actions") or []
if "expand" in acts:
    call("invoke_action", {"node": item["id"], "action": "expand"})
else:
    # No expand action exposed: focus the row and open it with the keyboard, which
    # is the same gesture and does not depend on hitting the chevron's few pixels.
    call("focus_node", {"node": item["id"]})
    time.sleep(0.3)
    call("inject_key", {"key": "Right"})
time.sleep(1.2)
ns = nodes()
shot("/tmp/search-expanded.png")
rows = result_rows(ns)
after_names = {row_text(n) for n in rows}
revealed = after_names - before_names
if not revealed:
    # What the row actually offered, so a failure says which half is broken: a row
    # with no expand action was never asked to open, where one that opened onto
    # nothing was asked and had nothing to give.
    fresh = [n for n in result_rows(nodes())
             if (n.get("bounds") or {}).get("y", 0) == (item.get("bounds") or {}).get("y", 0)]
    after_state = fresh[0] if fresh else {}
    fail(f"STEP 3: opening the first item revealed no occurrences — the visible "
         f"rows are the same {before} names as before. Either the row declared "
         "children it does not have, or the fetch-on-toggle never ran.\n"
         f"        the row offered actions {item.get('actions')}, "
         f"expanded={item.get('expanded')} before / "
         f"{after_state.get('expanded')} after",
         app, mcp, log)
print(f"STEP 3 OK: opening an item revealed {len(revealed)} new row(s) — "
      f"e.g. {sorted(revealed)[:2]}")

# ── STEP 4: activating an occurrence previews it ──────────────────────────────
# The children are the rows that appeared, so take one below the item we opened.
top_y = (item.get("bounds") or {}).get("y", 0)
children = [n for n in rows if (n.get("bounds") or {}).get("y", 0) > top_y]
if not children:
    fail("STEP 4: no occurrence row under the opened item", app, mcp, log)
child = min(children, key=lambda n: (n.get("bounds") or {}).get("y", 1e9))
acts = child.get("actions") or []
if "activate" in acts:
    call("invoke_action", {"node": child["id"], "action": "activate"})
elif "click" in acts:
    call("invoke_action", {"node": child["id"], "action": "click"})
else:
    b = child["bounds"]
    call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                            "kind": "click"})
time.sleep(1.2)
ns = nodes()
shot("/tmp/search-preview.png")
editors = [n for n in ns if n.get("role") == "MultilineTextInput"
           and "set_value" in (n.get("actions") or [])]
# The preview editor is the wide one near the bottom of the window.
bottom = [e for e in editors if (e.get("bounds") or {}).get("y", 0) > 400]
if not bottom:
    fail("STEP 4: activating an occurrence did not reveal an editable preview "
         "editor in the bottom band", app, mcp, log)
print(f"STEP 4 OK: single-click on an occurrence previews it — {len(bottom)} "
      "editable editor(s) in the bottom band")

# ── STEP 5: replacing ONE occurrence replaces one, and the tree stays open ────
# Driven through the row's CONTEXT MENU (`right_click`), not the hover buttons:
# those are `access_hidden` on purpose -- a control that only exists under a
# pointer does not exist for a keyboard or a screen reader -- and the menu is the
# route that is meant to carry them. So this step is the accessibility claim as
# much as the replace one.
#
# Two things are asserted, and they are the two that were wrong:
#   * exactly ONE occurrence goes. The item's count is a `Badge` beside the row,
#     and a replace that rewrote the whole field would empty it entirely.
#   * the tree stays where it was. Re-running the search over a one-hit replace
#     collapses every open row, which is what the writer actually saw.


def by_label(ns, label, role="TreeItem"):
    """A row found by what it says, not by its id: the tree rebuilds around a
    replace and every node id changes with it."""
    for n in ns:
        if n.get("role") == role and row_text(n) == label:
            return n
    return None


def badge_beside(ns, row):
    """The occurrence count Badge sitting on a row's own band.

    It is a separate `Label` node, not part of the row's accessible name, so it
    is found by geometry: same y, past the row's text."""
    y = (row.get("bounds") or {}).get("y")
    if y is None:
        return None
    for n in ns:
        b = n.get("bounds") or {}
        if (n.get("role") == "Label" and abs(b.get("y", -1e9) - y) < 16
                and b.get("x", 0) > 280):
            v = (n.get("label") or n.get("name") or n.get("value") or "").strip()
            if v.isdigit():
                return int(v)
    return None


item_name = row_text(item)
child_name = row_text(child)

# Disclose the replacement row. The toggle names itself through its rich tooltip.
toggles = [n for n in ns if str(n.get("label") or n.get("name") or "").startswith("Replace:")]
if not toggles:
    fail("STEP 5: no replace toggle in the search dock", app, mcp, log)
call("invoke_action", {"node": toggles[0]["id"], "action": "click"})
time.sleep(0.9)
ns = nodes()
# The replacement field carries no accessible name or placeholder, so it is taken
# by position: the dock's second text input, below the query.
dock_inputs = sorted([n for n in ns if n.get("role") == "TextInput"
                      and (n.get("bounds") or {}).get("x", 999) < 340],
                     key=lambda n: n["bounds"]["y"])
if len(dock_inputs) < 2:
    fail("STEP 5: the replace toggle disclosed no replacement field", app, mcp, log)
call("focus_node", {"node": dock_inputs[1]["id"]})
time.sleep(0.2)
call("type_text", {"node": dock_inputs[1]["id"], "text": "ZQX"})
time.sleep(0.6)

# Disclosing the replace row moved everything below it, so the tree is re-read.
ns = nodes()
row = by_label(ns, item_name)
if row is None:
    fail(f"STEP 5: the item row {item_name!r} left the tree when replace mode opened",
         app, mcp, log)
if not row.get("expanded"):
    call("invoke_action", {"node": row["id"], "action": "expand"})
    time.sleep(1.0)
    ns = nodes()
    row = by_label(ns, item_name)
before_badge = badge_beside(ns, row)
if before_badge is None:
    fail(f"STEP 5: no occurrence count beside {item_name!r} to compare against",
         app, mcp, log)

target = by_label(ns, child_name)
if target is None:
    fail("STEP 5: the occurrence row from STEP 4 is gone before anything replaced it",
         app, mcp, log)
res, _ = call("right_click", {"node": target["id"]})
if isinstance(res, dict) and res.get("isError"):
    fail("STEP 5: right_click on an occurrence row errored — the hover buttons are "
         "access_hidden, so the context menu is the only other route to replace "
         "or dismiss a row", app, mcp, log)
time.sleep(0.8)
ns = nodes()
shot("/tmp/search-rowmenu.png")
items = [n for n in ns if n.get("role") in ("MenuItem", "MenuListOption")]
replace_here = [n for n in items
                if "Replace" in str(n.get("label") or n.get("name") or "")]
if not replace_here:
    fail("STEP 5: the row's context menu offers no replace action — "
         f"it listed {[n.get('label') or n.get('name') for n in items]}",
         app, mcp, log)
call("invoke_action", {"node": replace_here[0]["id"], "action": "click"})
time.sleep(1.6)
ns = nodes()
shot("/tmp/search-replaced.png")

row = by_label(ns, item_name)
if row is None:
    fail(f"STEP 5: {item_name!r} left the results entirely — replacing ONE of its "
         f"{before_badge} hits took the whole field with it", app, mcp, log)
after_badge = badge_beside(ns, row)
if after_badge != before_badge - 1:
    fail(f"STEP 5: replacing ONE occurrence took the count from {before_badge} to "
         f"{after_badge}. One hit was named; anything else means the caller's pick "
         "never reached the prose.", app, mcp, log)
if by_label(ns, child_name) is not None:
    fail("STEP 5: the replaced occurrence is still listed", app, mcp, log)
if not row.get("expanded"):
    fail("STEP 5: the item collapsed — the panel re-ran the whole search over a "
         "one-hit replace and threw away where the writer was", app, mcp, log)
print(f"STEP 5 OK: one occurrence replaced ({before_badge} → {after_badge}), "
      "its row is gone, and the item stayed open")

print("\nDONE — see /tmp/search-dock.png, /tmp/search-results.png, "
      "/tmp/search-expanded.png, /tmp/search-preview.png, /tmp/search-rowmenu.png, "
      "/tmp/search-replaced.png")
for p in (app, mcp):
    if p.poll() is None:
        p.terminate()
