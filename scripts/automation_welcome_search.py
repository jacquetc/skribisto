#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto Launcher via the bastyde automation MCP bridge and
check the Welcome window's **search field**.

The bug under test: the Works pane's `SearchField` was bound to a `Signal<String>`
that nothing read (`WelcomePanel::search`, write-only). Typing moved the caret and
nothing else — the recent-works list never filtered.

What this asserts, in one Launcher session seeded with three recents
(Alpha / Beta / Gamma, most-recent-first):

  1. no query → all three rows are in the AccessKit tree;
  2. a query that matches nothing → NO rows, and the "no recent work matches"
     note replaces the list (distinct from the "no recent works yet" note — the
     recents did not disappear, they just don't match);
  3. clearing the field → all three rows are back;
  4. "beta" → exactly one row, and it is Beta;
  5. **clicking that one visible row opens Beta — not Alpha.** This is the sharp
     one. `ListView::on_activate` hands back an index into *what the view was
     given*; with a filter active that is the projection, not the MRU. Beta is
     visible row 0 but MRU row 1, so a handler that resolved the index against
     the unfiltered model would open Alpha — the top recent — and the writer
     would land in the wrong novel. The oracle is the open-registry lock file
     (`open_registry::claim`), which records the path actually loaded.

Recents are seeded on disk (a sandbox `recents.toml` pointing at 3 real .skrib
copies) rather than by opening projects, exactly as automation_welcome_keyboard.py
does: the list only shows *reachable* paths. XDG_RUNTIME_DIR is sandboxed too, so
the lock files read in step 5 are this run's and nobody else's.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"
EXAMPLE = "/home/cyril/Devel/skribisto/resources/examples/Starforgers.skrib"

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# Isolated XDG dirs — never touch the real ~/.config/skribisto. XDG_RUNTIME_DIR
# is deliberately NOT sandboxed: it is where the Wayland compositor's socket
# lives, and pointing it at a temp dir leaves winit with no compositor to talk
# to (the app panics before it ever opens a window). Step 5 therefore reads the
# open-registry locks out of the *real* runtime dir and keeps only the ones
# naming a project inside this run's sandbox — a Skribisto the user has open
# right now has locks there too.
_sandbox = tempfile.mkdtemp(prefix="skribisto_search_test_")
CONFIG = os.path.join(_sandbox, "config")
RUNTIME = os.environ.get("XDG_RUNTIME_DIR", "/tmp")
SANDBOX_ENV = {
    "XDG_CONFIG_HOME": CONFIG,
    "XDG_DATA_HOME": os.path.join(_sandbox, "data"),
    "HOME": _sandbox,
}

# 3 reachable projects. They are copies of the same example, so all three carry
# the title "Starforgers" *inside* the .skrib — which is precisely why step 5
# reads the lock file's path and not any on-screen title.
PROJECTS = []
_works = os.path.join(_sandbox, "works")
os.makedirs(_works, exist_ok=True)
for name in ("Alpha", "Beta", "Gamma"):
    dst = os.path.join(_works, f"{name}.skrib")
    shutil.copyfile(EXAMPLE, dst)
    PROJECTS.append((name, dst))

os.makedirs(os.path.join(CONFIG, "skribisto"), exist_ok=True)
with open(os.path.join(CONFIG, "skribisto", "recents.toml"), "w") as fh:
    fh.write("version = 1\n")
    for i, (name, path) in enumerate(PROJECTS):
        fh.write("\n[[items]]\n")
        fh.write(f'path = "{path}"\n')
        fh.write(f'title = "{name}"\n')
        # Descending: Alpha is the most recent, so the list reads Alpha, Beta,
        # Gamma — and Beta is MRU index 1, never 0. Step 5 depends on that.
        fh.write(f"last_opened_ms = {1_700_000_000_000 - i * 86_400_000}\n")
        fh.write("pinned = false\n")


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
    def __init__(self, args):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        env = {**os.environ, **SANDBOX_ENV}
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT, env=env)
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
                                      "clientInfo": {"name": "search-test", "version": "1"}})
            init = self._recv(timeout=4, fatal=False)
            if init is None and self.mcp.poll() is None:
                self.mcp.terminate()
                time.sleep(0.3)
        if init is None:
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
            t = text.strip()
            payload = json.loads(t) if (t.startswith("{") or t.startswith("[")) else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", []) if isinstance(p, dict) else []

    def labels(self):
        return [n.get("label") for n in self.nodes() if n.get("label")]

    def texts(self):
        """Every string the AT tree exposes — `label` (what a widget is called)
        *and* `value` (what it says). A plain `TextWidget` — the empty/no-match
        notes — carries its words in `value`, so a label-only scan misses them."""
        out = []
        for n in self.nodes():
            out += [n.get("label") or "", n.get("value") or ""]
        return [t for t in out if t]

    def has_text(self, substr):
        return substr.lower() in " | ".join(self.texts()).lower()

    def wait_label(self, substr, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            if self.has_text(substr):
                return True
            time.sleep(0.3)
        return False

    def click_node(self, n):
        """Activate a row the way the user does. `invoke_action` only works on a
        node that advertises `click`; a `ListView` row does not (it activates on
        a real pointer press — `ActivateOn::SingleClick`), so fall back to
        clicking its centre."""
        if "click" in (n.get("actions") or []):
            res, _ = self.call("invoke_action", {"node": n["id"], "action": "click"})
            if not (isinstance(res, dict) and res.get("isError")):
                return True
        b = n.get("bounds") or {}
        if "x" not in b:
            return False
        self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                     "y": b["y"] + b.get("height", 0) / 2,
                                     "kind": "click"})
        return True

    def role(self, role):
        return [n for n in self.nodes() if n.get("role") == role]

    def type_into(self, node_id, text):
        res, _ = self.call("type_text", {"node": node_id, "text": text})
        if isinstance(res, dict) and res.get("isError"):
            fail(f"type_text {text!r} errored: {res}", self.app, self.mcp, self.log)
        self.settle()

    def key(self, k, **mods):
        res, _ = self.call("inject_key", {"key": k, **mods})
        if isinstance(res, dict) and res.get("isError"):
            fail(f"inject_key {k} errored: {res}", self.app, self.mcp, self.log)
        self.settle()

    def settle(self):
        self.call("settle")
        time.sleep(0.35)

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                return
    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        try:
            self.app.wait(timeout=3)
        except Exception:
            self.app.kill()


