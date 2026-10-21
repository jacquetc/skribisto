#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Prove a launch with the System theme keeps following the desktop.

The bug this guards: `TeksiloAppBuilder::theme(..)` puts teksilo in
`ThemeMode::Manual`, so a writer who had picked *System* got the right light or
dark at launch and then an app deaf to the desktop until they picked *System*
again in Settings. `startup::launch_theme_mode` now launches *System* in
`ThemeMode::Native`, answered by `startup::desktop_theme`.

The desktop is a **sandboxed** KDE one, never the operator's: the run gets its
own `XDG_CONFIG_HOME` holding a `kdeglobals`, which teksilo's Native mode reads
on a KDE session (light or dark is decided by the scheme's window background).
The probe rewrites that file between Breeze Light and Breeze Dark and emits
KDE's `org.kde.kconfig.notify.ConfigChanged` on `/kdeglobals`, the signal
teksilo's `OsThemeWatch` listens for. Other KDE applications hear the signal
too and re-read their own, unchanged, configuration: nothing on the operator's
desktop changes.

Two runs:

  A. `ui.theme_mode = "system"`: the window opens in the sandbox scheme's
     shade, turns dark when the scheme does, and light again.
  B. `ui.theme_mode = "light"`: the same change to dark is ignored. That is the
     armed half: a broken watch, or a screenshot that cannot tell the shades
     apart, would fail A without B ever passing for the wrong reason.

Light or dark is read off the rendered window (mean luminance of a
screenshot), not off a widget-tree value: the claim is about what the writer
sees.

Needs a KDE Plasma session (`XDG_CURRENT_DESKTOP` listing KDE), `gdbus`, Pillow
and the debug build of the default feature set.
"""
import io, json, os, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

try:
    from PIL import Image, ImageStat
except ImportError:
    sys.exit("this probe needs Pillow (python3-pil) to read the screenshots")

SCHEMES = "/usr/share/color-schemes"
LIGHT_SCHEME = os.path.join(SCHEMES, "BreezeLight.colors")
DARK_SCHEME = os.path.join(SCHEMES, "BreezeDark.colors")
# Mean luminance of the whole window, 0..1. IntUI's light surfaces sit far above
# the first, its dark ones far below the second; the gap keeps a half-repainted
# frame from counting as either.
LIGHT_ABOVE = 0.6
DARK_BELOW = 0.35
CHANGE_TIMEOUT = 8.0

failures = []


def check(ok, what, detail=""):
    print(f"{'ok  ' if ok else 'FAIL'}  {what}")
    if not ok:
        if detail:
            print(f"        {detail}")
        failures.append(what)
    return ok


def report():
    print()
    if failures:
        print(f"FAIL: {len(failures)} check(s) failed:")
        for f in failures:
            print(f"  - {f}")
        sys.exit(1)
    print("PASS")
    sys.exit(0)


def preflight():
    desktops = os.environ.get("XDG_CURRENT_DESKTOP", "").upper().split(":")
    if "KDE" not in desktops and os.environ.get("KDE_FULL_SESSION") != "true":
        sys.exit("this probe drives a sandboxed KDE desktop and needs a Plasma session")
    if not shutil.which("gdbus"):
        sys.exit("this probe needs gdbus to emit KDE's ConfigChanged signal")
    for path in (LIGHT_SCHEME, DARK_SCHEME):
        if not os.path.isfile(path):
            sys.exit(f"missing colour scheme {path}")


def write_kdeglobals(config_home, scheme_path, name):
    """A user `kdeglobals` carrying the whole scheme, as Plasma writes one."""
    with open(scheme_path, encoding="utf-8") as fh:
        scheme = fh.read()
    with open(os.path.join(config_home, "kdeglobals"), "w", encoding="utf-8") as fh:
        fh.write(f"[General]\nColorScheme={name}\n\n{scheme}")


def notify_kdeglobals_changed():
    subprocess.run(
        ["gdbus", "emit", "--session", "--object-path", "/kdeglobals",
         "--signal", "org.kde.kconfig.notify.ConfigChanged",
         "@a{saay} {'General': [b'ColorScheme']}"],
        check=True, capture_output=True,
    )


class Run:
    """One app process in its own sandbox, with an MCP client attached."""

    def __init__(self, label, theme_mode, scheme_path, scheme_name):
        pins = {"ui.locale": "en-US", "ui.show_welcome": True, "ui.theme_mode": theme_mode}
        self.env = fixture.isolated_config(locale="en-US", label=label, pins=pins)
        self.config_home = self.env["XDG_CONFIG_HOME"]
        write_kdeglobals(self.config_home, scheme_path, scheme_name)
        pins_file = fixture.config_pins_file(pins, label=label)
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen(fixture.launch_argv(pins=pins_file), env=self.env,
                                    stdout=open(self.log, "w"), stderr=subprocess.STDOUT)
        self.mcp = None
        self._id = 0
        bridge = fixture.wait_for_bridge(self.log, self.app, timeout=90)
        self.mcp = subprocess.Popen(fixture.mcp_argv(bridge), stdin=subprocess.PIPE,
                                    stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                                    text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "skribisto-theme-follow",
                                                 "version": "1"}})
        if self._recv() is None:
            raise RuntimeError("could not reach the automation bridge")
        self._send("notifications/initialized", notif=True)

    def _send(self, method, params=None, notif=False):
        msg = {"jsonrpc": "2.0", "method": method}
        if params is not None:
            msg["params"] = params
        if not notif:
            self._id += 1
            msg["id"] = self._id
        self.mcp.stdin.write(json.dumps(msg) + "\n")
        self.mcp.stdin.flush()

    def _recv(self, timeout=20):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                return None
            ready, _, _ = select.select([self.mcp.stdout], [], [],
                                        max(0.0, end - time.time()))
            if not ready:
                return None
            line = self.mcp.stdout.readline()
            if not line:
                return None
            if line.strip():
                return json.loads(line)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        return (self._recv() or {}).get("result", {})

    def wait_rendered(self):
        """Until the Launcher is on screen, so the first shot is of a window."""
        end = time.time() + 60
        while time.time() < end:
            result = self.call("snapshot_tree")
            text = "".join(c.get("text", "") for c in result.get("content", [])
                           if c.get("type") == "text").lower()
            if "welcome to skribisto" in text:
                return True
            time.sleep(0.5)
        return False

    def luminance(self):
        self.call("settle")
        result = self.call("screenshot")
        for c in result.get("content", []):
            if c.get("type") == "image" and c.get("data"):
                import base64
                image = Image.open(io.BytesIO(base64.b64decode(c["data"]))).convert("L")
                return ImageStat.Stat(image).mean[0] / 255.0
        return None

    def wait_luminance(self, want_dark, timeout=CHANGE_TIMEOUT):
        """Poll until the window reads as `want_dark`; the last reading either way."""
        end = time.time() + timeout
        value = None
        while time.time() < end:
            value = self.luminance()
            if value is not None and (value < DARK_BELOW if want_dark else value > LIGHT_ABOVE):
                return True, value
            time.sleep(0.4)
        return False, value

    def stop(self):
        for proc in (self.mcp, self.app):
            if proc and proc.poll() is None:
                proc.terminate()
        for proc in (self.mcp, self.app):
            if proc:
                try:
                    proc.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    proc.kill()

    def log_tail(self):
        return "\n".join(open(self.log, encoding="utf-8", errors="replace")
                         .read().splitlines()[-20:])


def fmt(value):
    return "no screenshot" if value is None else f"mean luminance {value:.2f}"


def switch_scheme(run, scheme_path, name):
    write_kdeglobals(run.config_home, scheme_path, name)
    notify_kdeglobals_changed()


preflight()

# ── A. System follows the desktop after a launch ────────────────────────────
print("A. ui.theme_mode = system")
run = Run("theme-follow-system", "system", LIGHT_SCHEME, "BreezeLight")
try:
    check(run.wait_rendered(), "the Launcher rendered", run.log_tail())
    ok, value = run.wait_luminance(want_dark=False)
    check(ok, "it opens light on a light desktop", fmt(value))

    switch_scheme(run, DARK_SCHEME, "BreezeDark")
    ok, value = run.wait_luminance(want_dark=True)
    check(ok, "it turns dark when the desktop does, without a restart", fmt(value))

    switch_scheme(run, LIGHT_SCHEME, "BreezeLight")
    ok, value = run.wait_luminance(want_dark=False)
    check(ok, "and light again", fmt(value))
finally:
    run.stop()

time.sleep(2.0)  # let the open registry release the first run's claims

# ── B. A manual choice stays deaf to it ─────────────────────────────────────
print("B. ui.theme_mode = light")
run = Run("theme-follow-light", "light", LIGHT_SCHEME, "BreezeLight")
try:
    check(run.wait_rendered(), "the Launcher rendered", run.log_tail())
    ok, value = run.wait_luminance(want_dark=False)
    check(ok, "it opens light", fmt(value))

    switch_scheme(run, DARK_SCHEME, "BreezeDark")
    turned, value = run.wait_luminance(want_dark=True, timeout=3.0)
    check(not turned, "it stays light when the desktop turns dark", fmt(value))
finally:
    run.stop()

report()
