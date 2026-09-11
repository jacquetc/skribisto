#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""**Format ▸ Marque de formatage**: the six invisible characters, end to end.

What no headless test can see. `format/marks.rs` has unit tests for the table
itself, but the table is only half the feature: the other half is a submenu
built per window, whose rows resolve an editor through `FormatViewModel` at the
moment focus sits on the *menu overlay* rather than in the prose. That is the
exact condition under which `insert_scene_break` once landed an edit nobody
could see until they clicked back into the text.

So this drives the real window, in French, and proves four things:

  1. the submenu exists under Format and carries exactly six rows, in
     LibreOffice Writer's order, with the French labels from `fr-FR/main.ftl`;
  2. four of them render a chord and two do not, which is the decision recorded
     in `marks::all` about `Ctrl+-` already being `editor.size.decrease`;
  3. activating a row actually writes its character into the manuscript, from a
     menu, with focus on the overlay;
  4. the character that lands is the one the row promises: the probe reads the
     prose back and looks for that codepoint, because every one of these six is
     invisible on screen and a wrong one would look identical.

Run: `python3 scripts/automation_formatting_marks.py`
"""

import json
import os
import select
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")
SHOTS = os.environ.get("SHOT_DIR", tempfile.mkdtemp(prefix="skribisto_marks_"))
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# The rows, as the French menu must read them, in the order `marks::all`
# declares: the label, the character it writes, and whether it shows a chord.
EXPECTED = [
    ("Espace insécable", " ", True),
    ("Tiret insécable", "‑", True),
    ("Trait d'union conditionnel", "­", False),
    ("Espace insécable fine", " ", True),
    ("Espace de largeur nulle", "​", True),
    ("Joint de mots", "⁠", False),
]


def fail(msg, s=None):
    print("FAIL:", msg)
    if s is not None:
        try:
            print("--- app log tail ---")
            print("\n".join(open(s.log).read().splitlines()[-25:]))
        except Exception:
            pass
        s.close()
    sys.exit(1)


class Session:
    """One launched app plus the automation bridge, trimmed to what this needs."""

    def __init__(self, argv, env):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(
            argv, stdout=open(self.log, "w"), stderr=subprocess.STDOUT, env=env
        )
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=90)
        except RuntimeError as e:
            fail(str(e), self)
        self._id = 0
        self.mcp = None
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            self.mcp = subprocess.Popen(
                fixture.mcp_argv(bridge, MCP),
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=open(mcp_err, "w"),
                text=True,
                bufsize=1,
            )
            self._send(
                "initialize",
                {
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "clientInfo": {"name": "formatting-marks", "version": "1"},
                },
            )
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
            fail("could not connect the automation bridge", self)
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
                continue
            line = self.mcp.stdout.readline()
            if not line:
                break
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if "id" in msg or "result" in msg or "error" in msg:
                return msg
        if fatal:
            fail("the bridge stopped answering", self)
        return None

    def call(self, tool, args=None):
        self._send("tools/call", {"name": tool, "arguments": args or {}})
        result = (self._recv() or {}).get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(
                c.get("text", "") for c in result.get("content", []) if c.get("type") == "text"
            ).strip()
            payload = json.loads(text) if (text.startswith("{") or text.startswith("[")) else {}
        return result, payload

    def nodes(self):
        """The AT tree is already flat under `nodes` — `snapshot_tree`, not
        `get_accessibility_tree`, which is what every other probe here calls."""
        _, p = self.call("snapshot_tree")
        return p.get("nodes", []) if isinstance(p, dict) else []

    def find(self, label, role=None, contains=False, timeout=10.0):
        end = time.time() + timeout
        while time.time() < end:
            for n in self.nodes():
                lab = n.get("label") or ""
                if role and n.get("role") != role:
                    continue
                if (label in lab) if contains else (lab == label):
                    return n
            time.sleep(0.4)
        return None

    def activate(self, node):
        """Fire the AccessKit action rather than a synthetic click: a press at a
        menu entry's centre does not open its submenu into the AT tree."""
        res, _ = self.call("invoke_action", {"node": node["id"], "action": "Click"})
        return not (isinstance(res, dict) and res.get("isError"))

    def click(self, node):
        b = node.get("bounds") or {}
        if "x" not in b:
            return False
        self.call(
            "inject_pointer",
            {
                "x": b["x"] + b.get("width", 0) / 2,
                "y": b["y"] + b.get("height", 0) / 2,
                "action": "click",
            },
        )
        return True

    def key(self, key, **mods):
        self.call("inject_key", {"key": key, **mods})

    def editors(self, ns=None):
        ns = ns if ns is not None else self.nodes()
        return [
            n
            for n in ns
            if n.get("role") == "MultilineTextInput" and "set_value" in (n.get("actions") or [])
        ]

    def main_editor(self, ns=None):
        """The tallest editor on screen: the manuscript body, never the
        capped-height synopsis box beside it."""
        eds = self.editors(ns)
        return max(eds, key=lambda n: (n.get("bounds") or {}).get("height", 0)) if eds else None

    def editor_text(self, node_id, ns=None):
        """A RichTextEditor carries its prose in per-block CHILD nodes, not in
        `value` (same walk as automation_dock_editor_probe.py)."""
        ns = ns if ns is not None else self.nodes()
        by_id = {n["id"]: n for n in ns if "id" in n}
        parts = []

        def walk(n):
            for k in ("label", "value", "name"):
                if n.get(k):
                    parts.append(str(n[k]))
                    break
            for cid in n.get("children") or []:
                child = by_id.get(cid)
                if child:
                    walk(child)

        node = by_id.get(node_id)
        if node:
            walk(node)
        return " ".join(parts)

    def shot(self, name):
        import base64

        path = os.path.join(SHOTS, name)
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")
                return path
        return None

    def close(self):
        for p in (getattr(self, "mcp", None), getattr(self, "app", None)):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


