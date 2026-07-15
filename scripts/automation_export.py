#!/usr/bin/env python3
"""Drive a live Skribisto and verify the Export feature end to end.

Flow: launch with a real project on argv (skips the Launcher) → open a scene from
the binder (selecting a row opens its editor — the outline-selection → open effect)
→ assert the title-bar **Export** split-button became focus-adaptive (its primary
region now names the scene: "Export Scene", not the disabled "Export") → click it,
which fires the `export.scope` intent and opens the shared Export panel → assert the
panel's structure (the format segments, the Style picker, the Preview, and the footer
Export button) and screenshot it.

    scripts/automation_export.py                 # default fixture project
    scripts/automation_export.py PROJECT.skrib   # a real project

Assertions favour role + stable-label anchors so the harness passes whatever UI
language is persisted. Set SKRIBISTO_BIN to point at a worktree binary.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = os.environ.get("SKRIBISTO_BIN", "/home/cyril/Devel/skribisto/target/debug/skribisto")
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
OUT = os.environ.get("SHOT_DIR", "/tmp")

_here = os.path.dirname(os.path.abspath(__file__))
_default_fixture = os.path.join(_here, "..", "resources", "test", "skribisto_test_project.skrib")
project = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.abspath(_default_fixture)
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
app = subprocess.Popen([SKRIBISTO, project], stdout=open(log, "w"), stderr=subprocess.STDOUT)

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
                        "clientInfo": {"name": "export", "version": "1"}})
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


def find(pred, timeout=6.0):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            if pred(n):
                return n
        settle()
    return None


def find_label(label, timeout=6.0):
    return find(lambda n: n.get("label") == label, timeout)


def click_node(n):
    if not n:
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
    print("  shot:", path)
    return path


def labels():
    return [n.get("label") for n in nodes() if n.get("label")]


settle()
failures = []

# ── 1. Before opening a doc: the Export control exists (disabled "Export"). ──────
export_titlebar = find(lambda n: (n.get("label") or "").startswith("Export")
                       and (n.get("bounds") or {}).get("y", 999) < 55, timeout=8)
if not export_titlebar:
    die("no Export control in the title bar before opening a doc", app, mcp)
print(f"title-bar Export control present: {export_titlebar.get('label')!r} "
      f"(enabled={export_titlebar.get('enabled')})")

# ── 2. Open a scene from the binder → the button becomes focus-adaptive. ─────────
scene = find_label("1.1 Zeus", timeout=8)
if not scene:
    die("scene '1.1 Zeus' not found in the fixture binder", app, mcp)
click_node(scene)

# Poll for the split-button's primary label to name the scene's facet.
adaptive = None
e = time.time() + 8
while time.time() < e:
    adaptive = find(lambda n: (n.get("label") or "") in
                    ("Export Scene", "Export Note", "Export Chapter", "Export Book",
                     "Export Part", "Export Folder")
                    and (n.get("bounds") or {}).get("y", 999) < 55, timeout=1)
    if adaptive:
        break
    settle()
if not adaptive:
    failures.append("the Export split-button never became focus-adaptive after opening a scene")
    print("  title-bar labels:", [l for l in labels() if "xport" in (l or "")])
else:
    print(f"adaptive Export label: {adaptive.get('label')!r}")
shot("export_1_scene_focused")

# ── 3. Click the adaptive Export → the panel opens. ─────────────────────────────
if adaptive:
    click_node(adaptive)
    settle()
    time.sleep(0.6)
    lbset = labels()

    # Panel anchors: the format segments + a Preview label + the footer Export button.
    want_formats = [f for f in ("Word (.docx)", "HTML", "Markdown", "Djot", "Text", "LaTeX")
                    if f in lbset]
    if len(want_formats) < 4:
        failures.append(f"panel format segments missing (found {want_formats})")
    else:
        print(f"panel format segments: {want_formats}")

    # Structural anchors that surface as AT nodes: the Preview band label and the footer
    # Export button. (The FormLayout field labels + the Style ComboBox value render inline
    # and aren't exposed as label nodes, so they're verified visually, not asserted.)
    for anchor in ("Preview", "Export"):
        if anchor not in lbset:
            failures.append(f"panel is missing the {anchor!r} anchor")
    shot("export_2_panel")

    # ── 4. Optionally dispatch the real export (guarded; writes + deletes a file). ──
    # OFF by default so a plain run never writes into the project directory.
    if os.environ.get("EXPORT_CLICK"):
        out_file = os.path.splitext(project)[0] + ".docx"  # default path = project stem + .docx
        try:
            os.remove(out_file)
        except OSError:
            pass
        # The footer Export button is the clickable "Export" node (the header title is a
        # non-clickable label; the split-button reads "Export Scene").
        footer = find(lambda n: n.get("label") == "Export"
                      and "click" in (n.get("actions") or []), timeout=4)
        if not footer:
            failures.append("no footer Export button to click")
        else:
            click_node(footer)
            e = time.time() + 12
            wrote = False
            while time.time() < e:
                if os.path.exists(out_file) and os.path.getsize(out_file) > 0:
                    wrote = True
                    break
                time.sleep(0.4)
            if wrote:
                print(f"export wrote {os.path.getsize(out_file)} bytes to {out_file}")
                os.remove(out_file)
            else:
                failures.append("clicking Export produced no output file")

# ── report ──────────────────────────────────────────────────────────────────────
print()
if failures:
    print("FAIL:")
    for f in failures:
        print("  -", f)
else:
    print("PASS: Export UI end-to-end (adaptive split-button → panel with preview).")

# Leave the app running (KEEP_ALIVE) so an external screencapture can grab the open
# panel; otherwise tear both down.
if mcp and mcp.poll() is None:
    mcp.terminate()
if not os.environ.get("KEEP_ALIVE") and app and app.poll() is None:
    app.terminate()
sys.exit(1 if failures else 0)
