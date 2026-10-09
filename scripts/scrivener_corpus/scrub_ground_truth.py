#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Turn the owner's Scrivener ground-truth handover into committed test fixtures.

The handover is a Finder "Compress" zip of a folder of projects the project owner made in
Scrivener for Mac, with their own words, to settle what Scrivener really writes. It also
carries what must never enter the repository: the Mac's name, the macOS user name, a
downloaded research PDF and macOS debris. This script writes a copy with those removed and
everything else kept byte for byte, then proves it:

    python3 scripts/scrivener_corpus/scrub_ground_truth.py \\
        SOURCE.zip crates/scrivener_import/tests/fixtures/ground-truth-mac-3.5.2

Run it from outside the repository (a scratch folder): it extracts the unscrubbed source
into a temporary folder of its own, never into the tree.

What it changes, and nothing else:

1. every manifest's `Device="..."` and every lock's `host=` become `Writers-Mac`, one
   fixed value, so a manifest and its `binder.autosave` copy stay identical where they were;
2. a lock's `project_path` becomes `/Users/writer/Desktop/skribisto-ground-truth/<name>`,
   keeping its key order, its line ends and its missing final newline;
3. `binder.autosave` and `binder.backup` are unpacked, scrubbed and packed again with the
   same member, the same compression and the same timestamps;
4. a research PDF is replaced by a generated one-page PDF with no Info dictionary and no
   XMP, its binder item is titled `Sample` and its search-index entry carries the new
   page's text;
5. an embedded picture is replaced by a generated 225 x 225 palette PNG with no text
   chunks, and only the `docs.checksum` lines that were right before are recomputed (a
   stale line stays stale: that is evidence the tests rely on);
6. every `QuickLook/` folder is left out;
7. each zip is rebuilt with the same members in the same order, the same compression per
   member, the same timestamps, uid 501 and gid 20, no comment and no `__MACOSX`;
8. a `.scrivtemplate`'s project zip is decoded, scrubbed as a zip and re-encoded on one
   line, and only that text node changes;
9. a project folder's empty folders are not written: Git keeps no empty folder, and the
   tests must read exactly what a clone holds.

The personal strings are never written down here. They are read at run time: the Mac's name
from the source's manifests, the user name from its locks, the PDF's title and author from
the PDF itself, the owner's name from `git config user.name`, and any further string from
the environment variable SKRIBISTO_SCRUB_EXTRA (one per line).

The script refuses to finish, and removes what it wrote, unless:

* no byte of the result (nested zips and the decoded template payload included), read as
  UTF-8, UTF-16LE, UTF-16BE or Latin-1, holds the Mac's name or any of its words of four
  letters or more, the user name, the owner's forename or surname in any case, the PDF's
  author, or the PDF's title where it named the file;
* `/Users/` appears only as `/Users/writer/`, and no name or byte holds `kMDItem`,
  `com.apple.`, `__MACOSX`, `.DS_Store` or `QuickLook`;
* the only device and lock host left is `Writers-Mac`, no PDF keeps an Info dictionary or
  XMP, and no PNG keeps a text chunk;
* the importer raises the same notices on the scrubbed copy as on the source, the copied
  research file's name aside (it follows the item's title). This runs the crate's
  `notice_set` example through cargo, from a scratch folder, with the repository's
  manifest.
