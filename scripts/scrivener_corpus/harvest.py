#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Rebuild the Scrivener test corpus locally, byte for byte, from corpus.lock.json.

    python3 scripts/scrivener_corpus/harvest.py --dest ~/scrivener-corpus [--only id,id] [--skip-ll]
    SKRIBISTO_SCRIVENER_CORPUS=~/scrivener-corpus \
        cargo test -p skribisto-scrivener-import --test all -- --ignored corpus --nocapture

corpus.lock.json, beside this script, pins every source (a repository at a full commit and
the paths to check out, or a Literature and Latte download by its SHA-256) and lists, for each
project root found in it, the outcome the importer is expected to reach. It names projects; it
holds none of their files.

Nothing fetched here may enter the Skribisto repository: most sources carry no licence, and the
Literature and Latte downloads are L&L copyright. The script therefore refuses a destination inside
any git work tree, and writes a README beside the corpus saying so.

git entries: init + fetch --depth 1 --filter=blob:none of the exact commit + no-cone sparse
checkout of the pinned paths, core.autocrlf=false, then the tree digest is recomputed and must
equal the lock's. url entries: download, sha256 must match, unzip (skipping __MACOSX).
derive entries: decode a .scrivtemplate's <ProjectZipData> (base64 or hex) and unzip it."""
import argparse, base64, binascii, hashlib, io, json, os, re, shutil, subprocess, sys, time, unicodedata, urllib.request, zipfile
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

def tree_digest(base, paths, exclude=()):
    rows = []
    for p in paths:
        root = base if p == "." else os.path.join(base, p)
        for dp, dn, fn in os.walk(root):
            dn[:] = [d for d in dn if d != ".git" and not (dp == base and d in exclude)]
            for f in fn:
                full = os.path.join(dp, f)
                if os.path.islink(full) or not os.path.isfile(full):
                    continue
                rel = unicodedata.normalize("NFC", os.path.relpath(full, base).replace(os.sep, "/"))
                rows.append((rel, hashlib.sha256(open(full, "rb").read()).hexdigest()))
    rows.sort()
    h = hashlib.sha256()
    for rel, d in rows:
        h.update(rel.encode() + b"\0" + d.encode() + b"\n")
    return h.hexdigest()

def esc(p):
    return "/" + "".join("\\" + c if c in "[]*?!#\\" else c for c in p) + "/"

def git(args, cwd):
    r = subprocess.run(["git", "-c", "core.autocrlf=false", "-c", "core.quotepath=off"] + args, cwd=cwd, capture_output=True, text=True)
    if r.returncode:
        raise RuntimeError(f"git {' '.join(args)}: {r.stderr.strip()[-300:]}")
    return r.stdout

def inside_git_worktree(path):
    p = os.path.abspath(path)
    while True:
        if os.path.exists(os.path.join(p, ".git")):
            return p
        q = os.path.dirname(p)
        if q == p:
            return None
        p = q

def fetch_git(e, dest):
    d = os.path.join(dest, e["id"])
    if os.path.isdir(d) and tree_digest(d, e["paths"], e.get("exclude", ())) == e["tree_sha256"]:
        return "cached"
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    git(["init", "-q"], d)
    git(["remote", "add", "origin", e["repo"]], d)
    git(["fetch", "-q", "--depth", "1", "--filter=blob:none", "origin", e["commit"]], d)
    git(["sparse-checkout", "init", "--no-cone"], d)
    git(["sparse-checkout", "set", "--no-cone"] + (["/*"] if e["paths"] == ["."] else [esc(p) for p in e["paths"]]), d)
    git(["checkout", "-q", "FETCH_HEAD"], d)
    got = tree_digest(d, e["paths"], e.get("exclude", ()))
    if got != e["tree_sha256"]:
        raise RuntimeError(f"tree digest {got[:12]} != lock {e['tree_sha256'][:12]}")
    return "fetched"

