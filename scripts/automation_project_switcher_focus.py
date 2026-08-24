#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Clicking the ProjectSwitcherButton must move keyboard focus INTO the popover.

It didn't. `PopoverButton` does focus its content's first focusable descendant
when it opens — but the switcher's popover re-scans the open-project registry on
open by bumping `open_epoch`, a `BindingLevel::Rebuild` binding. That rebuild
runs in the *next* layout pass, destroys the row the popover had just focused,
and `revalidate_interaction_state` then dropped focus to `None`. The menu came
up with nothing focused: no arrow keys, no Enter.

Fixed in teksilo (`widget_tree`): a rebuild that destroys the focused widget now
re-enters the subtree that owned focus, after layout, instead of dumping focus.

This drives the real app to prove it: open the switcher, and assert an element
INSIDE the popover overlay carries the AT `focused` flag — then press Down and
confirm focus is still in the popover (i.e. the menu is really keyboard-live).
"""
import base64, json, os, re, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/Starforgers.skrib")

sandbox = tempfile.mkdtemp(prefix="skribisto_switcher_focus_")
env = {**os.environ, "XDG_CONFIG_HOME": os.path.join(sandbox, "config"),
       "XDG_DATA_HOME": os.path.join(sandbox, "data"), "HOME": sandbox}
work = os.path.join(sandbox, "Starforgers.skrib")
shutil.copyfile(EXAMPLE, work)


def die(msg, log=None):
    print("FAIL:", msg)
    if log:
        print("\n".join(open(log).read().splitlines()[-25:]))
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, work], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
sock = tok = None
end = time.time() + 25
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


def call(name, args=None):
    r = call_raw(name, args)
    p = r.get("structuredContent")
    if p is None:
        txt = "".join(c.get("text", "") for c in r.get("content", [])
                      if c.get("type") == "text").strip()
        p = json.loads(txt) if txt.startswith(("{", "[")) else {}
    return p


def shot(path):
    """Grab the app's own framebuffer — no window-raise dance, and no risk of
    the raise dismissing the very popover we are trying to photograph."""
    for c in call_raw("screenshot").get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"screenshot: {path}")
            return


mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                            "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                                       "clientInfo": {"name": "switcher-focus", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


def focused_node():
    return next((n for n in nodes() if n.get("focused") is True), None)


def overlay_count():
    return call("get_overlays").get("count", 0)


# The popover's content is a MenuList of project rows; anything focused in here
# is "inside the popup" as far as the user is concerned.
POPUP_ROLES = {"Menu", "MenuItem", "MenuListPopup", "ListBox", "ListBoxOption"}


# The switcher's trigger is the title-bar Button labelled with the open project
# (or "No work loaded"). It is a disclosure button, so it carries `expanded`.
SWITCHER_LABELS = ("Starforgers", "No work loaded")


def find_trigger():
    return next((n for n in nodes()
                 if n.get("role") == "Button"
                 and "expanded" in n
                 and any(lbl in (n.get("label") or "") for lbl in SWITCHER_LABELS)), None)


end = time.time() + 25
trigger = None
while time.time() < end:
    trigger = find_trigger()
    if trigger:
        break
    time.sleep(0.4)
if not trigger:
    labels = sorted({(n.get("label") or "") for n in nodes() if n.get("role") == "Button"})
    die(f"no switcher trigger among the buttons; labels seen: {labels}", log)

print(f"switcher trigger: {trigger.get('label')!r} (id={trigger['id']})")
print(f"overlays before click: {overlay_count()}")

# Click it the way the user does — the pointer travels onto the trigger, then
# clicks. (The hover matters: clicking cold, with no PointerMove first, is not
# a gesture a mouse can actually produce.)
b = trigger["bounds"]
cx, cy = b["x"] + b["width"] / 2, b["y"] + b["height"] / 2
call("inject_pointer", {"x": cx, "y": cy, "action": "move"})
time.sleep(0.3)
call("inject_pointer", {"x": cx, "y": cy, "action": "click"})
call("settle")
time.sleep(0.8)

if overlay_count() == 0:
    die("clicking the switcher opened no overlay at all", log)
print(f"overlays after click: {overlay_count()}")

foc = focused_node()
print(f"focused after opening the popover: {foc and (foc.get('role'), foc.get('label'))}")

shot("/tmp/skribisto-switcher-popover.png")

# ...and the popup must actually be keyboard-live: Down must keep focus inside it.
call("inject_key", {"key": "Down"})
call("settle")
time.sleep(0.5)
after_down = focused_node()
still_open = overlay_count() > 0
print(f"focused after ArrowDown: "
      f"{after_down and (after_down.get('role'), after_down.get('label'))}, "
      f"popover still open: {still_open}")

for p in (mcp, app):
    if p.poll() is None:
        p.terminate()
shutil.rmtree(sandbox, ignore_errors=True)

# Focus must be inside the popover — not nowhere (the bug), and not left behind
# on the trigger.
if foc is None:
    die("nothing is focused: the popover opened, then its on-open re-scan rebuilt "
        "the content, destroying the row the popover had just focused — focus was "
        "dropped entirely")
if foc.get("role") not in POPUP_ROLES:
    die(f"focus is not inside the popover: it sits on a "
        f"{foc.get('role')} ({foc.get('label')!r})")
if after_down is None or not still_open:
    die("the popover lost focus (or closed) on the first ArrowDown — it is not "
        "keyboard-live")

print("PASS: clicking the switcher moves focus INTO the popover "
      f"(focus on the {foc.get('role')}), and it survives keyboard navigation")
