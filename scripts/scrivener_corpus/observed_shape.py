#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Census the shape of real Scrivener projects: structure only, never content.

    python3 scripts/scrivener_corpus/observed_shape.py ~/scrivener-corpus \
        crates/scrivener_import/tests/fixtures/ground-truth-mac-3.5.2 \
        > crates/scrivener_fixtures/observed_shape.json

Every folder named is walked; the census is the union. The second is the project owner's own
ground truth, which the repository holds (UQ-48).

Writes, per generation and platform, every XML element path and attribute name the manifests
and Scrivener's own sidecar files use, the shape of every file path inside a project (UUIDs,
numbers, snapshot dates, extensions and user-chosen names masked), and the keys of the lock
file. No title, no text, no value is kept: the output is the vocabulary of the format, which is
what the fixture generator (crates/scrivener_fixtures) is checked against so it cannot invent a
format detail no real project holds.

Element paths collapse nesting: a BinderItem inside a BinderItem is recorded as one level, and
the same for keywords, so a binder of any depth gives the same paths."""
import json, os, re, sys, xml.etree.ElementTree as ET

UUID = re.compile(r"[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}")
STAMP = re.compile(r"^\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2}[+-]\d{4}\.rtf$")
RECURSIVE = ("BinderItem", "Keyword")


def collapse(stack):
    out = list(stack)
    for rec in RECURSIVE:
        if rec in out:
            first = out.index(rec)
            last = len(out) - 1 - out[::-1].index(rec)
            if first != last:
                out = out[:first] + out[last:]
    return "/".join(out)


def xml_paths(path, depth_limit=None):
    """Element paths and attribute paths of one XML file, or None when it does not parse."""
    found = set()
    stack = []
    try:
        for ev, el in ET.iterparse(path, events=("start", "end")):
            if ev == "start":
                stack.append(el.tag)
                if depth_limit and len(stack) > depth_limit:
                    continue
                key = collapse(stack)
                found.add(key)
                for attr in el.attrib:
                    found.add(key + "/@" + attr)
            else:
                stack.pop()
                el.clear()
    except (ET.ParseError, OSError, UnicodeDecodeError):
        return None
    return found


SAFE = re.compile(r"^[A-Za-z0-9._<>+~#\[\]-]{1,40}$")


def masked(part):
    """A name a writer or a machine chose (a title, a person's or a computer's name)
    becomes <NAME>, keeping its extension; Scrivener's own names are kept."""
    if SAFE.match(part):
        return part
    stem, ext = os.path.splitext(part)
    return "<NAME>" + (ext if SAFE.match(ext or "x") else "")


def shape(rel):
    """The shape of a path inside a project."""
    parts = rel.split("/")
    out = []
    for i, part in enumerate(parts):
        top = parts[0]
        p = UUID.sub("<UUID>", part)
        if i == len(parts) - 1 and STAMP.match(part):
            p = "<STAMP>.rtf"
        elif i == 0 and part.lower().endswith(".scrivx"):
            p = "<STEM>.scrivx"
        elif top == "Icons" and i > 0:
            p = "<NAME>"
        elif top == "Settings" and len(parts) > 2 and parts[1] == "Compile Formats":
            p = "<NAME>" + os.path.splitext(part)[1] if i == len(parts) - 1 else p
        else:
            p = re.sub(r"^\d+", "<N>", p)
            p = re.sub(r"^Note-\d+", "Note-<N>", p)
            # a media extension is whatever the item's file was
            m = re.match(r"^(content|card-image|<N>|<N>_icImage)\.(\w+)$", p)
            if m and m.group(2).lower() not in ("rtf", "comments", "styles", "links"):
                p = m.group(1) + ".<EXT>"
            p = masked(p)
        out.append(p)
    return "/".join(out)


def platform_of(creator):
    m = re.match(r"SCR(MAC|WIN|LIN|IOS)", creator or "")
    return {"MAC": "mac", "WIN": "win", "LIN": "lin", "IOS": "ios"}.get(m.group(1)) if m else "unknown"


