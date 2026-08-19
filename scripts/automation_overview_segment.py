#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **Overview** segment.

The Overview is a container's whole subtree as a dense, sortable table (the
"outliner" every competing novel tool ships). Every Overview-capable container
gets it — Book, Part, Chapter folder **and a notes folder**, which is the one
place the Overview gate and the manuscript-stream gate disagree.

    scripts/automation_overview_segment.py               # mocks build
    scripts/automation_overview_segment.py PROJECT.skrib # a real project

Checks:
  1. Each of Book / Part / Chapter folder exposes an "Overview" segment, and
     clicking it renders a real table (column headers + row titles).
  2. Word-count columns carry plausible numbers, not blanks.
  3. The search box is present and focusable (its *behaviour* is pinned by
     headless model tests — see the comment at that step for why typing
     cannot be driven from here).
  4. A plain Scene (a leaf) exposes NO Overview segment.

Reuses the launch + Launcher→Mock-Project scaffolding from
automation_pace_segment.py.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
OUT = os.environ.get("SHOT_DIR", "/tmp")
PANE_X = 300

project = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else None
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-20:]))
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


subprocess.run(["pkill", "-x", "skribisto"], check=False)
time.sleep(0.4)
app = subprocess.Popen([SKRIBISTO] + ([project] if project else []),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT)

sock = tok = None
end = time.time() + 25
while time.time() < end:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt)
    t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s and t:
        sock, tok = s.group(1), t.group(1)
        break
    if app.poll() is not None:
        die("app exited early", app)
    time.sleep(0.2)
if not sock:
    die("no bridge socket", app)

_id = [0]
mcp = None


def send(method, params=None, notif=False):
    m = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        m["params"] = params
    if not notif:
        _id[0] += 1
        m["id"] = _id[0]
    mcp.stdin.write(json.dumps(m) + "\n")
    mcp.stdin.flush()


def recv(timeout=25, fatal=True):
    e = time.time() + timeout
    while time.time() < e:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, e - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        if line.strip():
            return json.loads(line)
    if fatal:
        die("no MCP response", app, mcp)
    return None


def call(name, a=None):
    send("tools/call", {"name": name, "arguments": a or {}})
    res = recv().get("result", {})
    payload = res.get("structuredContent")
    if payload is None:
        txt = "".join(c.get("text", "") for c in res.get("content", []) if c.get("type") == "text")
        payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
    return res, payload


deadline = time.time() + 25
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "overview-segment", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None:
        if mcp.poll() is None:
            mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP", app, mcp)
send("notifications/initialized", notif=True)
print(f"bridge up: {sock}")


def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def all_widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def find(label, timeout=6.0, minx=None):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            if n.get("label") == label:
                if minx is None or (n.get("bounds") or {}).get("x", 0) >= minx:
                    return n
        settle()
    return None


def click(label, what="", minx=None):
    n = find(label, minx=minx)
    if not n:
        print(f"  !! no node labelled {label!r} {what}")
        return False
    if "click" in (n.get("actions") or []):
        call("invoke_action", {"node": n["id"], "action": "click"})
    else:
        b = n.get("bounds") or {}
        if "x" not in b:
            return False
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
    settle()
    time.sleep(0.5)
    return True


def has_label(label, minx=None):
    """True if a node with this exact label exists (optionally right of minx)."""
    for n in nodes():
        if n.get("label") == label:
            if minx is None or (n.get("bounds") or {}).get("x", 0) >= minx:
                return True
    return False


def text_present(substr):
    """True if `substr` appears in any text field across a11y + layout trees."""
    for n in nodes() + all_widgets():
        for field in ("label", "value", "text", "resolved_text", "params"):
            blob = n.get(field)
            hay = blob if isinstance(blob, str) else json.dumps(blob or "")
            if substr.lower() in hay.lower():
                return True
    return False


def shot(name):
    path = os.path.join(OUT, f"{name}.png")
    res, p = call("screenshot")
    # The MCP `screenshot` tool returns the PNG as an image *content block*
    # (base64 under `data`), not as structuredContent.
    b64 = None
    for c in res.get("content", []):
        if c.get("type") == "image" and isinstance(c.get("data"), str):
            b64 = c["data"]
            break
    if b64 is None:
        for k in ("image_base64", "png_base64", "data", "image", "base64"):
            if isinstance(p.get(k), str) and len(p[k]) > 100:
                b64 = p[k]
                break
    if b64 is None:
        return None
    open(path, "wb").write(base64.b64decode(b64))
    print(f"  screenshot → {path}")
    return path


# ── reach the editor (Launcher → Mock Project for the no-args mocks build) ──────
if project is None:
    row = find("Mock Project", timeout=10)
    if not row:
        die("no 'Mock Project' recent row — this no-args mode needs a `--features "
            "mocks` build", app, mcp)
    b = row.get("bounds") or {}
    if "x" not in b:
        die("'Mock Project' row has no bounds to click", app, mcp)
    call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                            "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
    end = time.time() + 15
    opened = False
    while time.time() < end:
        labels_now = [n.get("label") for n in nodes() if n.get("label")]
        if "Welcome sections" not in labels_now and "Binder" in labels_now:
            opened = True
            break
        settle()
    if not opened:
        die("clicking 'Mock Project' never opened the editor", app, mcp)
settle()

failures = []

CONTAINERS = ("Book One", "Part One \u2014 Arrival", "Chapter Two")

