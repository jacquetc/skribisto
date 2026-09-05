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
        "target", "debug", "skribisto",
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


def write_settings(config_home, locale="en-US", dark=False, show_welcome=True, pins=None):
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

    `config_home` is the directory the probe puts in `XDG_CONFIG_HOME`;
    `AppPaths::new("eu", "skribisto", "Skribisto")` resolves to `skribisto`
    under it and `config_file("general")` appends `.toml` — hence the layout
    written here. Nothing validates the keys — see `isolated_config`'s note on
    pairing this with `config_pins_file` where that matters.
    """
    settings = {
        "ui.dark": bool(dark),
        "ui.locale": locale,
        "ui.show_welcome": bool(show_welcome),
    }
    settings.update(pins or {})
    cfg = os.path.join(config_home, "skribisto")
    os.makedirs(cfg, exist_ok=True)
    path = os.path.join(cfg, "general.toml")
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(toml_pins(settings))
    return path


def isolated_config(locale="fr-FR", label="cfg", dark=False, show_welcome=True, pins=None):
    """A private `XDG_CONFIG_HOME` with the app's settings pinned. Returns an env dict.

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
    write_settings(root, locale=locale, dark=dark, show_welcome=show_welcome, pins=pins)
    env = dict(os.environ)
    env["XDG_CONFIG_HOME"] = root
    return env


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
        [os.path.join(t, "debug", "skribisto") for t in _target_dirs(root)],
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
            candidate = os.path.join(target, "debug", "teksilo-automation-mcp")
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
    """
    mcp = mcp or mcp_binary()
    if bridge.pid is not None:
        return [mcp, "--attach-pid", str(bridge.pid)]
    return [mcp, "--connect", bridge.endpoint, "--token", bridge.token]
