#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The Link command, end to end against the live app.

    scripts/automation_links.py resources/examples/Starforgers.skrib

A link is the one piece of formatting that cannot be a toggle — it needs a
destination — so it is the only format command that opens a dialog, and the only
one whose result cannot be read off a lit button. Everything here is therefore
asserted against the editor's own prose, via AccessKit, not against the dock.

Unlike `automation_images.py`, this flow is fully drivable: image insertion goes
through a **native** file dialog, which is outside the AT tree entirely, whereas
the link dialog is an ordinary in-tree modal with two text fields.

Asserts, with a scene open and the caret in its prose:

  1. the Format dock is up and carries a **Link** button;
  2. pressing it opens the dialog, with both fields present;
  3. filling them and confirming puts the link in the prose;
  4. the caret back inside that link re-opens the dialog in *edit* form — i.e.
     the extent lookup found the link — and it offers **Remove link**;
  5. removing it leaves the words behind as plain prose.

Step 4 is the one worth having. A link is a stretch of runs rather than an
object, so "find the link under the caret" is real work — and a version that
finds only part of it looks identical here until step 5 leaves half a link.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import isolated_config, working_copy

SKRIBISTO = os.environ.get(
    "SKRIBISTO_BIN", "/home/cyril/Devel/skribisto/target/debug/skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
OUT = "/tmp/links-probe"
os.makedirs(OUT, exist_ok=True)

URL = "https://example.com/logs"
NAME = "the lighthouse log"

# Both halves of the isolation matter, for the same reasons the heading-popover
# probe spells out: a private config keeps a previous run's desk from deciding
# whether the Format dock is open, and a private copy of the project keeps the
# link this probe writes out of the operator's example file.
# `--dark` re-runs the whole probe against the dark theme. A link's colour comes
# from the theme's own `TextRole::Link`, so "is a link legible" is a question
# that has to be asked once per theme, and the screenshots are the answer.
args = [a for a in sys.argv[1:] if a != "--dark"]
dark = "--dark" in sys.argv
env = isolated_config(locale="en-US", label="links", dark=dark, show_welcome=False)
project = working_copy(os.path.abspath(args[0]), "links") if args else None

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
argv = [SKRIBISTO] + ([project] if project else [])
app = subprocess.Popen(argv, env=env, stdout=open(log, "w"), stderr=subprocess.STDOUT)
mcp = None


def die(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


sock = tok = None
end = time.time() + 30
while time.time() < end:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        die("the app exited before printing its bridge socket")
    time.sleep(0.2)
if not sock:
    die("no bridge socket within 30s")

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
    stop = time.time() + timeout
    while time.time() < stop:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, stop - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        if line.strip():
            return json.loads(line.strip())
    if fatal:
        die("no MCP response within timeout")
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


init = None
end = time.time() + 30
while time.time() < end and init is None:
    while not os.path.exists(sock) and time.time() < end:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "links", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect the automation MCP")
send("notifications/initialized", notif=True)
print("connected")


def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def labels():
    return {n.get("label") for n in nodes() if n.get("label")}


def find(label, timeout=6.0):
    stop = time.time() + timeout
    while time.time() < stop:
        for n in nodes():
            if n.get("label") == label:
                return n
        settle()
    return None


def click_node(n):
    b = n.get("bounds") or {}
    if "x" not in b:
        return False
    call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                            "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    settle()
    time.sleep(0.4)
    return True


def click(label, what=""):
    n = find(label)
    if not n:
        print(f"  !! no node labelled {label!r} {what}")
        return False
    return click_node(n)


def shot(name):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            path = os.path.join(OUT, f"{name}.png")
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot → {path}")
            return path
    return None


def editors(ns=None):
    ns = ns if ns is not None else nodes()
    return [n for n in ns
            if n.get("role") == "MultilineTextInput"
            and "set_value" in (n.get("actions") or [])]


def editor_text(node_id, ns=None):
    """A `RichTextEditor`'s prose lives in per-block CHILD nodes, not in
    `value` — which stays empty. `children` are node *ids*, so they have to be
    resolved against the flat snapshot. Same walk as
    `automation_dock_editor_probe.py`."""
    ns = ns if ns is not None else nodes()
    by_id = {n["id"]: n for n in ns if "id" in n}
    node = by_id.get(node_id)
    if node is None:
        return ""
    parts = []

    def walk(n):
        for k in ("label", "value", "name"):
            if n.get(k):
                parts.append(str(n[k]))
                break
        for cid in (n.get("children") or []):
            child = by_id.get(cid)
            if child:
                walk(child)

    walk(node)
    return " ".join(parts)


# ── reach a scene with the caret in its prose ────────────────────────────────
if project is None:
    if not click("Mock Project", "(needs a --features mocks build)"):
        die("no 'Mock Project' recent row and no project argument")
# `load_work` is SLOW in a debug build (~20 s for the bundled example) — without
# this wait every check below runs against the empty shell.
end = time.time() + 120
while time.time() < end:
    if "Binder" in labels():
        break
    settle()
else:
    die("the project never finished loading")
settle()
print("project loaded")

for row in ("Prologue", "Chapter 1", "Chapter 2"):
    if click(row, "(a binder row)"):
        break
else:
    die("no scene row to open in the binder")


def reveal_format_dock():
    """Click the Format dock's activity-rail entry.

    Picked by shape, not by label alone: the menu bar carries a "Format" menu
    with the same accessible name, and clicking *that* opens a menu instead —
    which fails later as "the caret never landed", the one failure this probe
    must not confuse with the thing under test.
    """
    rail = [n for n in nodes()
            if n.get("label") == "Format"
            and 0 < (n.get("bounds") or {}).get("width", 0) < 60
            and 0 < (n.get("bounds") or {}).get("height", 0) < 60]
    return bool(rail) and click_node(rail[0])


def editor_area():
    best = None
    for n in nodes():
        if n.get("role") != "TabPanel":
            continue
        b = n.get("bounds") or {}
        if "x" not in b or b.get("height", 0) < 200:
            continue
        if best is None or b["width"] > best["width"]:
            best = b
    return best


def place_caret():
    """Click down the editor column until the dock says the caret is in prose.

    The dock is the oracle — Bold up means a formattable surface has focus.
    Several depths, because the top of the pane is the title field and a
    container's own page puts an epigraph box above its prose.
    """
    area = editor_area()
    if area is None:
        return None
    for frac in (0.35, 0.5, 0.22, 0.65):
        x = area["x"] + area["width"] * 0.45
        y = area["y"] + area["height"] * frac
        call("inject_pointer", {"x": x, "y": y, "action": "click"})
        settle()
        time.sleep(0.4)
        if "Bold" in labels():
            return (x, y)
    return None


if "Bold" not in labels():
    reveal_format_dock()
caret = place_caret()
if caret is None:
    shot("no-caret")
    die("the caret never landed in prose — the Format dock never came up")
print("caret in prose")

# ── STEP 1: the dock carries a Link button ───────────────────────────────────
if not find("Link…"):
    shot("no-link-button")
    die("STEP 1: the Format dock has no Link button")
print("STEP 1 OK: the dock carries a Link button")

# The editor the prose will be asserted against — the tallest writing editor.
eds = editors()
if not eds:
    die("no writing editor in the a11y tree")
ed = max(eds, key=lambda n: (n.get("bounds") or {}).get("height", 0))

# Type a word to link, so the probe never depends on the example's own prose.
call("type_text", {"node": ed["id"], "text": " MARKERWORD "})
settle()
time.sleep(0.6)

# ── STEP 2: the button opens the dialog ──────────────────────────────────────
if not click("Link…", "(the dock button)"):
    die("STEP 2: could not press the Link button")
shot("dialog-open")
present = labels()
# The dialog is identified by its buttons, not by its field labels: a
# `FormLayout` label is painted text and reaches AccessKit as part of the
# field's own node rather than as a label node of its own. Asserting on the
# buttons keeps the check about "is the dialog up" instead of about how the
# form labels itself.
missing = [l for l in ("Insert", "Cancel") if l not in present]
if missing:
    die(f"STEP 2: the link dialog is missing {missing} — present: "
        f"{sorted(l for l in present if l)[:40]}")

fields = [n for n in nodes()
          if n.get("role") in ("TextInput", "SingleLineTextInput")
          and "set_value" in (n.get("actions") or [])]
if len(fields) < 2:
    die(f"STEP 2: expected two text fields in the dialog, saw {len(fields)}")
print("STEP 2 OK: the dialog is up, with both fields")

# ── STEP 3: filling it puts a link in the prose ──────────────────────────────
fields.sort(key=lambda n: (n.get("bounds") or {}).get("y", 0))
# Click each field before typing into it. `type_text` delivers keys to the
# focused widget, so typing into the second field without focusing it first
# lands the URL in the first one — which shows up here as an empty destination
# and a correctly-disabled Insert, i.e. the app behaving and the probe not.
for field, text in ((fields[0], NAME), (fields[1], URL)):
    click_node(field)
    call("type_text", {"node": field["id"], "text": text})
    settle()
settle()
shot("dialog-filled")
if not click("Insert", "(the dialog's confirm)"):
    die("STEP 3: could not press Insert")
settle()
time.sleep(0.8)
shot("link-inserted")

prose = editor_text(ed["id"])
if NAME not in prose:
    die(f"STEP 3: the link's text is not in the prose — got {prose[:200]!r}")
print("STEP 3 OK: the link's words are in the prose")

# ── STEP 4: the caret inside it re-opens the dialog in edit form ─────────────
# Reached through Ctrl+K rather than the dock button, so the probe also covers
# the shortcut door — all three doors share one action, and this is the cheapest
# place to notice if that ever stops being true.
call("inject_key", {"key": "K", "ctrl": True})
settle()
time.sleep(0.5)
shot("dialog-edit")
present = labels()
if "Remove link" not in present:
    die("STEP 4: the dialog did not open in edit form — the extent lookup did "
        f"not find the link under the caret. present: "
        f"{sorted(l for l in present if l)[:40]}")
print("STEP 4 OK: the caret is in the link, and removal is offered")

# ── STEP 5: removing it leaves the words ─────────────────────────────────────
if not click("Remove link", "(the dialog's third button)"):
    die("STEP 5: could not press Remove link")
settle()
time.sleep(0.8)
shot("link-removed")

prose = editor_text(ed["id"])
if NAME not in prose:
    die(f"STEP 5: removing the link took its words too — got {prose[:200]!r}")
print("STEP 5 OK: the link is gone and its words remain")

print("\nPASS — the Link command inserts, re-opens for edit, and removes.")
for p in (app, mcp):
    if p and p.poll() is None:
        p.terminate()
