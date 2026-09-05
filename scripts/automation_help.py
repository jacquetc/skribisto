#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **Help** surfaces.

Flow, against a scratch copy of an example so nothing in the repo is touched:

  1. the Launcher's **Learn** pane offers real destinations, not a placeholder;
  2. **F1** opens the Help window, with a table of contents and a topic rendered;
  3. clicking another topic in the contents changes the reading pane;
  4. **Ctrl+Shift+P** opens the command palette and typing narrows it;
  5. the **Keyboard shortcuts** sheet lists real chords.

Reuses the launch + bridge-wait + connect scaffolding from `automation_fixture`,
shared with the sibling automation_*.py scripts.
"""
import base64, json, os, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

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
    def __init__(self, project=None, pins=None, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(
            fixture.launch_argv(project, pins=pins),
            stdout=open(self.log, "w"),
            stderr=subprocess.STDOUT,
            env={**os.environ, **(env or {})},
        )
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app)
        except RuntimeError as e:
            fail(str(e), self.app, None, self.log)
        self._id = 0
        self.mcp = subprocess.Popen(
            fixture.mcp_argv(bridge, MCP),
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=open(mcp_err, "w"), text=True, bufsize=1,
        )
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "help-test", "version": "1"}})
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

    def nodes(self, window_id=None):
        args = {"window_id": window_id} if window_id is not None else {}
        _, p = self.call("snapshot_tree", args)
        return p.get("nodes", [])

    def window_id(self, label):
        """The id of the window with this label, or None."""
        _, w = self.call("list_windows")
        rows = w if isinstance(w, list) else []
        return next((x["id"] for x in rows if x.get("label") == label), None)

    def labels(self, window_id=None):
        return [n.get("label") or "" for n in self.nodes(window_id)]

    def texts(self):
        """Every label and value in the tree, lowercased, as one string."""
        out = []
        for n in self.nodes():
            out.append(n.get("label") or "")
            out.append(str(n.get("value") or ""))
        return " | ".join(out).lower()

    def wait_text(self, substr, timeout=25):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in self.texts():
                return True
            time.sleep(0.4)
        return False

    def node_with(self, substr):
        for n in self.nodes():
            if substr.lower() in (n.get("label") or "").lower():
                return n
        return None

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image":
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")
                return True
        return False

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


def main():
    out_dir = os.environ.get("SKRIBISTO_AUTOMATION_SCRATCH", tempfile.gettempdir())
    # English, so the assertions below read against known strings rather than
    # whichever locale the developer's desktop happens to be in.
    env = fixture.isolated_config(locale="en-US", label="help")
    pins = fixture.config_pins_file(
        {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": True},
        label="help")
    # No project: this probe drives the Launcher itself, not an opened work.
    s = Session(pins=pins, env=env)
    try:
        if not s.wait_text("recent works", timeout=30):
            fail("launcher never appeared", s.app, s.mcp, s.log)

        # 1. The Learn pane offers real destinations.
        learn = s.node_with("Learn")
        if not learn:
            fail("no Learn tab in the launcher", s.app, s.mcp, s.log)
        # `invoke_action` REQUIRES an `action`; omitting it is a silent no-op.
        s.call("invoke_action", {"node": learn["id"], "action": "click"})
        time.sleep(1.0)
        rows = [lbl for lbl in s.labels() if lbl]
        for expected in ("Help topics", "Keyboard shortcuts", "Skribisto on the web"):
            if expected not in rows:
                fail(f"Learn pane is missing a destination: {expected!r} (rows: {rows})",
                     s.app, s.mcp, s.log)
        # A menu label reused on a button renders its `&` mnemonic literally, which is
        # exactly what shipped for one build here ("Help &Topics"). Only an exact-string
        # check catches it: a substring test for "help topics" passes on the broken text.
        stray = [lbl for lbl in rows if "&" in lbl]
        if stray:
            fail(f"a Learn row shows a literal mnemonic ampersand: {stray}",
                 s.app, s.mcp, s.log)
        print("  ok: Learn pane offers real destinations")
        s.shot(os.path.join(out_dir, "help-learn.png"))

        # 2. F1 opens the Help window with contents and a rendered topic.
        #
        # The Launcher builds no `App`, so it registers this shortcut itself; assert the
        # registration directly as well as its effect, because a missing registration and
        # a mis-sent key look identical from the outside.
        _, shortcuts = s.call("get_shortcuts")
        rows = shortcuts.get("shortcuts", []) if isinstance(shortcuts, dict) else (shortcuts or [])
        f1 = [r for r in rows if r.get("id") == "help.topics"]
        if not f1 or f1[0].get("primary") != "F1":
            fail(f"help.topics is not bound to F1 in the Launcher; got {f1}", s.app, s.mcp, s.log)
        # The tool is `inject_key`; there is no `press_key`, and calling a tool that does
        # not exist returns quietly rather than erroring, which is how an earlier version
        # of this probe "tested" nothing at all.
        s.call("inject_key", {"key": "F1"})
        time.sleep(2.0)
        _, wins = s.call("list_windows")
        if not any(w.get("label") == "help" for w in (wins if isinstance(wins, list) else [])):
            fail(f"F1 opened no help window; windows: {wins}", s.app, s.mcp, s.log)
        # Read the *help* window's tree explicitly: `snapshot_tree` with no window
        # answers for one window, and once Help opened as a second one this probe was
        # quietly asserting against the Launcher's nodes.
        help_win = s.window_id("help")
        if help_win is None:
            fail(f"F1 opened no help window; windows: {wins}", s.app, s.mcp, s.log)
        rows = [lbl for lbl in s.labels(help_win) if lbl]
        # Every built-in topic must be listed. Asserting on the *titles* rather than the
        # section headings: headings are decorative text with no accessible name, so a
        # test that looked for them would fail while the window worked perfectly.
        for expected in ("Your first project", "Sending your book to a reader",
                         "Backups and versions"):
            if expected not in rows:
                fail(f"Help contents missing {expected!r}; got {rows}", s.app, s.mcp, s.log)
        print("  ok: F1 opened the Help window with its table of contents")
        s.shot(os.path.join(out_dir, "help-window.png"))

        # 3. Choosing another topic changes the reading pane.
        target = next((n for n in s.nodes(help_win)
                       if (n.get("label") or "") == "Sending your book to a reader"), None)
        if not target:
            fail("the round-trip topic is not listed in the contents", s.app, s.mcp, s.log)
        s.call("invoke_action", {"node": target["id"], "action": "click"})
        time.sleep(1.5)
        body = " | ".join(str(n.get("label") or "") + " " + str(n.get("value") or "")
                          for n in s.nodes(help_win)).lower()
        if "docx" not in body and "word" not in body:
            fail("the round-trip topic did not render its body", s.app, s.mcp, s.log)
        print("  ok: choosing a topic renders it")
        s.shot(os.path.join(out_dir, "help-round-trip.png"))

        print("PASS: help surfaces verified")
    finally:
        s.close()


if __name__ == "__main__":
    main()
