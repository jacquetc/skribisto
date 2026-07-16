#!/usr/bin/env python3
"""Drive a live Skribisto and verify the Book-only **"Pace" segment**.

The Pace planner is the manuscript-wide writing plan (deadline + schedule +
progress). It is exposed as an extra segment in the Book container's
`SegmentedControl` — and *only* the Book's, not a Chapter's or Part's.

    scripts/automation_pace_segment.py               # mocks build (fabricated fixture)
    scripts/automation_pace_segment.py PROJECT.skrib # a real project

Checks:
  1. Opening "Book One" shows a "Pace" segment; clicking it shows the pane.
  2. Opening "Chapter Two" / "Part One — Arrival" shows NO "Pace" segment.

Reuses the launch + Launcher→Mock-Project scaffolding from automation_streams.py.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
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
                        "clientInfo": {"name": "pace-segment", "version": "1"}})
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

# ── 1. The Book has a Pace segment, and its pane renders ────────────────────────
print("\n=== Book One → Pace segment ===")
if not click("Book One", "(binder row)"):
    failures.append("Book One: not in the binder")
else:
    if not has_label("Pace", minx=PANE_X):
        shot("pace-book-missing")
        failures.append("Book One: no 'Pace' segment in the container bar")
    else:
        print("  'Pace' segment present on the Book ✓")
        if not click("Pace", "(segment)", minx=PANE_X):
            failures.append("Book One: 'Pace' segment did not click")
        elif not (text_present("Word goal") or text_present("Schedule")):
            shot("pace-book-pane-empty")
            failures.append("Book One/Pace: the planner did not render (no schedule form)")
        else:
            print("  Pace planner renders (schedule form present) ✓")
            shot("pace-book-pane")

# ── 2. A Chapter and a Part must NOT expose a Pace segment ──────────────────────
for container in ("Chapter Two", "Part One — Arrival"):
    print(f"\n=== {container} → no Pace segment ===")
    if not click(container, "(binder row)"):
        failures.append(f"{container}: not in the binder")
        continue
    if has_label("Pace", minx=PANE_X):
        shot(f"pace-leak-{container.split()[0].lower()}")
        failures.append(f"{container}: a 'Pace' segment leaked onto a non-Book container")
    else:
        print(f"  {container} has no Pace segment ✓")

for p in (mcp, app):
    if p and p.poll() is None:
        p.terminate()

if failures:
    print("\nFAIL:")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("\nPASS: the Pace segment is Book-only and its pane renders.")
