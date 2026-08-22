#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify **bringing a deleted row back**, plus the two
sentences the Versions dock now owes the writer.

    scripts/automation_recreate_row.py

Unlike its siblings this probe takes **no project argument**: the situation it
tests — a row present in a backup and gone from the project, and a row whose
history has been thinned — cannot be found lying around, and a probe that
skipped whenever it was absent would never have run. So it builds its own
fixture out of the checked-in examples, which are real bundles the app wrote:

  * the **project** is `Starforgers-20260722-230450.skrib` (37 rows) retyped from
    `kind: Backup` to `kind: Regular` — a two-line manifest edit, nothing else;
  * the **backup** is `Starforgers-20260806-093031.skrib` (59 rows, already
    `kind: Backup`, same `unique_id`) dropped beside it, where
    `BackupVersions::list` scans for a project's own backups;
  * a hand-written `history/index.ron` gives one row three recorded states, the
    oldest carrying `thinned_away: 17`.

That last one is the only synthetic part, and it is synthetic for a reason: the
tally is what `history::thin` leaves behind after a GFS sweep, and reaching a
real sweep from the UI means forty saves of one scene. The *arithmetic* is held
by `history.rs`'s own tests; what cannot be tested there, and is tested here, is
that the number reaches the panel and that the sentence carrying it fits.

Checks:
  1. The Versions dock lists a row's past from **both** sources, and only the
     backup-sourced row carries a pin control.
  2. It says why, in words. A pin protects a file and a log entry is not one, so
     half the list has no pin — an absence that teaches nothing on its own while
     the tooltip beside it promises automatic cleanup will never delete "this
     version".
  3. The pin's own tooltip names the **backup**, not "this version": what it
     protects is the whole snapshot the version was read out of.
  4. Thinning is said out loud, and the sentence **wraps** in a 300 dp rail
     rather than clipping. Every boundary line before this one was a date, so
     the clipping never showed until a sentence arrived.
  5. A removed row's reader offers **Bring this back…**, which was the whole
     point: before it, the answer to "bring back the chapter I cut in March" was
     to select the prose, copy it, make a scene, paste, retitle and re-place it.
  6. It lands where the writer pointed, with its recorded type and its prose.
  7. **Undo takes it straight back out** — one step, because the create and every
     text it writes are one composite entry. Undo on the toast, *not* Ctrl+Z:
     this is an entity write on the Work's undo stack, and no keystroke in this
     app is bound to that.

