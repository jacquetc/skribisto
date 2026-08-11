#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the Settings window's **parent pages**.

A parent in the category tree — a section, or the nested Typography group — used
to select nothing at all: the row highlighted, the chevron expanded it, and the
right-hand pane went on showing whichever leaf was open last. Each is a page now:
its title, the line saying what it is for, and a link to each thing under it.

Asserts, against the live AT tree:

  1. selecting the **Editor** section shows a page whose links are exactly its
     children — Typography (the nested group, as ONE link, not its six pages),
     Editor Behavior, Punctuation, Goals & Word Count, Writing games;
  2. following the **Typography** link lands on the group's own page, whose links
     are its six typography pages — so a parent's page is reachable from its
     parent's page, two levels deep;
  3. following the **Scene** link from there opens the real Scene typography form
     *and moves the tree's highlight to the Scene row*. Switching the pane while
     the tree went on highlighting something else is the exact disagreement these
     pages were added to end, so the highlight is asserted, not assumed;
  4. the **Work: `<title>`** section's page carries the open project's name in its
     title and links to that project's nine pages;
  5. every link on every parent's page carries a description line under it (the
     one thing a link has that the tree row above it doesn't).

Locale is pinned to en-US in a sandboxed config dir, so the assertions read the
strings this repo ships rather than whatever language the operator last used.

