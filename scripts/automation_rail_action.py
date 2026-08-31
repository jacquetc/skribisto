#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the leading rail's **Settings** action.

The rail's icons are normally *activities* — a tab with a dock behind it. The
Settings cog is the other thing that column can hold: a teksilo `DockAction`,
a dockless command that opens no panel. It is pinned past the rail's spacer
(the VS Code Manage-gear position) and fires the existing `app.settings` global
action, so the rail button, Work ▸ Settings and Ctrl+, can never drift apart.

    scripts/automation_rail_action.py                  # mocks / empty launch
    scripts/automation_rail_action.py PROJECT.skrib    # a real project

Checks:
  1. A node labelled "Settings" exists in the leading rail (x inside the rail
     strip), and it is a **button**, not a tab — an action controls no
     tabpanel, so announcing it as a tab would promise a panel that never
     appears.
  2. It sits **below** every activity item in the rail (the Pinned placement
     puts it past the spacer, at the far edge).
  3. Clicking it opens the Settings window.
  4. The rail's activity items are still tabs, and the action did NOT join
     their tab list — the ARIA structure this feature exists to keep valid.
"""
import json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
RAIL_MAX_X = 56  # the leading rail strip is ~48 dp wide

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
app = subprocess.Popen([SKRIBISTO, "--new-instance"] + ([project] if project else []),
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
        txt = "".join(c.get("text", "") for c in res.get("content", [])
                      if c.get("type") == "text")
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
                        "clientInfo": {"name": "rail-action", "version": "1"}})
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


def find(label, timeout=20.0, pred=None):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            if n.get("label") == label and (pred is None or pred(n)):
                return n
        settle()
    return None


def in_rail(n):
    b = n.get("bounds") or {}
    return b.get("x", 9999) < RAIL_MAX_X


failures = []


def check(ok, msg):
    print(("  ok   " if ok else "  FAIL ") + msg)
    if not ok:
        failures.append(msg)


# Wait for the shell (the binder rail) to exist before probing.
if not find("Binder", timeout=40) and not find("Outline", timeout=1):
    die("the project shell never appeared", app, mcp)
settle()

print("1. the Settings action is present in the leading rail")
cog = find("Settings", pred=in_rail)
if not cog:
    die("no 'Settings' node inside the leading rail strip", app, mcp)
cb = cog.get("bounds") or {}
print(f"   node={cog.get('id')} role={cog.get('role')!r} bounds={cb}")
check(str(cog.get("role", "")).lower() == "button",
      f"it is a Button, not a Tab (got {cog.get('role')!r})")

print("2. it is pinned below every activity item")
rail_tabs = [n for n in nodes()
             if str(n.get("role", "")).lower() == "tab" and in_rail(n)]
check(bool(rail_tabs), f"the rail still has activity tabs ({len(rail_tabs)})")
lowest_tab = max((n["bounds"]["y"] + n["bounds"].get("height", 0))
                 for n in rail_tabs if n.get("bounds"))
check(cb.get("y", 0) > lowest_tab,
      f"the cog sits past the activities (y={cb.get('y')} > {lowest_tab})")

print("3. clicking it opens Settings")
if "click" in (cog.get("actions") or []):
    call("invoke_action", {"node": cog["id"], "action": "click"})
else:
    call("inject_pointer", {"x": cb["x"] + cb.get("width", 0) / 2,
                            "y": cb["y"] + cb.get("height", 0) / 2, "action": "click"})
settle()
time.sleep(1.2)
labels = {n.get("label") for n in nodes()}
opened = any(l and ("Appearance" in l or "Preferences" in l or "Général" in l
                    or "General" in l) for l in labels)
check(opened, "the Settings window opened (a settings category is on screen)")
if not opened:
    print("   labels seen:", sorted(l for l in labels if l)[:40])

print()
if failures:
    print(f"FAILED ({len(failures)}):")
    for f in failures:
        print("  -", f)
else:
    print("PASS — the rail action renders as a pinned button and opens Settings")

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
sys.exit(1 if failures else 0)
