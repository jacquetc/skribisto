#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Walk the open/close cycle that grows the process, and record where the bytes go.

This is the empirical half of the memory investigation. It reproduces the scenario
the growth was first noticed in (launcher, open a project, open the Full Book
stream, close the tab, close the project, repeat) and at every step it records
three numbers that answer three different questions:

* **rss**: what the kernel says the process occupies. The number the user sees.
  It has two halves, and conflating them is the first trap: an unoptimised debug
  binary is hundreds of megabytes of *file-backed* pages, which no allocator ever
  touched and no fix can return.
* **anon**: the anonymous half of RSS, meaning heap, thread stacks and anonymous mmaps.
* **live**: what the program itself is holding, from the counting allocator in
  `teksilo_ui::memprof`. Allocated minus deallocated, exactly.
* **overhead**: `anon - live`, what the allocator is sitting on after the program
  gave it back. Fragmentation and arena retention, and nothing else.

A rise in `live` across a full cycle is a leak and no argument about allocators
touches it. A rise in `overhead` alone is not a leak, and the `trim` probe after
each teardown settles which one it is: `malloc_trim(0)` returns free pages at the
top of every arena, so if RSS falls there, the bytes were never live.

Needs a build with the instrumentation on:

    cargo build -p teksilo_ui --bin skribisto --features memprof
    scripts/automation_memory_profile.py [--cycles 3] [PROJECT.skrib]

