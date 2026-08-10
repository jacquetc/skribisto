#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Regenerate the container fixtures beside this script.

Run from anywhere:  python3 crates/document_ingest/tests/fixtures/generate.py

Why real files rather than XML built in the test body: hand-written XML cannot
reproduce what producers actually emit — Word splitting one styled word across
three runs, LibreOffice writing a point-anchored comment as a bare
`w:commentReference` with no range at all, pandoc rendering a thematic break as an
empty paragraph carrying nothing but a bottom border. Every one of those is a real
behaviour this importer had to be taught, and none would have been noticed against
a fixture we wrote ourselves.

The unit tests in `sources/odt.rs` and `sources/docx.rs` still build their
containers inline: those pin *rules*, and a rule is clearer next to its test than
in a binary. These pin *reality*.

  word-shaped.docx  Office Open XML as Word writes it, authored here: `w:outlineLvl`
                    headings, a ranged comment with a threaded reply through
                    `w15:commentsEx`, a resolved point comment, tracked changes, and
                    an unanchored comment of its own carrying bold/italic emphasis
                    across two paragraphs (M-S4: a comment body is Djot, not plain
                    text). Not produced by Word (which is not available on this
                    machine) — the wire format is Word's, the authorship is ours,
                    and this file says so rather than implying otherwise.
  libreoffice.odt   LibreOffice's own writer, from the flat ODF below. Carries an
                    `office:annotation` thread via `loext:parent-name`, a resolved
                    comment, a ranged comment closed by `office:annotation-end`, and
                    a second point comment on the same paragraph carrying bold/
                    italic emphasis across two paragraphs (M-S4).
  pandoc.odt        pandoc, from the Markdown below.
  pandoc.docx       pandoc, from the same Markdown — so the two formats can be
                    asserted to produce the *same* prose from the same source.
  roundtrip.odt     LibreOffice's own writer, from `ROUNDTRIP_FODT` below: the
                    returning-file case. Carries Skribisto's two bookmark marks
                    (`skrb_r…` on a row, a `skrb_c…` pair around a comment range)
                    *and* a `skrb:uid` attribute on the annotation — so a test can
                    pin both halves of what an editor's save does: the bookmarks
                    survive, the private attribute does not.
  roundtrip.docx    the same source through LibreOffice's OOXML writer, pinning the
                    same two facts on the Word path — where `skrb:uid` does not even
                    reach `comments.xml`.

Needs `soffice` and `pandoc` on PATH for all but the first.
"""

import pathlib
import shutil
import subprocess
import sys
import tempfile
import zipfile

HERE = pathlib.Path(__file__).resolve().parent

W = 'xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"'
W14 = 'xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"'
W15 = 'xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml"'
MC = ('xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" '
      'mc:Ignorable="w14 w15"')

MARKDOWN = """\
# The Salt Road

## Chapter One

She turned the corner and the street was *gone*. In its place, **nothing** — and
a smell of ~~rain~~ salt.

* * *

Later, she would say she had known. She had not.

- one
- two

## Chapter Two