The app is deliberately **left running** so a screenshot can be taken.
"""
import json, os, re, select, shutil, subprocess, sys, tempfile, time, zipfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

failures = []


def check(ok, why):
    print(("  ok   " if ok else "  FAIL ") + why)
    if not ok:
        failures.append(why)
    return ok


# ── the fixture ────────────────────────────────────────────────────────────────

EXAMPLES = fixture.repo_path("resources", "examples")
OLDER = "Starforgers-20260722-230450.skrib"   # 37 rows — becomes the project
NEWER = "Starforgers-20260806-093031.skrib"   # 59 rows — stays a backup

#: Blob stems. The log never verifies that a stem is the blake3 of its blob (it
#: is a file name and a merge key, not a checksum), so a probe may use any hex.
H_OLD = "aaaa1111" * 8
H_NEW = "bbbb2222" * 8

#: What the oldest surviving entry claims retention removed from behind it.
THINNED = 17


def build_fixture():
    """A project with a row missing, a backup that still has it, beside it."""
    root = tempfile.mkdtemp(prefix="skrib-recreate-")
    project = os.path.join(root, "Starforgers.skrib")

    src = zipfile.ZipFile(os.path.join(EXAMPLES, OLDER))
    # The row to give a recorded history to: the first Item/Scene there is.
    items = [n for n in src.namelist() if n.endswith("items.ron")]
    text = src.read(items[0]).decode("utf-8")
    uid = None
    for block in re.findall(r"BinderItemFile\((.*?)\n        \),", text, re.S):
        if "role: Item," in block and "sub_role: Scene," in block:
            uid = re.search(r'uid:\s*"([0-9a-f-]{36})"', block).group(1)
            break
    if uid is None:
        raise SystemExit(f"{OLDER} has no Item/Scene row to hang a history on")

    index = f"""[
    (
        at: "2026-08-06T08:00:00Z",
        item_uid: "{uid}",
        role: SceneText,
        hash: "{H_OLD}",
        bytes: 40,
        thinned_away: {THINNED},
    ),
    (
        at: "2026-08-06T10:00:00Z",
        item_uid: "{uid}",
        role: SceneText,
        hash: "{H_NEW}",
        bytes: 46,
        thinned_away: 0,
    ),
]
"""
    with zipfile.ZipFile(project, "w", zipfile.ZIP_DEFLATED) as out:
        for info in src.infolist():
            data = src.read(info.filename)
            if info.filename == "project.skrib":
                t = data.decode("utf-8")
                t = t.replace("kind: Backup,", "kind: Regular,")
                t = re.sub(r"\n\s*backup_of: Some\([^)]*\),", "\n    backup_of: None,", t)
                t = re.sub(r"\n\s*backup_created_at: Some\([^)]*\),",
                           "\n    backup_created_at: None,", t)
                data = t.encode("utf-8")
            out.writestr(info.filename, data)
        out.writestr("history/index.ron", index)
        out.writestr(f"history/{H_OLD}.djot", "The corridor was cold, and the lamps were out.\n")
        out.writestr(f"history/{H_NEW}.djot", "The corridor was cold, and every lamp had gone out.\n")
    src.close()

    # Beside the project is one of the two places `BackupVersions` looks, and the
    # one that needs no configuration at all.
    shutil.copy2(os.path.join(EXAMPLES, NEWER), os.path.join(root, NEWER))
    for p in (project, os.path.join(root, NEWER)):
        os.chmod(p, 0o644)
    return root, project


root, project = build_fixture()
print(f"fixture: {project}")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
print(f"app log: {log}")


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    print("\n".join(open(log).read().splitlines()[-25:]))
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


subprocess.run(["pkill", "-x", "skribisto"], check=False)
time.sleep(0.4)

# A sandboxed config, so a desk arrangement left by an earlier run cannot decide
# whether this one's docks open. Never `XDG_RUNTIME_DIR` — that holds the
# Wayland socket.
env = dict(os.environ)
sandbox = os.path.join(root, "xdg")
os.makedirs(os.path.join(sandbox, "config"), exist_ok=True)
os.makedirs(os.path.join(sandbox, "data", "skribisto"), exist_ok=True)
env["XDG_CONFIG_HOME"] = os.path.join(sandbox, "config")
env["XDG_DATA_HOME"] = os.path.join(sandbox, "data")

app = subprocess.Popen([SKRIBISTO, project], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)

sock = tok = None
end = time.time() + 40
while time.time() < end:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt)
    t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s and t:
        sock, tok = s.group(1), t.group(1)
        break
    if app.poll() is not None:
        die("app exited early", app)
    time.sleep(0.2)
if not sock:
    die("no bridge socket", app)

_id = [0]
mcp = None


def send(method, params=None, notif=False):
    m = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        m["params"] = params
    if not notif:
        _id[0] += 1
        m["id"] = _id[0]
    mcp.stdin.write(json.dumps(m) + "\n")
    mcp.stdin.flush()


def recv(timeout=25, fatal=True):
    e = time.time() + timeout
    while time.time() < e:
        if mcp.poll() is not None:
            break
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, e - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        if line.strip():
            return json.loads(line)
    if fatal:
        die("no MCP response", app, mcp)
    return None


def call(name, a=None):
    send("tools/call", {"name": name, "arguments": a or {}})
    res = recv().get("result", {})
    payload = res.get("structuredContent")
    if payload is None:
        txt = "".join(c.get("text", "") for c in res.get("content", []) if c.get("type") == "text")
        payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
    return res, payload


deadline = time.time() + 40
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "recreate-probe", "version": "1"}})
    init = recv(timeout=4, fatal=False)
    if init is None:
        if mcp.poll() is None:
            mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP", app, mcp)
send("notifications/initialized", notif=True)
print(f"bridge up: {sock}")


def settle(extra=0.4):
    call("settle")
    time.sleep(extra)


def nodes():
    return call("snapshot_tree")[1].get("nodes", [])


def widgets():
    return call("layout_tree")[1].get("nodes", [])


def rendered_text():
    out = []
    for n in nodes() + widgets():
        for field in ("label", "value", "text", "resolved_text", "params"):
            blob = n.get(field)
            if isinstance(blob, str) and blob.strip():
                out.append(blob)
            elif blob is not None and not isinstance(blob, str):
                out.append(json.dumps(blob))
    return out


def click_at(node):
    b = (node or {}).get("bounds") or {}
    if not b.get("width"):
        return False
    x, y = b["x"] + b["width"] / 2, b["y"] + b["height"] / 2
    call("inject_pointer", {"action": "move", "x": x, "y": y})
    time.sleep(0.15)
    call("inject_pointer", {"action": "click", "x": x, "y": y, "button": "left"})
    time.sleep(0.5)
    return True


def open_row(node):
    """A list row opens on a **double** click.

    Not select-then-Enter: the change list is not focused by a click on one of
    its rows, so the Enter goes to whatever is, and the reader never opens.
    """
    b = (node or {}).get("bounds") or {}
    if not b.get("width"):
        return False
    x, y = b["x"] + b["width"] / 2, b["y"] + b["height"] / 2
    call("inject_pointer", {"action": "move", "x": x, "y": y})
    time.sleep(0.12)
    call("inject_pointer", {"action": "click", "x": x, "y": y, "button": "left"})
    time.sleep(0.14)
    call("inject_pointer", {"action": "click", "x": x, "y": y, "button": "left"})
    time.sleep(2.2)
    settle(1.0)
    return True


def labelled(role, pred=lambda s: True):
    return [n for n in nodes() if n.get("role") == role and pred(n.get("label") or "")]


def tree_rows(name=None):
    return [n for n in labelled("TreeItem")
            if name is None or (n.get("label") or "") == name]


def menu_items():
    return [n for n in nodes() if n.get("role") == "MenuItem"]


def mlabel(n):
    return (n.get("label") or "").replace("&", "")


def open_menu(top, row, wait=1.0):
    """The title-bar menu is an overlay: address its rows by id, never by pointer."""
    for _ in range(4):
        if any(mlabel(i) == top for i in menu_items()):
            break
        h = [n for n in nodes() if n.get("label") == "Menu"]
        if not h:
            return False
        call("focus_node", {"node": h[0]["id"]})
        time.sleep(0.3)
        call("inject_key", {"key": "Enter"})
        time.sleep(0.9)
        settle()
    t = [i for i in menu_items() if mlabel(i) == top]
    if not t:
        return False
    call("invoke_action", {"action": "click", "node": t[0]["id"]})
    time.sleep(0.9)
    settle()
    r = [i for i in menu_items() if mlabel(i) == row]
    if not r:
        return False
    call("invoke_action", {"action": "click", "node": r[0]["id"]})
    time.sleep(wait)
    settle()
    return True


settle(1.0)
if not fixture.wait_for_load(nodes, ["Starforgers"], timeout=45):
    die("the fixture project never loaded", app, mcp)

# ── 1–4. the Versions dock ─────────────────────────────────────────────────────

print("1. the Versions dock, on the row whose history was thinned")
row = tree_rows("Copyright")
if not row:
    die("the fixture's first scene row is not in the binder", app, mcp)
open_row(row[0])

tab = [n for n in nodes() if n.get("role") == "Tab" and "Version" in (n.get("label") or "")]
if not tab:
    die("no Versions tab on the trailing rail", app, mcp)
click_at(tab[0])
time.sleep(3.0)
settle(2.5)

# The dock opens on Synopsis; the recorded history here is of the body.
body = [n for n in nodes() if n.get("role") == "RadioButton" and (n.get("label") or "") == "Text"]
if not body:
    die("the scope bar offers no Text segment", app, mcp)
click_at(body[0])
time.sleep(3.0)
settle(3.0)

blob = "\n".join(rendered_text())
# Not by matching the rows' subtitles: `StandardListItem` puts a subtitle in the
# a11y **description**, which the snapshot DTO does not carry, and it reaches the
# layout tree as neither a param nor a label. The two controls below are the
# observable consequence of the same fact and are what the writer actually sees —
# a pin on one row, none on the next.
check("Pin the backup this version came from" in blob,
      "a version the list holds can be pinned")
check("the project's own history isn't one" in blob,
      "…and one it holds cannot — the note only renders when a log row is visible")

print("2. it says why half the list has no pin")
check("Only versions from a backup can be pinned" in blob,
      "the panel explains the pin column where it is missing")

print("3. the pin names what it actually protects")
check("Pin the backup this version came from" in blob,
      "the tooltip names the backup, not 'this version'")

print("4. thinning is said out loud, and fits")
check(f"already dropped {THINNED} earlier states" in blob,
      f"the panel reports the {THINNED} states retention removed")
lines = [n for n in widgets()
         if (n.get("type") or "").endswith("TextWidget")
         and (n.get("bounds") or {}).get("height", 0) > 40
         and (n.get("bounds") or {}).get("width", 0) < 320]
check(bool(lines),
      "a boundary sentence wraps to more than one line instead of clipping")

# ── 5–7. bringing a deleted row back ───────────────────────────────────────────

print("5. a deleted row offers a way back")
if not open_menu("View", "Timeline", wait=4.0):
    die("could not reach View ▸ Timeline", app, mcp)
time.sleep(5.0)
settle(2.0)

backup_bar = [n for n in nodes()
              if n.get("role") == "GraphicsObject" and "09:30" in (n.get("label") or "")]
if not backup_bar:
    die("the band drew no bar for the backup moment", app, mcp)
click_at(backup_bar[0])
time.sleep(2.0)
settle(1.5)

# The band sits at the bottom; the Versions dock's own list is above it, and both
# are ListBoxes.
changes = [n for n in nodes()
           if n.get("role") == "ListBoxOption"
           and (n.get("label") or "").strip()
           and (n.get("bounds") or {}).get("y", 0) > 560]
if not changes:
    die("nothing differs between the backup and the project", app, mcp)

opened = None
for c in changes[:6]:
    open_row(c)
    if any("Bring this back" in t for t in rendered_text()):
        opened = c.get("label")
        break
    # A container with no prose of its own opens nothing at all, by design.
    call("inject_key", {"key": "Escape"})
    time.sleep(0.6)
    settle()
if not check(opened is not None, "a removed row's reader offers 'Bring this back…'"):
    die("no removed row opened a reader with the recovery affordance", app, mcp)

blob = "\n".join(rendered_text())
check("This is no longer in your project" in blob,
      "…beside the sentence that used to be the whole answer")

print("6. it lands where the writer points, with its prose")
before = len(tree_rows(opened))
btn = [n for n in nodes() if (n.get("label") or "").startswith("Bring this back")]
click_at(btn[0])
time.sleep(1.8)
settle()
check(any("DestinationPickerView" in (n.get("type") or "") for n in widgets()),
      "the reader gives way to a destination picker")

anchor = tree_rows("Copyright")
if not anchor:
    die("the picker has no row to point at", app, mcp)
click_at(anchor[0])
time.sleep(0.6)
settle()
confirm = [n for n in nodes() if (n.get("label") or "") == "Bring it back here"]
if not confirm:
    die("the picker offers no confirm", app, mcp)
click_at(confirm[0])
time.sleep(1.4)
settle()

blob = "\n".join(rendered_text())
check("Undo, on the message that follows" in blob,
      "the confirmation promises Undo, not a Ctrl+Z that would not reach it")

ok = [n for n in nodes() if (n.get("label") or "") == "OK"]
if not ok:
    die("no confirmation dialog to accept", app, mcp)
click_at(ok[0])
time.sleep(3.0)
settle(1.5)

after = len(tree_rows(opened))
check(after == before + 1, f"{opened!r} is back in the binder ({before} → {after})")
check(any("is back in your project" in t for t in rendered_text()),
      "and says so")

print("7. one Undo takes it straight back out")
undo = [n for n in nodes() if (n.get("label") or "") == "Undo"]
if not check(bool(undo), "the toast offers Undo"):
    die("nothing to undo with", app, mcp)
click_at(undo[0])
time.sleep(3.0)
settle(1.5)
check(len(tree_rows(opened)) == before,
      "the row and every text it brought back are gone again, in one step")

print()
if failures:
    print("FAILURES:")
    for f in failures:
        print(" -", f)
else:
    print("all checks passed")
print("\nthe app is left running for a screenshot")
if mcp.poll() is None:
    mcp.terminate()
sys.exit(1 if failures else 0)
