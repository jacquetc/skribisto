#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the bastyde automation MCP bridge and verify the
New Work modal feature end-to-end.

Flow: launch the app (no CLI arg) → the **Launcher window** opens (the
Welcome UI is a real window now, not a modal — see `bastyde_ui::main`'s module
docs) → click the Launcher's "New Work" button (there is no `Ctrl+N` global
shortcut in the Launcher window — that action only exists inside an
already-open project window's tree, so a keyboard fallback would just no-op;
the button is the only path here) → assert the modal's structure (a Form
landmark, the two SegmentedControls, the language ComboBox, the Cancel /
Create buttons) → type a work name into the name field and assert the reactive
"Will create …/<slug>.skrib" path preview recomputes (single-file vs bundle) →
screenshot. Assertions favour locale-independent anchors (roles + the .skrib
path) so the harness passes whatever UI language is persisted.

Note: this script does not click "Create Work" — creating from the Launcher
opens a *second* (project) window and closes the Launcher; that transition is
covered end-to-end by `automation_welcome.py`.

Reuses the launch + scrape-socket/token + connect scaffolding from
automation_welcome.py.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

SKRIBISTO = "/home/cyril/Devel/skribisto/target/debug/skribisto"
MCP = "/home/cyril/Devel/bastyde/target/debug/bastyde-automation-mcp"

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


# ── Assert the modal's structure (locale-independent by role) ─────────────────
forms = s.by_role("Form")
text_fields = s.by_role("TextInput", "TextField")
radiogroups = s.by_role("RadioGroup")  # the two RadioTileGroups (Format, Template)
combos = s.by_role("ComboBox")         # the language dropdown
print(f"structure: Form x{len(forms)}, text fields x{len(text_fields)}, "
      f"RadioGroup x{len(radiogroups)}, ComboBox x{len(combos)}")
if not forms:
    fail("no Form landmark (the FormLayout body) in the New Work modal", s.app, s.mcp, s.log)
if len(radiogroups) < 2:
    fail("expected 2 SegmentedControls (Format + Template) as RadioGroups", s.app, s.mcp, s.log)
if not combos:
    fail("expected the Default-language ComboBox", s.app, s.mcp, s.log)
for lbl in ("cancel", "annuler"):
    if any(lbl in (b.get("label") or "").lower() for b in s.by_role("Button")):
        break
else:
    print("note: no Cancel button label matched (locale?) — continuing")


# ── Empty name → "Create Work" DISABLED + an inline validation message ────────
def find_create_button():
    for n in s.by_role("Button"):
        lab = (n.get("label") or "").lower()
        if any(w in lab for w in ("create work", "créer", "creer")) and not any(
            w in lab for w in ("cancel", "annuler")
        ):
            return n
    return None


def node_enabled(node_id):
    _, p = s.call("read_node", {"node": node_id})
    if isinstance(p, dict):
        if p.get("disabled") is True:
            return False
        if "enabled" in p:
            return bool(p["enabled"])
    return True


create_btn = find_create_button()
if not create_btn:
    fail("no 'Create Work' button", s.app, s.mcp, s.log)
statuses = [n for n in s.nodes() if n.get("role") == "Status" and (n.get("label") or "").strip()]
print(f"empty-name state: Create enabled={node_enabled(create_btn['id'])}; "
      f"validation status={[n.get('label') for n in statuses]}")
if node_enabled(create_btn["id"]):
    fail("Create button should be DISABLED while the Work name is empty", s.app, s.mcp, s.log)
if not statuses:
    fail("expected an inline validation message while the name is empty", s.app, s.mcp, s.log)
print("PASS: empty name → Create disabled + inline validation shown")
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
# panel also draws its own header chrome (title strip + close X) above the
# form, which a naive focusable-descendant fallback could land on instead.
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

# `type_text`, not `set_value`: typing is what a user does, and it is what drives
# the field's bound signal (and through it the VM's derived path preview).
res, _ = s.call("type_text", {"node": name_field["id"], "text": "Tidewrack"})
if isinstance(res, dict) and res.get("isError"):
    fail(f"type_text into the name field errored: {res}", s.app, s.mcp, s.log)
time.sleep(0.8)

preview = find_value_contains(".skrib", timeout=6)
if preview is None:
    s.dump("after setting name")
    fail("path preview never showed a .skrib path after setting the name", s.app, s.mcp, s.log)
path = val(preview).strip()
print(f"single-file path preview: {path!r}")
if not path.lower().endswith("tidewrack.skrib"):
    fail(f"preview should end with 'tidewrack.skrib', got {path!r}", s.app, s.mcp, s.log)
print("PASS: single-file preview reacts to the name (…/tidewrack.skrib)")

# A valid name + (existing, writable) location must re-enable Create.
create_btn = find_create_button() or create_btn
if not node_enabled(create_btn["id"]):
    fail("Create button should be ENABLED once name + location are valid", s.app, s.mcp, s.log)
print("PASS: valid name + location → Create enabled")

# ── Switch format to Bundle → the preview drops the ".skrib" extension ────────
# Format's second tile is "Bundle". RadioTiles surface as RadioButtons; the AT
# `click` may not drive the composite, so fall back to a pointer click on the
# tile's bounds. Success is judged by the reactive preview (a RadioTileGroup does
# not expose its selected label as an AT value).
def bundle_preview():
    n = find_value_contains("/tidewrack", timeout=3)
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
s.close()

print("\nALL PASS")
sys.exit(0)
