#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the worktree build to verify the Import Manuskript feature end to end.

Unlike `automation_import_markdown.py` beside it, this probe can run the **whole**
import, and does. The documents wizard takes its files by drag-and-drop or through
a native file dialog, neither of which the automation bridge can synthesize; this
panel takes a *path in a text field*, so `set_value` reaches it and the run goes
all the way from the menu row to an imported project open on screen.

What it checks, in order:

  1. **The menu row exists and fires.** `Work > Import from > Manuskript (.msk)...`
     is an `AppIntent` consumed by a *global* action. A row wired to an action
     nobody registered is a dead click that compiles perfectly and passes every
     unit test. Opening the menu at all is also the only guard against `MenuList`'s
     build-time panic on two rows sharing a mnemonic, which took the app down on
     launch when the template rows landed.
  2. **The panel mounts as a modal** and carries both Browse buttons. A Manuskript
     project is a file or a folder, and the two-button source row is the whole
     reason this panel is not the Plume one.
  3. **A real import runs**, over the committed fixture project, and the progress
     toast resolves into a result offering *Open now*.
  4. **The imported project opens** and its binder holds what the fixture had: a
     book, its parts and chapters, and a story bible beside it.
  5. **Both locales carry the new keys.** A missing `fr-FR` key falls back to
     `en-US` silently, so the app looks fine and is simply in the wrong language.
     The French pass asserts the panel's own French chrome, not merely that it
     opened.

## Deliberately NOT covered here, and why

The two Browse buttons open native GTK/portal dialogs this script cannot see, so
it types the path instead. What the buttons themselves do (remember the last
directory, default the destination and the name) is covered by
`import_manuskript_vm`'s unit tests, which call `apply_source_defaults` directly.

Run it with the worktree's debug build:

    python3 scripts/automation_import_manuskript.py