With no project argument it uses the bundled StarForgers example, through
`working_copy` like every other probe, so autosave cannot touch the checked-in
fixture.
"""

import argparse
import json
import os
import re
import select
import subprocess
import sys
import tempfile
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

#: Everything left of this x is the binder dock; the editor pane is to its right.
PANE_X = 300
#: How long to let the process settle before sampling a step. Two sampler ticks
#: plus slack, so the row that carries a step's label is a row taken after the
#: step finished rather than during it.
SETTLE_S = 2.5

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("project", nargs="?", help="a .skrib to open (default: bundled StarForgers)")
parser.add_argument("--cycles", type=int, default=3, help="open/close cycles to walk")
parser.add_argument("--book", default="Starforgers",
                    help="the binder row to open (default: the bundled example's Book)")
parser.add_argument("--segment", default="Full Book",
                    help="the container segment whose stream to open")
parser.add_argument("--csv", help="where the timeline goes (default: a scratch file)")
parser.add_argument("--keep", action="store_true", help="leave the app running at the end")
parser.add_argument("--sites", action="store_true",
                    help="write large-allocation attribution reports at each step. Off by "
                         "default: symbolising a stack caches a whole-binary debug-info "
                         "index, so every reading after the first dump is tens of MB high "
                         "and the run can no longer be read as a total")
parser.add_argument("--skip-tab", action="store_true",
                    help="open and close the project only, never opening an editor tab, "
                         "isolates the project's own lifecycle from the editors'")
parser.add_argument("--quit-timeout", type=float, default=300.0,
                    help="seconds to wait for a clean quit (a dhat build needs minutes)")
parser.add_argument("--explore", action="store_true",
                    help="open the project, dump the accessibility tree with bounds, and stop")
args = parser.parse_args()

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()

source = args.project or fixture.repo_path("resources", "examples", "starforgers", "Starforgers.skrib")
if not os.path.exists(source):
    sys.exit(f"no such project: {source}")
project = fixture.working_copy(source, label="memprof")

csv_path = args.csv or os.path.join(fixture.SCRATCH, "skribisto-memprof.csv")
marker_path = csv_path + ".marker"

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
# A private config dir, so the recents list, the workspace layout and the window
# geometry of the operator's own install cannot change what this measures. Both
# launches below share it, which is also what lets the second one be handed to
# the first by the single-instance election.
env = fixture.isolated_config(locale="en-US", label="memprof", show_welcome=True)
env["SKRIBISTO_MEMPROF"] = csv_path

#: Every step's numbers, in order, for the table at the end.
steps = []


def die(msg, *procs):
    print("ERROR:", msg)
    print("--- app log tail ---")
    try:
        print("\n".join(open(log).read().splitlines()[-30:]))
    except OSError:
        pass
    for p in procs:
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


# ── launch the primary ────────────────────────────────────────────────────────
fixture.assert_no_running_instance(SKRIBISTO)
app = subprocess.Popen([SKRIBISTO], stdout=open(log, "w"), stderr=subprocess.STDOUT, env=env)

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
        # The exit code separates the two very different reasons an app vanishes
        # here: a crash (signal, or a non-zero status) and the single-instance
        # election handing this launch to an existing primary (status 0, and no
        # output at all, which is what makes it look like a crash).
        die(f"app exited early with status {app.returncode}", app)
    time.sleep(0.2)
if not sock:
    die("no bridge socket. Is this a debug build?", app)

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


#: How long a single bridge call may take. Generous, and deliberately so: under a
#: `dhat-heap` build every allocation walks the profiler's tables, and opening a
#: manuscript that takes 20 s in a plain debug build takes minutes. A short
#: timeout there reports "no MCP response", a symptom with no relation to the
#: cause.
CALL_TIMEOUT = float(os.environ.get("MEMPROF_CALL_TIMEOUT", "25"))


def recv(timeout=None, fatal=True):
    timeout = CALL_TIMEOUT if timeout is None else timeout
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


#: Which window every subsequent tool call addresses. Closing a project opens the
#: Launcher *beside* the project window rather than instead of it, so from cycle 2
#: on there are two windows and an unqualified snapshot answers with whichever the
#: host lists first, which is how a probe ends up reading the Launcher's tree and
#: reporting that the binder has no rows in it.
WINDOW = {"id": None}


def call(name, a=None):
    a = dict(a or {})
    if WINDOW["id"] is not None and "window_id" not in a:
        a["window_id"] = WINDOW["id"]
    send("tools/call", {"name": name, "arguments": a})
    res = recv().get("result", {})
    payload = res.get("structuredContent")
    if payload is None:
        txt = "".join(c.get("text", "") for c in res.get("content", []) if c.get("type") == "text")
        payload = json.loads(txt) if txt.strip().startswith("{") else {"_text": txt}
    return res, payload


deadline = time.time() + 30
init = None
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=subprocess.DEVNULL, text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "memprof", "version": "1"}})
    init = recv(timeout=5, fatal=False)
    if init is None:
        if mcp.poll() is None:
            mcp.terminate()
        time.sleep(0.3)
if init is None:
    die("could not connect MCP", app, mcp)
send("notifications/initialized", notif=True)
print(f"bridge up: {sock}")
print(f"timeline:  {csv_path}")
print(f"project:   {project}")


# ── driving ───────────────────────────────────────────────────────────────────
def settle():
    call("settle")
    time.sleep(0.35)


def nodes():
    _, p = call("snapshot_tree")
    return p.get("nodes", [])


def labels():
    return {n.get("label") for n in nodes() if n.get("label")}


def select_project_window():
    """Point every later call at the window that actually holds the binder.

    Asks each managed window for its own tree rather than trusting a title or an
    ordering: "the window with a Binder in it" is the definition that stays true
    when the Launcher is renamed, reordered, or opened second.
    """
    WINDOW["id"] = None
    _, payload = call("list_windows")
    # The bridge has answered with a bare list and with an object wrapping one,
    # depending on the tool; accept either rather than pinning one shape.
    raw = payload
    if isinstance(payload, dict):
        raw = payload.get("windows") or payload.get("_windows") or []
    ids = []
    for w in raw or []:
        wid = w.get("id") if isinstance(w, dict) else w
        if isinstance(wid, int):
            ids.append(wid)
    for wid in ids:
        _, p = call("snapshot_tree", {"window_id": wid})
        found = {n.get("label") for n in p.get("nodes", []) if n.get("label")}
        if "Binder" in found:
            WINDOW["id"] = wid
            return wid
    # One window, or a host that does not enumerate them: unqualified calls are
    # then already correct.
    return None


def find(label, timeout=8.0, minx=None, maxx=None, miny=None, role=None):
    e = time.time() + timeout
    while time.time() < e:
        for n in nodes():
            if n.get("label") != label:
                continue
            if role is not None and n.get("role") != role:
                continue
            b = n.get("bounds") or {}
            x, y = b.get("x", 0), b.get("y", 0)
            if minx is not None and x < minx:
                continue
            if maxx is not None and x >= maxx:
                continue
            if miny is not None and y < miny:
                continue
            return n
        settle()
    return None


def click(label, what="", minx=None, maxx=None, miny=None, role=None):
    n = find(label, minx=minx, maxx=maxx, miny=miny, role=role)
    if not n:
        print(f"  !! no node labelled {label!r} {what}")
        # A missing label is almost always a renamed fixture, so print what IS
        # there rather than making the next run a guess.
        seen = sorted({n.get("label") for n in nodes() if n.get("label")})
        print("     labels present:", ", ".join(repr(l) for l in seen[:60]))
        return False
    if "click" in (n.get("actions") or []):
        call("invoke_action", {"node": n["id"], "action": "click"})
    else:
        b = n.get("bounds") or {}
        if "x" not in b:
            return False
        call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                "y": b["y"] + b.get("height", 0) / 2, "action": "click"})
    settle()
    time.sleep(0.5)
    return True


def wait_for(pred, what, timeout=120.0):
    """Poll until `pred` accepts the current label set.

    Generous by default: `load_work` in a debug build takes the better part of
    half a minute on a real manuscript, and a timeout here reports a symptom
    ("no project window") a long way from any cause. On giving up it dumps the
    tree, because the useful question is always "then what IS on screen".
    """
    e = time.time() + timeout
    while time.time() < e:
        if pred(labels()):
            return True
        settle()
    print(f"--- tree at timeout ({what}) ---")
    dump_tree()
    die(f"timed out waiting for {what}", app, mcp)


# ── measurement ───────────────────────────────────────────────────────────────
def proc_rss(pid):
    """`(rss, anon)` in bytes, straight from the kernel.

    `rss` is the number `top` shows; `anon` is its heap-and-stacks half, read from
    `smaps_rollup` because the difference between the two is the difference
    between "the debug binary is large" and "the app is leaking".
    """
    rss = anon = 0
    try:
        with open(f"/proc/{pid}/statm") as f:
            rss = int(f.read().split()[1]) * os.sysconf("SC_PAGE_SIZE")
    except (OSError, IndexError, ValueError):
        pass
    # Two spellings, one per kernel lineage: `Anonymous` on 6.x, `Rss_Anon`
    # where the per-type breakdown is compiled in. Matching only one reports
    # zero, which reads as an empty heap rather than as a failed parse.
    try:
        with open(f"/proc/{pid}/smaps_rollup") as f:
            for line in f:
                if line.startswith(("Anonymous:", "Rss_Anon:")):
                    anon = int(line.split()[1]) * 1024
                    break
    except (OSError, IndexError, ValueError):
        pass
    return rss, anon


def last_row():
    """The newest sampler row, as a dict, or None before the first tick."""
    try:
        with open(csv_path) as f:
            lines = [l for l in f.read().splitlines() if l.strip()]
    except OSError:
        return None
    if len(lines) < 2:
        return None
    header = lines[0].split(",")
    # The label may itself contain no comma (the sampler never writes one), so a
    # plain split is exact.
    return dict(zip(header, lines[-1].split(",")))


def mark(label, trim=False, sites=False):
    """Stamp a step into the timeline, let it settle, and record the numbers.

    `sites` additionally asks the app to write its large-allocation attribution
    report for this step, which is how "50 MB is still held" becomes "50 MB is
    still held *by these call sites*".
    """
    if sites:
        with open(marker_path, "w") as f:
            f.write(f"sites:{label}\n")
        # Symbolising a few dozen stacks against a 400 MB debug binary is not
        # instant; give it several sampler ticks before moving on.
        time.sleep(SETTLE_S * 4)
    with open(marker_path, "w") as f:
        f.write("trim\n" if trim else label + "\n")
    time.sleep(SETTLE_S)
    if trim:
        # The trim row carries the sampler's own label; re-stamp the real one so
        # the rows that follow stay attributed to this step.
        with open(marker_path, "w") as f:
            f.write(label + "\n")
        time.sleep(SETTLE_S)
    row = last_row() or {}
    rss, anon = proc_rss(app.pid)
    rec = {
        "step": label,
        "rss": rss,
        "anon": anon,
        "live": int(row.get("live") or 0),
        "peak": int(row.get("peak") or 0),
        # Recomputed rather than read from the CSV: the app measures overhead
        # against its own reading of anonymous RSS, and this table must stay
        # right when the two disagree (an older binary, a different kernel).
        "overhead": max(0, anon - int(row.get("live") or 0)),
        "allocs": int(row.get("allocs") or 0),
        "frees": int(row.get("frees") or 0),
        "trimmed": trim,
    }
    steps.append(rec)
    mb = lambda v: v / (1 << 20)
    print(f"  [{label:28}] rss={mb(rec['rss']):7.1f}  anon={mb(rec['anon']):7.1f}  "
          f"live={mb(rec['live']):7.1f}  overhead={mb(rec['overhead']):7.1f}  MB"
          + ("   (after malloc_trim)" if trim else ""))
    return rec


def dump_tree():
    """Every node with its role and bounds, to read when a label moves."""
    for n in sorted(nodes(), key=lambda n: ((n.get("bounds") or {}).get("y", 0),
                                            (n.get("bounds") or {}).get("x", 0))):
        b = n.get("bounds") or {}
        print(f"  x={int(b.get('x', -1)):5} y={int(b.get('y', -1)):5} "
              f"w={int(b.get('width', 0)):5} {n.get('role', '?'):18} "
              f"{(n.get('label') or n.get('value') or '')[:70]!r}")


# ── the scenario ──────────────────────────────────────────────────────────────
print("\n=== baseline ===")
wait_for(lambda ls: "Welcome sections" in ls or "Works" in ls, "the Launcher window")
mark("launcher")
# The launcher's own settling jump (fonts, dictionaries, the recents scan) is
# real and takes a few seconds; sampling before it lands would attribute it to
# the first project open.
time.sleep(8)
mark("launcher-settled")
mark("launcher-settled", trim=True)

if args.explore:
    explore_env = dict(env)
    if "DHAT_OUT" in explore_env:
        explore_env["DHAT_OUT"] = explore_env["DHAT_OUT"] + ".handoff"
    explore_env["SKRIBISTO_MEMPROF"] = csv_path + ".handoff"
    subprocess.run([SKRIBISTO, project], env=explore_env, check=False,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=120)
    e = time.time() + 120
    while time.time() < e and select_project_window() is None:
        settle()
    print("\n=== tree: project open, no tab ===")
    dump_tree()
    click(args.book, "(binder row)", maxx=PANE_X, role="TreeItem")
    print(f"\n=== tree: after clicking {args.book!r} in the binder ===")
    dump_tree()
    for p in (mcp, app):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(0)

for cycle in range(1, args.cycles + 1):
    print(f"\n=== cycle {cycle} ===")

    # Opening: a second launch with the path on argv is handed to this primary by
    # the single-instance election, which is exactly the path a double-click takes.
    # A second launch with the path on argv is handed to this primary by the
    # single-instance election, the same path a double-click takes. The pause
    # before it lets the previous window's teardown finish: the open registry is
    # reaped by the window that held the claim, and a handoff that arrives first
    # is answered by a primary still holding the old lock.
    time.sleep(2.0)
    # The handoff process is a `dhat-heap` build too, and it would write its own
    # (empty, uninteresting) report over the primary's on the way out. Point it
    # somewhere else; the primary keeps the real one.
    handoff_env = dict(env)
    if "DHAT_OUT" in handoff_env:
        handoff_env["DHAT_OUT"] = handoff_env["DHAT_OUT"] + ".handoff"
    # Same trap on the memprof side: the handoff is a full app process, so its
    # own sampler would `File::create`, and so truncate, the primary's timeline
    # at the start of every cycle. The step numbers this probe prints are read
    # back immediately and were never wrong; the *file* silently lost every row
    # before the last handoff, which is exactly the kind of quiet loss that makes
    # a timeline untrustworthy after the fact.
    handoff_env["SKRIBISTO_MEMPROF"] = csv_path + ".handoff"
    handoff = subprocess.run([SKRIBISTO, project], env=handoff_env, check=False,
                             capture_output=True, text=True, timeout=120)
    if handoff.returncode != 0:
        print(f"  (handoff launch exited {handoff.returncode}: "
              f"{(handoff.stderr or '').strip()[:200]})")
    # Wait unqualified (the project window may not exist yet to be selected),
    # then pin every later call to it.
    e = time.time() + 120
    while time.time() < e and select_project_window() is None:
        settle()
    if WINDOW["id"] is None:
        print("--- windows at timeout ---")
        print(call("list_windows")[1])
        die("the project window never appeared", app, mcp)
    mark(f"c{cycle}-project-open")

    if not args.skip_tab:
        # The binder dock only. The project's own name also appears in the
        # window title and in the Launcher's recents, and clicking either of
        # those does something else entirely.
        if not click(args.book, "(binder row)", maxx=PANE_X, role="TreeItem"):
            die(f"no {args.book!r} row in the binder", app, mcp)
        mark(f"c{cycle}-book-tab-own-page")

        if not click(args.segment, "(segment)", minx=PANE_X):
            die(f"no {args.segment!r} segment on the container tab", app, mcp)
        mark(f"c{cycle}-full-book-stream", sites=args.sites)

        # Ctrl+F4 is `editor.tab.close`. Ctrl+W is `work.close` and resolves
        # globally before any tab sees it, which is why the tab command is not
        # the obvious chord.
        call("inject_key", {"key": "F4", "ctrl": True})
        settle()
        mark(f"c{cycle}-tab-closed")
        mark(f"c{cycle}-tab-closed", trim=True)

    if not args.skip_tab and args.sites:
        mark(f"c{cycle}-tab-closed", sites=True)

    call("inject_key", {"key": "w", "ctrl": True})
    e = time.time() + 120
    while time.time() < e:
        if select_project_window() is None:
            break
        settle()
    else:
        die("the project never closed", app, mcp)
    WINDOW["id"] = None
    mark(f"c{cycle}-work-closed")
    mark(f"c{cycle}-work-closed", trim=True)
    if args.sites:
        # Everything the writer opened has been closed, so whatever still holds
        # live bytes here is what one cycle leaves behind.
        mark(f"c{cycle}-work-closed", sites=True)


# ── the table ─────────────────────────────────────────────────────────────────
print("\n================ TIMELINE ================")
print(f"{'step':30} {'rss':>8} {'anon':>8} {'live':>8} {'overhead':>9} "
      f"{'Δanon':>8} {'Δlive':>8}   (MB)")
prev = None
for rec in steps:
    mb = lambda v: v / (1 << 20)
    danon = f"{mb(rec['anon'] - prev['anon']):+8.1f}" if prev else "       ."
    dlive = f"{mb(rec['live'] - prev['live']):+8.1f}" if prev else "       ."
    print(f"{rec['step']:30} {mb(rec['rss']):8.1f} {mb(rec['anon']):8.1f} "
          f"{mb(rec['live']):8.1f} {mb(rec['overhead']):9.1f} {danon} {dlive}")
    prev = rec

# The one number the whole exercise exists to produce: what a complete
# open/close cycle costs when everything it created has supposedly been closed.
closed = [r for r in steps if r["step"].endswith("work-closed") and r["trimmed"]]
if len(closed) >= 2:
    mb = lambda v: v / (1 << 20)
    print("\n---- cost of one complete open/close cycle, measured after malloc_trim ----")
    for a, b in zip(closed, closed[1:]):
        print(f"  {a['step']} -> {b['step']}: "
              f"anon {mb(b['anon'] - a['anon']):+.1f} MB, "
              f"live {mb(b['live'] - a['live']):+.1f} MB, "
              f"overhead {mb(b['overhead'] - a['overhead']):+.1f} MB")
    print("\n  live is what the program still holds; anon - live is the allocator's.")
    print("  A rising `live` is a leak. A rising `anon` with flat `live` is fragmentation.")

print(f"\nfull timeline: {csv_path}")

if not args.keep:
    # Ctrl+Q rather than SIGTERM: a signal skips every `Drop`, and a `dhat-heap`
    # build writes its report *from* a Drop at the end of `run`. Quitting for
    # real is also the only way to see what the process still held at the end.
    try:
        call("inject_key", {"key": "q", "ctrl": True})
    except SystemExit:
        raise
    except Exception:
        pass
    # Generous: tearing down a session under a heap profiler means every
    # deallocation walks the profiler's tables, and a `dhat-heap` build takes
    # minutes to unwind what a plain build drops in a second. Terminating early
    # loses the report entirely, since the report is written from a `Drop`.
    end = time.time() + args.quit_timeout
    while time.time() < end and app.poll() is None:
        time.sleep(1.0)
    if app.poll() is None:
        # Ctrl+Q may not have reached a window at all (the Launcher is the last
        # one standing and this probe stopped addressing it). Closing the final
        # window quits the process just as properly.
        print("  (Ctrl+Q did not quit; closing the last window)")
        try:
            n = find("Close", timeout=5)
            if n and "click" in (n.get("actions") or []):
                call("invoke_action", {"node": n["id"], "action": "click"})
        except SystemExit:
            raise
        except Exception:
            pass
        end = time.time() + args.quit_timeout
        while time.time() < end and app.poll() is None:
            time.sleep(1.0)
    if app.poll() is None:
        print("  (the app never quit; terminating. A dhat report will NOT have "
              "been written, it is produced by a Drop at the end of run())")
    for p in (mcp, app):
        if p and p.poll() is None:
            p.terminate()
