#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the worktree build to verify the Import documents feature's doors end to end.

Three things a headless test cannot reach, each of which has already been a real bug
class in this codebase:

  1. **The menu row exists and fires.** `Work > Import from > Documents...` is an
     `AppIntent` consumed by a *global* action whose view-model is threaded per window.
     A row wired to an action nobody registered is a dead click that compiles perfectly
     and passes every unit test. Opening the menu at all is also the only guard against
     `MenuList`'s build-time panic on two items sharing a mnemonic — the failure that
     took the app down on launch when the template rows landed.
  2. **The wizard mounts as a modal.** `ModalPresentation::InTree` from a title-bar
     overlay has its own history in this repo (a modal opened from a menu needed a
     window-level presentation fix in teksilo).
  3. **Both locales carry the new keys.** A missing `fr-FR` key falls back to `en-US`
     silently — the app looks fine and is simply in the wrong language. The French pass
     below asserts the wizard's own French chrome, not merely that it opened.

## Deliberately NOT covered here, and why

The full drop -> review -> retype -> Import -> Undo loop. The automation bridge has no
file-drop injection (its tool set is snapshot/inject_key/inject_pointer/invoke_action/
set_value/... — see `teksilo-automation/src/mcp_schema.rs`), and both ways into the file
list are a real drag-and-drop or a *native* file dialog, neither of which the bridge can
synthesize. Driving "Browse..." would open a GTK/portal dialog this script cannot see.

That loop is not untested, it is tested where it can be:

  * `teksilo_ui::view_models::import_document::tests::a_real_analysis_lands_a_plan_on_the_review_step`
    runs two Markdown files on disk through the real `analyze_document_import` backend
    and asserts the plan, the indents, the level rules and the break count.
  * `skribisto-import-management`'s ANALYSE -> APPLY integration test asserts the store
    tree the apply actually creates.
  * `panels::import_document::tests` mount the review tree and the destination picker.

