#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Does the scene editor actually lay Arabic out right-to-left?

Everything about the RTL work was verified headlessly — shaping, rule L2
reordering, alignment, caret motion. This drives the *real* app: opens a
project whose language is Arabic, types Arabic prose into a chapter's scene
editor, and reads the resulting geometry back out of the accessibility tree.

The check that matters is not "did the text appear" but *where*: an RTL
paragraph has to sit against the right edge of the writing column, not the
left. A pure-Arabic paragraph carries no stored `fmt_direction`, so getting
that right depends entirely on the bidi algorithm auto-detecting it.
"""
import json, os, re, subprocess, sys, tempfile, time, base64

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
PROJECT = os.path.expanduser("~/test_ar.skrib")

# "كتب الرجل رسالة طويلة" — "the man wrote a long letter".
# Three paragraphs, typed into a fresh scene:
#   1. pure Arabic          -> must auto-detect RTL and sit flush right
#   2. French with an Arabic phrase inside  -> LTR paragraph, Arabic reordered
#   3. Arabic with a French phrase inside   -> RTL paragraph, French reordered
PARAS = [
    "كتب الرجل رسالة طويلة",
    "Il a dit كتب الرجل puis il est parti",
    "قال الرجل bonjour tout le monde ثم غادر",
]


def die(msg, log=None):
    print("FAIL:", msg)
    if log:
        print("\n".join(open(log).read().splitlines()[-25:]))
    sys.exit(1)


sandbox = tempfile.mkdtemp(prefix="skribisto_rtl_")
env = {**os.environ, "XDG_CONFIG_HOME": os.path.join(sandbox, "config"),
       "XDG_DATA_HOME": os.path.join(sandbox, "data"), "HOME": sandbox}
work = os.path.join(sandbox, "test_ar.skrib")
import shutil
shutil.copyfile(PROJECT, work)

log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
app = subprocess.Popen([SKRIBISTO, work], stdout=open(log, "w"),
                       stderr=subprocess.STDOUT, env=env)
sock = tok = None
end = time.time() + 30
while time.time() < end:
    t = open(log).read()
    s = re.search(r"bridge socket = (\S+)", t)
    k = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", t)
    if s and k:
        sock, tok = s.group(1), k.group(1)
        break
    if app.poll() is not None:
        die("app exited before announcing the bridge", log)
    time.sleep(0.2)
if not sock:
    die("no bridge socket", log)
time.sleep(1.5)

mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok], stdin=subprocess.PIPE,
                       stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True, bufsize=1)
_id = 0


def call(name, args=None):
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
            r = m.get("result", {})
            p = r.get("structuredContent")
            if p is None:
                txt = "".join(c.get("text", "") for c in r.get("content", [])
                              if c.get("type") == "text").strip()
                p = json.loads(txt) if txt.startswith(("{", "[")) else {}
            return p


mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": 0, "method": "initialize",
                            "params": {"protocolVersion": "2024-11-05", "capabilities": {},
                                       "clientInfo": {"name": "rtl", "version": "1"}}}) + "\n")
mcp.stdin.flush()
mcp.stdout.readline()
mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
mcp.stdin.flush()


def nodes():
    return call("snapshot_tree").get("nodes", [])


def shot(path):
    r = call("screenshot")
    for key in ("data", "image", "png", "base64"):
        if isinstance(r.get(key), str):
            open(path, "wb").write(base64.b64decode(r[key]))
            return path
    return None


# Wait for the binder to populate, then open a chapter so an editor exists.
end = time.time() + 30
while time.time() < end:
    if len([n for n in nodes() if n.get("role") in ("TreeItem", "ListBoxOption")]) >= 3:
        break
    time.sleep(0.5)

ns = nodes()
rows = [n for n in ns if n.get("role") == "TreeItem"]
print(f"binder rows: {len(rows)}")
chapter = next((n for n in rows if "Chapter 1" in str(n.get("name") or n.get("label") or "")), None)
if chapter is None:
    chapter = rows[2] if len(rows) > 2 else (rows[0] if rows else None)
if chapter is None:
    die("no binder rows", log)
print(f"opening {str(chapter.get('name') or chapter.get('label'))!r}")

b = chapter.get("bounds") or {}
call("inject_pointer", {"x": b.get("x", 0) + b.get("width", 10) / 2,
                        "y": b.get("y", 0) + b.get("height", 10) / 2, "kind": "click"})
time.sleep(0.6)
call("inject_pointer", {"x": b.get("x", 0) + b.get("width", 10) / 2,
                        "y": b.get("y", 0) + b.get("height", 10) / 2, "kind": "double_click"})
time.sleep(2.0)


def editors():
    return [n for n in nodes()
            if n.get("role") == "MultilineTextInput"
            and "set_value" in (n.get("actions") or [])]


end = time.time() + 20
eds = []
while time.time() < end:
    eds = editors()
    if eds:
        break
    time.sleep(0.5)
if not eds:
    shot("/tmp/ar-fail.png")
    die(f"no editor opened; roles: {sorted({n.get('role') for n in nodes()})}", log)

print(f"editors: {len(eds)}")
for n in eds:
    print(f"  id={n.get('id')} bounds={n.get('bounds')}")


def area(b):
    if isinstance(b, dict):
        return float(b.get("width", 0)) * float(b.get("height", 0))
    return 0.0


target = max(eds, key=lambda n: area(n.get("bounds") or {}))
tbounds = target.get("bounds")
print(f"\ntyping into id={target.get('id')} bounds={tbounds}")

call("focus_node", {"node": target["id"]})
time.sleep(0.5)
for i, para in enumerate(PARAS):
    call("type_text", {"node": target["id"], "text": para})
    time.sleep(1.0)
    if i < len(PARAS) - 1:
        call("inject_key", {"key": "Return"})
        time.sleep(0.5)
time.sleep(1.5)
shot("/tmp/ar-typed.png")

after = {n.get("id"): n for n in nodes()}
tgt = after.get(target.get("id"), {})
print("value after typing:", repr(str(tgt.get("value"))[:80]))

# Geometry. A RichTextEditor keeps its prose in per-block CHILD nodes, so read
# their bounds: an RTL paragraph must sit against the RIGHT edge of the editor.
by_id = {n["id"]: n for n in nodes() if "id" in n}


def descendants(nid, depth=0):
    n = by_id.get(nid)
    if not n or depth > 6:
        return []
    out = [(depth, n)]
    for c in n.get("children") or []:
        out.extend(descendants(c, depth + 1))
    return out


print("\neditor subtree:")
kids = descendants(target["id"])
for d, n in kids:
    txt = n.get("value") or n.get("label") or n.get("name")
    print(f"  {'  '*d}role={n.get('role')} bounds={n.get('bounds')} text={str(txt)[:44]!r}")

ed_b = tbounds or {}
ed_left, ed_right = ed_b.get("x", 0), ed_b.get("x", 0) + ed_b.get("width", 0)
inner = [n for d, n in kids if d > 0 and n.get("bounds") and (n.get("value") or n.get("name") or n.get("label"))]
print(f"\neditor spans x {ed_left} .. {ed_right}")
verdict = "UNKNOWN"
for n in inner:
    b = n["bounds"]
    l, r = b.get("x", 0), b.get("x", 0) + b.get("width", 0)
    gap_l, gap_r = l - ed_left, ed_right - r
    print(f"  block x={l:.1f}..{r:.1f}  left-gap={gap_l:.1f} right-gap={gap_r:.1f}")
    if gap_r < gap_l - 5:
        verdict = "RTL (flush right)"
    elif gap_l < gap_r - 5:
        verdict = "LTR (flush left)"
print("verdict from a11y geometry:", verdict)

res = call("screenshot")
print("screenshot keys:", list(res.keys())[:8])
for key in ("data", "image", "png", "base64", "content"):
    v = res.get(key)
    if isinstance(v, str) and len(v) > 500:
        open("/tmp/ar-typed.png", "wb").write(base64.b64decode(v))
        print("wrote /tmp/ar-typed.png")
        break

json.dump({"editor_bounds": tbounds, "verdict": verdict,
           "blocks": [{"bounds": n.get("bounds"),
                       "text": str(n.get("value") or n.get("name") or n.get("label"))}
                      for n in inner]},
          open("/tmp/ar-probe.json", "w"), ensure_ascii=False, indent=2)

print("\nleaving the app running for a screenshot; pid", app.pid)
if mcp.poll() is None:
    mcp.terminate()
