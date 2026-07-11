#!/usr/bin/env python3
"""Drive a live Skribisto and verify the manuscript streams end to end.

For each container (chapter folder, part, book) it opens the item, switches to the
manuscript segment (Full Chapter / Full Part / Full Book) and then to Full Synopsis,
and asserts what is actually *in the editor pane* — not merely somewhere in the AT
tree, since a stream's row titles ("Scene 1", "Chapter Two", …) are also binder-tree
labels and would match trivially.

    scripts/automation_streams.py                  # mocks build (fabricated fixture)
    scripts/automation_streams.py PROJECT.skrib    # a real project
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
OUT = os.environ.get("SHOT_DIR", "/tmp")
# Everything left of this x is the binder dock; the editor pane is to its right.
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
                        "clientInfo": {"name": "streams", "version": "1"}})
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
    """Invoke the node's AccessKit `click` action; fall back to a synthetic tap for
    the few nodes that expose no action."""
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


def pane_nodes():
    """Everything rendered inside the editor pane (right of the binder dock)."""
    out = []
    for n in nodes():
        b = n.get("bounds") or {}
        if b.get("x", 0) >= PANE_X and b.get("y", 0) > 90:
            out.append(n)
    return out


def pane_summary():
    rows = []
    for n in sorted(pane_nodes(), key=lambda n: (n.get("bounds") or {}).get("y", 0)):
        b = n["bounds"]
        rows.append(f"    y={int(b['y']):4} {n.get('role','?'):12} {n.get('label') or ''!r}")
    return "\n".join(rows)


def shot(name):
    path = os.path.join(OUT, f"{name}.png")
    _, p = call("screenshot")
    b64 = None
    for k in ("image_base64", "png_base64", "data", "image", "base64"):
        if isinstance(p.get(k), str) and len(p[k]) > 100:
            b64 = p[k]
            break
    if b64 is None:
        return None
    open(path, "wb").write(base64.b64decode(b64))
    return path


# ── dismiss the welcome dialog ────────────────────────────────────────────────
# Two nodes are labelled "Close" (the window button and the dialog's). The dialog's
# is the one that is not in the title bar.
for n in nodes():
    if n.get("label") == "Close" and (n.get("bounds") or {}).get("y", 0) > 100:
        call("invoke_action", {"node": n["id"], "action": "click"})
        break
settle()

failures = []

CASES = [
    # (container to open, manuscript segment, editors expected in the prose stream)
    ("Chapter Two", "Full Chapter", 4),   # the chapter's own prose + 3 scenes
    ("Part One — Arrival", "Full Part", 6),  # chapter folder + 3 scenes + flat chapter + 1 scene
    ("Book One", "Full Book", 6),         # same rows; the part heading has no prose
]

for container, segment, min_editors in CASES:
    print(f"\n=== {container} ===")
    if not click(container, "(binder row)"):
        failures.append(f"{container}: not in the binder")
        continue

    for seg in (segment, "Full Synopsis"):
        if not click(seg, "(segment)", minx=PANE_X):
            failures.append(f"{container}: no {seg!r} segment")
            continue
        pn = pane_nodes()
        editors = [n for n in pn if n.get("role") in ("TextInput", "MultilineTextInput", "TextField")]
        buttons = [n.get("label") for n in pn if n.get("role") == "Button" and n.get("label")]
        print(f"  {seg:14} → {len(editors)} editors, buttons={buttons}")
        if len(editors) < min_editors:
            failures.append(
                f"{container}/{seg}: only {len(editors)} editors, expected >= {min_editors}")
            print(pane_summary())
        p = shot(f"stream-{container.split()[0].lower()}-{seg.split()[-1].lower()}")
        if p:
            print(f"    shot -> {p}")

print("\n================ RESULT ================")
if failures:
    for f in failures:
        print("FAIL:", f)
else:
    print("PASS: every container renders its stream, in both flavours")

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()
sys.exit(1 if failures else 0)