"""

from __future__ import annotations

import argparse
import base64
import hashlib
import os
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import zipfile
import zlib
from dataclasses import dataclass, field
from pathlib import Path

DEVICE = "Writers-Mac"
USER_HOME = "/Users/writer/"
LOCK_FOLDER = "/Users/writer/Desktop/skribisto-ground-truth/"
PDF_TITLE = "Sample"
PDF_TEXT = "Sample\nA page generated for the importer's tests."
UID, GID = 501, 20
FORBIDDEN = ("kMDItem", "com.apple.", "__MACOSX", ".DS_Store", "QuickLook")
REPO = Path(__file__).resolve().parents[2]


class Refused(Exception):
    """A check failed: the result must not be committed."""


# ── Generated replacements ─────────────────────────────────────────────────────────────


def generated_pdf() -> bytes:
    """A one-page PDF with a line of text, no Info dictionary, no XMP, no document id."""
    lines = PDF_TEXT.split("\n")
    text = "BT /F1 18 Tf 72 720 Td ({}) Tj 0 -28 Td /F1 12 Tf ({}) Tj ET\n".format(
        *(line.replace("\\", "\\\\").replace("(", "\\(").replace(")", "\\)") for line in lines)
    )
    stream = text.encode("ascii")
    objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] "
        b"/Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>",
        b"<< /Length %d >>\nstream\n" % len(stream) + stream + b"endstream",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>",
    ]
    out = bytearray(b"%PDF-1.4\n")
    offsets = []
    for number, body in enumerate(objects, start=1):
        offsets.append(len(out))
        out += b"%d 0 obj\n" % number + body + b"\nendobj\n"
    xref = len(out)
    out += b"xref\n0 %d\n0000000000 65535 f \n" % (len(objects) + 1)
    for offset in offsets:
        out += b"%010d 00000 n \n" % offset
    out += b"trailer\n<< /Size %d /Root 1 0 R >>\nstartxref\n%d\n%%%%EOF\n" % (
        len(objects) + 1,
        xref,
    )
    return bytes(out)


def generated_png(size: int = 225) -> bytes:
    """A size x size 8-bit palette PNG (IHDR, PLTE, IDAT, IEND): a pattern of 64 colours."""
    palette = bytearray()
    for i in range(64):
        palette += bytes((40 + 3 * i, 90 + 2 * i, 200 - 2 * i))

    def chunk(kind: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + kind
            + data
            + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)
        )

    rows = bytearray()
    for y in range(size):
        rows.append(0)
        for x in range(size):
            ring = max(abs(x - size // 2), abs(y - size // 2)) // 8
            rows.append((ring * 5 + (x + y) // 32) % 64)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 3, 0, 0, 0))
        + chunk(b"PLTE", bytes(palette))
        + chunk(b"IDAT", zlib.compress(bytes(rows), 9))
        + chunk(b"IEND", b"")
    )


# ── Zips, byte for byte ────────────────────────────────────────────────────────────────


@dataclass
class Member:
    """One zip member with everything a rebuild must keep."""

    name: str
    method: int
    flags: int
    create_version: int
    create_system: int
    extract_version: int
    internal_attr: int
    external_attr: int
    date_time: tuple
    local_extra: bytes
    central_extra: bytes
    raw: bytes  # the compressed bytes as stored
    data: bytes  # the bytes once inflated
    crc: int
    changed: bool = False


def read_zip(blob: bytes) -> tuple[list[Member], bytes]:
    """The members of a zip, in order, with their raw compressed bytes; and its comment."""
    import io

    archive = zipfile.ZipFile(io.BytesIO(blob))
    members = []
    for info in archive.infolist():
        offset = info.header_offset
        header = struct.unpack("<IHHHHHIIIHH", blob[offset : offset + 30])
        if header[0] != 0x04034B50:
            raise Refused(f"a zip member's local header is unreadable: {info.filename}")
        name_len, extra_len = header[9], header[10]
        start = offset + 30 + name_len + extra_len
        members.append(
            Member(
                name=info.filename,
                method=info.compress_type,
                flags=info.flag_bits,
                create_version=info.create_version,
                create_system=info.create_system,
                extract_version=info.extract_version,
                internal_attr=info.internal_attr,
                external_attr=info.external_attr,
                date_time=info.date_time,
                local_extra=blob[offset + 30 + name_len : start],
                central_extra=info.extra,
                raw=blob[start : start + info.compress_size],
                data=archive.read(info.filename),
                crc=info.CRC,
            )
        )
    return members, archive.comment


def owned_by_writer(extra: bytes) -> bytes:
    """`extra` with its Info-ZIP Unix ids (0x7875) set to uid 501, gid 20."""
    out = bytearray()
    at = 0
    while at + 4 <= len(extra):
        kind, size = struct.unpack("<HH", extra[at : at + 4])
        body = extra[at + 4 : at + 4 + size]
        if kind == 0x7875:
            body = b"\x01\x04" + struct.pack("<I", UID) + b"\x04" + struct.pack("<I", GID)
        out += struct.pack("<HH", kind, len(body)) + body
        at += 4 + size
    return bytes(out)


def write_zip(members: list[Member]) -> bytes:
    """A zip of `members`, in order: unchanged members keep their stored bytes."""
    out = bytearray()
    central = bytearray()
    for m in members:
        if m.changed:
            if m.method == zipfile.ZIP_STORED:
                m.raw = m.data
            elif m.method == zipfile.ZIP_DEFLATED:
                packer = zlib.compressobj(9, zlib.DEFLATED, -15)
                m.raw = packer.compress(m.data) + packer.flush()
            else:
                raise Refused(f"a member compressed with method {m.method}: {m.name}")
            m.crc = zlib.crc32(m.data) & 0xFFFFFFFF
        if m.flags & 0x08:
            raise Refused(f"a member written with a data descriptor: {m.name}")
        year, month, day, hour, minute, second = m.date_time
        dos_time = hour << 11 | minute << 5 | second // 2
        dos_date = (year - 1980) << 9 | month << 5 | day
        name = m.name.encode("utf-8")
        local_extra = owned_by_writer(m.local_extra)
        central_extra = owned_by_writer(m.central_extra)
        offset = len(out)
        out += struct.pack(
            "<IHHHHHIIIHH",
            0x04034B50,
            m.extract_version,
            m.flags,
            m.method,
            dos_time,
            dos_date,
            m.crc,
            len(m.raw),
            len(m.data),
            len(name),
            len(local_extra),
        )
        out += name + local_extra + m.raw
        central += struct.pack(
            "<IHHHHHHIIIHHHHHII",
            0x02014B50,
            3 << 8 | (m.create_version & 0xFF),
            m.extract_version,
            m.flags,
            m.method,
            dos_time,
            dos_date,
            m.crc,
            len(m.raw),
            len(m.data),
            len(name),
            len(central_extra),
            0,
            0,
            m.internal_attr,
            m.external_attr,
            offset,
        )
        central += name + central_extra
    start = len(out)
    out += central
    out += struct.pack(
        "<IHHHHIIH", 0x06054B50, 0, 0, len(members), len(members), len(central), start, 0
    )
    return bytes(out)


# ── The scrub ──────────────────────────────────────────────────────────────────────────


@dataclass
class Personal:
    """What must not survive, read from the source at run time."""

    devices: set[str] = field(default_factory=set)
    users: set[str] = field(default_factory=set)
    pdf_titles: set[str] = field(default_factory=set)
    pdf_authors: set[str] = field(default_factory=set)
    owner: list[str] = field(default_factory=list)
    extra: list[str] = field(default_factory=list)


DEVICE_RE = re.compile(rb'Device="([^"]*)"')
HOST_RE = re.compile(rb"^host=(.*)$", re.M)
PATH_RE = re.compile(rb"^project_path=/Users/([^/\n]*)/[^\n]*$", re.M)


def pdf_info(pdf: bytes) -> dict[str, str]:
    """A PDF's literal Info strings by key (`Title`, `Author`, ...)."""
    found: dict[str, str] = {}
    for key in ("Title", "Author", "Subject", "Keywords", "Creator", "Producer"):
        literal = re.search(rb"/" + key.encode() + rb"\s*\(((?:\\.|[^\\)])*)\)", pdf)
        hexed = re.search(rb"/" + key.encode() + rb"\s*<([0-9A-Fa-f\s]*)>", pdf)
        raw = None
        if literal:
            raw = re.sub(rb"\\(.)", rb"\1", literal.group(1))
        elif hexed:
            raw = bytes.fromhex(re.sub(rb"\s", b"", hexed.group(1)).decode())
        if raw is None:
            continue
        if raw.startswith(b"\xfe\xff"):
            found[key] = raw[2:].decode("utf-16-be", "replace")
        else:
            found[key] = raw.decode("latin-1")
    return found


