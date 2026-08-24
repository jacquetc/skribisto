#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Layout review of the Import documents Stepper via the teksilo automation MCP.

Opens Work ▸ Import from ▸ Documents on the bundled example, then:
  * screenshots the Files step
  * dumps the semantic tree + layout_tree bounds for the dialog, DropZone,
    Stepper chrome, footer buttons, and (pre-mounted) Destination tree
  * asserts geometry the unit tests cannot see (drop zone span, dialog size,
    step indicator labels for all three steps)

Steps 2–3 (Review / Destination) cannot be driven fully without a real file
drop or native file dialog (the bridge cannot synthesize either). Destination
content is still pre-mounted by the Stepper, so layout_tree can report its
allocated size when present. A second pass walks the AT tree for the three
step titles in the indicator strip.
"""

from __future__ import annotations

import base64
import json
import os
import pathlib
import re
import select
import subprocess
import sys
import tempfile
import time

_ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(_ROOT / "scripts"))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = str(_ROOT / "resources/examples/Starforgers.skrib")
SHOT_DIR = pathlib.Path("/tmp/skribisto-import-layout")
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

CARD_W = 920.0
CARD_H = 620.0


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-40:]))
    if os.path.exists(mcp_err):
        print("--- mcp stderr ---")
        print(open(mcp_err).read()[-2000:])
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


class Session:
    def __init__(self, args, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(
            [SKRIBISTO, *args],
            stdout=open(self.log, "w"),
            stderr=subprocess.STDOUT,
            env={**os.environ, **(env or {})},
        )
        sock = tok = None
        deadline = time.time() + 30
        while time.time() < deadline:
            txt = open(self.log).read()
            s = re.search(r"bridge socket = (\S+)", txt)
            t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
            if s and t:
                sock, tok = s.group(1), t.group(1)
                break
            if self.app.poll() is not None:
                fail("app exited before bridge socket", self.app, None, self.log)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 30s", self.app, None, self.log)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen(
                [MCP, "--connect", sock, "--token", tok],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=open(mcp_err, "w"),
                text=True,
                bufsize=1,
            )
            self._send(
                "initialize",
                {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "import-layout", "version": "1"},
                },
            )
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect MCP", self.app, self.mcp, self.log)
        self._send("notifications/initialized", notif=True)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=20, fatal=True):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                break
            r, _, _ = select.select(
                [self.mcp.stdout], [], [], max(0.0, end - time.time())
            )
            if not r:
                break
            line = self.mcp.stdout.readline()
            if not line:
                break
            if line.strip():
                return json.loads(line)
        if fatal:
            fail("no MCP response within timeout", self.app, self.mcp, self.log)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        result = self._recv().get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(
                c.get("text", "")
                for c in result.get("content", [])
                if c.get("type") == "text"
            )
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", [])

    def layout(self):
        _, p = self.call("layout_tree")
        return p

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def joined(self):
        return " | ".join(self.labels()).lower()

    def wait_label(self, substr, timeout=25):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in self.joined():
                return True
            time.sleep(0.4)
        return False

    def match(self, variants, rail_only=False, exact=False):
        for n in self.nodes():
            lab = (n.get("label") or "").strip().lower()
            if not lab:
                continue
            hit = lab in variants if exact else any(v in lab for v in variants)
            if not hit:
                continue
            if rail_only:
                b = n.get("bounds") or {}
                if (b.get("x", 9999) if isinstance(b, dict) else 9999) >= 280:
                    continue
            return n
        return None

    def click_node(self, n):
        b = n.get("bounds") or {}
        if isinstance(b, dict) and "x" in b:
            self.call(
                "inject_pointer",
                {
                    "x": b["x"] + b.get("width", 0) / 2,
                    "y": b["y"] + b.get("height", 0) / 2,
                    "action": "click",
                },
            )
            return True
        return self.call("invoke_action", {"node": n["id"], "action": "click"}) is not None

    def shot(self, path):
        path = pathlib.Path(path)
        path.parent.mkdir(parents=True, exist_ok=True)
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                path.write_bytes(base64.b64decode(c["data"]))
                print(f"screenshot -> {path}")
                return True
        print(f"screenshot FAILED (no image) -> {path}")
        return False

    def open_menu(self, title):
        ham = self.match(["menu"], rail_only=True, exact=True)
        if not ham:
            return False
        self.click_node(ham)
        time.sleep(1.0)
        top = self.match([title], exact=True)
        if not top:
            return False
        self.call("invoke_action", {"node": top["id"], "action": "click"})
        time.sleep(1.2)
        return True

    def dump_at(self, title, limit=100):
        print(f"--- AT: {title} ---")
        c = 0
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            if not lab:
                continue
            b = n.get("bounds") or {}
            geom = ""
            if isinstance(b, dict) and "width" in b:
                geom = f"  [{b.get('width'):.0f}×{b.get('height'):.0f} @ {b.get('x'):.0f},{b.get('y'):.0f}]"
            print(
                f"  [{n.get('role')}] {lab!r}"
                + ("  (disabled)" if n.get("disabled") else "")
                + geom
            )
            c += 1
            if c >= limit:
                print("  ...")
                break

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


failures = []


def check(cond, msg):
    print(("  ok   " if cond else "  FAIL ") + msg)
    if not cond:
        failures.append(msg)


def walk_layout(node, out, depth=0):
    """Flatten layout_tree nodes that carry a type name + bounds."""
    if not isinstance(node, dict):
        return
    name = node.get("type") or node.get("widget_type") or node.get("name") or ""
    bounds = node.get("bounds")
    if name and bounds:
        out.append((name, bounds, depth))
    for key in ("children", "nodes", "items"):
        kids = node.get(key)
        if isinstance(kids, list):
            for c in kids:
                walk_layout(c, out, depth + 1)


def find_layout(flat, substr):
    hits = []
    for name, bounds, depth in flat:
        if substr.lower() in name.lower():
            hits.append((name, bounds, depth))
    return hits


def fmt_bounds(b):
    if not isinstance(b, dict):
        return str(b)
    return (
        f"{b.get('width', 0):.0f}×{b.get('height', 0):.0f}"
        f" @ ({b.get('x', 0):.0f}, {b.get('y', 0):.0f})"
    )


# ---------------------------------------------------------------------------
fixture.assert_no_running_instance(SKRIBISTO)
SHOT_DIR.mkdir(parents=True, exist_ok=True)

print("== launch Starforgers ==")
s = Session(["--new-instance", EXAMPLE])
if not s.wait_label("starforgers", timeout=45):
    fail("example did not load", s.app, s.mcp, s.log)
print("loaded.")

print("\n== open Work ▸ Import from ▸ Documents ==")
check(s.open_menu("work"), "Work menu opens")
imp = s.match(["import from"])
check(imp is not None, "Import from is in the Work menu")
if imp:
    s.call("invoke_action", {"node": imp["id"], "action": "click"})
    time.sleep(1.0)
    # After Word/ODT support the label is "Documents (Markdown, Word, ODT)…"
    row = s.match(["documents (markdown"])
    check(row is not None, "Documents importer row is present")
    if row:
        s.call("invoke_action", {"node": row["id"], "action": "click"})
        time.sleep(1.5)

print("\n== Files step (step 1) ==")
opened = s.wait_label("import documents", timeout=12)
check(opened, "wizard dialog is open")
s.shot(SHOT_DIR / "01-files.png")
s.dump_at("files step")

joined = s.joined()
for needle in ("files", "review", "destination"):
    # Stepper indicator titles — may be exact or partial.
    check(needle in joined, f"step indicator includes {needle!r}")

check("drop documents here" in joined, "drop zone title is on screen")
# DropZone subtitle (format list) is painted but not always an AT `label` —
# Browse proves the zone chrome is live; formats are covered by unit tests + screenshot.
check("browse" in joined, "Browse is offered")

dlg = next(
    (
        n
        for n in s.nodes()
        if (n.get("role") or "").lower() == "dialog"
        and "import" in (n.get("label") or "").lower()
    ),
    None,
)
check(dlg is not None, "Role::Dialog with import name")
if dlg and isinstance(dlg.get("bounds"), dict):
    b = dlg["bounds"]
    print(f"  dialog geometry: {fmt_bounds(b)}")
    # Modal content is CARD_W × CARD_H; outer dialog may include chrome.
    check(b.get("width", 0) >= CARD_W * 0.85, f"dialog width ≥ 85% of card ({CARD_W})")
    check(b.get("height", 0) >= CARD_H * 0.7, f"dialog height ≥ 70% of card ({CARD_H})")

nxt = s.match(["next"], exact=True)
back = s.match(["back"], exact=True)
cancel = s.match(["cancel"], exact=True)
check(nxt is not None and nxt.get("disabled") is True, "Next greyed with no files")
# Stepper *hides* Back when can_back() is false (visible_when), rather than
# greying a dead control — so absence is the correct first-step chrome.
check(back is None, "Back is hidden on the first step (not merely greyed)")
check(cancel is not None and not cancel.get("disabled"), "Cancel is live")

print("\n== layout_tree geometry ==")
layout = s.layout()
flat = []
if isinstance(layout, dict):
    walk_layout(layout, flat)
    # Some servers wrap under "root" / "tree".
    if not flat:
        for k, v in layout.items():
            if isinstance(v, (dict, list)):
                if isinstance(v, list):
                    for item in v:
                        walk_layout(item, flat)
                else:
                    walk_layout(v, flat)
elif isinstance(layout, list):
    for item in layout:
        walk_layout(item, flat)

print(f"  layout nodes with bounds: {len(flat)}")
for needle in (
    "DropZone",
    "Stepper",
    "DestinationPicker",
    "TreeTableView",
    "ProgressBar",
    "FixedSize",
    "Expand",
):
    hits = find_layout(flat, needle)
    if not hits:
        print(f"  (no layout hit for {needle})")
        continue
    # Largest by area for reporting.
    hits.sort(
        key=lambda h: (h[1].get("width", 0) * h[1].get("height", 0))
        if isinstance(h[1], dict)
        else 0,
        reverse=True,
    )
    name, bounds, depth = hits[0]
    print(f"  {name}: {fmt_bounds(bounds)} (depth {depth}, {len(hits)} hits)")
    if needle == "DropZone" and isinstance(bounds, dict):
        w = bounds.get("width", 0)
        check(w >= CARD_W * 0.75, f"DropZone spans most of the card (width={w:.0f})")
    if needle == "DestinationPicker" and isinstance(bounds, dict):
        h = bounds.get("height", 0)
        # Pre-mounted destination step should eventually get real height when
        # active; on step 1 it may be 0. Report, don't hard-fail zero.
        print(f"    (destination pre-mount height={h:.0f}; 0 on inactive Switcher child is OK)")

# Semantic bounds for drop-related labels
drop = s.match(["drop documents here"])
if drop and isinstance(drop.get("bounds"), dict):
    b = drop["bounds"]
    print(f"  AT drop title: {fmt_bounds(b)}")
    check(b.get("width", 0) >= 200, "drop title has usable width")

print("\n== step chrome (indicator strip) ==")
# List items / tabs that look like step titles near the top of the dialog.
stepish = []
for n in s.nodes():
    lab = (n.get("label") or "").strip()
    if not lab:
        continue
    low = lab.lower()
    if low in ("files", "review", "destination", "reading") or low.startswith(
        ("files", "review", "destination")
    ):
        stepish.append(n)
        b = n.get("bounds") or {}
        print(f"  [{n.get('role')}] {lab!r}  {fmt_bounds(b) if isinstance(b, dict) else ''}")

titles = {(n.get("label") or "").strip().lower() for n in stepish}
check("files" in titles or any("files" in t for t in titles), "Files step title present")
check(
    "review" in titles or any("review" in t for t in titles), "Review step title present"
)
check(
    "destination" in titles or any("destination" in t for t in titles),
    "Destination step title present",
)

print("\n== note on steps 2–3 ==")
print(
    "  Review and Destination need chosen files (drop / native Browse)."
    " The bridge cannot inject either; drive them by hand with the sample"
    " docs from automation_import_markdown.py if you need a visual check."
)

s.shot(SHOT_DIR / "01-files-final.png")
s.close()

print()
if failures:
    print(f"{len(failures)} FAILURE(S):")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("ALL PASS — layout review of Files step + step chrome.")
print(f"screenshots in {SHOT_DIR}")
