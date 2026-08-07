#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""A popover must not outlive the focus that left it.

    scripts/automation_popover_focus_out.py resources/examples/Starforgers.skrib

Skribisto used to wrap all 22 of its popover bodies in
`FocusScope(TraversalScopePolicy::Cycle)`, enforced by a source-scanning lint,
because bastyde let Tab walk straight out of an open popover and leave the panel
hanging over the focus ring behind it — WCAG 2.2 SC 2.4.11, Focus Not Obscured.

Trapping was the wrong answer: a popover implements the Disclosure pattern,
which mandates *no* focus containment. bastyde now dismisses any non-modal
overlay the keyboard walks out of, so the wraps are gone. This drives the real
app to prove the framework half actually reaches the app half — headless tests
pin the framework, but only the live binary proves the popovers in *this* UI now
close when tabbed away from.

Asserts, against a live app with a project open:

  1. a popover trigger opens a panel (nodes appear that were not there);
  2. focus moves into it;
  3. Tab eventually carries focus back out of the panel;
  4. and the panel is *gone* by then — not merely unfocused.

Step 4 is the whole test. Before this change it stayed open.
"""

import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import isolated_config, working_copy

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = os.path.join(HERE, "target", "debug", "skribisto")
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"

VERBOSE = os.environ.get("VERBOSE") == "1"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


def die(msg, sess=None):
    print(f"FAIL: {msg}")
    if sess:
        try:
            print("--- app log tail ---")
            print("".join(open(sess.log).readlines()[-30:]))
        except Exception:
            pass
        sess.stop()
    sys.exit(1)


class Session:
    def __init__(self, path, env):
        self.mcp = None
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(
            [SKRIBISTO, "--new-instance", path],
            env=env, stdout=open(self.log, "w"), stderr=subprocess.STDOUT,
        )
        sock = tok = None
        # Debug `load_work` on the bundled example is slow — CLAUDE.md measures
        # ~20 s to the first open-registry claim. Budget well past that.
        deadline = time.time() + 90
        while time.time() < deadline:
            txt = open(self.log).read()
            a = re.search(r"bridge socket = (\S+)", txt)
            b = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
            if a and b:
                sock, tok = a.group(1), b.group(1)
                break
            if self.app.poll() is not None:
                die("app exited before printing the bridge socket", self)
            time.sleep(0.2)
        if not sock:
            die("no bridge socket in 90s", self)
        while not os.path.exists(sock) and time.time() < deadline:
            time.sleep(0.05)
        self._id = 0
        self.mcp = subprocess.Popen(
            [MCP, "--connect", sock, "--token", tok],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=open(mcp_err, "w"), text=True, bufsize=1,
        )
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "popover-focus-out", "version": "1"}})
        if self._recv(timeout=15) is None:
            die("MCP did not initialize", self)
        self._send("notifications/initialized", notif=True)
        time.sleep(4)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=25):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                return None
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
            if not r:
                return None
            line = self.mcp.stdout.readline()
            if not line:
                return None
            if line.strip():
                return json.loads(line)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        res = (self._recv() or {}).get("result", {})
        p = res.get("structuredContent")
        if p is None:
            txt = "".join(c.get("text", "") for c in res.get("content", [])
                          if c.get("type") == "text")
            p = json.loads(txt) if txt.strip().startswith("{") else {}
        return res, p

    def nodes(self):
        return self.call("snapshot_tree")[1].get("nodes", [])

    def shot(self, path):
        res, _ = self.call("screenshot", {})
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")

    def stop(self):
        for p in (self.mcp, self.app):
            if p:
                try:
                    p.terminate()
                    p.wait(timeout=5)
                except Exception:
                    try:
                        p.kill()
                    except Exception:
                        pass


def focused(sess):
    return next((n for n in sess.nodes() if n.get("focused")), None)


def ids(sess):
    return {n["id"] for n in sess.nodes() if "id" in n}


def key(sess, k, shift=False):
    sess.call("inject_key", {"key": k, "shift": shift})
    time.sleep(0.2)


def parents_of(nodes):
    """child id -> parent id, from every node's own `children` list."""
    out = {}
    for n in nodes:
        for c in n.get("children", []):
            out[c] = n["id"]
    return out


def within(nodes, node_id, region):
    """Is `node_id` inside `region` — itself or under any node in it?

    Ancestry, not set membership. A popover's content root is built *dormant
    and parented to its trigger*, so it already exists before the panel opens
    and never shows up in a "nodes that appeared" diff. Focus landing on that
    wrapper is still focus inside the popover; treating it as "outside" would
    invent a bug that is not there.
    """
    if node_id in region:
        return True
    par = parents_of(nodes)
    seen = set()
    cur = par.get(node_id)
    while cur is not None and cur not in seen:
        if cur in region:
            return True
        seen.add(cur)
        cur = par.get(cur)
    return False