def rows(s):
    """The recent-works rows, in visual order.

    `ListView` wraps each row in a `ListBoxOption`; the row's own text
    (`access_label_literal(title)` + `access_description_literal(path)`) lands on
    a `GenericContainer` child, so the title is read from the subtree, not the
    option itself.
    """
    ns = s.nodes()
    by_id = {n["id"]: n for n in ns}
    out = []
    for n in ns:
        if n.get("role") != "ListBoxOption":
            continue
        # Walk this option's subtree for the first labelled node.
        label = n.get("label")
        stack = list(n.get("children") or [])
        while stack and not label:
            c = by_id.get(stack.pop(0))
            if not c:
                continue
            label = c.get("label")
            stack.extend(c.get("children") or [])
        out.append({**n, "title": (label or "").strip()})
    return out


def titles(s):
    return [r["title"] for r in rows(s)]


def claimed_paths():
    """Project paths claimed as open (open-registry lock files), scoped to this
    run's sandbox — the shared runtime dir also holds the locks of any other
    Skribisto instance on this machine, and stale ones from earlier runs (whose
    sandbox path differs, so they cannot be confused with this run's)."""
    d = os.path.join(RUNTIME, "skribisto")
    found = []
    for f in os.listdir(d) if os.path.isdir(d) else []:
        if not f.endswith(".lock"):
            continue
        try:
            p = json.load(open(os.path.join(d, f)))["path"]
        except Exception:
            continue
        if p.startswith(_works):
            found.append(p)
    return found


def wait_claim(timeout=20):
    """Poll for the claim: opening a project is asynchronous (a project window is
    created, the work loads, *then* the path is claimed), so a fixed sleep either
    flakes or wastes time."""
    end = time.time() + timeout
    while time.time() < end:
        got = claimed_paths()
        if got:
            return got
        time.sleep(0.4)
    return []


def dump(s):
    print("--- AccessKit tree (role / label) ---")
    for n in s.nodes():
        print(f"  {n.get('role')}: {n.get('label')!r}")


