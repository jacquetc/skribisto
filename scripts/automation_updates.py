#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **update notice**.

Everything here is checked against a sandboxed `XDG_CONFIG_HOME`, so the machine's
real configuration is never read or written.

Flow:

  1. a fresh install, with nothing recorded, says nothing anywhere and does not
     even create `updates.toml` (a source build never checks on its own);
  2. seeded with a newer version, the Launcher sidebar names it, right under the
     running version;
  3. the notice is a link, so it can be acted on rather than merely read;
  4. turning the check off in Settings ▸ Notifications forgets the discovery, so
     turning it back on cannot resurrect it without a new check.

Deliberately **not** covered here: the hand-driven Help ▸ Check for Updates
command. Asserting on it means asserting on a live file at skribisto.eu, and a
repository probe that fails when the network is down, or when a release happens,
is worse than no probe. Its parsing, comparison and failure handling are unit
tested (`crates/teksilo_ui/src/updates/`), against the exact bytes the website
generator produces.

Reuses the launch/bridge/connect scaffolding from `automation_fixture` shared
by the sibling automation_*.py scripts.
"""
import json, os, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# What a seeded `updates.toml` looks like. 999.0.0 is newer than any build this
# repository will ever produce, so the assertion cannot rot with the version.
SEEDED = """version = 1
checked_on = "2026-09-04"
latest_version = "999.0.0"
latest_date = "2026-12-01"

[notes]
en = "https://www.skribisto.eu/news/"
fr = "https://www.skribisto.eu/fr/news/"

