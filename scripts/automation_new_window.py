#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Work ▸ New Window: a SECOND window onto the SAME open project.

Two windows on one `Work` is the shape the multi-Work migration built toward
(`WorkRegistry::attach`, the per-window `ordinal`, `attached_window_id_for`) and
that `PendingAction::AttachExisting` finally reaches. What makes it worth a live
probe rather than only unit tests is that every interesting claim is about state
*shared across two widget trees* — which no headless `WidgetTree` can hold.

What this asserts, in order:

  1. **A second window opens, and the first stays.** `list_windows` goes 1 → 2.
     (`open_window` *panics* without a live `WindowOps`, so a regression on this
     path takes the app down rather than degrading — see `Session.activate`.)
  2. **The two windows are distinguishable.** Their titles differ, and the newer
     one carries the " (Window 2)" suffix `window_title_text` builds from the
     ordinal. This is not cosmetic: on Wayland a client cannot place its own
     toplevel, so pinning one of them to a second monitor is done with a KWin
     rule matched on the title text.
  3. **They share one `Work`.** An item created in window 1 appears in window
     2's binder — one store, reached through one `WorkSession`, with the
     backend's entity events reaching both windows' subscribers. Not two copies
     of one file racing each other onto one path.

     Both windows' **save indicators** follow it too, which is the stricter
     half: `unsaved` is a plain `Signal<bool>` on the shared `WorkSession` with
     no backend event behind it, so it pins that a shared *signal* reaches every
     window — the binder rows above would still cross even if signals did not.
     (Editor *prose* is still not assertable here: a `RichTextEditor` exposes no
     `value` in the AT tree.)
  4. **Closing a *window* closes only that window.** With unsaved edits and a
     sibling still showing the Work, the title-bar close must not prompt, must
     not call the backend `close_work`, and must not return to the Launcher:
     `list_windows` goes 2 → 1 and the survivor still shows the project. (Before
     the sibling-aware branch in `shell::windows`'s `on_close_requested`, this
     path ran the full exit guard and tore the project out from under the
     surviving window.)
  5. **Closing the *Work* accounts for it once.** File ▸ Close Work asks about
     the unsaved edits exactly once — for the project, not once per window that
     shows it — and then leaves for the Launcher.

Driven through the **menu**, so the menu entry itself is under test, and the
menu bar specifically through `invoke_action` — see `Session.activate`. The item
fires the very same global `window.new` action Ctrl+Shift+N does.