class Scrubber:
    def __init__(self, personal: Personal) -> None:
        self.personal = personal
        self.png = generated_png()
        self.pdf = generated_pdf()
        self.pictures_replaced = 0
        self.pdfs_replaced = 0

    # Reading what is personal, before anything changes.
    def learn(self, path: str, data: bytes) -> None:
        base = path.rsplit("/", 1)[-1]
        if base.endswith(".scrivx"):
            for device in DEVICE_RE.findall(data):
                self.personal.devices.add(device.decode("utf-8"))
        elif base == "user.lock":
            for host in HOST_RE.findall(data):
                if host:
                    self.personal.devices.add(host.decode("utf-8"))
            for user in PATH_RE.findall(data):
                self.personal.users.add(user.decode("utf-8"))
        elif base in ("binder.autosave", "binder.backup"):
            members, _ = read_zip(data)
            for m in members:
                self.learn(m.name, m.data)
        elif base.endswith(".pdf"):
            info = pdf_info(data)
            if info.get("Title"):
                self.personal.pdf_titles.add(info["Title"])
            if info.get("Author"):
                self.personal.pdf_authors.add(info["Author"])
        elif base.endswith(".zip"):
            members, _ = read_zip(data)
            for m in members:
                self.learn(m.name, m.data)
        elif base.endswith(".scrivtemplate"):
            payload = template_payload(data)
            self.learn("payload.zip", base64.b64decode(payload[2]))

    # A project's files, keyed by their path inside the folder or the zip.
    def project(self, files: dict[str, bytes], name: str) -> dict[str, bytes]:
        """The scrubbed files of the project folder `name`, keyed by their paths inside the
        folder (or, in an archive, inside the archive)."""
        pdf_items = {
            path.rsplit("/", 2)[-2]
            for path in files
            if path.endswith("/content.pdf") and "/Files/Data/" in "/" + path
        }
        before = {path: hashlib.sha1(data).hexdigest() for path, data in files.items()}
        out: dict[str, bytes] = {}
        for path, data in files.items():
            if "QuickLook/" in path or path.endswith("QuickLook"):
                continue
            base = path.rsplit("/", 1)[-1]
            if base.endswith(".scrivx"):
                data = self.manifest(data, pdf_items)
            elif base in ("binder.autosave", "binder.backup"):
                data = self.nested(data, pdf_items)
            elif base == "user.lock":
                data = self.lock(data, name)
            elif base == "search.indexes":
                data = self.index(data, pdf_items)
            elif base == "content.pdf":
                data = self.pdf
                self.pdfs_replaced += 1
            elif base.endswith(".pdf"):
                raise Refused(f"a research file this script does not know how to replace: {path}")
            elif base.endswith(".rtf") and b"\\pngblip" in data:
                data = self.pictures(data)
            out[path] = data
        for path, data in list(out.items()):
            if path.endswith("docs.checksum"):
                out[path] = self.checksums(path, data, files, before, out)
        return out

    def manifest(self, data: bytes, pdf_items: set[str], live: bool = True) -> bytes:
        """A manifest scrubbed. A spare copy (`binder.autosave`, `binder.backup`) may predate
        the research file, or be the stub of a new project, so only the live one must hold
        it."""
        data = DEVICE_RE.sub(b'Device="' + DEVICE.encode() + b'"', data)
        for uuid in pdf_items:
            data, count = re.subn(
                rb'(<BinderItem UUID="' + re.escape(uuid.encode()) + rb'"[^>]*>\s*<Title>)[^<]*(</Title>)',
                rb"\g<1>" + PDF_TITLE.encode() + rb"\g<2>",
                data,
            )
            if count > 1 or (live and count != 1):
                raise Refused("a research PDF's binder item was not found in a manifest")
        return data

    def nested(self, data: bytes, pdf_items: set[str]) -> bytes:
        members, comment = read_zip(data)
        if comment:
            raise Refused("a binder copy carries a zip comment")
        for m in members:
            if m.name.endswith(".scrivx"):
                new = self.manifest(m.data, pdf_items, live=False)
                if new != m.data:
                    m.data, m.changed = new, True
        return write_zip(members)

    def lock(self, data: bytes, project: str) -> bytes:
        data = HOST_RE.sub(b"host=" + DEVICE.encode(), data)
        data, count = PATH_RE.subn(
            b"project_path=" + (LOCK_FOLDER + project).encode(), data
        )
        if count != 1:
            raise Refused("a lock file without exactly one project_path")
        return data

    def index(self, data: bytes, pdf_items: set[str]) -> bytes:
        for uuid in pdf_items:
            data, count = re.subn(
                rb'(<Document ID="' + re.escape(uuid.encode()) + rb'">\s*<Title>)[^<]*(</Title>\s*<Text>)[^<]*(</Text>)',
                rb"\g<1>" + PDF_TITLE.encode() + rb"\g<2>" + PDF_TEXT.encode() + rb"\g<3>",
                data,
            )
            if count != 1:
                raise Refused("a research PDF's search-index entry was not found")
        return data

    def pictures(self, data: bytes) -> bytes:
        def swap(match: re.Match) -> bytes:
            self.pictures_replaced += 1
            # Scrivener writes the picture as one line of lower-case hex after a space.
            return match.group(1) + self.png.hex().encode()

        new, count = re.subn(rb"(\\pngblip\s)[0-9a-fA-F\s]+(?=\})", swap, data)
        if count != data.count(b"\\pngblip"):
            raise Refused("an embedded picture in a form this script does not know")
        return new

    def checksums(
        self,
        path: str,
        data: bytes,
        before_files: dict[str, bytes],
        before: dict[str, str],
        after: dict[str, bytes],
    ) -> bytes:
        folder = path.rsplit("/", 1)[0]
        by_upper = {p.upper(): p for p in before_files}
        lines = data.split(b"\n")
        for i, line in enumerate(lines):
            if b"=" not in line:
                continue
            key, digest = line.split(b"=", 1)
            target = by_upper.get((folder + "/" + key.decode("utf-8")).upper())
            if target is None or target not in after:
                continue
            if before[target] == digest.decode("ascii").strip().lower():
                fresh = hashlib.sha1(after[target]).hexdigest().encode()
                lines[i] = key + b"=" + fresh
        return b"\n".join(lines)

    def zip(self, data: bytes) -> bytes:
        members, comment = read_zip(data)
        if comment:
            raise Refused("an archive carries a zip comment")
        kept = [m for m in members if not m.name.startswith("__MACOSX/")]
        kept = [m for m in kept if not m.name.endswith(".DS_Store")]
        files = {m.name: m.data for m in kept if not m.name.endswith("/")}
        wrappers = {name.split("/", 1)[0] for name in files if "/" in name}
        if len(wrappers) != 1 or not next(iter(wrappers)).endswith(".scriv"):
            raise Refused("an archive that does not wrap exactly one project folder")
        scrubbed = self.project(files, next(iter(wrappers)))
        out = []
        for m in kept:
            if m.name.endswith("/"):
                if "QuickLook/" in m.name:
                    continue
                out.append(m)
                continue
            if m.name not in scrubbed:
                continue
            if scrubbed[m.name] != m.data:
                m.data, m.changed = scrubbed[m.name], True
            out.append(m)
        return write_zip(out)

    def template(self, data: bytes) -> bytes:
        start, end, payload = template_payload(data)
        inner = self.zip(base64.b64decode(payload))
        return data[:start] + base64.b64encode(inner) + data[end:]


