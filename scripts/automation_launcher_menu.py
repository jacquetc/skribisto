#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""The **Launcher window's menu bar** and the Works pane's "Create from…" button.

The Launcher used to mount no `MenuBar` at all, so its two importer doors were
reachable by pointer only and — on macOS — a focused Launcher left the last
project window's menu sitting in the global bar. It now builds a model of its
own (`shell::launcher_menu`), collapsed to the same hamburger the project window
uses.

Everything here is about **two surfaces reaching one command**, which is exactly
what no headless test can see: the menu row and the button both fire a *named
global action*, and those actions are registered on the Launcher's own widget
tree by `WelcomePanel::build` — a tree that never builds an `App`, so nothing
`app/commands/*.rs` registers is reachable from it. A row wired to an action no
one registered opens nothing and reports no error, which is the failure this
probe exists to catch.

What it asserts, in order:

  1. **The hamburger is in the Launcher's title bar** and opens onto exactly one
     menu, Work. (The App and Window standard menus are macOS-only and invisible
     to the in-window bar — if one ever leaked into it, it would show up here.)
  2. **Work lists the four ways in, the preferences and the way out**: New
     Work, Open Work, Create from ▸, Settings, Quit.
  3. **Create from ▸ offers Document and Plume Creator**, and neither is a dead
     row: activating Plume Creator opens the Import Plume Creator modal, which
     is the whole point — that flow needs no open project, and until now the
     Launcher could not reach it.
  4. **The Works pane's "Create from…" popover offers the same two**, and its
     Document row opens the New Work wizard in its from-documents shape.
  5. **Ctrl+N reaches New Work from the Launcher.** The window registers
     `work.new`/`work.open` shortcuts of its own now; a writer arriving from a
     project window already has those chords.
  6. **Settings opens from the Launcher**, by its menu row *and* by Ctrl+, —
     and opens in its no-project shape (every app-level section reachable, no
     Work section). This is where the app starts on Linux and Windows, so while
     the row was withheld the theme, the text scale, the dictionaries, the
     keybindings and the backup defaults could not be changed until a project
     had been created or opened.
  7. **Quit really quits.** The Launcher's `app.quit` runs the shared
     `QuitSequencer` rather than closing its own window — it used to do the
     latter, which under single-instance left the app running with project
     windows still up.