Runs against a throwaway copy of the bundled example in an isolated XDG sandbox
(see `automation_fixture`'s module doc for why a probe never opens a checked-in
fixture), so autosave and format migration cannot touch the repo.
"""

import json, os, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

MARKER = "SHARED-BY-BOTH-WINDOWS"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
sandbox = tempfile.mkdtemp(prefix="skribisto_new_window_")
SANDBOX_ENV = {
    "XDG_CONFIG_HOME": os.path.join(sandbox, "config"),
    "XDG_DATA_HOME": os.path.join(sandbox, "data"),
    "HOME": sandbox,
}
# Every label this probe matches is written in English, so the language has to be
# SET rather than inherited: an unset `ui.locale` is not "English", it is the
# operator's OS language (`startup.rs`'s `auto_detect_os_locale`). Without this
# the probe passed on an English desktop and failed on a French one, reporting
# "the 'Work' menu did not open" — it had opened, as `Œuvre`.
fixture.write_settings(SANDBOX_ENV["XDG_CONFIG_HOME"])
# The same pins again, through `--config` this time: `write_settings` above
# writes `general.toml` directly and nothing validates it, so a typo'd key would
# silently run on defaults. `--config` makes the app check every key against its
# schema instead (and implies `--new-instance`, passed explicitly below anyway).
PINS = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": True},
    label="new-window")
WORK = os.path.join(sandbox, "Starforgers.skrib")
shutil.copyfile(EXAMPLE, WORK)


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

    def __init__(self, args, pins=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        env = {**os.environ, **SANDBOX_ENV}
        self.app = subprocess.Popen(fixture.launch_argv(list(args), pins=pins),
                                    stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=25)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = None
        # `wait_for_bridge` already proved the endpoint is bound; still retry the
        # handshake once, since the server-side accept can lag its own bind by a
        # beat and the first connect attempt lands a hair too early.
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "new-window", "version": "1"}})
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

    def nodes(self, window_id=None):
        args = {"window_id": window_id} if window_id is not None else {}
        _, p = self.call("snapshot_tree", args)
        return p.get("nodes", []) if isinstance(p, dict) else []

    def labels(self, window_id=None):
        return [n.get("label") for n in self.nodes(window_id) if n.get("label")]

    def list_windows(self):
        _, p = self.call("list_windows")
        return p if isinstance(p, list) else []

    def wait_window_count(self, want, timeout=20):
        end = time.time() + timeout
        seen = None
        while time.time() < end:
            seen = self.list_windows()
            if len(seen) == want:
                return seen
            time.sleep(0.3)
        return seen

    def wait_label(self, substr, timeout=30, window_id=None):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(l for l in self.labels(window_id)).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def find_exact(self, label, role=None, window_id=None):
        for n in self.nodes(window_id):
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def click(self, node, window_id=None):
        """A synthetic pointer press on `node` — how a user drives most of the
        UI, and what the binder rows and dialog buttons here respond to."""
        b = node.get("bounds") or {}
        if "x" not in b:
            return False
        args = {"x": b["x"] + b.get("width", 0) / 2,
                "y": b["y"] + b.get("height", 0) / 2, "action": "click"}
        if window_id is not None:
            args["window_id"] = window_id
        self.call("inject_pointer", args)
        return True

    def activate(self, node, window_id=None):
        """Fire `node`'s **AccessKit action** instead of a synthetic click.

        The menu bar is driven this way because a synthetic press+release at a
        top-level entry's centre does not open its submenu into the AT tree —
        the items simply are not there to click — while `invoke_action` opens it
        reliably. Everything else in this probe uses ordinary pointer clicks.
        """
        args = {"node": node["id"], "action": "Click"}
        if window_id is not None:
            args["window_id"] = window_id
        res, _ = self.call("invoke_action", args)
        return not (isinstance(res, dict) and res.get("isError"))

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


def window_title(w):
    """`list_windows` rows name their title under one of a couple of keys
    depending on the bridge version — take whichever is present rather than
    hard-coding one and silently comparing `None` to `None`."""
    for key in ("title", "caption", "name"):
        if isinstance(w, dict) and w.get(key):
            return w[key]
    return ""


def window_key(w):
    for key in ("id", "window_id", "string_id"):
        if isinstance(w, dict) and w.get(key) is not None:
            return w[key]
    return None


def menu_item(s, substr, timeout=8, window_id=None):
    deadline = time.time() + timeout
    while time.time() < deadline:
        for n in s.nodes(window_id):
            if n.get("role") == "MenuItem" and substr in (n.get("label") or "").lower():
                return n
        time.sleep(0.3)
    return None


def open_work_menu(s, window_id=None):
    """Title-bar hamburger → the Work menu. Returns once its items are up."""
    ham = s.find_exact("Menu", role="Button", window_id=window_id)
    if not ham:
        fail("no title-bar 'Menu' (hamburger) button in the AT tree", s.app, s.mcp, s.log)
    s.activate(ham, window_id)
    # Polled, not slept at: the bar is built when the button fires, and how long
    # that takes varies with what the window is doing (a fixed wait made this the
    # probe's flakiest line).
    work = None
    deadline = time.time() + 8
    while time.time() < deadline and not work:
        work = s.find_exact("Work", role="MenuItem", window_id=window_id)
        if not work:
            time.sleep(0.3)
    if not work:
        fail("the 'Work' menu did not open", s.app, s.mcp, s.log)
    s.activate(work, window_id)
    time.sleep(1.2)


def scene_editors(s, window_id=None):
    return [n for n in s.nodes(window_id) if n.get("role") == "MultilineTextInput"]


def open_prologue(s, window_id=None):
    """Click the 'Prologue' binder row and return this window's scene editor."""
    if not s.wait_label("prologue", timeout=30, window_id=window_id):
        fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
    row = next((n for n in s.nodes(window_id)
                if "prologue" in (n.get("label") or "").lower()
                and (n.get("bounds") or {}).get("width")), None)
    if not row:
        fail("no clickable 'Prologue' row", s.app, s.mcp, s.log)
    s.click(row, window_id)
    time.sleep(1.2)
    editors = scene_editors(s, window_id)
    if not editors:
        fail("no editor after opening Prologue", s.app, s.mcp, s.log)
    # The tallest input is the scene body; the shorter one is the synopsis.
    return max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))


# ── 0. One window, one project ────────────────────────────────────────────────
s = Session([WORK], pins=PINS)
if not s.wait_label("prologue", timeout=40):
    fail("the project never finished loading", s.app, s.mcp, s.log)

before = s.wait_window_count(1)
if len(before) != 1:
    fail(f"expected exactly one window before New Window, saw {before}", s.app, s.mcp, s.log)
first_key = window_key(before[0])
print(f"1 window: {window_title(before[0])!r}")

