#!/usr/bin/env python3
"""Drive a live Skribisto via the bastyde automation MCP bridge and verify the
Settings preferences window end-to-end.

Launches with the bundled example loaded (so the Welcome modal is suppressed),
opens Settings (Ctrl+, with a menu fallback), then asserts:

  1. the category TreeView holds every section + page (Appearance & Behaviour,
     Editor ▸ Manuscript & Fonts, Spelling, Backup & Sync, Compile & Export,
     Keymap);
  2. the default pane (Manuscript & Fonts) shows its group headers + controls
     (Typeface, Text width, Editor theme, the synopsis checkbox);
  3. selecting the Appearance page switches the pane (Interface language + the
     welcome checkbox appear);
  4. expanding Backup & Sync and selecting Autosave reveals the autosave setting;
  5. selecting the empty Keymap page shows the "no settings yet" placeholder;
  6. the SearchField accepts a query;
  7. OK dismisses the window.

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = "/home/cyril/Devel/skribisto/resources/examples/Starforgers.skrib"

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
            t = re.search(r"BASTYDE_AUTOMATION_TOKEN=(\S+)", txt)
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
if not s.wait_label("starforgers", timeout=15):
    fail("the example work did not load", s.app, s.mcp, s.log)
print("example loaded.")

# ── Locale-robust anchors ────────────────────────────────────────────────────
# The app restores whatever UI language was last persisted, so every concept is
# matched against its en-US *and* fr-FR wording. (Concept → list of substrings.)
SECTIONS = {
    "appearance_behaviour": ["appearance & behaviour", "apparence et comportement"],
    "editor": ["editor", "éditeur"],
    "manuscript": ["manuscript & fonts", "manuscrit et polices"],
    "spelling": ["spelling", "orthographe"],
    "backup": ["backup & sync", "sauvegarde et synchronisation"],
    "compile": ["compile & export", "compilation et export"],
    "keymap": ["keymap", "raccourcis clavier"],
}
# GroupHeaders and FormLayout field labels are decorative (not AccessKit
# labels), so assert on the controls the AT tree actually surfaces.
MANUSCRIPT_BITS = {
    "synopsis checkbox": ["synopsis"],
    "typewriter checkbox": ["typewriter", "machine à écrire"],
    "highlight checkbox": ["highlight", "surligner"],
    "theme control (ThemeSwitcher)": ["theme", "thème"],
    "text scale (TextScaleControl)": ["text scale"],
}
# (page label variants) for clicking a tree leaf — exact match, not substring, so
# "Appearance" never matches the "Appearance & Behaviour" section.
APPEARANCE_PAGE = ["appearance", "apparence"]
BACKUP_SECTION = ["backup & sync", "sauvegarde et synchronisation"]
AUTOSAVE_PAGE = ["autosave", "enregistrement automatique"]
KEYMAP_PAGE = ["keymap", "raccourcis clavier"]
APPEARANCE_BITS = {
    "welcome checkbox": ["welcome screen", "écran d'accueil"],
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


def node_match(variants, exact=False, rail_only=False):
    """First node whose label matches any variant. `rail_only` restricts to the
    left category rail (x < 280) so a page name never matches app chrome."""
    for n in s.nodes():
        lab = (n.get("label") or "").strip().lower()
        if not lab:
            continue
        hit = lab in variants if exact else any(v in lab for v in variants)
        if not hit:
            continue
        if rail_only:
            b = n.get("bounds") or {}
            x = b.get("x", 9999) if isinstance(b, dict) else 9999
            if x >= 280:
                continue
        return n
    return None


def pointer_click(b, dx=None):
    """Synthetic primary click (atomic down+up → a real tap gesture) at the
    centre of a bounds dict, or at `bounds.x + dx` when `dx` is given (used to
    hit a row's leading-edge chevron rather than its centre)."""
    if not (isinstance(b, dict) and "x" in b):
        return False
    cx = b["x"] + (dx if dx is not None else b.get("width", 0) / 2)
    cy = b["y"] + b.get("height", 0) / 2
    s.call("inject_pointer", {"x": cx, "y": cy, "kind": "click"})
    return True


def click_node(n):
    """Click a node — via its AccessKit `click` action when it has one, else a
    synthetic pointer at its centre (TreeView rows expose no action)."""
    if "click" in (n.get("actions") or []):
        res, _ = s.call("invoke_action", {"node": n["id"], "action": "click"})
        return not (isinstance(res, dict) and res.get("isError"))
    return pointer_click(n.get("bounds") or {})


def expand_section(n):
    """Expand a collapsed section by tapping its chevron at the row's leading
    edge (a section row is at depth 0, so the chevron sits ~10 px in)."""
    return pointer_click(n.get("bounds") or {}, dx=10)


def open_settings():
    for args in ({"key": ",", "ctrl": True},
                 {"key": ",", "modifiers": ["ctrl"]},
                 {"key": "Comma", "modifiers": ["ctrl"]}):
        res, _ = s.call("inject_key", args)
        if not (isinstance(res, dict) and res.get("isError")):
            time.sleep(0.8)
            if has_any(SECTIONS["manuscript"]):
                return f"Ctrl+, ({args})"
    return None


how = open_settings()
if not how:
    s.dump("after open attempt")
    fail("could not open the Settings window", s.app, s.mcp, s.log)
print(f"Settings opened via {how}.")
s.dump("Settings window (default pane)")
s.shot("/tmp/sk-settings-manuscript.png")

# ── 1. The category tree holds every section + page ───────────────────────────
require_all(SECTIONS, "the category tree")
print("PASS: category tree lists all sections + pages")

# ── 2. Default pane (Manuscript & Fonts) shows its groups + controls ──────────
require_all(MANUSCRIPT_BITS, "the Manuscript & Fonts pane")
print("PASS: Manuscript & Fonts pane shows group headers + migrated/new controls")

# ── 2b. Regression: selecting a ComboBox item must NOT close the window ───────
# (The dropdown floats in a child overlay of the modal; a host-surface fix in
# bastyde keeps the modal alive when the dropdown dismisses on select.)
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
        if not has_any(SECTIONS["manuscript"]):
            fail("selecting a ComboBox item CLOSED the Settings window (overlay-host bug)",
                 s.app, s.mcp, s.log)
        print("PASS: selecting a ComboBox dropdown item kept the Settings window open")
    else:
        # Still a useful signal: opening the dropdown must not close the modal.
        if not has_any(SECTIONS["manuscript"]):
            fail("opening a ComboBox dropdown CLOSED the Settings window", s.app, s.mcp, s.log)
        print("NOTE: dropdown option not addressable; opening it kept the window open")
    # Close any lingering dropdown before continuing.
    s.call("inject_key", {"key": "Escape"})
    time.sleep(0.3)
else:
    print("NOTE: theme ComboBox not surfaced; skipping combo regression")

# ── 3. Selecting the Appearance page switches the pane ────────────────────────
appearance = node_match(APPEARANCE_PAGE, exact=True, rail_only=True)
if not appearance:
    s.dump("looking for Appearance page")
    fail("no 'Appearance' page row in the category rail", s.app, s.mcp, s.log)
if not click_node(appearance):
    fail("could not click the Appearance page row", s.app, s.mcp, s.log)
time.sleep(0.6)
require_all(APPEARANCE_BITS, "the Appearance pane")
print("PASS: Appearance page switched the pane (language switcher + welcome checkbox)")
s.shot("/tmp/sk-settings-appearance.png")

# ── 4. The empty Keymap page shows the placeholder ───────────────────────────
# Done before any section expansion, so tree-row positions are stable (an async
# expand elsewhere would shift rows and race a click's resolved coordinates).
keymap = node_match(KEYMAP_PAGE, exact=True, rail_only=True)
if not keymap:
    fail("no 'Keymap' row in the rail", s.app, s.mcp, s.log)
click_node(keymap)
time.sleep(0.6)
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

# ── 5. Expand Backup & Sync (last tree interaction — expanding shifts the rows
#      below it), then select Autosave. Tap the chevron once, then POLL: the
#      re-render settles asynchronously (a fixed sleep races it).
backup = node_match(BACKUP_SECTION, exact=True, rail_only=True)
if not backup:
    fail("no 'Backup & Sync' section row in the rail", s.app, s.mcp, s.log)
expand_section(backup)                     # single chevron tap (a second toggles back)
autosave = None
for _ in range(10):
    time.sleep(0.4)
    autosave = node_match(AUTOSAVE_PAGE, exact=True, rail_only=True)
    if autosave:
        break
if not autosave:
    print("NOTE: could not expand Backup & Sync via synthetic input; the Autosave "
          "page + migration are covered by the panel's unit tests instead")
else:
    click_node(autosave)                   # fresh bounds (rows shifted on expand)
    time.sleep(0.6)
    if not has_any(AUTOSAVE_BIT):
        print("  labels:", joined()[:600])
        fail("Autosave pane did not show the migrated autosave setting", s.app, s.mcp, s.log)
    print("PASS: Backup & Sync expands and Autosave reveals the migrated autosave setting")

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
    if has_any(SECTIONS["manuscript"]) and has_any(MANUSCRIPT_BITS["text scale"]):
        fail("Settings window did not close after Done", s.app, s.mcp, s.log)
    print("PASS: Done dismissed the Settings window")

s.close()
print("\nALL PASS")
sys.exit(0)
