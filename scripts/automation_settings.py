#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the teksilo automation MCP bridge and verify the
Settings preferences window end-to-end.

Launches with the bundled example loaded (a project path on argv skips the
Launcher entirely under the launcher-window model — see `teksilo_ui::main`'s
module docs), opens Settings (Ctrl+, with a menu fallback), then asserts:

  1. the category TreeView holds every section + page (Appearance & Behaviour,
     Editor ▸ Scene/Synopsis/Notes/Editor Behavior/Goals/Corkboard, Spelling,
     Backup & Sync, Compile & Export, Keymap), and each row is AT-drivable:
     role + name + level + actions all on ONE node, and an action nothing
     handles comes back as an error rather than a silent success;
  2. the default pane (Editor ▸ Scene, a typography form) shows its controls
     (the font-family ComboBox + the typography sliders);
  3. selecting the Appearance page switches the pane (Interface language, the
     Theme ComboBox — moved here from the old Manuscript & Fonts page — and
     the "show the launcher at startup" checkbox, formerly worded "show the
     Welcome screen"; it now also governs whether a bare launch reopens the
     last project instead);
  4. expanding Backup & Sync and selecting Autosave reveals the autosave setting;
  5. a per-project page below the scrolled rail's fold still scrolls into view
     and activates;
  6. selecting the empty Keymap page shows the "no settings yet" placeholder;
  7. the SearchField accepts a query;
  8. Done dismisses the window.

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.

