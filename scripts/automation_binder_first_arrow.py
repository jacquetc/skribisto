#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The binder outline is a `TreeView`, and it carried the same keyboard bug the
Welcome list did: with no cursor yet, the first ArrowDown stepped to the SECOND
row, silently skipping the first (`focused_index` was `None`, `unwrap_or(0)`
made it read as row 0, and the key then stepped *past* it).

Fixed in bastyde (`tree_view/widget_impl.rs`): "no cursor" is now distinct from
"cursor on row 0", so the first Down lands ON the first row and the first Up on
the last. This drives the real binder to prove it in the running app, not just
in the framework's headless tests.

Opens the bundled example, focuses the binder tree, presses Down once, and
asserts the FIRST binder row is the selected one.
"""
import json, os, re, select, shutil, subprocess, sys, tempfile, time, base64

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = "/home/cyril/Devel/skribisto/resources/examples/Starforgers.skrib"

sandbox = tempfile.mkdtemp(prefix="skribisto_binder_kbd_")
env = {**os.environ, "XDG_CONFIG_HOME": os.path.join(sandbox, "config"),
       "XDG_DATA_HOME": os.path.join(sandbox, "data"), "HOME": sandbox}
work = os.path.join(sandbox, "Starforgers.skrib")
shutil.copyfile(EXAMPLE, work)


def die(msg, log=None):
    print("FAIL:", msg)
    if log:
        print("\n".join(open(log).read().splitlines()[-20:]))
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, work], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
sock = tok = None
end = time.time() + 25
while time.time() < end:
    t = open(log).read()
    s = re.search(r"bridge socket = (\S+)", t)
    k = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", t)
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
                                       "clientInfo": {"name": "binder-kbd", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


# Wait for the binder tree to be populated with the example's rows.
end = time.time() + 25
rows = []
while time.time() < end:
    rows = [n for n in nodes() if n.get("role") in ("TreeItem", "ListBoxOption")]
    if len(rows) >= 3:
        break
    time.sleep(0.5)
if len(rows) < 3:
    die(f"the binder never populated (got {len(rows)} rows)", log)

tree_node = next((n for n in nodes() if n.get("role") == "Tree"), None) or \
            next((n for n in nodes() if n.get("role") == "TreeGrid"), None)
if not tree_node:
    die(f"no Tree node; roles seen: {sorted({n.get('role') for n in nodes()})}", log)

selected = [i for i, n in enumerate(rows) if n.get("selected") is True]
print(f"binder rows: {len(rows)}, selected before any key: {selected}")

call("focus_node", {"node": tree_node["id"]})
time.sleep(0.4)
call("inject_key", {"key": "Down"})
time.sleep(0.6)

rows = [n for n in nodes() if n.get("role") in ("TreeItem", "ListBoxOption")]
sel = [i for i, n in enumerate(rows) if n.get("selected") is True]
res = call("screenshot")
print(f"selected row index after 1x ArrowDown: {sel}")

for p in (mcp, app):
    if p.poll() is None:
        p.terminate()
shutil.rmtree(sandbox, ignore_errors=True)

if sel != [0]:
    print(f"FAIL: the first ArrowDown selected row(s) {sel}, expected row 0 — "
          "with no cursor yet, Down must land ON the first binder row, not skip it")
    sys.exit(1)
print("PASS: the first ArrowDown in the binder lands on the FIRST row (nothing skipped)")