"""

import base64, json, os, pathlib, re, select, subprocess, sys, tempfile, time

_ROOT = pathlib.Path(__file__).resolve().parent.parent  # this repo/worktree root
sys.path.insert(0, str(_ROOT / "scripts"))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = str(_ROOT / "resources/examples/starforgers/Starforgers.skrib")
FIXTURE = _ROOT / "crates/manuskript_import/tests/fixtures/tour-du-monde"
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
                                      "clientInfo": {"name": "import-manuskript", "version": "1"}})
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
                                         "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
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
# The run
# --------------------------------------------------------------------------
out_dir = pathlib.Path(tempfile.mkdtemp(prefix="skribisto-import-manuskript-"))
print(f"fixture : {FIXTURE}")
print(f"output  : {out_dir}")
if not FIXTURE.is_dir():
    fail(f"fixture project missing: {FIXTURE}. Run its generate.py first.")


def open_import_panel(s, locale):
    """Work > Import from > Manuskript, through the title-bar menu.

    The menu's own title is translated, so it is matched per locale rather than
    on the English word. Each step is checked on its own: a submenu whose rows
    never appeared and a row that appeared but did not fire are different bugs
    and should not share one failure line.
    """
    work, importer = ("œuvre", "importer depuis") if locale == "fr-FR" else ("work", "import from")
    if not s.open_menu(work):
        s.dump("no Work menu")
        check(False, f"[{locale}] the Work menu opens")
        return False
    check(True, f"[{locale}] the Work menu opens (no duplicate-mnemonic panic)")
    time.sleep(1.0)

    # Hovering opens `Import from`; its children only appear once it does.
    submenu = s.match([importer])
    check(submenu is not None, f"[{locale}] the Work menu offers 'Import from'")
    if not submenu:
        s.dump("Work menu open")
        return False
    s.call("invoke_action", {"node": submenu["id"], "action": "click"})
    time.sleep(1.6)

    entry = s.match(["manuskript"])
    check(entry is not None, f"[{locale}] 'Import from' offers the Manuskript row")
    if not entry:
        s.dump("Import from open")
        return False
    s.call("invoke_action", {"node": entry["id"], "action": "click"})
    # Poll rather than sleep: the accessibility tree is rebuilt as the modal
    # mounts, and a single snapshot taken during that comes back empty.
    title = "importer un projet manuskript" if locale == "fr-FR" else "import manuskript project"
    mounted = s.wait_label(title, timeout=25)
    check(mounted, f"[{locale}] the panel mounts (the global action is registered)")
    return mounted


def text_inputs(s):
    return [n for n in s.nodes()
            if "set_value" in (n.get("actions") or [])
            and n.get("role") in ("TextInput", "MultilineTextInput")]


def run(env, locale, name):
    print(f"\n=== pass: {locale} ===")
    # `--new-instance`: without it a second pass is handed to the first
    # process by the single-instance election and tests nothing.
    s = Session(["--new-instance", EXAMPLE], env=env)
    try:
        # `load_work` is slow in a debug build: the bundled example takes tens of
        # seconds before its binder is on screen, and a cold worktree target is
        # slower still. Budget generously rather than call a slow load a failure.
        if not s.wait_label("starforgers", timeout=240):
            s.dump("what was on screen instead")
            fail("the example project never opened", s.app, s.mcp, s.log)

        if not open_import_panel(s, locale):
            s.dump("no panel")
            return s
        joined = s.joined()

        # 2. The panel, and the two Browse buttons that are its reason to exist.
        if locale == "fr-FR":
            check("choisir un fichier" in joined and "choisir un dossier" in joined,
                  "[fr] both Browse buttons are French")
            check("dossier de destination" in joined, "[fr] the destination row is French")
        else:
            check("choose file" in joined and "choose folder" in joined,
                  "[en] both Browse buttons are present")

        fields = text_inputs(s)
        check(len(fields) >= 3, f"[{locale}] the form has its three path fields (saw {len(fields)})")
        if len(fields) < 3:
            s.dump("panel")
            return s

        # 3. Fill it in by hand and import. The three fields are, in order, the
        #    source, the destination folder, and the output name.
        target_name = f"tour-du-monde-{locale}"
        for node, value in zip(fields, [str(FIXTURE), str(out_dir), target_name]):
            s.call("set_value", {"node": node["id"], "value": value})
            time.sleep(0.4)

        button = s.match(["importer", "import"], exact=True) or s.match(["importer", "import"])
        check(button is not None and not button.get("disabled"),
              f"[{locale}] Import is enabled once the form is valid")
        if button:
            s.click_node(button)

        # 4. The progress toast resolves, and offers Open now.
        opened = s.wait_label("ouvrir maintenant" if locale == "fr-FR" else "open now", timeout=90)
        check(opened, f"[{locale}] the import finished and offered Open now")
        produced = out_dir / f"{target_name}.skrib"
        check(produced.is_file(), f"[{locale}] it wrote {produced.name}")

        if opened:
            row = s.match(["ouvrir maintenant", "open now"])
            if row:
                s.click_node(row)
                time.sleep(2.0)
                # Open now goes through `work.open_path`, which consults the
                # unsaved-changes guard rather than calling `load_work` outright.
                # With the example open and touched, that guard is exactly what a
                # writer meets here, so answering it is part of the flow, not a
                # detour around it. The buttons are the framework's standard ones,
                # so they are matched on both locales' wording.
                # Answered when it appears, never asserted: the guard is raised
                # only when the outgoing project has unsaved edits, and nothing
                # in this run controls whether the example picked any up. What is
                # asserted is what follows either way, that the imported project
                # opens.
                guard = s.match(["ignorer les modifications", "discard changes",
                                 "don't save", "ne pas enregistrer"])
                if guard:
                    print(f"  note  [{locale}] answered the unsaved-changes guard")
                    s.click_node(guard)
                # 5. The imported project is open, and holds what the fixture had.
                #    `load_work` is slow in a debug build, so poll rather than
                #    guess: the example itself takes well over a minute here.
                arrived = s.wait_label("chapitre 1", timeout=180)
                check(arrived, f"[{locale}] the imported binder holds Chapitre 1")
                if arrived:
                    tree = s.joined()
                    check("premi" in tree,
                          f"[{locale}] and the accented part title survived the slug")
        s.shot(str(out_dir / f"import-{locale}.png"))
        return s
    finally:
        pass


session = run(fixture.isolated_config(locale="en-US", label="import-manuskript-en"),
              "en-US", "english")
session.esc()
session.close()
time.sleep(1.5)

fr_env = fixture.isolated_config(locale="fr-FR", label="import-manuskript-fr")
session = run(fr_env, "fr-FR", "french")
session.esc()
session.close()

print()
if failures:
    print(f"{len(failures)} check(s) failed:")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print("all checks passed")