def template_payload(data: bytes) -> tuple[int, int, bytes]:
    match = re.search(rb'<ProjectZipData Type="Base64">([^<]*)</ProjectZipData>', data)
    if match is None:
        raise Refused("a template without a Base64 project zip")
    return match.start(1), match.end(1), match.group(1)


# ── The checks ─────────────────────────────────────────────────────────────────────────


def views(blob: bytes) -> list[str]:
    """`blob` read every way a string could hide in it, lower-cased."""
    texts = [blob.decode("utf-8", "replace"), blob.decode("latin-1")]
    for codec in ("utf-16-le", "utf-16-be"):
        texts.append(blob.decode(codec, "replace"))
        texts.append(blob[1:].decode(codec, "replace"))
    return [t.lower() for t in texts]


def expand(name: str, blob: bytes) -> list[tuple[str, bytes]]:
    """`blob` and everything inside it: zip members (nested zips included) and a template's
    decoded project zip."""
    found = [(name, blob)]
    if blob[:4] == b"PK\x03\x04":
        members, comment = read_zip(blob)
        if comment:
            found.append((name + " (comment)", comment))
        for m in members:
            found.append((name + "!" + m.name + " (name)", m.name.encode()))
            found += expand(name + "!" + m.name, m.data)
    elif b"<ScrivenerTemplate" in blob[:512]:
        found += expand(name + "!payload", base64.b64decode(template_payload(blob)[2]))
    return found


