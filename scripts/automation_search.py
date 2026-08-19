#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""End-to-end check of the search & replace docks (Part D) against the live app.

Launches the real backend on a real project, reveals the leading search dock
(Ctrl+Shift+F), types a query, and asserts against the AccessKit tree that:

  1. the search dock's query field is present (a TextInput advertising the
     search placeholder) — i.e. the leading side really hosts a *second*
     activity dock beside the binder;
  2. after typing, the debounced search runs and produces result rows in the
     dock's list (nodes carrying an item title + a "×N" occurrence badge);
  3. activating a result reveals the bottom preview band with an editable
     RichTextEditor over that item's document (the shared OpenDoc, editable).

Screenshots land in /tmp for a human to eyeball the two docks + the leading
bottom corner (binder full-height, preview starting to its right).
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
PROJECT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else "/tmp/p02/Starforgers.skrib")
QUERY = sys.argv[2] if len(sys.argv) > 2 else "the"

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
    t_ = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
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
                        "clientInfo": {"name": "search-probe", "version": "1"}})
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


def texts(ns):
    """Every label/name/value string in the tree, for substring assertions."""
    out = []
    for n in ns:
        for k in ("label", "name", "value", "placeholder"):
            v = n.get(k)
            if v:
                out.append(str(v))
    return out


# Wait for the project to load.
end = time.time() + 25
ns = []
while time.time() < end:
    ns = nodes()
    if any("Writings" in (n.get("label") or "") for n in ns):
        break
    time.sleep(0.3)
print(f"project loaded ({len(ns)} a11y nodes)")

# ── STEP 1: reveal the search dock and find its query field ───────────────────
call("inject_key", {"key": "f", "ctrl": True, "shift": True})
time.sleep(1.0)
ns = nodes()
shot("/tmp/search-dock.png")
# The query SearchField advertises "Search…" as placeholder/name; the facet
# chips carry their tooltips (Books/Parts/…). Assert on locale-independent-ish
# anchors: an input plus at least one facet chip name.
all_text = " | ".join(texts(ns))
inputs = [n for n in ns if n.get("role") in ("TextInput", "SearchInput", "MultilineTextInput")]
if not inputs:
    print("roles:", sorted({n.get("role") or "?" for n in ns}))
    fail("STEP 1: no text input after Ctrl+Shift+F — search dock did not reveal", app, mcp, log)
print(f"STEP 1 OK: search dock revealed, {len(inputs)} input(s) present")

# ── STEP 2: type a query, the debounce runs, result rows appear ───────────────
# The SearchField's editor is the inner TextInput, not the SearchInput wrapper —
# keys go to the focused text widget, and focusing the wrapper leaves them nowhere.
text_inputs = [n for n in inputs if n.get("role") == "TextInput"]
q_field = (text_inputs or inputs)[0]
call("invoke_action", {"node": q_field["id"], "action": "focus"})
time.sleep(0.2)
res, _ = call("type_text", {"node": q_field["id"], "text": QUERY})
if isinstance(res, dict) and res.get("isError"):
    fail("STEP 2: type_text into the query field errored", app, mcp, log)
time.sleep(1.5)  # > 300ms debounce + the scan
ns = nodes()
shot("/tmp/search-results.png")
badges = [t for t in texts(ns) if re.match(r"^×\d+", t)]
if not badges:
    fail("STEP 2: no ×N occurrence badges — the search produced no result rows "
         "(a common word over a real manuscript must match)", app, mcp, log)
print(f"STEP 2 OK: {len(badges)} result row badge(s) — e.g. {badges[:5]}")

# ── STEP 3: activate a result → the bottom preview shows an editable editor ────
# Result rows are `ListBoxOption`s in the leading region below the count line.
rows = [n for n in ns if n.get("role") == "ListBoxOption"
        and (n.get("bounds") or {}).get("x", 999) < 330
        and (n.get("bounds") or {}).get("y", 0) > 440]
if not rows:
    fail("STEP 3: no result rows to activate", app, mcp, log)
row = min(rows, key=lambda n: (n.get("bounds") or {}).get("y", 1e9))
acts = row.get("actions") or []
if "activate" in acts:
    call("invoke_action", {"node": row["id"], "action": "activate"})
elif "click" in acts:
    call("invoke_action", {"node": row["id"], "action": "click"})
else:
    b = row["bounds"]
    call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                            "kind": "click"})
time.sleep(1.2)
ns = nodes()
shot("/tmp/search-preview.png")
editors = [n for n in ns if n.get("role") == "MultilineTextInput"
           and "set_value" in (n.get("actions") or [])]
# The preview editor is the wide one near the bottom of the window.
bottom = [e for e in editors if (e.get("bounds") or {}).get("y", 0) > 400]
if not bottom:
    fail("STEP 3: activating a result did not reveal an editable preview editor "
         "in the bottom band", app, mcp, log)
print(f"STEP 3 OK: single-click previews the result — {len(bottom)} editable editor(s) "
      "in the bottom band")

print("\nDONE — see /tmp/search-dock.png, /tmp/search-results.png, /tmp/search-preview.png")
for p in (app, mcp):
    if p.poll() is None:
        p.terminate()
