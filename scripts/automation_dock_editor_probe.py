#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Phase 0.2 de-risking probe — **can an editable RichTextEditor live in a dock?**

No editable widget exists inside any `DockWidget` in Skribisto or in teksilo's
own examples: every dock so far is a tree, a list, or read-only text. A dock
side's content is `visible_when`-parked while the side is collapsed
(teksilo-widgets `docking.rs`), a path only ever exercised by non-focusable
content. The search feature's bottom preview depends on this working, so find
out *now*, not after the preview is built on the assumption.

Asserts, against the live app, with NO editor tab open (so the only writing
editor in the tree is the docked one):

  1. the bottom "Preview (stub)" dock exposes a real `MultilineTextInput` that
     AccessKit can see and that advertises `set_value`;
  2. `type_text` on it lands — the docked editor takes focus and edits;
  3. after a full docking relayout (F9 toggles the outline side), the editor is
     still present, still editable, and its earlier text is intact — i.e. it
     survives the park/unpark cycle rather than being silently rebuilt or
     detached.
"""
import base64, json, os, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
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


fixture.assert_no_running_instance()
env = fixture.isolated_config(locale="en-US", label="dock-editor", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="dock-editor")
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(PROJECT, pins=pins),
                       stdout=open(log, "w"), stderr=subprocess.STDOUT, env=env)
try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
except RuntimeError as e:
    fail(str(e), app, None, log)

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
    mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "dock-editor-probe", "version": "1"}})
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


def editors(ns):
    return [n for n in ns
            if n.get("role") == "MultilineTextInput"
            and "set_value" in (n.get("actions") or [])]


def editor_text(node_id, ns=None):
    """A RichTextEditor's a11y node carries its prose in per-block CHILD nodes,
    not in `value` (which stays empty). `children` are node *ids*, so resolve
    them against the flat snapshot."""
    ns = ns if ns is not None else nodes()
    by_id = {n["id"]: n for n in ns if "id" in n}
    node = by_id.get(node_id)
    if node is None:
        return ""
    parts = []

    def walk(n):
        for k in ("label", "value", "name"):
            if n.get(k):
                parts.append(str(n[k]))
                break
        for cid in (n.get("children") or []):
            child = by_id.get(cid)
            if child:
                walk(child)

    walk(node)
    return " ".join(parts)


# Wait for the project to load.
end = time.time() + 25
ns = []
while time.time() < end:
    ns = nodes()
    if any("Writings" in (n.get("label") or "") for n in ns):
        break
    time.sleep(0.3)
print(f"project loaded ({len(ns)} a11y nodes)")

# ── STEP 1: the docked editor is in the a11y tree ─────────────────────────────
# No editor TAB is open (fresh load), so the only writing editor in the whole
# tree must be the one inside the bottom preview dock.
eds = editors(ns)
if not eds:
    roles = sorted({n.get("role") or "?" for n in ns})
    print("roles present:", roles)
    fail("STEP 1: the docked editor is NOT in the a11y tree — a dock cannot host "
         "an editable RichTextEditor as built", app, mcp, log)
ed = max(eds, key=lambda n: (n.get("bounds") or {}).get("height", 0))
print(f"STEP 1 OK: docked editor visible to AccessKit — id={ed['id']} "
      f"role={ed['role']} bounds={ed.get('bounds')} actions={ed.get('actions')}")

# ── STEP 2: it takes focus and edits ──────────────────────────────────────────
before = editor_text(ed["id"])
print(f"  text before: {before[:70]!r}")
res, _ = call("type_text", {"node": ed["id"], "text": "PROBE-ONE "})
if isinstance(res, dict) and res.get("isError"):
    fail("STEP 2: type_text on the docked editor errored — no focus / keys "
         "swallowed by the dock", app, mcp, log)
time.sleep(1.5)
after = editor_text(ed["id"])
print(f"  text after:   {after[:120]!r}")
shot(("/tmp/dock-probe-typed.png"))
if "PROBE-ONE" not in after:
    fail("STEP 2: typing into the docked editor did not change its a11y text", app, mcp, log)
print("STEP 2 OK: the docked editor takes focus and edits")

# ── STEP 3: survive the BOTTOM band's own park / unpark ───────────────────────
# This is the path that has never been exercised with focusable content: when a
# side collapses, its content is `ctx.visible_when(...)`-parked — out of paint,
# focus and the a11y tree. F9 (leading side) only relayouts; it does NOT park the
# bottom dock. F10 is the stub's bottom-band toggle, added for exactly this.
call("inject_key", {"key": "F10"})         # collapse the bottom band → PARK
time.sleep(1.0)
parked = editors(nodes())
print(f"  while collapsed: {len(parked)} editor node(s) in the a11y tree "
      f"(0 = correctly parked out of the tree)")
call("inject_key", {"key": "F10"})         # reveal it again          → UNPARK
time.sleep(1.2)

ns = nodes()
eds = editors(ns)
if not eds:
    fail("STEP 3: the docked editor DID NOT COME BACK after the bottom band was "
         "collapsed and revealed — parking destroys it", app, mcp, log)
ed = max(eds, key=lambda n: (n.get("bounds") or {}).get("height", 0))
kept = editor_text(ed["id"])
if "PROBE-ONE" not in kept:
    print(f"  text now: {kept[:120]!r}")
    fail("STEP 3: the editor lost its content across the relayout (rebuilt, not parked)",
         app, mcp, log)

res, _ = call("type_text", {"node": ed["id"], "text": "PROBE-TWO "})
if isinstance(res, dict) and res.get("isError"):
    fail("STEP 3: the docked editor no longer accepts input after a relayout",
         app, mcp, log)
time.sleep(1.2)
final = editor_text(ed["id"])
if "PROBE-TWO" not in final:
    print(f"  text now: {final[:160]!r}")
    fail("STEP 3: after a relayout the editor no longer edits", app, mcp, log)
print("STEP 3 OK: the editor survives the bottom band's park/unpark, keeps its text, still edits")

shot("/tmp/dock-editor-probe.png")

print(f"\nfinal text: {final[:160]!r}")
print("\nPASS — an editable RichTextEditor works inside a dock.")
for p in (app, mcp):
    if p.poll() is None:
        p.terminate()
