#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Settings opens again on the page the writer went to last, driven end to end.

The headless tests in `settings/tests.rs` prove the panel's rules over a store of
their own. What only a live app can prove is the store itself: that the page
written in one opening is what the next one reads, and that it survives a quit.

  1. **A fresh sandbox opens on Appearance**, and nothing is remembered yet.
  2. **Moving to Editor ▸ Focus and closing**, then opening Settings again with
     Ctrl+, lands on Focus, its row highlighted and its own controls on screen.
  3. **After quitting with Ctrl+Q and starting again** in the same sandbox,
     `general.toml` holds `ui.settings_last_page = "focus"` and Settings opens on
     Focus once more.
  4. **A page whose row is below the rail's fold** (Work ▸ Punctuation, the last
     of 22 rows in a rail that shows about 18) opens with its row scrolled into
     view, not only its page shown while the rail sits at the top.

Locale pinned to en-US in a sandboxed config dir, so the labels matched below are
the ones this repo ships. Run it after `cargo build -p teksilo_ui`.
"""
import json, os, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

env = fixture.isolated_config(locale="en-US", label="settings-last-page", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="settings-last-page")
work = fixture.working_copy(EXAMPLE, label="settings-last-page")
GENERAL = os.path.join(fixture.app_config_dir(env), "general.toml")


class App:
    """One run of the app and the MCP bridge attached to it."""

    def __init__(self):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(fixture.launch_argv(work, pins=pins),
                                    stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
        self.mcp = None
        self._id = 0
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=90)
        except RuntimeError as e:
            self.die(str(e))
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                                    stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                                    text=True, bufsize=1)
        self.send({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                   "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                              "clientInfo": {"name": "settings-last-page", "version": "1"}}})
        self.mcp.stdout.readline()
        self.send({"jsonrpc": "2.0", "method": "notifications/initialized"})
        # `load_work` is slow in a debug build: wait for the binder, not a duration.
        if not self.wait(lambda: self.tree_item("Chapter 3"), 120):
            self.die("the binder never populated with the example's chapters")

    def send(self, msg):
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def call(self, name, args=None):
        self._id += 1
        self.send({"jsonrpc": "2.0", "id": self._id, "method": "tools/call",
                   "params": {"name": name, "arguments": args or {}}})
        while True:
            line = self.mcp.stdout.readline()
            if not line:
                return {}
            m = json.loads(line)
            if m.get("id") == self._id:
                r = m.get("result", {})
                p = r.get("structuredContent")
                if p is None:
                    txt = "".join(c.get("text", "") for c in r.get("content", [])
                                  if c.get("type") == "text").strip()
                    p = json.loads(txt) if txt.startswith(("{", "[")) else {}
                return p

    def nodes(self):
        return self.call("snapshot_tree").get("nodes", [])

    def tree_item(self, label):
        return next((n for n in self.nodes() if n.get("role") == "TreeItem"
                     and (n.get("label") or "").strip() == label), None)

    def labels(self):
        return {(n.get("label") or "").strip() for n in self.nodes()}

    @staticmethod
    def wait(cond, timeout=15.0):
        end = time.time() + timeout
        while time.time() < end:
            got = cond()
            if got:
                return got
            time.sleep(0.4)
        return None

    def settings_open(self):
        return "Reset to defaults" in self.labels()

    def open_settings(self):
        self.call("inject_key", {"key": ",", "command": True})
        if not self.wait(self.settings_open):
            self.die("Ctrl+, did not open Settings")

    def close_settings(self):
        self.call("inject_key", {"key": "Escape"})
        if not self.wait(lambda: not self.settings_open()):
            self.die("Escape did not close Settings")

    def selected_page(self):
        """The highlighted rail row. Binder rows are TreeItems too, but none of
        them is named like a Settings page."""
        return next(((n.get("label") or "").strip() for n in self.nodes()
                     if n.get("role") == "TreeItem" and str(n.get("selected")).lower() == "true"
                     and (n.get("label") or "").strip() in PAGES), None)

    def rail(self):
        """The Settings rail: the Tree holding a row named Keymap."""
        ns = self.nodes()
        by_id = {n["id"]: n for n in ns}
        for tree in (n for n in ns if n.get("role") == "Tree"):
            stack = list(tree.get("children") or [])
            while stack:
                n = by_id.get(stack.pop())
                if not n:
                    continue
                if n.get("role") == "TreeItem" and (n.get("label") or "").strip() == "Keymap":
                    return tree
                stack.extend(n.get("children") or [])
        return None

    def selected_row_in_view(self):
        """Whether the highlighted rail row lies inside the rail's own bounds."""
        rail = self.rail()
        row = next((n for n in self.nodes() if n.get("role") == "TreeItem"
                    and str(n.get("selected")).lower() == "true"
                    and (n.get("label") or "").strip() in PAGES), None)
        if not rail or not row:
            return False
        r, b = rail.get("bounds") or {}, row.get("bounds") or {}
        return (b.get("y", -1) >= r.get("y", 0) - 0.5
                and b.get("y", 0) + b.get("height", 0) <= r.get("y", 0) + r.get("height", 0) + 0.5)

    def go_to(self, page, section):
        row = self.tree_item(page)
        if not row:
            sec = self.tree_item(section)
            if not sec:
                self.die(f"no {section} section in the Settings rail")
            self.call("invoke_action", {"node": sec["id"], "action": "expand"})
            row = self.wait(lambda: self.tree_item(page), 5)
        # A row below the rail's fold has no node until the rail is wheeled to it.
        # The wheel's sign is the bridge's, so try one way and turn back if the
        # rows do not change.
        rows = lambda: sorted((n.get("label") or "") for n in self.nodes()
                              if n.get("role") == "TreeItem")
        dy, flipped = -240, False
        for _ in range(24):
            if row:
                break
            rail = self.rail()
            if not rail:
                break
            before = rows()
            self.call("scroll", {"node": rail["id"], "dx": 0, "dy": dy})
            time.sleep(0.35)
            row = self.tree_item(page)
            if not row and rows() == before:
                if flipped:
                    break
                dy, flipped = -dy, True
        if not row:
            seen = [(n.get("label") or "").strip() for n in self.nodes()
                    if n.get("role") == "TreeItem"]
            self.die(f"no {page} row under {section}; rail rows: {seen}")
        self.call("invoke_action", {"node": row["id"], "action": "click"})
        if not self.wait(lambda: self.selected_page() == page, 5):
            self.die(f"{page} did not take the selection")

    def quit(self):
        """Ctrl+Q, the way the writer leaves, so the store flushes as it would."""
        self.call("inject_key", {"key": "q", "command": True})
        try:
            self.app.wait(timeout=30)
        except subprocess.TimeoutExpired:
            self.die("Ctrl+Q did not quit the app")
        self.mcp.terminate()

    def die(self, msg):
        print("FAIL:", msg)
        print("\n".join(open(self.log, errors="replace").read().splitlines()[-25:]))
        for p in (self.mcp, self.app):
            if p is not None and p.poll() is None:
                p.terminate()
        sys.exit(1)