def check(dest: Path, personal: Personal) -> list[str]:
    """Every reason the result at `dest` must not be committed."""
    needles: dict[str, str] = {}
    for device in personal.devices:
        needles[device.lower()] = "the Mac's name"
        for word in re.split(r"[^0-9A-Za-z]+", device):
            if len(word) >= 4:
                needles[word.lower()] = "a word of the Mac's name"
    for user in personal.users:
        needles[user.lower()] = "the macOS user name"
    for word in personal.owner:
        needles[word.lower()] = "the owner's name"
    for author in personal.pdf_authors:
        needles[author.lower()] = "the PDF's author"
    for word in personal.extra:
        needles[word.lower()] = "an extra string"
    for forbidden in FORBIDDEN:
        needles[forbidden.lower()] = forbidden
    needles.pop(DEVICE.lower(), None)
    needles.pop("", None)

    problems = []
    blobs = []
    for path in sorted(dest.rglob("*")):
        rel = path.relative_to(dest).as_posix()
        if any(f.lower() in rel.lower() for f in FORBIDDEN):
            problems.append(f"{rel}: a forbidden name")
        if path.is_file():
            blobs += expand(rel, path.read_bytes())
    for name, blob in blobs:
        texts = views(blob)
        for needle, what in needles.items():
            if any(needle in text for text in texts):
                problems.append(f"{name}: {what}")
        for text in texts[:2]:
            for home in re.findall(r"/users/[^/\s\"<]*/?", text):
                if home != USER_HOME.lower():
                    problems.append(f"{name}: a home folder other than {USER_HOME}")
        for device in DEVICE_RE.findall(blob):
            if device.decode("utf-8", "replace") != DEVICE:
                problems.append(f"{name}: a device other than {DEVICE}")
        for host in HOST_RE.findall(blob):
            if host.decode("utf-8", "replace") not in (DEVICE, ""):
                problems.append(f"{name}: a lock host other than {DEVICE}")
        if blob.startswith(b"%PDF"):
            if b"/Info" in blob or b"/Metadata" in blob or b"xmpmeta" in blob:
                problems.append(f"{name}: a PDF with an Info dictionary or XMP")
            for title in personal.pdf_titles:
                if title.encode("latin-1", "replace") in blob:
                    problems.append(f"{name}: the PDF's original title")
        for hexed in re.findall(rb"\\pngblip\s([0-9a-fA-F]+)", blob):
            picture = bytes.fromhex(hexed.decode())
            if any(kind in picture for kind in (b"tEXt", b"iTXt", b"zTXt", b"eXIf")):
                problems.append(f"{name}: a picture with a text chunk")
        # The PDF's title, where it named the file: its binder item and its index entry.
        for title in personal.pdf_titles:
            for match in re.finditer(rb"<Title>([^<]*)</Title>", blob):
                if match.group(1).decode("utf-8", "replace") == title and title != PDF_TITLE:
                    problems.append(f"{name}: the PDF's original title as an item title")
    return sorted(set(problems))


