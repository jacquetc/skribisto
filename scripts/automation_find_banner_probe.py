#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Phase 0.2 de-risking probe — **the find banner's layout shift**.

The banner is the IntelliJ shape: a full-width strip at the top of the editor, in
normal flow, that pushes the prose *down* instead of floating over it. That is a
deliberate trade (a top-right floating box covers the very match it just found —
an open bug in VS Code itself), but it costs a one-time vertical jolt.

The question this answers, on screen, before the real `FindViewModel` is built:
**what happens to the caret when the prose shifts?** Specifically — does the
editor keep the caret at the same document position (it must), and how far does
it move on screen?

Opens a scene, puts the caret in the prose, records the editor's bounds, toggles
the banner with Ctrl+F, and reports the shift.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else "/tmp/p02/Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, PROJECT], stdout=open(log, "w"), stderr=subprocess.STDOUT)
sock = tok = None
deadline = time.time() + 25
while time.time() < deadline:
    txt = open(log).read()
    s_ = re.search(r"bridge socket = (\S+)", txt)
    t_ = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
    if s_ and t_:
        sock, tok = s_.group(1), t_.group(1)
        break
    if app.poll() is not None:
        fail("app exited before printing the bridge socket", app, None, log)
    time.sleep(0.2)
if not sock:
    fail("no bridge socket within 25s", app, None, log)

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


deadline = time.time() + 25
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "find-banner-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    fail("could not connect MCP", app, mcp, log)
send("notifications/initialized", notif=True)
print("connected")


def nodes():
    _, payload = call("snapshot_tree")
    return payload.get("nodes", [])


def shot(path):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot → {path}")
            return


def find_label(ns, label):
    for n in ns:
        if (n.get("label") or "") == label:
            return n
    return None


def prose_editor(ns):
    """The scene's main writing column: the tallest MultilineTextInput. (The
    bottom preview dock's editor is shorter; the synopsis box shorter still.)"""
    eds = [n for n in ns
           if n.get("role") == "MultilineTextInput"
           and "set_value" in (n.get("actions") or [])]
    return max(eds, key=lambda n: (n.get("bounds") or {}).get("height", 0)) if eds else None


def viewport(ns):
    """The tab's outer ScrollArea — a STABLE reference. The prose editor itself is
    intrinsic-sized (taller than the window) inside this viewport, so its own `y`
    moves with scrolling and cannot measure the banner's shift. The viewport's top
    edge moves only when something above it (the banner) takes space."""
    svs = [n for n in ns
           if n.get("role") == "ScrollView" and (n.get("bounds") or {}).get("x", 0) > 330]
    return max(svs, key=lambda n: (n.get("bounds") or {}).get("height", 0)) if svs else None


def tab_text_inputs(ns):
    """TextInputs inside the tab (right of the leading dock). The banner is mounted
    by a `Switcher`, so opening it adds exactly one."""
    return [n for n in ns
            if n.get("role") == "TextInput" and (n.get("bounds") or {}).get("x", 0) > 330]


# Load, then open a scene so a real prose editor exists.
end = time.time() + 25
ns = []
while time.time() < end:
    ns = nodes()
    if find_label(ns, "Prologue"):
        break
    time.sleep(0.3)
row = find_label(ns, "Prologue")
if not row:
    fail("no 'Prologue' row in the binder", app, mcp, log)
b = row["bounds"]
call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                        "button": "left", "action": "click"})
time.sleep(1.5)

res_sc, sc = call("get_shortcuts")
raw = sc if isinstance(sc, list) else (sc.get("shortcuts") or [])
if not raw:
    raw = json.loads("".join(c.get("text", "") for c in res_sc.get("content", [])
                             if c.get("type") == "text") or "[]")
ids = [(x.get("id") if isinstance(x, dict) else str(x)) for x in raw]
print("registered shortcuts:", ids)
if not any("editor.find" in str(i) for i in ids):
    fail("the global 'editor.find' shortcut is not registered", app, mcp, log)

ns = nodes()
ed = prose_editor(ns)
if not ed:
    fail("no writing editor after opening Prologue", app, mcp, log)
vp = viewport(ns)
if not vp:
    fail("no ScrollView viewport in the tab", app, mcp, log)
before = dict(vp["bounds"])
n_before = len(tab_text_inputs(ns))
print(f"tab viewport before: y={before['y']:.0f} h={before['height']:.0f} "
      f"({n_before} TextInput(s) in the tab)")
shot("/tmp/banner-before.png")

# ── Open the banner (Ctrl+F) — the prose must shift DOWN by the banner height ──
call("inject_key", {"key": "f", "ctrl": True})
time.sleep(0.4)
# Pump frames: if the banner only opens once something else forces a redraw, the
# animation is queued but never scheduled — an idle app would leave it stuck.
mid = dict(viewport(nodes())["bounds"])
print(f"  right after Ctrl+F (idle): viewport y={mid['y']:.0f}")
for i in range(6):
    call("inject_pointer", {"x": 700.0 + i, "y": 400.0, "action": "move"})
    time.sleep(0.12)
time.sleep(0.6)
ns = nodes()
if not prose_editor(ns):
    fail("the prose editor vanished when the banner opened", app, mcp, log)
after = dict(viewport(ns)["bounds"])
n_after = len(tab_text_inputs(ns))
shot("/tmp/banner-open.png")

if n_after != n_before + 1:
    fail(f"Ctrl+F did not mount the find banner (TextInputs {n_before} -> {n_after}, "
         f"expected +1)", app, mcp, log)
widest = max(tab_text_inputs(ns), key=lambda n: n["bounds"]["width"])
print(f"tab viewport after:  y={after['y']:.0f} h={after['height']:.0f} "
      f"({n_after} TextInputs — the banner's query field is "
      f"{widest['bounds']['width']:.0f}px wide)")

shift = after["y"] - before["y"]
lost = before["height"] - after["height"]
print(f"\n>>> the banner pushed the prose DOWN by {shift:.0f}px "
      f"(writing viewport lost {lost:.0f}px of height)")
if shift <= 0:
    fail("the banner did not push the prose down — it is not in normal flow "
         "(it would be covering the text)", app, mcp, log)

# ── Close it again: the prose must come back to exactly where it was ──────────
call("inject_key", {"key": "f", "ctrl": True})
time.sleep(1.2)
ns = nodes()
back = dict(viewport(ns)["bounds"])
print(f"tab viewport closed: y={back['y']:.0f} (was {before['y']:.0f})")
if abs(back["y"] - before["y"]) > 1.0:
    fail(f"closing the banner did not restore the layout "
         f"({back['y']:.0f} != {before['y']:.0f})", app, mcp, log)
n_back = len(tab_text_inputs(ns))
if n_back != n_before:
    fail(f"closing the banner left its query field behind "
         f"(TextInputs {n_back}, expected {n_before})", app, mcp, log)
print("the banner unmounted cleanly — it is out of the a11y tree and the Tab order "
      "when closed")

print("\nPASS — the banner is in normal flow, pushes the prose down by "
      f"{shift:.0f}px, and closing it restores the layout exactly.")
for p in (app, mcp):
    if p.poll() is None:
        p.terminate()