# Every page label a selected rail row may carry in these steps.
PAGES = {"Appearance", "Focus", "Page layout", "Notifications", "Punctuation"}


def remembered():
    try:
        text = open(GENERAL, encoding="utf-8").read()
    except FileNotFoundError:
        return None
    for line in text.splitlines():
        key, _, value = line.partition("=")
        if key.strip() == "settings_last_page":
            return value.strip().strip('"')
    return None


# ── STEP 1: a fresh sandbox opens on Appearance ──────────────────────────────
a = App()
a.open_settings()
if a.selected_page() != "Appearance":
    a.die(f"a fresh sandbox opened Settings on {a.selected_page()!r}, not Appearance")
print("  STEP 1 ok: a fresh sandbox opens on Appearance")

# ── STEP 2: Focus, closed, reopened ──────────────────────────────────────────
a.go_to("Focus", "Editor")
a.close_settings()
a.open_settings()
if a.selected_page() != "Focus":
    a.die(f"reopened Settings on {a.selected_page()!r}, not on Focus")
if "Typewriter scrolling" not in a.labels():
    a.die("the Focus row is highlighted but its page is not the one shown")
print("  STEP 2 ok: Ctrl+, reopened Settings on Focus")
a.close_settings()
a.quit()

# ── STEP 3: after a quit and a fresh start ───────────────────────────────────
if remembered() != "focus":
    print(f"FAIL: general.toml remembers {remembered()!r}, not 'focus'")
    sys.exit(1)
b = App()
b.open_settings()
if b.selected_page() != "Focus":
    b.die(f"after a restart Settings opened on {b.selected_page()!r}, not on Focus")
print("  STEP 3 ok: the page survived a quit")

# ── STEP 4: a remembered row below the fold is scrolled into view ───────────
b.go_to("Punctuation", "Work: Starforgers")
b.close_settings()
# Editor was unfolded by the Focus row; fold nothing, so Punctuation is further
# down still. What the next opening must do is bring its row into view.
b.open_settings()
if b.selected_page() != "Punctuation":
    b.die(f"reopened Settings on {b.selected_page()!r}, not on Work ▸ Punctuation")
if not b.wait(b.selected_row_in_view, 5):
    b.die("Settings opened on Work ▸ Punctuation with its rail row out of view")
print("  STEP 4 ok: a remembered row below the fold opens in view")
b.close_settings()
b.quit()
print("PASS")
