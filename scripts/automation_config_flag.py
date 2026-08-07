#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Prove the settings-schema command line does what it claims, end to end.

Three phases, cheapest first — the first two never build a window:

  A. `--dump-config` prints a file that parses as TOML, lists every registered
     key exactly once, and marks which values came from disk.
  B. `--config` with a broken pins file exits non-zero and names every problem,
     with a suggestion for a near-miss key. This is the phase that matters most:
     the whole point of the schema is that a bad pin is a refusal to launch
     rather than a silent no-op, and "silently ignored" is exactly the failure a
     probe cannot see in its own results.
  C. `--config` against a live app. The sandbox's `general.toml` is written in
     **English** and the pins file asks for **French**, so a French Launcher
     proves two things at once: the pins reached the running app, and they beat
     what was already on disk.

Run it after touching `settings_keys.rs`, `parse_args`, or the startup ordering
in `main` — the unit tests cover the parsing and the merge, but only a launch
covers "and then the app actually reads it".
"""
import json, os, re, select, subprocess, sys, tempfile, time, tomllib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from automation_fixture import (  # noqa: E402
    assert_no_running_instance,
    config_pins_file,
    isolated_config,
)

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SKRIBISTO = os.path.join(REPO, "target", "debug", "skribisto")
MCP = "/home/cyril/Devel/teksilo/target/debug/teksilo-automation-mcp"

failures = []


def check(ok, what, detail=""):
    print(f"{'ok  ' if ok else 'FAIL'}  {what}")
    if not ok:
        if detail:
            print(f"        {detail}")
        failures.append(what)
    return ok


def report():
    """Defined up here, not at the end: `bail` calls it from the middle of phase
    C, and a definition below that point would only exist once the phase it is
    meant to abort had already finished."""
    print()
    if failures:
        print(f"FAIL: {len(failures)} check(s) failed:")
        for f in failures:
            print(f"  - {f}")
        sys.exit(1)
    print("PASS: --dump-config, --config validation and live pinning all behave")
    sys.exit(0)


# ─────────────────────────────────────────────────────────────────────────────
# Phase A — --dump-config
# ─────────────────────────────────────────────────────────────────────────────
print("\n== A. --dump-config ==")

env_a = isolated_config(locale="en-US", label="dump")
run = subprocess.run([SKRIBISTO, "--dump-config"], env=env_a,
                     capture_output=True, text=True, timeout=60)
check(run.returncode == 0, "--dump-config exits 0", run.stderr[-400:])

try:
    dumped = tomllib.loads(run.stdout)
except Exception as e:
    dumped = {}
    check(False, "--dump-config output parses as TOML", str(e))
else:
    check(True, "--dump-config output parses as TOML")


def flatten(tree, prefix=""):
    out = {}
    for key, value in tree.items():
        path = f"{prefix}.{key}" if prefix else key
        if isinstance(value, dict):
            out.update(flatten(value, path))
        else:
            out[path] = value
    return out


keys = flatten(dumped)
check(len(keys) > 50, f"the dump lists a full schema ({len(keys)} keys)")
# `isolated_config` wrote ui.locale, so the dump must report it as set, not default.
check(keys.get("ui.locale") == "en-US",
      "a value on disk is reported, not the default",
      f"got {keys.get('ui.locale')!r}")
check("— set" in run.stdout and "— default" in run.stdout,
      "the dump marks which values came from disk")
# The keys the app declares but nothing has ever written must still be listed.
check("editor.typewriter_anchor" in keys,
      "an untouched key is still listed as settable")


# ─────────────────────────────────────────────────────────────────────────────
# Phase B — --config rejects a broken pins file
# ─────────────────────────────────────────────────────────────────────────────
print("\n== B. --config validation ==")

env_b = isolated_config(locale="en-US", label="bad")
bad = os.path.join(tempfile.gettempdir(), f"skribisto-bad-pins-{os.getpid()}.toml")
with open(bad, "w", encoding="utf-8") as fh:
    fh.write('ui.local = "fr-FR"\n'            # near-miss: ui.locale
             'editor.autosave = "yes"\n'       # wrong type
             'editor.highlight_scope = "Line"\n'  # unknown enum variant
             'backup.retention.daily = 7\n')   # a real setting, but not in this store

run = subprocess.run([SKRIBISTO, "--config", bad], env=env_b,
                     capture_output=True, text=True, timeout=60)
err = run.stderr
check(run.returncode == 2, f"a broken pins file exits 2 (got {run.returncode})", err[-400:])
check("did you mean `ui.locale`" in err, "a near-miss key gets a suggestion", err[-400:])
check("expects bool" in err, "a wrong type names the expected type", err[-400:])
check("None | Sentence | Paragraph" in err,
      "a bad enum value lists the legal variants", err[-400:])
check("backup.retention.daily" in err,
      "a key outside this store is rejected too", err[-400:])
check(err.count("unknown setting") + err.count("expects") == 4,
      "every problem is reported in one pass, not just the first", err[-400:])

# A refused launch must not leave the pins behind: the run exited before applying
# anything, so the sandbox's settings must be exactly as isolated_config left them.
general_b = os.path.join(env_b["XDG_CONFIG_HOME"], "skribisto", "general.toml")
with open(general_b, "rb") as fh:
    after = flatten(tomllib.load(fh))
check(after.get("ui.locale") == "en-US",
      "a rejected pins file changes nothing on disk",
      f"got {after.get('ui.locale')!r}")

# The missing-value form must not swallow the project path.
run = subprocess.run([SKRIBISTO, "--config"], env=env_b,
                     capture_output=True, text=True, timeout=60)
check(run.returncode == 2 and "needs a file path" in run.stderr,
      "--config without a value is refused", run.stderr[-200:])


# ─────────────────────────────────────────────────────────────────────────────
# Phase C — --config against a live app
# ─────────────────────────────────────────────────────────────────────────────
print("\n== C. --config on a live app ==")

assert_no_running_instance(SKRIBISTO)

# On disk: English. In the pins: French. Only the pins winning produces a French
# Launcher, so this distinguishes "the flag worked" from "the sandbox worked".
env_c = isolated_config(locale="en-US", label="live")
pins = config_pins_file({"ui.locale": "fr-FR", "ui.show_welcome": True}, label="live")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, "--config", pins], env=env_c,
                       stdout=open(log, "w"), stderr=subprocess.STDOUT)
mcp = None


def cleanup():
    for proc in (mcp, app):
        if proc and proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=3)
            except Exception:
                proc.kill()


def bail(msg):
    check(False, msg, "\n".join(open(log).read().splitlines()[-20:]))
    cleanup()
    report()


sock = tok = None
deadline = time.time() + 60
while time.time() < deadline:
    text = open(log).read()
    s = re.search(r"bridge socket = (\S+)", text)
    t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", text)
    if s and t:
        sock, tok = s.group(1), t.group(1)
        break
    if app.poll() is not None:
        bail("the app exited before opening its automation bridge")
    time.sleep(0.2)
if not sock:
    bail("no automation bridge within 60s")

check(True, "the app launched with --config and opened its bridge")

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


def recv(timeout=20):
    end = time.time() + timeout
    while time.time() < end:
        if mcp.poll() is not None:
            return None
        r, _, _ = select.select([mcp.stdout], [], [], max(0.0, end - time.time()))
        if not r:
            return None
        line = mcp.stdout.readline()
        if not line:
            return None
        if line.strip():
            return json.loads(line)
    return None


def call(name, args=None):
    send("tools/call", {"name": name, "arguments": args or {}})
    result = (recv() or {}).get("result", {})
    payload = result.get("structuredContent")
    if payload is None:
        text = "".join(c.get("text", "") for c in result.get("content", [])
                       if c.get("type") == "text")
        payload = json.loads(text) if text.strip().startswith("{") else {}
    return payload


# The bridge announces its socket before binding it, so connect-and-initialize is
# retried rather than raced (same dance as automation_test.py).
mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name
init = None
deadline = time.time() + 30
while time.time() < deadline and init is None:
    while not os.path.exists(sock) and time.time() < deadline:
        time.sleep(0.05)
    mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                           stderr=open(mcp_err, "w"), text=True, bufsize=1)
    send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                        "clientInfo": {"name": "skribisto-config-flag", "version": "1"}})
    init = recv(timeout=4)
    if init is None and mcp.poll() is None:
        mcp.terminate()
        time.sleep(0.3)
if init is None:
    bail("could not reach the automation bridge")
send("notifications/initialized", notif=True)

FRENCH = "bienvenue dans skribisto"
ENGLISH = "welcome to skribisto"
joined = ""
found = False
end = time.time() + 30
while time.time() < end:
    nodes = call("snapshot_tree").get("nodes", [])
    joined = " | ".join(
        ((n.get("value") or "") + " " + (n.get("label") or "")) for n in nodes
    ).lower()
    if FRENCH in joined or ENGLISH in joined:
        found = True
        break
    time.sleep(0.5)

check(found, "the Launcher rendered", joined[:400])
check(FRENCH in joined,
      "the pinned locale reached the live app (French Launcher)",
      joined[:400])
check(ENGLISH not in joined,
      "the pins beat the general.toml already on disk (no English)",
      joined[:400])

# And the pins are on disk in the sandbox afterwards, where a follow-up run sees them.
general_c = os.path.join(env_c["XDG_CONFIG_HOME"], "skribisto", "general.toml")
with open(general_c, "rb") as fh:
    live = flatten(tomllib.load(fh))
check(live.get("ui.locale") == "fr-FR",
      "the pins were merged into the sandbox's settings file",
      f"got {live.get('ui.locale')!r}")

cleanup()
report()
