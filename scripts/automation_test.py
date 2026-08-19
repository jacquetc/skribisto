#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto via the teksilo automation MCP bridge and assert that
the project given on the command line actually loaded.

    scripts/automation_test.py                  # the bundled example
    scripts/automation_test.py PROJECT.skrib    # any project, zip or folder

Launches `skribisto <project>` (debug, with the automation bridge), reads the
bridge socket + token from its stderr, connects `teksilo-automation-mcp
--connect`, performs the MCP handshake, then *polls* the AccessKit tree until the
loaded work's content appears (the launch-load is driven by a backend event that
takes a few UI-thread ticks to reflect — a single early snapshot is stale).
Saves a screenshot of the settled state to $SHOT_DIR (default /tmp).

**What "loaded" means here.** The markers are read out of the project's own
manifest before launching — its work title, its binder names, its item titles —
so the assertion is about the file actually handed over, not about this repo's
example. It used to be a disjunction of three guesses at the example's strings
plus `len(nodes) > 8`, and that last clause silently made the other three
optional: the Welcome screen alone carries ~15 labelled nodes, so "more than
eight labelled nodes and no 'No work loaded'" is true of essentially any window
that opened at all — including one that opened empty. A probe reporting PASS in
that state is worse than no probe.
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time, zipfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
OUT = os.environ.get("SHOT_DIR", "/tmp")
# Resolve to an absolute path — the launched app resolves a relative path against
# its own working directory, which is not this script's.
SOURCE = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else
    fixture.repo_path("resources/examples/Starforgers.skrib"))


def fail(msg, app=None, mcp=None, log=None, mcp_err=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-20:]))
    if mcp_err:
        try:
            et = open(mcp_err).read()
            if et.strip():
                print("--- mcp stderr ---")
                print("\n".join(et.splitlines()[-15:]))
        except OSError:
            pass
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


# ---------------------------------------------------------------------------
# What this particular project should put on screen
# ---------------------------------------------------------------------------

def _ron_str(escaped):
    r"""Decode one RON string body (the bytes between its quotes).

    RON writes strings through Rust's `char::escape_debug`, so a title carrying a
    quote is stored as `\"` and a backslash as `\\`. Python's `unicode_escape`
    handles both, as well as `\n` and `\t`, and leaves ordinary text untouched.

    Decoding matters because the work title is this probe's *required*
    assertion: a book called `The "Chosen" One` is stored as
    `title: "The \"Chosen\" One"`, and an escape-blind regex would capture
    `The \\` — which no a11y tree can contain, so the probe would fail forever
    on a project that loads perfectly.
    """
    try:
        return escaped.encode("latin-1", "backslashreplace").decode("unicode_escape")
    except (UnicodeDecodeError, UnicodeEncodeError):
        return escaped


# One RON string field: `key: "…"`, honouring backslash escapes so an embedded
# quote does not end the match early.
def _field(key, text):
    m = re.search(key + r':\s*"((?:[^"\\]|\\.)*)"', text)
    return _ron_str(m.group(1)) if m else None


def _fields(key, text):
    return [_ron_str(m) for m in
            re.findall(key + r':\s*"((?:[^"\\]|\\.)*)"', text)]


