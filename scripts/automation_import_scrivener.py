#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive the worktree's debug build through a whole Scrivener import, end to end.

The projects it imports are generated from `crates/scrivener_fixtures` into a scratch
folder (`write_fixtures`, an example of that crate): no real Scrivener project can enter
the repository. The wizard takes a typed path, so `set_value` reaches it and the run goes
from the menu row to the imported project open on screen and edited.

What it checks (D7-16, with the two checks D8-12 adds), in an English pass:

  1. The Launcher's *Create from…* offers *Scrivener (.scriv)…*, and the row mounts the
     wizard; later, a project window's *Work > Import from* does too.
  2. Typing the Scrivener 3 project's path and pressing Enter reads it: the card says
     which Scrivener saved it and how many documents it holds (8, 4 in the manuscript),
     and *Next* is enabled.
  3. Two binder files with nothing to tell them apart ask which one, with *Next* off
     until one is chosen; choosing *Draft B* reads it, and importing it writes its own
     title, *Arrival B*.
  4. Picking the copy Scrivener kept before an upgrade selects and reads the upgraded
     project, which the card names.
  5. Edits from an iPhone or iPad keep *Next* off until *Import it without those
     changes* is ticked.
  6. A Scrivener 1 for Mac project is refused, *Next* off.
  7. *Review* shows the binder, and sending a top-level folder to *Leave out* takes its
     items out of the preview.
  8. *Import* ends in a result offering *Open now* and *See report*; the `.skrib` is
     there and no `.importing` file is left.
  9. *See report* opens *About this import*: its groups, the writer's own choice, *Copy
     report*; a group it opens closed, and a list of items in it, open to their height
     when pressed (the run says whether its window had the focus); once it is closed,
     the command palette's *Show Scrivener Import Report* opens it again.
 10. *Open now* opens the project in a window of its own; its binder holds the Draft's
     titles, and in that window the report's items open their rows.
 11. Edit and reopen (D8-12): the scene holding `10:30:45`, `--`, straight quotes and a
     paragraph opening with `S.` is edited and saved, the app is closed and relaunched on
     the project, and the phrases are intact.
 12. Overwrite: importing again to the same name asks *Replace existing file?*; importing
     onto the project open in a window is refused.
 13. Cancel: a project of 3,000 items is imported and its notice's Cancel pressed; the
     import must stop, leaving no output and no `.importing` file.
 14. Every project written, the upgraded one read in step 4 included, is unchanged by all
     of the above (D8-12, I11).

Every session is closed on the way out, whatever ends the run: a failed check, an
exception, Ctrl+C.

A French pass then checks the French chrome, the help topic the wizard's *Help* opens,
and the report's French group titles, in the dark theme.

The two Browse buttons open native dialogs the bridge cannot see; what they do is
covered by the view-model's unit tests (`apply_pick`).

Run it, serially and alone, against the worktree's debug build:

    cargo build -p teksilo_ui
    python3 scripts/automation_import_scrivener.py
"""

import atexit, base64, hashlib, json, os, pathlib, re, select, shutil, subprocess, sys
import tempfile, time, zipfile

_ROOT = pathlib.Path(__file__).resolve().parent.parent  # this repo/worktree root
sys.path.insert(0, str(_ROOT / "scripts"))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
OUT = pathlib.Path(tempfile.mkdtemp(prefix="skribisto-import-scrivener-",
                                    dir=fixture.SCRATCH if os.path.isdir(fixture.SCRATCH) else None))
SHOTS = pathlib.Path(os.environ.get("SKRIBISTO_SHOTS", str(OUT / "shots")))
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name

# What the novel fixture's Harbour scene holds: the phrases the editor's Djot round trip
# once rewrote on the first save of an edited scene (`write_fixtures.rs`, TRIGGERS).
TRIGGERS = ["10:30:45", "wrote--with care--a", "\"hi\" and 'bye'", "S. Russell's view"]

# What the novel fixture's card counts: its documents, and those in the manuscript.
NOVEL_COUNTS = (8, 4)

failures = []

# Every session still running, so a failure, an exception or Ctrl+C anywhere closes its
# app and its MCP server rather than leaving them behind.
SESSIONS = []


def close_all():
    for session in list(SESSIONS):
        session.close()


atexit.register(close_all)


def check(cond, msg):
    print(("  ok   " if cond else "  FAIL ") + msg)
    if not cond:
        failures.append(msg)
    return cond


def fail(msg, session=None):
    """Say why, show what the session saw when it still can, then close every session:
    the account of a failure must never be the reason an app is left running."""
    print("FAIL:", msg)
    try:
        if session:
            session.dump(msg)
            session.tail()
    except Exception as e:  # the MCP server may be the thing that failed
        print(f"  (could not show what the session saw: {e!r})")
    finally:
        close_all()
    sys.exit(1)


# --------------------------------------------------------------------------
# The fixtures
# --------------------------------------------------------------------------

def generate_fixtures(folder, scale):
    """Write the fixture projects into `folder` with the fixtures crate's example."""
    folder.parent.mkdir(parents=True, exist_ok=True)
    cargo = shutil.which("cargo") or "cargo"
    cmd = [cargo, "run", "-q", "--manifest-path", str(_ROOT / "Cargo.toml"),
           "-p", "skribisto-scrivener-fixtures", "--example", "write_fixtures", "--",
           str(folder), "--scale", str(scale)]
    # Run from the scratch folder, not the checkout: a developer's gitignored
    # `.cargo/config.toml` patches the sibling crates for any cargo started inside
    # it, and that rewrites Cargo.lock without their `source` and `checksum`
    # (`scripts/relock-crates-io.sh`). The fixtures need none of the siblings.
    done = subprocess.run(cmd, capture_output=True, text=True, cwd=str(folder.parent))
    if done.returncode != 0:
        print(done.stderr)
        fail("the fixtures could not be written")
    found = {}
    for line in done.stdout.splitlines():
        name, _, path = line.partition("\t")
        if path:
            found[name] = pathlib.Path(path)
    return found