def editor_of(s, node):
    """The editable `TextInput` inside a composite field (the `SearchField`
    publishes a `SearchInput` wrapper around one)."""
    by_id = {n["id"]: n for n in s.nodes()}
    stack = list(node.get("children") or [])
    while stack:
        c = by_id.get(stack.pop(0))
        if not c:
            continue
        if c.get("role") == "TextInput":
            return c["id"]
        stack.extend(c.get("children") or [])
    return node["id"]


failures = []
s = Session([])
# The nav TabBar's `access_label_literal` — the Launcher's landmark. (The
# "Recent Works" GroupHeader is decorative text and carries no AT label.)
if not s.wait_label("welcome sections"):
    dump(s)
    fail("no Launcher window (no 'Welcome sections' nav)", s.app, s.mcp, s.log)

# ── 1. No query: every recent work is listed ────────────────────────────────
before = titles(s)
if sorted(before) != ["Alpha", "Beta", "Gamma"]:
    dump(s)
    failures.append(f"1. seeded recents not all listed: {before}")
print(f"1. no query → rows {before}")

# The typing target is the SearchField's *editor*, not its `SearchInput`
# wrapper: keys go to the focused text widget, and focusing the wrapper leaves
# them nowhere. Take the `TextInput` in its subtree (falling back to the wrapper
# if the widget ever stops nesting one).
search_nodes = s.role("SearchInput")
if not search_nodes:
    dump(s)
    fail("no SearchInput in the tree — the Welcome search field is missing",
         s.app, s.mcp, s.log)
field = editor_of(s, search_nodes[0])

# ── 2. A query nothing matches: no rows, and the app says why ───────────────
s.type_into(field, "zzz")
after = titles(s)
if after:
    failures.append(f"2. 'zzz' matched nothing but {len(after)} row(s) remain: {after} "
                    "— THE SEARCH FIELD IS NOT WIRED TO THE LIST")
if not s.has_text("No recent work matches your search"):
    failures.append("2. no 'nothing matched' note — the region must not fall back "
                    "to 'No recent works yet' (the recents are still there)")
if s.has_text("No recent works yet"):
    failures.append("2. shows 'No recent works yet' for a failed search — "
                    "that says the recents are gone, and they are not")
print(f"2. 'zzz' → rows {after}, note shown = {s.has_text('No recent work matches')}")
s.shot("/tmp/sk-search-nomatch.png")

# ── 3. Clearing the field brings every row back ─────────────────────────────
s.key("A", ctrl=True)
s.key("Backspace")
restored = titles(s)
if sorted(restored) != ["Alpha", "Beta", "Gamma"]:
    failures.append(f"3. clearing the query did not restore the rows: {restored}")
print(f"3. cleared → rows {restored}")

# ── 4. A query that matches one row ─────────────────────────────────────────
s.type_into(field, "beta")
one = titles(s)
if one != ["Beta"]:
    failures.append(f"4. 'beta' should leave exactly [Beta], got {one}")
if s.has_text("No recent work matches your search"):
    failures.append("4. the 'nothing matched' note is showing over a live match")
print(f"4. 'beta' → rows {one}")
s.shot("/tmp/sk-search-beta.png")

# ── 5. Clicking the one visible row opens BETA, not the top recent ──────────
# Beta is visible row 0 but MRU row 1. An index resolved against the unfiltered
# model would open Alpha here.
r = rows(s)
if len(r) != 1:
    fail(f"5. cannot run: expected 1 visible row, got {titles(s)}", s.app, s.mcp, s.log)
if not s.click_node(r[0]):
    fail("5. could not click the row", s.app, s.mcp, s.log)

opened = wait_claim()
if not opened:
    failures.append("5. nothing was opened — the click did not activate the row")
elif not all(p.endswith("Beta.skrib") for p in opened):
    failures.append(f"5. WRONG PROJECT OPENED: {opened} — expected Beta.skrib. The "
                    "activated index was resolved against the unfiltered MRU "
                    "(Alpha is its row 0), not the filtered list the user clicked")
print(f"5. clicked the single 'beta' match → opened {opened}")
s.shot("/tmp/sk-search-opened.png")

s.close()
shutil.rmtree(_sandbox, ignore_errors=True)

print()
if failures:
    print("FAILURES:")
    for f in failures:
        print("  ✗", f)
    sys.exit(1)
print("PASS — the Welcome search filters the recents list, says so when nothing "
      "matches, restores on clear, and opens the row the user actually clicked.")
