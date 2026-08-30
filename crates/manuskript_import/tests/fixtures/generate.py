#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""Build the Manuskript fixture projects this crate's tests read.

Run from the repository root:

    python3 crates/manuskript_import/tests/fixtures/generate.py

## Why a generator and not a downloaded project

Manuskript is GPL-3-or-later and FernTech owns none of it, so no file of theirs
enters this tree: `skribisto-pro` links these crates as a proprietary path
dependency, and that only works because everything here is FernTech's to
relicense. The same rule that keeps their sample project out of `tests/fixtures`
keeps their code out of `src/`.

So the fixture is written here instead, in the shape Manuskript's own writer
produces, from content this repository already ships: the chapter titles and
editorial synopses of the bundled *Tour du monde en quatre-vingts jours* example.
Jules Verne's prose is public domain worldwide and the editorial text is the
Skribisto project's own. See the NOTICE beside this script.

The generated projects are committed, so a checkout can run the tests without
Python. Re-run this only when the fixture needs to change.

## What it produces

Three projects, covering the three ways a Manuskript project reaches a reader,
plus the awkward cases a real one accumulates:

  tour-du-monde.msk + tour-du-monde/   the modern default: a one-byte pointer
                                       file beside the project folder
  tour-du-monde-zip.msk                the single-file mode, a plain zip
  tour-du-monde-windows.msk            the same zip written with backslash member
                                       names, as Manuskript on Windows does
  tour-du-monde-2016.msk               format 0: one zip of XML, as Manuskript
                                       0.1.0 and 0.2.0 wrote it, including an
                                       `html` scene and the `summarySentance`
                                       misspelling of the day

