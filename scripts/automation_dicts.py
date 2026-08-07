#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the worktree build to verify the Dictionaries + Work-Language settings panes
and the per-item language field render. Reuses the launch/connect scaffolding shape of
the sibling automation_*.py scripts, pointed at the feature worktree binary."""
import base64, json, os, re, select, subprocess, sys, tempfile, time

import pathlib
_ROOT = pathlib.Path(__file__).resolve().parent.parent  # this repo/worktree root
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
    def __init__(self, args):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT)
        sock = tok = None
        deadline = time.time() + 20
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
            fail("no bridge socket within 20s", self.app, None, self.log)
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
                                      "clientInfo": {"name": "dicts-test", "version": "1"}})
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

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in " | ".join(self.labels()).lower():
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
            self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                         "y": b["y"] + b.get("height", 0) / 2, "kind": "click"})
            return True
        return self.call("invoke_action", {"node": n["id"], "action": "click"}) is not None

    def dump(self, title, limit=80):
        print(f"--- AT tree: {title} ---")
        c = 0
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            if lab:
                print(f"  [{n.get('role')}] {lab!r}")
                c += 1
                if c >= limit:
                    print("  …"); break

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


def has_any(*variants):
    j = " | ".join(s.labels()).lower()
    return any(v in j for v in variants)


def settings_open():
    return has_any("reset to defaults", "réinitialiser") and has_any("done", "terminé")


def expand_and_click(section_variants, leaf_variants):
    """Expand a tree section (click its chevron at the leading edge), then click a leaf."""
    sec = s.match(section_variants, rail_only=True)
    if sec:
        b = sec.get("bounds") or {}
        if isinstance(b, dict) and "x" in b:  # chevron ~10px in
            s.call("inject_pointer", {"x": b["x"] + 10, "y": b["y"] + b.get("height", 0) / 2,
                                      "kind": "click"})
            time.sleep(0.5)
    leaf = s.match(leaf_variants, rail_only=True, exact=True) or s.match(leaf_variants, rail_only=True)
    if leaf:
        s.click_node(leaf)
        time.sleep(0.8)
        return True
    return False


print("== launch with example ==")
s = Session([EXAMPLE])
if not s.wait_label("starforgers", timeout=15):
    fail("example did not load", s.app, s.mcp, s.log)
print("example loaded.")

# open Settings (try key variants until the always-present footer appears)
for args in ({"key": ",", "ctrl": True}, {"key": ",", "modifiers": ["ctrl"]},
             {"key": "Comma", "modifiers": ["ctrl"]}):
    s.call("inject_key", args)
    time.sleep(0.8)
    if settings_open():
        break
if not settings_open():
    s.dump("no settings")
    fail("could not open Settings", s.app, s.mcp, s.log)
print("settings opened.")

# LEAK CHECK: with Settings on the DEFAULT pane (Work-Language pane inactive), is any pill
# field leaking to the window's top-left (x < 60)?
leaked = [n for n in s.nodes()
          if (n.get("label") or "").strip() and n.get("role") in ("List", "ListItem")
          and "language" in (n.get("label") or "").lower()
          and isinstance(n.get("bounds"), dict) and n["bounds"].get("x", 999) < 60
          and n["bounds"].get("y", 999) < 60]
print(f"=== LEAK CHECK (inactive Work-Language pane): {len(leaked)} leaked pill node(s) at top-left ===")
for n in leaked:
    print(f"  LEAK [{n.get('role')}] {n.get('label')!r} bounds={n.get('bounds')}")
s.shot("/tmp/settings-default-noleaK.png")

# Navigate to Work ▸ Language robustly (invoke_action on the leaf node), then tab to focus a
# pill to verify the focus ring.
lang = s.match(["language", "langue"], rail_only=True, exact=True)
if lang:
    s.click_node(lang)  # inject_pointer — reliably drives the tree selection → pane switch
    time.sleep(0.9)
    s.shot("/tmp/work-language-active.png")
    # KEYBOARD focus the pill (Tab) so the `:focus-visible` ring shows, and capture each step
    # so we can see when the pill takes focus.
    for i in range(8):
        s.call("inject_key", {"key": "Tab"})
        time.sleep(0.3)
        foc = [n for n in s.nodes() if n.get("focused") and (n.get("label") or "").strip()]
        if foc:
            print(f"  tab {i}: focused = {[n.get('label') for n in foc]}")
        if any("turn spell" in (n.get("label") or "").lower() for n in foc):
            s.shot("/tmp/work-language-focused.png")
            print("  -> pill focused, ring screenshot taken")
            break
    print("captured work-language active + focused")

# Spelling ▸ Dictionaries
expand_and_click(["spelling", "orthographe"], ["dictionaries", "dictionnaires"])
s.dump("Dictionaries pane")
s.shot("/tmp/dicts-pane.png")
joined = " | ".join(s.labels()).lower()
print("=== Dictionaries checks ===")
for needle in ("installed", "get more", "installés", "english", "français", "download", "télécharger"):
    print(f"  {'OK ' if needle in joined else 'MISS'} '{needle}'")

# click the "Get more" tab
gm = s.match(["get more", "en obtenir"])
if gm:
    s.click_node(gm)
    time.sleep(0.8)
    s.shot("/tmp/dicts-getmore.png")
    j = " | ".join(s.labels()).lower()
    print("=== Get more checks ===")
    for needle in ("download", "télécharger", "view licence", "voir la licence", "français", "italiano"):
        print(f"  {'OK ' if needle in j else 'MISS'} '{needle}'")

# Work: <name> ▸ Language — the pill field in a controlled settings container
if expand_and_click(["work:", "œuvre :", "oeuvre"], ["language", "langue"]):
    time.sleep(0.8)
    print("=== Work Language pane: pill field bounds ===")
    for n in s.nodes():
        lab = (n.get("label") or "").strip()
        if lab and n.get("role") in ("List", "ListItem", "Button") and (
            "language" in lab.lower() or "spell" in lab.lower() or "add" in lab.lower()):
            print(f"  [{n.get('role')}] {lab!r} bounds={n.get('bounds')}")
    s.shot("/tmp/work-language-pane.png")

# close settings via Done, open a chapter (single-click opens it → the Inspector shows its
# language pill field), screenshot, then type a misspelling to check squiggles.
done = s.match(["done", "terminé"], exact=True)
if done:
    s.click_node(done)
    time.sleep(0.8)
ch = s.match(["chapter 1", "chapitre 1"])
if ch:
    s.click_node(ch)  # ActivateOn::SingleClick opens the editor tab
    time.sleep(1.5)
    print("=== inspector after opening Chapter 1 ===")
    for n in s.nodes():
        lab = (n.get("label") or "").strip()
        if lab and (n.get("role") in ("List", "ListItem", "Button") or "language" in lab.lower()
                    or "spell" in lab.lower()):
            b = n.get("bounds") or {}
            print(f"  [{n.get('role')}] {lab!r} bounds={b}")
    s.shot("/tmp/inspector-pill.png")
    s.call("inject_text", {"text": " thiss worrd missspelled "})
    time.sleep(1.2)
    s.shot("/tmp/editor-squiggles.png")
    print("screenshots taken")

s.close()
print("done.")