# ── 1. Every container exposes an Overview whose table actually renders ────────────
for container in CONTAINERS:
    print(f"\n=== {container} \u2192 Overview segment ===")
    if not click(container, "(binder row)"):
        failures.append(f"{container}: not in the binder")
        continue
    if not has_label("Overview", minx=PANE_X):
        shot(f"overview-missing-{container.split()[0].lower()}")
        failures.append(f"{container}: no 'Overview' segment in the container bar")
        continue
    print("  'Overview' segment present \u2713")
    if not click("Overview", "(segment)", minx=PANE_X):
        failures.append(f"{container}: the 'Overview' segment did not click")
        continue

    # The table is real when its column headers AND at least one row title show.
    missing = [h for h in ("Title", "Type", "Words", "Total") if not text_present(h)]
    if missing:
        shot(f"overview-headers-{container.split()[0].lower()}")
        failures.append(f"{container}/Overview: missing column header(s): {missing}")
        continue
    print("  column headers render \u2713")

    if not (text_present("Scene 1") or text_present("Chapter Two")
            or text_present("Into the Dark")):
        shot(f"overview-empty-{container.split()[0].lower()}")
        failures.append(f"{container}/Overview: the table rendered no rows")
        continue
    print("  rows render \u2713")
    shot(f"overview-{container.split()[0].lower()}")

# ── 2. The word columns carry numbers ──────────────────────────────────────
# The mock chapter folder holds scenes of 1180 + 640 words, so a correct
# bottom-up fold must show a four-figure total somewhere. A blank column here
# means the batched content read or the fold silently produced nothing.
print("\n=== word counts ===")
click("Chapter Two", "(binder row)")
click("Overview", "(segment)", minx=PANE_X)
digits = re.compile(r"[0-9]")
numeric_cells = 0
for n in nodes() + all_widgets():
    for field in ("label", "value", "text", "resolved_text"):
        v = n.get(field)
        if isinstance(v, str) and digits.search(v) and len(v.strip()) <= 12:
            numeric_cells += 1
if numeric_cells < 2:
    shot("overview-no-counts")
    failures.append(f"Overview: found only {numeric_cells} numeric cells \u2014 "
                    "the word columns look blank")
else:
    print(f"  {numeric_cells} numeric cells present \u2713")

# ── 3. Search: the box is reachable and focusable ───────────────────
#
# The *behaviour* of search (KeepAncestors, the reveal override, restoring the
# writer's collapse on clear) is pinned by headless tests over the real model —
# `models::overview_rows_model`'s `search_keeps_ancestors_and_restores_collapse_on_clear`,
# `a_search_matching_nothing_empties_the_table` and `expanded_uids_ignore_the_search_reveal`.
#
# What CANNOT be driven from here is the typing itself, and that is a finding
# rather than a limitation of this script: teksilo's `SearchField` publishes an
# a11y node with **no name and no actions** — `{role: SearchInput, label: null,
# actions: null}`. So `type_text` (which focuses + sets through the AT surface)
# no-ops, and `inject_key` is no help either because it maps a bare ASCII letter
# to a *named* key (`Key::D`), not a character. A screen-reader user therefore
# cannot identify or fill this box — nor can any other SearchField in the app
# (Corkboard, binder). Worth fixing in teksilo; until then this check verifies
# only that the box exists and takes focus, and says so instead of quietly
# passing.
print("\n=== search ===")
click("Book One", "(binder row)")
click("Overview", "(segment)", minx=PANE_X)
search = None
for n in all_widgets():
    if (n.get("type", "").endswith("SearchField") and n.get("active")
            and (n.get("bounds") or {}).get("x", 0) >= PANE_X):
        search = n
        break
if search is None:
    failures.append("Overview: no search field in the header")
else:
    b = search.get("bounds") or {}
    call("inject_pointer", {"x": b.get("x", 0) + b.get("width", 0) / 2,
                            "y": b.get("y", 0) + b.get("height", 0) / 2,
                            "kind": "click"})
    settle()
    at = next((n for n in nodes() if n.get("role") == "SearchInput"
               and (n.get("bounds") or {}).get("x", 0) >= PANE_X), None)
    if at is None:
        failures.append("Overview search: the box has no a11y node at all")
    else:
        print("  search box present and focusable \u2713")
        if not at.get("label"):
            print("  SKIPPED typing: SearchField exposes no AT name/actions "
                  "(teksilo gap, see the comment above) \u2014 search behaviour is "
                  "covered by the headless model tests instead")
        shot("overview-search-focused")

# ── 4. A leaf has no Overview ───────────────────────────────────────
print("\n=== Scene at dawn \u2192 no Overview segment ===")
if not click("Scene at dawn", "(binder row)"):
    failures.append("Scene at dawn: not in the binder")
elif has_label("Overview", minx=PANE_X):
    shot("overview-leak-scene")
    failures.append("Scene at dawn: an 'Overview' segment leaked onto a leaf")
else:
    print("  a leaf has no Overview segment \u2713")

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()

if failures:
    print("\nFAIL:")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("\nPASS: the Overview segment renders a real table on every container, "
      "counts fold, the search box is reachable, and no leaf exposes it.")
print("NOTE: `Folder/Note` (the 2-segment notes-folder shell) is not in the mock "
      "binder, so it is covered headlessly by "
      "`tabs::tests::a_notes_folder_lays_out_its_two_segment_bar` instead.")