The folder project deliberately contains one directory with no `folder.txt` and
one `.md` with no `ID:`. Manuskript itself skips the first with its whole subtree
and drops the second without a word; this importer keeps both and says so, and
that behaviour needs a fixture to be tested against.
"""

import os
import pathlib
import sys
import tomllib
import zipfile

ROOT = pathlib.Path(__file__).resolve().parents[4]
SPEC = (
    ROOT
    / "resources/examples/le_tour_du_monde_en_80_jours"
    / "le-tour-du-monde-en-quatre-vingts-jours.toml"
)
OUT = pathlib.Path(__file__).resolve().parent

# How many chapters to carry over. Enough to exercise ordering past nine, where a
# zero-padded prefix starts mattering, and small enough to stay readable in a diff.
CHAPTERS = 12
# Two parts, so the depth ladder has both rungs to find.
PART_SPLIT = 6


def slugify(name: str) -> str:
    """Manuskript's own `slugify`: ASCII letters and digits survive, whitespace
    becomes `_`, and everything else becomes `-`. Lossy on purpose and not
    reversible, which is why a title is only ever read from the metadata."""
    valid = set("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    out = []
    for c in name:
        if c in valid:
            out.append(c)
        elif c.isspace():
            out.append("_")
        else:
            out.append("-")
    return "".join(out)


def meta(name: str, value: str, width: int = 15) -> str:
    """`formatMetaData`: the value padded to a column, continuation lines indented
    past it, and a colon in the key escaped to `_.._`."""
    if "\n" in value:
        pad = " " * (width + 1)
        value = "\n".join(pad + line for line in value.split("\n"))[width + 1 :]
    name = name.replace(":", "_.._")
    return "{}:{}{}\n".format(name, " " * (width - len(name)), value)


def item_file(pairs, body: str = "") -> str:
    """A `.md` or `folder.txt`: the header, two blank lines, then the body."""
    header = "".join(meta(k, v) for k, v in pairs if v not in (None, ""))
    return header + "\n\n" + body


def numbered(index: int, total: int, title: str, suffix: str = "") -> str:
    """`{row index padded to the sibling count}-{slug}{ext}`."""
    return "{}-{}{}".format(str(index).zfill(len(str(total))), slugify(title), suffix)


def build(spec) -> dict:
    """The whole project, as `{member path: text}`."""
    files = {}
    book = spec["book"]
    chapters = spec["chapter"][:CHAPTERS]
    characters = spec["character"][:6]
    places = spec["place"][:5]

    files["MANUSKRIPT"] = "1"

    files["infos.txt"] = "".join(
        meta(k, v)
        for k, v in (
            ("Title", book["title"]),
            ("Subtitle", "Les Voyages extraordinaires"),
            # Manuskript's own spelling, which has never been corrected.
            ("Serie", "Les Voyages extraordinaires"),
            ("Volume", "11"),
            ("Genre", "Roman d’aventures"),
            ("License", "Domaine public"),
            ("Author", book["author"]),
            ("Email", ""),
        )
        if v
    )

    files["summary.txt"] = "".join(
        meta(k, v, 12)
        for k, v in (
            ("Situation", "Un pari tenu pour impossible."),
            ("Sentence", book["synopsis"].split(".")[0] + "."),
            ("Paragraph", book["synopsis"]),
            ("Page", ""),
            ("Full", ""),
        )
        if v
    )

    # The label list keeps Manuskript's empty first row out of the file: it
    # re-adds one on load, which is what makes an item's stored index 1-based.
    labels = [
        ("Idée", "#ffff00"),
        ("Note", "#00ff00"),
        ("Chapitre", "#0000ff"),
        ("Scène", "#ff0000"),
        ("Documentation", "#00ffff"),
    ]
    files["labels.txt"] = "".join(
        "{}:{}{}\n".format(name, " " * (20 - len(name)), color) for name, color in labels
    )
    statuses = [s["name"] for s in spec["status"]]
    files["status.txt"] = "".join(name + "\n" for name in statuses)

    # ── Characters ────────────────────────────────────────────────────────
    for index, character in enumerate(characters):
        pairs = [
            ("Name", character["name"]),
            ("ID", str(index)),
            ("Importance", "2" if index == 0 else "1" if index < 3 else "0"),
            ("POV", "True" if index < 3 else "False"),
            ("Motivation", "Tenir le pari." if index == 0 else ""),
            ("Goal", ""),
            ("Conflict", ""),
            ("Epiphany", ""),
            ("Phrase Summary", character["note"].split(".")[0] + "."),
            ("Paragraph Summary", character["note"]),
            ("Full Summary", ""),
            ("Notes", "Aussi appelé : " + ", ".join(character.get("aliases", []))),
        ]
        body = "".join(meta(k, v, 20) for k, v in pairs if v)
        # The swatch comes before the writer's own fields, which is what makes
        # first-wins the right rule for reading it back.
        body += meta("Color", ["#ff0000", "#00ff00", "#0000ff", "#c0392b", "#8e44ad", "#16a085"][index], 20)
        if index == 0:
            body += meta("Ville d’origine", "Londres", 20)
            # A second Color: a field the writer added, not the swatch.
            body += meta("Color", "yeux gris", 20)
        files["characters/{}.txt".format(numbered(index, len(characters), character["name"]))] = body

    # ── World ─────────────────────────────────────────────────────────────
    world_rows = "".join(
        '      <outline name="{}" ID="{}" description="{}"/>\n'.format(
            xml_attr(place["name"]), 10 + n, xml_attr(place["note"])
        )
        for n, place in enumerate(places)
    )
    files["world.opml"] = (
        "<?xml version='1.0' encoding='UTF-8'?>\n"
        '<opml version="1.0">\n  <body>\n'
        '    <outline name="Lieux" ID="1" passion="Le tour du monde tient dans ces noms.">\n'
        + world_rows
        + "    </outline>\n"
        '    <outline name="Objets" ID="2"/>\n'
        "  </body>\n</opml>\n"
    )

    # ── Plots ─────────────────────────────────────────────────────────────
    files["plots.xml"] = (
        "<?xml version='1.0' encoding='UTF-8'?>\n<root>\n"
        '  <plot name="Le pari" ID="0" importance="2" characters="0,1"'
        ' description="Fogg parie vingt mille livres." result="Il gagne d’un jour.">\n'
        '    <step name="Le pari est tenu" ID="0" meta="ouverture" summary="Au Reform-Club."/>\n'
        '    <step name="Le jour gagné" ID="1" meta="dénouement" summary="La ligne de changement de date."/>\n'
        "  </plot>\n"
        '  <plot name="La poursuite de Fix" ID="1" importance="1" characters="2"/>\n'
        "</root>\n"
    )

    files["settings.txt"] = (
        '{"saveToZip": false, "defaultTextType": "md", "dict": "fr_FR",'
        ' "revisions": {"keep": true}}'
    )

    # ── The outline ───────────────────────────────────────────────────────
    parts = [
        ("Première partie", chapters[:PART_SPLIT]),
        ("Seconde partie", chapters[PART_SPLIT:]),
    ]
    revisions = []
    next_id = 100
    for part_index, (part_title, part_chapters) in enumerate(parts):
        part_dir = "outline/" + numbered(part_index, len(parts), part_title)
        files[part_dir + "/folder.txt"] = item_file(
            [
                ("title", part_title),
                ("ID", str(next_id)),
                ("type", "folder"),
                ("compile", "2"),
                ("summaryFull", "Le voyage, moitié par moitié."),
            ]
        )
        next_id += 1
        for chapter_index, chapter in enumerate(part_chapters):
            chapter_dir = "{}/{}".format(
                part_dir, numbered(chapter_index, len(part_chapters), chapter["title"][:40])
            )
            chapter_id = next_id
            next_id += 1
            files[chapter_dir + "/folder.txt"] = item_file(
                [
                    ("title", "Chapitre {}".format(chapter["number"])),
                    ("ID", str(chapter_id)),
                    ("type", "folder"),
                    ("compile", "2"),
                    ("setGoal", "2000"),
                    ("label", "3"),
                    ("status", str(1 + (chapter_index % len(statuses)))),
                    ("summarySentence", chapter["title"]),
                    ("summaryFull", chapter["synopsis"]),
                    ("POV", "0"),
                ]
            )
            scene_id = next_id
            next_id += 1
            # A reference marker, the way Manuskript stores an inline mention.
            body = (
                "{}\n\nOn y retrouve {{C:0:%s}}, et le récit passe par {{W:10:%s}}."
                % (characters[0]["name"], places[0]["name"])
            ).format(chapter["synopsis"])
            files[chapter_dir + "/0-" + slugify(chapter["title"][:30]) + ".md"] = item_file(
                [
                    ("title", chapter["title"][:60]),
                    ("ID", str(scene_id)),
                    ("type", "md"),
                    ("compile", "2" if chapter_index % 5 else "0"),
                    ("label", "4"),
                    ("status", str(1 + (chapter_index % len(statuses)))),
                    ("notes", "{P:0:Le pari} traverse ce chapitre."),
                    ("summarySentence", chapter["title"][:60]),
                ],
                body,
            )
            revisions.append((scene_id, 1_455_000_000 + scene_id * 86_400, body[:120]))

    # A directory with no `folder.txt`. Manuskript skips it and everything under
    # it; this importer keeps the scenes and says what it did.
    orphan = "outline/{}/0-Fragment.md".format(numbered(2, 3, "Chutes"))
    files[orphan] = item_file(
        [("title", "Un fragment"), ("ID", str(next_id)), ("type", "md")],
        "Une page mise de côté.",
    )
    next_id += 1
    # A `.md` with no ID at all. Manuskript drops it without a word.
    nameless = "Une page sans identité"
    files[
        "outline/{}/{}".format(
            numbered(2, 3, "Chutes"), numbered(1, 2, nameless, ".md")
        )
    ] = item_file([("title", nameless), ("type", "md")], "Elle arrive quand même.")

    files["revisions.xml"] = build_revisions(files, revisions)
    return files


def xml_attr(text: str) -> str:
    return (
        text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace('"', "&quot;")
    )


def build_revisions(files, revisions) -> str:
    """`revisions.xml` mirrors the whole outline a second time, prose included,
    which is why a real one outgrows everything else in the project."""
    rows = []
    for item_id, timestamp, text in revisions:
        rows.append(
            '  <outlineItem title="scène" ID="{}" type="md" compile="2">\n'
            '    <revision timestamp="{}" text="{}"/>\n'
            "  </outlineItem>".format(item_id, timestamp, xml_attr(text))
        )
    return (
        "<?xml version='1.0' encoding='UTF-8'?>\n"
        '<outlineItem title="Root" ID="0" type="folder" compile="2" lastPath="">\n'
        + "\n".join(rows)
        + "\n</outlineItem>\n"
    )


def write_folder(base: pathlib.Path, files: dict) -> None:
    if base.exists():
        for path in sorted(base.rglob("*"), reverse=True):
            path.rmdir() if path.is_dir() else path.unlink()
        base.rmdir()
    for name, text in files.items():
        path = base / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf8", newline="\n")


def write_zip(path: pathlib.Path, files: dict, separator: str = "/") -> None:
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as archive:
        for name, text in files.items():
            # Manuskript builds member names with `os.path.join`, so an archive
            # written on Windows carries backslashes. Its own reader split on the
            # local separator, which is why such a file opened with an empty
            # outline on Linux and then saved the emptiness back over itself.
            archive.writestr(name.replace("/", separator), text)


def build_v0(spec, files) -> dict:
    """The 2016 format: one zip of XML, no marker file.

    A different shape entirely, and worth a fixture of its own rather than a unit
    test: the `type="html"` scene below is the case that used to import as an
    empty document, because the body reached a Markdown parser as raw markup and
    came back as nothing. It also carries `summarySentance`, misspelled as every
    project of that era spells it, and the empty leading rows that format 1 leaves
    out of its vocabulary files.
    """
    chapters = spec["chapter"][:4]
    characters = spec["character"][:3]

    def model(rows, columns=None):
        head = "".join(
            '      <label row="{}" text="{}"/>\n'.format(n, xml_attr(name))
            for n, name in enumerate(columns or [])
        )
        body = ""
        for r, cells in enumerate(rows):
            body += '    <row row="{}">\n'.format(r)
            for c, cell in enumerate(cells):
                text, color = cell if isinstance(cell, tuple) else (cell, None)
                attr = ' color="{}"'.format(color) if color else ""
                if text:
                    body += '      <col col="{}"{}>{}</col>\n'.format(c, attr, xml_attr(text))
                else:
                    body += '      <col col="{}"{}/>\n'.format(c, attr)
            body += "    </row>\n"
        return (
            "<?xml version='1.0' encoding='UTF-8'?>\n"
            '<model version="0.1.1">\n  <header>\n    <horizontal>\n'
            + head
            + "    </horizontal>\n  </header>\n  <data>\n"
            + body
            + "  </data>\n</model>\n"
        )

    out = {}
    # Row 0 is the project's details, row 1 the summary ladder.
    out["flatModel.xml"] = model([
        [spec["book"]["title"], "Les Voyages extraordinaires", "Les Voyages extraordinaires",
         "11", "Roman d’aventures", "Domaine public", spec["book"]["author"], ""],
        ["Un pari tenu pour impossible.", "", spec["book"]["synopsis"], "", ""],
    ])
    # Both vocabularies keep Manuskript's own empty "none" row, which format 1
    # leaves out of the file and re-adds on load. Colours are #aarrggbb here.
    out["labels.xml"] = model([
        [("", "#00000000")], [("Idée", "#ffffff00")], [("Note", "#ff00ff00")],
        [("Chapitre", "#ff0000ff")], [("Scène", "#ffff0000")],
    ])
    out["status.xml"] = model([[""]] + [[s["name"]] for s in spec["status"]])
    out["perso.xml"] = model(
        [
            [(c["name"], ["#ffff0000", "#ff00ff00", "#ff0000ff"][n]), str(n),
             "2" if n == 0 else "1", "", "", "", "",
             c["note"].split(".")[0] + ".", c["note"], "", "", "", ""]
            for n, c in enumerate(characters)
        ],
        columns=["name", "ID", "importance", "motivation", "goal", "conflict", "epiphany",
                 "summarySentance", "summaryPara", "summaryFull", "notes", "Name", "Value"],
    )
    out["world.xml"] = model([["Lieux", "1", "", "", ""]])
    # Format 0 plots: the characters and the steps are sub-rows, and the cell's
    # own text is a placeholder the model seeds it with.
    out["plots.xml"] = (
        "<?xml version='1.0' encoding='UTF-8'?>\n"
        '<model version="0.1.1">\n  <data>\n    <row row="0">\n'
        '      <col col="0">Le pari</col>\n      <col col="1">0</col>\n'
        '      <col col="2">2</col>\n'
        '      <col col="3">Persos<row row="0"><col col="0">0</col></row>'
        '<row row="1"><col col="0">1</col></row></col>\n'
        '      <col col="4"/>\n      <col col="5"/>\n'
        '      <col col="6">Subplots</col>\n      <col col="7"/>\n'
        "    </row>\n  </data>\n</model>\n"
    )

    # The outline, prose and all, in one file. The first scene is `html`.
    rows = []
    for n, chapter in enumerate(chapters):
        body = chapter["synopsis"]
        if n == 0:
            declared = "html"
            text = "<p>{}</p><p>Et <b>ainsi</b> commence le voyage.</p>".format(body)
        else:
            declared = "md"
            text = body
        rows.append(
            '  <outlineItem title="{}" ID="{}" type="{}" compile="2"'
            ' summarySentance="{}" text="{}">\n'
            '    <revision timestamp="{}" text="{}"/>\n'
            "  </outlineItem>".format(
                xml_attr("Chapitre {}".format(chapter["number"])), 10 + n, declared,
                xml_attr(chapter["title"][:60]), xml_attr(text),
                1_455_000_000 + n * 86_400, xml_attr(text[:80]),
            )
        )
    out["outline.xml"] = (
        "<?xml version='1.0' encoding='UTF-8'?>\n"
        '<outlineItem title="Root" ID="0" type="folder" compile="2">\n'
        + "\n".join(rows)
        + "\n</outlineItem>\n"
    )
    # Never opened, and never deserialized: `pickle.loads` on it was
    # CVE-2021-35196. A real format-0 project carries one, so the fixture does.
    out[":pickle:"] = b"\x80\x03}q\x00."
    _ = files
    return out


def main() -> int:
    if not SPEC.is_file():
        print("cannot find {}".format(SPEC), file=sys.stderr)
        return 1
    with open(SPEC, "rb") as handle:
        spec = tomllib.load(handle)
    files = build(spec)

    write_folder(OUT / "tour-du-monde", files)
    # In folder mode the `.msk` is one byte holding the format version, and the
    # project is the directory beside it. Both have to travel together.
    (OUT / "tour-du-monde.msk").write_text("1", encoding="utf8", newline="\n")
    write_zip(OUT / "tour-du-monde-zip.msk", files)
    write_zip(OUT / "tour-du-monde-windows.msk", files, separator="\\")

    v0 = build_v0(spec, files)
    pickle_bytes = v0.pop(":pickle:")
    with zipfile.ZipFile(OUT / "tour-du-monde-2016.msk", "w", zipfile.ZIP_DEFLATED) as archive:
        for name, text in v0.items():
            archive.writestr(name, text)
        archive.writestr("settings.pickle", pickle_bytes)

    print("wrote {} members".format(len(files)))
    print("  tour-du-monde.msk + tour-du-monde/")
    print("  tour-du-monde-zip.msk")
    print("  tour-du-monde-windows.msk")
    print("  tour-du-monde-2016.msk  (format 0)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
