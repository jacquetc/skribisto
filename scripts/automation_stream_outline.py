#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto (mocks build) and check a stream's outline in the real window.

The headless tests pin the geometry against stand-ins and the mocks' tab; this checks
what a writer sees, through the accessibility tree of the running app:

1. **Indent by level.** Full Synopsis of the mocks' Book One, once per setting (None,
   Small, Medium, Large), each pinned with `--config editor.stream_indent`. Every row's
   synopsis box sits one step further in per level than the book's own box (the part
   one step, its chapters two, their scenes three) and every box ends at the same right
   edge. With the pane wide enough, the step is the setting's own.
2. **Levels shown.** "Show down to chapters" takes the scenes out, and a banner says
   "Scenes are hidden". On Full Book, reading the book as exported hides the banner
   (a reading shows every row it prints) and turning it off brings the banner back.
   "Show everything" in the banner brings every row back and the banner goes.
3. **Right to left** (he-IL): the right edges step in and the left edges stay.

Screenshots of each state go to `$SHOT_DIR` (default: the fixture's scratch directory),
one launch in the dark theme among them.

    scripts/automation_stream_outline.py
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
OUT = os.environ.get("SHOT_DIR", fixture.SCRATCH)
# Everything left of this x is the binder dock; the editor pane is to its right.
PANE_X = 300
# A rich text editor is a `Document` to AccessKit; the others are older bridges' names.
EDITOR_ROLES = ("Document", "TextInput", "MultilineTextInput", "TextField")
# The mocks' Book One, in Full Synopsis: the book's own synopsis, then its part, the
# chapter folder and its three scenes, the flat chapter and its scene.
DEPTHS = [0, 1, 2, 3, 3, 3, 2, 3]
STEPS = {"None": 0.0, "Small": 16.0, "Medium": 32.0, "Large": 48.0}
TOLERANCE = 1.5


def step_bounds(setting, own_width):
    """What `stream::level_inset` may give each level on this page.

    At most the setting's step. At least the setting's step capped by a third of the
    reference width shared out over the levels, and the reference (the narrower of the
    row's width and the text width) is at least as wide as the book's own editor, which
    sits inside it: so the floor is computed from that editor's width. Where the editor
    alone leaves room for the whole step, the two bounds meet and the step is exact."""
    whole = STEPS[setting]
    return min(whole, own_width / (3 * max(DEPTHS))), whole

failures = []


def ftl_value(locale, key):
    """A string as `locale` words it, read from its catalogue: what the probe has to
    look for when the interface is not in English."""
    path = fixture.repo_path("crates", "teksilo_ui", "locales", locale, "main.ftl")
    for line in open(path, encoding="utf-8"):
        m = re.match(rf"^{re.escape(key)} = (.*)$", line.rstrip("\n"))
        if m:
            return m.group(1)
    raise KeyError(f"{key} not in {path}")


class App:
    """One sandboxed launch with its MCP session."""

    def __init__(self, label, pins):
        fixture.assert_no_running_instance(SKRIBISTO)
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        env = fixture.isolated_config(locale=pins.get("ui.locale", "en-US"), label=label,
                                      show_welcome=False)
        pins_file = fixture.config_pins_file(pins, label=label)
        self.app = subprocess.Popen(fixture.launch_argv(None, pins=pins_file),
                                    stdout=open(self.log, "w"), stderr=subprocess.STDOUT,
                                    env=env)
        self.mcp = None
        self._id = 0
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app, timeout=90)
        except RuntimeError as e:
            self.die(str(e))
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                                    stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                                    text=True, bufsize=1)
        self.send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                 "clientInfo": {"name": "stream-outline", "version": "1"}})
        if self.recv(timeout=20, fatal=False) is None:
            self.die("could not connect MCP")
        self.send("notifications/initialized", notif=True)

    # ── the wire ──
    def send(self, method, params=None, notif=False):
        m = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            m["params"] = params
        if not notif:
            self._id += 1
            m["id"] = self._id
        self.mcp.stdin.write(json.dumps(m) + "\n")
        self.mcp.stdin.flush()

    def recv(self, timeout=25, fatal=True):
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
            self.die("no MCP response")
        return None

    def call(self, name, args=None):
        self.send("tools/call", {"name": name, "arguments": args or {}})
        res = self.recv().get("result", {})
        payload = res.get("structuredContent")
        if payload is None:
            txt = "".join(c.get("text", "") for c in res.get("content", [])
                          if c.get("type") == "text")
            payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
        return payload

    def die(self, msg):
        print("ERROR:", msg)
        print("--- app log tail ---")
        print("\n".join(open(self.log).read().splitlines()[-20:]))
        self.close()
        sys.exit(1)

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        for p in (self.mcp, self.app):
            if p:
                try:
                    p.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    p.kill()

    # ── the tree ──
    def settle(self):
        self.call("settle")
        time.sleep(0.35)

    def nodes(self):
        return self.call("snapshot_tree").get("nodes", [])

    def find(self, label, timeout=8.0, minx=None, role=None):
        end = time.time() + timeout
        while time.time() < end:
            for n in self.nodes():
                if n.get("label") != label or (role and n.get("role") != role):
                    continue
                if minx is None or (n.get("bounds") or {}).get("x", 0) >= minx:
                    return n
            self.settle()
        return None

    def has(self, label):
        return any(n.get("label") == label and n.get("role") != "TextRun"
                   for n in self.nodes())

    def press(self, node):
        b = node.get("bounds") or {}
        self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                     "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
        self.settle()
        time.sleep(0.4)

    def click(self, label, minx=None, role=None):
        n = self.find(label, minx=minx, role=role)
        if not n:
            return False
        if "click" in (n.get("actions") or []):
            self.call("invoke_action", {"node": n["id"], "action": "click"})
            self.settle()
            time.sleep(0.4)
        else:
            self.press(n)
        return True

    def shot(self, name):
        """The window, saved as `name.png`. The bridge hands the picture over as an
        `image` content item; the structured payload carries only its size."""
        self.send("tools/call", {"name": "screenshot", "arguments": {}})
        res = self.recv().get("result", {})
        for item in res.get("content", []):
            if item.get("type") == "image" and isinstance(item.get("data"), str):
                path = os.path.join(OUT, f"{name}.png")
                open(path, "wb").write(base64.b64decode(item["data"]))
                print(f"    shot -> {path}")
                return path
        failures.append(f"{name}: the bridge returned no screenshot")
        return None

    # ── the stream ──
    def open_book(self, segment):
        row = self.find("Mock Project", timeout=15)
        if not row:
            self.close()
            fixture.skip("not a `--features mocks` build: this probe opens the mocks' "
                         "Mock Project")
        self.press(row)
        end = time.time() + 20
        while time.time() < end and not self.has("Book One"):
            self.settle()
        if not self.click("Book One"):
            self.die("no 'Book One' row in the binder")
        if not self.click(segment, minx=PANE_X):
            self.die(f"no {segment!r} segment")
        self.settle()

    def editors(self):
        out = [n for n in self.nodes() if n.get("role") in EDITOR_ROLES
               and (n.get("bounds") or {}).get("x", 0) >= PANE_X
               and (n.get("bounds") or {}).get("y", 0) > 90]
        return sorted(out, key=lambda n: n["bounds"]["y"])