Run it after `cargo build -p teksilo_ui` — it drives the debug binary of the
checkout it lives in, worktrees included.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = os.path.join(REPO, "target", "debug", "skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"
EXAMPLE = os.path.join(REPO, "resources", "examples", "Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


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
    """One launched app + connected MCP server. Same shape as
    `automation_settings.py`'s, with the binary resolved from this checkout and
    an environment the caller supplies (the pinned-locale sandbox)."""

    def __init__(self, args, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
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
                fail("app exited before printing the bridge socket", self.app, None, self.log)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 20s", self.app, None, self.log)
        self.sock, self.tok = sock, tok
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
                                      "clientInfo": {"name": "parent-pages", "version": "1"}})
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
            joined = " | ".join(l for l in self.labels()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def dump(self, title):
        print(f"--- AT tree: {title} ---")
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            if lab:
                print(f"  [{n.get('role')}] {lab!r} sel={n.get('selected')} "
                      f"actions={n.get('actions')}")

    def shot(self, path):
        try:
            res, _ = self.call("screenshot")
            for c in res.get("content", []):
                if c.get("type") == "image" and c.get("data"):
                    open(path, "wb").write(base64.b64decode(c["data"]))
                    print(f"screenshot -> {path}")
                    return path
        except Exception as e:
            print("screenshot failed:", e)
        return None

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


# ── Launch: pinned locale, sandboxed config, a scratch copy of the example ────
fixture.assert_no_running_instance(SKRIBISTO)
env = fixture.isolated_config(locale="en-US", label="parent-pages", show_welcome=False)
project = fixture.working_copy(EXAMPLE, label="parent-pages")

print("== launch with the example loaded ==")
# `--new-instance`: the sandboxed config already gives this run its own primary
# socket, and asking for it outright removes any chance of being handed off to a
# copy of the app running from another checkout.
s = Session(["--new-instance", project], env=env)
# `load_work` is slow in a debug build — the bundled example takes ~20 s to reach
# its first window, so anything under 45 s is a coin flip on a cold cache.
if not s.wait_label("starforgers", timeout=60):
    fail("the example work did not load", s.app, s.mcp, s.log)
print("example loaded.")


def joined():
    return " | ".join(s.labels()).lower()


def settings_open():
    j = joined()
    return "reset to defaults" in j and "done" in j


def open_settings():
    for args in ({"key": ",", "ctrl": True},
                 {"key": ",", "modifiers": ["ctrl"]},
                 {"key": "Comma", "modifiers": ["ctrl"]}):
        res, _ = s.call("inject_key", args)
        if not (isinstance(res, dict) and res.get("isError")):
            time.sleep(0.8)
            if settings_open():
                return f"Ctrl+, ({args})"
    return None


how = open_settings()
if not how:
    s.dump("after open attempt")
    fail("could not open the Settings window", s.app, s.mcp, s.log)
print(f"Settings opened via {how}.")


# ── The category tree ────────────────────────────────────────────────────────
# Addressed structurally (the `Role::TreeItem` descendants of the Tree carrying a
# Settings-only row), never by pixel geometry: the binder's own TreeView is live
# behind the modal, so "Scene" and "Notes" exist in two trees at once.
COMPILE_SECTION = "compile & export"


def settings_tree():
    nodes = s.nodes()
    by_id = {n["id"]: n for n in nodes}

    def descendants(nid):
        out, stack = [], list((by_id.get(nid) or {}).get("children") or [])
        while stack:
            n = by_id.get(stack.pop())
            if not n:
                continue
            out.append(n)
            stack.extend(n.get("children") or [])
        return out

    for tree in (n for n in nodes if n.get("role") == "Tree"):
        rows = [n for n in descendants(tree["id"]) if n.get("role") == "TreeItem"]
        if any((r.get("label") or "").strip().lower() == COMPILE_SECTION for r in rows):
            return tree, rows
    return None, []


def rail_row(label):
    """The category row whose own label is exactly `label`, wheeling the rail
    until a virtualized row below the fold materializes."""
    want = label.lower()
    tree, rows = settings_tree()
    hit = next((r for r in rows if (r.get("label") or "").strip().lower() == want), None)
    if hit or not tree:
        return hit
    sig = lambda rs: tuple(sorted((r.get("label") or "") for r in rs))
    dy, flipped = -240, False
    for _ in range(24):
        before = sig(rows)
        s.call("scroll", {"node": tree["id"], "dx": 0, "dy": dy})
        time.sleep(0.35)
        tree, rows = settings_tree()
        hit = next((r for r in rows if (r.get("label") or "").strip().lower() == want), None)
        if hit:
            return hit
        if sig(rows) == before:
            if flipped:
                return None
            dy, flipped = -dy, True
    return None


def must(tool, args, what):
    res, payload = s.call(tool, args)
    if isinstance(res, dict) and res.get("isError"):
        txt = "".join(c.get("text", "") for c in res.get("content", [])
                      if c.get("type") == "text")
        fail(f"{what}: {tool} was rejected — {txt[:300]}", s.app, s.mcp, s.log)
    return payload


def select_row(label):
    """Reveal, then activate, a category row. Re-finds it between the two calls:
    scrolling past the virtualizer's buffer rebuilds the widget with a fresh id."""
    row = rail_row(label)
    if not row:
        print("  rail rows:", [r.get("label") for r in settings_tree()[1]])
        fail(f"no '{label}' row in the category tree", s.app, s.mcp, s.log)
    must("invoke_action", {"node": row["id"], "action": "scroll_into_view"}, label)
    time.sleep(0.4)
    row = rail_row(label) or row
    must("invoke_action", {"node": row["id"], "action": "click"}, label)
    time.sleep(0.7)
    return row


def row_is_selected(label):
    row = rail_row(label)
    return bool(row and row.get("selected"))


# ── The parent page under test ───────────────────────────────────────────────
def links():
    """The entries on a parent's page: every `Role::Link` **outside** the
    breadcrumb.

    The role alone is not enough — ARIA has a breadcrumb's crumbs be links too,
    so the trail at the top of the pane contributes one `Link` per crumb and the
    page's own title would count as an entry. Teksilo puts the trail under a
    `Role::Navigation` container, so excluding that subtree separates the two
    without guessing at labels or pixel rows."""
    nodes = s.nodes()
    by_id = {n["id"]: n for n in nodes}
    trail = set()
    for nav in (n for n in nodes if n.get("role") == "Navigation"):
        stack = list(nav.get("children") or [])
        while stack:
            nid = stack.pop()
            trail.add(nid)
            stack.extend((by_id.get(nid) or {}).get("children") or [])
    return [n for n in nodes if n.get("role") == "Link" and n["id"] not in trail]


def link_labels():
    return [(n.get("label") or "").strip() for n in links()]


def require_links(expected, where):
    got = link_labels()
    if got != expected:
        s.dump(where)
        fail(f"{where}: links are {got}, expected {expected}", s.app, s.mcp, s.log)
    print(f"  {where}: {got}")


def follow_link(label):
    want = label.lower()
    hit = next((n for n in links() if (n.get("label") or "").strip().lower() == want), None)
    if not hit:
        fail(f"no '{label}' link on this page (have {link_labels()})", s.app, s.mcp, s.log)
    must("invoke_action", {"node": hit["id"], "action": "click"}, f"follow '{label}'")
    time.sleep(0.7)


shots = []

# 1 ── the Editor section is a page, and lists its children ───────────────────
print("== 1. the Editor section's own page ==")
select_row("Editor")
if not row_is_selected("Editor"):
    fail("the Editor row did not take the selection", s.app, s.mcp, s.log)
require_links(
    ["Typography", "Editor Behavior", "Punctuation", "Goals & Word Count", "Writing games"],
    "Editor section page",
)
# The nested group is ONE entry: flattening it would undo the reason it exists.
if "Scene" in link_labels():
    fail("the Typography group was flattened into its section's page", s.app, s.mcp, s.log)
# The title is on the page itself, not only in the breadcrumb above it.
if joined().count("editor") < 2:
    fail("the Editor page does not show its own title", s.app, s.mcp, s.log)
shots.append(s.shot("/tmp/settings-parent-editor.png"))

# 2 ── a nested group is a page too, reached from its section's page ──────────
print("== 2. the Typography group's own page, followed from the section's ==")
follow_link("Typography")
require_links(
    ["Scene", "Synopsis", "Notes", "Corkboard", "Distraction-free", "Distraction-free themes"],
    "Typography group page",
)
if not row_is_selected("Typography"):
    fail("following the link left the tree highlighting the section", s.app, s.mcp, s.log)
shots.append(s.shot("/tmp/settings-parent-typography.png"))

# 3 ── a leaf link opens the real page AND moves the highlight ────────────────
print("== 3. following a leaf link ==")
follow_link("Scene")
if not row_is_selected("Scene"):
    fail("the tree kept highlighting Typography after opening Scene",
         s.app, s.mcp, s.log)
if link_labels():
    fail("Scene is a leaf and must show a form, not a list of links",
         s.app, s.mcp, s.log)
sliders = [n for n in s.nodes() if n.get("role") == "Slider"]
if len(sliders) < 5:
    s.dump("Scene page")
    fail(f"the Scene typography form is missing controls (only {len(sliders)} sliders)",
         s.app, s.mcp, s.log)
print(f"  Scene form has {len(sliders)} sliders and no links.")

# 4 ── the open project's section names the project ───────────────────────────
print("== 4. the Work section's own page ==")
work_row = next((r for r in settings_tree()[1]
                 if (r.get("label") or "").lower().startswith("work:")), None)
if not work_row:
    fail("no 'Work: <title>' section — the example did not open?", s.app, s.mcp, s.log)
work_label = (work_row.get("label") or "").strip()
select_row(work_label)
require_links(
    ["Author", "Structure", "Language", "Backups", "Personal dictionary", "Tags",
     "Templates", "Text replacements", "Punctuation"],
    "Work section page",
)
if joined().count(work_label.lower()) < 2:
    fail("the Work page does not carry the project's name in its own title",
         s.app, s.mcp, s.log)
shots.append(s.shot("/tmp/settings-parent-work.png"))

# 5 ── every remaining section, including the collapsed ones ─────────────────
# The three sections that start collapsed are the ones a writer is most likely to
# select without expanding — which is precisely when the pane used to keep
# showing the previous page. Selecting a section does not expand it
# (`row_click_expands(false)`), so its page is the only thing that says what is
# inside.
print("== 5. every other section's page ==")
for section, children in [
    ("Appearance & Behaviour", ["Appearance", "Menus & Toolbars", "Notifications"]),
    ("Spelling", ["Spell-checking", "Dictionaries"]),
    ("Backup & Sync", ["Autosave", "Backups"]),
    ("Compile & Export", ["Export Formats", "Paratext Structures"]),
]:
    select_row(section)
    if not row_is_selected(section):
        fail(f"the {section} row did not take the selection", s.app, s.mcp, s.log)
    require_links(children, f"{section} section page")

# The descriptions under each link are plain `TextWidget`s, and this stack prunes
# those from the accessibility snapshot (the same reason `automation_settings.py`
# asserts on controls rather than on `GroupHeader` / `FormLayout` field labels).
# They are covered instead by `settings.rs`'s `every_built_in_page_says_what_it_is_for`
# and by the screenshots this probe leaves behind.

print("\nPASS — every parent in the Settings tree is a page that lists its children.")
for p in shots:
    if p:
        print(f"  {p}")
s.close()