def main():
    src = sys.argv[1] if len(sys.argv) > 1 else "resources/examples/Starforgers.skrib"
    if not os.path.isabs(src):
        src = os.path.join(HERE, src)
    proj = working_copy(src, label="popover-focus-out")
    # Pin the locale rather than inherit it, and skip the welcome screen so the
    # project window is what comes up.
    env = isolated_config(locale="en-US", label="popover-focus-out", show_welcome=False)

    print("== launch ==")
    sess = Session(proj, env)
    print("connected")

    # A popover trigger is a Button carrying disclosure state: the snapshot
    # exposes `expanded`, which `PopoverButton`/`PopoverIconButton` track.
    # (`Splitter` also reports `expanded`; the role filter keeps it out.)
    triggers = [
        n for n in sess.nodes()
        if n.get("role") == "Button" and "expanded" in n and not n.get("disabled")
    ]
    if not triggers:
        die("no popover trigger on screen", sess)
    print(f"  {len(triggers)} popover trigger(s) visible: "
          f"{[t.get('label') for t in triggers]}")

    def expanded_now(tid):
        return next((n.get("expanded") for n in sess.nodes() if n.get("id") == tid), None)

    def check(t):
        """Open `t`, Tab until focus is out of its panel, and report.

        Returns (verdict, detail). `verdict` is 'ok', 'trapped', 'orphaned'
        (the failure this change exists to remove), or 'skip'.
        """
        label = t.get("label") or str(t["id"])
        before = ids(sess)
        sess.call("invoke_action", {"node": t["id"], "action": "click"})
        time.sleep(0.8)
        panel = ids(sess) - before
        if not (expanded_now(t["id"]) and panel):
            if expanded_now(t["id"]):
                key(sess, "Escape")
            return "skip", "opened nothing"

        ns0 = sess.nodes()
        f0 = next((n for n in ns0 if n.get("focused")), None)
        if VERBOSE:
            print(f"      open: focus={f0 and (f0.get('label') or f0['id'])} "
                  f"in_panel={bool(f0 and within(ns0, f0['id'], panel))} "
                  f"panel={len(panel)} nodes")
        for step in range(1, 15):
            key(sess, "Tab")
            ns = sess.nodes()
            cur = next((n for n in ns if n.get("focused")), None)
            alive = bool(panel & {n["id"] for n in ns if "id" in n})
            inside = bool(cur and within(ns, cur["id"], panel))
            if VERBOSE:
                print(f"      tab{step}: focus={cur and (cur.get('label') or cur['id'])} "
                      f"role={cur and cur.get('role')} in_panel={inside} alive={alive}")
            if not inside:
                # A dismissed overlay with a fade/roll-back tween stays on the
                # stack until the tween finishes (the collapsible MenuBar is
                # deliberately built that way). Poll for quiescence rather than
                # sleeping a magic number: under load the frame pacing shifts,
                # and a fixed wait turns an animated surface into a coin flip.
                deadline = time.time() + 6.0
                while (alive or expanded_now(t["id"])) and time.time() < deadline:
                    time.sleep(0.25)
                    alive = bool(panel & {n["id"] for n in sess.nodes() if "id" in n})
                exp = expanded_now(t["id"])
                if alive or exp:
                    return "orphaned", (
                        f"focus left after {step} Tab(s) but the panel is still "
                        f"up (alive={alive}, expanded={exp})"
                    )
                return "ok", f"closed when focus left, after {step} Tab(s)"
        key(sess, "Escape")
        return "trapped", "focus never left the panel in 14 Tabs"

    results = []
    only = os.environ.get("ONLY")
    for t in triggers:
        if only and only not in (t.get("label") or ""):
            continue
        label = t.get("label") or str(t["id"])
        verdict, detail = check(t)
        results.append((label, verdict, detail))
        print(f"  [{verdict:9}] {label!r}: {detail}")
        # Leave a clean slate for the next trigger.
        key(sess, "Escape")
        time.sleep(0.3)

    sess.shot("/tmp/popover-after-tab.png")
    checked = [r for r in results if r[1] != "skip"]
    bad = [r for r in checked if r[1] != "ok"]
    print(f"\n  {len(checked)} popover(s) actually exercised, {len(bad)} bad")
    if not checked:
        die("no trigger opened a panel — nothing was verified", sess)
    if bad:
        die("popovers that did not follow focus out: "
            + "; ".join(f"{l} ({v}: {d})" for l, v, d in bad), sess)
    print("PASS: every popover closed when keyboard focus left it")
    sess.stop()


main()