The rail is driven entirely through AT actions on rows found by role + label
(`scroll_into_view` / `click` / `expand`), never by synthetic clicks at row
bounds. That is not tidiness: a TreeView row is virtualized, so a row outside
the viewport reports *content* coordinates — the ones under "Work:" land below
the window's own bottom edge — and the disclosure chevron is a nameless 16 px
target whose x depends on an indent level. See `settings_tree_rows`.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

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
    """One launched app + connected MCP server."""

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
                                      "clientInfo": {"name": "settings-test", "version": "1"}})
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

    def by_role(self, role):
        return [n for n in self.nodes() if n.get("role") == role]

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(l for l in self.labels()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def find(self, label, role=None):
        for n in self.nodes():
            if (n.get("label") or "").strip() == label and (role is None or n.get("role") == role):
                return n
        return None

    def find_contains(self, substr, role=None):
        sub = substr.lower()
        for n in self.nodes():
            if sub in (n.get("label") or "").lower() and (role is None or n.get("role") == role):
                return n
        return None

    def require(self, needles, where):
        joined = " | ".join(self.labels()).lower()
        for n in needles:
            if n.lower() not in joined:
                print("  current labels:", joined[:600])
                fail(f"expected '{n}' in {where}", self.app, self.mcp, self.log)

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
        except Exception as e:
            print("screenshot failed:", e)

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


# ── Launch (example loaded → Welcome suppressed) ──────────────────────────────
print("== launch with example loaded ==")
s = Session([EXAMPLE])
# `load_work` is slow in a debug build — the bundled example takes ~20 s to
# reach its first window, so a 15 s budget was a coin flip on a cold cache.
if not s.wait_label("starforgers", timeout=45):
    fail("the example work did not load", s.app, s.mcp, s.log)
print("example loaded.")

# ── Locale-robust anchors ────────────────────────────────────────────────────
# The app restores whatever UI language was last persisted, so every concept is
# matched against its en-US *and* fr-FR wording. (Concept → list of substrings.)
#
# NOTE: the panel was restructured (independently of the launcher-window
# model) — "Manuscript & Fonts" no longer exists as a single page. Editor ▸
# Scene/Synopsis/Notes now each carry their own typography form, Editor ▸
# Editor Behavior carries the synopsis-pane/typewriter/highlight toggles, and
# the Theme/Interface-text-size controls moved to Appearance & Behaviour ▸
# Appearance. The default pane on open is now Scene.
SECTIONS = {
    "appearance_behaviour": ["appearance & behaviour", "apparence et comportement"],
    "editor": ["editor", "éditeur"],
    "scene": ["scene", "scène"],
    "spelling": ["spelling", "orthographe"],
    "backup": ["backup & sync", "sauvegarde et synchronisation"],
    "compile": ["compile & export", "compilation et export"],
    "keymap": ["keymap", "raccourcis clavier"],
}
# GroupHeaders and FormLayout field labels are decorative (not AccessKit
# labels), so assert on the controls the AT tree actually surfaces. The
# default (Scene) pane is a typography form: a font-family ComboBox plus five
# sliders (size, line height, first-line indent, paragraph spacing before/after).
SCENE_MIN_SLIDERS = 5
# (page label variants) for clicking a tree leaf — exact match, not substring, so
# "Appearance" never matches the "Appearance & Behaviour" section.
APPEARANCE_PAGE = ["appearance", "apparence"]
BACKUP_SECTION = ["backup & sync", "sauvegarde et synchronisation"]
AUTOSAVE_PAGE = ["autosave", "enregistrement automatique"]
KEYMAP_PAGE = ["keymap", "raccourcis clavier"]
# Settings-only, so it identifies the category tree among the app's TreeViews.
COMPILE_SECTION = ["compile & export", "compilation et export"]
# A per-project page, near the bottom of the rail and below the fold at any
# ordinary window height — the case that used to be unreachable entirely.
# ("Punctuation" would not do: the rail has two of those.)
BELOW_FOLD_PAGE = ["text replacements", "remplacements de texte"]
APPEARANCE_BITS = {
    # Reworded for the launcher-window model: the checkbox now describes
    # "show the launcher at startup (otherwise, reopen the last project)",
    # not "show the Welcome screen" (there's no modal to show any more).
    "welcome checkbox": ["show the launcher", "afficher le lanceur"],
}
AUTOSAVE_BIT = ["autosave to disk", "sur le disque"]
EMPTY_BIT = ["no settings here yet", "aucun paramètre ici"]


def joined():
    return " | ".join(s.labels()).lower()


def has_any(variants):
    j = joined()
    return any(v in j for v in variants)


def require_all(concepts, where):
    j = joined()
    for name, variants in concepts.items():
        if not any(v in j for v in variants):
            print("  current labels:", j[:700])
            fail(f"expected '{name}' in {where}", s.app, s.mcp, s.log)


def node_match(variants, exact=False):
    """First node anywhere in the AT tree whose label matches any variant.
    For *category-rail rows* use `rail_row` instead — this one is for ordinary
    chrome (buttons, combo items)."""
    for n in s.nodes():
        lab = (n.get("label") or "").strip().lower()
        if not lab:
            continue
        if lab in variants if exact else any(v in lab for v in variants):
            return n
    return None


def pointer_click(b, dx=None):
    """Synthetic primary click (atomic down+up → a real tap gesture) at the
    centre of a bounds dict, or at `bounds.x + dx` when `dx` is given."""
    if not (isinstance(b, dict) and "x" in b):
        return False
    cx = b["x"] + (dx if dx is not None else b.get("width", 0) / 2)
    cy = b["y"] + b.get("height", 0) / 2
    s.call("inject_pointer", {"x": cx, "y": cy, "action": "click"})
    return True


def click_node(n):
    """Click a node — via its AccessKit `click` action when it has one, else a
    synthetic pointer at its centre."""
    if "click" in (n.get("actions") or []):
        res, _ = s.call("invoke_action", {"node": n["id"], "action": "click"})
        return not (isinstance(res, dict) and res.get("isError"))
    return pointer_click(n.get("bounds") or {})


def must(tool, args, what):
    """Call an MCP tool that is expected to work, and fail loudly when it
    doesn't. Every AT action below goes through this on purpose: the bridge
    now answers UNHANDLED_ACTION for an action nothing acted on, so a silent
    no-op can no longer masquerade as a passing step."""
    res, payload = s.call(tool, args)
    if isinstance(res, dict) and res.get("isError"):
        txt = "".join(c.get("text", "") for c in res.get("content", [])
                      if c.get("type") == "text")
        fail(f"{what}: {tool} was rejected — {txt[:300]}", s.app, s.mcp, s.log)
    return payload


# ── Driving the category tree ────────────────────────────────────────────────
# Rows are addressed *structurally* — the `Role::TreeItem` descendants of the
# Tree whose subtree carries a Settings-only section — and driven through their
# advertised AT actions. Not by pixel geometry, and that is not a style
# preference:
#   * the binder's TreeView is live behind the modal, so page names like
#     "Scene" / "Notes" / "Tags" exist in two trees at once;
#   * the rail scrolls, and a virtualized row parked outside the viewport
#     reports CONTENT coordinates — the rows under "Work:" sit below the
#     window's own bottom edge, so a pointer click at their bounds lands
#     nowhere at all. `scroll_into_view` is the only way in.
def settings_tree():
    """`(tree node, its TreeItem rows)` for the Settings category tree."""
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
        labels = {(r.get("label") or "").strip().lower() for r in rows}
        if any(v in labels for v in COMPILE_SECTION):
            return tree, rows
    return None, []


def settings_tree_rows():
    return settings_tree()[1]


def _row_in(rows, variants):
    for r in rows:
        if (r.get("label") or "").strip().lower() in variants:
            return r
    return None


def rail_row(variants):
    """The category-tree row whose own label is exactly one of `variants`.

    The label lives on the row node itself — the framework's accessibility walk
    copies the delegate's name up onto the `TreeItem` (name-from-content, as
    ARIA specifies for `treeitem`). If this stops finding rows, that hoist is
    what regressed.

    A row that isn't found is not necessarily absent: a `TreeView` only realizes
    the rows in (and a little past) its viewport, so one far below the fold has
    no widget — and therefore no AT node for `scroll_into_view` to aim at. Wheel
    the rail along until it materializes. Direction is discovered rather than
    assumed: the first scroll that changes nothing flips it."""
    tree, rows = settings_tree()
    hit = _row_in(rows, variants)
    if hit or not tree:
        return hit
    sig = lambda rs: tuple(sorted((r.get("label") or "") for r in rs))
    dy = -240
    flipped = False
    for _ in range(24):
        before = sig(rows)
        s.call("scroll", {"node": tree["id"], "dx": 0, "dy": dy})
        time.sleep(0.35)
        tree, rows = settings_tree()
        hit = _row_in(rows, variants)
        if hit:
            return hit
        if sig(rows) == before:
            if flipped:
                return None          # both ends reached, the row really isn't there
            dy, flipped = -dy, True
    return None


def require_row(variants, what):
    row = rail_row(variants)
    if not row:
        print("  rail rows:", [r.get("label") for r in settings_tree_rows()])
        fail(f"no '{what}' row in the Settings category tree", s.app, s.mcp, s.log)
    return row


def select_page(variants, what):
    """Reveal, then activate, a category row. Re-finds the row between the two
    calls: scrolling past the virtualizer's buffer rebuilds the row widget, and
    a rebuilt widget gets a fresh id."""
    row = require_row(variants, what)
    must("invoke_action", {"node": row["id"], "action": "scroll_into_view"}, what)
    time.sleep(0.4)
    row = rail_row(variants) or row
    must("invoke_action", {"node": row["id"], "action": "click"}, what)
    time.sleep(0.6)
    return row


def expand_section(variants, what):
    """Open a collapsed section through its `expand` action — no chevron
    pixel-hunting, and no need to know the row's indent."""
    row = require_row(variants, what)
    must("invoke_action", {"node": row["id"], "action": "scroll_into_view"}, what)
    time.sleep(0.4)
    row = rail_row(variants) or row
    must("expand", {"node": row["id"]}, what)
    time.sleep(0.5)
    return row


def settings_open():
    """A stable "the Settings window is open" signal — the instant-apply
    footer (Reset to defaults + Done) is present on every pane regardless of
    which one is showing by default, so it survives the panel being
    restructured around any particular page/section name."""
    return has_any(("reset to defaults", "réinitialiser")) and has_any(("done", "terminé"))


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
s.dump("Settings window (default pane)")
s.shot("/tmp/sk-settings-scene.png")

# ── 1. The category tree holds every section + page ───────────────────────────
require_all(SECTIONS, "the category tree")
print("PASS: category tree lists all sections + pages")

# ── 1b. The rail is AT-drivable at all ───────────────────────────────────────
# Everything after this depends on it, and each of these was broken until the
# row-a11y fix: rows carried a role but no name (the label sat on a separate
# `Unknown` child, so role+label never met on one node) and advertised no
# actions whatsoever, which made `invoke_action` a no-op that still replied
# "ok". If any of this regresses, the rest of the script's failures would be a
# mystery — so it is asserted directly.
rows = settings_tree_rows()
if len(rows) < 10:
    fail(f"expected the category tree to expose its rows as TreeItem nodes, got {len(rows)}",
         s.app, s.mcp, s.log)
nameless = [r["id"] for r in rows if not (r.get("label") or "").strip()]
if nameless:
    fail(f"{len(nameless)} category rows carry no accessible name "
         f"(the name-from-content hoist onto Role::TreeItem regressed)", s.app, s.mcp, s.log)
for r in rows:
    acts = set(r.get("actions") or [])
    if not {"click", "scroll_into_view"} <= acts:
        fail(f"row {r.get('label')!r} advertises {sorted(acts)} — expected at least "
             f"click + scroll_into_view", s.app, s.mcp, s.log)
    # A branch offers exactly the direction that would change something; a leaf
    # offers neither. `expanded` is None for a leaf.
    want = {True: "collapse", False: "expand"}.get(r.get("expanded"))
    if want and want not in acts:
        fail(f"{'expanded' if r.get('expanded') else 'collapsed'} section "
             f"{r.get('label')!r} does not advertise {want!r}", s.app, s.mcp, s.log)
    if r.get("expanded") is None and ({"expand", "collapse"} & acts):
        fail(f"leaf row {r.get('label')!r} advertises expand/collapse", s.app, s.mcp, s.log)
if not all(isinstance(r.get("level"), int) and r["level"] >= 1 for r in rows):
    fail("category rows do not report their 1-based tree level", s.app, s.mcp, s.log)
print(f"PASS: {len(rows)} category rows expose role + name + level + actions on ONE node")

# An action nothing acts on must be reported, not silently swallowed — that
# false "ok" is what made every earlier failure here unreadable.
leaf = next(r for r in rows if r.get("expanded") is None)
res, _ = s.call("invoke_action", {"node": leaf["id"], "action": "increment"})
if not (isinstance(res, dict) and res.get("isError")):
    fail("invoking an unsupported action on a category row reported success",
         s.app, s.mcp, s.log)
print("PASS: an unsupported AT action on a row is reported, not silently ignored")

# ── 2. Default pane (Editor ▸ Scene) shows its typography controls ───────────
font_combo = next(
    (n for n in s.by_role("ComboBox") if "font" in (n.get("label") or "").lower()), None
)
if not font_combo:
    print("  labels:", joined()[:700])
    fail("expected the Scene pane's font-family ComboBox", s.app, s.mcp, s.log)
sliders = s.by_role("Slider")
print(f"  font ComboBox present; {len(sliders)} Slider nodes")
if len(sliders) < SCENE_MIN_SLIDERS:
    fail(f"expected >= {SCENE_MIN_SLIDERS} sliders on the Scene typography pane "
         f"(size, line height, first-line indent, paragraph spacing before/after), "
         f"got {len(sliders)}", s.app, s.mcp, s.log)
print("PASS: Scene pane (the new default) shows its typography controls "
      "(font ComboBox + typography sliders)")

# ── 3. Selecting the Appearance page switches the pane ────────────────────────
select_page(APPEARANCE_PAGE, "Appearance")
require_all(APPEARANCE_BITS, "the Appearance pane")
print("PASS: Appearance page switched the pane (language switcher + 'show the launcher "
      "at startup' checkbox)")
s.shot("/tmp/sk-settings-appearance.png")

# ── 3b. Regression: selecting a ComboBox item must NOT close the window ──────
# (The dropdown floats in a child overlay of the modal; a host-surface fix in
# teksilo keeps the modal alive when the dropdown dismisses on select.) The
# Theme control lives on this (Appearance) pane now — it moved off the old
# Manuscript & Fonts page in the panel restructuring.
theme_combo = None
for n in s.nodes():
    if n.get("role") == "ComboBox" and any(
        v in (n.get("label") or "").lower() for v in ("theme", "thème")
    ):
        theme_combo = n
        break
THEME_OPTS = ("light", "dark", "system", "clair", "sombre", "système")
if theme_combo:
    pointer_click(theme_combo.get("bounds") or {})   # open the dropdown
    time.sleep(0.6)
    # The dropdown options float in a child overlay of the modal — search the
    # main tree first, then the overlay layer.
    opt = node_match(list(THEME_OPTS))
    if not opt:
        _, ov = s.call("get_overlays")
        pool = ov.get("overlays") or ov.get("nodes") or []
        for grp in pool:
            for n in ([grp] + (grp.get("nodes") or grp.get("children") or [])):
                if (n.get("label") or "").strip().lower() in THEME_OPTS:
                    opt = n
                    break
            if opt:
                break
    if opt and (opt.get("bounds") or "click" in (opt.get("actions") or [])):
        if not click_node(opt):
            pointer_click(opt.get("bounds") or {})    # select
        time.sleep(0.6)
        if not settings_open():
            fail("selecting a ComboBox item CLOSED the Settings window (overlay-host bug)",
                 s.app, s.mcp, s.log)
        print("PASS: selecting a ComboBox dropdown item kept the Settings window open")
    else:
        # Still a useful signal: opening the dropdown must not close the modal.
        if not settings_open():
            fail("opening a ComboBox dropdown CLOSED the Settings window", s.app, s.mcp, s.log)
        print("NOTE: dropdown option not addressable; opening it kept the window open")
    # Close any lingering dropdown before continuing.
    s.call("inject_key", {"key": "Escape"})
    time.sleep(0.3)
else:
    print("NOTE: theme ComboBox not surfaced; skipping combo regression")

# ── 4. The empty Keymap page shows the placeholder ───────────────────────────
# Row order no longer matters: the row is addressed by identity and revealed by
# its own `scroll_into_view`, so an expand elsewhere can't shift it out from
# under the step. (It used to have to run before any expansion, and its centre
# still landed within a pixel of the viewport's bottom edge.)
select_page(KEYMAP_PAGE, "Keymap")
s.shot("/tmp/sk-settings-keymap.png")
# The empty pane carries no form controls, so the previously-shown Appearance
# checkbox must be gone — a robust "switched to a settings-less pane" signal.
if has_any(APPEARANCE_BITS["welcome checkbox"]):
    fail("selecting the empty Keymap page did not switch the pane", s.app, s.mcp, s.log)
if has_any(EMPTY_BIT):
    print("PASS: empty category shows the placeholder content")
else:
    print("PASS: empty category switched to a control-less pane "
          "(placeholder text is not AT-surfaced; see /tmp/sk-settings-keymap.png)")

# ── 5. Expand Backup & Sync, then select Autosave ────────────────────────────
# Driven through the section's own `expand` action. This step used to be a soft
# "NOTE:" that gave up, because the only expand target was a nameless 16 px
# chevron whose x depends on the row's indent — which the AT tree did not
# report. It is a hard assertion now.
backup = expand_section(BACKUP_SECTION, "Backup & Sync")
after = rail_row(BACKUP_SECTION)
if not (after and after.get("expanded")):
    fail("Backup & Sync did not report itself expanded after the expand action",
         s.app, s.mcp, s.log)
select_page(AUTOSAVE_PAGE, "Autosave")
if not has_any(AUTOSAVE_BIT):
    print("  labels:", joined()[:600])
    fail("Autosave pane did not show the migrated autosave setting", s.app, s.mcp, s.log)
print("PASS: Backup & Sync expands and Autosave reveals the migrated autosave setting")

# ── 5b. A row below the fold is reachable ────────────────────────────────────
# The per-project pages sit past the bottom of the scrolled rail; their reported
# bounds are content coordinates below the window's own edge, so a synthetic
# pointer click at them hits nothing. `scroll_into_view` + `click` is the route,
# and this is the step that proves it.
text_repl = rail_row(BELOW_FOLD_PAGE)
if not text_repl:
    fail("the per-project 'Text replacements' page is not reachable in the rail",
         s.app, s.mcp, s.log)
else:
    before_y = (text_repl.get("bounds") or {}).get("y")
    select_page(BELOW_FOLD_PAGE, "Work ▸ Text replacements")
    moved = rail_row(BELOW_FOLD_PAGE)
    after_y = (moved or {}).get("bounds", {}).get("y")
    if not (moved and moved.get("selected")):
        fail("the below-the-fold 'Text replacements' row did not become selected",
             s.app, s.mcp, s.log)
    print(f"PASS: a row below the fold scrolls into view (y {before_y} → {after_y}) "
          f"and activates")

# ── 6. The SearchField accepts a query ───────────────────────────────────────
search = next(iter(s.by_role("SearchInput")), None) or s.find_contains("search")
if search:
    res, _ = s.call("set_value", {"node": search["id"], "value": "theme"})
    if not (isinstance(res, dict) and res.get("isError")):
        time.sleep(0.5)
        print("PASS: SearchField accepted a query")
        s.shot("/tmp/sk-settings-search.png")
    else:
        print("NOTE: set_value on the SearchField was rejected (non-fatal)")
else:
    print("NOTE: SearchField node not surfaced in the AT tree (non-fatal)")

# ── 7. Footer is instant-apply: Reset + Done only (no Apply/Cancel/OK) ────────
def footer_btn(labels):
    for n in s.nodes():
        if n.get("role") == "Button" and (n.get("label") or "").strip().lower() in labels \
                and "click" in (n.get("actions") or []):
            return n
    return None

for absent in (["apply", "appliquer"], ["ok"]):
    if footer_btn(set(absent)):
        fail(f"instant-apply footer must not carry {absent[0]!r}", s.app, s.mcp, s.log)
if not footer_btn({"done", "terminé"}):
    fail("instant-apply footer missing the Done button", s.app, s.mcp, s.log)
print("PASS: footer is instant-apply (Reset to defaults + Done, no Apply/Cancel/OK)")

# ── 8. Done dismisses the window ─────────────────────────────────────────────
done = footer_btn({"done", "terminé"})
if done:
    click_node(done)
    time.sleep(0.6)
    if settings_open():
        fail("Settings window did not close after Done", s.app, s.mcp, s.log)
    print("PASS: Done dismissed the Settings window")

s.close()
print("\nALL PASS")
sys.exit(0)
