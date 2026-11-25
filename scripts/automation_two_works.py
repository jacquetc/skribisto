#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Two DIFFERENT Works in two windows of one process: closing one leaves the other.

`automation_new_window.py` covers two windows on the SAME Work. This probe covers
the other multi-window shape: a second project opened while the first is still
up. The second launch hands its path to the running primary, which opens it in a
window of its own, so one process holds two Works.

Each window knows its Work only through its own `AppIds`, never through an
app-wide one (see `crate::app_ids`'s module doc). An app-wide `AppIds` would be
the FIRST window's, so a close driven from the SECOND window would name the
wrong Work. That is what this asserts against, in order:

  1. **Two windows, two Works.** Starforgers is launched first and becomes the
     primary; the Verne example is launched second and handed to it.
     `list_windows` goes 1 → 2, and each window shows its own project.
  2. **Each window edits its own Work.** A Book is created in each window; each
     window's save indicator turns "unsaved".
  3. **Close Work in the SECOND window closes the second Work only.** The guard
     asks once; Discard. Window 1 still shows Starforgers, still holds the Book
     created there, and its indicator still says "unsaved": its edits were not
     discarded with the other project's.

Runs against throwaway copies of the bundled examples in an isolated XDG
sandbox, so autosave and format migration cannot touch the repo. Linux only:
the handoff runs over the open registry's socket, which this probe does not set
up for the other platforms.
"""

import json, os, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

fixture.require_linux("the second launch's handoff to the primary is only set up here on Linux")

MCP = fixture.mcp_binary()
STARFORGERS = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")
VERNE = fixture.repo_path(
    "resources/examples/le_tour_du_monde_en_80_jours/"
    "le-tour-du-monde-en-quatre-vingts-jours.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
sandbox = tempfile.mkdtemp(prefix="skribisto_two_works_")
SANDBOX_ENV = fixture.sandbox_env(os.path.join(sandbox, "config"), base={}, extra={
    "XDG_DATA_HOME": os.path.join(sandbox, "data"),
    "HOME": sandbox,
})
# No `--config` pins here: `--config` implies `--new-instance`, which makes a
# launch Standalone, and this probe needs the first launch to win the election
# and the second to be handed to it. The settings file is the only way to pin
# the language, so it is written directly.
fixture.write_settings(SANDBOX_ENV["XDG_CONFIG_HOME"], show_welcome=False)
WORK_A = os.path.join(sandbox, "Starforgers.skrib")
WORK_B = os.path.join(sandbox, "Verne.skrib")
shutil.copyfile(STARFORGERS, WORK_A)
shutil.copyfile(VERNE, WORK_B)
ENV = {**os.environ, **SANDBOX_ENV}

app = mcp = None
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
_id = 0


def fail(msg):
    print("FAIL:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log, errors="replace").read().splitlines()[-25:]))
    try:
        et = open(mcp_err).read()
        if et.strip():
            print("--- mcp stderr ---")
            print("\n".join(et.splitlines()[-15:]))
    except OSError:
        pass
    for p in (mcp, app):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


def send(method, params=None, notif=False):
    global _id
    msg = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        msg["params"] = params
    if not notif:
        _id += 1
        msg["id"] = _id
    mcp.stdin.write(json.dumps(msg) + "\n")
    mcp.stdin.flush()


def recv(timeout=25, fatal=True):
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
            return json.loads(line)
    if fatal:
        fail("no MCP response within timeout")
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = recv().get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(c.get("text", "") for c in result.get("content", [])
                       if c.get("type") == "text").strip()
        payload = json.loads(text) if text[:1] in ("{", "[") else {}
    return result, payload


def nodes(window_id):
    _, p = call("snapshot_tree", {"window_id": window_id})
    return p.get("nodes", []) if isinstance(p, dict) else []


def list_windows():
    _, p = call("list_windows")
    return p if isinstance(p, list) else []


def wait_window_count(want, timeout=30):
    end = time.time() + timeout
    seen = None
    while time.time() < end:
        seen = list_windows()
        if len(seen) == want:
            return seen
        time.sleep(0.3)
    return seen


def title(w):
    for key in ("title", "caption", "name"):
        if isinstance(w, dict) and w.get(key):
            return w[key]
    return ""


def key(w):
    for k in ("id", "window_id", "string_id"):
        if isinstance(w, dict) and w.get(k) is not None:
            return w[k]
    return None


def labels(window_id):
    return [n.get("label") for n in nodes(window_id) if n.get("label")]


def find_exact(label, window_id, role=None):
    for n in nodes(window_id):
        if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
            return n
    return None


def click(node, window_id):
    b = node.get("bounds") or {}
    call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                            "y": b["y"] + b.get("height", 0) / 2,
                            "action": "click", "window_id": window_id})


def activate(node, window_id):
    call("invoke_action", {"node": node["id"], "action": "Click", "window_id": window_id})


CHROME_NOISE = ("saved", "unsaved")


def save_indicator(window_id):
    for label in labels(window_id):
        if any(w in label.lower() for w in CHROME_NOISE):
            return label.lower()
    return ""


def binder_rows(window_id):
    return sorted(l for l in labels(window_id)
                  if not any(w in l.lower() for w in CHROME_NOISE))


def create_book(window_id):
    """Click the binder's "Book" create button; return the rows it added."""
    before = binder_rows(window_id)
    btn = next((n for n in nodes(window_id)
                if n.get("role") == "Button" and (n.get("label") or "").strip() == "Book"), None)
    if not btn:
        fail(f"no binder 'Book' create button in window {window_id}")
    click(btn, window_id)
    time.sleep(3.0)
    gained = [l for l in binder_rows(window_id) if l not in before]
    if not gained:
        fail(f"creating a Book in window {window_id} added no binder row")
    return gained


def close_work(window_id):
    """Title-bar hamburger → Work → Close Work, all in `window_id`."""
    ham = find_exact("Menu", window_id, role="Button")
    if not ham:
        fail(f"no title-bar 'Menu' button in window {window_id}")
    activate(ham, window_id)
    work = None
    deadline = time.time() + 8
    while time.time() < deadline and not work:
        work = find_exact("Work", window_id, role="MenuItem")
        if not work:
            time.sleep(0.3)
    if not work:
        fail(f"the 'Work' menu did not open in window {window_id}")
    activate(work, window_id)
    time.sleep(1.2)
    item = next((n for n in nodes(window_id) if n.get("role") == "MenuItem"
                 and "close work" in (n.get("label") or "").lower()), None)
    if not item:
        fail(f"no 'Close Work' item in window {window_id}'s Work menu")
    activate(item, window_id)


def discard_buttons():
    found = []
    for w in list_windows():
        for n in nodes(key(w)):
            if (n.get("role") == "Button"
                    and (n.get("label") or "").strip().lower() == "discard"):
                found.append((key(w), n))
    return found


# ── 1. Two windows, two Works ────────────────────────────────────────────────
app = subprocess.Popen(fixture.launch_argv(WORK_A, new_instance=False),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT, env=ENV)
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=60)
except RuntimeError as e:
    fail(str(e))