The second chapter opens quietly.
"""


def word_shaped_docx(path):
    """Office Open XML in the shape Word writes it."""
    document = f'''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document {W} {W14} {W15} {MC}><w:body>
<w:p><w:pPr><w:pStyle w:val="Heading1"/><w:outlineLvl w:val="0"/></w:pPr>
  <w:r><w:t>The Salt Road</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="Heading2"/><w:outlineLvl w:val="1"/></w:pPr>
  <w:r><w:t>Chapter One</w:t></w:r></w:p>
<w:p>
  <w:r><w:t xml:space="preserve">She turned the corner and </w:t></w:r>
  <w:commentRangeStart w:id="1"/>
  <w:r><w:rPr><w:i/></w:rPr><w:t>the st</w:t></w:r>
  <w:r><w:rPr><w:i/></w:rPr><w:t>reet was g</w:t></w:r>
  <w:r><w:rPr><w:i/></w:rPr><w:t>one</w:t></w:r>
  <w:commentRangeEnd w:id="1"/>
  <w:r><w:commentReference w:id="1"/></w:r>
  <w:r><w:t>. In its place, </w:t></w:r>
  <w:r><w:rPr><w:b/></w:rPr><w:t>nothing</w:t></w:r>
  <w:r><w:t>.</w:t></w:r>
</w:p>
<w:p>
  <w:r><w:t xml:space="preserve">Later, she </w:t></w:r>
  <w:ins w:id="20" w:author="Editor" w:date="2026-01-05T10:00:00Z">
    <w:r><w:t xml:space="preserve">would say she </w:t></w:r></w:ins>
  <w:del w:id="21" w:author="Editor" w:date="2026-01-05T10:00:00Z">
    <w:r><w:delText xml:space="preserve">always claimed she </w:delText></w:r></w:del>
  <w:r><w:t>had known.</w:t></w:r>
</w:p>
<w:p><w:pPr><w:pStyle w:val="Heading2"/><w:outlineLvl w:val="1"/></w:pPr>
  <w:r><w:t>Chapter Two</w:t></w:r></w:p>
<w:p>
  <w:r><w:t>The second chapter opens quietly.</w:t></w:r>
  <w:r><w:commentReference w:id="3"/></w:r>
</w:p>
</w:body></w:document>'''

    comments = f'''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:comments {W} {W14} {W15} {MC}>
<w:comment w:id="1" w:author="Editor" w:date="2026-01-02T03:04:05Z" w:initials="E">
  <w:p w14:paraId="AAAA0001"><w:r><w:t>Is this the right word?</w:t></w:r></w:p></w:comment>
<w:comment w:id="2" w:author="Writer" w:date="2026-01-03T03:04:05Z" w:initials="W">
  <w:p w14:paraId="AAAA0002"><w:r><w:t>Yes, I meant it.</w:t></w:r></w:p></w:comment>
<w:comment w:id="3" w:author="Editor" w:date="2026-01-04T03:04:05Z" w:initials="E">
  <w:p w14:paraId="AAAA0003"><w:r><w:t>A whole-paragraph note.</w:t></w:r></w:p></w:comment>
<w:comment w:id="4" w:author="Editor" w:date="2026-01-06T03:04:05Z" w:initials="E">
  <w:p w14:paraId="AAAA0004"><w:r><w:t xml:space="preserve">This needs </w:t></w:r><w:r><w:rPr><w:b/></w:rPr><w:t>real</w:t></w:r><w:r><w:t xml:space="preserve"> emphasis, and </w:t></w:r><w:r><w:rPr><w:i/></w:rPr><w:t>italics</w:t></w:r><w:r><w:t xml:space="preserve"> too.</w:t></w:r></w:p>
  <w:p w14:paraId="AAAA0006"><w:r><w:t>A second paragraph in the same note.</w:t></w:r></w:p></w:comment>
</w:comments>'''

    extended = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w15:commentsEx xmlns:w15="http://schemas.microsoft.com/office/word/2012/wordml">
<w15:commentEx w15:paraId="AAAA0001" w15:done="0"/>
<w15:commentEx w15:paraId="AAAA0002" w15:paraIdParent="AAAA0001" w15:done="0"/>
<w15:commentEx w15:paraId="AAAA0003" w15:done="1"/>
</w15:commentsEx>'''

    styles = f'''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles {W}>
<w:style w:type="paragraph" w:styleId="Normal"><w:name w:val="Normal"/></w:style>
<w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/>
  <w:basedOn w:val="Normal"/><w:pPr><w:outlineLvl w:val="0"/></w:pPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/>
  <w:basedOn w:val="Normal"/><w:pPr><w:outlineLvl w:val="1"/></w:pPr></w:style>
</w:styles>'''

    content_types = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
<Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
<Override PartName="/word/comments.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"/>
<Override PartName="/word/commentsExtended.xml" ContentType="application/vnd.ms-word.commentsExtended+xml"/>
</Types>'''

    rels = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>'''

    document_rels = '''<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" Target="comments.xml"/>
<Relationship Id="rId2" Type="http://schemas.microsoft.com/office/2011/relationships/commentsExtended" Target="commentsExtended.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
</Relationships>'''

    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("[Content_Types].xml", content_types)
        z.writestr("_rels/.rels", rels)
        z.writestr("word/_rels/document.xml.rels", document_rels)
        z.writestr("word/document.xml", document)
        z.writestr("word/styles.xml", styles)
        z.writestr("word/comments.xml", comments)
        z.writestr("word/commentsExtended.xml", extended)


FLAT_ODT = '''<?xml version="1.0" encoding="UTF-8"?>
<office:document xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
 xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
 xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
 xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0"
 xmlns:dc="http://purl.org/dc/elements/1.1/"
 xmlns:loext="urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0"
 office:version="1.3" office:mimetype="application/vnd.oasis.opendocument.text">
 <office:automatic-styles>
  <style:style style:name="Em" style:family="text">
   <style:text-properties fo:font-style="italic"/></style:style>
  <style:style style:name="Strong" style:family="text">
   <style:text-properties fo:font-weight="bold"/></style:style>
 </office:automatic-styles>
 <office:body><office:text>
  <text:h text:outline-level="1">The Salt Road</text:h>
  <text:h text:outline-level="2">Chapter One</text:h>
  <text:p>She turned the corner and <office:annotation office:name="c1"><dc:creator>Editor</dc:creator><dc:date>2026-01-02T03:04:05</dc:date><text:p>Is this the right word?</text:p></office:annotation><text:span text:style-name="Em">the street was gone</text:span><office:annotation-end office:name="c1"/><office:annotation office:name="c2" loext:parent-name="c1"><dc:creator>Writer</dc:creator><dc:date>2026-01-03T03:04:05</dc:date><text:p>Yes, I meant it.</text:p></office:annotation>. In its place, <text:span text:style-name="Strong">nothing</text:span>.</text:p>
  <text:h text:outline-level="2">Chapter Two</text:h>
  <text:p>The second chapter opens quietly.<office:annotation office:name="c3" loext:resolved="true"><dc:creator>Editor</dc:creator><dc:date>2026-01-04T03:04:05</dc:date><text:p>A whole-paragraph note.</text:p></office:annotation><office:annotation office:name="c4"><dc:creator>Editor</dc:creator><dc:date>2026-01-06T03:04:05</dc:date><text:p>This needs <text:span text:style-name="Strong">real</text:span> emphasis, and <text:span text:style-name="Em">italics</text:span> too.</text:p><text:p>A second paragraph in the same note.</text:p></office:annotation></text:p>
 </office:text></office:body>
</office:document>
'''


# The returning file: what an editor hands back after opening an export and saving.
#
# Two carriers are deliberately present on the same annotation, because the whole
# point of this fixture is that they do not fare alike:
#
#   * `skrb:uid` — a private-namespace attribute. It is the identity carrier the
#     first comment-export milestone relied on, and LibreOffice **deletes it on
#     save**, along with the namespace declaration. Verified against a real
#     returning file (LibreOffice 25.8.5.2) before this fixture existed.
#   * `skrb_c…` — a bookmark pair around the same range. Bookmarks are first-class
#     in both ODF and OOXML, so both writers preserve them, name and position.
#
# `skrb_r…` marks the row. Its name packs a 64-bit `BinderItem.uid` prefix and a
# 48-bit digest of the row's normalised text at export time; 35 characters, chosen
# to stay inside Word's 40-character cap on bookmark names.
#
# The reply's leading "Répondre à …" paragraph is not decoration: it is the citation
# block LibreOffice's own Reply button writes into the body, transcribed from a real
# returning file. It has to be stripped on import or it accumulates in the writer's
# thread on every round trip.
ROUNDTRIP_FODT = '''<?xml version="1.0" encoding="UTF-8"?>
<office:document xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0"
 xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"
 xmlns:style="urn:oasis:names:tc:opendocument:xmlns:style:1.0"
 xmlns:fo="urn:oasis:names:tc:opendocument:xmlns:xsl-fo-compatible:1.0"
 xmlns:dc="http://purl.org/dc/elements/1.1/"
 xmlns:loext="urn:org:documentfoundation:names:experimental:office:xmlns:loext:1.0"
 xmlns:skrb="urn:ferntech:text-document:comment:1"
 office:version="1.3" office:mimetype="application/vnd.oasis.opendocument.text">
 <office:body><office:text>
  <text:h text:outline-level="1">The Salt Road</text:h>
  <text:h text:outline-level="2">Chapter One</text:h>
  <text:p><text:bookmark text:name="skrb_r0000000000000001_aaaaaaaaaaaa"/>She turned the corner and <text:bookmark-start text:name="skrb_c000000000000c001"/><office:annotation office:name="c1" skrb:uid="6f1d4c9e-0b2a-4d7f-9c31-5a8e2b6f0d44"><dc:creator>Editor</dc:creator><dc:date>2026-01-02T03:04:05</dc:date><text:p>Is this the right word?</text:p></office:annotation>the street was gone<office:annotation-end office:name="c1"/><text:bookmark-end text:name="skrb_c000000000000c001"/><office:annotation office:name="c2" loext:parent-name="c1" skrb:uid="1a7b93d0-55e4-42c6-8f10-c3d9a4e70b28"><dc:creator>Writer</dc:creator><dc:date>2026-01-03T03:04:05</dc:date><text:p text:style-name="Comment">Répondre à <text:s/>(02/01/2026, 03:04): &quot;Is this the right word?&quot;</text:p><text:p text:style-name="Comment">Yes, I meant it.</text:p></office:annotation>. In its place, nothing.</text:p>
  <text:h text:outline-level="2">Chapter Two</text:h>
  <text:p><text:bookmark text:name="skrb_r0000000000000002_bbbbbbbbbbbb"/>The second chapter opens quietly.</text:p>
 </office:text></office:body>
</office:document>
'''


def convert(source_text, source_name, out_name, tool):
    """Write `source_text`, run `tool` on it, and move the result beside this file."""
    with tempfile.TemporaryDirectory() as tmp:
        tmp = pathlib.Path(tmp)
        src = tmp / source_name
        src.write_text(source_text, encoding="utf-8")
        target = HERE / out_name
        if tool == "soffice":
            subprocess.run(
                ["soffice", "--headless", "--norestore",
                 "--convert-to", target.suffix.lstrip("."), str(src),
                 "--outdir", str(tmp)],
                check=True, capture_output=True,
            )
            produced = tmp / (src.stem + target.suffix)
        else:
            produced = tmp / out_name
            subprocess.run(["pandoc", "-s", str(src), "-o", str(produced)],
                           check=True, capture_output=True)
        shutil.move(str(produced), str(target))
        print("wrote", target.name)


def main():
    word_shaped_docx(HERE / "word-shaped.docx")
    print("wrote word-shaped.docx")

    missing = [t for t in ("soffice", "pandoc") if shutil.which(t) is None]
    if missing:
        print(f"skipping {', '.join(missing)}-produced fixtures: not on PATH",
              file=sys.stderr)
        return 1 if missing else 0

    convert(FLAT_ODT, "source.fodt", "libreoffice.odt", "soffice")
    convert(MARKDOWN, "source.md", "pandoc.odt", "pandoc")
    convert(MARKDOWN, "source.md", "pandoc.docx", "pandoc")
    convert(ROUNDTRIP_FODT, "roundtrip.fodt", "roundtrip.odt", "soffice")
    convert(ROUNDTRIP_FODT, "roundtrip.fodt", "roundtrip.docx", "soffice")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
