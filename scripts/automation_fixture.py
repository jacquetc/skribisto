# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""One rule, in one place: a live-app probe never opens a checked-in fixture.

Driving the real app means Ctrl+S, autosave, and format migration all happen for
real, and any of them rewrites whatever file was passed on the command line —
silently corrupting the repo's fixture for the next run. A probe that mutates
its own fixture still passes, so nothing catches it.

`working_copy()` hands back a throwaway in the scratchpad, and probes open
*that*. Cheap enough (a few MB) that there is no reason to skip it.
"""

import collections
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib

#: Session scratchpad when one is set, else the system temp dir. Never inside the repo.
#: Set `SKRIBISTO_AUTOMATION_SCRATCH` to steer it.
SCRATCH = os.environ.get("SKRIBISTO_AUTOMATION_SCRATCH", tempfile.gettempdir())


def wait_for_load(nodes_fn, markers, timeout=30.0, interval=0.5):
    """Poll `nodes_fn()` until any of `markers` appears in a node's label or value.

    Returns True on success, False on timeout.

    A single snapshot taken straight after the bridge connects is not enough: the
    legacy SQLite fixture is migrated on open, and the binder is populated some
    way after the window first answers. Checking once reports "the fixture did
    not load" for a project that loads perfectly a second later — a false failure
    that costs a 40-second launch to diagnose. Every probe should wait rather
    than sleep-and-hope.
    """
    import time as _t
    if isinstance(markers, str):
        markers = (markers,)
    deadline = _t.time() + timeout
    while _t.time() < deadline:
        for n in nodes_fn():
            text = ((n.get("value") or "") + " " + (n.get("label") or "")).lower()
            if any(m.lower() in text for m in markers):
                return True
        _t.sleep(interval)
    return False


def assert_no_running_instance(binary=None, strict=False):
    """Report another live instance of the same build. Only *fails* when `strict`.

    This used to refuse outright, and that was right when a probe launched the
    app bare: the single-instance election hands a second launch off to the
    existing process and exits, so the probe scraped the announce of a process
    on its way out and every call then timed out — reporting a symptom (e.g.
    "the modal did not open") several layers from the real cause.

    [`launch_argv`] passes `--new-instance`, which makes that handoff impossible,
    and `isolated_config` puts the run in its own configuration directory — which
    the open registry is namespaced by, so the two instances do not even see each
    other's open projects. Neither reason to refuse survives, and refusing costs
    something real: a probe could not run while the operator had the app open,
    which is most of the time.

    So it now prints a note and continues. Pass `strict=True` where a probe
    genuinely must be alone — one asserting on the election itself, say.

    Deliberately does NOT kill anything. The match is on this worktree's debug
    binary, which is also what `run-app` launches, so a stray instance may well
    be the operator's own session with unsaved work in it.
    """
    binary = binary or os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        "target", "debug", "skribisto" + EXE,
    )
    try:
        out = subprocess.run(["pgrep", "-af", binary], capture_output=True, text=True).stdout
    except (OSError, subprocess.SubprocessError):
        return  # pgrep unavailable: not worth failing the probe over
    # ⚠ `pgrep -af <pattern>` matches on the whole command line, and this
    # process's own shell command line contains the pattern. Drop our own
    # ancestry, or a probe reports itself as the instance in the way.
    mine = {str(os.getpid()), str(os.getppid())}
    live = [l for l in out.splitlines()
            if l.strip() and l.split(maxsplit=1)[0] not in mine]
    if not live:
        return
    listing = "\n  ".join(live)
    if strict:
        raise RuntimeError(
            f"another instance of {binary} is already running:\n  {listing}\n"
            "This probe asked to be alone (strict=True). Close it (or "
            "`pkill -f target/debug/skribisto` if it is a probe leftover and you "
            "have nothing unsaved) and re-run."
        )
    print(f"note      : {len(live)} other instance(s) of {binary} are running; "
          "launching with --new-instance in a private config, so they are ignored")


# ---------------------------------------------------------------------------
# Platforms
# ---------------------------------------------------------------------------
#
# The probes were written on Linux and sandbox the app with `XDG_CONFIG_HOME`.
# teksilo's `AppPaths` puts the app's directories in the XDG layout on Linux and
# under `%APPDATA%` on Windows. On macOS teksilo 0.21.0 used the XDG layout too
# (`~/.config`, `~/.local/share`); later versions use Apple's,
# `~/Library/Application Support/<bundle id>/{config,data}`, and on first start
# move the XDG folders there. A probe that set only `XDG_CONFIG_HOME` ran against
# the machine's real settings on Windows, and on such a Mac it would create the
# real Application Support folder, after which the user's own settings are never
# moved. So every sandbox is built by [`sandbox_env`] (which sandboxes `HOME` too
# on macOS), and every path into one comes from [`app_config_dir`] or
# [`app_data_dir`]. The same probe then isolates itself on all three platforms,
# and reads back the files the app really wrote.

#: `identity::AppIdentity::community()`'s triple, from which `AppPaths::new`
#: derives the app's directory names on every platform.
APP_QUALIFIER, APP_ORGANIZATION, APP_NAME = "eu", "skribisto", "Skribisto"

IS_WINDOWS = sys.platform == "win32"
IS_MACOS = sys.platform == "darwin"
IS_LINUX = sys.platform.startswith("linux")

#: The suffix cargo gives an executable on this platform.
EXE = ".exe" if IS_WINDOWS else ""

#: The exit status of a probe that does not apply on this machine (the automake
#: convention). `run_automation.py` reports it as SKIP, never as a pass.
SKIP_EXIT = 77


if IS_WINDOWS:
    # Every probe waits for the MCP client's next line with
    # `select.select([mcp.stdout], [], [], timeout)`. Windows' `select` takes
    # sockets only and raises on a pipe, so on Windows that call is answered here
    # instead: a pipe is ready when `PeekNamedPipe` reports bytes waiting in it
    # (or the pipe has closed, so that `readline` returns ""), which is what
    # `select` reports for a pipe on Linux. Installed on the module, not handed
    # to the probes, because they call `select.select` through the module at
    # call time; anything that is not a pipe goes to the real `select`.
    import ctypes
    import msvcrt
    import select as _select
    from ctypes import wintypes

    _peek_named_pipe = ctypes.WinDLL("kernel32", use_last_error=True).PeekNamedPipe
    _peek_named_pipe.argtypes = [wintypes.HANDLE, ctypes.c_void_p, wintypes.DWORD,
                                 ctypes.c_void_p, ctypes.POINTER(wintypes.DWORD),
                                 ctypes.c_void_p]
    _peek_named_pipe.restype = wintypes.BOOL
    _socket_select = _select.select

    def _pipe_handle(stream):
        try:
            return msvcrt.get_osfhandle(stream.fileno())
        except (AttributeError, OSError, ValueError):
            return None

    def _pipe_ready(handle):
        waiting = wintypes.DWORD(0)
        if not _peek_named_pipe(handle, None, 0, None, ctypes.byref(waiting), None):
            return True  # closed: readable, and the read reports the end
        return waiting.value > 0

    def _select_with_pipes(rlist, wlist, xlist, timeout=None):
        handles = [(stream, _pipe_handle(stream)) for stream in rlist]
        if wlist or xlist or any(handle is None for _, handle in handles):
            return _socket_select(rlist, wlist, xlist, timeout)
        deadline = None if timeout is None else time.monotonic() + timeout
        while True:
            ready = [stream for stream, handle in handles if _pipe_ready(handle)]
            if ready or (deadline is not None and time.monotonic() >= deadline):
                return ready, [], []
            time.sleep(0.01)

    _select.select = _select_with_pipes


def skip(reason):
    """End the probe as skipped, saying why. Never use it for a failure."""
    print(f"SKIP: {reason}", flush=True)
    sys.exit(SKIP_EXIT)


def require_linux(reason):
    """Skip unless on Linux, for a probe that drives something only Linux has
    (KWin, a `kdeglobals`, `XDG_RUNTIME_DIR`, `/proc`). `reason` names it."""
    if not IS_LINUX:
        skip(f"Linux only: {reason}")


def sandbox_env(config_home, extra=None, base=None):
    """The environment of a run whose settings all live under `config_home`.

    `config_home` is the `XDG_CONFIG_HOME` on Linux and macOS, and on Windows
    it holds `%APPDATA%` and `%LOCALAPPDATA%`. `extra` is merged in first, so a
    probe's own variables (`XDG_DATA_HOME`, a `HOME`) survive.

    macOS adds two things. The app's sockets live in its data directory, and
    Darwin's `sun_path` holds 104 bytes, which a data directory deep in a
    scratch folder overflows (the bind then fails without a word). So there the
    data directory is reached through a short link in `/tmp` (see
    [`_short_link`]): the probe's own `XDG_DATA_HOME` when it set one, else a
    `data` folder beside `config_home`. Never the machine's real
    `~/.local/share`, which a probe that set no data directory used to share.
    And `HOME` is a sandbox too, also through a short link (the probe's own
    `HOME` when it set one, else a `home` folder beside `config_home`): a
    teksilo that puts a Mac application's files in `~/Library/Application
    Support` reads `HOME` and nothing else.

    `base` defaults to this process's environment.
    """
    env = dict(os.environ if base is None else base)
    env.update(extra or {})
    env["XDG_CONFIG_HOME"] = config_home
    if IS_MACOS:
        data_home = (extra or {}).get("XDG_DATA_HOME") or os.path.join(config_home, "data")
        env["XDG_DATA_HOME"] = _short_link(data_home)
        home = (extra or {}).get("HOME") or os.path.join(config_home, "home")
        env["HOME"] = _short_link(home)
    elif IS_WINDOWS:
        env["APPDATA"] = os.path.join(config_home, "AppData", "Roaming")
        env["LOCALAPPDATA"] = os.path.join(config_home, "AppData", "Local")
    return env


def _short_link(path):
    """A short path in `/tmp` to the directory `path` (macOS, for sockets).

    Built against teksilo 0.21.0, the app binds its sockets in
    `<data dir>/skribisto/run/`, and Darwin's `sun_path` holds 104 bytes with
    the NUL. A symlink named after a hash of `path` keeps that well inside the
    limit: the same directory always gets the same link, and every file still
    lands in `path`. (A later teksilo gives the app a runtime directory under
    `/var/folders/` for its sockets, short whatever `HOME` is.)
    """
    import hashlib

    path = os.path.abspath(path)
    os.makedirs(path, exist_ok=True)
    link = "/tmp/sk-" + hashlib.sha1(path.encode("utf-8")).hexdigest()[:10]
    if os.path.islink(link) and os.readlink(link) != path:
        os.remove(link)
    if not os.path.lexists(link):
        os.symlink(path, link)
    return link


def looks_like_path(text):
    """Does a field's text read as a file system path, on this platform too?
    (`C:\\Users\\...` has no `/` in it.)"""
    text = str(text or "")
    return os.path.isabs(text) or "/" in text or "\\" in text



def app_config_dir(env):
    """Where the app launched with `env` keeps its settings (`general.toml`,
    `workspace.toml`, ...), as `AppPaths::config_dir` resolves it."""
    if IS_WINDOWS:
        return os.path.join(env["APPDATA"], APP_ORGANIZATION, APP_NAME, "config")
    if IS_MACOS and _apple_layout(env):
        return os.path.join(_bundle_dir(env), "config")
    root = env.get("XDG_CONFIG_HOME") or os.path.join(env["HOME"], ".config")
    return os.path.join(root, APP_NAME.lower())


def app_data_dir(env):
    """Where the app launched with `env` keeps its data, as
    `AppPaths::data_dir` resolves it."""
    if IS_WINDOWS:
        return os.path.join(env["APPDATA"], APP_ORGANIZATION, APP_NAME, "data")
    if IS_MACOS and _apple_layout(env):
        return os.path.join(_bundle_dir(env), "data")
    root = env.get("XDG_DATA_HOME") or os.path.join(env["HOME"], ".local", "share")
    return os.path.join(root, APP_NAME.lower())


def _bundle_dir(env):
    """`~/Library/Application Support/<bundle id>` under `env`'s `HOME`."""
    bundle = f"{APP_QUALIFIER}.{APP_ORGANIZATION.lower()}.{APP_NAME}"
    return os.path.join(env["HOME"], "Library", "Application Support", bundle)


_LAYOUTS = {}


def _apple_layout(env):
    """Whether the binary under test keeps a Mac's files in Application Support.

    Asked of the binary itself, since that depends on the teksilo it was built
    against and no version number says which: `--config` with nothing in it
    reports the settings file it would write, and `--dump-config` stops before
    any window. Only ever under a sandboxed `HOME` ([`sandbox_env`]): run with the
    real one, it would create the real Application Support folder.
    """
    home = env.get("HOME") or ""
    if not home.startswith("/tmp/sk-"):
        raise RuntimeError(f"refusing to ask the app where it keeps its files under "
                           f"HOME={home!r}: build the environment with sandbox_env")
    binary = skribisto_binary()
    key = (binary, home, env.get("XDG_CONFIG_HOME"))
    if key not in _LAYOUTS:
        empty = os.path.join(home, "empty-pins.toml")
        with open(empty, "w", encoding="utf-8"):
            pass
        done = subprocess.run([binary, "--config", empty, "--dump-config"], env=env,
                              capture_output=True, text=True, timeout=60)
        said = re.search(r"pinned \d+ settings? from .* into (.*)$", done.stderr, re.M)
        if not said:
            raise RuntimeError(f"the app did not say where it keeps its settings:\n"
                               f"{done.stderr[-2000:]}")
        _LAYOUTS[key] = "/Library/Application Support/" in said.group(1)
    return _LAYOUTS[key]


def stop_strays(binary=None):
    """Kill every process running `binary` (this checkout's debug build by
    default), and only those: an installed Skribisto, or one started from
    another checkout, is left alone. Returns how many were asked to stop.

    For a runner between two probes, and for the probes that start by clearing
    the way (they used `pkill -x skribisto`, which also ended an installed
    Skribisto and whatever was open in it).
    """
    binary = os.path.abspath(binary or skribisto_binary())
    if IS_WINDOWS:
        script = (
            "$n = 0; Get-Process | Where-Object { $_.Path -eq $args[0] } | "
            "ForEach-Object { Stop-Process -Id $_.Id -Force; $n++ }; $n"
        )
        try:
            out = subprocess.run(
                ["powershell", "-NoProfile", "-Command", script, binary],
                capture_output=True, text=True, timeout=60).stdout
            return int(out.strip() or 0)
        except (OSError, subprocess.SubprocessError, ValueError):
            return 0
    try:
        # POSIX ERE, which pgrep reads: escape its specials only (Python's
        # `re.escape` also escapes `-`, which ERE leaves undefined).
        pattern = "^" + re.sub(r"([.^$*+?()\[\]{}|\\])", r"\\\1", binary) + "( |$)"
        out = subprocess.run(["pgrep", "-f", pattern],
                             capture_output=True, text=True).stdout
    except (OSError, subprocess.SubprocessError):
        return 0
    pids = [int(p) for p in out.split() if p.isdigit() and int(p) != os.getpid()]
    for pid in pids:
        try:
            os.kill(pid, 9)
        except OSError:
            pass
    return len(pids)


def toml_scalar(value):
    """Render a Python scalar as a TOML value.

    Bool before int on purpose: `bool` is a subclass of `int` in Python, so the
    obvious ordering writes `dark = 1`, which the settings store then rejects as
    the wrong type for a boolean key.
    """
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return repr(value)
    if isinstance(value, (list, tuple)):
        return "[" + ", ".join(toml_scalar(v) for v in value) + "]"
    escaped = str(value).replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def toml_pins(pins):
    """Render a dict of dotted keys → values as flat dotted TOML.

    Flat rather than `[sections]`: dotted keys are valid TOML at the top level,
    several of them may share a prefix, and the result is order-independent — so a
    caller can hand over any mix of keys without first grouping them by table. It
    is also exactly the shape `skribisto --dump-config` emits, so a dump can be
    trimmed and handed straight back.
    """
    return "".join(f"{key} = {toml_scalar(value)}\n" for key, value in sorted(pins.items()))


def write_settings(config_home, locale="en-US", dark=False, show_welcome=True, pins=None,
                   pace_summary=False):
    """Write `general.toml` into a sandbox **you** already built. Returns its path.

    The other half of [`isolated_config`], for the probes that roll their own
    sandbox because they need more than `XDG_CONFIG_HOME` — an `XDG_DATA_HOME`
    and a `HOME` of their own, usually. Those hand-built sandboxes used to write
    no settings at all, which is not the neutral choice it looks like: with
    `startup.rs`'s `auto_detect_os_locale` on, an unset `ui.locale` is not
    "English", it is *the operator's OS language*. Every probe matching an
    English label passed on an English desktop and failed on a French one, with
    a symptom that named the widget rather than the language ("the 'Work' menu
    did not open" — it had opened, as `Œuvre`).

    So the language is written on every call here for the same reason
    `isolated_config` writes it: a probe that reads labels must **set** the
    language, never inherit it. `en-US` by default because that is what the
    label literals in these scripts are written in; pass `locale=` to assert on
    a translation deliberately.

    `config_home` is the directory the probe hands [`sandbox_env`] (on Linux,
    the one it puts in `XDG_CONFIG_HOME`); the file goes where the app launched
    with that sandbox reads it, [`app_config_dir`], on every platform. Nothing
    validates the keys — see `isolated_config`'s note on pairing this with
    `config_pins_file` where that matters.

    `pace_summary` is the "Where the book stands" card a project with a pace plan
    shows as it opens (`pace.summary_on_open`), off unless a probe asks for it.
    It is an in-window modal that takes the focus, and since teksilo 0.21 the
    page's shortcuts do not reach past one and the click that dismisses it does
    not reach the page either: a probe that did not dismiss it first lost its
    first click and its first chord, and failed on something else entirely.
    """
    settings = {
        "ui.dark": bool(dark),
        "ui.locale": locale,
        "ui.show_welcome": bool(show_welcome),
        "pace.summary_on_open": bool(pace_summary),
    }
    settings.update(pins or {})
    cfg = app_config_dir(sandbox_env(config_home, base={}))
    os.makedirs(cfg, exist_ok=True)
    path = os.path.join(cfg, "general.toml")
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(toml_pins(settings))
    return path


def isolated_config(locale="fr-FR", label="cfg", dark=False, show_welcome=True, pins=None,
                    pace_summary=False):
    """A private settings sandbox with the app's settings pinned. Returns an env dict.

    The sandbox is built by [`sandbox_env`]: a private `XDG_CONFIG_HOME` on
    Linux, `$HOME` on macOS, `%APPDATA%` on Windows.

    A probe that asserts on translated text must SET the language, never inherit
    it, and this writes `ui.locale` on every call for exactly that reason. Two
    ways inheriting goes wrong: a probe reading the operator's real
    `general.toml` compares expectations against whatever locale happens to be
    pinned there, and — since `startup.rs` turned `auto_detect_os_locale` on — a
    sandbox that left the key *unset* would take the language from the
    operator's OS, so the same probe would pass in Boston and fail in Lyon.

    Pointing `XDG_CONFIG_HOME` at a scratch directory fixes that, and also
    side-steps a settings file written by a newer build (e.g. `workspace.toml`
    on a schema version the running binary predates), which otherwise makes
    every launch fall back to in-memory defaults.

    `pins` takes any dotted key from the app's settings schema. `locale`, `dark`
    and `show_welcome` are just the three most-wanted keys promoted to named
    arguments with defaults (`label` names the sandbox, and is not a setting);
    anything in `pins` overrides them. Run `skribisto --dump-config` for the full
    list with types and current values.

        env = isolated_config(pins={"editor.autosave": False,
                                    "editor.typewriter_scroll": False,
                                    "editor.scene.size": 1.25})

    **Nothing validates these keys.** This function writes `general.toml`
    directly, and the store silently ignores a key nothing reads — so a typo is
    not an error, it is a probe quietly running on defaults. Where that matters,
    write the same dict with `config_pins_file` and pass `--config` on the command
    line instead: the app validates every key against its schema and refuses to
    start on an unknown one. Use both together — this for the sandbox, the flag
    for the pins.

    The settings themselves are written by [`write_settings`], which a probe that
    builds its own sandbox calls directly — this is that plus the scratch
    directory and the env dict.
    """
    base = SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir()
    root = os.path.join(base, f"probe-config-{label}-{os.getpid()}")
    write_settings(root, locale=locale, dark=dark, show_welcome=show_welcome, pins=pins,
                   pace_summary=pace_summary)
    return sandbox_env(root)


def config_pins_file(pins, label="pins"):
    """Write `pins` to a scratchpad TOML file and return its path, for `--config`.

    The validating half of the pair with `isolated_config`. Passing the returned
    path as `skribisto --config <path>` makes the app check every key against its
    schema (`crates/teksilo_ui/src/settings_keys.rs`) before it starts, and exit
    non-zero naming the offender — with its nearest legal neighbour — on an
    unknown key or a value of the wrong type, rather than silently ignoring a
    typo'd pin.

    `--config` also implies `--new-instance`, so a run holding pins can never be
    elected away to a primary that is not holding them.

    It merges into whatever configuration directory the process resolves, so it
    belongs with a sandboxed `XDG_CONFIG_HOME` — pass the env from
    `isolated_config` and the two compose:

        env = isolated_config(label="mine")
        pins = config_pins_file({"editor.autosave": False}, label="mine")
        subprocess.Popen([SKRIBISTO, "--config", pins, project], env=env)
    """
    base = SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir()
    os.makedirs(base, exist_ok=True)
    path = os.path.join(base, f"probe-pins-{label}-{os.getpid()}.toml")
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(toml_pins(pins))
    return path


def _make_writable(path):
    """Give the owner write permission, for a file or a whole tree."""
    def w(p):
        try:
            os.chmod(p, os.stat(p).st_mode | 0o200)
        except OSError:
            pass
    w(path)
    if os.path.isdir(path):
        for root, dirs, files in os.walk(path):
            for n in dirs + files:
                w(os.path.join(root, n))


def column_right(nodes, inner):
    """Right edge of the innermost `ScrollView` holding `inner`'s top-left corner.

    What a clipping check measures a control against: the column it sits in.
    The rightmost label in that column is not its edge, it moves whenever the
    labels get shorter, and a control ending on the column's padding then reads
    as overflowing. None when no scroll column holds `inner`.
    """
    b = inner.get("bounds") or {}
    x, y = b.get("x", 0), b.get("y", 0)
    columns = [n.get("bounds") or {} for n in nodes if n.get("role") == "ScrollView"]
    holding = [c for c in columns
               if c.get("x", 0) <= x < c.get("x", 0) + c.get("width", 0)
               and c.get("y", 0) <= y < c.get("y", 0) + c.get("height", 0)]
    if not holding:
        return None
    inner_most = min(holding, key=lambda c: c.get("width", 0) * c.get("height", 0))
    return inner_most.get("x", 0) + inner_most.get("width", 0)


def working_copy(src, label="fixture"):
    """Copy `src` into the scratchpad and return the copy's path.

    Works for both `.skrib` shapes: a zip file is copied, an exploded folder is
    copied whole. The name carries `label` and the pid so parallel probes — and
    successive runs of one probe — never share a file.
    """
    base = SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir()
    os.makedirs(base, exist_ok=True)
    src = os.path.abspath(src)
    if not os.path.exists(src):
        raise FileNotFoundError(f"no fixture at {src}")

    stem = os.path.basename(src.rstrip("/")) or "project"
    dst = os.path.join(base, f"probe-{label}-{os.getpid()}-{stem}")
    # A previous run at the same pid (or a crash) can leave one behind.
    if os.path.isdir(dst):
        shutil.rmtree(dst)
    elif os.path.exists(dst):
        os.remove(dst)

    if os.path.isdir(src):
        shutil.copytree(src, dst)
    else:
        shutil.copy2(src, dst)

    # `copy2` preserves the source's mode, and the checked-in fixture is kept
    # read-only precisely so a stray write to it fails loudly. Without this the
    # protection would follow the copy and the app would fail to save into its
    # own scratch project — a confusing failure a long way from its cause.
    _make_writable(dst)

    # Belt and braces: if this ever returns a path inside the repo, the probe is
    # about to do the exact thing this module exists to prevent.
    repo = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    assert not os.path.abspath(dst).startswith(repo + os.sep), (
        f"working copy landed inside the repo ({dst}) — refusing to hand it back"
    )
    return dst


# ---------------------------------------------------------------------------
# Where the binaries are
# ---------------------------------------------------------------------------
#
# A probe that names `/home/cyril/Devel/...` is a demo of a bug, not a test of
# one: it passes on exactly one machine and cannot be run by CI, by a
# contributor, or from a git worktree. Both binaries are resolved here instead,
# and both are overridable, so a probe stays a probe wherever it is checked out.

#: Override the app binary (an installed build, a release build, another
#: checkout). Absolute path.
SKRIBISTO_BIN_ENV = "SKRIBISTO_BIN"

#: Override the automation MCP server binary.
MCP_BIN_ENV = "TEKSILO_MCP_BIN"


def repo_root():
    """The Skribisto checkout this file belongs to.

    Derived from `__file__` rather than the working directory, so it is right
    however the probe was invoked — and, in a `.claude/worktrees/` checkout, it
    is that worktree's root. Guessing would silently drive the *main* checkout's
    binary and report its behaviour as the worktree's, which is the failure this
    is placed here to prevent.
    """
    return os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def repo_path(*parts):
    """A path to something inside this checkout — a fixture, a resource, a binary.

        fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

    The same reason as `repo_root`: a probe that spells the path out in full
    tests one machine's filesystem, and reports its own absence as a failure of
    the app.
    """
    return os.path.join(repo_root(), *parts)


#: Override the teksilo checkout used to build the MCP server from source.
TEKSILO_ROOT_ENV = "TEKSILO_ROOT"


def teksilo_version():
    """The teksilo version this checkout pins, read from `teksilo_ui`'s manifest.

    The MCP server has to speak the same wire protocol as the bridge inside the
    app, and that protocol is not frozen: 0.9.3 replaced the socket announce with
    an endpoint descriptor and put a deadline on the token handshake. A server
    older than the app therefore fails at connect time with a symptom that names
    neither version, which is why [`mcp_binary`] compares them and this reads the
    number to compare against.

    Returns None if the manifest cannot be read, leaving the caller to skip the
    check rather than refuse to run over it.
    """
    manifest = os.path.join(repo_root(), "crates", "teksilo_ui", "Cargo.toml")
    try:
        with open(manifest, "rb") as fh:
            dep = tomllib.load(fh).get("dependencies", {}).get("teksilo")
    except (OSError, tomllib.TOMLDecodeError):
        return None
    if isinstance(dep, str):
        return dep.lstrip("^~=")
    if isinstance(dep, dict) and dep.get("version"):
        return str(dep["version"]).lstrip("^~=")
    return None


def _version_tuple(text):
    """`"0.9.4"` → `(0, 9, 4)`. Non-numeric tails are dropped, so a pre-release sorts low."""
    parts = []
    for chunk in str(text).split("."):
        digits = re.match(r"\d+", chunk)
        if not digits:
            break
        parts.append(int(digits.group()))
    return tuple(parts)


def teksilo_root():
    """A teksilo checkout to build the MCP server from, or None.

    ⚠ **teksilo is a registry dependency, not a path one.** This used to read
    `teksilo = { path = ... }` out of a crate manifest, which was the only
    statement of the relationship that could not go stale — right up until the
    dependency became `teksilo = { version = "0.9.4" }` and no manifest named a
    path at all. The function then returned None, `mcp_binary` had nothing to
    try, and all 72 probes died on a `FileNotFoundError` whose message told the
    reader to build the server in `<teksilo checkout>` without saying where that
    was. Nothing in the repo noticed, because no workflow runs these.

    So the checkout is now genuinely optional, and only a convenience for
    someone developing teksilo alongside this app. `$TEKSILO_ROOT` names it; a
    sibling directory beside this checkout is the fallback guess. The supported
    route for everyone else is `cargo install teksilo-automation-mcp`, which
    [`mcp_binary`] finds on `$PATH` and which the error message now names.
    """
    override = os.environ.get(TEKSILO_ROOT_ENV)
    if override:
        return override if os.path.isdir(override) else None
    for candidate in (
        # A worktree reaches its framework through this symlink; check it first,
        # so a worktree builds against its own teksilo and not the main one.
        os.path.join(repo_root(), ".claude", "worktrees", "teksilo"),
        os.path.join(os.path.dirname(repo_root()), "teksilo"),
    ):
        if os.path.isdir(os.path.join(candidate, "crates", "teksilo-automation-mcp")):
            return candidate
    return None


def _target_dirs(root):
    """Cargo's target directory for `root`, honouring `$CARGO_TARGET_DIR`.

    A contributor or CI runner that redirects the target directory has a real
    binary on disk that `<root>/target` never finds — and the resulting "build
    it with cargo build" message would send them to rebuild something they had
    already built. Checked first, since a set `CARGO_TARGET_DIR` is where cargo
    would actually have put it.
    """
    override = os.environ.get("CARGO_TARGET_DIR")
    dirs = [override] if override else []
    return dirs + [os.path.join(root, "target")]


def _resolve(env_var, name, candidates, build_hint):
    """First of `candidates` that exists, else `PATH`, else a message that says what to build."""
    override = os.environ.get(env_var)
    if override:
        if not os.path.exists(override):
            raise FileNotFoundError(f"{env_var} points at {override}, which does not exist")
        return override
    for path in candidates:
        if path and os.path.exists(path):
            return path
    found = shutil.which(name)
    if found:
        return found
    tried = "\n  ".join(p for p in candidates if p) or "(nothing to try)"
    raise FileNotFoundError(
        f"cannot find `{name}`. Tried:\n  {tried}\nand $PATH.\n"
        f"Build it with:\n  {build_hint}\nor set ${env_var} to an existing binary."
    )


def skribisto_binary():
    """Path to this checkout's debug `skribisto`, or ``$SKRIBISTO_BIN``.

    Debug and not release on purpose: the automation bridge every probe drives is
    `#[cfg(debug_assertions)]`, so a release binary has no socket to connect to
    and the probe would hang waiting for a line it can never print.
    """
    root = repo_root()
    return _resolve(
        SKRIBISTO_BIN_ENV, "skribisto",
        [os.path.join(t, "debug", "skribisto" + EXE) for t in _target_dirs(root)],
        f"cargo build -p teksilo_ui   # in {root}",
    )


#: Set to "1"/"yes" to install the MCP client without asking, "0"/"never" to
#: refuse. Unset means: ask, if there is a terminal to ask on.
MCP_AUTOINSTALL_ENV = "SKRIBISTO_MCP_AUTOINSTALL"


def mcp_install_command(version=None):
    """The command that puts `teksilo-automation-mcp` on `$PATH`.

    ⚠ **The authority on this is teksilo, not this file.**
    `teksilo_automation::client::install_command` composes the same string from
    the toolkit's own version, and the app prints it on startup when the client
    is missing. This is the same command, reachable before the app has been
    launched — which is when a probe needs it, since it must spawn the client
    itself. The version comes from *this* checkout's own pin, which is a thing
    Skribisto legitimately knows about itself.
    """
    version = version or teksilo_version()
    cmd = ["cargo", "install", "teksilo-automation-mcp"]
    if version:
        cmd += ["--version", version]
    return cmd + ["--locked"]


def _wants_install(prompt):
    """Ask whether to install. Env var wins; otherwise ask, if anyone can answer.

    A probe run by CI or by an agent has no terminal, and a prompt there is not
    a question, it is a hang. So the absence of a TTY means "no" and the caller
    raises with the command spelled out instead.
    """
    choice = os.environ.get(MCP_AUTOINSTALL_ENV, "").strip().lower()
    if choice in ("1", "y", "yes", "true", "always"):
        return True
    if choice in ("0", "n", "no", "false", "never"):
        return False
    try:
        if not sys.stdin.isatty():
            return False
        return input(prompt).strip().lower() in ("y", "yes")
    except (OSError, EOFError, KeyboardInterrupt):
        return False


def install_mcp(version=None):
    """Run `cargo install teksilo-automation-mcp`, and return the resulting path.

    Raises `RuntimeError` if cargo is missing or the build fails — with cargo's
    own output, which says far more than "install failed" ever could.
    """
    cmd = mcp_install_command(version)
    if not shutil.which("cargo"):
        raise RuntimeError(
            "cargo is not on $PATH, so `teksilo-automation-mcp` cannot be installed "
            f"automatically. Install Rust, then run:\n  {' '.join(cmd)}"
        )
    print(f"installing : {' '.join(cmd)}\n  (this builds the client once; it takes a "
          "couple of minutes and then never again)")
    done = subprocess.run(cmd, capture_output=True, text=True)
    if done.returncode != 0:
        tail = "\n".join((done.stderr or done.stdout).splitlines()[-20:])
        raise RuntimeError(f"`{' '.join(cmd)}` failed (exit {done.returncode}):\n{tail}")
    found = shutil.which("teksilo-automation-mcp")
    if not found:
        raise RuntimeError(
            f"`{' '.join(cmd)}` reported success but the binary is still not on "
            "$PATH. Is cargo's bin directory (usually ~/.cargo/bin) on it?"
        )
    print(f"installed  : {found}")
    return found


def mcp_binary():
    """Path to `teksilo-automation-mcp`, installing it if it is missing.

    A probe has to spawn the client itself, so it has to resolve a path to it.
    That is all this does. **Why** the client is a separate binary, and what to
    do when it is absent, is teksilo's to say and it says it:
    `teksilo_automation::client` owns the name, the matching version and the
    install command, and a debug build prints them on startup when the client
    is not on `$PATH`. This file used to carry its own copy of that reasoning,
    including an account of teksilo's protocol history, which is not an
    application's business to know.

    Resolution order, `$PATH` first:

    1. ``$TEKSILO_MCP_BIN`` — an explicit override, which must exist.
    2. ``$PATH`` — where `cargo install` puts it, and the supported route.
    3. A teksilo checkout's `target/debug` — for someone developing the
       framework and the app together; opt in with ``$TEKSILO_ROOT``, or have
       the checkout beside this one. Deliberately *after* `$PATH`, because a
       checkout can hold a build from any point in its history and silently
       preferring it is how a stale client ends up talking to a current app.
    4. Nothing found — offer to install, or raise naming the command.
    """
    want = teksilo_version()
    cmd = " ".join(mcp_install_command(want))

    override = os.environ.get(MCP_BIN_ENV)
    if override:
        if not os.path.exists(override):
            raise FileNotFoundError(
                f"${MCP_BIN_ENV} points at {override}, which does not exist")
        _check_mcp_version(override, want, cmd)
        return override

    found = shutil.which("teksilo-automation-mcp")
    if not found:
        tek = teksilo_root()
        for target in (_target_dirs(tek) if tek else []):
            candidate = os.path.join(target, "debug", "teksilo-automation-mcp" + EXE)
            if os.path.exists(candidate):
                found = candidate
                break

    if not found:
        if _wants_install(
            f"\n`teksilo-automation-mcp` is not installed.\n"
            f"  Install it now with `{cmd}`? [y/N] "
        ):
            found = install_mcp(want)
        else:
            raise FileNotFoundError(
                "cannot find `teksilo-automation-mcp`, the MCP client these probes "
                f"drive the app through.\nInstall it with:\n  {cmd}\n"
                f"then re-run. (Set ${MCP_AUTOINSTALL_ENV}=1 to have this do it for "
                f"you, or ${MCP_BIN_ENV} to point at an existing build.)"
            )

    _check_mcp_version(found, want, cmd)
    return found


def mcp_version(binary=None):
    """The MCP client's own version string, or None if it will not say.

    Deliberately tolerant: a client that cannot be asked is not a reason to
    refuse to run, only a reason to skip the comparison.
    """
    binary = binary or os.environ.get(MCP_BIN_ENV) or shutil.which("teksilo-automation-mcp")
    if not binary:
        return None
    try:
        out = subprocess.run([binary, "--version"], capture_output=True,
                             text=True, timeout=30).stdout
    except (OSError, subprocess.SubprocessError):
        return None
    m = re.search(r"(\d+\.\d+\.\d+\S*)", out)
    return m.group(1) if m else None


def _check_mcp_version(binary, want, install_cmd):
    """Refuse a client older than the teksilo this checkout builds against.

    The reasoning behind the rule lives in `teksilo_automation::client`; the
    rule is enforced here because this is what spawns the client. Offers the
    same install as [`mcp_binary`], since "yours is too old" and "you have none"
    want the same one-line answer.
    """
    if not want:
        return
    have = mcp_version(binary)
    if not have or _version_tuple(have) >= _version_tuple(want):
        return
    problem = (
        f"`{binary}` is teksilo-automation-mcp {have}, but this checkout builds "
        f"against teksilo {want}, whose bridge protocol it predates. It would "
        f"fail at connect time."
    )
    if _wants_install(f"\n{problem}\n  Update it now with `{install_cmd}`? [y/N] "):
        install_mcp(want)
        return
    raise RuntimeError(
        f"{problem}\nUpdate it with:\n  {install_cmd}\n"
        f"or point ${MCP_BIN_ENV} at a build of {want} or newer."
    )


# ---------------------------------------------------------------------------
# Launching the app, and attaching the bridge to it
# ---------------------------------------------------------------------------
#
# Every probe used to open-code these three steps, and all 72 of them carried
# the same two assumptions: that a launch with no flags gets a process of its
# own, and that the bridge announces itself as `bridge socket = ...`. Neither
# survived. The single-instance election means a bare launch can hand off to a
# primary and exit — so the probe drives someone else's window, or none — and
# 0.9.3 renamed the announce to `bridge endpoint = ...`, which every probe's
# regex missed, waited 60 seconds for, and reported as "no bridge socket".
#
# Both are decided here now, once.

#: The bridge's announce line. 0.9.3 renamed `socket` to `endpoint`; both are
#: accepted so a probe still runs against an older app binary.
BRIDGE_ENDPOINT_RE = re.compile(r"bridge (?:endpoint|socket) = (\S+)")
BRIDGE_TOKEN_RE = re.compile(r"TEKSILO_AUTOMATION_TOKEN=(\S+)")
BRIDGE_DESCRIPTOR_RE = re.compile(r"teksilo-automation: descriptor = (\S+)")

#: What [`wait_for_bridge`] hands back. `pid` is the app process that owns the
#: bridge, which is the launched process only because every probe passes
#: `--new-instance` — see [`launch_argv`].
Bridge = collections.namedtuple("Bridge", "endpoint token descriptor pid")


def launch_argv(project=None, pins=None, new_instance=True, extra=()):
    """The command line every probe should launch the app with.

    Two things are always wanted and were usually missing:

    **`--new-instance`.** The first live copy wins an election and becomes the
    primary; every later launch forwards its command line over a socket and
    exits in milliseconds. A probe that launches without this flag while any
    other copy is up therefore scrapes the announce of a process on its way out,
    and every bridge call then times out — a symptom several layers from its
    cause, which `assert_no_running_instance` used to guard against by refusing
    to run at all. The flag removes the failure mode instead of detecting it,
    and lets probes run beside the operator's own session.

    **`--config`.** `isolated_config` gives the run a private
    `XDG_CONFIG_HOME`, but nothing validates the keys written into it — a typo
    is not an error, it is a probe quietly running on defaults. Passing the same
    pins through `--config` makes the app check every one against its schema and
    refuse to start on an unknown key. Use both: the sandbox for isolation, the
    flag for validation. (`--config` implies `--new-instance`; the flag is still
    passed explicitly, because a probe that drops its pins should not silently
    lose its own process too.)

        env = fixture.isolated_config(locale="en-US", label="mine")
        pins = fixture.config_pins_file({"editor.autosave": False}, label="mine")
        app = subprocess.Popen(fixture.launch_argv(project, pins=pins),
                               stdout=open(log, "w"),
                               stderr=subprocess.STDOUT, env=env)

    `project` may be None (launch to the Launcher), a path, or a list of
    arguments; `extra` appends further flags before it.
    """
    argv = [skribisto_binary()]
    if new_instance:
        argv.append("--new-instance")
    if pins:
        argv += ["--config", pins]
    argv += list(extra)
    if project is None:
        return argv
    if isinstance(project, (list, tuple)):
        return argv + [str(p) for p in project]
    return argv + [str(project)]


def wait_for_bridge(log, proc=None, timeout=90.0, interval=0.2):
    """Poll the app's log until the automation bridge announces itself.

    Returns a [`Bridge`]. Raises `RuntimeError` on timeout, or as soon as the
    process exits without announcing — waiting the full timeout for a process
    that is already gone only delays the log tail the caller needs.

    The bridge binds the endpoint, publishes its descriptor and spawns the
    accept thread *before* printing any of this, so the endpoint is connectable
    the instant it is read and no client needs a retry loop. That ordering is
    load-bearing and is asserted here: if `endpoint` is announced before it
    exists, the announce-before-bind race is back (teksilo
    `automation_bridge::spawn_bridge_thread`) and every client of this bridge is
    racing, not just this one.
    """
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            text = open(log, encoding="utf-8", errors="replace").read()
        except OSError:
            text = ""
        endpoint = BRIDGE_ENDPOINT_RE.search(text)
        token = BRIDGE_TOKEN_RE.search(text)
        if endpoint and token:
            descriptor = BRIDGE_DESCRIPTOR_RE.search(text)
            path = endpoint.group(1)
            # Unix endpoints are filesystem paths; a Windows named pipe is not,
            # so only check the shape that can be checked.
            if path.startswith("/") and not os.path.exists(path):
                raise RuntimeError(
                    f"the bridge announced {path} before binding it — the "
                    "announce-before-bind race is back; see teksilo "
                    "automation_bridge::spawn_bridge_thread"
                )
            return Bridge(path, token.group(1),
                          descriptor.group(1) if descriptor else None,
                          proc.pid if proc is not None else None)
        if proc is not None and proc.poll() is not None:
            tail = "\n".join(text.splitlines()[-25:])
            raise RuntimeError(
                f"the app exited (code {proc.returncode}) before announcing its "
                f"automation bridge.\n--- log tail ---\n{tail}"
            )
        time.sleep(interval)
    tail = "\n".join(open(log, encoding="utf-8", errors="replace").read().splitlines()[-25:])
    raise RuntimeError(
        f"no automation bridge announced within {timeout:g}s.\n"
        "A release build has no bridge at all (it is `#[cfg(debug_assertions)]`), "
        "and a build without teksilo's `automation` feature has none either.\n"
        f"--- log tail ---\n{tail}"
    )


def mcp_argv(bridge, mcp=None):
    """The command line that attaches the MCP server to `bridge`.

    Prefers `--attach-pid`, which reads the endpoint and the token out of the
    descriptor the app published. The alternative, `--connect <endpoint> --token
    <uuid>`, puts the token on a command line — and a command line is readable
    by every user on the machine through `/proc/<pid>/cmdline`, which is the
    exposure 0.9.3 tightened the socket mode and the descriptor mode to close.
    Falls back to `--connect` when the pid is unknown (a bridge discovered from
    a log rather than a process this probe launched).

    Always `--connect` on Windows. There the app publishes its descriptor under
    `%LOCALAPPDATA%\\Teksilo\\teksilo-automation\\`, and [`sandbox_env`] moves the
    app's `%LOCALAPPDATA%` into the sandbox while the client runs in the probe's
    own environment: `--attach-pid` looked in the real one and found nothing, for
    every probe. The token on the command line is readable by the same user only.
    """
    mcp = mcp or mcp_binary()
    if bridge.pid is not None and not IS_WINDOWS:
        return [mcp, "--attach-pid", str(bridge.pid)]
    return [mcp, "--connect", bridge.endpoint, "--token", bridge.token]


class McpError(RuntimeError):
    """A bridge call that was refused, reported an error, or got no answer."""


class McpSession:
    """The MCP client attached to one app's bridge, for a probe to call.

        session = fixture.McpSession(bridge, "my-probe")
        payload = session.call("snapshot_tree")
        session.close()

    Portable where the open-coded clients in older probes are not: the client's
    output is read on a thread into a queue, so a call waits with a deadline
    without `select` on a pipe (which Windows does not have). A refused op is an
    exception, never a quiet no-op: the bridge answers an argument outside its
    vocabulary with a JSON-RPC error having performed nothing, and a probe that
    carried on would fail far from the cause.
    """

    def __init__(self, bridge, name, mcp=None, timeout=30.0):
        import queue
        import threading

        self._lines = queue.Queue()
        self._next_id = 0
        self._errors = tempfile.TemporaryFile(mode="w+")
        self.proc = subprocess.Popen(
            mcp_argv(bridge, mcp), stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=self._errors, text=True, encoding="utf-8", bufsize=1)

        def pump(stream, lines):
            for line in stream:
                if line.strip():
                    lines.put(line)
            lines.put(None)

        threading.Thread(target=pump, args=(self.proc.stdout, self._lines),
                         daemon=True).start()
        self.request("initialize", {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": name, "version": "1"},
        }, timeout=timeout)

    def request(self, method, params=None, timeout=30.0):
        """Send one JSON-RPC request and return its `result`."""
        import json
        import queue

        self._next_id += 1
        message = {"jsonrpc": "2.0", "id": self._next_id, "method": method}
        if params is not None:
            message["params"] = params
        try:
            self.proc.stdin.write(json.dumps(message) + "\n")
            self.proc.stdin.flush()
        except OSError as e:
            raise McpError(f"{method}: the MCP client is gone ({e}){self._stderr_tail()}")
        deadline = time.monotonic() + timeout
        while True:
            remaining = deadline - time.monotonic()
            try:
                line = self._lines.get(timeout=max(0.0, remaining))
            except queue.Empty:
                raise McpError(f"{method}: no answer within {timeout:g}s{self._stderr_tail()}")
            if line is None:
                raise McpError(f"{method}: the MCP client exited{self._stderr_tail()}")
            reply = json.loads(line)
            if reply.get("id") != self._next_id:
                continue  # a notification, or the answer to an abandoned call
            if (error := reply.get("error")) is not None:
                raise McpError(f"{method} {params or {}} was refused: "
                               f"{error.get('message', error)}")
            return reply.get("result", {})

    def call(self, tool, args=None, timeout=30.0):
        """Call a bridge tool; return its structured payload (a dict)."""
        import json

        result = self.request("tools/call", {"name": tool, "arguments": args or {}},
                              timeout=timeout)
        text = "".join(c.get("text", "") for c in result.get("content", [])
                       if c.get("type") == "text")
        if result.get("isError"):
            raise McpError(f"{tool} {args or {}} reported an error: {text or result}")
        payload = result.get("structuredContent")
        if payload is None:
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return payload

    def nodes(self):
        """The accessibility tree, as a flat list of nodes."""
        return self.call("snapshot_tree").get("nodes", [])

    def settle(self, times=3):
        for _ in range(times):
            self.call("settle")

    def close(self):
        if self.proc.poll() is None:
            self.proc.terminate()
            try:
                self.proc.wait(timeout=10)
            except subprocess.TimeoutExpired:
                self.proc.kill()

    def _stderr_tail(self):
        try:
            self._errors.seek(0)
            tail = "\n".join(self._errors.read().splitlines()[-10:])
        except (OSError, ValueError):
            return ""
        return f"\n--- MCP client stderr ---\n{tail}" if tail else ""
