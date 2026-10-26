# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Run every `automation_*.py` probe, one at a time, and report each outcome.

    python3 scripts/run_automation.py --out <dir> [--only <substring> ...]
                                      [--exclude <substring> ...]
                                      [--timeout <seconds>] [--rerun-failures]

One at a time on purpose: two probes driving two app instances on one desktop
flake (focus, timing, the compositor), so a failure from a contended run says
nothing. Each probe runs under the interpreter running this script, from the
repository root, with its own scratch directory under `<dir>/scratch/` and its
whole output in `<dir>/<probe>.log`. Between two probes, any process still
running this checkout's debug build is stopped (`automation_fixture.stop_strays`),
so a probe that failed half-way cannot leave its window up for the next one.

An outcome is the probe's exit status: 0 PASS, 77 SKIP (the probe does not apply
here, and says why on its last `SKIP:` line), TIMEOUT when it ran past
`--timeout`, anything else FAIL. `--rerun-failures` runs each failure or timeout
once more on its own, and reports it FAIL only when the second run fails too
(FLAKY when it passes).

The summary goes to `<dir>/summary.md` (a table, then the last lines of every
failure's log) and `<dir>/summary.json`. Exit status 0 when nothing failed.
"""

import argparse
import json
import os
import platform
import signal
import subprocess
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

PASS, FAIL, SKIP, TIMEOUT, FLAKY = "PASS", "FAIL", "SKIP", "TIMEOUT", "FLAKY"


def probes(only, exclude):
    here = os.path.dirname(os.path.abspath(__file__))
    names = sorted(
        n for n in os.listdir(here)
        if n.startswith("automation_") and n.endswith(".py") and n != "automation_fixture.py"
    )
    if only:
        names = [n for n in names if any(o in n for o in only)]
    if exclude:
        names = [n for n in names if not any(x in n for x in exclude)]
    return [os.path.join(here, n) for n in names]


def kill_tree(proc):
    """Stop a probe that ran out of time, with everything it started."""
    if fixture.IS_WINDOWS:
        subprocess.run(["taskkill", "/T", "/F", "/PID", str(proc.pid)],
                       capture_output=True)
    else:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except OSError:
            pass
    try:
        proc.wait(timeout=30)
    except subprocess.TimeoutExpired:
        pass


def last_lines(path, count):
    try:
        with open(path, encoding="utf-8", errors="replace") as fh:
            return fh.read().splitlines()[-count:]
    except OSError:
        return []


def run_one(path, out_dir, timeout, attempt):
    name = os.path.splitext(os.path.basename(path))[0]
    suffix = "" if attempt == 1 else f".run{attempt}"
    log = os.path.join(out_dir, f"{name}{suffix}.log")
    scratch = os.path.join(out_dir, "scratch", f"{name}{suffix}")
    os.makedirs(scratch, exist_ok=True)
    env = dict(os.environ)
    env["SKRIBISTO_AUTOMATION_SCRATCH"] = scratch
    # The probes that leave their sandbox to the caller (timeline, versions) get
    # one here, so a run never touches the machine's real settings.
    env["PROBE_XDG"] = os.path.join(scratch, "xdg")
    env["PYTHONUNBUFFERED"] = "1"
    # Text pipes and files opened without an encoding decode with the ANSI code
    # page on Windows, which garbles every French or Arabic label a probe reads.
    env["PYTHONUTF8"] = "1"
    # On macOS a window's menu lives in the system menu bar, which the bridge
    # cannot open: a debug build keeps the in-window copy too when asked, so the
    # probes that choose a command from a menu run there unchanged. Inert
    # elsewhere. See `project_menus::native_menu_mode`.
    env["SKRIBISTO_IN_WINDOW_MENU"] = "1"
    # A probe that asks before installing the MCP client would wait for ever:
    # there is nobody at a terminal here.
    env.setdefault(fixture.MCP_AUTOINSTALL_ENV, "0")
    kwargs = {}
    if fixture.IS_WINDOWS:
        kwargs["creationflags"] = subprocess.CREATE_NEW_PROCESS_GROUP
    else:
        kwargs["start_new_session"] = True
    started = time.monotonic()
    with open(log, "w", encoding="utf-8") as fh:
        proc = subprocess.Popen([sys.executable, path], cwd=fixture.repo_root(), env=env,
                                stdout=fh, stderr=subprocess.STDOUT,
                                stdin=subprocess.DEVNULL, **kwargs)
        try:
            code = proc.wait(timeout=timeout)
            status = PASS if code == 0 else SKIP if code == fixture.SKIP_EXIT else FAIL
        except subprocess.TimeoutExpired:
            kill_tree(proc)
            code, status = None, TIMEOUT
    seconds = round(time.monotonic() - started, 1)
    strays = fixture.stop_strays()
    reason = ""
    if status == SKIP:
        reason = next((l[len("SKIP:"):].strip() for l in reversed(last_lines(log, 40))
                       if l.startswith("SKIP:")), "")
    return {"probe": name, "status": status, "exit": code, "seconds": seconds,
            "log": os.path.relpath(log, out_dir), "strays_stopped": strays,
            "skip_reason": reason}


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", required=True, help="directory for logs and the summary")
    parser.add_argument("--only", action="append", default=[], metavar="SUBSTRING")
    parser.add_argument("--exclude", action="append", default=[], metavar="SUBSTRING")
    parser.add_argument("--timeout", type=float, default=900.0)
    parser.add_argument("--rerun-failures", action="store_true")
    args = parser.parse_args()

    out_dir = os.path.abspath(args.out)
    os.makedirs(out_dir, exist_ok=True)
    binary = fixture.skribisto_binary()
    stopped = fixture.stop_strays(binary)
    if stopped:
        print(f"stopped {stopped} stray instance(s) of {binary} before starting", flush=True)

    results = []
    todo = probes(args.only, args.exclude)
    for index, path in enumerate(todo, 1):
        result = run_one(path, out_dir, args.timeout, 1)
        if args.rerun_failures and result["status"] in (FAIL, TIMEOUT):
            again = run_one(path, out_dir, args.timeout, 2)
            result["rerun"] = again
            if again["status"] == PASS:
                result["status"] = FLAKY
        results.append(result)
        print(f"[{index}/{len(todo)}] {result['status']:<7} {result['seconds']:>7}s "
              f"{result['probe']}", flush=True)

    counts = {s: sum(r["status"] == s for r in results)
              for s in (PASS, FAIL, TIMEOUT, FLAKY, SKIP)}
    meta = {
        "platform": platform.platform(),
        "python": sys.version.split()[0],
        "binary": binary,
        "mcp": os.environ.get(fixture.MCP_BIN_ENV) or "PATH",
        "counts": counts,
    }
    with open(os.path.join(out_dir, "summary.json"), "w", encoding="utf-8") as fh:
        json.dump({"meta": meta, "results": results}, fh, indent=2)

    lines = [f"# Automation run on {meta['platform']}", "",
             f"Python {meta['python']}, binary `{binary}`.", "",
             " · ".join(f"{s} {n}" for s, n in counts.items()), "",
             "| Probe | Outcome | Seconds | Log | Note |", "|---|---|---|---|---|"]
    for r in results:
        note = r["skip_reason"]
        if r.get("rerun"):
            note = f"rerun alone: {r['rerun']['status']}"
        lines.append(f"| {r['probe']} | {r['status']} | {r['seconds']} | {r['log']} | {note} |")
    for r in results:
        if r["status"] in (FAIL, TIMEOUT, FLAKY):
            lines += ["", f"## {r['probe']} ({r['status']})", "", "```"]
            lines += last_lines(os.path.join(out_dir, r["log"]), 30)
            lines += ["```"]
    with open(os.path.join(out_dir, "summary.md"), "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")

    print(" · ".join(f"{s} {n}" for s, n in counts.items()))
    print(f"summary: {os.path.join(out_dir, 'summary.md')}")
    return 1 if counts[FAIL] or counts[TIMEOUT] else 0


if __name__ == "__main__":
    sys.exit(main())
