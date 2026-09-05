#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify **which chrome distraction-free mode keeps**.

The headless tests pin the pieces (`tab_bar_policy`, the strip's `VisibleWhen`
gates, and — in teksilo — that a bound `TabBarVisibility` flips the strip in
place). What they cannot see is the assembled window: that entering the mode
really does reach `TabWidget::bar_visibility`, and that the editor tab strip
disappears while the Exit button stays.

Flow: load a scratch copy of an example →
  1. open a scene → an editor tab strip (Role::TabList) exists
  2. Shift+F11    → the tab strip is GONE and the strip's Exit button is present
  3. Shift+F11    → the tab strip is BACK

Reuses the launch + bridge-wait + connect scaffolding from
`automation_fixture` — see that module for why the app now launches with
`--new-instance --config <pins>` and how the bridge announce is read.
"""
import base64, json, os, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

# Resolved from this script's own location, not hardcoded to the main
# checkout — this feature was built in a worktree, which has its own `target/`.
REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = os.path.join(REPO, "resources/examples/starforgers/Starforgers.skrib")

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
    def __init__(self, args, env=None, pins=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(fixture.launch_argv(list(args), pins=pins),
                                    stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=90)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "df-chrome-test", "version": "1"}})
        if self._recv(timeout=15, fatal=False) is None:
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

    def wait_label(self, substr, timeout=30):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(n.get("label") or "" for n in self.nodes()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image":
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot → {path}")

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


def click(s, n):
    if "click" in (n.get("actions") or []):
        s.call("invoke_action", {"node": n["id"], "action": "click"})
    b = n.get("bounds") or {}
    if "x" in b:
        s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                  "y": b["y"] + b.get("height", 0) / 2, "action": "click"})


# The window-control cluster's own a11y names, from teksilo-widgets'
# `a11y-window-*-name` keys. Matched exactly, not by substring: the strip's
# "Exit distraction-free mode" and the split-pane "Close split" would both trip
# a naive `"close" in label` test.
WINDOW_CONTROL_NAMES = {"minimize", "maximize", "restore", "close"}


def window_control_buttons(s):
    """Every window-control button the accessibility tree exposes."""
    return [
        n
        for n in s.nodes()
        if n.get("role") == "Button"
        and (n.get("label") or "").strip().lower() in WINDOW_CONTROL_NAMES
    ]


def tab_lists(s):
    """Every Role::TabList in the window. The editor panes are the only tab
    strips in a project window, so a non-empty result means the editor tab
    strip is mounted."""
    return [n for n in s.nodes() if n.get("role") == "TabList"]


def has_label(s, substr):
    """Match a node's accessible name, whichever field carries it.

    A `Button` publishes its name as `label`; a `Label` (what a plain
    `TextWidget` becomes — the strip's item name, the word count) publishes it
    as `value`. Reading only `label` silently misses every readout, which is how
    the item-name check below first "failed" against a strip that was showing
    the name perfectly well."""
    needle = substr.lower()
    return any(
        needle in (n.get("label") or "").lower() or needle in (n.get("value") or "").lower()
        for n in s.nodes()
    )


def wait_for(pred, timeout=10):
    end = time.time() + timeout
    while time.time() < end:
        if pred():
            return True
        time.sleep(0.4)
    return False


print("== launch with a scratch copy of the Starforgers example ==")
fixture.assert_no_running_instance()
scratch = tempfile.mkdtemp(prefix="skribisto-df-chrome-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
env = fixture.isolated_config(locale="en-US", label="df-chrome", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False},
    label="df-chrome")
s = Session([project], env=env, pins=pins)
if not s.wait_label("starforgers", timeout=30):
    fail("the example work did not load", s.app, s.mcp, s.log)
print(f"  loaded {project}")

# ── 1. Open a scene so an editor tab (and therefore a tab strip) exists ──────
print("== open a scene ==")
row = next((n for n in s.nodes() if (n.get("label") or "").strip() == "Prologue"), None)
if not row:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
editors = []
for _ in range(3):
    click(s, row)
    end = time.time() + 10
    while time.time() < end and not editors:
        editors = [n for n in s.nodes() if n.get("role") == "MultilineTextInput"
                   and "set_value" in (n.get("actions") or [])]
        if not editors:
            time.sleep(0.5)
    if editors:
        break
if not editors:
    fail("no writing editor after opening Prologue", s.app, s.mcp, s.log)

if not wait_for(lambda: len(tab_lists(s)) > 0):
    fail("no editor tab strip before entering the mode — the rest of this "
         "script would pass vacuously", s.app, s.mcp, s.log)
before = len(tab_lists(s))
print(f"  {before} tab strip(s) present outside the mode")
controls_before = len(window_control_buttons(s))
if controls_before == 0:
    fail("no window-control buttons outside the mode — the fullscreen check "
         "below would pass vacuously", s.app, s.mcp, s.log)
print(f"  {controls_before} window-control button(s) present outside the mode")

# The title bar's own slots, not just its control cluster. Gating the controls
# once cost the bar its menu, title and tools while leaving the cluster in
# place — and every assertion here still passed, because none of them looked at
# the slots. `TitleBar::build` consumes them, so anything that rebuilds the bar
# empties it; this is the check that notices.
if not has_label(s, "export"):
    s.shot("/tmp/df-chrome-empty-title-bar.png")
    fail("the title bar has no Export control outside the mode — its leading/"
         "center/trailing slots are missing", s.app, s.mcp, s.log)
print("  title-bar slots populated (Export control present)")
s.shot("/tmp/df-chrome-before.png")

# ── 2. Enter distraction-free → the tab strip goes, Exit stays ───────────────
print("== Shift+F11 enters distraction-free ==")
s.call("inject_key", {"key": "F11", "shift": True})
if not wait_for(lambda: len(tab_lists(s)) == 0, timeout=10):
    s.shot("/tmp/df-chrome-still-there.png")
    fail(f"the editor tab strip survived entering the mode "
         f"({len(tab_lists(s))} TabList node(s) remain)", s.app, s.mcp, s.log)
print("  tab strip hidden")

if not wait_for(lambda: has_label(s, "exit"), timeout=10):
    s.shot("/tmp/df-chrome-no-exit.png")
    fail("no Exit button in the distraction-free strip — the one control that "
         "must never be absent", s.app, s.mcp, s.log)
print("  Exit button present")

# The title bar goes with the rest of the chrome: minimize/maximize/close are
# meaningless over a window with no frame, and every desktop convention hides
# them in fullscreen. The strip's Exit button, Shift+F11 and Escape are the
# replacement way out — the first of which the check above just confirmed.
if not wait_for(lambda: len(window_control_buttons(s)) == 0, timeout=10):
    s.shot("/tmp/df-chrome-controls-left.png")
    left = [n.get("label") for n in window_control_buttons(s)]
    fail(f"window controls survived entering the mode: {left}", s.app, s.mcp, s.log)
print("  window controls hidden")
s.shot("/tmp/df-chrome-during.png")

# The item's name is in the strip. With the tab strip and the title bar both
# gone, and a plain Scene carrying no title field in its own pane, this is the
# only thing on screen that says which document the writer is in — and Alt+Up /
# Alt+Down move them between documents without it.
if not wait_for(lambda: has_label(s, "prologue"), timeout=10):
    s.shot("/tmp/df-chrome-no-title.png")
    fail("the strip does not name the open item — nothing on screen says what "
         "is being written", s.app, s.mcp, s.log)
print("  the strip names the open item")

# The mode's own settings are reachable without leaving it: the menu bar is
# parked dormant behind the surface, so the gear in the strip is the only route
# that does not put the whole Settings modal over the manuscript.
if not has_label(s, "distraction-free settings"):
    s.shot("/tmp/df-chrome-no-gear.png")
    fail("no quick-settings control in the strip — the mode's own settings are "
         "unreachable from inside it", s.app, s.mcp, s.log)
print("  quick-settings control present")

# The dock commands are no-ops while the surface is up. Disabling a dock side
# never disabled its *commands*: `reveal_dock` sets visible and picks a tab
# regardless, so before the guard Ctrl+Shift+F did nothing you could see here
# and then the leading rail came back showing Search instead of the binder.
for key, mods in (("F9", {}), ("F10", {}), ("F", {"ctrl": True, "shift": True})):
    s.call("inject_key", {"key": key, **mods})
time.sleep(0.5)
if len(tab_lists(s)) != 0:
    s.shot("/tmp/df-chrome-dock-command-leaked.png")
    fail("a dock command changed the desk from inside the mode", s.app, s.mcp, s.log)
print("  dock commands are no-ops inside the mode")

# ── 3. Leave the mode → the tab strip comes back ────────────────────────────
print("== Shift+F11 leaves distraction-free ==")
s.call("inject_key", {"key": "F11", "shift": True})
if not wait_for(lambda: len(tab_lists(s)) == before, timeout=10):
    s.shot("/tmp/df-chrome-not-restored.png")
    fail(f"the tab strip did not come back on leaving the mode "
         f"(want {before}, got {len(tab_lists(s))})", s.app, s.mcp, s.log)
print("  tab strip restored")
if not wait_for(lambda: len(window_control_buttons(s)) == controls_before, timeout=10):
    s.shot("/tmp/df-chrome-controls-not-restored.png")
    fail(f"the window controls did not come back on leaving the mode "
         f"(want {controls_before}, got {len(window_control_buttons(s))})",
         s.app, s.mcp, s.log)
print("  window controls restored")
if not wait_for(lambda: has_label(s, "export"), timeout=10):
    s.shot("/tmp/df-chrome-slots-not-restored.png")
    fail("the title bar came back empty — its slots did not survive the mode",
         s.app, s.mcp, s.log)
print("  title-bar slots restored")
s.shot("/tmp/df-chrome-after.png")

# ── 4. The settings surface exists and is on the right page ─────────────────
# These checkboxes are the only way a writer changes any of this, so a pane that
# silently failed to grow them would leave the feature unreachable even though
# every gate below it works. They moved off Editor Behavior when the mode's
# settings were consolidated onto one page — typography, column width and the
# strip's items together — so this looks for that page now.
print("== Settings ▸ Editor ▸ Distraction-free carries the strip checkboxes ==")


def settings_open():
    """The instant-apply footer is on every pane, so it is a page-agnostic
    "the Settings window is open" signal."""
    return has_label(s, "reset to defaults") and has_label(s, "done")


opened = None
for args in ({"key": ",", "ctrl": True}, {"key": "Comma", "ctrl": True}):
    res, _ = s.call("inject_key", args)
    if not (isinstance(res, dict) and res.get("isError")):
        time.sleep(0.8)
        if settings_open():
            opened = args
            break
if not opened:
    fail("could not open the Settings window", s.app, s.mcp, s.log)

page = next((n for n in s.nodes()
             if (n.get("label") or "").strip().lower() == "distraction-free"), None)
if not page:
    fail("no 'Distraction-free' page row in the settings category rail", s.app, s.mcp, s.log)
if "click" in (page.get("actions") or []):
    s.call("invoke_action", {"node": page["id"], "action": "click"})
else:
    b = page.get("bounds") or {}
    s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                              "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
time.sleep(0.8)

WANT = ["keep the item's name", "keep the word count",
        "keep the writing session", "keep the previous and next buttons"]
missing = [w for w in WANT if not has_label(s, w)]
s.shot("/tmp/df-chrome-settings.png")
if missing:
    fail(f"the Distraction-free pane is missing {missing}", s.app, s.mcp, s.log)
print("  all four checkboxes present")

# The "keep the editor tabs" setting was deleted with the chrome collapse it
# governed: the mode does not undress the shell any more, it covers it, and its
# surface shows exactly one document with no tab row for a setting to act on.
if has_label(s, "keep the editor tabs"):
    fail("the editor-tabs checkbox is still here — it governs nothing now",
         s.app, s.mcp, s.log)
print("  and no editor-tabs checkbox, which now governs nothing")

# Exit is promised to have no checkbox — a settings row that could take it away
# would defeat the strip's whole reason for being always-visible.
if has_label(s, "keep the exit"):
    fail("an Exit checkbox appeared — Exit must never be optional",
         s.app, s.mcp, s.log)
print("  and no Exit checkbox, as promised")

print("\nPASS: distraction-free hides the editor tab strip, keeps Exit, restores "
      "the strip on the way out, and its toggles are reachable in Settings.")
s.close()
