#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Settings ▸ Appearance ▸ Text contrast and Text rendering, driven end to end.

The headless tests prove the vocabulary, the store round trip and that the stored
choice is copied into the live signals (`shared/text_rendering.rs`). What only a
live app can prove is the rest of the chain:

  1. **The pinned keys reach the page.** teksilo's own keys,
     `accessibility.text_contrast = "standard"` and
     `appearance.glyph_rendering = "natural"`, arrive through `--config`, so the
     schema entries and the store signals are in the loop, and the two controls
     show them.
  2. **Strongest draws darker text, with no restart.** A screenshot of the same
     page before and after the click: the text carries more ink.
  3. **Sharp moves the glyphs, with no restart.** The page is redrawn with
     different pixels at the same contrast.
  4. **Both are on disk**: the sandbox's `general.toml` holds what was picked.
  5. **System names what the desktop resolved to**: plain "System" while another
     choice holds, then "System (smooth)" or "System (sharp)" once picked, read
     from teksilo's `EffectiveGlyphRendering`. Outside GNOME the expected one is
     derived from fontconfig (`fc-match`), the same input teksilo reads there.

Runs against a throwaway copy of the bundled example in an isolated sandbox, with
the locale pinned to en-US so the labels matched below are the ones this repo
ships. Run it after `cargo build -p teksilo_ui`. Needs Pillow.
"""
import base64, io, json, os, shutil, subprocess, sys, tempfile, time, tomllib

from PIL import Image, ImageChops

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

fixture.assert_no_running_instance(SKRIBISTO)
env = fixture.isolated_config(locale="en-US", label="text-rendering", show_welcome=False)
pins = fixture.config_pins_file(
    {"ui.locale": "en-US", "ui.dark": False, "ui.theme_mode": "light",
     "ui.show_welcome": False,
     "accessibility.text_contrast": "standard", "appearance.glyph_rendering": "natural"},
    label="text-rendering")
work = fixture.working_copy(EXAMPLE, label="text-rendering")

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen(fixture.launch_argv(work, pins=pins), stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
mcp = None


def die(msg):
    print("FAIL:", msg)
    print("\n".join(open(log, errors="replace").read().splitlines()[-25:]))
    for p in (mcp, app):
        if p is not None and p.poll() is None:
            p.terminate()
    sys.exit(1)


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=90)
except RuntimeError as e:
    die(str(e))
mcp = subprocess.Popen(fixture.mcp_argv(bridge, MCP), stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, bufsize=1)
_id = 0


def call_raw(name, args=None):
    global _id
    _id += 1
    mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": _id, "method": "tools/call",
                                "params": {"name": name, "arguments": args or {}}}) + "\n")
    mcp.stdin.flush()
    while True:
        line = mcp.stdout.readline()
        if not line:
            return {}
        m = json.loads(line)
        if m.get("id") == _id:
            return m.get("result", {})


def call(name, args=None):
    r = call_raw(name, args)
    p = r.get("structuredContent")
    if p is None:
        txt = "".join(c.get("text", "") for c in r.get("content", [])
                      if c.get("type") == "text").strip()
        p = json.loads(txt) if txt.startswith(("{", "[")) else {}
    return p


def shot(name):
    """The window as the renderer draws it, saved under `name` and returned."""
    for c in call_raw("screenshot").get("content", []):
        if c.get("type") == "image" and c.get("data"):
            data = base64.b64decode(c["data"])
            path = os.path.join(fixture.SCRATCH, name)
            open(path, "wb").write(data)
            print(f"  screenshot -> {path}")
            return Image.open(io.BytesIO(data)).convert("L")
    die("the bridge returned no screenshot")


def text_box(img):
    """The pixel box of the Settings tree, from its User row to its Keymap row:
    text only, and nothing in it changes when a segment is picked, so the moving
    selection highlight cannot pass for darker text."""
    ns = nodes()
    rows = {(n.get("label") or "").strip(): n.get("bounds") for n in ns
            if n.get("role") == "TreeItem"}
    top, bottom = rows.get("User"), rows.get("Keymap")
    window = next((n.get("bounds") for n in ns if n.get("role") == "Window"), None)
    if not (top and bottom):
        die("could not find the Settings tree's User and Keymap rows")
    scale = img.size[0] / window["width"] if window and window.get("width") else 1.0
    return tuple(round(v * scale) for v in (
        top["x"], top["y"], top["x"] + top["width"], bottom["y"] + bottom["height"]))


def ink(img):
    """How much darkness the page carries: the sum of (255 - grey) over every
    pixel. On a light theme that is the text, and darker edges raise it."""
    hist = img.histogram()
    return sum((255 - v) * n for v, n in enumerate(hist))


mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                            "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                                       "clientInfo": {"name": "text-rendering",
                                                      "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


def has_label(label):
    return any((n.get("label") or "").strip() == label for n in nodes())


# `load_work` is slow in a debug build: wait for the binder, not a duration.
end = time.time() + 120
while time.time() < end and not has_label("Chapter 3"):
    time.sleep(0.5)
if not has_label("Chapter 3"):
    die("the binder never populated with the example's chapters; the bridge answered "
        f"{json.dumps(call_raw('snapshot_tree'))[:400]}")

# Settings opens on Appearance & Behavior ▸ Appearance (automation_settings.py).
call("inject_key", {"key": ",", "command": True})
end = time.time() + 15
while time.time() < end and not has_label("Strongest"):
    time.sleep(0.4)
if not has_label("Strongest"):
    die("Settings did not open on a page showing the Text contrast control")


def is_on(node):
    """A segment is a RadioButton; its state is `toggled`, reported as a string."""
    return str(node.get("toggled")).lower() == "true"


def control(member):
    """The labelled segments of the control that owns the segment `member`."""
    ns = nodes()
    seg = next((n for n in ns if (n.get("label") or "").strip() == member), None)
    if not seg:
        return {}
    parent = next((n for n in ns if seg["id"] in (n.get("children") or [])), None)
    if not parent:
        return {}
    by_id = {n["id"]: n for n in ns}
    kids = [by_id[c] for c in parent.get("children") or [] if c in by_id]
    return {(k.get("label") or "").strip(): k for k in kids if (k.get("label") or "").strip()}


def selected(member):
    return [label for label, n in control(member).items() if is_on(n)]


def pick(member):
    call("invoke_action", {"node": control(member)[member]["id"], "action": "click"})
    time.sleep(1.0)
    if selected(member) != [member]:
        die(f"clicking {member} left {selected(member)} selected")


# ── STEP 1: the pinned values are the ones shown ─────────────────────────────
contrast = control("Strongest")
if set(contrast) != {"Standard", "Stronger", "Strongest"}:
    die(f"the Text contrast control offers {sorted(contrast)}")
rendering = control("Sharp")
if set(rendering) != {"System", "Smooth", "Sharp"}:  # System is not the choice yet
    die(f"the Text rendering control offers {sorted(rendering)}")
if selected("Strongest") != ["Standard"] or selected("Sharp") != ["Smooth"]:
    die(f"the page shows {selected('Strongest')} / {selected('Sharp')} while the store "
        "holds Standard / Smooth")
print("  STEP 1 ok: the page shows Standard and Smooth, as pinned")

# ── STEP 2: Strongest carries more ink ───────────────────────────────────────
before = shot("text-rendering-standard.png")
pick("Strongest")
after = shot("text-rendering-strongest.png")
if before.size != after.size:
    die(f"the window changed size between the two screenshots: {before.size} → {after.size}")
box = text_box(before)
gain = ink(after.crop(box)) / max(ink(before.crop(box)), 1)
if gain <= 1.05:
    die(f"Strongest drew the page with {gain:.3f}× the ink of Standard; expected darker text")
print(f"  STEP 2 ok: Strongest drew the Settings tree's text with {gain:.3f}× the ink "
      "of Standard, with no restart")

# ── STEP 3: Sharp moves the glyphs ───────────────────────────────────────────
pick("Sharp")
sharp = shot("text-rendering-sharp.png")
if sharp.size != after.size:
    die(f"the window changed size between the two screenshots: {after.size} → {sharp.size}")
diff = ImageChops.difference(after.crop(box), sharp.crop(box))
changed = sum(n for v, n in enumerate(diff.histogram()) if v > 16)
if changed < 200:
    die(f"Sharp changed only {changed} pixels of the Settings tree's text by more than "
        "16 levels; expected glyphs redrawn on whole pixels")
print(f"  STEP 3 ok: Sharp redrew {changed} pixels of the Settings tree's text, "
      "with no restart")

# ── STEP 4: both are on disk ─────────────────────────────────────────────────
general = os.path.join(fixture.app_config_dir(env), "general.toml")
end = time.time() + 10  # the store writes after a short debounce
stored = {}
while time.time() < end:
    with open(general, "rb") as f:
        data = tomllib.load(f)
    stored = (data.get("accessibility", {}).get("text_contrast"),
              data.get("appearance", {}).get("glyph_rendering"))
    if stored == ("strongest", "crisp"):
        break
    time.sleep(0.5)
if stored != ("strongest", "crisp"):
    die(f"{general} holds text_contrast / glyph_rendering = {stored}, "
        "not strongest / crisp")
print("  STEP 4 ok: general.toml holds strongest and crisp under teksilo's keys")


# ── STEP 5: System names what the desktop resolved to ────────────────────────
def desktop_rendering():
    """What teksilo's FollowDesktop gives on this desktop, or None when this probe
    cannot tell. Mirrors `teksilo-platform`'s `font_hinting`: outside GNOME it asks
    fontconfig, which Plasma's font settings write at once (kdeglobals'
    `XftHintStyle` lags one Apply behind, so it is not read); medium or full
    hinting is sharp. GNOME's gsettings path is not reproduced here."""
    if sys.platform != "linux" or "GNOME" in os.environ.get("XDG_CURRENT_DESKTOP", ""):
        return None
    if not shutil.which("fc-match"):
        return None
    out = subprocess.run(["fc-match", "-f", "%{hinting}|%{hintstyle}", "sans-serif"],
                         capture_output=True, text=True).stdout.strip()
    hinting, _, style = out.partition("|")
    if hinting.strip().lower() == "false":
        return "smooth"
    if style.strip() not in ("0", "1", "2", "3"):
        return None
    return "sharp" if style.strip() in ("2", "3") else "smooth"


system = next((label for label in control("Sharp") if label.startswith("System")), None)
if system != "System":
    die(f"with Sharp chosen, the System segment reads {system!r}, not plain System")
call("invoke_action", {"node": control("Sharp")[system]["id"], "action": "click"})
time.sleep(1.0)
labels = control("Sharp")
system = next((label for label in labels if label.startswith("System")), None)
if system not in ("System (sharp)", "System (smooth)") or not is_on(labels[system]):
    die(f"after picking System the control reads {sorted(labels)}, selected "
        f"{[label for label, n in labels.items() if is_on(n)]}")
want = desktop_rendering()
if want and system != f"System ({want})":
    die(f"the desktop's hinting gives {want} text, but the segment reads {system!r}")
print(f"  STEP 5 ok: System chosen, the segment reads {system!r}"
      + (f", as this desktop's hinting gives" if want else ""))

for p in (mcp, app):
    if p.poll() is None:
        p.terminate()
print("PASS")
