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

import os
import shutil
import subprocess
import tempfile

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


def assert_no_running_instance(binary=None):
    """Refuse to launch while another instance of the same build is up.

    The single-instance election hands a second launch off to the existing
    process and exits, so a probe that launches into that situation scrapes a
    `bridge socket = ...` line belonging to a process on its way out, and every
    call then times out — reporting a symptom (e.g. "the modal did not open")
    several layers from the real cause. Failing loudly up front costs one line.

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
    live = [l for l in out.splitlines() if l.strip()]
    if live:
        listing = "\n  ".join(live)
        raise RuntimeError(
            f"another instance of {binary} is already running:\n  {listing}\n"
            "Launching now would hand off to it and exit, and every bridge call "
            "would time out. Close it (or `pkill -f target/debug/skribisto` if it "
            "is a probe leftover and you have nothing unsaved) and re-run."
        )


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


def isolated_config(locale="fr-FR", label="cfg", dark=False, show_welcome=True, pins=None):
    """A private `XDG_CONFIG_HOME` with the app's settings pinned. Returns an env dict.

    A probe that asserts on translated text must SET the language, never inherit
    it — `auto_detect_os_locale(false)` in main.rs means the OS locale is never
    consulted, so a probe reading the operator's real `general.toml` compares
    expectations against whatever locale happens to be pinned there instead.

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

    `AppPaths::new("eu", "skribisto", "Skribisto")` resolves to
    `$XDG_CONFIG_HOME/skribisto` on Linux, and `config_file("general")` appends
    `.toml` — hence the layout written here.
    """
    settings = {
        "ui.dark": bool(dark),
        "ui.locale": locale,
        "ui.show_welcome": bool(show_welcome),
    }
    settings.update(pins or {})

    base = SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir()
    root = os.path.join(base, f"probe-config-{label}-{os.getpid()}")
    cfg = os.path.join(root, "skribisto")
    os.makedirs(cfg, exist_ok=True)
    with open(os.path.join(cfg, "general.toml"), "w", encoding="utf-8") as fh:
        fh.write(toml_pins(settings))
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
