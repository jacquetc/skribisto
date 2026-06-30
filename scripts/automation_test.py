#!/usr/bin/env python3
"""Drive a live Skribisto via the bastyde automation MCP bridge and assert that
the project given on the command line actually loaded.

Launches `skribisto <project>` (debug, with the automation bridge), reads the
bridge socket + token from its stderr, connects `bastyde-automation-mcp
--connect`, performs the MCP handshake, then *polls* the AccessKit tree until the
loaded work's content appears (the launch-load is driven by a backend event that
takes a few UI-thread ticks to reflect — a single early snapshot is stale).
Saves a screenshot of the settled state to /tmp/sk-auto-shot.png.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
# Resolve to an absolute path — the launched app resolves a relative path against
# its own working directory, which is not this script's.
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else
    "/home/cyril/Devel/skribisto/resources/examples/Starforgers.skrib")


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-20:]))
    try:
        et = open(mcp_err).read()
        if et.strip():
            print("--- mcp stderr ---")
            print("\n".join(et.splitlines()[-15:]))
    except Exception:
        pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


# 1. Launch the app with the project; bridge prints socket+token to stderr.
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, PROJECT], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT)
sock = tok = None
deadline = time.time() + 20
while time.time() < deadline:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt)
    t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
    if s and t:
        sock, tok = s.group(1), t.group(1)
        break
    if app.poll() is not None:
        fail("app exited before printing the bridge socket", app, None, log)
    time.sleep(0.2)
if not sock:
    fail("no bridge socket within 20s", app, None, log)
print(f"bridge up: socket={sock} token={tok[:8]}…")

# 2. Connect the MCP server to the live app.
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
def launch_mcp():
    return subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                            stderr=open(mcp_err, "w"), text=True, bufsize=1)
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
            break  # server exited (e.g. lost the announce-before-bind race)
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, end - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        line = line.strip()
        if line:
            return json.loads(line)
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

# The bridge announces the socket path *before* it is bound/listening, so an
# immediate connect races and the server dies with ENOENT (worse while the app is
# busy migrating a legacy .skrib). Wait for the socket file, then retry the whole
# connect+initialize until the server answers.
deadline = time.time() + 20
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = launch_mcp()
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "skribisto-automation-test", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    fail("could not connect MCP (socket never became reachable)", app, mcp, log)
server = init.get("result", {}).get("serverInfo", {})
print("connected:", server.get("name", "?"), server.get("version", ""))
send("notifications/initialized", notif=True)

# 3. Poll the AccessKit tree until the loaded work's content appears.
stem = os.path.splitext(os.path.basename(PROJECT))[0].lower()
def looks_loaded(labels, n):
    j = " | ".join(l for l in labels if l).lower()
    return (stem in j) or ("manuscript" in j) or ("writings" in j) or (
        "no work" not in j and n > 8)

labels = []
loaded = False
end = time.time() + 20
while time.time() < end:
    _, payload = call("snapshot_tree")
    nodes = payload.get("nodes", [])
    labels = [n.get("label") for n in nodes if n.get("label")]
    if looks_loaded(labels, len(nodes)):
        loaded = True
        break
    time.sleep(0.5)

print(f"snapshot: {len(nodes)} nodes")
print("labels:", " | ".join(l for l in labels if l)[:600])

# Screenshot the settled state for visual inspection.
try:
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open("/tmp/sk-auto-shot.png", "wb").write(base64.b64decode(c["data"]))
            print("screenshot saved -> /tmp/sk-auto-shot.png")
        elif c.get("type") == "text":
            print("screenshot note:", c.get("text", "")[:160])
except SystemExit:
    raise
except Exception as e:
    print("screenshot failed:", e)

# 4. Tidy up and report.
mcp.terminate()
app.terminate()
try:
    app.wait(timeout=3)
except Exception:
    app.kill()

if loaded:
    print(f"PASS: '{os.path.basename(PROJECT)}' loaded — its content is in the live a11y tree")
    sys.exit(0)
fail(f"project '{PROJECT}' does not appear loaded (no content nodes, empty state)")
