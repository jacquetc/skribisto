#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The binder tree still notices edits after reloads were made frame-coalesced.

`models::coalesced_reload` changed three models — the binder tree, the trash tree
and the Overview — from "reload on every backend event" to "mark stale, reload once
on the next frame". That fixed a measured cost (a 200-row import re-queried and
rebuilt a 4,000-item binder 200 times; see
`import_management::a_batched_import_fires_at_most_one_event_per_row`), and it
introduced exactly one catastrophic failure mode: if the frame never arrives, the
binder tree silently stops noticing anything. The app looks fine. Nothing errors.
Renames and new items just never appear until something unrelated forces a repaint.

Unit tests cover the coalescer's logic and its wiring
(`models::coalesced_reload::tests`), but they drive `frame_tick` by hand. Only a
real window proves the *wake* actually pumps a frame — which is the half that
would fail silently, and the half a headless test cannot reach: `test_support`'s
`NullPoster` drops backend events, so no headless tree ever takes this path end to
end.

So: duplicate an item and watch the tree grow. That is one backend mutation, one
event burst, one coalesced reload, in a live window.

## Assert on titles, never on "Chapter N"

An earlier version of this script trashed "Chapter 5" and asserted the label was
gone. It never is — and the app was right. **"Chapter N" is a numbering badge,
not a title:** trash the fifth chapter and the sixth becomes "Chapter 5" a frame
later. Row counts are no better, because a reload re-applies remembered expansion
(`reload_preserving`) and can reveal rows that were not previously built.

So both checks below name an item by a real title. "Prologue" leaves the binder
tree and arrives in the Trash panel under its own name, which is a statement that
can only be true if the reload actually happened."""

import base64, json, os, pathlib, re, select, subprocess, sys, tempfile, time

_ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(_ROOT / "scripts"))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = str(_ROOT / "target/debug/skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
EXAMPLE = str(_ROOT / "resources/examples/Starforgers.skrib")
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


class Session:
    def __init__(self, args, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT,
                                    env={**os.environ, **(env or {})})
        sock = tok = None
        deadline = time.time() + 25
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
            fail("no bridge socket within 25s", self.app, None, self.log)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "tree-refresh", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate(); time.sleep(0.3)
        if init is None:
            fail("could not connect MCP", self.app, self.mcp, self.log)
        self._send("notifications/initialized", notif=True)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1; msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n"); self.mcp.stdin.flush()

    def _recv(self, timeout=20, fatal=True):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                break
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
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
            text = "".join(c.get("text", "") for c in result.get("content", [])
                           if c.get("type") == "text")
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", [])

    def tree_items(self):
        """Every binder row, by label. The tree is the thing under test."""
        return [(n.get("label") or "").strip() for n in self.nodes()
                if (n.get("role") or "") == "TreeItem" and (n.get("label") or "").strip()]

    def joined(self):
        return " | ".join(
            (n.get("label") or "") for n in self.nodes() if n.get("label")
        ).lower()

    def wait_label(self, substr, timeout=40):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in self.joined():
                return True
            time.sleep(0.4)
        return False

    def match(self, variants, exact=False):
        for n in self.nodes():
            lab = (n.get("label") or "").strip().lower()
            if not lab:
                continue
            if (lab in variants) if exact else any(v in lab for v in variants):
                return n
        return None

    def click_node(self, n):
        b = n.get("bounds") or {}
        if isinstance(b, dict) and "x" in b:
            self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                         "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
            return True
        return self.call("invoke_action", {"node": n["id"], "action": "click"}) is not None

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"screenshot -> {path}")
                return

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


failures = []


def check(cond, msg):
    print(("  ok   " if cond else "  FAIL ") + msg)
    if not cond:
        failures.append(msg)


fixture.assert_no_running_instance(SKRIBISTO)

print("== launch with the bundled example ==")
env = fixture.isolated_config(locale="en-US", label="tree-refresh")
s = Session(["--new-instance", EXAMPLE], env=env)
if not s.wait_label("starforgers", timeout=90):
    print("labels on screen:", s.tree_items()[:10], "|", s.joined()[:300])
    fail("example did not load", s.app, s.mcp, s.log)

before = s.tree_items()
check(len(before) > 5, f"the binder tree populated on load ({len(before)} rows)")

# A project loads through the same coalesced path, so a tree that populated at
# all already proves the wake pumps a frame. The duplicate below proves it again
# for an *incremental* edit, which is the case a stuck flag would break while
# leaving the initial load looking fine.
print("\n== duplicate an item (Ctrl+D) ==")
row = s.match(["chapter 3"])
check(row is not None, "found a row to duplicate")
if row:
    s.click_node(row)
    time.sleep(1.0)
    s.call("inject_key", {"key": "d", "modifiers": ["ctrl"]})

    # Poll rather than sleep-and-hope: the whole question is *whether* the
    # reload arrives, so a fixed sleep that happened to be long enough would
    # hide a slow one and a short one would report a false failure.
    grew = False
    deadline = time.time() + 10
    while time.time() < deadline:
        if len(s.tree_items()) > len(before):
            grew = True
            break
        time.sleep(0.3)

    check(grew, "the duplicate reached the binder tree "
                f"({len(before)} -> {len(s.tree_items())} rows)")
    s.shot("/tmp/tree-refresh-after-duplicate.png")

# The trash tree is the second model this changed, and Empty Trash is its own
# worst-case burst (one event per trashed item). Trashing exercises both trees at
# once: the row must leave the binder AND arrive in the trash panel, each through
# its own coalesced reload.
print("\n== trash an item ==")
# By title, not by badge, and not by count — see the module doc. "Prologue" is
# one of the few rows in the bundled example whose label is its own title.
row = s.match(["prologue"])
check(row is not None, "found a titled row to trash")
if row:
    s.click_node(row)
    time.sleep(1.0)
    selected = [n.get("label") for n in s.nodes()
                if (n.get("role") or "") == "TreeItem" and n.get("selected")]
    # `trash_selected` reads the selection model; a click that only opened a tab
    # would make the whole step a silent no-op that still "passes".
    check("Prologue" in selected, f"the click selected the row (selection: {selected})")

    ham = s.match(["menu"], exact=True)
    opened = False
    if ham:
        s.click_node(ham)
        time.sleep(1.0)
        doc = s.match(["document"], exact=True)
        if doc:
            s.call("invoke_action", {"node": doc["id"], "action": "click"})
            time.sleep(1.2)
            trash_row = s.match(["move to trash"])
            if trash_row:
                s.call("invoke_action", {"node": trash_row["id"], "action": "click"})
                opened = True
    check(opened, "Document > Move to Trash is reachable")

    if opened:
        gone = False
        deadline = time.time() + 10
        while time.time() < deadline:
            if "Prologue" not in s.tree_items():
                gone = True
                break
            time.sleep(0.3)
        check(gone, "the trashed row left the binder tree")

        # The trash tree is the second model the coalescer changed, so it gets
        # its own assertion rather than riding on the first.
        tab = s.match(["trash"], exact=True)
        check(tab is not None, "the Trash rail tab is there")
        if tab:
            s.click_node(tab)
            time.sleep(2.0)
            check("Prologue" in s.tree_items(),
                  "the trashed row arrived in the Trash panel")
            s.shot("/tmp/tree-refresh-trash-panel.png")

check(s.app.poll() is None, "the app is still running")
s.close()

print()
if failures:
    print(f"{len(failures)} FAILURE(S):")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("all checks passed.")
