#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the teksilo automation MCP bridge and verify the
New Work **wizard** end-to-end.

The modal is a three-step `Stepper` (Details → Language & structure →
Template), so this script walks all three: the old one-page form no longer
exists and a single structural assertion could not see past step one.

Flow: launch the app (no CLI arg) → the **Launcher window** opens (the
Welcome UI is a real window now, not a modal — see `teksilo_ui::main`'s module
docs) → click the Launcher's "New Work" button (there is no `Ctrl+N` global
shortcut in the Launcher window — that action only exists inside an
already-open project window's tree, so a keyboard fallback would just no-op;
the button is the only path here) → **step 1**: assert the Form landmark, the
Format RadioGroup, that Next is gated off while the name is empty, that focus
opens on the name field, and that the reactive "Will create …/<slug>.skrib"
path preview recomputes (single-file vs bundle) → **step 2**: the language and
paratext ComboBoxes → **step 3**: the Template RadioGroup and an enabled
"Create Work" → screenshot each. Assertions favour locale-independent anchors
(roles + the .skrib path) so the harness passes whatever UI language is
persisted.

Then a second phase for the Launcher's **From documents…**, which reuses the very
same wizard for a project an import is about to fill: it must open *directly*
(that door used to pick files first and then have the import wizard ask for them
again), title itself for the flow, ask the language only on step 2, offer no
template on step 3, and — this one does press Finish, in an isolated config over a
scratch folder — hand off to a project window with the import wizard already open
over the new project.

Note: the ordinary "Create Work" is not clicked; creating from the Launcher opens
a *second* (project) window and closes the Launcher, and that transition is
covered end-to-end by `automation_welcome.py`.