deadline = time.time() + 20
init = None
while time.time() < deadline and init is None:
    mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                           stdout=subprocess.PIPE, stderr=open(mcp_err, "w"),
                           text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "two-works", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    fail("could not connect MCP")
send("notifications/initialized", notif=True)

first = wait_window_count(1, timeout=40)
if len(first) != 1:
    fail(f"expected one window after the first launch, saw {first}")
key_a = key(first[0])
end = time.time() + 40
while time.time() < end and not any("prologue" in l.lower() for l in labels(key_a)):
    time.sleep(0.5)
if not any("prologue" in l.lower() for l in labels(key_a)):
    fail("Starforgers never finished loading in window 1")
print(f"window 1: {title(first[0])!r}")

# The second launch carries no `--new-instance`, so it hands its path to the
# primary above and exits.
second_launch = subprocess.run(fixture.launch_argv(WORK_B, new_instance=False),
                               env=ENV, capture_output=True, text=True, timeout=60)
if second_launch.returncode != 0:
    fail(f"the second launch exited {second_launch.returncode}: "
         f"{(second_launch.stdout + second_launch.stderr)[-800:]}")

both = wait_window_count(2, timeout=60)
if len(both) != 2:
    fail(f"the handed-off project did not open a second window (saw {both})")
key_b = next(key(w) for w in both if key(w) != key_a)