Runs in an isolated `XDG_CONFIG_HOME` pinned to en-US, so the label matching
below is against strings this probe controls rather than whatever locale the
operator happens to have persisted.
"""

import json, os, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
SHOTS = os.environ.get("SHOT_DIR", tempfile.mkdtemp(prefix="skribisto_launcher_menu_"))


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    try:
        et = open(mcp_err).read()
        if et.strip():
            print("--- mcp stderr ---")
            print("\n".join(et.splitlines()[-15:]))
    except Exception:
        pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


class Session:
    """One launched app + connected MCP server."""

    def __init__(self, argv, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(argv, stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=25)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = None
        # `wait_for_bridge` already asserts the endpoint exists by the time it
        # returns; the retry loop below is only for the (rarer) case where the
        # accept thread is not yet scheduled to service a connection.
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "launcher-menu", "version": "1"}})
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

    def _recv(self, timeout=25, fatal=True):
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
            t = text.strip()
            payload = json.loads(t) if (t.startswith("{") or t.startswith("[")) else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", []) if isinstance(p, dict) else []

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def wait_label(self, substr, timeout=30):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in " | ".join(self.labels()).lower():
                return True
            time.sleep(0.4)
        return False

    def find_exact(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def find_contains(self, substr, role=None, timeout=8):
        end = time.time() + timeout
        while time.time() < end:
            for n in self.nodes():
                if substr.lower() in (n.get("label") or "").lower() and (
                    role is None or n.get("role") == role
                ):
                    return n
            time.sleep(0.3)
        return None

    def roles(self, *want):
        w = {r.lower() for r in want}
        return [n for n in self.nodes() if (n.get("role") or "").lower() in w]

    def click(self, node):
        b = node.get("bounds") or {}
        if "x" not in b:
            return False
        self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                     "y": b["y"] + b.get("height", 0) / 2,
                                     "action": "click"})
        return True

    def activate(self, node):
        """Fire `node`'s AccessKit action rather than a synthetic click.

        The menu bar is driven this way for the reason `automation_new_window`
        documents: a press+release at a top-level entry's centre does not open
        its submenu into the AT tree, while `invoke_action` does.
        """
        res, _ = self.call("invoke_action", {"node": node["id"], "action": "Click"})
        return not (isinstance(res, dict) and res.get("isError"))

    def key(self, key, **mods):
        """`inject_key` takes its modifiers as plain booleans (`ctrl=True`),
        not a list — see the other probes."""
        self.call("inject_key", {"key": key, **mods})

    def shot(self, name):
        path = os.path.join(SHOTS, name)
        try:
            import base64
            res, _ = self.call("screenshot")
            for c in res.get("content", []):
                if c.get("type") == "image" and c.get("data"):
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"  screenshot -> {path}")
        except Exception as e:
            print("  screenshot failed:", e)

    def dump(self, header=""):
        print(f"--- AT tree{(' ' + header) if header else ''} ---")
        for n in self.nodes():
            lab = (n.get("label") or "").replace("\n", " ")
            print(f"  role={str(n.get('role')):<16} label={lab[:46]!r:<48} id={n.get('id')}")

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


def menu_labels(s):
    return [n.get("label") for n in s.nodes() if n.get("role") == "MenuItem"]


def dismiss_menu(s):
    s.key("Escape")
    time.sleep(0.4)
    s.key("Escape")
    time.sleep(0.4)


def dismiss_modal(s, timeout=8):
    """Escape until no `Form` landmark is left — the two wizards this probe
    opens are `EscapeOrClickOutside` modals, but a focused text field may eat
    the first keystroke, and a modal still up blocks the title bar."""
    end = time.time() + timeout
    while time.time() < end:
        if not next(iter(s.roles("Form")), None):
            return True
        s.key("Escape")
        time.sleep(0.6)
    return False


print("== Launcher menu bar + Create from… ==")
env = fixture.isolated_config(locale="en-US", label="launcher-menu")
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": True},
    label="launcher-menu")
# `fixture.launch_argv` always adds `--new-instance`: a primary already running
# on this machine would otherwise win the election and this launch would hand
# off and exit, leaving the probe driving nothing. No project: this probe is
# about the Launcher itself.
s = Session(fixture.launch_argv(None, pins=pins), env=env)
if not s.wait_label("welcome sections"):
    s.dump("startup")
    fail("the Launcher window did not appear", s.app, s.mcp, s.log)
print("Launcher window is up.")
s.shot("01-launcher.png")

# ── 1. The hamburger, and what it opens onto ─────────────────────────────────
ham = s.find_exact("Menu", role="Button")
if not ham:
    s.dump("no hamburger")
    fail("no title-bar 'Menu' (hamburger) button in the Launcher", s.app, s.mcp, s.log)
print("hamburger present in the Launcher title bar.")
if not s.activate(ham):
    fail("the hamburger refused its Click action", s.app, s.mcp, s.log)
time.sleep(0.9)

top = menu_labels(s)
if top != ["Work"]:
    s.dump("hamburger open")
    fail(f"the hamburger must open onto exactly the Work menu, saw {top!r}",
         s.app, s.mcp, s.log)
print("hamburger opens onto exactly one menu: Work.")

# ── 2. The Work menu's rows ──────────────────────────────────────────────────
work = s.find_exact("Work", role="MenuItem")
if not s.activate(work):
    fail("the Work menu refused to open", s.app, s.mcp, s.log)
time.sleep(1.0)
rows = [r for r in menu_labels(s) if r != "Work"]
print("  Work ▸", rows)
for want in ("New Work", "Open Work…", "Create from", "Settings", "Quit"):
    if not any(want.lower() in (r or "").lower() for r in rows):
        s.dump("Work menu")
        fail(f"the Work menu is missing a {want!r} row (saw {rows!r})", s.app, s.mcp, s.log)
s.shot("02-work-menu.png")

# ── 3. Create from ▸ Document / Plume Creator ────────────────────────────────
create = s.find_contains("Create from", role="MenuItem")
if not s.activate(create):
    fail("the Create from submenu refused to open", s.app, s.mcp, s.log)
time.sleep(1.0)
# The parent menu stays in the AT tree while its submenu is open, so subtract
# its rows rather than printing the union and calling it the submenu.
sub = [r for r in menu_labels(s) if r not in rows and r != "Work"]
print("  Create from ▸", sub)
for want in ("Document", "Plume Creator"):
    if not any(want.lower() in (r or "").lower() for r in sub):
        s.dump("Create from submenu")
        fail(f"Create from is missing a {want!r} row (saw {sub!r})", s.app, s.mcp, s.log)
s.shot("03-create-from-submenu.png")

# The row is only worth anything if it reaches something. Plume, from the menu:
# this flow needs no open project, and the Launcher could not reach it at all
# before — so it is the one that proves the action really is registered on this
# window's tree.
plume = next(n for n in s.nodes()
             if n.get("role") == "MenuItem" and "plume" in (n.get("label") or "").lower())
s.activate(plume)
time.sleep(1.4)
if not s.find_contains("Plume project", timeout=8) and not s.find_contains("Import", timeout=2):
    s.dump("after Create from ▸ Plume Creator")
    fail("Create from ▸ Plume Creator opened no Import Plume modal", s.app, s.mcp, s.log)
print("Create from ▸ Plume Creator opens the Import Plume Creator modal.")
s.shot("04-import-plume-from-menu.png")
dismiss_menu(s)

# ── 4. The Works pane's own "Create from…" popover ───────────────────────────
btn = s.find_contains("Create from", role="Button", timeout=8)
if not btn:
    s.dump("no Create from button")
    fail("no 'Create from…' button in the Works pane", s.app, s.mcp, s.log)
if not s.activate(btn):
    fail("the 'Create from…' button refused its Click action", s.app, s.mcp, s.log)
time.sleep(0.9)
pop = menu_labels(s)
print("  Create from… popover:", pop)
for want in ("Document", "Plume Creator"):
    if not any(want.lower() in (r or "").lower() for r in pop):
        s.dump("Create from popover")
        fail(f"the popover is missing a {want!r} row (saw {pop!r})", s.app, s.mcp, s.log)
s.shot("05-create-from-popover.png")

doc = next(n for n in s.nodes()
           if n.get("role") == "MenuItem" and "document" in (n.get("label") or "").lower())
s.activate(doc)
time.sleep(1.6)
if not next(iter(s.roles("Form")), None):
    s.dump("after Create from… ▸ Document")
    fail("the popover's Document row opened no New Work wizard", s.app, s.mcp, s.log)
print("Create from… ▸ Document opens the New Work (from documents) wizard.")
s.shot("06-from-documents-wizard.png")
if not dismiss_modal(s):
    fail("the from-documents wizard would not close", s.app, s.mcp, s.log)

# ── 5. Ctrl+N reaches New Work from the Launcher ─────────────────────────────
time.sleep(0.6)
s.key("n", ctrl=True)
time.sleep(1.6)
if not next(iter(s.roles("Form")), None):
    s.dump("after Ctrl+N")
    fail("Ctrl+N did not open New Work in the Launcher", s.app, s.mcp, s.log)
print("Ctrl+N opens New Work from the Launcher.")
s.shot("07-ctrl-n-new-work.png")
if not dismiss_modal(s):
    fail("the New Work wizard would not close", s.app, s.mcp, s.log)

# ── 6. Settings reaches the preferences window from the Launcher ─────────────
# The row that was withheld the longest, and the one with the most room to be
# dead: `app.settings` opens `SettingsPanel`, which used to demand a Tier-2
# `WorkSession` this window has none of. On Linux and Windows the app *starts*
# here, so while it was withheld the theme, the interface text scale, the
# dictionaries, the keybindings and the backup defaults could not be reached at
# all before a project existed.
#
# Both halves are checked, because they are registered separately and either can
# rot alone: the menu row (which fires the *named action*, and would open
# nothing at all had `WelcomePanel::build` used `register_action` instead of
# `register_action_global` — the intent walks source-widget → root and this menu
# renders in an overlay that is a sibling of the window root) and Ctrl+, (a
# `register_shortcut_global` on this window's own registry, since the Launcher
# never builds an `App` and so cannot reach `app/commands/file.rs`).


def open_settings(s, via):
    """Wait for the Settings dialog, whichever door was just used."""
    end = time.time() + 10
    while time.time() < end:
        for n in s.nodes():
            if (n.get("role") == "Dialog"
                    and (n.get("label") or "").strip() == "Settings"):
                return n
        time.sleep(0.4)
    s.dump(f"after {via}")
    fail(f"{via} opened no Settings window", s.app, s.mcp, s.log)


def close_settings(s, timeout=8):
    """Escape until the dialog is gone — it is an `EscapeKey` modal, but the
    rail's search field takes the initial focus and may eat the first press."""
    end = time.time() + timeout
    while time.time() < end:
        if not any(n.get("role") == "Dialog"
                   and (n.get("label") or "").strip() == "Settings" for n in s.nodes()):
            return True
        s.key("Escape")
        time.sleep(0.6)
    return False