def _manifest_texts(project):
    """(work_title, [binder names], [item titles]) read from a `.skrib`.

    Both bundle shapes: a zip, or the exploded folder `save_as` can write. The
    RON here is flat enough that a regex beats a parser — `title:`/`name:` are
    plain quoted strings (`skrib_format::bundle`'s `WorkFile.title`,
    `BinderFile.name`, `BinderItemFile.title`) — and the alternative is teaching
    Python to read RON for three fields.
    """
    def read(entry):
        if os.path.isdir(project):
            path = os.path.join(project, entry)
            return open(path, encoding="utf-8").read() if os.path.exists(path) else None
        try:
            return zip_.read(entry).decode("utf-8")
        except KeyError:
            return None

    zip_ = None if os.path.isdir(project) else zipfile.ZipFile(project)
    try:
        manifest = read("project.skrib")
        if manifest is None:
            raise ValueError(f"{project} has no project.skrib — not a .skrib bundle?")
        # Anchor on `work:` rather than taking the first `title:` in the file.
        # `ProjectManifest`'s fields before `work` carry no string today, so the
        # two are equivalent right now — but the manifest gains additive fields
        # every few format versions, and one named `*title*` landing above `work`
        # would silently retarget the assertion at something else.
        title = _field("title", manifest.split("work:", 1)[-1])

        if os.path.isdir(project):
            bdir = os.path.join(project, "binders")
            entries = [f"binders/{d}/items.ron" for d in sorted(os.listdir(bdir))] \
                if os.path.isdir(bdir) else []
        else:
            entries = [n for n in zip_.namelist()
                       if re.fullmatch(r"binders/[^/]+/items\.ron", n)]

        binders, items = [], []
        for entry in sorted(entries):
            text = read(entry)
            if not text:
                continue
            # `ItemsFile(binder: BinderFile(…), items: [ … ])`. Split at the
            # field key on its own line, not at the first "items:" anywhere:
            # a binder the writer renamed to `Notes (items: to sort)` would
            # otherwise split inside its own name and lose it.
            cut = re.search(r"^\s*items:\s*\[", text, re.M)
            head, tail = (text[:cut.start()], text[cut.start():]) if cut else (text, "")
            name = _field("name", head)
            if name:
                binders.append(name)
            items += _fields("title", tail)
        return (title or ""), binders, items
    finally:
        if zip_ is not None:
            zip_.close()


# A live probe drives a *real* app: it migrates, it autosaves, it saves on close.
# Opening the checked-in example directly would rewrite it — see
# `automation_fixture`'s module doc.
fixture.assert_no_running_instance(SKRIBISTO)
PROJECT = fixture.working_copy(SOURCE, "smoke")

work_title, binder_names, item_titles = _manifest_texts(PROJECT)
# Required: the work title reaches the a11y tree through the title bar's own
# TextWidget, which every project window mounts regardless of layout. Nothing
# else is unconditional — the Outline can be hidden, and a tab need not be open.
if not work_title:
    fail(f"{SOURCE} declares no work title; nothing in it can be asserted on")
# Corroborating: any binder name or item title, which can only come from the
# loaded tree. Matched only when at least 3 characters long — a title like "I"
# or "II" occurs by accident in a tree full of prose and chrome, so finding one
# would prove nothing.
titles = binder_names + item_titles
content = [t for t in titles if len(t) > 2]
print(f"project   : {os.path.basename(SOURCE)}")
print(f"work title: {work_title!r}  ({len(content)} of {len(titles)} binder/item "
      f"titles usable as markers)")
if titles and not content:
    print("note      : every binder/item title is 1-2 characters — too short to "
          "match on, so only the work title is asserted below")
elif not titles:
    print("note      : this project declares no binder or item titles, so only "
          "the work title is asserted below")

# ---------------------------------------------------------------------------
# 1. Launch the app with the project; bridge prints socket+token to stderr.
# ---------------------------------------------------------------------------
env = fixture.isolated_config(locale="en-US", label="smoke", show_welcome=False)
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, PROJECT], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
sock = tok = None
deadline = time.time() + 60
while time.time() < deadline:
    txt = open(log).read()
    s = re.search(r"bridge socket = (\S+)", txt)
    t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
    if s and t:
        sock, tok = s.group(1), t.group(1)
        break
    if app.poll() is not None:
        fail("app exited before printing the bridge socket", app, None, log)
    time.sleep(0.2)
if not sock:
    fail("no bridge socket within 60s", app, None, log)

# The bridge binds and *then* announces, so the path is connectable the instant
# it is printed. Asserting that here is what lets everything below connect once
# instead of retrying: if this ever fires, the announcement has drifted back
# ahead of the bind (teksilo `automation_bridge::spawn_bridge_thread`) and every
# client of this bridge is racing again, not just this one.
if not os.path.exists(sock):
    fail(f"bridge announced {sock} before binding it — the announce-before-bind "
         f"race is back; see teksilo automation_bridge::spawn_bridge_thread",
         app, None, log)
