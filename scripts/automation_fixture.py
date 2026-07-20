# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""One rule, in one place: a live-app probe never opens a checked-in fixture.

Driving the real app means Ctrl+S, autosave, and format migration all happen for
real. Any of them rewrites whatever file was passed on the command line. Three
separate incidents came from forgetting that:

  * a probe saved into `resources/test/skribisto_test_project.skrib`, so the next
    run's toggle silently undid the previous run's;
  * a run migrated the same fixture from format v2 to v3, which broke a test that
    had pinned the version — the test looked wrong, the probe was;
  * the same file was later reverted on the assumption a *test* had rewritten it.
    No test does. Bisecting the suite proved it, after the fact.

None of those were caught by the probe that caused them, because a probe that
mutates its own fixture still passes — it just stops testing what it claims to,
and takes the repo's copy with it.

So: `working_copy()` hands back a throwaway in the scratchpad, and probes open
*that*. Cheap enough (a few MB) that there is no reason to skip it.
"""

import os
import shutil
import subprocess
import tempfile

#: Session scratchpad when one is set, else the system temp dir. Never inside the repo.
#:
#: The fallback is deliberately generic. An earlier version hard-coded one session's
#: scratchpad path, which stops existing the moment that session ends — so every later
#: run silently fell through to the temp dir anyway, while the dead path sat in the
#: source looking authoritative. Set `SKRIBISTO_AUTOMATION_SCRATCH` to steer it.
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

    Skribisto runs one process per project behind an open-registry lock, and a
    second launch HANDS OFF to the existing instance and exits. A probe that
    launches into that situation still scrapes a `bridge socket = ...` line out
    of its log — belonging to a process which is on its way out — and then every
    single call times out. The probe reports "the modal did not open" or "the
    fixture did not load", naming a symptom several layers from the cause.

    That happened three runs in a row here, and the diagnosis (a `Form` landmark
    that had supposedly regressed) was entirely wrong: the landmark was fine, the
    probe was talking to a corpse. Failing loudly up front costs one line and
    saves that whole detour.

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


def isolated_config(locale="fr-FR", label="cfg", dark=False, show_welcome=True):
    """A private `XDG_CONFIG_HOME` pinning the app's locale. Returns an env dict.

    A probe that asserts on translated text must SET the language, never inherit
    it. This one learned that the expensive way: `automation_tag_presets.py`
    asserts the Basic preset's tags come out in French (plan step 9 — presets are
    generated through `tr!`, so they must translate), but the app reads its
    locale from `~/.config/skribisto/general.toml` and that file says
    `[ui] locale = "en-US"`, with `auto_detect_os_locale(false)` in main.rs
    meaning the machine's French locale is never consulted. The probe therefore
    compared French expectations against a correctly-English app and reported a
    translation bug that did not exist — and the diagnosis that followed chased a
    non-existent fault in bastyde's i18n layer for some time before the probe's
    own output turned out to be printing its French constants unconditionally.

    Pointing `XDG_CONFIG_HOME` at a scratch directory fixes both halves: the
    locale is whatever the probe says, and the run cannot read or write the
    operator's real settings. It also side-steps a settings file written by a
    newer build (`workspace.toml` on schema v3 against a v2 reader), which
    otherwise makes every launch fall back to in-memory defaults.

    `AppPaths::new("eu", "skribisto", "Skribisto")` resolves to
    `$XDG_CONFIG_HOME/skribisto` on Linux, and `config_file("general")` appends
    `.toml` — hence the layout written here.
    """
    base = SCRATCH if os.path.isdir(SCRATCH) else tempfile.gettempdir()
    root = os.path.join(base, f"probe-config-{label}-{os.getpid()}")
    cfg = os.path.join(root, "skribisto")
    os.makedirs(cfg, exist_ok=True)
    with open(os.path.join(cfg, "general.toml"), "w", encoding="utf-8") as fh:
        fh.write(
            "[ui]\n"
            f"dark = {str(bool(dark)).lower()}\n"
            f'locale = "{locale}"\n'
            f"show_welcome = {str(bool(show_welcome)).lower()}\n"
        )
    env = dict(os.environ)
    env["XDG_CONFIG_HOME"] = root
    return env


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
