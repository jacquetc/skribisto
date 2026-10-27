#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""**A trashed item's text stays as it was**, whatever route reaches it.

A trashed scene opens from the Trash dock in a read-only editor: the writer can
read it, select it and copy it out, and nothing else. teksilo enforces that on
the editor's own keys, but the editor's handle writes whenever it is asked, and
every Skribisto command that writes goes through a handle. So each route has to
ask first (`crate::shared::editor_writes`), and this drives the real window to
prove they do:

  1. control: over an ordinary scene, Format ▸ Bold is enabled (so a disabled
     row below means the gate, not a probe that cannot see the flag);
  2. the scene is trashed, and reopened from the Trash dock under its banner;
  3. Backspace, Delete, a typed letter, Ctrl+X, Ctrl+V, Ctrl+B, Shift+Enter and
     Ctrl+Shift+Enter change nothing. The writing game's filter, `All` while the
     game is off, used to be pushed onto the read-only editor in place of its
     `ReadOnly`, so it took keys as an ordinary editor does: the first run of this
     probe saw Ctrl+X cut four characters out of the trashed scene;
  4. the right-click menu offers Copy and no Paste, Bold strip or Split scene;
  5. Format ▸ Bold and Insert line break, and Document ▸ Insert footnote and
     Insert image, are disabled;
  6. after all of it, the text reads exactly as it did.