print(f"bridge up : socket={sock} token={tok[:8]}… (existed when announced)")

# ---------------------------------------------------------------------------
# 2. Connect the MCP server to the live app.
# ---------------------------------------------------------------------------
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                       stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                       stderr=open(mcp_err, "w"), text=True, bufsize=1)

_id = [0]
def send(method, params=None, notif=False):
    msg = {"jsonrpc": "2.0", "method": method}
    if params is not None:
        msg["params"] = params
    if not notif:
        _id[0] += 1
        msg["id"] = _id[0]
    mcp.stdin.write(json.dumps(msg) + "\n")
    mcp.stdin.flush()

def recv(timeout=20, fatal=True):
    end = time.time() + timeout
    while time.time() < end:
        if mcp.poll() is not None:
            break  # server exited — its stderr says why
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, end - time.time()))
        if not r:
            break
        line = mcp.stdout.readline()
        if not line:
            break
        line = line.strip()
        if line:
            return json.loads(line)
    if fatal:
        fail("no MCP response within timeout", app, mcp, log, mcp_err)
    return None

def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = recv().get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(c.get("text", "") for c in result.get("content", [])
                       if c.get("type") == "text")
        payload = json.loads(text) if text.strip().startswith("{") else {}
    return result, payload

send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                    "clientInfo": {"name": "skribisto-automation-test", "version": "1"}})
init = recv(timeout=20)
server = init.get("result", {}).get("serverInfo", {})
print("connected :", server.get("name", "?"), server.get("version", ""))
send("notifications/initialized", notif=True)

# ---------------------------------------------------------------------------
# 3. Poll the AccessKit tree until this project's own text appears.
# ---------------------------------------------------------------------------
labels, nodes = [], []
found_title, found_content = False, []
end = time.time() + 60
while time.time() < end:
    _, payload = call("snapshot_tree")
    nodes = payload.get("nodes", [])
    labels = [n.get("label") for n in nodes if n.get("label")]
    values = [n.get("value") for n in nodes if n.get("value")]
    joined = " | ".join(labels + values)
    found_title = work_title in joined
    found_content = [t for t in content if t in joined]
    if found_title and (found_content or not content):
        break
    time.sleep(0.5)

print(f"snapshot  : {len(nodes)} nodes, {len(labels)} labelled")
print("labels    :", " | ".join(labels)[:600])

# Screenshot the settled state for visual inspection.
shot = os.path.join(OUT, "sk-auto-shot.png")
try:
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            with open(shot, "wb") as fh:
                fh.write(base64.b64decode(c["data"]))
            print("screenshot:", shot)
        elif c.get("type") == "text":
            print("screenshot note:", c.get("text", "")[:160])
except SystemExit:
    raise
except Exception as e:
    print("screenshot failed:", e)

# ---------------------------------------------------------------------------
# 4. Tidy up and report.
# ---------------------------------------------------------------------------
mcp.terminate()
app.terminate()
try:
    app.wait(timeout=5)
except subprocess.TimeoutExpired:
    app.kill()

if not found_title:
    fail(f"the work title {work_title!r} never appeared in the a11y tree — "
         f"'{os.path.basename(SOURCE)}' did not load", log=log, mcp_err=mcp_err)
if content and not found_content:
    fail(f"the title bar shows {work_title!r} but not one of this project's "
         f"{len(content)} binder/item titles reached the tree — the work is "
         f"named but its contents are missing", log=log, mcp_err=mcp_err)
if content:
    print(f"PASS: '{os.path.basename(SOURCE)}' loaded — title {work_title!r} and "
          f"{len(found_content)} of its own binder/item titles are in the live "
          f"a11y tree")
else:
    print(f"PASS: '{os.path.basename(SOURCE)}' loaded — title {work_title!r} is in "
          f"the live a11y tree (no title long enough to corroborate with)")
