#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Exercise the *drive* side of the teksilo automation MCP against a live
Skribisto: load a work, then open a binder item by invoking its AT action (and,
as a fallback, a synthetic pointer click), and verify an editor tab appears.

Prints the relevant tool schemas (self-guiding) and notes anything that looks
like an MCP bug or gap.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else
    fixture.repo_path("resources/examples/starforgers/Starforgers.skrib"))
TARGET = "Prologue"   # binder item to open


mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

def die(msg, *procs, log=None):
    print("ERROR:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-15:]))
    try:
        et = open(mcp_err).read()
        if et.strip():
            print("--- mcp stderr ---")
            print("\n".join(et.splitlines()[-15:]))
    except Exception:
        pass
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, PROJECT], stdout=open(log, "w"), stderr=subprocess.STDOUT)
sock = tok = None
end = time.time() + 20
while time.time() < end:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt); t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s and t:
        sock, tok = s.group(1), t.group(1); break
    if app.poll() is not None:
        die("app exited early", app, log=log)
    time.sleep(0.2)
if not sock:
    die("no bridge socket", app, log=log)
print(f"bridge up: {sock}")

def launch_mcp():
    return subprocess.Popen([MCP, "--connect", sock, "--token", tok], stdin=subprocess.PIPE,
                            stdout=subprocess.PIPE, stderr=open(mcp_err, "w"), text=True, bufsize=1)

mcp = None
_id = [0]
def send(method, params=None, notif=False):
    m = {"jsonrpc": "2.0", "method": method}
    if params is not None: m["params"] = params
    if not notif:
        _id[0] += 1; m["id"] = _id[0]
    mcp.stdin.write(json.dumps(m) + "\n"); mcp.stdin.flush()
def recv(timeout=20, fatal=True):
    e = time.time() + timeout
    while time.time() < e:
        if mcp.poll() is not None:
            break  # server exited (e.g. lost the announce-before-bind race)
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, e - time.time()))
        if not r: break
        line = mcp.stdout.readline()
        if not line: break
        if line.strip(): return json.loads(line)
    if fatal:
        die("no MCP response", app, mcp, log=log)
    return None
def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    res = recv().get("result", {})
    payload = res.get("structuredContent")
    if payload is None:
        txt = "".join(c.get("text", "") for c in res.get("content", []) if c.get("type") == "text")
        payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
    return res, payload

# The bridge announces the socket path *before* it is bound/listening, so an
# immediate connect races and the MCP server dies with ENOENT — worse while the
# app is busy migrating a legacy .skrib. Wait for the socket file, then retry the
# whole connect+initialize until the server answers.
deadline = time.time() + 20
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = launch_mcp()
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "explore", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None:
        if mcp.poll() is None: mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP (socket never became reachable)", app, mcp, log=log)
send("notifications/initialized", notif=True)

# Tool schemas (self-guiding).
send("tools/list")
tools = {t["name"]: t for t in recv().get("result", {}).get("tools", [])}
print(f"tools: {len(tools)} ->", ", ".join(sorted(tools)))
for n in ("invoke_action", "inject_pointer", "find_node"):
    if n in tools:
        print(f"\n# {n} schema:", json.dumps(tools[n].get("inputSchema", {}))[:400])

def snap():
    _, p = call("snapshot_tree")
    ns = p.get("nodes", [])
    return ns, {n.get("label"): n for n in ns if n.get("label")}

# Wait for the (legacy-migrated) work to load.
nodes = bylabel = None
end = time.time() + 25
while time.time() < end:
    nodes, bylabel = snap()
    if TARGET in bylabel:
        break
    time.sleep(0.5)
print(f"\nloaded: {len(nodes)} nodes; target '{TARGET}' present:", TARGET in bylabel)
if TARGET not in bylabel:
    die(f"'{TARGET}' never appeared", app, mcp, log=log)

node = bylabel[TARGET]
print(f"target node: id={node.get('id')} role={node.get('role')} "
      f"actions={node.get('actions')} bounds={node.get('bounds')}")

def tab_open():
    """Heuristic: an editor tab/pane for TARGET appeared (a 2nd labelled
    occurrence, or a tab/document/textbox node referencing it)."""
    ns, _ = snap()
    hits = [n for n in ns if (n.get("label") or "").strip() == TARGET]
    roles = [n.get("role") for n in ns]
    return len(hits) >= 2, roles

before2 = sum(1 for n in nodes if (n.get('label') or '').strip() == TARGET)
print("TARGET occurrences before open:", before2)

opened = False
method = None
# Try the semantic AT action first.
if node.get("actions"):
    print("\n-> invoke_action on target")
    try:
        res, _ = call("invoke_action", {"node": node["id"]})
        time.sleep(1.0)
        ok, roles = tab_open()
        if ok:
            opened, method = True, "invoke_action(node)"
    except SystemExit:
        raise
    except Exception as e:
        print("invoke_action raised:", e)

# Fallback: synthetic pointer click at the node centre.
if not opened and node.get("bounds"):
    b = node["bounds"]
    # bounds may be {x,y,width,height} or [x,y,w,h]
    if isinstance(b, dict):
        cx, cy = b.get("x", 0) + b.get("width", 0) / 2, b.get("y", 0) + b.get("height", 0) / 2
    else:
        cx, cy = b[0] + b[2] / 2, b[1] + b[3] / 2
    print(f"\n-> inject_pointer click at ({cx:.0f},{cy:.0f})")
    for args in ({"x": cx, "y": cy, "action": "click"}, {"x": cx, "y": cy, "button": "left"},
                 {"x": cx, "y": cy}):
        try:
            res, _ = call("inject_pointer", args)
            if isinstance(res, dict) and res.get("isError"):
                print("  args", args, "-> error:", json.dumps(res)[:160]); continue
            time.sleep(1.0)
            ok, roles = tab_open()
            if ok:
                opened, method = True, f"inject_pointer({list(args)})"
                break
            print("  args", args, "-> no tab yet")
        except SystemExit:
            raise
        except Exception as e:
            print("  args", args, "raised:", e)

# Screenshot the result.
try:
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open("/tmp/sk-explore-shot.png", "wb").write(base64.b64decode(c["data"]))
            print("screenshot -> /tmp/sk-explore-shot.png")
except Exception as e:
    print("screenshot failed:", e)

mcp.terminate(); app.terminate()
try: app.wait(timeout=3)
except Exception: app.kill()

print(f"\nRESULT: opened={opened} via {method}")
sys.exit(0 if opened else 2)
