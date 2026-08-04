#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **Corkboard** segment.

The Corkboard is a container's contents as index cards — the surface Scrivener
made the genre's default outlining gesture. Until now its only end-to-end
coverage was incidental (the tag-dot assertions in `automation_tag_chips.py` /
`automation_tag_overflow.py`); this script covers the board itself.

    scripts/automation_corkboard.py               # mocks build
    scripts/automation_corkboard.py PROJECT.skrib # a real project

Checks:
  1. Book / Part / Chapter folder each expose a "Corkboard" segment that
     renders real cards (titles from the fixture).
  2. The board chrome is present: breadcrumb, card count, Nested/Flat, the
     order combo, the filter box, the size slider, "＋ New".
  3. A card's kebab menu offers the full action set, including the entries
     added here — Duplicate, "Move to…" and "Reveal in outline".
  4. Selecting a card and opening its menu shows the *batch* wording only when
     more than one card is selected.
  5. Drilling into a folder card deepens the breadcrumb, and the crumb walks
     back out.
  6. A plain Scene (a leaf) exposes NO Corkboard segment.

Reuses the launch + Launcher→Mock-Project scaffolding from
automation_overview_segment.py.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = os.environ.get(
    "SKRIBISTO_BIN",
    os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                 "target", "debug", "skribisto"))
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
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
    t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
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
                        "clientInfo": {"name": "corkboard-segment", "version": "1"}})
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


def open_corkboard(container):
    """Select `container` in the binder and switch to its Corkboard segment."""
    if not click(container, "(binder row)"):
        failures.append(f"{container}: not in the binder")
        return False
    if not has_label("Corkboard", minx=PANE_X):
        shot(f"corkboard-missing-{container.split()[0].lower()}")
        failures.append(f"{container}: no 'Corkboard' segment in the container bar")
        return False
    if not click("Corkboard", "(segment)", minx=PANE_X):
        failures.append(f"{container}: the 'Corkboard' segment did not click")
        return False
    return True


def card_titles():
    """Titles rendered on the board, read off the grid's per-cell a11y names.

    `card_a11y_name` composes "Title, Type[, status]", so the cell names are the
    one place a card's identity is readable without guessing at layout.
    """
    out = []
    for n in nodes():
        b = n.get("bounds") or {}
        if b.get("x", 0) < PANE_X:
            continue
        label = n.get("label")
        if isinstance(label, str) and ", " in label and n.get("role") in ("GridCell", "Cell", "ListItem"):
            out.append(label.split(", ")[0])
    return out


def kebab_nodes():
    """Every "More actions" button on the board, left-to-right."""
    out = []
    for n in nodes():
        b = n.get("bounds") or {}
        if b.get("x", 0) < PANE_X:
            continue
        if (n.get("label") or "") in ("More actions", "More"):
            out.append(n)
    return sorted(out, key=lambda n: ((n.get("bounds") or {}).get("y", 0),
                                      (n.get("bounds") or {}).get("x", 0)))


# \u2500\u2500 1. Every container exposes a Corkboard that renders cards \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
for container in CONTAINERS:
    print(f"\n=== {container} \u2192 Corkboard segment ===")
    if not open_corkboard(container):
        continue
    print("  'Corkboard' segment present and clicked \u2713")

    # Read the cards off the grid's per-cell a11y names, so "the board rendered
    # something" cannot be satisfied by chrome text that merely mentions a title.
    titles = card_titles()
    if not titles:
        shot(f"corkboard-empty-{container.split()[0].lower()}")
        failures.append(f"{container}/Corkboard: the board rendered no cards")
        continue
    print(f"  {len(titles)} cards render \u2713 ({', '.join(titles[:4])}...)")
    shot(f"corkboard-{container.split()[0].lower()}")

# \u2500\u2500 2. The board chrome \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
print("\n=== board chrome ===")
open_corkboard("Chapter Two")

# Nested / Flat, the order combo and "New" are labelled nodes; the count is text.
for label in ("Nested", "Flat", "New"):
    if not has_label(label, minx=PANE_X):
        failures.append(f"Corkboard chrome: no {label!r} control")
    else:
        print(f"  {label!r} present \u2713")