time.sleep(0.6)
settings_row = None
for _ in range(4):
    ham = s.find_exact("Menu", role="Button")
    if ham:
        s.activate(ham)
        time.sleep(0.8)
    work = s.find_exact("Work", role="MenuItem")
    if work:
        s.activate(work)
        time.sleep(1.0)
    settings_row = s.find_exact("Settings", role="MenuItem")
    if settings_row:
        break
    dismiss_menu(s)
if not settings_row:
    s.dump("looking for Settings")
    fail("no Settings row in the Launcher's Work menu", s.app, s.mcp, s.log)
s.activate(settings_row)
open_settings(s, "Work \u25b8 Settings")
print("Work \u25b8 Settings opens the preferences window from the Launcher.")
s.shot("08-settings-from-launcher.png")

# The no-project shape, live: every app-level section is there and the Work
# section is not. `tree_spec(false, ..)` is unit-tested, but only the running
# window proves the panel really was built in its no-project variant rather
# than refusing, half-building, or panicking behind the modal.
rail = [(n.get("label") or "").strip() for n in s.nodes()]
for want in ("Appearance", "Spelling"):
    if not any(want.lower() in r.lower() for r in rail):
        s.dump("settings rail")
        fail(f"the app-level {want!r} section is unreachable from the Launcher",
             s.app, s.mcp, s.log)