# ── 1. Work ▸ New Window ──────────────────────────────────────────────────────
open_work_menu(s)
item = menu_item(s, "window")
if not item:
    labels = [n.get("label") for n in s.nodes() if n.get("role") == "MenuItem"]
    fail(f"no 'New Window' item in the Work menu; items: {labels}", s.app, s.mcp, s.log)
print(f"activating {item.get('label')!r}")
s.activate(item)

after = s.wait_window_count(2, timeout=30)
if len(after) != 2:
    fail(f"Work ▸ New Window did not open a second window (saw {after})", s.app, s.mcp, s.log)
if s.app.poll() is not None:
    fail("the app exited while opening the second window", s.app, s.mcp, s.log)
print(f"2 windows: {[window_title(w) for w in after]}")

# ── 2. The two windows are distinguishable ───────────────────────────────────
titles = [window_title(w) for w in after]
if len(set(titles)) != 2:
    fail(f"both windows carry the same title {titles!r} — a KWin rule could not "
         "tell them apart, which is what the ordinal suffix exists for",
         s.app, s.mcp, s.log)
if not any("(Window 2)" in t for t in titles):
    fail(f"no window carries the ' (Window 2)' ordinal suffix; titles: {titles!r}",
         s.app, s.mcp, s.log)
second_key = next(window_key(w) for w in after if window_key(w) != first_key)
print("titles differ and the second carries the ordinal suffix")

# ── 3. One Work, two windows ─────────────────────────────────────────────────
# A structural edit made in window 1 must appear in window 2's binder. Both
# windows read one store through one `WorkSession` (`WorkRegistry::attach` hands
# the second window the *same* session rather than minting one), and the backend
# publishes its entity events to every window's subscribers — so window 2's own
# outline reloads without anyone telling it to.
#
# Two independently-loaded copies of one file would each have their own store
# subtree and window 2's binder would not move. That is the failure this rules
# out, and it is why New Window is not "open the same path again".


# The save indicator's label is chrome, not a binder row, so it is excluded from
# the row comparison — and asserted separately, right below, because it is its
# own claim: BOTH windows' indicators must follow the edit.
CHROME_NOISE = ("saved", "unsaved")


def binder_labels(window_id):
    return sorted(l for n in s.nodes(window_id)
                  if (l := n.get("label"))
                  and not any(w in l.lower() for w in CHROME_NOISE))


def save_indicator(window_id):
    """This window's save-indicator label, lowercased ('' if absent)."""
    for n in s.nodes(window_id):
        label = (n.get("label") or "").lower()
        if any(w in label for w in CHROME_NOISE):
            return label
    return ""


before_2 = binder_labels(second_key)
indicator_2_before = save_indicator(second_key)
create = next((n for n in s.nodes(first_key)
               if n.get("role") == "Button" and (n.get("label") or "").strip() == "Book"), None)
if not create:
    fail("no binder create button in the first window", s.app, s.mcp, s.log)
s.click(create, first_key)
time.sleep(3.0)

after_1, after_2 = binder_labels(first_key), binder_labels(second_key)
gained_1 = [l for l in after_1 if l not in before_2]
gained_2 = [l for l in after_2 if l not in before_2]
if not gained_1:
    fail("creating a Book in the first window changed nothing there either — the "
         "probe's own precondition failed", s.app, s.mcp, s.log)
# Compared as a subset, not for equality: the acting window also gains chrome
# labels of its own (its save indicator flips to "Unsaved changes…", which the
# sibling's deliberately does not — see the note in this module's docstring), and
# those are not binder rows. What must hold is that every row window 2 gained is
# one window 1 gained too, and that it gained something at all.
if not gained_2:
    fail("an item created in the first window did not appear in the second's binder "
         "at all — the two windows are not reading one store, which is the whole "
         "point of New Window", s.app, s.mcp, s.log)
if not set(gained_2) <= set(gained_1):
    fail(f"the second window gained rows the first did not: "
         f"{sorted(set(gained_2) - set(gained_1))}", s.app, s.mcp, s.log)
print(f"an item created in window 1 appeared in window 2's binder: {gained_2} — one shared Work")

# ...and so must the chrome that reads purely shared state. `unsaved` is a plain
# `Signal<bool>` on the shared `WorkSession` — no backend event behind it — bound
# by `SaveIndicator` at `BindingLevel::Rebuild`. It is therefore the strictest
# available check that a shared *signal* (not merely a shared store reached via
# entity events) reaches every window: the binder rows above would still cross
# even if signals did not.
indicator_1, indicator_2 = save_indicator(first_key), save_indicator(second_key)
if "unsaved" not in indicator_1:
    fail(f"window 1's own save indicator did not follow its own edit "
         f"({indicator_1!r}) — the probe's precondition failed", s.app, s.mcp, s.log)
