#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the app's own **backup folder**.

Two things this asserts, both of which shipped broken and neither of which a
headless test can reach — the first is filesystem state a launch produces, the
second is which path a button hands to the desktop:

  1. `<data_dir>/backups` exists **at launch**, before any project has been
     backed up. It used to appear only when the first backup was written into
     it, so Settings ▸ Backup defaults printed a path that was not on the disk
     — on a Flatpak install,
     `~/.var/app/eu.skribisto.skribisto/data/skribisto/backups`;
  2. the pane's **Show folder** button opens that folder, not its parent. Every
     reveal went through the helper that takes a path's *containing* folder,
     which is right for a backup file and one level too high for a directory:
     the button printed `<data>/skribisto/backups` and opened `<data>/skribisto`.

`xdg-open` is intercepted by a shim on `PATH`, so the assertion is the exact
path the app asked the desktop to open rather than a screenshot of a file
manager. `XDG_DATA_HOME` is sandboxed alongside the config dir, so the expected
root is known and the operator's real backups are never touched.

Linux only — the shim replaces `xdg-open`; the same rule holds on the other two
platforms through `open` / `explorer`.

Run it after `cargo build -p teksilo_ui` — it drives the debug binary of the
checkout it lives in, worktrees included.
"""
import json, os, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = os.path.join(REPO, "resources", "examples", "starforgers", "Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
failures = []


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


def check(ok, what):
    print(f"  {'OK  ' if ok else 'FAIL'} {what}")
    if not ok:
        failures.append(what)


class Session:
    """The launch/connect scaffolding the sibling automation_*.py scripts share."""

    def __init__(self, argv, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(argv, stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
        try:
            self.bridge = fixture.wait_for_bridge(self.log, self.app, timeout=45)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            self.mcp = subprocess.Popen(fixture.mcp_argv(self.bridge, MCP),
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "backup-root", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect MCP (socket never reachable)", self.app, self.mcp, self.log)
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
        return self.call("snapshot_tree")[1].get("nodes", [])

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def wait_label(self, substr, timeout=60):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in " | ".join(self.labels()).lower():
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

    def activate(self, node):
        self.call("invoke_action", {"node": node["id"], "action": "click"})

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


# ── A sandbox with an intercepted `xdg-open` ─────────────────────────────────
#
# `XDG_DATA_HOME` is sandboxed as well as the config dir: the whole point is to
# assert on a path under it, and a probe that wrote into the operator's own
# backup folder would both be unpredictable and leave litter behind.
SANDBOX = os.path.join(fixture.SCRATCH if os.path.isdir(fixture.SCRATCH) else tempfile.gettempdir(),
                       f"probe-backup-root-{os.getpid()}")
shutil.rmtree(SANDBOX, ignore_errors=True)
os.makedirs(os.path.join(SANDBOX, "shim"))
os.makedirs(os.path.join(SANDBOX, "data"))
OPEN_LOG = os.path.join(SANDBOX, "xdg-open.log")
shim = os.path.join(SANDBOX, "shim", "xdg-open")
with open(shim, "w", encoding="utf-8") as fh:
    fh.write(f'#!/bin/sh\necho "$@" >> "{OPEN_LOG}"\n')
os.chmod(shim, 0o755)

env = fixture.isolated_config(locale="en-US", label="backup-root", show_welcome=False)
env["XDG_DATA_HOME"] = os.path.join(SANDBOX, "data")
env["PATH"] = os.path.join(SANDBOX, "shim") + os.pathsep + env["PATH"]

EXPECTED_ROOT = os.path.join(SANDBOX, "data", "skribisto", "backups")
print(f"expected backup root: {EXPECTED_ROOT}")

print("== launch with the bundled example ==")
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="backup-root")
s = Session(fixture.launch_argv(EXAMPLE, pins=pins), env=env)

# ── 1. The folder is there before a single backup has been taken ─────────────
print("=== the app's backup folder exists at launch ===")
check(os.path.isdir(EXPECTED_ROOT), "the backup root exists before any backup ran")
kept = sorted(os.listdir(EXPECTED_ROOT)) if os.path.isdir(EXPECTED_ROOT) else []
check(kept == [], f"and it is empty, as a first launch should leave it (found {kept})")

if not s.wait_label("starforgers"):
    fail("the example never loaded", s.app, s.mcp, s.log)
print("example loaded.")

# ── 2. Settings ▸ Backup & Sync ▸ Backup defaults ▸ Show folder ──────────────
settings = s.match(["settings"], exact=True)
if not settings:
    fail("no Settings button in the activity rail", s.app, s.mcp, s.log)
s.activate(settings)
time.sleep(2.0)
if "reset to defaults" not in " | ".join(s.labels()).lower():
    fail("Settings did not open", s.app, s.mcp, s.log)


def category_tree():
    """The Settings category tree and its rows, found by a row only it has."""
    nodes = s.nodes()
    by_id = {n["id"]: n for n in nodes}

    def descendants(nid):
        out, stack = [], list((by_id.get(nid) or {}).get("children") or [])
        while stack:
            n = by_id.get(stack.pop())
            if not n:
                continue
            out.append(n)
            stack.extend(n.get("children") or [])
        return out

    for tree in (n for n in nodes if n.get("role") == "Tree"):
        rows = [n for n in descendants(tree["id"]) if n.get("role") == "TreeItem"]
        if any((r.get("label") or "").strip().lower() == "backup & sync" for r in rows):
            return tree, rows
    return None, []


def select_row(label):
    """Reveal, then activate, a category row — re-finding it between the two
    calls, since scrolling past the virtualizer's buffer rebuilds it with a
    fresh id (the recipe `automation_settings_parent_pages.py` established)."""
    want = label.lower()
    _, rows = category_tree()
    row = next((r for r in rows if (r.get("label") or "").strip().lower() == want), None)
    if not row:
        return False
    s.call("invoke_action", {"node": row["id"], "action": "scroll_into_view"})
    time.sleep(0.4)
    _, rows = category_tree()
    row = next((r for r in rows if (r.get("label") or "").strip().lower() == want), row)
    s.activate(row)
    time.sleep(1.0)
    return True


# "Backup & Sync" is a section landing page; the defaults live on the page under
# it, so select the section first to unfold it.
select_row("Backup & Sync")
time.sleep(0.6)
if not select_row("Backup defaults"):
    link = s.match(["backup defaults"])
    if not link:
        fail("could not reach the Backup defaults page", s.app, s.mcp, s.log)
    s.activate(link)
    time.sleep(1.0)

print("=== Show folder opens the folder the pane names ===")
button = s.match(["show folder"])
if not button:
    fail("no 'Show folder' button on the Backup defaults page", s.app, s.mcp, s.log)
s.activate(button)
time.sleep(2.0)

opened = open(OPEN_LOG).read().strip() if os.path.exists(OPEN_LOG) else ""
check(opened != "", "the button asked the desktop to open something")
check(opened == EXPECTED_ROOT,
      f"and it was the backup root itself, not its parent (opened {opened!r})")

s.close()
shutil.rmtree(SANDBOX, ignore_errors=True)
print()
if failures:
    print(f"FAILED ({len(failures)}):")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("PASS: the backup folder is made at launch, and Show folder opens it.")
