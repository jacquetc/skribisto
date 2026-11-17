#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Settings ▸ Editor ▸ Editor Behavior ▸ Tabs kept open, driven end to end.

The headless tests in `editors/editors_vm/tests.rs` prove the view-model's rules
(least recently used, pinned tabs exempt, per pane, an unsaved tab kept). What only
a live app can prove is the wiring between them:

  1. **The pinned key reaches the editors.** `editor.tab_limit = "One"` arrives
     through `--config`, so the schema entry, the store signal `App` hands to
     `EditorsViewModel::new` and the binder's click path (`activate` → `open_in`)
     are all in the loop: clicking three chapters leaves one tab, the last.
  2. **The settings page shows the stored value**, on the segment the store holds.
  3. **Changing it there takes effect at the next open, with no restart**: pick
     All, close Settings, open two more chapters, and all three stay.

Runs against a throwaway copy of the bundled example in an isolated sandbox, with
the locale pinned to en-US so the labels matched below are the ones this repo
ships. Run it after `cargo build -p teksilo_ui`.
"""
import base64, json, os, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")
DOCS = ["Chapter 1", "Chapter 2", "Chapter 3"]

fixture.assert_no_running_instance(SKRIBISTO)
env = fixture.isolated_config(locale="en-US", label="tab-limit", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False,
     "editor.tab_limit": "One"},
    label="tab-limit")
work = fixture.working_copy(EXAMPLE, label="tab-limit")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(work, pins=pins), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
mcp = None


def die(msg):
    print("FAIL:", msg)
    print("\n".join(open(log, errors="replace").read().splitlines()[-25:]))
    for p in (mcp, app):
        if p is not None and p.poll() is None:
            p.terminate()
    sys.exit(1)


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
except RuntimeError as e:
    die(str(e))
mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, bufsize=1)
_id = 0


def call_raw(name, args=None):
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
            return m.get("result", {})


def call(name, args=None):
    r = call_raw(name, args)
    p = r.get("structuredContent")
    if p is None:
        txt = "".join(c.get("text", "") for c in r.get("content", [])
                      if c.get("type") == "text").strip()
        p = json.loads(txt) if txt.startswith(("{", "[")) else {}
    return p


def shot(path):
    for c in call_raw("screenshot").get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            print(f"  screenshot -> {path}")
            return


mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                            "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                                       "clientInfo": {"name": "tab-limit", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


def row(label):
    """The binder row named `label`: a TreeItem, so a tab of the same name never
    answers for it."""
    for n in nodes():
        if n.get("role") == "TreeItem" and (n.get("label") or "").strip() == label:
            return n
    return None


def open_doc(label):
    """One click on a binder row, the way the writer opens an item."""
    n = row(label)
    if not n:
        die(f"no binder row named {label!r}")
    call("invoke_action", {"node": n["id"], "action": "click"})
    time.sleep(1.0)


def editor_tabs():
    """The editor tabs showing one of DOCS, left to right. Filtered on DOCS because
    the dock rails are tab strips too (see `automation_tab_context_menu.py`)."""
    found = [n for n in nodes()
             if n.get("role") == "Tab" and (n.get("label") or "").strip() in DOCS]
    found.sort(key=lambda n: (n.get("bounds") or {}).get("x", 0.0))
    return [(n.get("label") or "").strip() for n in found]


def wait_tabs(want, timeout=10.0):
    end = time.time() + timeout
    got = None
    while time.time() < end:
        got = editor_tabs()
        if got == want:
            return got
        time.sleep(0.4)
    die(f"expected editor tabs {want}, saw {got}")


# `load_work` is slow in a debug build: wait for the binder, not a duration.
end = time.time() + 120
while time.time() < end and not row("Chapter 3"):
    time.sleep(0.5)
if not row("Chapter 3"):
    die("the binder never populated with the example's chapters")

# ── STEP 1: with One, each click replaces the tab ────────────────────────────
for title in DOCS:
    open_doc(title)
    wait_tabs([title])
print("  STEP 1 ok: with One pinned, each click left only the item just opened")

# ── STEP 2: the settings page shows One ──────────────────────────────────────
call("inject_key", {"key": ",", "command": True})
end = time.time() + 15
while time.time() < end and not any(
        (n.get("label") or "").strip() == "Reset to defaults" for n in nodes()):
    time.sleep(0.4)


def settings_row(label):
    for n in nodes():
        if n.get("role") == "TreeItem" and (n.get("label") or "").strip() == label:
            return n
    return None


# Editor Behavior sits under the Editor section, which may start folded.
if not settings_row("Editor Behavior"):
    section = settings_row("Editor")
    if not section:
        die("Settings opened with no Editor section in its tree")
    call("invoke_action", {"node": section["id"], "action": "expand"})
    time.sleep(0.6)
page = settings_row("Editor Behavior")
if not page:
    die("no Editor Behavior row in the Settings tree")
call("invoke_action", {"node": page["id"], "action": "click"})
time.sleep(1.0)


def is_on(node):
    """A segment is a RadioButton; its state is `toggled`, reported as a string."""
    return str(node.get("toggled")).lower() == "true"


def segments():
    """The three segments of the tab-limit control, found by their own labels and a
    shared parent, so another control's "All" cannot answer for this one's."""
    ns = nodes()
    five = next((n for n in ns if (n.get("label") or "").strip() == "Last five"), None)
    if not five:
        return {}
    parent = next((n for n in ns if five["id"] in (n.get("children") or [])), None)
    if not parent:
        return {}
    by_id = {n["id"]: n for n in ns}
    kids = [by_id[c] for c in parent.get("children") or [] if c in by_id]
    # The control also carries an unlabelled child (its selection indicator).
    return {(k.get("label") or "").strip(): k for k in kids if (k.get("label") or "").strip()}


seg = segments()
if set(seg) != {"One", "Last five", "All"}:
    die(f"the tab-limit control offers {sorted(seg)}, not One / Last five / All")
shot(os.path.join(fixture.SCRATCH, "tab-limit-settings.png"))
chosen = [label for label, n in seg.items() if is_on(n)]
if chosen != ["One"]:
    die(f"the control shows {chosen} selected while the store holds One; "
        f"the One segment reads {seg['One']}")
print("  STEP 2 ok: the settings page shows One")

# ── STEP 3: All, chosen there, applies at the next open ──────────────────────
call("invoke_action", {"node": seg["All"]["id"], "action": "click"})
time.sleep(0.6)
chosen = [label for label, n in segments().items() if is_on(n)]
if chosen != ["All"]:
    die(f"clicking All left {chosen} selected")
call("inject_key", {"key": "Escape"})
time.sleep(1.0)

open_doc("Chapter 1")
open_doc("Chapter 2")
wait_tabs(["Chapter 3", "Chapter 1", "Chapter 2"])
print("  STEP 3 ok: after picking All, every tab stayed")

for p in (mcp, app):
    if p.poll() is None:
        p.terminate()
print("PASS")