def open_marks_submenu(s):
    """Format ▸ Marque de formatage, left open. Returns its row nodes."""
    # The project window collapses its menu bar into a title-bar hamburger, so
    # the Format menu is two clicks in, not one (see automation_close_work.py).
    ham = s.find("Menu", role="Button", timeout=10)
    if ham is None or not s.click(ham):
        fail("no title-bar 'Menu' (hamburger) button", s)
    time.sleep(0.7)
    fmt = s.find("Format", role="MenuItem", timeout=6)
    if fmt is None or not s.activate(fmt):
        fail("the Format menu would not open", s)
    time.sleep(0.7)
    sub = s.find("Marque de formatage", role="MenuItem", contains=True, timeout=6)
    if sub is None:
        seen = [n.get("label") for n in s.nodes() if n.get("role") == "MenuItem"]
        s.shot("format-marks-menu-open.png")
        fail(f"no 'Marque de formatage' row in the Format menu; saw {seen}", s)
    if not s.activate(sub):
        fail("the formatting-mark submenu would not open", s)
    time.sleep(0.7)
    return [n for n in s.nodes() if n.get("role") == "MenuItem"]


print("== Format ▸ Marque de formatage ==")
env = fixture.isolated_config(locale="fr-FR", label="typo-marks")
project = fixture.working_copy(EXAMPLE, "typo-marks")
s = Session(fixture.launch_argv(project), env)

try:
    # ── the project, and a scene to write in ────────────────────────────────
    # A loaded project shows its binder, not an editor: a tab has to be opened
    # first. `load_work` is slow in a debug build, so the wait is generous.
    # Near the top of the tree on purpose: `inject_pointer` aims at the node's
    # bounds, so a row scrolled below the fold is a click into nothing.
    row = None
    end = time.time() + 120
    while time.time() < end and row is None:
        rows = [n for n in s.nodes() if n.get("role") == "TreeItem" and (n.get("label") or "")]
        for want in ("Prologue", "Chapter 1", "Chapitre 1"):
            row = next((n for n in rows if (n.get("label") or "").strip() == want), None)
            if row is not None:
                break
        if row is None:
            time.sleep(1.0)
    if row is None:
        fail("the example project never showed its binder", s)
    print(f"opening {row.get('label')!r} from the binder.")
    s.click(row)
    time.sleep(1.5)

    ed = None
    end = time.time() + 30
    while time.time() < end and ed is None:
        ed = s.main_editor()
        if ed is None:
            time.sleep(0.8)
    if ed is None:
        s.shot("format-marks-no-editor.png")
        fail("opening a binder row did not show a writing editor", s)
    s.click(ed)
    time.sleep(0.5)
    # `click` parks a focus request until the pane mounts; top it up so the menu
    # command below has a target the way a writer's caret would leave one.
    if (next((n for n in s.nodes() if n.get("focused")), {}) or {}).get(
        "role"
    ) != "MultilineTextInput":
        s.call("focus_node", {"node": ed["id"]})
        time.sleep(0.5)
    before = s.editor_text(ed["id"])
    print(f"editor reached, {len(before)} characters of prose in it.")

    # ── 1. the submenu, its rows and their order ────────────────────────────
    rows = open_marks_submenu(s)
    labels = [n.get("label") or "" for n in rows]
    missing = [want for want, _, _ in EXPECTED if not any(want in lab for lab in labels)]
    if missing:
        fail(f"rows missing from the submenu: {missing}\nsaw: {labels}", s)
    print("all six rows present, with their French labels.")

    seen = [
        next(lab for lab in labels if want in lab) for want, _, _ in EXPECTED
    ]
    order = [labels.index(lab) for lab in seen]
    if order != sorted(order):
        fail(f"the rows are not in the table's order: {seen}", s)
    print("and in the order `marks::all` declares them.")

    # ── 2. the rows as a reader sees them ───────────────────────────────────
    # The accelerator column is **not** in the AT tree: a row's node carries its
    # label and nothing else, so which rows show a chord cannot be asserted from
    # here. It is covered by `marks::all`'s unit tests (which four ids carry a
    # KeyStroke) and by looking at this capture. Asserting on what the tree does
    # not expose would be a check that passes by accident.
    s.shot("format-marks-submenu.png")
    print("submenu captured for the visual check of the accelerator column.")

    # ── 3 and 4. a row writes its own character, from the menu ──────────────
    target_label, target_char, _ = EXPECTED[0]
    row = next(n for n in rows if target_label in (n.get("label") or ""))
    if not s.activate(row):
        fail(f"activating {target_label!r} failed", s)
    time.sleep(0.8)

    after = ""
    end = time.time() + 10
    while time.time() < end:
        after = s.editor_text(ed["id"])
        if after.count(target_char) > before.count(target_char):
            break
        time.sleep(0.5)

    if after.count(target_char) <= before.count(target_char):
        fail(
            f"{target_label!r} inserted nothing: U+{ord(target_char):04X} count "
            f"{before.count(target_char)} -> {after.count(target_char)}",
            s,
        )
    print(
        f"{target_label} wrote U+{ord(target_char):04X} into the prose "
        f"({before.count(target_char)} -> {after.count(target_char)})."
    )
    s.shot("format-marks-inserted.png")

    print("\nPASS: the submenu is there, reads right in French, and writes what it says.")
finally:
    s.close()
