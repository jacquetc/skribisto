<!-- SPDX-License-Identifier: GPL-3.0-only -->
<!-- SPDX-FileCopyrightText: 2015 Cyril Jacquet -->

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

- [Skribisto](#skribisto)
  * [What it does today](#what-it-does-today)
  * [User manual](#user-manual)
  * [Discussions](#discussions)
  * [Support](#support)
  * [Help is always appreciated](#help-is-always-appreciated)
    + [Easier tasks for beginners](#easier-tasks-for-beginners)
  * [For tech people, under the hood](#for-tech-people-under-the-hood)
    + [The writing model](#the-writing-model)
    + [The project format](#the-project-format)
    + [Workspace layout](#workspace-layout)
  * [Build it, test it](#build-it-test-it)
    + [Prerequisites](#prerequisites)
    + [Building and running](#building-and-running)
    + [Linux (Flatpak)](#linux-flatpak)
    + [Windows](#windows)
    + [macOS](#macos)
  * [Translation](#translation)
  * [To contact me](#to-contact-me)
  * [License](#license)
    + [The manuscript is always free](#the-manuscript-is-always-free)
  * [Contributing](#contributing)
  * [Commercial support](#commercial-support)
  * [Trademark](#trademark)

# Skribisto

**Skribisto** is born from the ashes of **Plume Creator**, keeping the goals of its ancestor
while adopting more recent ways to think an application.

Where Plume Creator was geared toward writing novels, Skribisto aims to be more generic. You
organise a project in a **binder** tree of folders and items, but the book's actual structure
does not come from where a row sits in that tree: it comes from the *role* you give the row,
whether Book, Part, Chapter, Scene, Note or plain Text. Every writing row owns both its prose
and a synopsis, which is why the editor has two panes.

What Skribisto is not: LibreOffice, Calligra or Word. A project exports to DOCX, EPUB, PDF,
HTML, Markdown, Djot, LaTeX or plain text, so the final formatting can happen in a full word
processor.

Accessibility is too often forgotten. The interface exposes an accessibility tree (AccessKit),
so screen readers can drive it; JAWS and NVDA are the ones used for testing. Please get in
touch if you hit a glaring gap.

## What it does today

- Binder tree with full editing: create, rename, duplicate, move, indent, outdent, promote
- Dual editor pane (prose plus synopsis), with split panes and tabs
- Manuscript streams: read a whole Chapter, Part or Book, or every synopsis, as one document
- Corkboard, writing sessions, live word count, pace tracking
- An Analysis tab for the Book: pacing/shape, repetition, synopsis-vs-prose drift, and
  vocabulary variety, always measured against the manuscript's own numbers, never a norm
- Anchored comments in the margin (LibreOffice-style), with threaded replies and both a
  project-wide and a per-document comments dock
- Note templates: built-in presets (character sheet, location, object, beat sheet, faction,
  research note) or save your own
- Images in the prose — a map, a character reference, a photograph of a street — carried
  inside the project and through every export, with a book cover
- Colour tags per project, with curated genre presets, and point-of-view marking on scenes
- Replace-while-typing: a custom lexicon plus locale-aware smart punctuation (curly quotes,
  dashes, ellipsis, French spacing…)
- Distraction-free writing mode, with its own colour themes
- Search and replace across the project
- Trash and restore, with undo
- Autosave, manual save, save-as, and backups (retention policy, multiple destinations,
  scheduler, and opening a backup read-only)
- Opens legacy `.skrib` SQLite projects, upgrading them on load
- Imports from Plume Creator (`.plume`)
- Exports to DOCX, EPUB, PDF, HTML, Markdown, Djot, LaTeX and plain text, with a live preview
- Spell checking with downloadable dictionaries
- Light and dark themes, per-editor typography, adjustable text scale
- English and French user interface, for now
- Single instance: several projects can be open at once, each in its own window, without
  spawning a new process per project

## User manual

The dedicated website for the user manual is [here](https://manual.skribisto.eu/en_US/manual.html).

The dedicated website for the FAQ is [here](https://manual.skribisto.eu/en_US/faq.html).

Each one can be translated (see the [Translation](#translation) section).

## Discussions

A Discord server is available. Do you need help, want to offer suggestions, or just talk?
Join us [here](https://discord.gg/5BSkvQmyVH).

## Support

This is a GPL v3 project, so support is on a voluntary basis. Personally, I will only accept
bug issues from users running Skribisto through these packaging methods:

- on Linux: Flatpak only
- on Windows: the Inno Setup installer published with a release

## Help is always appreciated

If this project takes your interest, if you want to help or wish for more details, you can
contact me or create issues.

### Easier tasks for beginners

- Solve one of the [good first issue](https://github.com/jacquetc/skribisto/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) tickets
- Translate the software, the user manual or the FAQ (see the [Translation](#translation) section)
- Complete the FAQ or the user manual in the dedicated repository
  [here](https://github.com/jacquetc/skribisto-help-website/tree/develop)

## For tech people, under the hood

Skribisto is written in **Rust** (edition 2024), end to end. The desktop UI is built on
**[Bastyde](https://github.com/ferntech-eu/bastyde)**, a pure-Rust GUI framework, so there is
no Qt, no QML and no C++ in the build. The backend is generated by **Qleany**, a Clean
Architecture code generator, from [qleany.yaml](qleany.yaml). Dependencies run one way: UI to
controllers, controllers to use cases, use cases to repositories, repositories to an in-memory
store, with undo/redo and an event bus alongside.

> The original C++/Qt6 implementation was removed once the Rust app reached parity. It is
> preserved under the `cpp-final` tag if you need to consult it.

### The writing model

The binder tree is **organisational only**. A book's real structure is a state machine over
the flat, ordered stream of items, driven by two typed axes: a `role` (Folder or Item, purely
a UI concern) and a composable `sub_role` (Book, Part, Chapter, Scene, ChapterScene, Note,
Text). Text is explicit: each content row carries its own role, be it scene text, note text,
synopsis or title. The valid combinations live in a single table, in `crates/skribisto_model`,
which drives backend validation, the "＋ Create" menu and the editor panes alike. Anything
absent from that table is invalid by construction.

### The project format

A project is a `.skrib` **bundle**: RON manifests plus [Djot](https://djot.net) prose, stored
either as a single zip (the default) or as an exploded folder that is comfortable to keep in
git. Legacy SQLite `.skrib` files from the C++ era are detected and upgraded when opened.

Images live in `assets/` inside the bundle, named by the blake3 hash of their bytes, and the
prose references them as ordinary Djot — `![alt](assets/<hash>.png){width=… height=…}`. So the
same picture inserted twice costs one copy, an exploded-folder project resolves its own images
on disk, and a plain Markdown viewer pointed at the folder shows them. A project that carries
images requires format version 8; one that does not still opens in an older build.

### Workspace layout

A cargo workspace under `crates/`:

- `bastyde_ui`, the desktop app (the binary is named `skribisto`)
- `skribisto_model`, the writing-model constraint matrix
- `skrib_format`, the `.skrib` bundle reader and writer
- `skribisto_compiler` and `skribisto-fonts`, the export pipeline and its bundled typefaces
- `work_management`, `binder_item_management`, `trash_management`, `search_management`,
  `import_management`, `export_management`, `handling_app_lifecycle`, `progress_management`,
  `analysis_management`, `mention_management`, `note_template_management` and
  `tag_management`, the Qleany features
- `common`, `direct_access`, `frontend`, `macros` and `binder_ordering`, the shared backend
  layers

## Build it, test it

### Prerequisites

A recent stable Rust toolchain (edition 2024). Install it from [rustup.rs](https://rustup.rs).

On Linux you also need the usual desktop development libraries. The exact package list CI
installs is in
[.github/actions/install-linux-deps/action.yml](.github/actions/install-linux-deps/action.yml).

Skribisto is built from path dependencies on sibling repositories, so clone them all into the
same parent directory:

```
~/Devel/skribisto
~/Devel/bastyde         # the GUI framework
~/Devel/text-document   # the rich-text document model
~/Devel/text-typeset    # the typesetter under Bastyde's text layer
```

Skribisto points at `bastyde` and `text-document` itself; `bastyde` in turn resolves
`text-typeset` and `text-document` the same way, so all four checkouts have to be present, and
side by side.

This sibling layout is a **local-development requirement only**. CI never clones the other
repositories: workflows (and jobs) that need to resolve the Rust dependency graph first run
[.github/actions/strip-path-deps](.github/actions/strip-path-deps/action.yml) (5 of the 8
workflow files: `audit.yml`, `ci.yml`, `release-macos.yml`, `release.yml`, `rust-next.yml`),
which drops the `path = "../…"` attribute from each external dependency so that the `version =`
beside it resolves from crates.io instead. Internal `crates/…` paths are left untouched. Jobs
that never touch Cargo — `packaging-lint.yml`, `generate-release-in-appdata.yml`,
`spelling.yml`, and `ci.yml`'s rustfmt/spdx/locales jobs — skip this step entirely, and
`release.yml`'s `flatpak` job strips paths via its own
[package/flatpak/gen-cargo-sources.sh](package/flatpak/gen-cargo-sources.sh) script (which
duplicates the same sed logic) rather than via this composite action.

### Building and running

```bash
cargo build                                  # the whole workspace
cargo build -p bastyde_ui                    # just the app
cargo build -p bastyde_ui --features mocks   # the app with fabricated data, no backend
cargo run   -p bastyde_ui                    # run it
cargo run   -p bastyde_ui -- path/to/project.skrib
cargo test                                   # backend and UI tests
```

PDF export sits behind an opt-in feature, because it pulls in a large typesetting dependency:

```bash
cargo build -p bastyde_ui --features pdf
```

### Linux (Flatpak)

Make sure `flatpak` and `flatpak-builder` are installed, then add Flathub and the runtime (see
the [Flathub setup guide](https://flatpak.org/setup/)). The exact runtime version is declared
in the manifest,
[package/flatpak/eu.skribisto.skribisto.yml](package/flatpak/eu.skribisto.skribisto.yml).

```bash
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
```

Build and install from your local checkout. The manifest builds the repository directory it
sits in:

```bash
flatpak-builder --user --repo=local-repo build-dir \
    package/flatpak/eu.skribisto.skribisto.yml --force-clean
flatpak build-update-repo local-repo
flatpak --user remote-add --no-gpg-verify local-repo local-repo   # once
flatpak install local-repo eu.skribisto.skribisto -y --reinstall
flatpak run eu.skribisto.skribisto
```

To remove it: `flatpak remove eu.skribisto.skribisto`.

### Windows

The installer is built with [Inno Setup](https://jrsoftware.org/isdl.php) from
[package/windows/setup.iss](package/windows/setup.iss):

```powershell
cargo build --release --target x86_64-pc-windows-msvc -p bastyde_ui --features pdf
ISCC.exe package\windows\setup.iss
```

CI runs an equivalent (but not verbatim) sequence: the same `cargo build` line above, followed
by a fuller `ISCC.exe` invocation that passes `/DMyAppVersion`, `/DMySourceExe`, `/O` and `/F`
switches and calls the tool via its full install path rather than bare `ISCC.exe`. See
[.github/workflows/release.yml](.github/workflows/release.yml) (lines 120 and 141-145).

### macOS

`cargo build -p bastyde_ui` works. A `.dmg` is produced with
[cargo-packager](https://github.com/crabnebula-dev/cargo-packager); its configuration lives in
`[package.metadata.packager]` in [crates/bastyde_ui/Cargo.toml](crates/bastyde_ui/Cargo.toml).

## Translation

The interface is translated with [Fluent](https://projectfluent.org). The catalogues are plain
`.ftl` files under [crates/bastyde_ui/locales/](crates/bastyde_ui/locales/), one directory per
locale, split into four files each (`main.ftl`, `tooltips.ftl`, `tags.ftl`, `templates.ftl`):

```
crates/bastyde_ui/locales/en-US/{main,tooltips,tags,templates}.ftl
crates/bastyde_ui/locales/fr-FR/{main,tooltips,tags,templates}.ftl
```

`en-US` is the source language and the one keys are validated against at compile time; other
locales fall back to it at runtime for anything missing. To add a language, copy the `en-US`
directory to your locale code and translate the values. No build-system change is needed.

There is no Transifex, no `lupdate` or `lrelease`, and no `.ts`/`.qm` step any more. Edit the
`.ftl` files directly and open a pull request.

## To contact me

cyril.jacquet@ferntech.eu (UTC+1)

## License

Skribisto is free software under the **GNU General Public License v3.0**. See
[LICENSE](LICENSE). It is developed as **open core**: the community edition is, and will
remain, GPLv3, while a separate commercial edition with additional features
may be offered under proprietary terms.

### The manuscript is always free

Open core draws a line through a project. This is where Skribisto's line runs, and it does not
move: everything you need to write a book stays under the GPL, permanently.

- **Writing is community-edition work.** Creating, editing, opening, saving and exporting a
  manuscript, in every format Skribisto supports, belongs to the free edition.
- **Your project stays readable.** The `.skrib` bundle format stays open and documented, and a
  project touched by a commercial edition remains readable, editable and exportable by the
  community edition.
- **Nothing is taken back.** No feature that has shipped in the community edition will ever be
  moved out of it.
- **No key, no server, no permission.** Skribisto will never require a licence key, an
  activation step or a network connection to open or edit your own work.

A commercial edition adds capability *around* the manuscript, and services that genuinely need
a server to exist at all. It will never stand between you and your book.

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) first. Because
Skribisto is developed open-core, contributions are accepted under a **Contributor License
Agreement** ([CLA.md](CLA.md)) rather than a bare sign-off: you keep the copyright to your
work, you let it serve both the community and commercial editions, and in return your
contribution is guaranteed to always remain available under the GPL. You agree by signing off
your commits (`git commit -s`).

## Commercial support

A commercial edition is planned. For commercial-licensing or
priority-support enquiries, contact <support@ferntech.eu>. For everyone else, the
[issue tracker](https://github.com/jacquetc/skribisto/issues) and the
[Discord](https://discord.gg/5BSkvQmyVH) are the right places.

## Trademark

Skribisto™ is a trademark of Cyril Jacquet. The GPL source license does **not** grant
trademark rights. Forks and derivative works may use the source code under the GPL but must
adopt a **distinct name and distinct branding** when distributed (compare Firefox and
Iceweasel, or Chromium and Chrome). Nominative use is fine, as in "built on Skribisto", "a
Skribisto import filter", or articles describing Skribisto. For other uses, contact
<trademarks@ferntech.eu>.