def tree_digest(path):
    """A digest of every file under `path`: names, modes and contents."""
    h = hashlib.sha256()
    for root, dirs, files in sorted(os.walk(path)):
        dirs.sort()
        for name in sorted(files):
            p = pathlib.Path(root) / name
            h.update(str(p.relative_to(path)).encode())
            h.update(oct(p.stat().st_mode).encode())
            h.update(p.read_bytes())
    return h.hexdigest()


def unescape(djot):
    """Djot's backslash escapes of ASCII punctuation taken out: the text as typed."""
    out, i = [], 0
    while i < len(djot):
        c = djot[i]
        if c == "\\" and i + 1 < len(djot) and djot[i + 1] in "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~":
            out.append(djot[i + 1])
            i += 2
            continue
        out.append(c)
        i += 1
    return "".join(out)


def leftovers(folder):
    """Staged `.importing` files left anywhere under `folder`."""
    return [p for p in pathlib.Path(folder).rglob("*") if ".importing" in p.name]


def skrib_text(path):
    """Every text file of a `.skrib`, its manifests included, joined."""
    with zipfile.ZipFile(path) as z:
        return "\n".join(z.read(n).decode("utf-8", "replace") for n in z.namelist()
                         if n.endswith((".djot", ".ron", ".toml", ".json")))


def skrib_prose(path):
    """Every prose file of a `.skrib`, joined."""
    with zipfile.ZipFile(path) as z:
        return "\n".join(z.read(n).decode("utf-8", "replace")
                         for n in z.namelist() if n.endswith(".djot"))


# --------------------------------------------------------------------------
# The session
# --------------------------------------------------------------------------