if "unsaved" not in indicator_2:
    fail(f"window 2's save indicator still reads {indicator_2!r} (was "
         f"{indicator_2_before!r}) after window 1 dirtied the shared Work — a "
         f"shared Signal must reach EVERY window's tree, not just whichever one "
         f"reconciles first", s.app, s.mcp, s.log)
print(f"both windows' save indicators followed the shared edit: "
      f"{indicator_1!r} / {indicator_2!r}")

# ── 4. Closing a WINDOW closes only that window ──────────────────────────────
# The Work is dirty (the Book we just created is unsaved) and a sibling still
# shows it, so the title-bar close must be silent: no unsaved-changes prompt, no
# backend `close_work`, no Launcher. Before the sibling-aware branch in
# `shell::windows`'s `on_close_requested`, this path ran the full exit guard and
# tore the project out from under the surviving window.
close_btn = s.find_exact("Close", role="Button", window_id=second_key)
if not close_btn:
    fail("no title-bar Close button in the second window", s.app, s.mcp, s.log)
s.click(close_btn, second_key)
time.sleep(2.0)

prompt = next((n for n in s.nodes()
               if n.get("role") == "Button"
               and (n.get("label") or "").strip().lower() in ("discard", "ignorer")), None)
if prompt:
    fail("closing ONE window of a Work still shown by a sibling raised the "
         "unsaved-changes guard — nothing is going away, so there is nothing to "
         "guard, and answering it would close a project the other window is "
         "still displaying", s.app, s.mcp, s.log)

left = s.wait_window_count(1, timeout=20)
if len(left) != 1:
    fail(f"closing one window left {len(left)} windows: {left}", s.app, s.mcp, s.log)
survivor = window_key(left[0])
if survivor != first_key:
    fail(f"the wrong window survived: {left}", s.app, s.mcp, s.log)
if "Starforgers" not in window_title(left[0]):
    fail(f"the surviving window no longer shows the project: {window_title(left[0])!r} "
         "— closing a sibling must not close the Work", s.app, s.mcp, s.log)
if not s.wait_label("prologue", timeout=10, window_id=first_key):
    fail("the surviving window's binder is empty — the Work was torn down with "
         "its sibling", s.app, s.mcp, s.log)
print(f"one window closed silently; {window_title(left[0])!r} still holds the project")

# ── 5. Closing the WORK accounts for it once ─────────────────────────────────
open_work_menu(s, first_key)
close_item = menu_item(s, "close", window_id=first_key)
if not close_item:
    fail("no 'Close Work' item in the Work menu", s.app, s.mcp, s.log)
s.activate(close_item, first_key)
time.sleep(1.5)

# `Close Work` means the Work — so it takes BOTH windows and returns to the
# Launcher, after asking about the unsaved edits exactly once (not once per
# window). Assert the guard appeared, then discard.
discard = None
deadline = time.time() + 10
while time.time() < deadline:
    discard = next((n for n in s.nodes()
                    if n.get("role") == "Button"
                    and (n.get("label") or "").strip().lower() in ("discard", "ignorer")), None)
    if discard:
        break
    time.sleep(0.4)
if not discard:
    # Name what IS on screen: "no Discard button" reads as "the guard is missing",
    # but the same symptom is produced by a guard that never got as far as being
    # asked for (a menu click swallowed by a stale overlay, say).
    on_screen = [n.get("label") for n in s.nodes() if n.get("role") == "Button"]
    fail("Close Work over unsaved edits did not raise the guard — it must never "
         f"discard a writer's edits without asking (buttons on screen: {on_screen})",
         s.app, s.mcp, s.log)
prompts = [n for n in s.nodes()
           if n.get("role") == "Button"
           and (n.get("label") or "").strip().lower() in ("discard", "ignorer")]
if len(prompts) != 1:
    fail(f"the unsaved-changes guard appeared {len(prompts)} times for one Work — "
         "two windows must not each ask about the same project", s.app, s.mcp, s.log)
print("Close Work asked once, for the Work, not once per window")
s.activate(discard)

left = s.wait_window_count(1, timeout=20)
if len(left) != 1:
    fail(f"closing the Work left {len(left)} windows, expected the Launcher alone: {left}",
         s.app, s.mcp, s.log)
if s.app.poll() is not None:
    fail("the app exited instead of returning to the Launcher", s.app, s.mcp, s.log)
print(f"the project window closed, {window_title(left[0])!r} remains — and the app is alive")

print("\nPASS: Work ▸ New Window opens a distinguishable second window on the same "
      "project, sharing one live document, and Close Work accounts for the Work once.")
s.close()
shutil.rmtree(sandbox, ignore_errors=True)