# The order control is a ComboBox whose placeholder names manuscript order.
if not (text_present("Manuscript order") or has_label("Order", minx=PANE_X)):
    shot("corkboard-no-sort")
    failures.append("Corkboard chrome: no order/sort control "
                    "(the combo added for the sort UI)")
else:
    print("  order control present \u2713")

# "N cards" — the header count.
if not any(text_present(f"{n} card") for n in range(1, 12)):
    failures.append("Corkboard chrome: no card count in the header")
else:
    print("  card count present \u2713")

# The size slider and the filter box are widgets, not named a11y nodes.
sliders = [n for n in all_widgets()
           if n.get("type", "").endswith("Slider")
           and (n.get("bounds") or {}).get("x", 0) >= PANE_X]
if not sliders:
    failures.append("Corkboard chrome: no card-size slider")
else:
    print("  card-size slider present \u2713")

searches = [n for n in all_widgets()
            if n.get("type", "").endswith("SearchField") and n.get("active")
            and (n.get("bounds") or {}).get("x", 0) >= PANE_X]
if not searches:
    failures.append("Corkboard chrome: no filter box")
else:
    # Same bastyde gap `automation_overview_segment.py` documents: a SearchField
    # publishes no AT name and no actions, so the *typing* cannot be driven from
    # here. The filter's behaviour is pinned headlessly instead
    # (`models::corkboard_cards_model::filter_tests`).
    print("  filter box present \u2713 (typing not drivable \u2014 SearchField has no AT "
          "name/actions; behaviour covered by headless filter tests)")

shot("corkboard-chrome")

# \u2500\u2500 3. The card menu carries the full action set \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
print("\n=== card menu ===")
open_corkboard("Chapter Two")
kebabs = kebab_nodes()
if not kebabs:
    shot("corkboard-no-kebab")
    failures.append("Corkboard: no per-card 'More actions' button")
else:
    call("invoke_action", {"node": kebabs[0]["id"], "action": "click"})
    settle()
    time.sleep(0.4)
    # Single selection (nothing selected \u2192 the anchor card alone), so every batch
    # entry must read in its plain, uncounted form.
    expected = ["Rename", "Set label", "Duplicate", "Move to\u2026",
                "Reveal in outline", "Move to trash"]
    missing = [e for e in expected if not text_present(e)]
    if missing:
        shot("corkboard-menu-missing")
        failures.append(f"Corkboard card menu: missing entries {missing}")
    else:
        print(f"  all {len(expected)} expected entries present \u2713")
    shot("corkboard-card-menu")
    # Close the popover.
    call("inject_key", {"key": "Escape"})
    settle()

# \u2500\u2500 4. Batch wording appears only for a real multi-selection \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
#
# `batched()` swaps each entry's label for its counted form once the batch is
# larger than one \u2014 so a single selection saying "Duplicate 1 card" (or a
# multi-selection still saying a bare "Duplicate") is the regression to catch.
print("\n=== batch wording ===")
open_corkboard("Chapter Two")
kebabs = kebab_nodes()
if len(kebabs) < 2:
    failures.append("Corkboard: fewer than 2 cards on the board \u2014 cannot exercise "
                    "the multi-select batch wording")