class Session:
    def __init__(self, label, pins, env, project=None):
        self.label = label
        # Set before anything can fail, so a failure path can read them.
        self.window = None
        self.mcp = None
        self._id = 0
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(fixture.launch_argv(project, pins=pins),
                                    stdout=open(self.log, "w"), stderr=subprocess.STDOUT,
                                    env=env)
        SESSIONS.append(self)
        try:
            bridge = fixture.wait_for_bridge(self.log, self.app)
        except RuntimeError as e:
            self.tail()
            fail(str(e))
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP),
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "import-scrivener", "version": "1"}})
        if self._recv(timeout=20, fatal=False) is None:
            self.tail()
            fail("could not connect MCP")
        self._send("notifications/initialized", notif=True)

    # -- wire ---------------------------------------------------------------
    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=30, fatal=True):
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
            fail("no MCP response within timeout", self)
        return None

    def call(self, name, args=None):
        args = dict(args or {})
        if self.window is not None and name not in ("list_windows",) and "window_id" not in args:
            args["window_id"] = self.window
        self._send("tools/call", {"name": name, "arguments": args})
        result = (self._recv() or {}).get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(c.get("text", "") for c in result.get("content", [])
                           if c.get("type") == "text")
            try:
                payload = json.loads(text) if text.strip()[:1] in "[{" else {}
            except ValueError:
                payload = {}
        return result, payload

    def pin_first_window(self):
        """Address the first window from now on: a Help window opened later takes the
        focus, and a snapshot with no window names the focused one."""
        ws = self.windows()
        if ws:
            self.window = ws[0].get("id")

    # -- reading ------------------------------------------------------------
    def windows(self):
        _, w = self.call("list_windows")
        if isinstance(w, dict):
            w = w.get("windows", [])
        return w if isinstance(w, list) else []

    def nodes(self, window=None):
        args = {} if window is None else {"window_id": window}
        _, p = self.call("snapshot_tree", args)
        return p.get("nodes", []) if isinstance(p, dict) else []

    def text(self, window=None):
        out = []
        for n in self.nodes(window):
            out.append(n.get("label") or "")
            out.append(str(n.get("value") or ""))
        return " | ".join(out).lower()

    def wait_text(self, *wanted, timeout=30, window=None):
        end = time.time() + timeout
        while time.time() < end:
            t = self.text(window)
            for w in wanted:
                if w.lower() in t:
                    return w
            time.sleep(0.4)
        return None

    def texts(self, window=None):
        """Every text a node carries: its value, and the text a reader reviews in it."""
        out = []
        for n in self.nodes(window):
            out.append(str(n.get("value") or ""))
            info = n.get("text") or {}
            if isinstance(info, dict):
                out.append(str(info.get("document_text") or ""))
        return [t for t in out if t]

    def says(self, phrase, window=None):
        return any(phrase in t for t in self.texts(window))

    def find(self, label, role=None, exact=False, window=None):
        want = label.lower()
        for n in self.nodes(window):
            lab = (n.get("label") or "").strip().lower()
            if not lab:
                continue
            if (lab == want) if exact else (want in lab):
                if role is None or n.get("role") == role:
                    return n
        return None

    def wait_find(self, label, role=None, exact=False, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            n = self.find(label, role, exact)
            if n:
                return n
            time.sleep(0.4)
        return None

    def role(self, role):
        return [n for n in self.nodes() if n.get("role") == role]

    def button(self, label):
        return self.find(label, role="Button", exact=True)

    def enabled(self, label):
        b = self.button(label)
        return b is not None and not b.get("disabled")

    # -- acting -------------------------------------------------------------
    def click(self, node):
        """A pointer click at the node's centre: what selects a row of a list."""
        b = node.get("bounds") or {}
        if not isinstance(b, dict) or "x" not in b:
            return self.activate(node)
        self.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                     "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
        time.sleep(0.6)
        return True

    def activate(self, node):
        res, _ = self.call("invoke_action", {"node": node["id"], "action": "click"})
        time.sleep(0.6)
        return not (isinstance(res, dict) and res.get("isError"))

    def key(self, key, **mods):
        self.call("inject_key", {"key": key, **mods})
        time.sleep(0.4)

    def set_field(self, node, value, submit=False):
        self.call("set_value", {"node": node["id"], "value": value})
        time.sleep(0.3)
        if submit:
            self.call("focus_node", {"node": node["id"]})
            self.key("Enter")

    def shot(self, name):
        SHOTS.mkdir(parents=True, exist_ok=True)
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                path = SHOTS / name
                path.write_bytes(base64.b64decode(c["data"]))
                print(f"  screenshot -> {path}")
                return

    def dump(self, title, limit=120):
        print(f"--- AT tree: {title} ---")
        c = 0
        for n in self.nodes():
            lab = (n.get("label") or "").strip()
            val = str(n.get("value") or "").strip()
            if lab or val:
                extra = "  (disabled)" if n.get("disabled") else ""
                print(f"  [{n.get('role')}] {lab[:70]!r} {val[:50]!r}{extra}")
                c += 1
                if c >= limit:
                    print("  ...")
                    break

    def tail(self):
        print("--- app log tail ---")
        print("\n".join(open(self.log).read().splitlines()[-25:]))

    def close(self):
        if self in SESSIONS:
            SESSIONS.remove(self)
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()
        for p in (self.mcp, self.app):
            if p is None:
                continue
            try:
                p.wait(timeout=10)
            except Exception:
                p.kill()


# --------------------------------------------------------------------------
# Steps
# --------------------------------------------------------------------------

WORDS = {
    "en-US": {
        "launcher": "welcome sections", "create_from": "create from", "title": "import a scrivener project",
        "source": "scrivener project", "next": "Next", "back": "Back", "import": "Import",
        "made_with": "last saved by scrivener", "documents": "documents,",
        "see_report": "See report", "open_now": "Open now", "report": "about this import",
        "copy": "Copy report", "copied": "report copied", "close": "Close", "help": "Help",
        "cancel": "Cancel",
        "groups": ["check these (", "brought across differently (", "left behind ("],
    },
    "fr-FR": {
        "launcher": "welcome sections", "create_from": "créer depuis", "title": "importer un projet scrivener",
        "source": "projet scrivener", "next": "Suivant", "back": "Précédent", "import": "Importer",
        "made_with": "enregistré en dernier par scrivener", "documents": "documents,",
        "see_report": "Voir le compte rendu", "open_now": "Ouvrir maintenant",
        "report": "à propos de cet import", "copy": "Copier le compte rendu",
        "copied": "compte rendu copié", "close": "Fermer", "help": "Aide",
        "cancel": "Annuler",
        "groups": ["à vérifier (", "repris autrement (", "laissé de côté ("],
    },
}


def open_wizard_from_launcher(s, w):
    btn = s.wait_find(w["create_from"], role="Button")
    if not check(btn is not None, f"[{s.label}] the Launcher offers Create from…"):
        return False
    s.activate(btn)
    row = s.wait_find("scrivener (.scriv)", role="MenuItem", timeout=8)
    if not check(row is not None, f"[{s.label}] Create from… offers Scrivener (.scriv)…"):
        return False
    s.activate(row)
    return check(s.wait_text(w["title"], timeout=20) is not None,
                 f"[{s.label}] the row mounts the wizard")


def source_field(s, w):
    for n in s.nodes():
        if "set_value" in (n.get("actions") or []) and w["source"] in (n.get("label") or "").lower():
            return n
    return None


def read_project(s, w, path, *expect, timeout=60):
    field = source_field(s, w)
    if field is None:
        fail("no path field on the Project page", s)
    s.set_field(field, str(path), submit=True)
    return s.wait_text(*expect, timeout=timeout)


def project_page_checks(s, w, fx):
    # 2. The Scrivener 3 project reads.
    got = read_project(s, w, fx["novel"], w["made_with"])
    check(got is not None, "[en] the card says which Scrivener last saved the project")
    counts = re.search(r"(\d+) documents, (\d+) in the manuscript", s.text())
    check(counts is not None and (int(counts.group(1)), int(counts.group(2))) == NOVEL_COUNTS,
          f"[en] the card counts the documents, {NOVEL_COUNTS[0]} with {NOVEL_COUNTS[1]} in "
          f"the manuscript (saw {counts.group(0) if counts else None!r})")
    check(s.enabled(w["next"]), "[en] Next is enabled for a project that reads")
    s.shot("01-project-card.png")

    # 3. Two binder files and nothing to tell them apart.
    got = read_project(s, w, fx["manifests"], "binder files")
    check(got is not None, "[en] two binder files are asked about")
    radios = [n for n in s.role("RadioButton") if ".scrivx" in (n.get("label") or "")]
    check(len(radios) == 2, f"[en] one answer per binder file (saw {len(radios)})")
    check(not s.enabled(w["next"]), "[en] Next waits for the answer")
    b = next((r for r in radios if "draft b" in (r.get("label") or "").lower()), None)
    if check(b is not None, "[en] Draft B.scrivx is one of the answers"):
        s.activate(b)
        check(s.wait_text(w["made_with"], timeout=40) is not None and s.enabled(w["next"]),
              "[en] choosing one reads it, and Next is enabled")
        s.shot("02-two-binder-files.png")

    # 4. The copy kept before an upgrade selects the upgraded project.
    got = read_project(s, w, fx["upgrade"], "the project you went on writing in")
    check(got is not None, "[en] the pre-upgrade copy offers its upgraded sibling")
    current = s.find("the project you went on writing in", role="RadioButton")
    check(current is not None and current.get("toggled") in (True, "true", "True", 1),
          "[en] the upgraded project is selected")
    check(s.wait_text(w["made_with"], timeout=40) is not None,
          "[en] and read")
    # The card's name is a label's text, its value.
    names = [str(n.get("value") or "").strip() for n in s.nodes() if n.get("role") == "Label"]
    check("Lighthouse" in names and "Lighthouse Backup" not in names,
          "[en] the card names the upgraded project, not the copy picked")

    # 5. Edits from an iPhone or iPad.
    got = read_project(s, w, fx["ios"], "import it without those changes")
    check(got is not None, "[en] unmerged edits from an iPhone or iPad are asked about")
    check(not s.enabled(w["next"]), "[en] Next is off until they are accepted")
    accept = s.find("import it without those changes", role="CheckBox")
    if accept:
        s.activate(accept)
        time.sleep(0.6)
    check(s.enabled(w["next"]), "[en] ticking the acceptance enables Next")

    # 6. Scrivener 1 for Mac is refused.
    got = read_project(s, w, fx["scrivener1"], "made by scrivener 1 for mac")
    check(got is not None, "[en] a Scrivener 1 for Mac project is refused, in words")
    check(not s.enabled(w["next"]), "[en] Next is off for a refused project")
    s.shot("03-refusal.png")


def close_wizard(s, w):
    """The wizard closes on its own Cancel only: Escape is the fields'."""
    b = s.button(w["cancel"])
    if b:
        s.activate(b)
    time.sleep(1.0)
    return s.wait_text(w["title"], timeout=1) is None


def open_menu_row(s, *path):
    """Open the title-bar menu, then each row of `path`, by their AccessKit actions."""
    ham = s.find("menu", role="Button", exact=True)
    if ham is None:
        return False
    s.click(ham)
    time.sleep(1.0)
    for label in path:
        row = s.wait_find(label, role="MenuItem", exact=True, timeout=6)
        if row is None:
            row = s.wait_find(label, role="MenuItem", timeout=3)
        if row is None:
            s.dump(f"no menu row {label!r}")
            return False
        s.activate(row)
        time.sleep(0.8)
    return True


def go_next(s, w):
    b = s.button(w["next"])
    if b is None or b.get("disabled"):
        return False
    s.activate(b)
    time.sleep(1.0)
    return True


def review_page_checks(s, w):
    check(s.wait_text("top-level folders", timeout=20) is not None, "[en] Review shows its choices")
    t = s.text()
    check("becomes" in t and "in exports" in t, "[en] the preview says what each item becomes")
    check("top-level folder is left out" not in t, "[en] nothing is left out yet")
    combo = s.find("notes", role="ComboBox", exact=True)
    if not check(combo is not None, "[en] the top-level folder has a destination to choose"):
        s.dump("review")
        return
    s.shot("04-review.png")
    s.activate(combo)
    leave = s.wait_find("leave out", timeout=6)
    if leave is None:
        s.dump("combo open")
    else:
        s.click(leave)
    time.sleep(1.0)
    # The preview's rows carry no accessible text, so what the choice did is read off the
    # strip under it, which the same plan fills, and later off the imported binder.
    check(s.wait_text("1 top-level folder is left out", timeout=6) is not None,
          "[en] Leave out takes the folder out of the plan")
    s.shot("04b-review-left-out.png")


def save_and_import(s, w, folder, name):
    fields = [n for n in s.nodes() if "set_value" in (n.get("actions") or [])
              and n.get("role") in ("TextInput",)]
    location = next((n for n in fields if os.path.isabs(str(n.get("value") or ""))
                     and not str(n.get("value") or "").endswith((".scriv", ".scrivx"))), None)
    if location is None:
        fail("no destination folder field on the Save page", s)
    s.set_field(location, str(folder))
    fields = [n for n in s.nodes() if "set_value" in (n.get("actions") or [])
              and n.get("role") in ("TextInput",)]
    name_field = next((n for n in fields if (n.get("value") or "") == "Lighthouse"), None)
    if name_field is None:
        fail("no file name field on the Save page", s)
    s.set_field(name_field, name)
    time.sleep(0.6)
    b = s.button(w["import"])
    if not check(b is not None and not b.get("disabled"), f"[{s.label}] Import is enabled"):
        s.dump("save page")
        return False
    s.activate(b)
    return True


def report_checks(s, w, chosen=None):
    see = s.wait_find(w["see_report"], exact=True, timeout=5)
    if not check(see is not None, f"[{s.label}] the result offers See report"):
        return False
    s.activate(see)
    if not check(s.wait_text(w["report"], timeout=10) is not None,
                 f"[{s.label}] See report opens the report"):
        return False
    t = s.text()
    for g in w["groups"]:
        check(g in t, f"[{s.label}] the report has its group {g!r}")
    if chosen:
        check(chosen.lower() in t, f"[{s.label}] the report records the writer's choice {chosen!r}")
    copy = s.button(w["copy"])
    if copy:
        s.activate(copy)
        check(s.wait_text(w["copied"], timeout=5) is not None, f"[{s.label}] Copy report says it copied")
        pasted = paste()
        if pasted is None:
            print(f"  skip [{s.label}] the clipboard could not be read (no wl-paste or no "
                  "Wayland session): what Copy report put there is not checked")
        else:
            check(w["report"] in pasted.lower() and (not chosen or chosen.lower() in pasted.lower()),
                  f"[{s.label}] the clipboard holds the report, choices included")
    return True


def has_focus(s):
    """Whether the session's window holds the OS focus (`list_windows` leaves `focused` out
    of a window's entry when it does not)."""
    return any(win.get("id") == s.window and win.get("focused", False) for win in s.windows())


def starting(s, prefix, role="Button"):
    """The first node of `role` whose label starts with `prefix`, ignoring case."""
    want = prefix.lower()
    return next((n for n in s.nodes() if n.get("role") == role
                 and (n.get("label") or "").lower().startswith(want)), None)


def laid_out_y(s, node):
    """Where `node` is laid out, whether or not the AT tree still lists it."""
    _, info = s.call("inspect_node", {"node": node["id"]})
    return ((info or {}).get("bounds") or {}).get("y") if isinstance(info, dict) else None


def scroll_report_to(s, w, prefix):
    """Bring the button `prefix` into the report's column, between the dialog's header and
    its footer, so a pointer can press it. Giving it the keyboard focus scrolls the report
    to it, as Tab would."""
    node, copy = starting(s, prefix), s.button(w["copy"])
    if node is None or copy is None:
        print(f"  (no {prefix!r} or no {w['copy']!r} in the report)")
        return None
    s.call("focus_node", {"node": node["id"]})
    time.sleep(0.5)
    node, copy = starting(s, prefix), s.button(w["copy"])
    if node is None or copy is None:
        print(f"  (focusing {prefix!r} closed the report)")
        return None
    b, cb = node["bounds"], copy["bounds"]
    # The lowest Close above the footer is the dialog header's cross (the window's own
    # sits higher): the column starts under it.
    crosses = [n["bounds"]["y"] + n["bounds"]["height"] for n in s.role("Button")
               if n.get("label") == w["close"] and n["bounds"]["y"] < cb["y"] - 100]
    top = max(crosses) + 8 if crosses else 0
    bottom = cb["y"] - 8
    if top <= b["y"] and b["y"] + b["height"] <= bottom:
        return node
    print(f"  ({prefix!r} at y={b['y']} once focused, outside the column y={top}..{bottom})")
    return None


def press_disclosure(s, w, prefix, below, what):
    """Press the disclosure `prefix` with a pointer and check it opens: it says it is
    expanded, and `below`, laid out under it, moves down."""
    node = scroll_report_to(s, w, prefix)
    if not check(node is not None, f"[{s.label}] {what} can be scrolled into view"):
        return False
    before = laid_out_y(s, below)
    s.click(node)
    time.sleep(1.0)
    after = laid_out_y(s, below)
    pressed = starting(s, prefix) or {}
    check(pressed.get("expanded") is True, f"[{s.label}] {what} says it is expanded")
    return check(before is not None and after is not None and after > before + 4,
                 f"[{s.label}] {what} opens to its height ({before} -> {after})")


def report_disclosure_checks(s, w):
    """A group the report opens closed, then a list of items in it, open to their height
    when pressed. Teksilo before 0.15.1 froze a one-shot tween in a window without the
    focus, so there an opened accordion said it was expanded and showed nothing: which of
    the two this run saw is printed."""
    print(f"  note [{s.label}] the report's window {'has' if has_focus(s) else 'does not have'} "
          "the focus")
    group, under = w["groups"][1], w["groups"][2]
    below = starting(s, under)
    if not check(below is not None, f"[{s.label}] the report has its group {under!r}"):
        return
    if not press_disclosure(s, w, group, below, f"the group {group!r}"):
        return
    # A list of items inside that group: a list button laid out between it and the next.
    gy = laid_out_y(s, starting(s, group))
    ly = laid_out_y(s, below)
    lists = [n for n in s.role("Button") if ": " in (n.get("label") or "")
             and gy is not None and ly is not None
             and gy < (laid_out_y(s, n) or -1) < ly]
    if check(bool(lists), f"[{s.label}] the group lists items behind a button of their own"):
        press_disclosure(s, w, lists[0]["label"], below, "its first list of items")


def paste():
    tool = shutil.which("wl-paste")
    if not tool or not os.environ.get("WAYLAND_DISPLAY"):
        return None
    try:
        return subprocess.run([tool, "--no-newline"], capture_output=True, text=True,
                              timeout=5).stdout
    except Exception:
        return None


def inside(node, box):
    """Whether `node`'s bounds lie within `box`'s."""
    b, d = node.get("bounds") or {}, box.get("bounds") or {}
    if not all(isinstance(x, dict) and "x" in x for x in (b, d)):
        return False
    return (d["x"] <= b["x"] and b["x"] + b.get("width", 0) <= d["x"] + d.get("width", 0)
            and d["y"] <= b["y"] and b["y"] + b.get("height", 0) <= d["y"] + d.get("height", 0))


def wait_text_anywhere(s, *wanted, timeout=10):
    """The first of `wanted` any window shows, and that window's id.

    A `MessageBox` is a window of its own on macOS and Windows, and a dialog inside
    the window that asked on Linux, so a question is looked for in every window.
    `(None, None)` when none shows one in time.
    """
    end = time.time() + timeout
    while time.time() < end:
        for win in s.windows():
            t = s.text(win.get("id"))
            for phrase in wanted:
                if phrase.lower() in t:
                    return phrase, win.get("id")
        time.sleep(0.4)
    return None, None


def descendants(nodes, root):
    """The ids of every node under `root` in the accessibility tree.

    What "in the dialog" means for a control. Bounds alone are not: a toast drawn
    over the dialog's rectangle (the import's own, with its "See report" link)
    lies inside them without being part of it.
    """
    by_id = {n.get("id"): n for n in nodes}
    out, todo = set(), list(root.get("children") or [])
    while todo:
        nid = todo.pop()
        if nid in out:
            continue
        out.add(nid)
        todo.extend((by_id.get(nid) or {}).get("children") or [])
    return out


def dialog_button(s, w, label):
    """The report's own button `label`: one inside the dialog."""
    dialog = s.find(w["report"], role="Dialog")
    found = [n for n in s.role("Button") if (n.get("label") or "") == label
             and dialog is not None and inside(n, dialog)]
    return max(found, key=lambda n: (n.get("bounds") or {}).get("y", 0)) if found else None


def close_dialog(s, w):
    """The report's own Close, in its footer: never the window's own Close button, which
    carries the same name."""
    dialog = s.find(w["report"], role="Dialog")
    closes = [n for n in s.role("Button") if (n.get("label") or "") == w["close"]
              and dialog is not None and inside(n, dialog)]
    if not closes:
        s.dump("no Close in the report")
        return
    s.activate(max(closes, key=lambda n: (n.get("bounds") or {}).get("y", 0)))
    time.sleep(0.8)


def open_report_from_palette(s, name):
    """Help > Command Palette…, then the entry `name`."""
    if not open_menu_row(s, "Help", "Command Palette…"):
        return False
    field = s.wait_find("command palette", role="TextInput", timeout=6)
    if field is None:
        s.dump("no command palette")
        return False
    s.set_field(field, name)
    time.sleep(0.8)
    s.call("focus_node", {"node": field["id"]})
    s.key("Enter")
    return True


# --------------------------------------------------------------------------
# The English pass
# --------------------------------------------------------------------------

def english(fx):
    w = WORDS["en-US"]
    env = fixture.isolated_config(locale="en-US", label="import-scrivener-en")
    pins = fixture.config_pins_file({"ui.locale": "en-US", "ui.dark": False,
                                     "ui.show_welcome": True}, label="import-scrivener-en")
    s = Session("en", pins, env)
    if not s.wait_text(w["launcher"], timeout=60):
        fail("the Launcher never appeared", s)
    s.pin_first_window()
    if not open_wizard_from_launcher(s, w):
        fail("no wizard from the Launcher", s)
    project_page_checks(s, w, fx)

    # 7. Back to the readable project, then Review.
    read_project(s, w, fx["novel"], w["made_with"])
    if not go_next(s, w):
        fail("Next did not leave the Project page", s)
    review_page_checks(s, w)

    # 8. Save, Import.
    go_next(s, w)
    dest = OUT / "en"
    dest.mkdir(parents=True, exist_ok=True)
    started = time.time()
    if not save_and_import(s, w, dest, "Lighthouse"):
        fail("the import did not start", s)
    done = s.wait_text("open now", "import failed", "could not", timeout=180)
    check(done == "open now", f"[en] the import finished and offered Open now ({time.time() - started:.1f} s)")
    target = dest / "Lighthouse.skrib"
    check(target.is_file(), "[en] it wrote Lighthouse.skrib")
    check(not leftovers(dest), "[en] no .importing file is left")
    s.shot("05-result.png")

    # 9. The report. Reading it leaves the notice up, with Open now.
    if report_checks(s, w, chosen="Notes: Leave out"):
        s.shot("06-report.png")
        report_disclosure_checks(s, w)
        s.shot("06b-report-opened.png")
        close_dialog(s, w)
        check(s.wait_text(w["report"], timeout=2) is None, "[en] Close closes the report")
        check(s.find(w["open_now"], exact=True) is not None
              and s.find(w["see_report"], exact=True) is not None,
              "[en] the notice is still up, with Open now and See report")
        see = s.find(w["see_report"], exact=True)
        if see:
            s.activate(see)
        check(s.wait_text(w["report"], timeout=10) is not None, "[en] See report opens it again")

    # 10. Open now, from the report: the project in a window of its own.
    opener = dialog_button(s, w, w["open_now"])
    if not check(opener is not None, "[en] the report offers Open now"):
        fail("no Open now", s)
    s.activate(opener)
    project_window = None
    end = time.time() + 240
    while time.time() < end and project_window is None:
        for win in s.windows():
            label = (win.get("label") or "") + " " + (win.get("title") or "")
            if "lighthouse" in label.lower():
                project_window = win.get("id")
        time.sleep(1.0)
    check(project_window is not None, "[en] Open now opened the project in a window of its own")
    if project_window is None:
        fail("the imported project never opened", s)
    s.window = project_window
    check(s.wait_text("night watch", timeout=240) is not None,
          "[en] its binder holds the Draft's titles")
    check(s.find("ideas", exact=True) is None,
          "[en] and not the folder the review left out")
    s.shot("07-project.png")

    # The command palette reopens the report in the project's window, where its items
    # open their rows.
    open_report_from_palette(s, "Show Scrivener Import Report")
    if check(s.wait_text(w["report"], timeout=10) is not None,
             "[en] the command palette reopens the report in the project's window"):
        # Each list's button is named with its line ("3 items: …").
        disclosure = s.find("3 items: ", role="Button")
        if disclosure:
            s.activate(disclosure)
            # Animations run on the bridge's clock: let the disclosure open.
            s.call("settle")
        dialog = s.find(w["report"], role="Dialog")
        nodes = s.nodes()
        under = descendants(nodes, dialog) if dialog is not None else set()
        links = [n for n in nodes if n.get("role") == "Link" and n.get("id") in under]
        s.shot("08-report-linked.png")
        if check(bool(links), "[en] there, the report's items are links"):
            label = links[0].get("label") or ""
            s.activate(links[0])
            check(s.wait_text(w["report"], timeout=2) is None,
                  f"[en] a link closes the report to open its item ({label!r})")
        else:
            close_dialog(s, w)

    # 12a. Importing onto the project open in this window is refused.
    menu_ok = (open_menu_row(s, "Work", "Import from", "Scrivener (.scriv)…")
               and s.wait_text(w["title"], timeout=15) is not None)
    check(menu_ok, "[en] Work > Import from > Scrivener (.scriv)… mounts the wizard in a project window")
    if menu_ok:
        read_project(s, w, fx["novel"], w["made_with"])
        go_next(s, w)
        go_next(s, w)
        if save_and_import(s, w, dest, "Lighthouse"):
            got, where = wait_text_anywhere(s, "this project is open", "replace existing file")
            check(got == "this project is open",
                  "[en] importing onto the project open in a window is refused")
            # Answered where it is shown: on macOS and Windows, a window of its own.
            saved, s.window = s.window, where or s.window
            ok = s.button("OK") or s.button("Cancel")
            if ok:
                s.activate(ok)
            s.window = saved
        check(close_wizard(s, w), "[en] the wizard closes on Cancel")

    # 11. Edit and reopen: the trigger phrases survive an edit and a save.
    harbour = s.find("the harbour", role="TreeItem") or s.find("the harbour")
    edited = False
    if check(harbour is not None, "[en] the Harbour scene is in the binder"):
        s.activate(harbour)
        time.sleep(2.0)
        editor = next((n for n in s.nodes() if "10:30:45" in str(n.get("value") or "")), None)
        if editor is None:
            s.dump("the Harbour scene, opened")
        if check(editor is not None, "[en] the scene opens with its trigger phrases"):
            for phrase in TRIGGERS:
                check(s.says(phrase), f"[en] as imported, the scene says {phrase!r}")
            s.call("focus_node", {"node": editor["id"]})
            s.key("End", command=True)
            s.call("type_text", {"node": editor["id"], "text": " Tide."})
            time.sleep(0.6)
            before = target.stat().st_mtime
            s.key("s", command=True)
            end = time.time() + 60
            while time.time() < end and target.stat().st_mtime == before:
                time.sleep(0.5)
            edited = check(target.stat().st_mtime != before, "[en] the edited scene was saved")
            stored = unescape(skrib_prose(target))
            check("Tide." in stored and all(p in stored for p in TRIGGERS),
                  "[en] the saved project holds the edit and every phrase as typed")
    s.close()
    return target, edited


def reopen(target):
    """Relaunch on the saved project and read the Harbour scene back."""
    w = WORDS["en-US"]
    env = fixture.isolated_config(locale="en-US", label="import-scrivener-reopen", show_welcome=False)
    pins = fixture.config_pins_file({"ui.locale": "en-US", "ui.dark": False,
                                     "ui.show_welcome": False}, label="import-scrivener-reopen")
    s = Session("reopen", pins, env, project=str(target))
    if not s.wait_text("night watch", timeout=240):
        fail("the saved project did not reopen", s)
    harbour = s.find("the harbour", role="TreeItem") or s.find("the harbour")
    if harbour:
        s.activate(harbour)
        time.sleep(2.0)
    if check(s.says("Tide."), "[en] reopened, the scene holds the edit"):
        for phrase in TRIGGERS:
            check(s.says(phrase), f"[en] reopened, the scene still says {phrase!r}")
    s.shot("09-reopened.png")
    s.close()


def overwrite_and_cancel(fx, target):
    """12b and 13, on a Launcher of their own."""
    w = WORDS["en-US"]
    env = fixture.isolated_config(locale="en-US", label="import-scrivener-cancel")
    pins = fixture.config_pins_file({"ui.locale": "en-US", "ui.dark": False,
                                     "ui.show_welcome": True}, label="import-scrivener-cancel")
    s = Session("en", pins, env)
    if not s.wait_text(w["launcher"], timeout=60):
        fail("the Launcher never appeared", s)
    s.pin_first_window()

    # 3b. Draft B differs from Draft A by one title, which the preview does not expose to
    # the bridge: import it, and look for that title in what was written.
    if open_wizard_from_launcher(s, w):
        read_project(s, w, fx["manifests"], "binder files")
        radios = [n for n in s.role("RadioButton") if "draft b" in (n.get("label") or "").lower()]
        dest = OUT / "draft-b"
        dest.mkdir(parents=True, exist_ok=True)
        if (check(bool(radios), "[en] Draft B.scrivx can be chosen")
                and s.activate(radios[0])
                and s.wait_text(w["made_with"], timeout=40) is not None
                and go_next(s, w) and go_next(s, w)
                and save_and_import(s, w, dest, "Draft B")):
            done = s.wait_text("open now", "import failed", "could not", timeout=180)
            written = dest / "Draft B.skrib"
            check(done == "open now" and written.is_file()
                  and "Arrival B" in skrib_text(written),
                  "[en] the binder file chosen is the one imported: it holds Arrival B")
        else:
            check(False, "[en] the project read from Draft B.scrivx imports")

    # 12b. The same name again asks before replacing it.
    if open_wizard_from_launcher(s, w):
        read_project(s, w, fx["novel"], w["made_with"])
        go_next(s, w)
        go_next(s, w)
        if save_and_import(s, w, target.parent, "Lighthouse"):
            asked, where = wait_text_anywhere(s, "replace existing file")
            check(asked is not None,
                  "[en] importing again to the same name asks before replacing it")
            # The question's own Cancel, not the wizard's under it, answered where the
            # question is shown: on macOS and Windows, a window of its own.
            saved, s.window = s.window, where or s.window
            nodes = s.nodes()
            question = next((n for n in nodes
                             if "replace existing file" in (n.get("label") or "").lower()
                             and n.get("role") in ("Dialog", "AlertDialog")), None)
            under = descendants(nodes, question) if question is not None else set()
            cancels = [n for n in nodes if n.get("role") == "Button"
                       and (n.get("label") or "") == "Cancel" and n.get("id") in under]
            if cancels:
                s.activate(cancels[0])
            else:
                s.dump("the overwrite question")
                s.key("Escape")
            s.window = saved
            check(wait_text_anywhere(s, "replace existing file", timeout=1)[0] is None,
                  "[en] Cancel answers the question")
        check(close_wizard(s, w), "[en] the wizard closes on Cancel")

    # 13. Cancel a large import from its notice.
    if open_wizard_from_launcher(s, w):
        got = read_project(s, w, fx["large"], w["made_with"], timeout=120)
        check(got is not None, "[en] the large project reads")
        go_next(s, w)
        go_next(s, w)
        dest = OUT / "cancel"
        dest.mkdir(parents=True, exist_ok=True)
        if save_and_import(s, w, dest, "Lighthouse"):
            # The wizard closes as the import starts: its Cancel is gone before the
            # notice's is looked for, so the one pressed is the notice's.
            end = time.time() + 10
            while time.time() < end and s.wait_text(w["title"], timeout=0.5) is not None:
                time.sleep(0.2)
            check(s.wait_text(w["title"], timeout=0.5) is None,
                  "[en] the wizard closes as the import starts")
            progress = s.wait_find("importing scrivener project", timeout=10)
            cancel = s.wait_find("Cancel", role="Button", exact=True, timeout=10)
            if check(progress is not None and cancel is not None,
                     "[en] the progress notice offers Cancel"):
                s.activate(cancel)
            outcome = s.wait_text("import canceled", "open now", timeout=240)
            check(outcome == "import canceled",
                  f"[en] Cancel stops the import (it ended with {outcome!r})")
            check(not (dest / "Lighthouse.skrib").exists(), "[en] a cancelled import writes nothing")
            time.sleep(1.0)
            check(not leftovers(dest), "[en] no .importing file is left after a cancel")
    s.close()


# --------------------------------------------------------------------------
# The French pass
# --------------------------------------------------------------------------

def french(fx):
    w = WORDS["fr-FR"]
    env = fixture.isolated_config(locale="fr-FR", label="import-scrivener-fr", dark=True)
    pins = fixture.config_pins_file({"ui.locale": "fr-FR", "ui.dark": True,
                                     "ui.show_welcome": True}, label="import-scrivener-fr")
    s = Session("fr", pins, env)
    if not s.wait_text(w["launcher"], "œuvres", timeout=60):
        fail("the Launcher never appeared", s)
    s.pin_first_window()
    if not open_wizard_from_launcher(s, w):
        fail("no wizard from the Launcher", s)
    t = s.text()
    check("choisir un fichier" in t and "choisir un dossier" in t, "[fr] the wizard's buttons are French")

    # The wizard's Help opens "Coming from Scrivener", in French.
    helpb = s.button(w["help"])
    if check(helpb is not None, "[fr] the wizard has Help"):
        s.activate(helpb)
        time.sleep(2.0)
        hw = next((x.get("id") for x in s.windows() if x.get("label") == "help"), None)
        if check(hw is not None, "[fr] Help opens the help window"):
            check(s.wait_text("venir de scrivener", timeout=10, window=hw) is not None,
                  "[fr] on the topic Venir de Scrivener")
            body = s.text(window=hw)
            check("literature and latte" in body, "[fr] which ends with the trademark sentence")
            saved = s.window
            s.window = hw
            s.shot("10-help-fr.png")
            s.window = saved

    got = read_project(s, w, fx["novel"], w["made_with"])
    check(got is not None, "[fr] the card is French")
    go_next(s, w)
    check(s.wait_text("dossiers de premier niveau", timeout=20) is not None, "[fr] Review is French")
    go_next(s, w)
    dest = OUT / "fr"
    dest.mkdir(parents=True, exist_ok=True)
    if save_and_import(s, w, dest, "Phare"):
        check(s.wait_text("ouvrir maintenant", timeout=180) is not None,
              "[fr] the import finished and offered Ouvrir maintenant")
        if report_checks(s, w):
            s.shot("11-report-fr-dark.png")
            check("ce qui est arrivé" in s.text(), "[fr] the counts are French")
    s.close()


# --------------------------------------------------------------------------
# The run
# --------------------------------------------------------------------------

def main():
    print(f"output : {OUT}")
    # D8-12 #8: a project of 3,000 items, so that Cancel lands before the import ends.
    scale = max(3000, int(os.environ.get("SKRIBISTO_SCRIVENER_SCALE", "3000")))
    fx = generate_fixtures(OUT / "fixtures", scale)
    for name in ("novel", "manifests", "upgrade", "upgrade-current", "ios", "scrivener1",
                 "large"):
        if name not in fx:
            fail(f"the fixtures have no {name!r} project")
    # Every project written, the upgraded one step 4 reads through the sibling question
    # included: it is a source the import reads like any other.
    digests = {name: tree_digest(path) for name, path in fx.items()}

    try:
        target, edited = english(fx)
        time.sleep(1.5)
        if edited:
            reopen(target)
            time.sleep(1.5)
        overwrite_and_cancel(fx, target)
        time.sleep(1.5)
        french(fx)
    finally:
        close_all()

    # 14. Nothing read was changed.
    for name, path in fx.items():
        check(tree_digest(path) == digests[name], f"the {name} project is unchanged by the imports")

    print()
    if failures:
        print(f"{len(failures)} check(s) failed:")
        for f in failures:
            print("  -", f)
        sys.exit(1)
    print("all checks passed")


if __name__ == "__main__":
    main()