[download]
en = "https://www.skribisto.eu/download/"
fr = "https://www.skribisto.eu/fr/download/"
"""

NOTICE = "version 999.0.0 is available"


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
    def __init__(self, extra=(), env=None, pins=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(
            fixture.launch_argv(pins=pins, extra=extra),
            stdout=open(self.log, "w"),
            stderr=subprocess.STDOUT,
            env={**os.environ, **(env or {})},
        )
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=60)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = subprocess.Popen(
            fixture.mcp_argv(bridge, MCP),
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=open(mcp_err, "w"), text=True, bufsize=1,
        )
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "updates-test", "version": "1"}})
        if self._recv(timeout=20, fatal=False) is None:
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

    def nodes(self, window_id=None):
        args = {"window_id": window_id} if window_id is not None else {}
        _, p = self.call("snapshot_tree", args)
        return p.get("nodes", [])

    def texts(self, window_id=None):
        out = []
        for n in self.nodes(window_id):
            out.append(n.get("label") or "")
            out.append(str(n.get("value") or ""))
        return " | ".join(out).lower()

    def wait_text(self, substr, timeout=25, window_id=None):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in self.texts(window_id):
                return True
            time.sleep(0.4)
        return False

    def node_with(self, substr, window_id=None):
        for n in self.nodes(window_id):
            if substr.lower() in (n.get("label") or "").lower():
                return n
        return None

    def key(self, key, **mods):
        """`inject_key` takes its modifiers as plain booleans (`ctrl=True`)."""
        self.call("inject_key", {"key": key, **mods})

    def find_exact(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def activate(self, node):
        self.call("invoke_action", {"node": node["id"], "action": "click"})

    def dialog(self, label, timeout=10):
        """Wait for a separate window carrying this dialog label."""
        end = time.time() + timeout
        while time.time() < end:
            for n in self.nodes():
                if n.get("role") == "Dialog" and (n.get("label") or "").strip() == label:
                    return n
            time.sleep(0.4)
        return None

    def click(self, substr, window_id=None):
        node = self.node_with(substr, window_id)
        if node is None:
            fail(f"no node labelled like {substr!r}", self.app, self.mcp, self.log)
        self.activate(node)
        return node

    def switches(self):
        """Every toggle in the tree. The update row's own label is empty because
        it is `labelled_externally` (the `FormLayout` line carries the words), so
        it can only be found by role."""
        return [n for n in self.nodes() if n.get("role") == "Switch"]

    def stop(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
                try:
                    p.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    p.kill()


def sandbox(seeded, label):
    """An env for one run, the config dir inside it, and a `--config` pins file.

    `isolated_config` returns an environment dict, not a path, and pins the
    locale: a probe asserting on English text must set the language rather than
    inherit the operator's. The same keys go through `config_pins_file` so the
    launch also validates them — a typo'd key is a startup error, not a probe
    quietly running on defaults. `label` keeps three runs in one process from
    sharing one scratch directory.
    """
    env = fixture.isolated_config(locale="en-US", label=label)
    pins = fixture.config_pins_file(
        {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": True},
        label=label)
    cfg = os.path.join(env["XDG_CONFIG_HOME"], "skribisto")
    os.makedirs(cfg, exist_ok=True)
    if seeded:
        with open(os.path.join(cfg, "updates.toml"), "w", encoding="utf-8") as f:
            f.write(SEEDED)
    return env, cfg, pins


def step_1_clean_install_says_nothing():
    env, cfg, pins = sandbox(seeded=False, label="clean")
    s = Session(env=env, pins=pins)
    try:
        # Give any check that was going to happen every chance to happen.
        time.sleep(3)
        if NOTICE in s.texts():
            fail("a clean install must not claim a newer version exists", s.app, s.mcp, s.log)
        if os.path.exists(os.path.join(cfg, "updates.toml")):
            fail(
                "a source build wrote updates.toml, so it made a request it must not make",
                s.app, s.mcp, s.log,
            )
        print("  1. clean install: silent, and no request made")
    finally:
        s.stop()


def step_2_to_4_seeded():
    env, cfg, pins = sandbox(seeded=True, label="seeded")
    s = Session(env=env, pins=pins)
    try:
        if not s.wait_text(NOTICE):
            fail("the Launcher sidebar does not name the newer version", s.app, s.mcp, s.log)
        print("  2. Launcher sidebar names the newer version")

        # The line has to be a link, not decoration.
        node = s.node_with(NOTICE)
        if (node.get("role") or "").lower() not in ("link", "button"):
            fail(f"the notice is a {node.get('role')!r}, not something to click",
                 s.app, s.mcp, s.log)
        print(f"  3. it is a {node.get('role')}, so it can be acted on")

        # Turning the check off has to clear the view AND forget the discovery.
        #
        # Reached through the menu rather than Ctrl+,: `app.settings` is a named
        # global action, not an `AppIntent` variant, and the chord only answers
        # once this window has had keyboard focus. The menu row is the door a
        # reader actually uses, and it is registered by `WelcomePanel::build` on
        # this window's own tree because the Launcher never builds an `App`.
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
            s.key("Escape")
            time.sleep(0.4)
        if settings_row is None:
            fail("no Settings row in the Launcher's Work menu", s.app, s.mcp, s.log)
        s.activate(settings_row)
        if s.dialog("Settings") is None:
            fail("the Settings row opened no window", s.app, s.mcp, s.log)

        if not s.wait_text("notifications", timeout=10):
            fail("Settings has no Notifications page", s.app, s.mcp, s.log)
        s.click("notifications")
        time.sleep(1.2)
        if not s.wait_text("check for new versions", timeout=10):
            fail("Settings > Notifications has no update-check row", s.app, s.mcp, s.log)
        if "updates" not in s.texts():
            fail("the update row has no group heading", s.app, s.mcp, s.log)
        toggles = s.switches()
        if len(toggles) != 1:
            fail(f"expected exactly one toggle on this page, found {len(toggles)}",
                 s.app, s.mcp, s.log)
        s.activate(toggles[0])
        time.sleep(1.5)
        recorded = open(os.path.join(cfg, "updates.toml"), encoding="utf-8").read()
        if "999.0.0" in recorded:
            fail("turning the check off left the discovery in updates.toml",
                 s.app, s.mcp, s.log)
        print("  4. turning it off forgot the discovery, not just the view")
    finally:
        s.stop()


def main():
    fixture.assert_no_running_instance()
    print("update notice:")
    step_1_clean_install_says_nothing()
    step_2_to_4_seeded()
    print("PASS")


if __name__ == "__main__":
    main()