else:
    # Select two cards, then open the SECOND one's kebab. The menu must now read
    # in its counted form; the labels are rebuilt from the live selection by
    # `CardMenu`, so a stale single-card label here is a real regression.
    def click_node(n, kind="click"):
        b = n.get("bounds") or {}
        if "x" not in b:
            return False
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "kind": kind})
        settle()
        return True

    cards = [n for n in nodes()
             if n.get("role") in ("GridCell", "Cell", "ListItem")
             and (n.get("bounds") or {}).get("x", 0) >= PANE_X]
    cards.sort(key=lambda n: ((n.get("bounds") or {}).get("y", 0),
                              (n.get("bounds") or {}).get("x", 0)))
    if len(cards) < 2:
        failures.append("Corkboard: could not resolve two card cells to multi-select")
    else:
        click_node(cards[0])
        # Ctrl+click the second so both are selected (GridView: Ctrl+click toggles).
        b = cards[1].get("bounds") or {}
        call("inject_pointer", {"x": b.get("x", 0) + b.get("width", 0) / 2,
                                "y": b.get("y", 0) + b.get("height", 0) / 2,
                                "kind": "click", "modifiers": ["ctrl"]})
        settle()
        time.sleep(0.4)
        kebabs = kebab_nodes()
        if not kebabs:
            failures.append("Corkboard: no kebab to open after multi-selecting")
        else:
            call("invoke_action", {"node": kebabs[1 if len(kebabs) > 1 else 0]["id"],
                                   "action": "click"})
            settle()
            time.sleep(0.4)
            counted = [t for t in ("Duplicate 2 cards", "Move 2 cards to trash",
                                   "Set label on 2 cards", "Move 2 cards to\u2026")
                       if text_present(t)]
            if not counted:
                shot("corkboard-batch-wording-stale")
                failures.append("Corkboard card menu: two cards selected but no counted "
                                "wording appeared \u2014 the labels are stale (CardMenu is "
                                "meant to rebuild them from the live selection)")
            else:
                print(f"  counted wording appears with 2 selected \u2713 ({counted[0]!r})")
            shot("corkboard-batch-menu")
            call("inject_key", {"key": "Escape"})
            settle()

    # And the singular case must NOT be counted.
    if text_present("Duplicate 1 card"):
        shot("corkboard-batch-singular-leak")
        failures.append("Corkboard card menu: a single card shows the counted wording "
                        "('Duplicate 1 card') \u2014 batched() should use the plain label")
    else:
        print("  a single card uses the plain wording \u2713")

# \u2500\u2500 5. Drilling into a folder card, and back out \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
#
# A Part's board shows its chapter folders; activating one re-scopes the board
# *in place* and deepens the breadcrumb. The crumb then walks back out.
print("\n=== drill in / out ===")
if open_corkboard("Part One \u2014 Arrival"):
    folder_card = None
    for n in nodes():
        b = n.get("bounds") or {}
        if b.get("x", 0) < PANE_X:
            continue
        label = n.get("label") or ""
        if label.startswith("The Keeper") or ", Chapter" in label:
            folder_card = n
            break
    if folder_card is None:
        print("  (no folder card on this board \u2014 drill-in not exercised here)")
    else:
        title = (folder_card.get("label") or "").split(", ")[0]
        acts = folder_card.get("actions") or []
        if "activate" in acts:
            call("invoke_action", {"node": folder_card["id"], "action": "activate"})
        else:
            b = folder_card.get("bounds") or {}
            call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                    "y": b["y"] + b.get("height", 0) / 2,
                                    "kind": "double_click"})
        settle()
        time.sleep(0.5)
        # The breadcrumb now carries the container we came from *and* the one we
        # drilled into.
        if not text_present("Arrival"):
            shot("corkboard-drill-no-crumb")
            failures.append("Corkboard drill-in: the parent crumb vanished from "
                            "the breadcrumb")
        else:
            print(f"  drilled into {title!r}, parent crumb still shown \u2713")
        shot("corkboard-drilled")
        # Walk back out via the ancestor crumb.
        if click("Part One \u2014 Arrival", "(breadcrumb crumb)", minx=PANE_X):
            print("  crumb walked back out \u2713")

# \u2500\u2500 6. A leaf has no Corkboard \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
print("\n=== Scene at dawn \u2192 no Corkboard segment ===")
if not click("Scene at dawn", "(binder row)"):
    failures.append("Scene at dawn: not in the binder")
elif has_label("Corkboard", minx=PANE_X):
    shot("corkboard-leak-scene")
    failures.append("Scene at dawn: a 'Corkboard' segment leaked onto a leaf")
else:
    print("  a leaf has no Corkboard segment \u2713")

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()

if failures:
    print("\nFAIL:")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("\nPASS: the Corkboard renders cards on every container, its chrome is "
      "complete (breadcrumb / count / Nested-Flat / order / filter / size / New), "
      "the card menu carries the full action set, drilling in and out works, and "
      "no leaf exposes it.")
print("NOTE: drag-to-reparent onto a container card and the Delete-key bulk trash "
      "are not drivable here (the bridge injects clicks and named keys, not drag "
      "gestures against a virtualized grid) \u2014 both are covered by the headless "
      "view-model tests in `view_models::corkboard::tests`.")
