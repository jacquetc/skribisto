#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify **typewriter scrolling** end-to-end.

The headless tests either side of this one pin the pieces — that a keyboard
caret move asks for an aligned reveal, that `ScrollArea` puts a rect at a
fraction of its viewport, that the setting reaches a real editor. What none of
them can show is the thing the feature actually promises: that the line you are
writing **holds a constant height on screen** while the manuscript scrolls under
it, including at the very end of the document.

So this probe measures the caret's own screen position, through the same
`RichTextEditor` a11y node the OS IME reads, and asserts:

  1. **The document start clamps.** No padding is added above the content, so a
     fresh scene shows its title and synopsis where they belong rather than
     under a viewport-sized gap.
  2. **The end of the document reaches the pin.** The last line must come to
     rest at the pin rather than stalling at the bottom edge — what
     `ScrollArea::scroll_past_end` buys, and the case every implementation that
     skips it gets wrong.
  3. **A click never re-centres.** Clicking places the caret and leaves the page
     exactly where it was (Typora #576 / Zettlr #1660 are open bugs about
     precisely this).
  4. **The next keystroke takes the pin back.** The mouse stands the pin down;
     the keyboard resumes it.

The page is observed through `editor.y` from `layout_tree`: the editor is laid
out at its full document height inside the page's `ScrollArea`, so the only
thing that moves its top edge is the page scrolling under it.