def titles():
    rows = {key(w): title(w) for w in list_windows()}
    return rows.get(key_a, ""), rows.get(key_b, "")


# A window's title names its Work once the Work has loaded; until then it is
# the bare app name.
end = time.time() + 40
title_a, title_b = titles()
while time.time() < end and not ("starforgers" in title_a.lower()
                                 and "tour du monde" in title_b.lower()):
    time.sleep(0.5)
    title_a, title_b = titles()
if "starforgers" not in title_a.lower():
    fail(f"window 1 does not name Starforgers: {title_a!r}")
if "tour du monde" not in title_b.lower():
    fail(f"window 2 does not name the Verne Work: {title_b!r}")
# Loaded once its binder's create bar is up.
end = time.time() + 40
while time.time() < end and not find_exact("Book", key_b, role="Button"):
    time.sleep(0.5)
time.sleep(2.0)
if any("prologue" in l.lower() for l in labels(key_b)):
    fail("window 2's binder shows Starforgers' Prologue: it is not its own Work")
print(f"window 2: {title_b!r}, a different Work in the same process")

# ── 2. Each window edits its own Work ────────────────────────────────────────
gained_a = create_book(key_a)
gained_b = create_book(key_b)
if "unsaved" not in save_indicator(key_a) or "unsaved" not in save_indicator(key_b):
    fail(f"both indicators should read unsaved: {save_indicator(key_a)!r} / "
         f"{save_indicator(key_b)!r}")
print(f"each window created a Book in its own Work ({gained_a} / {gained_b}); both unsaved")

# ── 3. Close Work in window 2 closes the second Work only ────────────────────
# Both new Books carry the same default labels, so finding them in window 1
# afterwards proves little; its whole binder must be unchanged instead.
rows_a_before = binder_rows(key_a)
close_work(key_b)
found = []
deadline = time.time() + 10
while time.time() < deadline and not found:
    found = discard_buttons()
    if not found:
        time.sleep(0.4)
if len(found) != 1:
    fail(f"expected exactly one unsaved-changes guard, saw {len(found)}")
activate(found[0][1], found[0][0])
time.sleep(3.0)

if app.poll() is not None:
    fail("the app exited after closing the second Work")
after = list_windows()
still_a = next((w for w in after if key(w) == key_a), None)
if not still_a:
    fail(f"window 1 is gone after Close Work in window 2: {after}")
if "starforgers" not in title(still_a).lower():
    fail(f"window 1 no longer shows Starforgers: {title(still_a)!r}")
if any("tour du monde" in title(w).lower() for w in after):
    fail(f"the second Work is still shown somewhere: {[title(w) for w in after]}")
rows_a = binder_rows(key_a)
if not any("prologue" in l.lower() for l in rows_a):
    fail("window 1's binder is empty: its Work was torn down with the other one")
if rows_a != rows_a_before:
    lost = [l for l in rows_a_before if l not in rows_a]
    fail(f"window 1's binder changed when window 2's Work closed (lost {lost}): "
         "its edits were discarded")
if "unsaved" not in save_indicator(key_a):
    fail(f"window 1's indicator now reads {save_indicator(key_a)!r}: the Discard "
         "answered for the wrong Work")
print(f"window 2's Work closed ({[title(w) for w in after]}); window 1 kept "
      f"Starforgers, its Book and its unsaved state")

print("\nPASS: two Works in two windows of one process; Close Work in the second "
      "window closed the second Work and left the first one untouched.")
for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
shutil.rmtree(sandbox, ignore_errors=True)