def notice_sets(paths: list[Path]) -> dict[str, list[str]]:
    """The notices the importer raises on each of `paths`, by file name."""
    with tempfile.TemporaryDirectory() as scratch:
        # From a scratch folder, so a local `.cargo/config.toml` in the tree never applies.
        result = subprocess.run(
            [
                "cargo",
                "run",
                "--quiet",
                "--manifest-path",
                str(REPO / "Cargo.toml"),
                "-p",
                "skribisto-scrivener-import",
                "--example",
                "notice_set",
                "--",
                *map(str, paths),
            ],
            cwd=scratch,
            capture_output=True,
            text=True,
        )
    if result.returncode != 0:
        raise Refused("the notice_set example failed:\n" + result.stderr[-4000:])
    sets: dict[str, list[str]] = {}
    current = None
    for line in result.stdout.splitlines():
        if line.startswith("== "):
            current = Path(line[3:]).name
            sets[current] = []
        elif current is not None:
            # The copied research file is named after its item, which the scrub retitles.
            line = re.sub(r'(MediaFileCopied \{.*path: )"[^"]*"', r'\1"<research file>"', line)
            sets[current].append(line)
    return {name: sorted(lines) for name, lines in sets.items()}


# ── Driving it ─────────────────────────────────────────────────────────────────────────