def fetch_url(e, dest, cache):
    os.makedirs(cache, exist_ok=True)
    z = os.path.join(cache, os.path.basename(e["url"]))
    if not (os.path.exists(z) and hashlib.sha256(open(z, "rb").read()).hexdigest() == e["sha256"]):
        req = urllib.request.Request(e["url"], headers={"User-Agent": "skribisto-corpus-harvest"})
        with urllib.request.urlopen(req, timeout=120) as r:
            data = r.read()
        got = hashlib.sha256(data).hexdigest()
        if got != e["sha256"]:
            # L&L rebuild their downloads (the manual source was rebuilt 2026-04-16): say so, keep going
            return f"CHANGED UPSTREAM (sha256 {got[:12]} != lock {e['sha256'][:12]}); not unpacked"
        open(z, "wb").write(data)
    d = os.path.join(dest, e["id"])
    shutil.rmtree(d, ignore_errors=True)
    with zipfile.ZipFile(z) as zf:
        for m in zf.infolist():
            n = m.filename
            if n.startswith("__MACOSX/") or "/._" in "/" + n or os.path.isabs(n) or ".." in n.split("/"):
                continue
            zf.extract(m, d)
    return "fetched"

def derive(e, dest):
    src = os.path.join(dest, e["from"])
    tpl = None
    for dp, _, fn in os.walk(src):
        if e["input"] in fn:
            tpl = os.path.join(dp, e["input"]); break
    if tpl is None:
        return "input missing"
    txt = open(tpl, encoding="utf-8").read()
    m = re.search(r"<ProjectZipData(?: Type=\"(\w+)\")?>(.*?)</ProjectZipData>", txt, re.S)
    raw = re.sub(r"\s+", "", m.group(2))
    data = base64.b64decode(raw) if m.group(1) == "Base64" else binascii.unhexlify(raw)
    out = os.path.join(src, e["output"])
    shutil.rmtree(out, ignore_errors=True)
    zipfile.ZipFile(io.BytesIO(data)).extractall(out)
    return "derived"

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dest", required=True)
    ap.add_argument("--lock", default=os.path.join(HERE, "corpus.lock.json"))
    ap.add_argument("--only", default="")
    ap.add_argument("--skip-ll", action="store_true", help="do not download Literature and Latte material")
    a = ap.parse_args()
    wt = inside_git_worktree(a.dest)
    if wt:
        sys.exit(f"refusing: {a.dest} is inside the git work tree {wt}; the corpus must never be committable")
    os.makedirs(a.dest, exist_ok=True)
    lock = json.load(open(a.lock))
    only = set(filter(None, a.only.split(",")))
    t0 = time.time(); bad = 0
    for e in lock["entries"]:
        if only and e["id"] not in only and e.get("from") not in only:
            continue
        if a.skip_ll and e["kind"] == "url":
            continue
        t = time.time()
        try:
            st = {"git": lambda: fetch_git(e, a.dest), "url": lambda: fetch_url(e, a.dest, os.path.join(a.dest, ".downloads")),
                  "derive": lambda: derive(e, a.dest)}[e["kind"]]()
        except Exception as x:
            st = f"FAILED: {x}"; bad += 1
        print(f"{e['id']:32s} {e.get('licence', ''):12.12s} {st}  ({time.time() - t:.1f}s)", flush=True)
    open(os.path.join(a.dest, "README-NOT-FOR-REDISTRIBUTION.txt"), "w").write(
        "Scrivener projects fetched by scripts/scrivener_corpus/harvest.py for Skribisto's opt-in corpus tests.\n"
        "Most carry no licence; the Literature and Latte downloads are L&L copyright.\n"
        "Read them locally. Never copy any of these files into the Skribisto repository.\n")
    print(f"done in {time.time() - t0:.0f}s, {bad} failed")
    sys.exit(1 if bad else 0)

if __name__ == "__main__":
    main()