Reuses the launch + scrape-socket/token + connect scaffolding from
automation_welcome.py.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

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
        # The bridge announces the socket before binding; retry connect+init.
        deadline = time.time() + 20
        init = None
        while time.time() < deadline and init is None:
            while not os.path.exists(sock) and time.time() < deadline:
                time.sleep(0.05)
            self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                        stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=open(mcp_err, "w"), text=True, bufsize=1)
            self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                      "clientInfo": {"name": "new-work-test", "version": "1"}})
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

    def tools(self):
        self._send("tools/list")
        return {t["name"]: t for t in self._recv().get("result", {}).get("tools", [])}

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

    def find_label_contains(self, substr, timeout=20):
        """First node whose label contains `substr` (case-insensitive)."""
        end = time.time() + timeout
        while time.time() < end:
            for n in self.nodes():
                if substr.lower() in (n.get("label") or "").lower():
                    return n
            time.sleep(0.3)
        return None

    def by_role(self, *roles):
        want = {r.lower() for r in roles}
        return [n for n in self.nodes() if (n.get("role") or "").lower() in want]

    def dump(self, header=""):
        print(f"--- AT tree{(' ' + header) if header else ''} ---")
        for n in self.nodes():
            lab = (n.get("label") or "").replace("\n", " ")
            print(f"  role={n.get('role'):<16} label={lab[:48]!r:<50} "
                  f"value={str(n.get('value'))[:32]!r} id={n.get('id')} "
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


# ── Launch, wait for the Launcher window ──────────────────────────────────────
print("== New Work modal via the automation MCP ==")
s = Session([])
tools = s.tools()
print(f"tools: {len(tools)} ->", ", ".join(sorted(tools)))
for n in ("inject_key", "type_text", "focus_node", "set_value", "invoke_action"):
    if n in tools:
        print(f"# {n} schema:", json.dumps(tools[n].get("inputSchema", {}))[:300])

if not s.wait_label("welcome sections"):
    fail("the Launcher window did not appear at startup", s.app, s.mcp, s.log)
print("Launcher window is up.")


# Text shows up in a node's `value` (AccessKit), not `label`.
def val(n):
    return "" if n.get("value") is None else str(n.get("value"))


def find_value_contains(substr, timeout=6):
    end = time.time() + timeout
    while time.time() < end:
        for n in s.nodes():
            if substr.lower() in val(n).lower():
                return n
        time.sleep(0.3)
    return None


# ── Open the New Work modal ───────────────────────────────────────────────────
# Click the Launcher's "New Work" button — this exercises the real wiring
# (`WelcomeViewModel::new_work` presents `NewWorkPanel::new_for_launcher`
# directly in the Launcher window). This is the *only* path here: unlike an
# already-open project window, the Launcher registers no `work.new` global
# action/shortcut (there is no `App` mounted there), so a `Ctrl+N` fallback
# would just no-op. The modal is detected structurally by its FormLayout body
# (role "Form") — the SegmentedControls surface as RadioGroups and the label
# is localized, so role is the robust anchor.
def new_work_form():
    return next(iter(s.by_role("Form")), None)


def welcome_new_work_button():
    for n in s.nodes():
        if (n.get("role") == "Button") and any(
            w in (n.get("label") or "").lower()
            for w in ("new work", "nouvelle œuvre", "nouvelle oeuvre")
        ):
            return n
    return None


def open_new_work():
    btn = welcome_new_work_button()
    if not btn:
        return None
    res, _ = s.call("invoke_action", {"node": btn["id"], "action": "click"})
    if res.get("isError"):
        return None
    time.sleep(0.8)
    if new_work_form():
        return f"Launcher 'New Work' button ({btn.get('label')!r})"
    return None


how = open_new_work()
if not how:
    s.dump("after open attempt")
    fail("could not open the New Work modal", s.app, s.mcp, s.log)
print(f"New Work modal opened via {how}.")
s.dump("New Work modal")


# ── Step 1 (Details): assert the structure (locale-independent by role) ───────
# Only the Details page is mounted — a `Stepper` builds the active step alone,
# so the Template RadioGroup and the language ComboBox are legitimately absent
# here and are asserted on their own pages further down.
forms = s.by_role("Form")
text_fields = s.by_role("TextInput", "TextField")
radiogroups = s.by_role("RadioGroup")  # the Format RadioTileGroup
combos = s.by_role("ComboBox")
print(f"step 1 structure: Form x{len(forms)}, text fields x{len(text_fields)}, "
      f"RadioGroup x{len(radiogroups)}, ComboBox x{len(combos)}")
if not forms:
    fail("no Form landmark (the FormLayout body) in the New Work wizard", s.app, s.mcp, s.log)
if len(radiogroups) < 1:
    fail("expected the Format RadioTileGroup as a RadioGroup", s.app, s.mcp, s.log)
for lbl in ("cancel", "annuler"):
    if any(lbl in (b.get("label") or "").lower() for b in s.by_role("Button")):
        break
else:
    print("note: no Cancel button label matched (locale?) — continuing")


# ── Footer buttons ────────────────────────────────────────────────────────────
def find_button(words, exclude=()):
    for n in s.by_role("Button"):
        lab = (n.get("label") or "").lower()
        if any(w in lab for w in words) and not any(x in lab for x in exclude):
            return n
    return None


def find_next_button():
    # "Next" / "Suivant". Excluded from the Create match below, and vice versa.
    return find_button(("next", "suivant"))


def find_create_button():
    return find_button(("create work", "créer", "creer"), exclude=("cancel", "annuler"))


def node_enabled(node_id):
    _, p = s.call("read_node", {"node": node_id})
    if isinstance(p, dict):
        if p.get("disabled") is True:
            return False
        if "enabled" in p:
            return bool(p["enabled"])
    return True


# ── Empty name → "Next" DISABLED + an inline validation message ───────────────
# The gate moved with the fields it guards: name + location live on step 1, so
# step 1's `complete_when` is what refuses to advance. Create is not even
# mounted yet.
next_btn = find_next_button()
if not next_btn:
    s.dump("step 1")
    fail("no 'Next' button in the wizard footer", s.app, s.mcp, s.log)
if find_create_button():
    fail("'Create Work' must not be reachable from step 1", s.app, s.mcp, s.log)
statuses = [n for n in s.nodes() if n.get("role") == "Status" and (n.get("label") or "").strip()]
print(f"empty-name state: Next enabled={node_enabled(next_btn['id'])}; "
      f"validation status={[n.get('label') for n in statuses]}")
if node_enabled(next_btn["id"]):
    fail("Next should be DISABLED while the Work name is empty", s.app, s.mcp, s.log)
if not statuses:
    fail("expected an inline validation message while the name is empty", s.app, s.mcp, s.log)
print("PASS: empty name → Next disabled + inline validation shown")
s.shot("/tmp/sk-new-work-invalid.png")


# ── The modal must open with the Work name field focused ─────────────────────
# Also how the name field is *identified*: "the empty text field" is ambiguous —
# the Launcher window sits behind this modal and its "Search works" box is an
# empty TextInput too, higher up the tree AND higher on screen (y≈57 vs y≈70), so
# a first-match heuristic picks the Launcher's field and every later assertion
# then tests the wrong widget.
#
# The focused node is unambiguous, and asserting it is worth doing in its own
# right: `NewWorkPanel::initial_focus_hint` points at the name field, but the
# stepper's indicator strip and footer sit around the step body, either of which
# a naive focusable-descendant fallback could land on instead.
raw, _ = s.call("snapshot_tree")
focus_id = json.loads(raw["content"][0]["text"]).get("focus")
name_field = next((n for n in s.nodes() if n.get("id") == focus_id), None)
if name_field is None or name_field.get("role") != "TextInput":
    focused_role = name_field.get("role") if name_field else None
    fail(f"New Work should open with the Work name field focused, but focus is on "
         f"{focused_role!r} (id={focus_id}) — a user typing straight into the "
         f"dialog would lose their keystrokes", s.app, s.mcp, s.log)
if val(name_field):
    fail(f"the focused field should be the empty Work name, got {val(name_field)!r}",
         s.app, s.mcp, s.log)
print("PASS: the modal opens with the Work name field focused")

# Location is the field carrying a path; exclude the focused name field so the
# Launcher's search box behind the modal can never be mistaken for either.
location_field = next((n for n in text_fields
                       if "/" in val(n) and n.get("id") != name_field.get("id")), None)
if location_field is None:
    fail("could not identify the Location field", s.app, s.mcp, s.log)
loc = val(location_field)
print(f"name field id={name_field.get('id')}; location={loc!r}")

# Identify the "Will create …" preview *before* typing, by the one property no
# other node in the tree shares: it is a Label that is empty now and fills in
# when the name does. Searching the whole tree for a ".skrib" value instead
# matches the Launcher sitting behind the modal — its recent-works list is full
# of real project paths, and the mocks build adds "/mock/Mock Project.skrib" —
# so the assertion would read a node the dialog never wrote.
empty_labels_before = {n["id"] for n in s.nodes() if n.get("role") == "Label" and not val(n)}

# `type_text`, not `set_value`: typing is what a user does, and it is what drives
# the field's bound signal (and through it the VM's derived path preview).
res, _ = s.call("type_text", {"node": name_field["id"], "text": "Tidewrack"})
if isinstance(res, dict) and res.get("isError"):
    fail(f"type_text into the name field errored: {res}", s.app, s.mcp, s.log)
time.sleep(0.8)


def filled_preview(timeout=6):
    """The formerly-empty Label that now carries a path, or None."""
    end = time.time() + timeout
    while time.time() < end:
        for n in s.nodes():
            if n.get("id") in empty_labels_before and "/" in val(n):
                return n
        time.sleep(0.3)
    return None


preview = filled_preview()
if preview is None:
    s.dump("after setting name")
    fail("path preview never showed a path after setting the name", s.app, s.mcp, s.log)
preview_id = preview["id"]
path = val(preview).strip()
print(f"single-file path preview: {path!r}")
if not path.lower().endswith("tidewrack.skrib"):
    fail(f"preview should end with 'tidewrack.skrib', got {path!r}", s.app, s.mcp, s.log)
print("PASS: single-file preview reacts to the name (…/tidewrack.skrib)")

# A valid name + (existing, writable) location must re-open the gate.
next_btn = find_next_button() or next_btn
if not node_enabled(next_btn["id"]):
    fail("Next should be ENABLED once name + location are valid", s.app, s.mcp, s.log)
print("PASS: valid name + location → Next enabled")

# ── Switch format to Bundle → the preview drops the ".skrib" extension ────────
# Format's second tile is "Bundle". RadioTiles surface as RadioButtons; the AT
# `click` may not drive the composite, so fall back to a pointer click on the
# tile's bounds. Success is judged by the reactive preview (a RadioTileGroup does
# not expose its selected label as an AT value).
def bundle_preview():
    n = next((n for n in s.nodes() if n.get("id") == preview_id), None)
    return val(n).strip() if n else ""


fmt_radios = sorted(s.by_role("RadioButton"), key=lambda n: n["id"])[:2]
print(f"clicking Bundle tile {fmt_radios[1].get('label')!r}")
s.call("invoke_action", {"node": fmt_radios[1]["id"], "action": "click"})
time.sleep(0.4)
if not bundle_preview() or bundle_preview().lower().endswith(".skrib"):
    b = fmt_radios[1].get("bounds")
    if b:
        cx = (b.get("x", 0) + b.get("width", 0) / 2) if isinstance(b, dict) else (b[0] + b[2] / 2)
        cy = (b.get("y", 0) + b.get("height", 0) / 2) if isinstance(b, dict) else (b[1] + b[3] / 2)
        s.call("inject_pointer", {"x": cx, "y": cy, "kind": "click"})
        time.sleep(0.4)

bpath = bundle_preview()
print(f"bundle path preview: {bpath!r}")
if not bpath or bpath.lower().endswith(".skrib") or not bpath.lower().endswith("/tidewrack"):
    fail(f"bundle preview should be a bare folder '…/tidewrack', got {bpath!r}",
         s.app, s.mcp, s.log)
print("PASS: switching to Bundle drops the extension (…/tidewrack)")
s.shot("/tmp/sk-new-work-shot.png")


# ── Step 2 (Language & structure) ─────────────────────────────────────────────
def advance():
    """Click Next and give the Switcher a frame to swap the step body."""
    btn = find_next_button()
    if not btn:
        s.dump("looking for Next")
        fail("no 'Next' button to advance the wizard", s.app, s.mcp, s.log)
    res, _ = s.call("invoke_action", {"node": btn["id"], "action": "click"})
    if isinstance(res, dict) and res.get("isError"):
        fail(f"clicking Next errored: {res}", s.app, s.mcp, s.log)
    time.sleep(0.8)


advance()
combos = s.by_role("ComboBox")
print(f"step 2 structure: ComboBox x{len(combos)} "
      f"-> {[c.get('label') for c in combos]}")
# Two: the writing language, and the paratext ("Book structure") preset.
if len(combos) < 2:
    s.dump("step 2")
    fail("expected the language and paratext ComboBoxes on step 2", s.app, s.mcp, s.log)
if s.by_role("RadioGroup"):
    fail("the Template RadioGroup belongs to step 3, not step 2", s.app, s.mcp, s.log)
print("PASS: step 2 shows language + book structure")
s.shot("/tmp/sk-new-work-step2.png")


# ── Step 3 (Template) ─────────────────────────────────────────────────────────
advance()
radiogroups = s.by_role("RadioGroup")
radios = s.by_role("RadioButton")
checkboxes = s.by_role("CheckBox", "Switch")  # the "Flat chapters" toggle
print(f"step 3 structure: RadioGroup x{len(radiogroups)}, RadioButton x{len(radios)}, "
      f"toggle x{len(checkboxes)}")
if not radiogroups:
    s.dump("step 3")
    fail("expected the Template RadioTileGroup on step 3", s.app, s.mcp, s.log)
# Five templates: None / Empty Novel / Light Novel / Novel / Notebook.
if len(radios) < 5:
    fail(f"expected 5 template tiles, got {len(radios)}", s.app, s.mcp, s.log)
if not checkboxes:
    fail("expected the 'Flat chapters' toggle on step 3", s.app, s.mcp, s.log)

create_btn = find_create_button()
if not create_btn:
    s.dump("step 3")
    fail("no 'Create Work' button on the last step", s.app, s.mcp, s.log)
if not node_enabled(create_btn["id"]):
    fail("Create Work should be ENABLED on the last step (name + location are valid)",
         s.app, s.mcp, s.log)
print("PASS: step 3 shows the templates + flat-chapters toggle, Create enabled")
s.shot("/tmp/sk-new-work-step3.png")

s.close()
time.sleep(1.0)


# ══════════════════════════════════════════════════════════════════════════════
# The Launcher's "From documents…" — the same wizard, a different purpose.
# ══════════════════════════════════════════════════════════════════════════════
# This door used to open a bare file picker first and then ask for the very same
# files again in the import wizard afterwards. It now goes straight to this
# wizard, which drops the questions an import answers for itself (template,
# paratexts) and says on its last step what comes next.
#
# Run in an isolated config + a scratch folder, because unlike the phase above it
# does press Finish: the whole point is the handoff — project created, import
# wizard opened over it, files chosen there, once.
print("\n== From documents… ==")
env = fixture.isolated_config(locale="en-US", label="new-work-docs")
target = tempfile.mkdtemp(prefix="new-work-docs-")
s = Session([], env=env)
s.tools()
if not s.wait_label("welcome sections"):
    fail("the Launcher window did not appear", s.app, s.mcp, s.log)

docs_btn = next((n for n in s.nodes() if n.get("role") == "Button"
                 and "from documents" in (n.get("label") or "").lower()), None)
if not docs_btn:
    fail("no 'From documents…' button on the Launcher", s.app, s.mcp, s.log)
s.call("invoke_action", {"node": docs_btn["id"], "action": "click"})
time.sleep(1.2)

# No file picker: the wizard itself is what the click opens. (A native picker is
# a separate OS surface the bridge cannot see — what it *can* see is that the
# wizard is already up, which the old flow could not manage until files had been
# chosen and the dialog dismissed.)
if not s.by_role("Form"):
    s.dump("after 'From documents…'")
    fail("clicking 'From documents…' did not open the New Work wizard directly",
         s.app, s.mcp, s.log)
if not s.find_label_contains("from documents", timeout=3):
    fail("the wizard does not title itself as the from-documents flow",
         s.app, s.mcp, s.log)
print("PASS: 'From documents…' opens the wizard directly, titled for the flow")
s.shot("/tmp/sk-new-work-docs-step1.png")

# Fill step one and walk.
raw, _ = s.call("snapshot_tree")
focus_id = json.loads(raw["content"][0]["text"]).get("focus")
name_field = next((n for n in s.nodes() if n.get("id") == focus_id), None)
if name_field is None or name_field.get("role") != "TextInput":
    fail("the from-documents wizard must open focused on the Work name",
         s.app, s.mcp, s.log)
s.call("type_text", {"node": name_field["id"], "text": "Imported Book"})
time.sleep(0.5)
location_field = next((n for n in s.by_role("TextInput", "TextField")
                       if "/" in val(n) and n["id"] != name_field["id"]), None)
if location_field is None:
    fail("could not identify the Location field", s.app, s.mcp, s.log)
s.call("set_value", {"node": location_field["id"], "value": target})
time.sleep(0.6)

advance()
combos = s.by_role("ComboBox")
print(f"step 2 structure: ComboBox x{len(combos)}")
# One, not two: the paratext picker is absent — with no template there is no
# Book row to furnish, so it could only lie.
if len(combos) != 1:
    s.dump("from-documents step 2")
    fail(f"expected only the language ComboBox on step 2, got {len(combos)}",
         s.app, s.mcp, s.log)
print("PASS: step 2 asks the language only")

advance()
radios = s.by_role("RadioButton")
checkboxes = s.by_role("CheckBox", "Switch")
print(f"step 3 structure: RadioButton x{len(radios)}, toggle x{len(checkboxes)}")
if radios:
    fail(f"the template picker must be gone from a from-documents wizard "
         f"({len(radios)} tiles found) — a template collides with the import",
         s.app, s.mcp, s.log)
if not checkboxes:
    fail("the flat-chapters toggle must stay: it is Work.chapter_mode, which "
         "every chapter the import creates resolves through", s.app, s.mcp, s.log)
create_btn = find_button(("import",), exclude=("cancel", "annuler"))
if not create_btn or not node_enabled(create_btn["id"]):
    s.dump("from-documents step 3")
    fail("expected an enabled 'Create & import…' on the last step", s.app, s.mcp, s.log)
print(f"PASS: step 3 explains what comes next; finish is {create_btn.get('label')!r}")
s.shot("/tmp/sk-new-work-docs-step3.png")

# Finish → the project window, with the import wizard already over it.
s.call("invoke_action", {"node": create_btn["id"], "action": "click"})
deadline = time.time() + 40
opened = False
while time.time() < deadline and not opened:
    opened = any("import documents" in (n.get("label") or "").lower() for n in s.nodes())
    if not opened:
        time.sleep(0.5)
created = os.listdir(target)
print(f"created: {created}")
if not opened:
    s.dump("after Create & import")
    fail("the import wizard did not open over the freshly created project",
         s.app, s.mcp, s.log)
if "imported-book.skrib" not in created:
    fail(f"the project was not created at {target}: {created}", s.app, s.mcp, s.log)
print("PASS: Create & import → project on disk + the import wizard over it")
s.shot("/tmp/sk-new-work-docs-after.png")
s.close()
# The project this phase created is a test artefact, not a fixture: take it with
# us, or every run leaves another one in the scratch directory.
shutil.rmtree(target, ignore_errors=True)

print("\nALL PASS")
sys.exit(0)