def read_source(path: Path) -> dict[str, bytes]:
    """The handover's files below its one top folder, without macOS debris."""
    with zipfile.ZipFile(path) as archive:
        files = {}
        for info in archive.infolist():
            name = info.filename
            if name.startswith("__MACOSX/") or info.is_dir():
                continue
            top, _, rest = name.partition("/")
            if not rest or rest.endswith(".DS_Store"):
                continue
            files[rest] = archive.read(info)
    return files


def owner_name() -> list[str]:
    result = subprocess.run(
        ["git", "config", "user.name"], cwd=REPO, capture_output=True, text=True
    )
    return [word for word in result.stdout.split() if len(word) >= 3]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("source", type=Path, help="the handover zip, outside the repository")
    parser.add_argument("dest", type=Path, help="the fixture folder to write")
    args = parser.parse_args()

    source = read_source(args.source)
    personal = Personal(owner=owner_name())
    personal.extra = [
        s.strip() for s in os.environ.get("SKRIBISTO_SCRUB_EXTRA", "").splitlines() if s.strip()
    ]
    scrubber = Scrubber(personal)
    for path, data in source.items():
        scrubber.learn(path, data)
    if not personal.devices or not personal.users:
        print("refused: the source names no device or no user, so nothing can be checked")
        return 1

    tops = sorted({path.split("/", 1)[0] for path in source})
    out: dict[str, bytes] = {}
    for top in tops:
        if top.endswith(".scriv"):
            files = {p[len(top) + 1 :]: d for p, d in source.items() if p.startswith(top + "/")}
            for rel, data in scrubber.project(files, top).items():
                out[top + "/" + rel] = data
        elif top.endswith(".zip"):
            out[top] = scrubber.zip(source[top])
        elif top.endswith(".scrivtemplate"):
            out[top] = scrubber.template(source[top])
        else:
            print(f"refused: an entry this script does not know: {top}")
            return 1

    dest = args.dest
    notice = dest / "NOTICE"
    kept_notice = notice.read_bytes() if notice.exists() else None
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    for rel, data in out.items():
        target = dest / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    if kept_notice is not None:
        notice.write_bytes(kept_notice)

    problems = check(dest, personal)
    with tempfile.TemporaryDirectory() as unscrubbed:
        root = Path(unscrubbed)
        with zipfile.ZipFile(args.source) as archive:
            for info in archive.infolist():
                top, _, rest = info.filename.partition("/")
                if info.filename.startswith("__MACOSX/") or not rest:
                    continue
                target = root / rest
                if info.is_dir():
                    target.mkdir(parents=True, exist_ok=True)
                else:
                    target.parent.mkdir(parents=True, exist_ok=True)
                    target.write_bytes(archive.read(info))
        try:
            before = notice_sets([root / top for top in tops])
            after = notice_sets([dest / top for top in tops])
        except Refused as e:
            problems.append(str(e))
            before, after = {}, {}
    for name in sorted(set(before) | set(after)):
        if before.get(name) != after.get(name):
            gone = sorted(set(before.get(name, [])) - set(after.get(name, [])))
            new = sorted(set(after.get(name, [])) - set(before.get(name, [])))
            problems.append(f"{name}: the importer's notices changed: lost {gone}, gained {new}")

    if problems:
        shutil.rmtree(dest)
        if kept_notice is not None:
            dest.mkdir(parents=True)
            notice.write_bytes(kept_notice)
        print("refused: the result is not fit to commit, and was removed")
        for problem in problems:
            print("  " + problem)
        return 1

    size = sum(p.stat().st_size for p in dest.rglob("*") if p.is_file())
    count = sum(1 for p in dest.rglob("*") if p.is_file())
    print(f"wrote {count} files, {size} bytes, to {dest}")
    print(
        f"  pictures replaced: {scrubber.pictures_replaced}, "
        f"research PDFs replaced: {scrubber.pdfs_replaced}"
    )
    print(
        f"  checked for {len(personal.devices)} device name(s), {len(personal.users)} user "
        f"name(s), {len(personal.owner)} word(s) of the owner's name, "
        f"{len(personal.pdf_authors)} PDF author(s), {len(personal.pdf_titles)} PDF title(s)"
    )
    for name in sorted(after):
        print(f"  {name}: {len(after[name])} notices, the same as the source")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Refused as e:
        print(f"refused: {e}")
        sys.exit(1)
