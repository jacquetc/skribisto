#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Distraction-free mode paints the prose in the THEME's ink, not the app palette's.

A distraction-free theme names an ink (`editor_text`), and
`distraction_free::theme::apply_to` writes it onto the surface's theme tokens.
The prose editor sets no colour of its own, so teksilo paints it with the
theme's `editor_fg` role. If `apply_to` leaves that role alone, the prose keeps
the app palette's editor ink and the theme's ink never reaches it.

Midnight is used because its ink (#f2f2f2) is far from the dark palette's
editor ink, so the two cannot be confused in a screenshot:

  1. with the synopsis's Text color set to Normal, open Prologue, press F11;
  2. screenshot the window and crop it to the prose editor, then to the
     synopsis beside it;
  3. the ink in each must be the theme's, within a small tolerance for
     antialiasing. A Normal synopsis paints with the same `editor_fg` role.

Pixels are read inside the editor's own bounds because the surface's chrome
(the strip, the Exit button) paints with other roles that already follow the
theme, and would pass this check whatever the prose does.
"""

import base64, collections, io, os, sys, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

try:
    from PIL import Image
except ImportError:
    fixture.skip("needs Pillow to read the screenshot")

THEME = "midnight"
INK = (0xF2, 0xF2, 0xF2)
TOLERANCE = 12

EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")
PROJECT = fixture.working_copy(EXAMPLE, label="df-ink")
PINS_MAP = {"ui.locale": "en-US", "ui.dark": False, "ui.show_welcome": False,
            "editor.distraction_free.theme": THEME, "editor.synopsis.text_color": "Normal"}
ENV = fixture.isolated_config(locale="en-US", label="df-ink", show_welcome=False,
                              pins=PINS_MAP)
PINS = fixture.config_pins_file(PINS_MAP, label="df-ink")

log = os.path.join(fixture.SCRATCH, f"df-ink-{os.getpid()}.log")
app = fixture.subprocess.Popen(fixture.launch_argv(PROJECT, pins=PINS),
                               stdout=open(log, "w"), stderr=fixture.subprocess.STDOUT,
                               env=ENV)
session = None


def done(code, msg):
    print(msg)
    if code:
        print("--- app log tail ---")
        print("\n".join(open(log, errors="replace").read().splitlines()[-20:]))
    if session:
        session.close()
    if app.poll() is None:
        app.terminate()
    sys.exit(code)


try:
    bridge = fixture.wait_for_bridge(log, app, timeout=60)
    session = fixture.McpSession(bridge, "df-ink")
except (RuntimeError, fixture.McpError) as e:
    done(1, f"FAIL: {e}")


def editors():
    return [n for n in session.nodes() if n.get("role") == "MultilineTextInput"
            and (n.get("bounds") or {}).get("height")]


end = time.time() + 40
row = None
while time.time() < end and not row:
    row = next((n for n in session.nodes()
                if (n.get("label") or "").strip().lower() == "prologue"
                and (n.get("bounds") or {}).get("width")), None)
    if not row:
        time.sleep(0.5)
if not row:
    done(1, "FAIL: no 'Prologue' row in the binder")
b = row["bounds"]
session.call("inject_pointer", {"x": b["x"] + b["width"] / 2, "y": b["y"] + b["height"] / 2,
                                "action": "click"})
time.sleep(1.5)
if not editors():
    done(1, "FAIL: no editor after opening Prologue")

session.call("inject_key", {"key": "F11"})
time.sleep(2.0)
session.settle()
found = editors()
prose = max(found, key=lambda n: n["bounds"]["height"])
others = [n for n in found if n is not prose]
if not others:
    done(1, "FAIL: no synopsis editor beside the prose in distraction-free mode")
synopsis = min(others, key=lambda n: n["bounds"]["x"])

result = session.request("tools/call", {"name": "screenshot", "arguments": {}})
data = next((c["data"] for c in result.get("content", []) if c.get("type") == "image"), None)
if not data:
    done(1, "FAIL: the bridge returned no screenshot")
img = Image.open(io.BytesIO(base64.b64decode(data))).convert("RGB")
shot = os.path.join(fixture.SCRATCH, f"df-ink-{os.getpid()}.png")
img.save(shot)

# Node bounds are logical pixels; the screenshot is physical.
root_w = max((n.get("bounds") or {}).get("width", 0) for n in session.nodes())
scale = img.width / root_w if root_w else 1.0


def hexc(c):
    return "#%02x%02x%02x" % c


def ink_of(node):
    """The most common colour well above the page inside `node`'s bounds.

    Not the brightest pixel: the caret is a few dozen pixels of its own colour,
    and the glyphs' antialiased edges are many shades between page and ink.
    """
    b = node["bounds"]
    box = tuple(int(v * scale) for v in (b["x"], b["y"], b["x"] + b["width"],
                                         b["y"] + b["height"]))
    counts = collections.Counter(img.crop(box).getdata())
    page = counts.most_common(1)[0][0]
    brightest = max(counts, key=sum)
    threshold = (sum(page) + sum(brightest)) / 2
    return next(p for p, _ in counts.most_common() if sum(p) > threshold)


failures = []
for name, node in (("prose", prose), ("synopsis (Text color: Normal)", synopsis)):
    got = ink_of(node)
    ok = all(abs(a - b) <= TOLERANCE for a, b in zip(got, INK))
    print(f"{'OK  ' if ok else 'FAIL'} {name}: {hexc(got)} (theme ink {hexc(INK)})")
    if not ok:
        failures.append(name)
print(f"screenshot: {shot}")
if failures:
    done(1, f"FAIL: {', '.join(failures)} not painted in the {THEME} theme's ink: "
            "`apply_to` does not reach the editor's ink role")
done(0, f"PASS: the prose and a Normal synopsis paint in the {THEME} theme's ink")