A probe that faked its own staging would be worse than none — see the same note in
`automation_note_templates.py` for the precedent."""

import base64, json, os, pathlib, re, select, subprocess, sys, tempfile, time

_ROOT = pathlib.Path(__file__).resolve().parent.parent  # this repo/worktree root
sys.path.insert(0, str(_ROOT / "scripts"))
import automation_fixture as fixture  # noqa: E402

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
    def __init__(self, args, env=None):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT,
                                    env={**os.environ, **(env or {})})
        sock = tok = None
        deadline = time.time() + 25
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
            fail("no bridge socket within 25s", self.app, None, self.log)
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
                                      "clientInfo": {"name": "import-md", "version": "1"}})
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

    def joined(self):
        return " | ".join(self.labels()).lower()

    def wait_label(self, substr, timeout=25):
        end = time.time() + timeout
        while time.time() < end:
            if substr.lower() in self.joined():
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

    def dump(self, title, limit=90):
        print(f"--- AT tree: {title} ---")
        c = 0
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            if lab:
                print(f"  [{n.get('role')}] {lab!r}"
                      + ("  (disabled)" if n.get("disabled") else ""))
                c += 1
                if c >= limit:
                    print("  ..."); break

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"screenshot -> {path}")
                return

    def esc(self, times=4):
        for _ in range(times):
            self.call("inject_key", {"key": "Escape"})
            time.sleep(0.3)

    def open_menu(self, title):
        """Open the title-bar hamburger, then one of its menus.

        Menu titles live in an overlay whose reported bounds can be negative, so
        they are activated through `invoke_action` rather than a pointer click.
        """
        ham = self.match(["menu"], rail_only=True, exact=True)
        if not ham:
            return False
        self.click_node(ham)
        time.sleep(1.0)
        top = self.match([title], exact=True)
        if not top:
            return False
        self.call("invoke_action", {"node": top["id"], "action": "click"})
        time.sleep(1.2)
        return True

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


failures = []


def check(cond, msg):
    print(("  ok   " if cond else "  FAIL ") + msg)
    if not cond:
        failures.append(msg)


# --------------------------------------------------------------------------
# The documents themselves. Written even though nothing can feed them to the
# wizard through the UI, because the *paths* are what a human reproducing this
# script by hand needs — the whole point of a worked example.
# --------------------------------------------------------------------------
docs = pathlib.Path(tempfile.mkdtemp(prefix="skribisto-import-md-"))
(docs / "01-the-arrival.md").write_text(
    "# The Salt Road\n"
    "\n"
    "## Chapter One\n"
    "\n"
    "The tide came in early that year.\n"
    "\n"
    "* * *\n"
    "\n"
    "By dusk the harbour was a different place.\n",
    encoding="utf-8",
)
(docs / "02-the-crossing.md").write_text(
    "## Chapter Two\n"
    "\n"
    "Nobody spoke on the crossing.\n",
    encoding="utf-8",
)
print(f"sample documents in {docs}")
print("  (drop these on the wizard by hand to exercise the review step —")
print("   the bridge cannot synthesize a file drop; see this script's docstring)")

fixture.assert_no_running_instance(SKRIBISTO)

# ==========================================================================
# 1. The in-project door: Work > Import from > Documents...
# ==========================================================================
print("\n== launch with the bundled example ==")
s = Session(["--new-instance", EXAMPLE])
if not s.wait_label("starforgers", timeout=40):
    fail("example did not load", s.app, s.mcp, s.log)
print("example loaded.")

print("\n== Work menu ==")
check(s.open_menu("work"), "Work menu opens (no duplicate-mnemonic panic)")
s.shot("/tmp/import-md-work-menu.png")

# The submenu row itself. Hovering opens `Import from`; its children only appear
# once it does.
imp = s.match(["import from"])
check(imp is not None, "Work menu offers 'Import from'")
if imp:
    s.call("invoke_action", {"node": imp["id"], "action": "click"})
    time.sleep(1.2)
    row = s.match(["documents (markdown"])
    check(row is not None, "'Import from' offers 'Documents (Markdown, text)...'")
    check(row is not None and row.get("disabled") is not True,
          "the Documents row is live with a project open")
    if row:
        s.call("invoke_action", {"node": row["id"], "action": "click"})
        time.sleep(1.5)

print("\n== the wizard ==")
opened = s.wait_label("import documents", timeout=10)
check(opened, "the Import documents wizard opens (the global action is registered)")
if not opened:
    s.dump("wizard did not open")
s.shot("/tmp/import-md-wizard.png")

# The dialog node itself. `ctx.present_modal` does not wrap a hand-drawn panel in
# a `ModalContainer`, so without `ImportDocumentPanel::accessibility` there is no
# `Role::Dialog` node at all and a screen reader meets an unnamed surface that has
# appeared over everything. This is the assertion that pins that fix.
dlg = next((n for n in s.nodes()
            if (n.get("role") or "").lower() == "dialog"
            and "import documents" in (n.get("label") or "").lower()), None)
check(dlg is not None, "the wizard announces itself as a named dialog")

joined = s.joined()
check("drop documents here" in joined, "step one offers a drop zone")
check("browse" in joined, "…and a Browse button for people who don't drag")

# Greying is the whole affordance for "you have not chosen anything yet": with no
# files, Next must be dead. Back is a different story — the `Stepper` *hides* it
# where it would have nowhere to go rather than greying it, so on step one there
# is no Back button at all. This used to assert a greyed one and had been failing
# against the framework's actual behaviour.
nxt = s.match(["next"], exact=True)
back = s.match(["back"], exact=True)
cancel = s.match(["cancel"], exact=True)
check(nxt is not None and nxt.get("disabled") is True,
      "Next is greyed with no files chosen")
check(back is None, "Back is absent on the first step (the Stepper hides it)")
check(cancel is not None and cancel.get("disabled") is not True,
      "Cancel is live")

# It must close, and the app must survive it.
if cancel:
    s.click_node(cancel)
    time.sleep(1.2)
check("drop documents here" not in s.joined(), "Cancel closes the wizard")
check(s.app.poll() is None, "the app survives the wizard")
s.esc()
s.close()
time.sleep(1.5)

# ==========================================================================
# 2. The cold-start door: the Launcher's "From documents..."
# ==========================================================================
print("\n== the Launcher's cold-start door ==")
env = fixture.isolated_config(locale="en-US", label="import-md-launcher")
s = Session(["--new-instance"], env=env)
launcher = s.wait_label("recent works", timeout=30) or s.wait_label("new work", timeout=10)
check(launcher, "the Launcher opens with no project")
if launcher:
    s.shot("/tmp/import-md-launcher.png")
    check("from documents" in s.joined(),
          "the Launcher offers 'From documents…' beside Open / New Work")
else:
    s.dump("launcher did not open")
s.close()
time.sleep(1.5)

# ==========================================================================
# 3. French. A missing fr-FR key falls back to en-US in silence.
# ==========================================================================
print("\n== fr-FR ==")
env = fixture.isolated_config(locale="fr-FR", label="import-md-fr")
s = Session(["--new-instance", EXAMPLE], env=env)
if not s.wait_label("starforgers", timeout=40):
    fail("example did not load under fr-FR", s.app, s.mcp, s.log)

if s.open_menu("œuvre"):
    imp = s.match(["importer depuis"])
    check(imp is not None, "the French Work menu offers 'Importer depuis'")
    if imp:
        s.call("invoke_action", {"node": imp["id"], "action": "click"})
        time.sleep(1.2)
        row = s.match(["documents (markdown, texte)"])
        check(row is not None, "…and the French Documents row (not the English fallback)")
        if row:
            s.call("invoke_action", {"node": row["id"], "action": "click"})
            time.sleep(1.5)
    joined = s.joined()
    check("importer des documents" in joined, "the wizard's French dialog name")
    check("déposez les documents ici" in joined, "…its French drop zone")
    check("parcourir" in joined, "…its French Browse")
    check("suivant" in joined, "…its French Next")
    check("annuler" in joined, "…its French Cancel")
    s.shot("/tmp/import-md-wizard-fr.png")
    if "importer des documents" not in joined:
        s.dump("fr wizard")
else:
    check(False, "the French Work menu opens")
s.esc()
s.close()

print()
if failures:
    print(f"{len(failures)} FAILURE(S):")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("all checks passed.")
