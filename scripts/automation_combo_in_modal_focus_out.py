#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""A dropdown opened inside Settings must close when Tab moves on.

    scripts/automation_combo_in_modal_focus_out.py resources/examples/Starforgers.skrib

Regression probe for a real bug report: arrow through a `ComboBox` list inside
Settings, press Tab, and focus moved on while the popover stayed on screen.

The mechanism is anchor-aware for a reason — a non-searchable `ComboBox` keeps
focus on its own trigger the whole time its list is up, so "focus left the
overlay's content" is never true for it. But the trigger here lives inside the
Settings **modal**, and looking the overlay up by content first answered with
the modal (which deliberately does not follow focus out) instead of the
dropdown anchored to the focused widget. `overlay_orbit_for_widget` now makes
one top-down pass asking both questions, so the nearer overlay wins.

`focus_impl.rs`'s `a_dropdown_inside_a_modal_still_follows_focus_out` pins the
same thing headlessly; this proves it reaches the shipped Settings dialog.
"""

import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import isolated_config, working_copy

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = os.path.join(HERE, "target", "debug", "skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
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
        deadline = time.time() + 90
        while time.time() < deadline:
            txt = open(self.log).read()
            a = re.search(r"bridge socket = (\S+)", txt)
            b = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
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
                                  "clientInfo": {"name": "combo-in-modal", "version": "1"}})
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


def ids(sess):
    return {n["id"] for n in sess.nodes() if "id" in n}


def key(sess, k, shift=False):
    sess.call("inject_key", {"key": k, "shift": shift})
    time.sleep(0.25)


def click(sess, node_id):
    """A *pointer* click, not `invoke_action`.

    The AccessKit click action activates a widget without the pointer-down
    focus step, so it opens a dropdown while leaving focus wherever it already
    was — which is not what a user does, and turns any focus assertion
    afterwards into a measurement of the probe rather than the app.
    """
    b = next((n.get("bounds") for n in sess.nodes() if n.get("id") == node_id), None)
    if b:
        sess.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                     "y": b["y"] + b.get("height", 0) / 2,
                                     "action": "click"})
    else:
        sess.call("invoke_action", {"node": node_id, "action": "click"})
    time.sleep(0.6)


def click_row(sess, node_id):
    """Rail rows are driven through the a11y action deliberately — they scroll
    and virtualize, so a coordinate can be stale by the time it lands."""
    sess.call("invoke_action", {"node": node_id, "action": "click"})
    time.sleep(0.6)


def main():
    src = sys.argv[1] if len(sys.argv) > 1 else "resources/examples/Starforgers.skrib"
    if not os.path.isabs(src):
        src = os.path.join(HERE, src)
    proj = working_copy(src, label="combo-in-modal")
    env = isolated_config(locale="en-US", label="combo-in-modal", show_welcome=False)

    print("== launch ==")
    sess = Session(proj, env)
    print("connected")

    def labels():
        return [(n.get("label") or "").lower() for n in sess.nodes()]

    def settings_open():
        ls = labels()
        return (any("reset to defaults" in l or "réinitialiser" in l for l in ls)
                and any(l.strip() in ("done", "terminé") for l in ls))

    # Settings is a centered modal — that is the whole point of this probe.
    opened_via = None
    for args in ({"key": ",", "ctrl": True},
                 {"key": ",", "modifiers": ["ctrl"]},
                 {"key": "Comma", "modifiers": ["ctrl"]}):
        res, _ = sess.call("inject_key", args)
        if isinstance(res, dict) and res.get("isError"):
            continue
        time.sleep(1.2)
        if settings_open():
            opened_via = args
            break
    if not opened_via:
        sess.shot("/tmp/combo-in-modal-noopen.png")
        die("could not open the Settings window", sess)
    print(f"  Settings open (via {opened_via})")

    def parents_of(nodes):
        out = {}
        for n in nodes:
            for c in n.get("children", []):
                out[c] = n["id"]
        return out

    def within(nodes, node_id, region):
        """Ancestry, not set membership — a panel's content root is built
        dormant under its trigger, so it predates the panel and never shows up
        in a nodes-that-appeared diff."""
        if node_id in region:
            return True
        par, seen, cur = parents_of(nodes), set(), None
        cur = par.get(node_id)
        while cur is not None and cur not in seen:
            if cur in region:
                return True
            seen.add(cur)
            cur = par.get(cur)
        return False

    def expanded_now(tid):
        return next((n.get("expanded") for n in sess.nodes() if n.get("id") == tid), None)

    def combos():
        return [n for n in sess.nodes()
                if "expanded" in n and not n.get("disabled")
                and n.get("role") in ("ComboBox", "PopUpButton")]

    def exercise(t, page):
        """Open one combo, arrow, then Tab until focus leaves. Returns a verdict."""
        label = t.get("label") or str(t["id"])
        before = ids(sess)
        click(sess, t["id"])
        panel = ids(sess) - before
        if not (expanded_now(t["id"]) and panel):
            if expanded_now(t["id"]):
                key(sess, "Escape")
            return "skip", f"{page}/{label}: opened nothing"

        ns = sess.nodes()
        f0 = next((n for n in ns if n.get("focused")), None)
        print(f"      open: focus={f0 and (f0.get('label') or f0['id'])} "
              f"role={f0 and f0.get('role')} in_panel={bool(f0 and within(ns, f0['id'], panel))} "
              f"trigger={t['id']} panel={len(panel)}")

        # The reported sequence: arrow through the list first.
        key(sess, "ArrowDown")
        key(sess, "ArrowDown")
        ns = sess.nodes()
        f1 = next((n for n in ns if n.get("focused")), None)
        print(f"      arrows: focus={f1 and (f1.get('label') or f1['id'])} "
              f"role={f1 and f1.get('role')} in_panel={bool(f1 and within(ns, f1['id'], panel))}")

        for step in range(1, 16):
            key(sess, "Tab")
            ns = sess.nodes()
            cur = next((n for n in ns if n.get("focused")), None)
            inside = bool(cur and within(ns, cur["id"], panel))
            print(f"      tab{step}: focus={cur and (cur.get('label') or cur['id'])} "
                  f"role={cur and cur.get('role')} in_panel={inside}")
            if inside:
                continue
            alive = bool(panel & {n["id"] for n in ns if "id" in n})
            deadline = time.time() + 5.0
            while (alive or expanded_now(t["id"])) and time.time() < deadline:
                time.sleep(0.25)
                alive = bool(panel & {n["id"] for n in sess.nodes() if "id" in n})
            exp = expanded_now(t["id"])
            if alive or exp:
                return "ORPHANED", (f"{page}/{label}: focus left after {step} Tab(s) "
                                    f"but the list is still up (alive={alive}, expanded={exp})")
            return "ok", f"{page}/{label}: closed when focus left, after {step} Tab(s)"
        key(sess, "Escape")
        return "TRAPPED", f"{page}/{label}: focus never left the list in 15 Tabs"

    def rail_row(label):
        return next((n for n in sess.nodes()
                     if n.get("role") == "TreeItem"
                     and (n.get("label") or "").strip().lower() == label.lower()), None)

    def select_page(label):
        """Reveal then activate a category row, re-finding it in between: the
        TreeView virtualizes, so scrolling rebuilds the row with a fresh id."""
        row = rail_row(label)
        if not row:
            return False
        sess.call("invoke_action", {"node": row["id"], "action": "scroll_into_view"})
        time.sleep(0.4)
        row = rail_row(label) or row
        click_row(sess, row["id"])
        time.sleep(0.8)
        return True

    results = []
    for page in ("Appearance", "Editor Behavior", "Scene", "Punctuation"):
        if not select_page(page):
            print(f"  (no {page!r} row)")
            continue
        found = combos()
        print(f"  {page}: {[c.get('label') for c in found]}")
        for t in found:
            verdict, detail = exercise(t, page)
            if verdict != "skip":
                results.append((verdict, detail))
                print(f"  [{verdict:9}] {detail}")
            key(sess, "Escape")
            time.sleep(0.2)

    if not results:
        die("no ComboBox was exercised on any settings page", sess)
    bad = [r for r in results if r[0] != "ok"]
    print(f"\n  {len(results)} combo(s) exercised, {len(bad)} bad")
    sess.shot("/tmp/combo-in-modal-after-tab.png")
    if bad:
        die("dropdowns that did not follow focus out: "
            + "; ".join(d for _, d in bad), sess)
    print("PASS: every ComboBox in Settings closed when focus left it")
    sess.stop()
    return





main()