# The menus are dismissed by now, so a bare "Work" row can only be the settings
# tree's own section — which must not be there with no project open.
stray = [n for n in s.nodes()
         if (n.get("label") or "").strip() == "Work" and n.get("role") != "MenuItem"]
if stray:
    s.dump("settings rail")
    fail("the settings tree offers a Work section with no project open "
         f"(roles: {[n.get('role') for n in stray]!r})", s.app, s.mcp, s.log)
print("  app-level sections present; no Work section, as with no project open.")
if not close_settings(s):
    fail("the Settings window would not close", s.app, s.mcp, s.log)

# Ctrl+, — the same command by its chord, registered on this window's own
# shortcut registry.
time.sleep(0.6)
s.key(",", ctrl=True)
open_settings(s, "Ctrl+,")
print("Ctrl+, opens Settings from the Launcher.")
s.shot("09-settings-ctrl-comma.png")
if not close_settings(s):
    fail("the Settings window would not close after Ctrl+,", s.app, s.mcp, s.log)

# ── 7. Quit really quits ─────────────────────────────────────────────────────
# The Launcher's `app.quit` runs the shared `QuitSequencer` now, not a bare
# `close_window()`. With nothing open the queue drains at once and `finish`
# force-closes every window including this one — teksilo exits when its window
# map empties, so "the process is gone" is the only observable proof, and the
# path it takes (through the sequencer) is the one that would also have prompted
# had a project been open.
time.sleep(0.6)
# Reopening the bar after a modal has come and gone is flaky by nature — the
# hamburger toggles, so a stray extra activation closes what the previous one
# opened. Poll for the row instead of assuming one round trip lands it.
quit_row = None
for _ in range(4):
    ham = s.find_exact("Menu", role="Button")
    if ham:
        s.activate(ham)
        time.sleep(0.8)
    work = s.find_exact("Work", role="MenuItem")
    if work:
        s.activate(work)
        time.sleep(1.0)
    quit_row = s.find_exact("Quit", role="MenuItem")
    if quit_row:
        break
    dismiss_menu(s)
if not quit_row:
    s.dump("looking for Quit")
    fail("no Quit row in the Launcher's Work menu", s.app, s.mcp, s.log)
s.activate(quit_row)
deadline = time.time() + 15
while time.time() < deadline and s.app.poll() is None:
    time.sleep(0.3)
if s.app.poll() is None:
    fail("Quit did not terminate the process", s.app, s.mcp, s.log)
print("Quit terminates the process.")

print(f"\nPASS — shots in {SHOTS}")
s.close()