def check_outline(app, setting, rtl, tag):
    """Every box against the book's own: `depth × step` in from the leading edge, the
    trailing edge where the book's is."""
    eds = app.editors()
    if len(eds) != len(DEPTHS):
        failures.append(f"{tag}: {len(eds)} synopsis boxes in the pane, expected {len(DEPTHS)}")
        for n in eds:
            print("     ", n.get("role"), n["bounds"])
        return
    own = eds[0]["bounds"]
    own_left, own_right = own["x"], own["x"] + own["width"]
    first = eds[1]["bounds"]
    step = (own_right - first["x"] - first["width"]) if rtl else (first["x"] - own_left)
    low, high = step_bounds(setting, own["width"])
    print(f"    book's box {own['width']:.1f} wide: {step:.1f} px per level at {setting} "
          f"(allowed {low:.1f} to {high:.1f})")
    if not (low - TOLERANCE <= step <= high + TOLERANCE):
        failures.append(f"{tag}: a level is set in {step:.1f}, outside {low:.1f} to {high:.1f}")
    for n, depth in zip(eds, DEPTHS):
        b = n["bounds"]
        left, right = b["x"], b["x"] + b["width"]
        inset = (own_right - right) if rtl else (left - own_left)
        still = (left - own_left) if rtl else (own_right - right)
        print(f"    depth {depth}: inset {inset:6.1f}  (trailing edge off by {still:4.1f})")
        if abs(inset - depth * step) > TOLERANCE:
            failures.append(f"{tag}: a box {depth} deep is set in {inset:.1f}, not "
                            f"{depth} steps of {step:.1f}: the levels are not even")
        if abs(still) > TOLERANCE:
            failures.append(f"{tag}: a box {depth} deep moved its other edge by {still:.1f}")


