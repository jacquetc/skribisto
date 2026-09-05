#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Live-app probe for the **Always forward** writing game.

The one thing no headless test can prove: that a writer sitting in front of the
real app, with the real keyboard, cannot take prose back once the game is on —
and that everything the game deliberately leaves open still works.

Asserts, against the running app:

  1. the **Writing games** dock mounts on the leading rail and its toggle starts
     off;
  2. typing into a scene works normally before the game starts;
  3. with the game on, **Backspace and Delete land on the document and change
     nothing** — the invariant, driven through real key events rather than
     through the view-model;
  4. typing still works while the game is on (the draft grows), and the
     status-bar warning has appeared;
  5. switching the game off gives Backspace back — on the *same* editor, without
     reopening the tab, which is the "swap the filter on a mounted editor" path.

Run: python3 scripts/automation_writing_games.py [project.skrib]
"""
import base64, json, os, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import working_copy  # noqa: E402

import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
SRC = os.path.abspath(
    sys.argv[1]
    if len(sys.argv) > 1
    else fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")
)
# Never open the checked-in fixture: this probe types into it and autosave is real.
PROJECT = working_copy(SRC)

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = mcp = None

# Every label this probe matches ("Writing games", "Always forward", "deleting
# is disabled", …) is English, so the language has to be SET rather than
# inherited from the operator's OS (`startup.rs`'s `auto_detect_os_locale`); a
# private `XDG_CONFIG_HOME` also keeps this run off the operator's real
# settings/recents. A project is always passed, so the Welcome window never
# shows either way.
ENV = fixture.isolated_config(locale="en-US", label="writing-games", show_welcome=False)
PINS = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="writing-games")


def fail(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    try:
        print("\n".join(open(log).read().splitlines()[-25:]))
    except OSError:
        pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=ENV)
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=60)
except RuntimeError as e:
    fail(str(e))

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
        fail("no MCP response within timeout")
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = recv().get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(
            c.get("text", "") for c in result.get("content", []) if c.get("type") == "text"
        )
        payload = json.loads(text) if text.strip().startswith("{") else {}
    return result, payload


# `wait_for_bridge` only returns once the bridge has bound its endpoint and
# spawned its accept thread, so there is nothing left to retry here.
mcp = subprocess.Popen(
    fixture.mcp_argv(bridge, MCP),
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=open(mcp_err, "w"),
    text=True,
    bufsize=1,
)
send(
    "initialize",
    {
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {"name": "writing-games-probe", "version": "1"},
    },
)
if recv(timeout=20, fatal=False) is None:
    fail("could not connect MCP (no initialize response)")
send("notifications/initialized", notif=True)
print("connected")


def nodes():
    _, payload = call("snapshot_tree")
    return payload.get("nodes", [])


def settle(times=3):
    for _ in range(times):
        call("settle")


def shot(path):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot → {path}")
            return


def text_of(n):
    return " ".join(str(n.get(k, "")) for k in ("label", "value", "name") if n.get(k))


def find(ns, needle, role=None):
    out = []
    for n in ns:
        if role and n.get("role") != role:
            continue
        if needle.lower() in text_of(n).lower():
            out.append(n)
    return out


def editors(ns):
    return [
        n
        for n in ns
        if n.get("role") == "MultilineTextInput" and "set_value" in (n.get("actions") or [])
    ]


def editor_text(node_id, ns=None):
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
        for cid in n.get("children") or []:
            child = by_id.get(cid)
            if child:
                walk(child)

    walk(node)
    return " ".join(parts)


# ── Wait for the project ────────────────────────────────────────────────────
end = time.time() + 40
ns = []
while time.time() < end:
    ns = nodes()
    if find(ns, "Manuscript") or find(ns, "Chapter"):
        break
    time.sleep(0.5)
else:
    fail("project did not load within 40s")
print("project loaded")

# ── 1. Open a scene and type into its manuscript prose ──────────────────────
#
# Before the dock: opening the Writing games panel takes over the leading rail,
# and the binder goes with it — so the scene has to be opened while the binder is
# still the visible activity. That is also the order a writer works in.
def prose_editor():
    """The manuscript editor, re-resolved every time.

    Never cache the id across an interaction: a tab rebuild re-mints a11y node
    ids, so a stored id silently resolves to nothing and every later read comes
    back empty — which reads as "the text vanished" rather than "the probe is
    holding a stale handle".
    """
    ns = nodes()
    best, best_len = None, -1
    for e in editors(ns):
        n = len(editor_text(e["id"], ns))
        if n > best_len:
            best, best_len = e["id"], n
    return best, best_len


def prose_text():
    ed, _ = prose_editor()
    return editor_text(ed) if ed is not None else ""


def click_into(node_id):
    """Put a real caret in the manuscript editor.

    Two things have to be true and neither is free. `focus_node` gives the widget
    keyboard focus but leaves **no caret**, so typed text has nowhere to land —
    only a pointer click does both. And a chapter's manuscript sits *below* its
    title and synopsis, so on a fresh tab its box starts past the bottom of the
    window: clicking its visible sliver lands on the padding around it, not in
    the text. So scroll it up into the page first, re-read its bounds (scrolling
    moves them), then click well inside.
    """
    for _ in range(6):
        n = next((x for x in nodes() if x.get("id") == node_id), None)
        b = (n or {}).get("bounds") or {}
        if b.get("y", 0.0) <= 200.0:
            break
        call("scroll", {"node": node_id, "dx": 0, "dy": -400})
        settle(2)
    n = next((x for x in nodes() if x.get("id") == node_id), None)
    b = (n or {}).get("bounds") or {}
    x = b.get("x", 0.0) + min(b.get("width", 200.0) / 2, 200.0)
    y = max(b.get("y", 0.0) + 24.0, 24.0)
    call("inject_pointer", {"x": x, "y": y, "button": "left"})
    settle(3)


# A scene leaf, not a chapter heading: a chapter tab stacks title + synopsis
# above its prose, so the manuscript editor starts below the fold and a click
# lands on the padding around it rather than in the text.
opened = False
for n in nodes():
    if n.get("role") == "TreeItem" and text_of(n).strip().startswith("Prologue"):
        bb = n.get("bounds")
        if bb:
            call(
                "inject_pointer",
                {"x": bb["x"] + bb["width"] / 2, "y": bb["y"] + bb["height"] / 2,
                 "button": "left"},
            )
            settle(5)
            opened = True
        break
if not opened:
    fail("could not open a scene from the binder")

ed, ed_len = prose_editor()
if ed is None or ed_len <= 0:
    fail("no editable prose editor with text after opening the scene")
click_into(ed)
call("type_text", {"node": ed, "text": "ABC"})
settle(4)
before = prose_text()
if "ABC" not in before:
    fail(f"typing did not land before the game started (text={before[-160:]!r})")
print("1. OK — typing works before the game")

# ── 2. The dock is on the rail, and carries the toggle ──────────────────────
rail = find(nodes(), "Writing games")
if not rail:
    fail("no 'Writing games' surface found — the dock did not mount")

# A real pointer click at the tab's own bounds, not `invoke_action` — an
# activity-rail tab's a11y click does not switch the side's visible panel, so an
# action-driven probe passes while the writer sees nothing happen.
tab = next((n for n in rail if n.get("role") == "Tab" and n.get("bounds")), None)
if tab is None:
    fail("the Writing games rail tab carries no bounds to click")
b = tab["bounds"]
call(
    "inject_pointer",
    {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2, "button": "left"},
)
settle(6)
time.sleep(0.5)
shot("/tmp/games-dock.png")

toggles = [n for n in nodes() if n.get("role") in ("CheckBox", "Switch", "ToggleButton")]
forward = [n for n in toggles if "forward" in text_of(n).lower()]
if not forward:
    fail("the dock has no 'Always forward' toggle")
print(f"2. OK — the dock mounts and offers {text_of(forward[0])!r}")

# ── 3. Start the game, then try to delete ───────────────────────────────────
#
# Click the switch with a pointer and then **prove the game is on** before
# testing anything. `invoke_action` does not reliably flip a Toggle (the same way
# it does not switch a rail tab), and a probe that assumes the click worked would
# report "Backspace took nothing away" for a game that was never running — a pass
# that means nothing. The dock's own status line is the witness: it reads
# "Playing" only from the signal every frozen editor reads.
def playing():
    # Match the phrase only the *playing* line carries. Matching "playing" would
    # also match "Not playing", which is how this check first reported the game
    # as already running before it had been started.
    return bool(find(nodes(), "deleting is disabled"))


def click_toggle():
    node = next(
        (
            n
            for n in nodes()
            if n.get("role") in ("CheckBox", "Switch", "ToggleButton")
            and "forward" in text_of(n).lower()
            and n.get("bounds")
        ),
        None,
    )
    if node is None:
        fail("the 'Always forward' toggle vanished")
    bb = node["bounds"]
    call(
        "inject_pointer",
        {"x": bb["x"] + bb["width"] / 2, "y": bb["y"] + bb["height"] / 2, "button": "left"},
    )
    settle(5)


if playing():
    fail("the game was already on before the probe started it")
click_toggle()
if not playing():
    fail("clicking the toggle did not start the game — the dock still reads 'Not playing'")
print("3. OK — the game is on, and the dock says so")
shot("/tmp/games-playing.png")

ed, _ = prose_editor()
click_into(ed)
guarded_before = prose_text()
if not guarded_before:
    fail("lost sight of the manuscript editor after switching the game on")
for _ in range(5):
    call("inject_key", {"node": ed, "key": "Backspace"})
for _ in range(5):
    call("inject_key", {"node": ed, "key": "Delete"})
settle(5)
guarded_after = prose_text()
if guarded_after != guarded_before:
    fail(
        "THE INVARIANT BROKE: prose changed under Backspace/Delete while the game was on\n"
        f"  before={guarded_before[-80:]!r}\n  after ={guarded_after[-80:]!r}"
    )
print("4. OK — Backspace and Delete take nothing away while playing")

# The draft may still grow.
ed, _ = prose_editor()
call("type_text", {"node": ed, "text": "XYZ"})
settle(4)
grown = prose_text()
if "XYZ" not in grown:
    fail(f"typing stopped working while the game was on (text={grown[-160:]!r})")
print("5. OK — the draft still grows while playing")

# ── 4. Stop the game: deleting comes back, same editor ──────────────────────
click_toggle()
if playing():
    fail("clicking the toggle again did not end the game")
print("6. OK — the game is off again, and the dock says so")

ed, _ = prose_editor()
click_into(ed)
released_before = prose_text()
call("inject_key", {"node": ed, "key": "Backspace"})
settle(4)
released_after = prose_text()
if released_after == released_before:
    fail(
        "Backspace did not come back after the game ended — the filter swap is one-way\n"
        f"  text={released_after[-80:]!r}"
    )
print("7. OK — deleting works again the moment the game ends")
shot("/tmp/games-after.png")

print("\nPASS — the invariant holds on the live app, and nothing else was taken away.")
for p in (app, mcp):
    if p and p.poll() is None:
        p.terminate()