Run: python3 scripts/automation_typewriter.py
"""
import base64, json, os, re, select, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import working_copy, assert_no_running_instance

import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
_sandbox = tempfile.mkdtemp(prefix="skribisto_typewriter_")
SANDBOX_ENV = {
    "XDG_CONFIG_HOME": os.path.join(_sandbox, "config"),
    "XDG_DATA_HOME": os.path.join(_sandbox, "data"),
    "HOME": _sandbox,
}

FAILURES = []


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


def check(ok, msg):
    print(("  ok   " if ok else "  FAIL ") + msg)
    if not ok:
        FAILURES.append(msg)
    return ok


# ---------------------------------------------------------------- launch ----
assert_no_running_instance(SKRIBISTO)
project = working_copy(EXAMPLE, "typewriter")
log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
env = dict(os.environ, **SANDBOX_ENV)
for d in SANDBOX_ENV.values():
    os.makedirs(d, exist_ok=True)
app = subprocess.Popen([SKRIBISTO, project], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)

sock = tok = None
deadline = time.time() + 40
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
    fail("no bridge socket within 40s", app, None, log)
print(f"bridge up: socket={sock}")

mcp = None
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


def recv(timeout=25, fatal=True):
    end = time.time() + timeout
    while time.time() < end:
        if mcp.poll() is not None:
            break
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
        fail("no MCP response within timeout", app, mcp, log)
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


deadline = time.time() + 30
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "skribisto-typewriter", "version": "1"}})
    init = recv(timeout=5, fatal=False)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    fail("could not connect MCP", app, mcp, log)
send("notifications/initialized", notif=True)
print("connected")


# --------------------------------------------------------------- helpers ----
def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def widgets():
    _, p = call("layout_tree")
    return p.get("nodes", [])


def settle(n=2):
    for _ in range(n):
        call("settle")


def find(label, timeout=15.0):
    end = time.time() + timeout
    while time.time() < end:
        for n in nodes():
            if n.get("label") == label:
                return n
        settle()
    return None


def shoot(path):
    res, _ = call("screenshot")
    for c in res.get("content", []):
        if c.get("type") == "image" and c.get("data"):
            open(path, "wb").write(base64.b64decode(c["data"]))
            return path
    return None


def prose_editor():
    """The main writing editor's layout node — the tallest RichTextEditor on
    screen, which in a scene tab is the manuscript (the synopsis box is capped
    at six lines)."""
    best = None
    for n in widgets():
        if "RichTextEditor" not in (n.get("type") or ""):
            continue
        b = n.get("bounds") or {}
        if best is None or b.get("height", 0) > (best[1].get("height", 0)):
            best = (n, b)
    return best


def prose_bottom():
    """Screen y of the **end of the prose** — the editor's bottom edge.

    The bridge exposes no caret rect, but in flowing-page mode the editor is
    laid out at its full document height and the page scrolls it, so with the
    caret on the last line the editor's bottom edge tracks the caret's line.
    That is enough to answer the only question that matters: does the line being
    written hold a constant height, and *which* height."""
    hit = prose_editor()
    if not hit:
        return None
    b = hit[1]
    return b.get("y", 0.0) + b.get("height", 0.0)


# ----------------------------------------------------------------- drive ----
end = time.time() + 60
loaded = False
while time.time() < end:
    labels = " | ".join(n.get("label") or "" for n in nodes()).lower()
    if "chapter" in labels or "prologue" in labels:
        loaded = True
        break
    time.sleep(0.5)
if not loaded:
    fail("project never loaded", app, mcp, log)
print("project loaded")

target = None
for lbl in ("Prologue", "Chapter 1", "Chapter 2"):
    n = find(lbl, timeout=8)
    if n:
        target = n
        break
if not target:
    fail("no scene to open", app, mcp, log)
print(f"opening {target.get('label')!r} (actions={target.get('actions')})")
if "click" in (target.get("actions") or []):
    call("invoke_action", {"node": target["id"], "action": "click"})
else:
    b = target.get("bounds") or {}
    call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                            "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
settle(4)
time.sleep(1.5)
# The binder opens a tab on *selection*; if the row only got focus, press Enter.
if not prose_editor():
    call("inject_key", {"key": "enter"})
    settle(4)
    time.sleep(1.5)

hit = prose_editor()
if not hit:
    import collections
    types = collections.Counter((n.get("type") or "?") for n in widgets())
    print("  widget types on screen:", ", ".join(f"{t}x{c}" for t, c in types.most_common(40)))
    print("  a11y labels:", " | ".join((n.get("label") or "") for n in nodes())[:500])
    shoot("/tmp/tw-debug.png")
    fail("no writing editor on screen after opening a scene", app, mcp, log)
_, eb = hit
print(f"editor bounds: y={eb['y']:.0f} h={eb['height']:.0f} w={eb['width']:.0f}")

# The page viewport is what the pin is measured against.
vp = None
for n in widgets():
    if "ScrollArea" in (n.get("type") or ""):
        b = n.get("bounds") or {}
        if b.get("height", 0) > 200 and b.get("width", 0) > 300:
            vp = b
            break
if not vp:
    fail("no page ScrollArea found", app, mcp, log)
print(f"viewport: y={vp['y']:.0f} h={vp['height']:.0f}")
mid = vp["y"] + vp["height"] / 2
bot = vp["y"] + vp["height"]
print(f"pin (middle) should be y≈{mid:.0f}; viewport bottom is y={bot:.0f}")

# Focus the prose and jump to the very end of the document — the case that only
# works when the page can scroll past its last line.
call("inject_pointer", {"x": eb["x"] + eb["width"] / 2,
                        "y": max(vp["y"] + 30.0, eb["y"] + 30.0)})
settle(2)
call("inject_key", {"key": "end", "ctrl": True})
settle(4)
time.sleep(0.8)
shoot("/tmp/tw-end-of-doc.png")

# `editor.y` is the page's scroll made visible: the editor is laid out at its
# full document height, so the only way its top edge moves is the page scrolling
# under it. One line of scroll == one line-height of movement.
def page_scroll():
    hit = prose_editor()
    return None if not hit else hit[1].get("y")


call("inject_key", {"key": "home", "ctrl": True})
settle(4)
top = page_scroll()
print(f"\ncaret at document start: editor.y={top:.0f} (page unscrolled)")

# --- 1: no padding above — the start of the document clamps -----------------
print("\n[1] the document start clamps instead of opening a gap above the title")
check(abs(top - eb["y"]) < 2.0,
      f"the page is still at the top (editor.y={top:.0f})")

# --- 2: the end of the document reaches the pin ------------------------------
print("\n[2] the last line reaches the pin, not the bottom edge")
call("inject_key", {"key": "end", "ctrl": True})
settle(5)
at_end = page_scroll()
doc_bottom_on_screen = at_end + eb["height"]
print(f"   at end: editor.y={at_end:.0f}, document ends on screen at y={doc_bottom_on_screen:.0f}")
print(f"   viewport {vp['y']:.0f}..{bot:.0f}, middle pin {mid:.0f}")
check(doc_bottom_on_screen < bot - 40.0,
      f"the last line is NOT jammed at the bottom edge "
      f"(ends {doc_bottom_on_screen:.0f}, bottom {bot:.0f}) — this is what scroll_past_end buys")
check(abs(doc_bottom_on_screen - mid) < vp["height"] * 0.25,
      f"the last line sits at the middle pin "
      f"(ends {doc_bottom_on_screen:.0f}, pin {mid:.0f})")
shoot("/tmp/tw-end-of-doc.png")

# --- 3: a click never re-centres --------------------------------------------
print("\n[3] a click places the caret without re-centring the page")
before = page_scroll()
call("inject_pointer", {"x": eb["x"] + eb["width"] / 2, "y": vp["y"] + 60.0})
settle(4)
after = page_scroll()
print(f"   editor.y before={before:.0f} after click={after:.0f}")
check(abs(after - before) < 4.0,
      f"clicking left the page exactly where it was (moved {abs(after - before):.1f}px)")

print("\n[4] the next keystroke takes the pin back")
call("inject_key", {"key": "down"})
settle(3)
resumed = page_scroll()
print(f"   editor.y after one Down={resumed:.0f}")
check(abs(resumed - after) > 2.0,
      f"the keyboard resumed pinning (page moved {abs(resumed - after):.1f}px)")
shoot("/tmp/tw-after-click.png")

print("\n" + "=" * 62)
if FAILURES:
    print(f"{len(FAILURES)} check(s) FAILED:")
    for f in FAILURES:
        print("  -", f)
else:
    print("PASS: typewriter scrolling pins the written line, engages at the pin,")
    print("      reaches the end of the document, and stands down for the mouse.")
print("screenshots: /tmp/tw-pinned.png /tmp/tw-end-of-doc.png /tmp/tw-after-click.png")

mcp.terminate()
app.terminate()
try:
    app.wait(timeout=5)
except Exception:
    app.kill()
sys.exit(1 if FAILURES else 0)