def indent_run(setting, dark=False):
    tag = f"indent-{setting.lower()}" + ("-dark" if dark else "")
    print(f"\n=== Indent by level: {setting}{' (dark)' if dark else ''} ===")
    app = App(tag, {"ui.locale": "en-US", "ui.dark": dark, "ui.show_welcome": False,
                    "editor.stream_indent": setting})
    try:
        app.open_book("Full Synopsis")
        check_outline(app, setting, rtl=False, tag=tag)
        app.shot(f"stream-outline-{tag}")
    finally:
        app.close()


def levels_run():
    tag = "levels"
    print("\n=== Levels shown ===")
    app = App(tag, {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False,
                    "editor.stream_indent": "Medium"})
    try:
        app.open_book("Full Synopsis")
        top = app.editors()[0]["bounds"]["y"]
        combo = app.find("Levels shown", minx=PANE_X)
        if not combo:
            app.die("no 'Levels shown' control on Full Synopsis")
        app.call("invoke_action", {"node": combo["id"], "action": "click"})
        app.settle()
        option = app.find("Show down to chapters", role="ListBoxOption") or \
            app.find("Show down to chapters")
        if not option:
            app.die("the levels control offers no 'Show down to chapters'")
        app.press(option)
        eds = app.editors()
        n_eds = len(eds)
        print(f"    down to chapters: {n_eds} synopsis boxes")
        # The banner is pinned above the page, outside its scroll area, so the page
        # shows exactly its height lower; anything more is the page having scrolled.
        banner = next((n for n in app.nodes() if n.get("role") == "Status"
                       and n.get("label") == "Scenes are hidden"), None)
        if banner is None:
            failures.append(f"{tag}: the banner is not a status a screen reader hears")
        elif eds:
            moved = eds[0]["bounds"]["y"] - top
            pushed = banner["bounds"]["height"]
            print(f"    the page moved {moved:.1f} px, the banner is {pushed:.1f} px tall")
            if abs(moved - pushed) > TOLERANCE:
                failures.append(f"{tag}: hiding the scenes scrolled the page: the book's box "
                                f"moved {moved:.1f} px under a {pushed:.1f} px banner")
        if n_eds != 4:
            failures.append(f"{tag}: {n_eds} boxes with the scenes hidden, expected 4 "
                            "(the book's own, its part, its two chapters)")
        if not app.has("Scenes are hidden"):
            failures.append(f"{tag}: no 'Scenes are hidden' banner while scenes are hidden")
        app.shot("stream-outline-down-to-chapters")

        if not app.click("Full Book", minx=PANE_X):
            app.die("no 'Full Book' segment")
        if not app.has("Scenes are hidden"):
            failures.append(f"{tag}: Full Book lost the banner the filter still calls for")
        if not app.click("Read as exported", minx=PANE_X):
            app.die("no 'Read as exported' switch")
        if app.has("Scenes are hidden"):
            failures.append(f"{tag}: the banner stayed while the book is read as exported")
        app.shot("stream-outline-reading")
        app.click("Read as exported", minx=PANE_X)
        if not app.has("Scenes are hidden"):
            failures.append(f"{tag}: the banner did not come back after the reading")

        if not app.click("Show everything", minx=PANE_X, role="Button"):
            app.die("no 'Show everything' button in the banner")
        if app.has("Scenes are hidden"):
            failures.append(f"{tag}: the banner stayed after Show everything")
        if not app.click("Full Synopsis", minx=PANE_X):
            app.die("no 'Full Synopsis' segment")
        n_eds = len(app.editors())
        if n_eds != len(DEPTHS):
            failures.append(f"{tag}: {n_eds} boxes after Show everything, expected "
                            f"{len(DEPTHS)}")
        app.shot("stream-outline-everything-again")
    finally:
        app.close()


def rtl_run():
    tag = "rtl"
    print("\n=== Right to left (he-IL) ===")
    app = App(tag, {"ui.locale": "he-IL", "ui.dark": False, "ui.show_welcome": False,
                    "editor.stream_indent": "Medium"})
    try:
        app.open_book(ftl_value("he-IL", "full-synopsis"))
        check_outline(app, "Medium", rtl=True, tag=tag)
        app.shot("stream-outline-rtl")
    finally:
        app.close()


for setting in ("None", "Small", "Medium", "Large"):
    indent_run(setting)
indent_run("Medium", dark=True)
levels_run()
rtl_run()

print("\n================ RESULT ================")
if failures:
    for f in failures:
        print("FAIL:", f)
else:
    print("PASS: every level is set in by its step, the filter hides what it says and the "
          "banner follows it, and right to left mirrors the indent")
sys.exit(1 if failures else 0)