def header(path):
    try:
        head = open(path, "rb").read(65536).decode("utf-8", "replace")
    except OSError:
        return {}
    m = re.search(r"<ScrivenerProject\b([^>]*)>", head)
    if not m:
        return {}
    return dict(re.findall(r'([A-Za-z]+)="([^"]*)"', m.group(1)))


SIDE = {
    "content.comments": lambda d, f: f == "content.comments",
    "N.comments": lambda d, f: re.fullmatch(r"\d+\.comments", f),
    "N.links": lambda d, f: re.fullmatch(r"\d+\.links", f),
    "ProjectNotes/index.xml": lambda d, f: f == "index.xml" and d.endswith("/ProjectNotes"),
    "snapshots/index.xml": lambda d, f: f == "index.xml" and d.endswith(".snapshots"),
    "snapshot.indexes": lambda d, f: f == "snapshot.indexes",
    "search.indexes": lambda d, f: f == "search.indexes",
    "templateinfo.xml": lambda d, f: f == "templateinfo.xml",
    "compile.xml": lambda d, f: f == "compile.xml" and d.endswith("/Settings"),
    "styles.xml": lambda d, f: f == "styles.xml" and d.endswith("/Files"),
    "writing.history": lambda d, f: f == "writing.history",
}


def walk_all(bases):
    for base in bases:
        yield from os.walk(base)


def main():
    bases = sys.argv[1:]
    if not bases:
        sys.exit("usage: observed_shape.py FOLDER [FOLDER...]")
    manifests, sidecars, files, lock_keys, templates = {}, {}, {}, set(), set()
    for dp, dn, fn in walk_all(bases):
        dn[:] = sorted(d for d in dn if d not in (".git", "__MACOSX"))
        names = set(fn)
        scrivx = sorted(f for f in fn if f.lower().endswith(".scrivx") and not f.startswith("._"))
        strong = "binder.scrivproj" in names or (scrivx and any(os.path.isdir(os.path.join(dp, d)) for d in ("Files", "Mobile")))
        for f in fn:
            if f.lower().endswith(".scrivtemplate"):
                found = xml_paths(os.path.join(dp, f), depth_limit=2)
                if found:
                    templates |= found
        if not strong:
            continue
        # a project root: never descend further on its behalf
        if "binder.scrivproj" in names and not scrivx:
            generation, platform = "scrivener1", "mac"
        else:
            h = header(os.path.join(dp, scrivx[0]))
            if h.get("MobileStub") == "Yes":
                generation, platform = "ios", "ios"
            else:
                generation, platform = h.get("Version", "unknown"), platform_of(h.get("Creator"))
        key = f"{generation}/{platform}"
        for sdp, sdn, sfn in os.walk(dp):
            sdn[:] = [d for d in sdn if d not in (".git", "__MACOSX")]
            for f in sfn:
                full = os.path.join(sdp, f)
                rel = os.path.relpath(full, dp).replace(os.sep, "/")
                if os.path.islink(full):
                    continue
                files.setdefault(generation, set()).add(shape(rel))
                if rel.lower().endswith(".scrivx") and "/" not in rel:
                    found = xml_paths(full)
                    if found:
                        manifests.setdefault(key, set()).update(found)
                elif rel.endswith("binder.mob"):
                    found = xml_paths(full)
                    if found:
                        manifests.setdefault("binder.mob", set()).update(found)
                elif f == "user.lock":
                    for line in open(full, "rb").read().decode("utf-8", "replace").splitlines():
                        if "=" in line:
                            lock_keys.add(line.split("=", 1)[0].strip())
                for kind, test in SIDE.items():
                    if test(sdp, f):
                        found = xml_paths(full)
                        if found:
                            sidecars.setdefault(kind, {}).setdefault(generation, set()).update(found)
        dn[:] = []
    out = {
        "version": 1,
        "note": "Structure only: element and attribute names, path shapes, lock keys. No content.",
        "manifests": {k: sorted(v) for k, v in sorted(manifests.items())},
        "sidecars": {k: {g: sorted(p) for g, p in sorted(v.items())} for k, v in sorted(sidecars.items())},
        "files": {k: sorted(v) for k, v in sorted(files.items())},
        "lock_keys": sorted(lock_keys),
        "template": sorted(templates),
    }
    json.dump(out, sys.stdout, indent=1, ensure_ascii=False)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
