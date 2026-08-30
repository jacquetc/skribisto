<!-- SPDX-License-Identifier: GPL-3.0-only -->
<!-- SPDX-FileCopyrightText: 2015 Cyril Jacquet -->

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

- [Skribisto](#skribisto)
  * [What it does today](#what-it-does-today)
  * [Help](#help)
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
      - [Building it from Linux](#building-it-from-linux)
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

What Skribisto is not: LibreOffice, Calligra or Word. A project exports to DOCX, ODT, EPUB,
PDF, HTML, Markdown, Djot, LaTeX or plain text, so the final formatting can happen in a full
word processor.

Accessibility is too often forgotten. The interface exposes an accessibility tree (AccessKit),
so screen readers can drive it; JAWS and NVDA are the ones used for testing, sometimes with a
braille display. Please get in touch if you hit a glaring gap.

## What it does today

- Binder tree with full editing: create, rename, duplicate, move, indent, outdent, promote
- Dual editor pane (prose plus synopsis), with split panes, tabs, and tabs you can pin
- Manuscript streams: read a whole Chapter, Part or Book, or every synopsis, as one document
- Corkboard, and an overview table of any subtree you can sort and filter
- Book, part and chapter numbers that follow the manuscript, so an untitled chapter is still
  "Chapter 7" everywhere you meet it
- Writing sessions, live word count, and pace tracking against a deadline and a weekday
  schedule
- Word or character targets on any item, with a way to spread a container's target across
  what is inside it
- A status ladder you name yourself ("zero draft", "needs a pass", "final"), shown on the row,
  in the overview table's filters, and in a readout of where the book stands
- An Analysis tab for the Book: its shape (words per scene, dialogue share, footnote words),
  always measured against the manuscript's own numbers, never a norm; and how the text of this
  session arrived, typed, pasted, dictated or imported
- A story bible: notes for the people, places and things in the book, with aliases, and every
  scene each one is named in
- Anchored comments in the margin (LibreOffice-style), with threaded replies and both a
  project-wide and a per-document comments dock
- Footnotes, numbered by the book rather than stored, so inserting one renumbers the rest
- Note templates: built-in presets (character sheet, location, object, beat sheet, faction,
  research note) or save your own
- Images in the prose, a map or a character reference or a photograph of a street, carried
  inside the project and through every export, with a book cover
- Colour tags per project, with curated genre presets, and point-of-view marking on scenes
- Replace-while-typing: a custom lexicon plus locale-aware smart punctuation (curly quotes,
  dashes, ellipsis, French spacing…)
- Margin marks: a strip beside the scroll bar saying where the comments, the search hits and
  the document boundaries are
- Distraction-free writing mode, with its own colour themes
- "Always forward", a mode that refuses every way of taking back what you have already
  written, so a first draft can only grow
- One Undo for the whole application. Ctrl+Z takes back whatever you were looking at, and the
  Edit menu names it first, as in "Undo trashing «Chapter 3»"
- Search and replace across the project (prose, titles, synopses, comments and footnotes),
  down to the single occurrence
- Trash and restore
- Autosave, manual save, save-as, and backups (retention policy, multiple destinations,
  scheduler, and opening a backup read-only)
- Version history built out of those backups: read what a scene said last week, set it against
  what it says now, put it back, or bring back a row you deleted months ago
- Opens legacy `.skrib` SQLite projects, upgrading them on load
- Imports whole projects from Manuskript (`.msk`, folder or single file) and Plume
  Creator (`.plume`)
- Imports documents (Markdown, plain text, ODT and DOCX), showing you every row it would
  create before anything is
- Sends a chapter out to an editor as DOCX or ODT and takes it back: their comments arrive as
  real comments, anchored to the words they were about
- Exports to DOCX, ODT, EPUB, PDF, HTML, Markdown, Djot, LaTeX and plain text
- Spell checking with downloadable dictionaries
- Light and dark themes, per-editor typography, adjustable text scale
- English and French user interface, for now
- Single instance: several projects can be open at once, each in its own window, without
  spawning a new process per project

## Help

The help ships with the application: **F1** opens the help topics, there is a keyboard
shortcut window beside them, and Ctrl+Shift+P opens a command palette that finds any command
by name. Because it is part of the binary, it describes the version you are running rather
than the version somebody last wrote about.

The topics are translated alongside the interface, though they are not `.ftl` files: the
longer pages are Djot, under [crates/teksilo_ui/help/](crates/teksilo_ui/help/), one directory
per locale. The shorter ones are the same text the tooltips use, so they are translated once
and read in both places. See the [Translation](#translation) section for both.

## Discussions

A Discord server is available. Do you need help, want to offer suggestions, or just talk?
Join us [here](https://discord.gg/5BSkvQmyVH).

## Support

This is a GPL v3 project, so support is on a voluntary basis. Personally, I will only accept
bug issues from users running Skribisto through these packaging methods:

- on Linux: Flatpak only
- on Windows: the NSIS installer published with a release

## Help is always appreciated

If this project takes your interest, if you want to help or wish for more details, you can
contact me or create issues.

### Easier tasks for beginners

- Solve one of the [good first issue](https://github.com/jacquetc/skribisto/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) tickets
- Translate the software, help topics included (see the [Translation](#translation) section)
- Improve a help topic that explains the wrong thing, or write one that does not exist yet

## For tech people, under the hood

Skribisto is written in **Rust** (edition 2024), end to end. The desktop UI is built on
**[Teksilo](https://github.com/ferntech-eu/teksilo)**, a pure-Rust GUI framework, so there is
no Qt, no QML and no C++ in the build. The backend is generated by **Qleany**, a Clean
Architecture code generator, from [qleany.yaml](qleany.yaml). Dependencies run one way: UI to
controllers, controllers to use cases, use cases to repositories, repositories to an in-memory
store, with undo/redo and an event bus alongside.

> The original C++/Qt6 implementation was removed once the Rust app reached parity. It is
> preserved under the `cpp-final` tag if you need to consult it.

### The writing model

The binder tree is **organisational only**. A book's real structure is a state machine over
the flat, ordered stream of items, driven by two typed axes: a `role` (Folder or Item, purely
a UI concern) and a composable `sub_role` (Book, Part, Scene, ChapterScene, Note, Text,
Paratext, BookBegin, BookEnd and None). There is no `Chapter` variant: a chapter is a
UI-level composite that resolves to a folder or a flat row depending on
`Work.chapter_mode`. Text is explicit: each content row carries its own role, be it scene
text, note text, synopsis or title. The valid combinations live in a single table, in
`crates/skribisto_model`, which drives backend validation, the "＋ Create" menu and the
editor panes alike. Anything absent from that table is invalid by construction.

### The project format

A project is a `.skrib` **bundle**: RON manifests plus [Djot](https://djot.net) prose, stored
either as a single zip (the default) or as an exploded folder that is comfortable to keep in
git. Legacy SQLite `.skrib` files from the C++ era are detected and upgraded when opened.

Images live in `assets/` inside the bundle, named by the blake3 hash of their bytes, and the
prose references them as ordinary Djot: `![alt](assets/<hash>.png){width=… height=…}`. So the
same picture inserted twice costs one copy, an exploded-folder project resolves its own images
on disk, and a plain Markdown viewer pointed at the folder shows them.

The format version a project *requires* is decided by what it actually carries, not by the
version that wrote it. A bundle claims a floor only where an older build would get it wrong
rather than merely ignore it, either because that build's first save would silently destroy
something (note templates need 5, images 8, footnotes 9, a status ladder 14) or because it
cannot parse the file at all (epigraphs 6, paratexts 7). Delete every image and the next save
lowers the floor again. A project using none of them opens in any build back to version 4, and
one that does is refused **by name**, with a message that says "needs format 7 or newer",
rather than being opened and quietly stripped of what the reader could not represent.

### Workspace layout

A cargo workspace under `crates/`:

- `teksilo_ui`, the desktop app (the binary is named `skribisto`)
- `skribisto_model`, the writing-model constraint matrix
- `skrib_format`, the `.skrib` bundle reader and writer
- `document_ingest`, the format-agnostic scanner behind Markdown/DOCX/ODT import
- `manuskript_import` and `plume_import`, the whole-project converters
- `spellcheck_engine`, the pure-Rust Hunspell-compatible checker
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
~/Devel/teksilo         # the GUI framework
~/Devel/text-document   # the rich-text document model
~/Devel/text-typeset    # the typesetter under Teksilo's text layer
```

Skribisto points at `teksilo` and `text-document` itself; `teksilo` in turn resolves
`text-typeset` and `text-document` the same way, so all four checkouts have to be present, and
side by side.

This sibling layout is a **local-development requirement only**. CI never clones the other
repositories: workflows (and jobs) that need to resolve the Rust dependency graph first run
[.github/actions/strip-path-deps](.github/actions/strip-path-deps/action.yml) (5 of the 8
workflow files: `audit.yml`, `ci.yml`, `release-macos.yml`, `release.yml`, `rust-next.yml`),
which drops the `path = "../…"` attribute from each external dependency so that the `version =`
beside it resolves from crates.io instead. Internal `crates/…` paths are left untouched. Jobs
that never touch Cargo (`packaging-lint.yml`, `generate-release-in-appdata.yml`,
`spelling.yml`, and `ci.yml`'s rustfmt/spdx/locales jobs) skip this step entirely, and
`release.yml`'s `flatpak` job strips paths via its own
[package/flatpak/gen-cargo-sources.sh](package/flatpak/gen-cargo-sources.sh) script (which
duplicates the same sed logic) rather than via this composite action.

### Building and running

```bash
cargo build                                  # the whole workspace
cargo build -p teksilo_ui                    # just the app
cargo build -p teksilo_ui --features mocks   # the app with fabricated data, no backend
cargo run   -p teksilo_ui                    # run it
cargo run   -p teksilo_ui -- path/to/project.skrib
cargo test                                   # backend and UI tests
```

PDF export sits behind an opt-in feature, because it pulls in a large typesetting dependency:

```bash
cargo build -p teksilo_ui --features pdf
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

The installer is built with [NSIS](https://nsis.sourceforge.io) from
[package/windows/setup.nsi](package/windows/setup.nsi):

```powershell
cargo build --release --target x86_64-pc-windows-msvc -p teksilo_ui --features pdf
makensis package\windows\setup.nsi
```

CI runs an equivalent (but not verbatim) sequence natively on `windows-latest`: the same
`cargo build` line, followed by a fuller `makensis` invocation passing `/DAPP_VERSION`,
`/DSRC_EXE` and `/DOUT_FILE`. See [.github/workflows/release.yml](.github/workflows/release.yml).
Note that `makensis` resolves relative paths against the directory holding the script rather than
the working directory, so any path passed with `/D` should be absolute.

#### Building it from Linux

[package/windows/build.py](package/windows/build.py) does the whole job (compile, verify, zip,
installer) on either host, which is useful when you have no Windows machine to hand. The release
is not built this way; it stays native so a real Windows machine remains in the release path.

```bash
sudo apt-get install -y nsis clang lld llvm   # once
rustup target add x86_64-pc-windows-msvc      # once
cargo install --locked cargo-xwin             # once

python3 package/windows/build.py --version 3.0.0
```

That writes `dist/skribisto.exe`, `dist/Skribisto-portable.zip` and `dist/Skribisto-setup.exe`.
The target stays `x86_64-pc-windows-msvc` (not mingw): `cargo-xwin` supplies the MSVC CRT and
Windows SDK and links with `lld-link`. On first use it downloads those from Microsoft and asks
you to accept their licence; export `XWIN_ACCEPT_LICENSE=1` to answer ahead of time.

Since a cross-build never executes the binary it produces, the script inspects the finished exe
for the two failures that are otherwise silent: a dropped `+crt-static` flag, and an icon or
VERSIONINFO that failed to embed. `--check-only --exe <path>` runs just those checks against any
build, including one CI produced.

### macOS

`cargo build -p teksilo_ui` works. A `.dmg` is produced with
[cargo-packager](https://github.com/crabnebula-dev/cargo-packager); its configuration lives in
`[package.metadata.packager]` in [crates/teksilo_ui/Cargo.toml](crates/teksilo_ui/Cargo.toml).

**No macOS build ships with a release yet.**
[.github/workflows/release-macos.yml](.github/workflows/release-macos.yml) is written and
works, but runs on manual dispatch only, so tagging a version builds Linux and Windows and
never macOS. What it produces is unsigned, because signing needs an Apple Developer Program
enrolment this project does not have. The workflow's own header says what to do to turn it on.

## Translation

The interface is translated with [Fluent](https://projectfluent.org). The catalogues are plain
`.ftl` files under [crates/teksilo_ui/locales/](crates/teksilo_ui/locales/), one directory per
locale, split into five files each:

```
crates/teksilo_ui/locales/en-US/{main,tooltips,tags,templates,story_bible}.ftl
crates/teksilo_ui/locales/fr-FR/{main,tooltips,tags,templates,story_bible}.ftl
```

The longer help pages are separate, and are Djot rather than Fluent. There are ten pages per
locale, under [crates/teksilo_ui/help/](crates/teksilo_ui/help/):

```
crates/teksilo_ui/help/en-US/*.djot
crates/teksilo_ui/help/fr-FR/*.djot
```

`en-US` is the source language and the one keys are validated against at compile time; other
locales fall back to it at runtime for anything missing, and a help page served in a language
you did not ask for says so in a banner rather than passing itself off.

To add a language, copy both `en-US` directories to your locale code, translate the values,
and then register the locale in three places. The strings are compiled into the binary rather
than discovered on disk, so a new directory on its own is never loaded:

- `SUPPORTED_LOCALES` in [crates/teksilo_ui/src/startup.rs](crates/teksilo_ui/src/startup.rs)
- the `.compile_in(…)` block in the same file, which needs one `include_str!` line per `.ftl`
- the `djot_page!` rows in [crates/teksilo_ui/src/help.rs](crates/teksilo_ui/src/help.rs), one
  per help page

That is three source edits and no build-system change; nothing else in the build has to move,
and a locale nobody has translated yet costs the ones that exist nothing.

There is no Transifex, no `lupdate` or `lrelease`, and no `.ts`/`.qm` step any more. Edit the
`.ftl` and `.djot` files directly and open a pull request.

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

Skribisto™ is a trademark of FernTech. The GPL source license does **not** grant
trademark rights. Forks and derivative works may use the source code under the GPL but must
adopt a **distinct name and distinct branding** when distributed (compare Firefox and
Iceweasel, or Chromium and Chrome). Nominative use is fine, as in "built on Skribisto", "a
Skribisto import filter", or articles describing Skribisto.

**Distribution packagers may keep the Skribisto name.** Packagers for operating-system
distributions and ecosystems (Debian, Fedora, Arch, Nixpkgs, Homebrew, Guix and the like) may
ship a package called Skribisto, as long as it tracks upstream releases. That includes the
changes packaging normally requires:

- backported security and bug fixes;
- adjusted dependency bounds, de-vendoring, unbundling;
- build-system, path and packaging-metadata changes;
- patches carried while an upstream release is pending.

The line is provenance, not patching. What needs a distinct name is a package that changes
Skribisto's behaviour, adds or removes features, or ships from a fork rather than from
upstream releases. If you maintain a package and are not sure which side of that line your
patch set falls on, write to <trademarks@ferntech.eu> rather than renaming preemptively. We
would rather answer the question than lose the package.

For anything not covered here, contact <trademarks@ferntech.eu>.
