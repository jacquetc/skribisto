#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Drive a live Skribisto and verify the **status-bar focused word count**.

The word count is the quiet "how many words in this scene" a writer glances at,
sitting just after the save glyph. It counts the *live* document, so it tracks
typing.

Flow: load a scratch copy of an example →
  1. open a scene → a "N words" count appears in the status bar
  2. type prose   → the count goes UP by (roughly) the words typed

Reuses the launch + scrape-socket/token + connect scaffolding from the sibling
automation_*.py scripts.
"""
import base64, json, os, re, select, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import automation_fixture as fixture  # noqa: E402

SKRIBISTO = fixture.skribisto_binary()
MCP = fixture.mcp_binary()
EXAMPLE = fixture.repo_path("resources/examples/starforgers/Starforgers.skrib")

mcp_err = tempfile.NamedTemporaryFile(suffix=".mcperr", delete=False).name


def fail(msg, app=None, mcp=None, log=None):
    print("FAIL:", msg)
    if log:
        print("--- app log tail ---")
        print("\n".join(open(log).read().splitlines()[-25:]))
    for p in (app, mcp):
        if p and p.poll() is None:
            p.terminate()
    sys.exit(1)


class Session:
    def __init__(self, args):
        self.log = tempfile.NamedTemporaryFile(suffix=".log", delete=False).name
        self.app = subprocess.Popen([SKRIBISTO, *args], stdout=open(self.log, "w"),
                                    stderr=subprocess.STDOUT)
        sock = tok = None
        deadline = time.time() + 20
        while time.time() < deadline:
            txt = open(self.log).read()
            s = re.search(r"bridge socket = (\S+)", txt)
            t = re.search(r"TEKSILO_AUTOMATION_TOKEN=(\S+)", txt)
            if s and t:
                sock, tok = s.group(1), t.group(1)
                break
            if self.app.poll() is not None:
                fail("app exited before printing the bridge socket", self.app, None, self.log)
            time.sleep(0.2)
        if not sock:
            fail("no bridge socket within 20s", self.app, None, self.log)
        self._id = 0
        while not os.path.exists(sock) and time.time() < deadline:
            time.sleep(0.05)
        self.mcp = subprocess.Popen([MCP, "--connect", sock, "--token", tok],
                                    stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                    stderr=open(mcp_err, "w"), text=True, bufsize=1)
        self._send("initialize", {"protocolVersion": "2024-11-05", "capabilities": {},
                                  "clientInfo": {"name": "word-count-test", "version": "1"}})
        if self._recv(timeout=15, fatal=False) is None:
            fail("could not connect MCP", self.app, self.mcp, self.log)
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

    def _recv(self, timeout=20, fatal=True):
        end = time.time() + timeout
        while time.time() < end:
            if self.mcp.poll() is not None:
                break
            r, _, _ = select.select([self.mcp.stdout], [], [], max(0.0, end - time.time()))
            if not r:
                break
            line = self.mcp.stdout.readline()
            if not line:
                break
            if line.strip():
                return json.loads(line)
        if fatal:
            fail("no MCP response within timeout", self.app, self.mcp, self.log)
        return None

    def call(self, name, args=None):
        self._send("tools/call", {"name": name, "arguments": args or {}})
        result = self._recv().get("result", {})
        payload = result.get("structuredContent")
        if payload is None:
            text = "".join(c.get("text", "") for c in result.get("content", [])
                           if c.get("type") == "text")
            payload = json.loads(text) if text.strip().startswith("{") else {}
        return result, payload

    def nodes(self):
        _, p = self.call("snapshot_tree")
        return p.get("nodes", [])

    def wait_label(self, substr, timeout=30):
        end = time.time() + timeout
        while time.time() < end:
            joined = " | ".join(n.get("label") or "" for n in self.nodes()).lower()
            if substr.lower() in joined:
                return True
            time.sleep(0.4)
        return False

    def shot(self, path):
        res, _ = self.call("screenshot")
        for c in res.get("content", []):
            if c.get("type") == "image":
                open(path, "wb").write(base64.b64decode(c["data"]))
                print(f"  screenshot → {path}")

    def close(self):
        for p in (self.mcp, self.app):
            if p and p.poll() is None:
                p.terminate()


WORDS_RE = re.compile(r"(\d[\d, ]*)\s+words?", re.IGNORECASE)


def all_widgets(s):
    """The full layout tree (includes static text the a11y tree prunes)."""
    _, p = s.call("layout_tree")
    return p.get("nodes", [])


def word_count(s, timeout=10):
    """The status-bar word count's number, or None. Matched on its text
    ('N words') across the a11y AND the full layout tree (a TextWidget's text may
    live under `value`/`text`/`label`, and static text is pruned from the AT tree)."""
    end = time.time() + timeout
    while time.time() < end:
        for n in s.nodes() + all_widgets(s):
            for field in ("label", "value", "text", "resolved_text", "params"):
                blob = n.get(field)
                m = WORDS_RE.search(blob if isinstance(blob, str) else json.dumps(blob or ""))
                if m:
                    return int(re.sub(r"[^\d]", "", m.group(1))), n
        time.sleep(0.4)
    return None, None


def click(s, n):
    if "click" in (n.get("actions") or []):
        s.call("invoke_action", {"node": n["id"], "action": "click"})
    b = n.get("bounds") or {}
    if "x" in b:
        s.call("inject_pointer", {"x": b["x"] + b.get("width", 0) / 2,
                                  "y": b["y"] + b.get("height", 0) / 2, "action": "click"})


print("== launch with a scratch copy of the Starforgers example ==")
scratch = tempfile.mkdtemp(prefix="skribisto-wordcount-")
project = os.path.join(scratch, "Starforgers.skrib")
shutil.copy2(EXAMPLE, project)
s = Session([project])
if not s.wait_label("starforgers", timeout=30):
    fail("the example work did not load", s.app, s.mcp, s.log)
print(f"  loaded {project}")

# ── 1. Open a scene → the word count appears ─────────────────────────────────
print("== opening a scene shows a word count ==")
row = next((n for n in s.nodes() if (n.get("label") or "").strip() == "Prologue"), None)
if not row:
    fail("no 'Prologue' row in the binder", s.app, s.mcp, s.log)
editors = []
for _ in range(3):
    click(s, row)
    end = time.time() + 10
    while time.time() < end and not editors:
        editors = [n for n in s.nodes() if n.get("role") == "MultilineTextInput"
                   and "set_value" in (n.get("actions") or [])]
        if not editors:
            time.sleep(0.5)
    if editors:
        break
if not editors:
    fail("no writing editor after opening Prologue", s.app, s.mcp, s.log)
main = max(editors, key=lambda n: (n.get("bounds") or {}).get("height", 0))

count, node = word_count(s)
if count is None:
    s.shot("/tmp/wordcount-missing.png")
    fail("no word count in the status bar after opening a scene", s.app, s.mcp, s.log)
print(f"  word count: {node.get('label')!r}  (= {count})")
s.shot("/tmp/wordcount-initial.png")

# ── 2. Typing raises the count ───────────────────────────────────────────────
print("== typing raises the count ==")
added = "one two three four five six seven eight nine ten "  # ten words
s.call("type_text", {"node": main["id"], "text": added})
time.sleep(1.2)
count2, node2 = word_count(s)
if count2 is None:
    fail("the word count vanished after typing", s.app, s.mcp, s.log)
print(f"  word count after typing: {node2.get('label')!r}  (= {count2})")
s.shot("/tmp/wordcount-typed.png")
if count2 <= count:
    fail(f"the count did not rise after typing 10 words ({count} -> {count2}) — it is "
         "not tracking the live document", s.app, s.mcp, s.log)
if count2 - count < 8:
    fail(f"the count rose by only {count2 - count} after typing 10 words — suspicious",
         s.app, s.mcp, s.log)

print(f"\nPASS: word count appears and tracks typing ({count} -> {count2}, +{count2 - count}).")
s.close()