Run: `python3 scripts/automation_trashed_text.py`
"""

import base64
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
SHOTS = os.environ.get("SHOT_DIR", tempfile.mkdtemp(prefix="skribisto_trashed_"))
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
SCENE = "Prologue"


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
                    "clientInfo": {"name": "trashed-text", "version": "1"},
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

    def settle(self, times=3):
        for _ in range(times):
            self.call("settle")

    def nodes(self):
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
        res, _ = self.call("invoke_action", {"node": node["id"], "action": "Click"})
        return not (isinstance(res, dict) and res.get("isError"))

    def click(self, node, button="primary"):
        b = node.get("bounds") or {}
        if "x" not in b:
            return False
        self.call(
            "inject_pointer",
            {
                "x": b["x"] + min(b.get("width", 0) / 2, 200.0),
                "y": b["y"] + min(b.get("height", 0) / 2, 24.0),
                "button": button,
            },
        )
        self.settle()
        return True

    def key(self, key, **mods):
        """A key press to the focused widget; `inject_key` takes no node, and a
        refused call fails the probe rather than passing for a key that did nothing."""
        res, _ = self.call("inject_key", {"key": key, **mods})
        if not res or res.get("isError"):
            fail(f"inject_key {key} {mods} was refused: {res}", self)
        self.settle(2)

    def prose(self, ns=None):
        """The manuscript editor, whichever role it has: an editable one is a
        `MultilineTextInput`, a read-only one a `Document`. The one holding the
        most text is the manuscript, not the synopsis beside it."""
        ns = ns if ns is not None else self.nodes()
        eds = [n for n in ns if n.get("role") in ("MultilineTextInput", "Document")]
        return max(eds, key=lambda e: len(self.subtree_text(e["id"], ns)), default=None)

    @staticmethod
    def subtree_text(node_id, ns):
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

    def prose_text(self):
        ns = self.nodes()
        ed = self.prose(ns)
        return self.subtree_text(ed["id"], ns) if ed else ""

    def shot(self, name):
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
            self.app.wait(timeout=10)
        except Exception:
            self.app.kill()


def is_menu_row(n):
    """A menu row of any kind: a plain `MenuItem`, or a checkable one such as Bold
    (`MenuItemCheckBox`) or an alignment (`MenuItemRadio`)."""
    return (n.get("role") or "").startswith("MenuItem")


def open_menu(s, top, contains_label=None):
    """Open a top-level menu through the title-bar hamburger and return its rows.
    `contains_label` waits for a row of that menu to appear. Tried twice: right
    after another menu closed, the first click can land while it is still going."""
    rows = []
    for attempt in range(2):
        ham = s.find("Menu", role="Button", timeout=10)
        if ham is None or not s.click(ham):
            fail("no title-bar 'Menu' (hamburger) button", s)
        time.sleep(0.6)
        entry = s.find(top, role="MenuItem", timeout=6)
        if entry is not None and s.activate(entry):
            time.sleep(0.6)
            end = time.time() + 6
            while time.time() < end:
                rows = [n for n in s.nodes() if is_menu_row(n)]
                if not contains_label or row(rows, contains_label) is not None:
                    return rows
                time.sleep(0.4)
        close_menus(s)
        time.sleep(1.0)
    fail(f"the {top} menu did not open with {contains_label!r}; saw "
         f"{[n.get('label') for n in rows]}", s)


def row(rows, label):
    return next((n for n in rows if (n.get("label") or "") == label), None)


def close_menus(s):
    for _ in range(3):
        s.key("Escape")
    time.sleep(0.5)


def unchanged(s, before, what):
    after = s.prose_text()
    if after != before:
        at = next((i for i, (a, b) in enumerate(zip(before, after)) if a != b),
                  min(len(before), len(after)))
        s.shot("trashed-changed.png")
        fail(
            f"{what} changed the trashed text\n"
            f"  before=...{before[max(0, at - 40):at + 40]!r}\n"
            f"  after =...{after[max(0, at - 40):at + 40]!r}",
            s,
        )


print("== A trashed item's text stays as it was ==")
env = fixture.isolated_config(locale="en-US", label="trashed-text", show_welcome=False)
project = fixture.working_copy(EXAMPLE, "trashed-text")
s = Session(fixture.launch_argv(project), env)

try:
    # ── 1. The control, over an ordinary scene ───────────────────────────────
    item = None
    end = time.time() + 120
    while time.time() < end and item is None:
        item = next(
            (n for n in s.nodes()
             if n.get("role") == "TreeItem" and (n.get("label") or "").strip() == SCENE),
            None,
        )
        if item is None:
            time.sleep(1.0)
    if item is None:
        fail(f"the binder never showed {SCENE!r}", s)
    s.click(item)
    time.sleep(1.5)
    ed = s.prose()
    if ed is None or ed.get("role") != "MultilineTextInput":
        fail(f"{SCENE} did not open in an editable editor", s)
    s.click(ed)
    rows = open_menu(s, "Format", "Bold")
    if row(rows, "Bold") is None or row(rows, "Bold").get("disabled"):
        fail("control: Format ▸ Bold is not enabled over an ordinary scene", s)
    close_menus(s)
    print("1. Control: Format ▸ Bold is enabled over an ordinary scene.")

    # ── 2. Trash it, and reopen it from the Trash dock ───────────────────────
    rows = open_menu(s, "Document", "Move to Trash")
    if not s.activate(row(rows, "Move to Trash")):
        fail("Document ▸ Move to Trash would not run", s)
    time.sleep(1.5)
    tab = next(
        (n for n in s.nodes() if n.get("role") == "Tab" and (n.get("label") or "") == "Trash"),
        None,
    )
    if tab is None or not s.click(tab):
        fail("no 'Trash' rail tab", s)
    time.sleep(1.0)
    trashed = s.find(SCENE, role="TreeItem", timeout=10)
    if trashed is None:
        s.shot("trashed-dock.png")
        fail(f"{SCENE} is not in the Trash dock", s)
    s.click(trashed)
    if s.find("This item is in the Trash", contains=True, timeout=15) is None:
        s.shot("trashed-open.png")
        fail("the trashed scene did not open under its banner", s)
    ed = s.prose()
    if ed is None or ed.get("role") != "Document":
        fail(f"the trashed scene's editor is not read-only: {ed and ed.get('role')}", s)
    before = s.prose_text()
    if not before:
        fail("the trashed scene shows no text", s)
    s.shot("trashed-open.png")
    print("2. Trashed and reopened from the Trash dock, read-only, under its banner.")

    # ── 3. The keys that write, including the Edit route's ───────────────────
    s.click(ed)
    for _ in range(4):
        s.key("Left", shift=True)
    s.key("c", command=True)
    for what, args in [
        ("Backspace", {"key": "Backspace"}),
        ("Delete", {"key": "Delete"}),
        ("a typed letter", {"key": "z", "text": "z"}),
        ("Ctrl+X", {"key": "x", "command": True}),
        ("Ctrl+V", {"key": "v", "command": True}),
        ("Ctrl+B", {"key": "b", "command": True}),
        ("Shift+Enter", {"key": "Enter", "shift": True}),
        ("Ctrl+Shift+Enter", {"key": "Enter", "command": True, "shift": True}),
    ]:
        name = args.pop("key")
        s.key(name, **args)
        unchanged(s, before, what)
    print(
        "3. Backspace, Delete, a typed letter, Ctrl+X, Ctrl+V, Ctrl+B, Shift+Enter and"
        " Ctrl+Shift+Enter changed nothing."
    )

    # ── 4. The right-click menu ──────────────────────────────────────────────
    ed = s.prose()
    s.click(ed, button="secondary")
    time.sleep(0.8)
    labels = [n.get("label") or "" for n in s.nodes() if is_menu_row(n)]
    if "Copy" not in labels:
        s.shot("trashed-context.png")
        fail(f"the right-click menu has no Copy; saw {labels}", s)
    offered = [w for w in ("Paste", "Paste Unformatted", "Split scene", "Bold") if w in labels]
    if offered:
        s.shot("trashed-context.png")
        fail(f"the right-click menu offers {offered} over trashed text; saw {labels}", s)
    close_menus(s)
    print("4. The right-click menu offers Copy, and nothing that writes.")

    # ── 5. The menus ─────────────────────────────────────────────────────────
    rows = open_menu(s, "Format", "Bold")
    on = [w for w in ("Bold", "Insert line break", "Insert scene break")
          if row(rows, w) is not None and not row(rows, w).get("disabled")]
    if on:
        s.shot("trashed-format-menu.png")
        fail(f"Format menu rows enabled over trashed text: {on}", s)
    bold = row(rows, "Bold")
    if bold is not None:
        s.activate(bold)
        time.sleep(0.6)
    close_menus(s)
    unchanged(s, before, "Format ▸ Bold")
    rows = open_menu(s, "Document", "Insert footnote")
    on = [w for w in ("Insert footnote", "Insert image…")
          if row(rows, w) is not None and not row(rows, w).get("disabled")]
    if on:
        s.shot("trashed-document-menu.png")
        fail(f"Document menu rows enabled over trashed text: {on}", s)
    close_menus(s)
    print("5. Format ▸ Bold, line and scene breaks, Insert footnote and Insert image are off.")

    # ── 6. Nothing moved ─────────────────────────────────────────────────────
    unchanged(s, before, "the whole run")
    print("6. The trashed text reads exactly as it did.")
    print("\nPASS: a trashed item's text took no write from any route.")
finally:
    s.close()
