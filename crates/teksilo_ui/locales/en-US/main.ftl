# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: UI strings (source locale).
# A single `&` marks the keyboard mnemonic of a menu label; `&&` is a literal `&`.

## Menu bar: Work
menu-work = &Work
menu-new-work = &New Work
menu-open-work = &Open Work…
menu-new-window = New &Window
menu-import-from = &Import from
# The Launcher's counterpart to `menu-import-from`, over the same two rows: these
# produce a brand-new project, where the project window's importers land content
# *in* the one already open.
menu-create-from = &Create from
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documents (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-export = E&xport
menu-export-book = Export Book
menu-export-part = Export Part
menu-export-chapter = Export Chapter
menu-export-scene = Export Scene
menu-export-note = Export Note
menu-export-paratext = Export Paratext
menu-export-folder = Export Folder
menu-export-choose = Choose…
menu-export-none = Open a document to export
menu-save = &Save
menu-save-as-file = Save as single &file…
menu-save-as-folder = Save as fol&der…
menu-backup = &Back up now
menu-close-work = &Close Work
menu-welcome = We&lcome…
menu-settings = S&ettings
menu-quit = &Quit

## Menu bar: View
menu-view = &View
menu-outline = &Outline
menu-search = &Search in Project
menu-timeline = &Go back in time
menu-search-preview = Search &Preview
menu-fullscreen = &Fullscreen
menu-focus-mode = &Distraction-free Mode
menu-format = F&ormat
menu-scene-break = Insert scene brea&k
menu-major-scene-break = Insert &major scene break

## Menu bar: Go
menu-go = &Go
menu-go-next-scene = Next &Scene
menu-go-prev-scene = Pre&vious Scene
menu-go-next-chapter = Next &Chapter
menu-go-prev-chapter = Previous C&hapter
menu-go-next-note = Next &Note
menu-go-prev-note = Previous No&te

menu-tools = &Tools
menu-help = &Help

## Binder context menu
ctx-add = &Add
ctx-new-item = &New Item
ctx-new-folder = New &Folder
ctx-rename = &Rename
ctx-duplicate = &Duplicate
ctx-import-here = &Import here…
ctx-indent = &Indent
ctx-outdent = &Outdent
ctx-trash = Move to &Trash
ctx-open-to-side = Open to the &Side
ctx-open = &Open
ctx-reveal-in-outline = Reveal in Out&line
ctx-move-up = Move &Up
ctx-move-down = Move Dow&n

## Editor tab context menu
# Fresh `ctx-tab-*` keys, never a reuse of the `ctx-*` rows above: those double as
# menu-BAR rows (shell/project_menus/document.rs), so retuning one of their
# mnemonics to fit this menu would break the bar's own per-locale uniqueness.
# Three rows are either/or — the split, the move and the pin each build exactly
# one of their pair — so a letter may repeat across a pair but never within one
# built menu, which is a debug_assert! panic in MenuList::build, not a warning.
ctx-tab-close = &Close
ctx-tab-close-others = Close &others
ctx-tab-close-all = Close a&ll
# The split rows read directionally, from the pane the tab is already in, so the
# writer never has to work out which half "the side" means. They duplicate: the
# item ends up open in both panes over one shared document.
ctx-tab-open-to-side = Open to the &Side
ctx-tab-open-in-main = Open &in the main pane
# The move rows read the same way, but the tab leaves the pane it came from.
ctx-tab-move-to-side = &Move to the side
ctx-tab-move-to-main = &Move to the main pane
ctx-tab-move-to-new-window = Move into a &new window
# Why that last row is unavailable. A second window is opened onto the project's
# file, so a project that has never been saved has nothing for it to open — a
# tooltip on the disabled row, not a menu label, hence no mnemonic.
ctx-tab-move-window-unsaved = Save this project first — a second window opens onto a file on disk
ctx-tab-pin = &Pin this tab
ctx-tab-unpin = &Unpin this tab
# Tab tooltip, not a menu row: no mnemonic. It names the two commands a pin
# actually protects against, because the tab itself shows only a glyph and a
# missing close button — neither of which says what the pin is for.
tab-pinned-tooltip = Pinned — "Close others" and "Close all" leave it open

## Create recommendations: logical type labels (SplitButton title + Add ▸ rows)
create-book = Book
create-part = Part
create-chapter = Chapter
create-scene = Scene
create-note = Note
create-note-folder = Note folder
create-folder = Folder
create-paratext = Paratext
create-paratext-folder = Paratext folder
create-book-end = End of Book
create-story-bible-entry = Story bible entry…
# Item-type names for the Overview's Type column (the rest reuse the create-* nouns).
type-book-start = Book start
type-text = Text

## Create: the default title a new row is given.
## Data, not chrome — resolved once at creation and then owned by the writer,
## so switching language never retitles anything already created.
## (A scene reuses `new-scene-title`, shared with split-scene.)
new-item-book = New Book
new-item-part = New Part
new-item-chapter = New Chapter
new-item-note = New Note
new-item-note-folder = New Note Folder
new-item-folder = New Folder
new-item-paratext = New Paratext
new-item-paratext-folder = New Paratext Folder
new-item-story-bible-entry = New Story Bible Entry

## Create recommendations: trailing placement hint on each row
placement-inside = inside
placement-after = after
placement-after-parent = after parent
placement-top-level = top level

# (Writing-model rich tooltips live in this locale's tooltips.ftl.)

## Promote: convert a binder item to its paired type
ctx-promote = &Convert to
promote-chapter-folder = Chapter folder
promote-flat-chapter = Flat chapter
promote-lossy-title = Nowhere to keep the text
promote-lossy-text = A { $target } has nowhere to keep these: { $kinds }. Move or clear that text first, then convert.
# Content-role names, for explaining what a conversion cannot carry over.
content-scene-text = Scene text
content-note-text = Note text
content-book-title = Book title
content-book-subtitle = Book subtitle
content-part-title = Part title
content-chapter-title = Chapter title
content-epigraph-text = Epigraph
content-paratext-text = Paratext
promote-blocked-title = Chapter not empty
promote-blocked-text = { $count ->
    [one] This chapter still holds { $count } item. Move or trash it before converting it to a flat chapter.
   *[other] This chapter still holds { $count } items. Move or trash them before converting it to a flat chapter.
}

## Inspector (trailing dock) + status-bar dock toggles
inspector = Inspector
inspector-empty = Open an item to inspect it.
inspector-promote = Convert to…
statusbar-toggle-outline = Toggle the binder
statusbar-toggle-inspector = Toggle the inspector
# The save indicator (status bar, next to the binder toggle).
statusbar-save-unsaved = Unsaved changes. Click to save
statusbar-save-saved = All changes saved
statusbar-save-autosave = Autosave is on. Changes are saved as you write
statusbar-saving = Saving…
# The focused item's live word count (status bar).
statusbar-word-count = { $count ->
    [one] { $count } word
   *[other] { $count } words
}
statusbar-word-count-tooltip = Words in the scene you're editing
# Words + characters, shown when "Show character count" is on (Settings ▸ Goals).
statusbar-word-char-count = { $words ->
    [one] { $words } word
   *[other] { $words } words
} · { $chars ->
    [one] { $chars } character
   *[other] { $chars } characters
}
# The writing session (status-bar sprint timer + word tracker).
session-toggle = Writing session: start or pause a focused sprint
session-configure = Set the session's word goal and time limit
session-configure-title = Writing session
session-word-goal = Word goal
session-time-limit = Time limit
# The spin box's non-editable trailing unit. An abbreviation, not a word that
# agrees with the number: a `SpinBox` suffix is static text and cannot carry a
# plural selector. The separating space is prepended in code (Qt's `" min"`
# convention), so give the unit alone. Never shown at 0 — that reads
# `session-no-limit` instead.
session-time-limit-unit = min
session-no-goal = No goal
session-no-limit = No limit
session-reset = Reset session
session-readout = { $words } words · { $time }
session-readout-timed = { $words } words · { $time } left
# Distraction-free mode's always-visible control strip (Increment 2).
statusbar-focus-exit = Exit distraction-free mode
# The strip's Next/Previous buttons (Increment 4) — fire the same go.next/go.prev
# actions as the shortcut and the Go menu's generic pair.
statusbar-focus-synopsis = Synopsis
statusbar-focus-go-prev = Previous item (Alt+Up)
statusbar-focus-go-next = Next item (Alt+Down)

## Activity-rail actions (dockless commands in the icon rail)
rail-settings = Settings

## Settings
language = Language
theme = Theme
english = English
french = French
light = Light
dark = Dark
settings-text-width = Text width
settings-preview-width = Search preview width
settings-autosave = Autosave to disk
settings-show-welcome = Show the launcher at startup
settings-show-welcome-tip = When off, the last project reopens instead.

## Settings window: chrome
settings-title = Settings
settings-close = Close
settings-search = Search settings
settings-reset = Reset to defaults
settings-done = Done
settings-cancel = Cancel
settings-apply = Apply
settings-ok = OK
settings-reset-confirm-title = Reset all settings to defaults?
settings-reset-confirm-body = Restores this application's appearance, editor and writing settings. Your project's own settings, keyboard shortcuts and saved styles are not affected. It can't be undone.
settings-empty-title = No settings here yet
settings-empty-hint = This section will gain options in a future update.

## Settings window: categories
settings-sec-appearance-behaviour = Appearance & Behavior
settings-sec-editor = Editor
settings-sec-spelling = Spelling
settings-sec-backup = Backup & Sync
settings-sec-compile = Compile & Export
settings-page-appearance = Appearance
settings-field-image-size-policy = Large images
settings-image-policy-ask = Ask each time
settings-image-policy-keep = Keep original
settings-image-policy-downscale = Optimise
settings-hint-image-size-policy = What to do when an inserted image is larger than the manuscript needs.
settings-page-notifications = Notifications
settings-page-scene = Scene
settings-page-synopsis = Synopsis
settings-page-notes = Notes
settings-page-editor-behavior = Editor Behavior
settings-page-goals = Goals & Word Count
settings-page-games = Writing games
settings-page-corkboard = Corkboard
settings-page-distraction-free = Distraction-free
settings-page-dictionaries = Dictionaries
settings-page-autosave = Autosave
settings-page-export = Export Formats
settings-page-paratext = Paratext Structures
settings-page-keymap = Keymap
# Filter box on the Keymap page (filters the ShortcutSettings list by name / id / category).
settings-keymap-filter = Filter shortcuts

## Settings window: what each page is for
# One line per page, shown under that page's link on its parent's page — and,
# for a parent, under its own title. Keep them to one line: they describe what
# the page holds, not how to use it.
settings-desc-sec-appearance-behaviour = How the application itself looks, and what it does when it starts.
settings-desc-sec-editor = The writing surface: how it looks, and what it does as you type.
settings-desc-sec-spelling = Spell-checking, and the dictionaries behind it.
settings-desc-sec-backup = How your work reaches the disk, and what copies are kept.
settings-desc-sec-compile = What leaves Skribisto, and in what shape.
settings-desc-sec-work = Settings that belong to this project and travel inside its file.
settings-desc-sec-extensions = Pages contributed by the extensions installed here.
settings-desc-group-typography = One page per kind of text — typeface, size, line height and spacing.
settings-desc-appearance = Interface language, theme, text size, and the launcher at startup.
settings-desc-notifications = Every message this session has shown, and the actions you can replay.
settings-desc-scene = How scene prose is set: typeface, size, line height, indents and spacing.
settings-desc-synopsis = How the synopsis pane is set, independently of the manuscript.
settings-desc-notes = How notes are set, independently of the manuscript.
settings-desc-corkboard = Card size, what a card shows, and how the board arranges them.
settings-desc-distraction-free = Typography, column width and control strip for full-screen writing.
settings-desc-distraction-free-themes = The theme library for full-screen writing — the shipped ones and your own.
settings-desc-editor-behavior = Text width, synopsis position, typewriter scrolling and the caret highlight.
settings-desc-punctuation = The typographic house style new projects start from. Every project follows this unless it sets its own.
settings-desc-goals = Word and character targets, and how words are counted.
settings-desc-games = Constraints you set yourself while drafting, such as Always forward.
settings-desc-spellcheck = The single switch that turns spell-checking on and off everywhere.
settings-desc-dictionaries = Install, remove and browse the spelling dictionaries on this machine.
settings-desc-autosave = Whether your edits are written to disk on their own.
settings-desc-backup = When copies are taken, where they are kept, and how many. Every project follows this unless it sets its own.
settings-desc-export = The styles every export compiles through — the shipped ones and your own.
settings-desc-paratext = The front and back matter a new project can start with.
settings-desc-keymap = Every shortcut, and what it is bound to.
settings-page-user = User
settings-desc-user = Who you are, for the comments you write.
settings-group-identity = Identity
settings-field-user-name = Your name
settings-field-user-name-placeholder = Optional
settings-field-user-name-hint = Signs the comments and replies you write. Distinct from a project's author name, which is the book's byline and travels inside the file.
settings-field-user-initials = Your initials
settings-field-user-initials-placeholder = From your name
settings-field-user-initials-hint = What a word processor shows beside your comment in the margin. Leave it blank to use the ones shown, taken from your name.
settings-field-user-hint = Both are optional and apply to every project on this computer. Changing them signs the comments you write next — comments already written keep the name they were written under.
settings-desc-author = The name that goes on this project.
settings-desc-structure = Whether this project's chapters are folders or single items.
settings-desc-language = The language this project's prose is checked against.
settings-desc-work-backup = This project's own backup policy, or the general one.
settings-desc-personal-dictionary = The words this project treats as correctly spelled.
settings-desc-tags = The color-coded labels this project marks its items with.
settings-desc-templates = The note templates you can insert while writing in this project.
settings-desc-text-replacements = Shortcuts that expand as you type in this project.
settings-desc-work-punctuation = The quotes, dashes and spacing this project's prose follows.

## Settings window: fields
settings-group-typography = Typography
settings-group-writing-column = Writing column
settings-group-writing-view = Writing view
# The distraction-free control strip's optional items (Exit is never optional).
settings-group-distraction-free-strip = Control strip
settings-group-theme = Theme
settings-group-language = Language
settings-group-startup = Startup
settings-group-autosave = Autosave
settings-field-typeface = Typeface
settings-field-size = Text size

# The readout shown while Ctrl+Wheel / Ctrl+= / Ctrl+- / Ctrl+0 change an
# editor's text size. One whole message per surface rather than a shared
# "{ $surface }: { $percent }" template: the surface name is not a plain
# substitution in every language, and a translator should get complete
# sentences. $percent arrives already formatted ("110%").
editor-size-changed-manuscript = Manuscript text size: { $percent }
editor-size-changed-synopsis = Synopsis text size: { $percent }
editor-size-changed-notes = Notes text size: { $percent }
editor-size-changed-corkboard = Corkboard card text size: { $percent }
editor-size-changed-corkboard-expanded = Expanded editor text size: { $percent }
editor-size-changed-distraction-free = Distraction-free text size: { $percent }

menu-text-size-increase = &Increase text size
menu-text-size-decrease = De&crease text size
menu-text-size-reset = &Reset text size
settings-field-line-height = Line height
settings-field-first-line-indent = First-line indent
settings-field-paragraph-spacing-before = Space before paragraph
settings-field-paragraph-spacing-after = Space after paragraph
settings-field-column-width = Column width
settings-distraction-free-width-hint = Applies only while distraction-free mode is on — your Scene, Synopsis and Notes column widths are untouched.
settings-distraction-free-title = Keep the item's name
settings-distraction-free-word-count = Keep the word count
settings-distraction-free-session = Keep the writing session
settings-distraction-free-go-to = Keep the Go to… button
settings-distraction-free-go = Keep the Previous and Next buttons
settings-distraction-free-chrome-hint = The Exit button always stays, whatever you choose here — it is your way out if Escape is busy.
settings-field-app-theme = Theme
# The two entries of the Theme picker (the Fluent light / dark appearances).
settings-theme-light = Light
settings-theme-dark = Dark
settings-field-text-scale = Interface text size
settings-field-language = Interface language
settings-synopsis-placement = Synopsis position
settings-synopsis-placement-none = None
settings-synopsis-placement-top = Above
settings-synopsis-placement-side = Beside
settings-typewriter = Typewriter scrolling
settings-typewriter-tip = Holds the line you are writing at a fixed height while the manuscript scrolls under it. Clicking still puts the caret where you click.
settings-typewriter-position = Line position
settings-typewriter-position-top-third = Top third
settings-typewriter-position-middle = Middle
settings-typewriter-position-bottom-quarter = Bottom quarter
settings-highlight-scope = Highlight around the caret
settings-highlight-scope-none = None
settings-highlight-scope-sentence = Sentence
settings-highlight-scope-paragraph = Paragraph
settings-highlight-scope-tip-none = Leave the page plain. Nothing is shaded as you write.
settings-highlight-scope-tip-sentence = Shade the sentence you are writing, so it stands out from the ones around it.
settings-highlight-scope-tip-paragraph = Shade the whole paragraph you are writing, to keep the passage you are working on in view.
settings-group-container-views = Container views
settings-remember-view = Remember the last view for each item type
settings-remember-view-tip = Open a container on the view you last used for that type
settings-remember-view-tip-more =
    A Book, Part and Chapter each offer several views (its own page, the full
    manuscript, the full synopsis). Turn this on and each type reopens on the
    view you last chose for it, e.g. switch one Chapter to Full Chapter and the
    next Chapter you open shows Full Chapter too. Each type remembers its own view.
# Goals & Word Count pane
settings-group-counting = Word counting
settings-counting-auto = Automatic (by language)
settings-counting-whitespace = Split on spaces
settings-counting-unicode-words = Unicode words
settings-counting-cjk-hybrid = CJK-aware (per character)
settings-counting-hint = Automatic counts Chinese and Japanese by character and every other language by word. Change it only if the count looks wrong for your language.
settings-group-goals-display = Display
settings-show-characters = Show the character count in the status bar
settings-autosave-hint = Changes are written to disk automatically as you write.

## Settings: Work (the open project)
settings-sec-work = Work
settings-page-structure = Structure
settings-page-author = Author
settings-field-author-name = Author name
settings-field-author-placeholder = Optional
settings-field-author-hint = Appears on the compiled title page and in exported file metadata. Leave it blank to omit it. This is the book's byline — the name your comments are signed with is in Settings ▸ User.
settings-group-chapters = Chapters
tidy-titles-title = Tidy chapter titles
tidy-titles-none = No chapter or part title is merely repeating its own number.
tidy-titles-lead = { $count ->
        [one] One chapter or part is titled with nothing but its own number.
       *[other] { $count } chapters and parts are titled with nothing but their own number.
    }
tidy-titles-explain = Clearing those titles leaves each one named by the number the book already knows — the same number the export prints. Nothing else changes, and one undo puts them all back.
menu-document-tidy-titles = Tid&y chapter titles…
settings-group-numbering = Numbering
settings-number-chapters = Number chapters and parts
settings-number-chapters-tip = Chapters and parts carry a number worked out from where they sit in the book — shown beside their title here, and printed by the export.
settings-number-chapters-tip-more = The number is never stored in the title, so it stays right when you reorder, insert or delete. Turn this off and the export prints titles alone, whatever the export style asks for. To leave one chapter out — a prologue, an interlude — use the Inspector's Numbering switch instead: that keeps it in the book but stops it taking a number.
settings-part-resets-chapter = Restart chapter numbers at each part
settings-part-resets-chapter-tip = Off by default: chapters run straight on across the parts of a book, so "Part Two" opens on Chapter Eleven.
settings-part-resets-chapter-tip-more = That is the usual trade practice, and what a reader expects. Turn it on for a book whose parts are meant to read as separate volumes, where each one opens on Chapter One.
settings-chapter-flat = Flat chapters
settings-chapter-flat-hint = On: a chapter is a single row. You write into it, and it holds no scenes. Off: a chapter is a folder. You still write into it, but it can hold scenes as well. New chapters follow this setting; existing ones convert via Promote.

## Settings: Export styles (Compile & Export ▸ Export Formats)
settings-styles-builtin = Built-in styles
settings-styles-user = My styles
settings-styles-builtin-badge = Built-in
settings-styles-duplicate = Duplicate
settings-styles-edit = Edit
settings-styles-delete = Delete
settings-styles-import = Import…
settings-styles-export = Export…
settings-styles-copy-suffix = (copy)
settings-styles-json-filter = Export style
settings-styles-editor-title = Edit style
settings-styles-editor-none = Select a custom style to edit it, or duplicate a built-in one.
settings-styles-imported = Style imported
settings-styles-import-failed = Could not import style
settings-styles-exported = Style exported
settings-styles-export-failed = Could not export style
# Editor field labels
settings-styles-field-name = Name
settings-styles-field-chapters = Chapter headings
settings-styles-field-parts = Part headings
settings-styles-field-scene-break = Scene break
settings-styles-field-major-scene-break = Major scene break
settings-styles-field-spacing = Line spacing
settings-styles-field-justify = Justify text
settings-styles-field-notes = Include notes
settings-styles-field-synopses = Include synopses
settings-styles-field-scene-titles = Include scene titles
settings-styles-field-epigraphs = Include epigraphs
settings-styles-field-paratexts = Include paratexts
settings-styles-field-epigraph-placement = Epigraph position
settings-styles-epigraph-after = After the title
settings-styles-epigraph-before = Before the title
settings-styles-field-footnotes = Include footnotes
settings-styles-field-footnote-numbering = Footnote numbering
settings-styles-footnote-numbering-continuous = Continuous
settings-styles-footnote-numbering-per-chapter = Restart each chapter
settings-styles-footnote-numbering-per-book = Restart each book
settings-styles-field-images = Images
menu-image = &Image
settings-styles-images-beside = Beside the document
settings-styles-images-embed = Inside the document
settings-styles-images-omit = Leave out
settings-styles-group-round-trip = Sending to an editor
settings-styles-field-comments = Include comments
settings-styles-field-round-trip-marks = Include round-trip markers
settings-styles-round-trip-hint = DOCX and ODT only. Markers are invisible identifiers that let a returning file update this project instead of landing beside it as a second copy.
settings-styles-group-pages = Pages
settings-styles-field-cover = Open with the cover
settings-styles-field-word-count = Word count on title page
settings-styles-field-page-books = New page at each book
settings-styles-field-page-parts = New page at each part
settings-styles-field-page-chapters = New page at each chapter
settings-styles-field-page-paratexts = New page at each paratext
# Heading-scheme options
settings-styles-heading-none = No heading
settings-styles-heading-numbered = Number only
settings-styles-heading-title = Title only
settings-styles-heading-both = Number + title
# Scene-break options (glyph values stay as typed)
settings-styles-break-blank = Blank line
settings-styles-break-none = None
# Line-spacing options
settings-styles-spacing-single = Single
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Double

## Welcome
welcome-title = Welcome to Skribisto
# $version is stamped from the git tag at build time (see src/version.rs).
welcome-version = Version { $version }
welcome-search = Search works
welcome-open = Open
welcome-new-work = New Work
welcome-create-from = Create from…
welcome-recent-works = Recent Works
welcome-empty-recents = No recent works yet.
# Shown in place of the recents list when the search matched none of them,
# distinct from having no recent works at all.
welcome-no-matches = No recent work matches your search.
welcome-learn-soon = Guides and tips are coming soon.
welcome-about-blurb = Skribisto, a Rust + Teksilo rewrite of the writing app.
# The *…* is inline markup, not decoration: it italicizes the line (the widget
# renders it in a serif italic). Keep the asterisks when translating.
welcome-tagline = *A quiet place to write long things.*
# Tooltip and screen-reader name of the two icon links under the sidebar nav.
# Product names: keep them as they are.
welcome-github = GitHub
welcome-discord = Discord
nav-works = Works
nav-examples = Examples
nav-learn = Learn
nav-about = About

## Editor tabs & panes
synopsis = Synopsis
epigraph = Epigraph
pane-manuscript = Manuscript
corkboard = Corkboard
overview = Overview

## Corkboard
corkboard-card-count = { $count ->
    [one] { $count } card
   *[other] { $count } cards
}
corkboard-child-count = { $count ->
    [one] { $count } item
   *[other] { $count } items
}
corkboard-view-nested = Nested
corkboard-view-flat = Flat
corkboard-layout-hint = Nested shows a container's direct children — open a folder card to go inside it. Flat shows every scene in the whole container at once.
corkboard-card-size = Card size
corkboard-search-placeholder = Filter cards…
corkboard-empty-title = Nothing here yet
corkboard-empty-hint = Use “＋ New” above to add the first one.
corkboard-new = New
corkboard-show-card-numbers = Number the cards
corkboard-modal-size = Expanded editor size
corkboard-scope-hint = Applies to every open corkboard, in every project.
corkboard-sort-manuscript = Manuscript order
corkboard-sort-title-asc = Title A–Z
corkboard-sort-title-desc = Title Z–A
corkboard-card-number = Card { $number }

## Corkboard batch actions (the card kebab acts on the whole selection)
duplicate = Duplicate
duplicate-n = { $count ->
    [one] Duplicate { $count } card
   *[other] Duplicate { $count } cards
}
set-label-n = { $count ->
    [one] Set label on { $count } card
   *[other] Set label on { $count } cards
}
move-to-trash-n = { $count ->
    [one] Move { $count } card to trash
   *[other] Move { $count } cards to trash
}
reveal-in-outline = Reveal in outline

## Corkboard “Move to…” destination picker
corkboard-move-to = Move to…
corkboard-move-to-n = { $count ->
    [one] Move { $count } card to…
   *[other] Move { $count } cards to…
}
corkboard-move-picker-title = Move to…
corkboard-move-picker-empty = No binders yet — create one first.
corkboard-move-picker-cancel = Cancel
corkboard-move-here = Move Here
corkboard-moved-ok = { $count ->
    [one] { $count } card moved
   *[other] { $count } cards moved
}
corkboard-move-into-self = A container cannot be moved inside itself. Pick a destination outside it.
corkboard-move-failed = Those cards could not be moved there.

## Overview (the container's contents as a sortable table)
overview-col-title = Title
overview-col-type = Type
overview-col-label = Label
overview-col-tags = Tags
# Only ever built when the Work has two or more Books: see `overview_columns`'s
# own gate, the same one every Books surface in this edition shares.
overview-col-books = Books
overview-col-own-words = Words
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] { $count } row
   *[other] { $count } rows
}
overview-search-placeholder = Filter rows…
overview-expand-all = Expand all
overview-collapse-all = Collapse all
overview-table-label = Contents
overview-empty-title = Nothing here yet
overview-gone-title = This container is gone
overview-gone-hint = It was moved to the Trash. Restore it, or close this tab.
overview-empty-hint = Use “＋ New” above to add the first one.
corkboard-grid-label = Corkboard cards
corkboard-rename-field = Rename item
corkboard-expand-synopsis = Expand synopsis
corkboard-synopsis-modal-title = Synopsis
corkboard-badge-scene = Scene
corkboard-badge-chapter = Chapter
corkboard-badge-part = Part
corkboard-badge-book = Book
corkboard-badge-note = Note
corkboard-badge-folder = Folder
corkboard-badge-paratext = Paratext
corkboard-badge-text = Text
corkboard-badge-end = End
settings-group-corkboard-layout = Layout
settings-group-corkboard-cards = Cards
corkboard-show-word-count = Show word count on cards
text-heading = Text
no-content = This item has no editable content.
untitled = Untitled
placeholder-title = Title…
placeholder-subtitle = Subtitle…
placeholder-chapter-title = Chapter title…
split-editor = Split editor
close-split-view = Close split view
drop-open-here = Open here
drop-open-to-side = Open to the side

## Manuscript streams (Full Chapter / Part / Book + Full Synopsis)
# The container's own page. "Chapter" is this chapter; "Full Chapter" is this chapter
# and every scene in it.
segment-chapter = Chapter
segment-part = Part
segment-book = Book
segment-pace = Pace
segment-notes = Notes
segment-story-bible = Story bible
# The three segments on an `Item/Note` tab: its own prose, its story-bible fields, and
# (only when it carries a discoverable tag) the manuscript prose it has been declared
# present in. See `teksilo_ui::tabs::item_note`.
segment-note-own = Note
segment-note-details = Details
segment-note-in-prose = In prose
pace-placeholder = The Pace planner appears here.
# Pace planner
pace-empty-title = Plan this book's pace
pace-empty-body = Set a word goal and a deadline, and Skribisto works out the daily pace to get you there.
pace-start-planning = Start planning
pace-section-schedule = Schedule
pace-section-progress = Progress
pace-advancement = Advancement
pace-goal = Word goal
pace-deadline = Deadline
pace-active = Pace active
pace-day-mon = Mon
pace-day-tue = Tue
pace-day-wed = Wed
pace-day-thu = Thu
pace-day-fri = Fri
pace-day-sat = Sat
pace-day-sun = Sun
pace-card-written = words written
pace-card-of-goal = of the goal
pace-card-rate = words / writing day
pace-card-days-left = writing days left
pace-card-streak = day streak
pace-card-ahead = words ahead of pace
pace-card-behind = words behind pace
pace-charts-empty = Progress charts appear here once your word count is recorded on a save.
pace-chart-progression = Words written vs. target
pace-chart-words-per-day = Words per day
pace-series-actual = Actual
pace-series-target = Target
pace-series-words-per-day = Words/day
pace-daily-target-line = Even pace: { $count ->
    [one] { $count } word/day
   *[other] { $count } words/day
}
pace-section-holidays = Holidays
pace-section-milestones = Milestones
pace-holidays-none = No holidays. Every scheduled day counts.
pace-holiday-label = Holiday name
pace-add-holiday = Add
pace-remove = Remove
pace-milestones-none = No milestones yet. Set one on a Part or Chapter in the Inspector.
full-chapter = Full Chapter
full-part = Full Part
full-book = Full Book
full-synopsis = Full Synopsis
rename = Rename
set-label = Set label
insert-scene = Insert scene
insert-chapter = Insert chapter
split-scene = Split scene
move-up = Move up
move-down = Move down
merge-with-previous = Merge with previous
move-to-trash = Move to trash
rename-chapter = Rename chapter
add-scene = Add new scene
add-chapter = Add new chapter
new-scene-title = New Scene
placeholder-scene-name = Scene name…
menu-cut = Cut
menu-copy = Copy
menu-paste = Paste
menu-paste-unformatted = Paste Unformatted
menu-select-all = Select All
# Tooltips on the editor context menu's formatting row. Icon-only buttons, so
# the tooltip is their only accessible name — not decoration.
format-bold = Bold
format-italic = Italic
format-underline = Underline
format-strikethrough = Strikethrough

## Binder / recents
binder = Binder
no-work = No work loaded
no-recent-works = No recent works
switcher-open-section = Currently open
switcher-recent-section = Recent
switcher-this-window = this window
open-project-title = Open project
open-project-question = How do you want to open “{ $title }”?
open-in-new-window = Open in new window
open-here = Open here

## Binder switcher + search
binder-all = All Binders
binder-show-all = Show all binders
binder-new = New binder…
binder-item-count = { $count ->
    [one] { $count } item
   *[other] { $count } items
}
binder-search-placeholder = Filter the outline…
binder-search-scope = Search all binders
binder-trash-confirm-title = Move binder to trash?
binder-trash-confirm-text = “{ $name }” and all its items will be moved to the trash.

## Dialogs
dialog-rename = Rename
dialog-set-label = Set label
dialog-new-scene = New scene
close-work-question = Save changes before closing the work?
quit-question = Save changes before quitting?
quit-save-work-question = Save changes to { $title } before quitting?
unsaved-changes = This work has unsaved changes.
# Shown by Quit when another open Work (not this window's own) still has
# unsaved edits — the design's "one dialog listing every dirty Work" (see
# app::commands::file's `app.quit` action). Quit is refused until those are
# saved or closed from their own window.
# Replacing the open work in this window (New Work, Open Work, "Open here", the
# import toast's "Open now"), the same guard as closing, since the open work is
# closed either way.
new-work-unsaved-question = Save changes before creating a new work?
open-work-unsaved-question = Save changes before opening another work?
switch-backup-discard-title = Discard changes to this backup?
switch-backup-discard-text = Changes to a backup can't be saved to it. Use Save As or Restore to keep them, or discard them and open the other work.
switch-save-failed = The work couldn't be saved, so it wasn't replaced: { $error }
switch-save-not-started = The work couldn't be saved, so it wasn't replaced.
close-save-failed = The work couldn't be saved, so it wasn't closed: { $error }
close-save-not-started = The work couldn't be saved, so it wasn't closed.
save-not-started = The work couldn't be saved.

## Toasts
# Toast title. Kept short because a toast title is one line and truncates; the cause goes
# in the body, which is the untranslated error text itself. $file is the file's base name.
could-not-open-work = Could not open "{ $file }"
# A project saved by a newer Skribisto than this one — title and body.
# $written_by is the version that wrote it, $requires the lowest version that can open it
# (the two differ when the newer build happened to use nothing new), $supported the newest
# this build understands.
could-not-open-work-too-new = "{ $file }" needs a newer Skribisto
could-not-open-work-too-new-detail = Saved by Skribisto format { $written_by }; opening it needs format { $requires } or newer, and this build supports up to format { $supported }. Update Skribisto to open it.
could-not-open-example = Could not open example: { $error }
# The browser (or whatever handles http links) could not be started for one of
# the Welcome sidebar's links. $url is shown so the address can still be copied.
could-not-open-link = Could not open { $url }: { $error }
could-not-create-work = Could not create work: { $error }
saving-as-file = Saving as { $target }…
saving-as-folder = Saving as { $target }/…
saved-as = Saved to { $target }
save-error = Could not save: { $error }
backup-error = Could not back up: { $error }
backing-up = Backing up…
backup-nothing-open = No project is open to back up.
backup-already-running = A backup is already in progress.
backup-complete = Backup complete ({ $ok } saved, { $skipped } already current)
backup-partial = Backup finished: { $ok } saved, { $failed ->
    [one] { $failed } destination failed
   *[other] { $failed } destinations failed
}
backup-no-destination-title = No backup location available
backup-no-destination-text = None of the configured backup destinations can be reached (for example, an external drive may be unplugged). Plug it in and retry, or exit without backing up.
backup-failed-close-title = The backup could not be saved
backup-failed-close-text = No backup copy could be written before closing. Every destination failed (the drive may have been removed, or it may be full or write-protected). Fix it and retry, or exit without backing up.

## Open-a-backup (choice modal + permanent banner + restore)
backup-choice-title = Backup file
backup-choice-heading = You've opened a backup copy
backup-choice-subtitle = This is a point-in-time backup of a project.
backup-choice-subtitle-dated = Backup taken { $date }.
backup-choice-body = You can open it and edit freely, but changes can only be kept with Save As. The original project file is not touched. Or restore this project to exactly this backup.
backup-choice-open = Open the backup
backup-choice-restore = Restore this project to this point…
backup-choice-not-a-backup = No, open it normally
backup-banner-title = Backup copy: changes can't be saved here
backup-banner-description = Use Save As to keep your edits in a new file, or Restore to replace the original project with this backup.
backup-banner-restore = Restore…
backup-banner-save-as = Save As…
backup-restore-original-missing = Can't find the original project to restore over. Use Save As to keep this backup as a new project instead.
backup-restore-close-elsewhere-title = Project open in another window
backup-restore-close-elsewhere-text = The project you're restoring over is open in another window. Please close it there first, then retry.
backup-restore-focus-window = Focus that window
backup-restore-confirm-title = Restore this backup?
backup-restore-confirm-text = The current version of the project will be copied aside as a safety backup before being replaced with this one.
backup-restore-confirm-ok = Restore
backup-restore-error = Could not restore: { $error }
backup-restored-ok = Project restored.
backup-restored-with-safety = Project restored. Your previous version was saved to { $path }.
close-backup-discard-title = Discard changes to this backup?
close-backup-discard-text = Changes to a backup can't be saved to it. Use Save As to keep them, or discard and close.
quit-backup-discard-title = Discard changes and quit?
quit-backup-discard-work-question = Discard changes to { $title } and quit?
quit-backup-discard-text = Changes to a backup can't be saved to it. Use Save As to keep them, or discard and quit.
backup-nudge-text = No backups are set up for this project.
backup-nudge-action = Set up backups…

## Backups list panel
menu-backups-list = Bac&kups…
backups-title = Backups
backups-loading = Scanning for backups…
backups-empty = No backups found for this project yet.
backups-open = Open
backups-reveal = Reveal
backups-delete = Delete this backup
backups-delete-confirm-title = Delete this backup?
backups-delete-confirm-text = “{ $name }” will be permanently deleted. This can't be undone.
backups-delete-error = Could not delete the backup: { $error }
backups-refresh = Refresh
backups-close = Close

## Backup settings panes
settings-page-backup = Backup defaults
settings-page-work-backup = Backups
settings-backup-general-title = Default backup settings
settings-backup-work-title = Backups for this project
settings-backup-inherit = Use the general backup settings
settings-backup-inheriting = This project uses the general backup settings.
settings-backup-none-hint = No automatic backups are set up (every trigger is off).
settings-backup-last = Last backup: { $date }
settings-backup-last-never = No backups yet.
settings-backup-open-list = Open backups list…
settings-backup-triggers = When to back up
settings-backup-on-close = Back up when closing the project
settings-backup-on-open = Back up when opening the project
settings-backup-interval = Back up periodically, every
settings-backup-destinations = Backup destinations
settings-backup-default-location = Default location
settings-backup-usage-since = { $count ->
    [one] { $count } backup · { $size } · oldest { $oldest }
   *[other] { $count } backups · { $size } · oldest { $oldest }
}
settings-backup-usage = { $count ->
    [one] { $count } backup · { $size }
   *[other] { $count } backups · { $size }
}
settings-backup-usage-empty = Nothing kept here yet
settings-backup-usage-measuring = Measuring…
settings-backup-reveal-root = Show folder
settings-backup-dest-none = No destinations. Backups are saved next to the project.
settings-backup-dest-remove = Remove
settings-backup-dest-add = Add folder…
settings-backup-dest-refresh = Refresh
settings-backup-retention = How many to keep
settings-backup-retention-tiered = Tiered
settings-backup-retention-keep-n = Keep last N
settings-backup-retention-tip = How old backups are pruned.
settings-backup-retention-tip-more = Tiered keeps one backup per hour for a day, per day for a week, per week for a month, and per month beyond, so recent history stays dense and old history thins out. “Keep last N” simply keeps the N most recent copies. In both modes the newest copies (the minimum below) are always kept.
settings-backup-gfs-hourly = Hourly (last 24 h)
settings-backup-gfs-daily = Daily (last week)
settings-backup-gfs-weekly = Weekly (last month)
settings-backup-gfs-monthly = Monthly
settings-backup-keep-n = Number to keep
settings-backup-min-keep = Always keep at least
settings-backup-dedup = Skip a backup when nothing has changed

## New-work dialog
new-work-title = New Work
new-work-close = Close
new-work-name = Work name
new-work-name-placeholder = Untitled
new-work-author = Author
new-work-author-placeholder = Optional
new-work-format = Format
new-work-single-file = Single file
new-work-bundle = Bundle
new-work-convert-later = You can convert between formats later.
new-work-location = Location
new-work-will-create = Will create
new-work-language = Default language
new-work-language-hint = Applied to new texts & spellcheck. Each text can be switched to another language.
new-work-template = Template
new-work-template-none = None
new-work-template-empty-novel = Empty Novel
new-work-template-light-novel = Light Novel
new-work-template-novel = Novel
new-work-template-novel-in-parts = Novel in parts
new-work-template-notebook = Notebook
new-work-cancel = Cancel
new-work-create = Create Work
# Wizard steps & navigation
new-work-step-details = Details
new-work-step-language = Language & structure
new-work-step-template = Template
new-work-back = Back
new-work-next = Next
# The same wizard, opened from the Launcher's "From documents…"
new-work-documents-title = New Work from documents
new-work-step-import = Import
new-work-create-and-import = Create & import…
new-work-documents-next-title = Your documents come next
new-work-documents-next-body = Creating the project opens the import wizard over it, where you choose the documents, review the structure Skribisto reads from them, and pick where it lands. Nothing is written into the project until you confirm it there.
new-work-documents-no-template = This project starts empty on purpose: no template, so the imported documents are the only thing in it.
new-work-documents-chapter-scene-hint = Applies to the chapters the import creates. You can change it later in the project's settings.
# Format tile descriptions
new-work-single-file-desc = One .skrib archive (zip). Portable, easy to back up.
new-work-bundle-desc = A folder holding every text & asset. Friendlier to version control.
# Template row trailing counts
new-work-template-none-count = empty binder
new-work-template-empty-novel-count = 1 chapter
new-work-template-light-novel-count = 15 chapters
new-work-template-novel-count = 20 chapters
new-work-template-novel-in-parts-count = 3 parts, 24 chapters
new-work-template-notebook-count = free-form notes
# ChapterScene toggle (novel templates)
new-work-chapter-scene = Flat chapters
new-work-chapter-scene-tip = Each chapter is a single row you write straight into, with no scenes under it. Leave this off for the classic layout, where a chapter is a folder: you write straight into that too, but it can hold scenes as well.
new-work-chapter-scene-tip-more = You write into a chapter either way. The only difference is whether it can *contain* scenes. Skribisto's binder tree is organizational only, so both layouts compile to the same book, you can mix them freely, and Promote converts a chapter between the two without losing a word.
# Field validation
new-work-name-required = Enter a name for the work
new-work-name-invalid = This name has no usable characters
new-work-location-required = Choose a location
new-work-location-missing = This folder does not exist
new-work-location-not-folder = This path is not a folder
new-work-location-readonly = This folder is not writable

## New-work template labels (passed to the backend, which can't do i18n)
new-work-manuscript = Manuscript
new-work-notes = Notes
new-work-research = Research
new-work-notebook = Notebook
new-work-scene = Scene
new-work-note = Note
new-work-front-matter = Front matter
new-work-back-matter = Back matter
new-work-paratext = Book structure
new-work-paratext-none = No structure
new-work-paratext-hint = The front and back matter a tradition opens and closes a book with. You can move, rename or delete any of it afterwards.
new-work-tags = Tags
new-work-tags-none = No tags
new-work-tags-hint = A starting palette for tagging characters, places and the rest. Optional, and every tag can be renamed, recolored or deleted afterwards.
new-work-note-templates = Note templates
new-work-note-templates-none = No templates
new-work-note-templates-hint = The shape a story-bible note starts in. Optional, and every template can be edited or deleted afterwards.
new-work-characters = Characters
new-work-places = Places

## Import Plume Creator dialog
import-plume-title = Import Plume Creator project
import-plume-close = Close
import-plume-source = Plume project
import-plume-source-hint = Choose a .plume or .plume_backup file (any Plume Creator version).
import-plume-location = Destination folder
import-plume-name = File name
import-plume-name-placeholder = Project name
import-plume-will-create = Will create
import-plume-trash-warning = ⚠ Trashed / deleted items are not migrated.
import-plume-cancel = Cancel
import-plume-import = Import
# Field validation
import-plume-source-required = Choose a Plume project file
import-plume-source-missing = This file does not exist
import-plume-source-not-file = This path is not a file
import-plume-location-required = Choose a destination folder
import-plume-location-missing = This folder does not exist
import-plume-location-not-folder = This path is not a folder
import-plume-location-readonly = This folder is not writable
import-plume-name-required = Enter a file name
import-plume-name-exists = A file with this name already exists here. Import will confirm overwrite
# Overwrite confirmation
import-plume-overwrite-title = Replace existing file?
import-plume-overwrite-text = “{ $name }” already exists. Replace it with the imported project?
# Binder names passed to the backend (which can't do i18n)
import-plume-manuscript-binder = Manuscript
import-plume-story-bible-binder = Story Bible
# Progress toast (the import runs as a long operation)
import-plume-progress-title = Importing Plume project…
import-plume-cancel-import = Cancel
import-plume-cancelled = Import canceled
# Result
import-plume-done = Imported { $imported ->
    [one] { $imported } item.
   *[other] { $imported } items.
} { $skipped ->
    [one] { $skipped } trashed item was not migrated.
   *[other] { $skipped } trashed items were not migrated.
}
import-plume-open-now = Open now
# Shown when the importer could not carry something over verbatim.
import-plume-warnings = { $count ->
    [one] 1 thing could not be imported exactly
   *[other] { $count } things could not be imported exactly
}
import-plume-details = Details
import-plume-warnings-title = Import warnings
# Error toast: a short reason in the body, the full technical chain behind Details
import-plume-error-title = Could not import the project
import-plume-error-details = Details

## Import Manuskript project dialog
import-manuskript-title = Import Manuskript project
import-manuskript-close = Close
import-manuskript-source = Manuskript project
# A Manuskript project is a .msk file plus, in its usual mode, a folder of the
# same name beside it. Either one gets you there, and so does the folder itself.
import-manuskript-source-hint = Choose the .msk file or the project folder (any Manuskript version).
import-manuskript-source-file = Choose file…
import-manuskript-source-folder = Choose folder…
import-manuskript-location = Destination folder
import-manuskript-name = File name
import-manuskript-name-placeholder = Project name
import-manuskript-will-create = Will create
import-manuskript-cancel = Cancel
import-manuskript-import = Import
# Field validation
import-manuskript-source-required = Choose a Manuskript project
import-manuskript-source-missing = This file or folder does not exist
import-manuskript-location-required = Choose a destination folder
import-manuskript-location-missing = This folder does not exist
import-manuskript-location-not-folder = This path is not a folder
import-manuskript-location-readonly = This folder is not writable
import-manuskript-name-required = Enter a file name
import-manuskript-name-exists = A file with this name already exists here. Import will confirm overwrite
# Overwrite confirmation
import-manuskript-overwrite-title = Replace existing file?
import-manuskript-overwrite-text = “{ $name }” already exists. Replace it with the imported project?
# Names passed to the backend (which can't do i18n). Manuskript stores none of
# these: it has no binders, no story-bible groups, and its importance scale is
# three numbers whose names live in its own interface.
import-manuskript-manuscript-binder = Manuscript
import-manuskript-story-bible-binder = Story bible
import-manuskript-characters-group = Characters
import-manuskript-world-group = World
import-manuskript-plots-group = Plots
import-manuskript-project-info-note = Project information
import-manuskript-summary-note = Summary
import-manuskript-importance-minor = Minor
import-manuskript-importance-secondary = Secondary
import-manuskript-importance-main = Main
# Progress toast (the import is a long operation)
import-manuskript-progress-title = Importing Manuskript project…
import-manuskript-cancel-import = Cancel
import-manuskript-cancelled = Import canceled
# Result
import-manuskript-done = { $imported ->
    [one] { $imported } item imported.
   *[other] { $imported } items imported.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } earlier version came with it.
   *[other] { $revisions } earlier versions came with them.
}
import-manuskript-open-now = Open now
# Shown when the importer could not carry everything across as it was.
import-manuskript-warnings = { $count ->
    [one] 1 thing to know about this import
   *[other] { $count } things to know about this import
}
import-manuskript-details = Details
import-manuskript-warnings-title = About this import
# Error toast: short reason in the body, full technical chain behind “Details”
import-manuskript-error-title = Could not import the project
import-manuskript-error-details = Details

## Export dialog
export-title = Export
export-close = Close
export-scope-label = What
export-format-label = Format
export-style-label = Style
export-path-label = File
export-browse = Browse…
export-preview-label = Preview
export-preview-empty = Nothing to preview for this selection.
export-show-non-exportable = Show non-exportable
export-choose-empty = No project to choose from.
export-cancel = Cancel
export-export = Export
export-save-dialog-title = Export to file
# Left-pane section headers (normal case) + scope segmented control
export-section-what = What to export
export-section-style = Style
export-section-destination = Destination
export-custom-selection = Custom selection
export-selected-count = { $count } selected
# Live-preview header
export-preview-compiled = compiled
export-preview-live = Live preview
# Style-preset summary chips
export-chip-chapters-none = Chapters: none
export-chip-chapters-numbered = Chapters: numbered
export-chip-chapters-title = Chapters: title only
export-chip-chapters-both = Chapters: number + title
export-chip-scene-break-glyph = Scene break: { $glyph }
export-chip-scene-break-blank = Scene break: blank line
export-chip-scene-break-none = Scene break: none
export-chip-major-break-glyph = Major break { $glyph }
export-chip-major-break-blank = Major break: blank line
export-chip-major-break-none = Major break: none
export-chip-major-break-same = Both break tiers alike
export-chip-spacing-single = Spacing: single
export-chip-spacing-onehalf = Spacing: 1½
export-chip-spacing-double = Spacing: double
export-chip-notes-included = Notes included
export-chip-notes-excluded = Notes excluded
# Output formats
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Text
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
# Overwrite confirmation
export-overwrite-title = Replace existing file?
export-overwrite-text = “{ $name }” already exists. Replace it?
# Progress toast (the export runs as a long operation)
export-progress-title = Exporting…
export-cancelled = Export canceled
export-done = Exported { $count ->
    [one] { $count } item
   *[other] { $count } items
}
export-open-file = Open
export-show-in-folder = Show in folder
# Error toast: a short reason in the body, the full technical chain behind Details
export-error-title = Could not export
export-error-details = Details

## Backup scheduler (progress toast + failure/prune-warning details, backup review, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Starting…
backup-progress-retention = Cleaning up old backups…
backup-progress-done = Done
backup-progress-destination = Destination { $i } of { $n }
backup-details = Details
backup-issues-title = Backup issues
backup-failed-title = Backup failed
backup-complete-prune-warning = Backup complete ({ $ok } saved, { $skipped } already current). Some old backups could not be removed

# Search & Replace
search = Search
search-query-placeholder = Search…
search-replace-placeholder = Replace with…
search-replace-toggle = Toggle replace
search-replace-all = Replace All
search-opt-case = Match case
search-opt-whole-word = Whole word
search-opt-diacritics = Match accents
search-scope-body = Body
search-scope-title = Title
search-scope-synopsis = Synopsis
search-scope-label = Label
search-facet-book = Books
search-facet-part = Parts
search-facet-chapter = Chapters
search-facet-scene = Scenes
search-facet-note = Notes
search-facet-folder = Folders
# Rich tooltips for the option toggles
search-tip-case = Match case: treat uppercase and lowercase as different, so “Elena” and “elena” are separate matches.
search-tip-whole-word = Whole word: match only complete words, so “cat” is not found inside “category”.
search-tip-diacritics = Match accents: treat accented letters as distinct, so “cafe” does not match “café”.
search-tip-body = Body: search the prose of scenes and notes.
search-tip-title = Title: search the titles of binder items.
search-tip-synopsis = Synopsis: search each writing row’s summary.
search-tip-label = Label: search the short note shown under a binder item’s title.
search-tip-comment = Comments: search the text of comment threads and their replies. A comment is about the manuscript rather than part of it, so it has its own switch — and a replace leaves comment matches unticked until you tick them.
search-tip-book = Books: the book container and its begin / end markers.
search-tip-part = Parts: part-level dividers.
search-tip-chapter = Chapters: chapters, in whichever way the project stores them.
search-tip-scene = Scenes: the rows that hold your prose.
search-tip-note = Notes: free-form notes.
search-tip-folder = Folders: plain organizing folders and separators.
search-tip-paratext = A text that belongs to the book but not to its story — a preface, a dedication, an afterword. Never counted in the manuscript.
search-tip-preserve-case = Preserve case: a replacement takes the case it found, so “ELENA” becomes “MARTA” and “Elena” becomes “Marta”.
search-tip-replace = Replace: show the replacement field and Replace All.
search-error = Search failed: { $message }
search-no-matches = No matches
search-count =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } matches
    } in { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } match
       *[other] { $matches } matches
    } in { $items ->
        [one] { $items } document
       *[other] { $items } documents
    } (first results only)
search-occurrences = { $count }
search-field-body = Body
search-field-title = Title
search-field-synopsis = Synopsis
search-field-label = Label
search-field-epigraph = Epigraph
search-field-comment = Comment
search-field-comment-reply = Reply
search-field-footnote = Footnote
search-include-in-replace = Include in Replace All
search-collapse-all = Collapse all results
search-replace-here = Replace this
search-dismiss = Dismiss from the results
search-undo-dismiss = Bring back the last dismissed result
search-replace-nothing = (nothing)
search-replace-confirm-title = Replace all matches?
search-replace-confirm-text =
    Replace { $occurrences ->
        [one] { $occurrences } occurrence
       *[other] { $occurrences } occurrences
    } of "{ $query }" with "{ $replacement }" across { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }? You can undo this from the notification afterwards.
search-replace-done-title = Replace complete
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } occurrence
       *[other] { $occurrences } occurrences
    } replaced in { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } occurrence
       *[other] { $occurrences } occurrences
    } replaced in { $items ->
        [one] { $items } document
       *[other] { $items } documents
    }. { $skipped ->
        [one] { $skipped } field
       *[other] { $skipped } fields
    } skipped (changed since the search).
search-replace-undo = Undo
search-replace-failed-title = Replace failed
search-replace-skipped-title = Nothing replaced
search-replace-skipped-body =
    { $fields ->
        [one] This text
       *[other] { $fields } of these texts
    } changed since the search, so { $fields ->
        [one] it was
       *[other] they were
    } left alone. Search again to see where the words are now.
search-replace-undo-failed-title = Undo failed
search-preview = Preview
search-preview-empty = Select a result to preview it here
search-preview-no-prose = This match has no editable text
search-preview-prompt = To see a preview here, run a search.
search-preview-open-search = Search in Project
search-preview-footnote-prompt = This match is in a footnote's own text — open it in the Footnotes dock to see and edit it.
search-preview-open-footnotes = Open Footnotes

# Per-editor find banner (Ctrl+F)
find-placeholder = Find in this document…
find-count = { $current } of { $total }
find-no-results = No results
find-close = Close find
find-previous = Previous match (Shift+Enter)
find-next = Next match (Enter)
find-opt-case = Match case
find-opt-whole-word = Whole word
find-replace-toggle = Toggle replace (Ctrl+R)
find-replace-placeholder = Replace with…
find-replace = Replace
find-replace-all = Replace all
find-preserve-case = Preserve case

## Dictionaries & spell-checking
# Download toasts (app-local download, not a backend long operation)
dict-download-title = Downloading { $name }…
dict-download-done = Installed { $name }
dict-download-failed = Couldn't download { $name }: { $error }
dict-removed = Removed { $name }
dict-accept-first = Accept the license before downloading { $name }
# Settings ▸ Dictionaries pane
settings-dict-tab-installed = Installed
settings-dict-tab-get-more = Get more
settings-dict-tab-personal = Personal words
dict-installed-empty = No dictionaries found on this computer yet.
dict-get-more-search = Search languages
dict-system-badge = on your system
dict-unusable-badge = unusable
dict-download-button = Download
dict-downloading = Downloading…
dict-installed-label = Installed
dict-remove = Remove
dict-view-license = View license
dict-approx-size = ~{ $size }
dict-personal-empty = No personal words in this project yet.
dict-personal-add = Add
dict-personal-placeholder = Add a word…
# Missing-dictionary prompt after opening a project
dict-missing-toast = { $count ->
    [one] This project uses { $count } dictionary you don't have installed
   *[other] This project uses { $count } dictionaries you don't have installed
}
dict-missing-action = Get dictionaries
# License modal
dict-license-title = { $name } license
dict-license-accept = Accept & Download
dict-license-cancel = Cancel
dict-license-close = Close
# Add a dictionary (sideload local .aff/.dic files)
dict-add-button = Add dictionary…
dict-add-title = Add a dictionary
dict-add-close = Close
dict-add-name = Name
dict-add-name-placeholder = e.g. My Latin dictionary
dict-add-code = Language code
dict-add-code-placeholder = e.g. la or fr-FR-x-custom
dict-add-code-hint = A short tag of your choosing. This is what you pick as a document's language.
dict-add-aff = Affix file (.aff)
dict-add-dic = Word list (.dic)
dict-add-submit = Add
dict-add-cancel = Cancel
dict-add-name-required = Give the dictionary a name.
dict-add-code-required = Enter a language code.
dict-add-code-invalid = Use only letters, digits, and - _ .
dict-add-code-reserved = That is a built-in dictionary code. Choose another.
dict-add-file-required = Choose a file.
dict-add-file-missing = That file does not exist.
dict-add-code-taken = A dictionary for that code is already installed.
dict-add-done = Added { $name }
dict-add-unusable = These files are not a usable dictionary: { $error }
dict-add-failed = Couldn't add the dictionary: { $error }
# The Inspector's per-item export controls (M3)
inspector-export = Export
inspector-exportable = Include in exports
ctx-number = &Number this chapter
ctx-unnumber = Do not &number this chapter
inspector-numbering = Numbering
inspector-numbered = Numbered
inspector-numbered-tip = This chapter takes its place in the book's numbering. Switch it off for a prologue, an epilogue or an interlude.
inspector-numbered-tip-more = An unnumbered chapter stays in the book exactly as it was — its heading, its prose and its word count are untouched. It simply prints no number, and does not use one up: the chapter after a prologue is chapter one, not chapter two. Leaving it out of the export instead is a different switch, above, and that one removes the chapter from the book altogether.
inspector-apply-to-children = Apply to children
# The Inspector's per-Part/Chapter milestone date (M5), shown on the Book's Pace.
inspector-milestone = Milestone date
inspector-milestone-none = No date
inspector-milestone-clear = Clear
# The language pill field (Inspector + Settings)
inspector-tags = Tags
inspector-aliases = Also known as
inspector-dict-language = Language
inspector-apply-language-to-children = Apply language to children
settings-page-language = Language
settings-field-dict-language = Languages
dict-tradeoff-hint = Each extra language accepts more words, so fewer mistakes are caught.
lang-inherit-hint = Inherited: this scene uses the book's or project's languages.
lang-pill-list = Languages
lang-pill-add = Add a language
lang-pill-remove = Remove { $name }
lang-pill-mute = Turn spell-checking off for { $name }
lang-pill-unmute = Turn spell-checking on for { $name }

## Settings: Work ▸ Personal dictionary (per-project word list)
settings-page-personal-dictionary = Personal dictionary
settings-user-dict-desc = Words you add here are treated as correctly spelled everywhere in the app, and never flagged again.
settings-user-dict-add-placeholder = Add a word…
settings-user-dict-add = Add word
settings-user-dict-search = Filter words
settings-user-dict-count = { $count ->
    [one] { $count } word
   *[other] { $count } words
}
settings-user-dict-rename = Rename
settings-user-dict-remove = Remove
settings-user-dict-empty = No words yet. Add words the spell-checker should ignore, or import a list.
settings-user-dict-import = Import…
settings-user-dict-export = Export…
settings-user-dict-import-tip = Add words from a plain text file, one word per line. Existing words are kept.
settings-user-dict-export-tip = Save the whole list to a plain text file, one word per line.
settings-user-dict-txt-filter = Text files
settings-user-dict-duplicate = Already in the dictionary
settings-user-dict-imported = Imported { $count ->
    [one] { $count } word
   *[other] { $count } words
} ({ $duplicates } already present).
settings-user-dict-import-failed = Could not read the word list: { $error }
settings-user-dict-exported = Saved { $count ->
    [one] { $count } word.
   *[other] { $count } words.
}
settings-user-dict-export-failed = Could not save the word list: { $error }

## Settings: Work ▸ Text replacements (per-project custom lexicon)
settings-page-text-replacements = Text replacements
settings-text-repl-desc = Replace shorthand with the full text as you type: "btw" becomes "by the way" the moment you type a space or punctuation.
settings-text-repl-enable = Use text replacements in this project
settings-text-repl-disabled-hint = Turn this on to define shorthand that expands as you write.
settings-text-repl-add = Add rule
settings-text-repl-trigger-placeholder = Shorthand
settings-text-repl-replacement-placeholder = What it becomes
settings-text-repl-added = Added "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" already has a rule
settings-text-repl-filter = Filter rules
settings-text-repl-count = { $n ->
    [one] 1 rule
   *[other] { $n } rules
}
settings-text-repl-row-enabled = Use this rule
settings-text-repl-delete = Delete the rule for { $trigger }
settings-text-repl-deleted = Deleted the rule for "{ $trigger }"
settings-text-repl-empty = No rules yet.
settings-text-repl-csv-filter = CSV files
settings-text-repl-import = Import…
settings-text-repl-export = Export…
settings-text-repl-imported = Imported { $added }, skipped { $skipped }
settings-text-repl-exported = Exported { $n ->
    [one] 1 rule
   *[other] { $n } rules
}

## Editor: spelling (context menu + toast)
# Shown in place of the corrections when a flagged word has none to offer.
editor-menu-no-suggestions = No suggestions
editor-menu-add-to-dictionary = Add “{ $word }” to dictionary
editor-menu-add-words-to-dictionary = Add selected words to dictionary
editor-dict-added = Added “{ $word }” to your dictionary.
editor-dict-added-multi = { $count ->
    [one] Added { $count } word to your dictionary.
   *[other] Added { $count } words to your dictionary.
}
toast-undo = Undo

## Taking one operation back
##
## The Undo on a notification is offered for one specific thing. If anything
## else has happened since, it says so rather than taking back whatever now
## happens to be last.
undo-superseded-title = That step is no longer the last one
undo-superseded-body = Something else has changed in this project since. Undo would take that back instead, so it hasn’t. Nothing this step did has been reversed — use Edit ▸ Undo to step back through the history yourself.
undo-failed = Undo failed: { $error }

## The Edit menu
##
## The Undo row names what it will take back, because on a stack where prose and
## structure both appear a bare "Undo" leaves the writer guessing whether the
## next press retypes a word or resurrects a chapter.
menu-edit = &Edit
menu-edit-undo = &Undo
menu-edit-redo = &Redo
menu-edit-undo-target = &Undo { $target }
menu-edit-redo-target = &Redo { $target }
shortcut-name-edit-undo = Undo
shortcut-name-edit-redo = Redo
undo-target-typing = typing
undo-target-project = the last change to this project
prose-history-reset-title = Typing history reset
prose-history-reset-body = { $count ->
    [one] One open scene was restored from an earlier state, so its typing history no longer applies.
   *[other] { $count } open scenes were restored from an earlier state, so their typing history no longer applies.
}
undo-target-trash = moving to the trash
undo-target-restore = restoring from the trash
undo-target-delete-forever = deleting for good
undo-target-replace-all = replacing across the project
undo-target-import = the document import
undo-target-duplicate = duplicating
undo-target-move = moving
undo-target-merge = merging two scenes
undo-target-split = splitting a scene
undo-target-promote = changing the type
undo-target-tidy-titles = tidying the chapter titles
undo-target-import-tags = importing labels
undo-target-import-templates = importing note templates
undo-target-create = creating
undo-target-remove = deleting
undo-target-rename = renaming
undo-target-edit = that edit
menu-edit-find = &Find…
menu-edit-find-next = Find &next
menu-edit-find-prev = Find pre&vious
menu-edit-replace = Find and rep&lace…
menu-edit-find-in-project = Find in pro&ject…
menu-edit-replace-in-project = Replace &in project…
undo-frozen = Undo (“Always forward” is on)
redo-frozen = Redo (“Always forward” is on)
menu-edit-cut = Cu&t
menu-edit-copy = &Copy
menu-edit-paste = &Paste
menu-edit-paste-plain = Paste &without formatting
menu-edit-select-all = Select &all
shortcut-name-edit-cut = Cut
shortcut-name-edit-copy = Copy
shortcut-name-edit-paste = Paste
shortcut-name-edit-paste-plain = Paste without formatting
shortcut-name-edit-select-all = Select all

## Spell-check: the master switch (title bar / View menu / F7 / Settings ▸ Spelling)
titlebar-spellcheck-on = Spell-checking is on. Click to stop checking (F7)
titlebar-spellcheck-off = Spell-checking is off. Click to check again (F7)
menu-spellcheck = &Check spelling
menu-comments = Co&mments
settings-page-spellcheck = Spell-checking
settings-group-spellcheck = Spell-checking
settings-spellcheck-enabled = Check spelling as I write
settings-spellcheck-hint = Underlines words no installed dictionary knows. Turning this off stops all checking, in every project, until you turn it back on. To stop checking one language only, clear its check mark on the Language field of the work or of an item.

## Trash panel
menu-trash = &Trash
trash-title = Trash
trash-empty-state = Trash is empty.
trash-empty-button = Empty Trash…
trash-restore = &Restore
trash-restore-to = Restore &to…
trash-delete-forever = &Delete Forever
trash-restored-ok = { $count } restored.
trash-restore-error = Could not restore: { $error }
trash-restore-orphaned = This item's original spot is gone — choose where to restore it.
trash-restore-no-project = No project is open, so there is nothing to restore.
trash-delete-no-project = No project is open, so there is nothing to delete.
trash-empty-confirm-title = Empty the trash?
trash-empty-confirm-text = { $count ->
    [one] { $count } trashed entry will be permanently deleted.
   *[other] { $count } trashed entries will be permanently deleted.
} They leave the trash for good. Undo can still bring them back until you do something else.
trash-emptied-title = Trash emptied
trash-emptied-body = Everything in the trash was permanently deleted.
trash-empty-no-project = No project is open, so there is no trash to empty.
trash-delete-forever-confirm-title = Delete forever?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } item will be permanently deleted.
   *[other] { $count } items will be permanently deleted.
} They leave the trash for good. Undo can still bring them back until you do something else.
trash-deleted-title = Deleted forever
trash-deleted-body = { $count } permanently deleted.
trash-undo = Undo
trash-restore-picker-title = Restore to…
trash-restore-to-confirm-title = Restore here?
trash-restore-to-confirm-text = Restore "{ $item }" into "{ $destination }"?
trash-restore-picker-restore-here = Restore Here
trash-restore-picker-cancel = Cancel
trash-restore-picker-empty = No binders yet — create one first.
trash-banner-title = This item is in the Trash
trash-banner-description = It won't appear in the outline or exports until you restore it.
trash-banner-restore = Restore…
trash-tab-tooltip = In Trash

# Composite tooltip: the full parameter sheet shown on an export-style row.
settings-styles-sheet-yes = Yes
settings-styles-sheet-no = No
settings-styles-sheet-font = Font
settings-styles-sheet-indent = First-line indent
settings-styles-sheet-para-spacing = Paragraph spacing
settings-styles-sheet-page = Page size
settings-styles-sheet-margins = Margins (T/R/B/L)
settings-styles-sheet-title-page = Book title page
settings-styles-sheet-heading-language = Heading language
settings-styles-sheet-auto = Follows the text
settings-styles-sheet-digits = Numerals
settings-styles-sheet-direction = Text direction
settings-styles-sheet-formats = Formats
settings-styles-sheet-all-formats = All

settings-styles-editor-group = Style editor
settings-styles-editor-missing = That style is no longer available.

settings-styles-page-letter = Letter
settings-styles-digits-western = Western (0–9)
settings-styles-digits-eastern-arabic = Eastern Arabic (٠–٩)
settings-styles-direction-ltr = Left to right
settings-styles-direction-rtl = Right to left

## Format dock
format-dock-title = Format
format-panel-empty = Place your cursor in a scene, note, or synopsis to see formatting options.
# Group headers.
format-group-marks = Text
format-group-block = Paragraph
format-group-lists = Lists
format-group-tables = Table
format-group-breaks = Scene breaks
# Button tooltips. Icon-only buttons, so each tooltip is that button's only
# accessible name — not decoration.
format-superscript = Superscript
format-subscript = Subscript
format-link = Link…
# The Link command: a hyperlink in the prose. One command, three doors (the
# Format dock, the Format menu and Ctrl+K), so these strings are shared.
link-dialog-insert-title = Insert link
link-dialog-edit-title = Edit link
link-dialog-text-label = Text
link-dialog-text-placeholder = the words the reader sees
link-dialog-url-label = Links to
link-dialog-url-placeholder = example.com
link-dialog-insert = Insert
link-dialog-apply = Apply
link-dialog-cancel = Cancel
link-dialog-remove = Remove link
# Refused because a document could otherwise make a click launch a program.
link-scheme-refused = Only web and email links can be opened. { $url } was left alone.
menu-format-link = Li&nk…
format-clear = Clear formatting
format-heading = Heading level
format-heading-normal = Normal text
format-heading-1 = Heading 1
format-heading-2 = Heading 2
format-heading-3 = Heading 3
format-heading-4 = Heading 4
format-heading-5 = Heading 5
format-heading-6 = Heading 6
format-align-left = Align left
format-align-center = Center
format-direction-rtl = Right-to-left paragraph
format-blockquote = Blockquote
format-list-bullet = Bulleted list
format-list-numbered = Numbered list
format-indent = Indent
format-outdent = Outdent
format-table-insert = Insert table
format-table-row-above = Insert row above
format-table-row-below = Insert row below
format-table-col-before = Insert column before
format-table-col-after = Insert column after
format-table-row-delete = Delete row
format-table-col-delete = Delete column
format-table-remove = Remove table

## Menu: Format
menu-format-marks-bold = Bol&d
menu-format-marks-italic = &Italic
menu-format-marks-underline = &Underline
menu-format-marks-strike = &Strikethrough
menu-format-marks-superscript = Su&perscript
menu-format-marks-subscript = Su&bscript
menu-format-marks-clear = &Clear Formatting
menu-format-heading = &Heading
menu-format-heading-normal = &Normal Text
menu-format-heading-1 = Heading &1
menu-format-heading-2 = Heading &2
menu-format-heading-3 = Heading &3
menu-format-heading-4 = Heading &4
menu-format-heading-5 = Heading &5
menu-format-heading-6 = Heading &6
menu-format-alignment = &Alignment
menu-format-align-left = Align &Left
menu-format-align-center = &Center
menu-format-direction = Dir&ection
menu-format-direction-auto = &Automatic
menu-format-direction-ltr = &Left to Right
menu-format-direction-rtl = &Right to Left
menu-format-blockquote = Block&quote
menu-format-lists = &Lists
menu-format-list-bullet = &Bulleted List
menu-format-list-numbered = &Numbered List
menu-format-indent = &Indent
menu-format-outdent = &Outdent
menu-format-table = &Table
menu-format-table-insert = &Insert Table
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = Insert Row Abo&ve
menu-format-table-row-below = Insert Row Belo&w
menu-format-table-col-before = Insert Co&lumn Before
menu-format-table-col-after = Insert Column Af&ter
menu-format-table-row-delete = &Delete Row
menu-format-table-col-delete = Delete &Column
menu-format-table-remove = &Remove Table

## About panel

menu-about = &About Skribisto…
about-title = About Skribisto
about-version = Version { $version }
about-tagline = A novel-writing app for long-form fiction, written in Rust with the Teksilo toolkit.
about-license = Released under the GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Close

## Native (macOS) menu bar
# Labels for the platform-standard App and Window menus mirrored into the macOS
# global menu bar. No `&` mnemonics here: macOS has none, and the native bridge
# resolves these without stripping one, so an ampersand would print literally.
# The application name is data (the running edition's own name), so it arrives
# as an argument — same rule as the window titles below.
native-menu-about = About { $app }
native-menu-hide = Hide { $app }
native-menu-quit = Quit { $app }
native-menu-settings = Settings…
native-menu-window = Window
native-menu-minimize = Minimize
native-menu-zoom = Zoom

# Window titles. The application name is data (the running edition's own name),
# so it arrives as an argument rather than being written into the value here.
window-title = { $title } — { $app }
window-title-numbered = { $title } — { $app } (Window { $n })
window-title-empty = { $app }

# First-run settings import (an edition with its own config directory, finding
# the community installation's settings beside it).
first-run-window-title = Welcome
first-run-title = Set up { $app }
first-run-body = { $app } keeps its settings separately from Skribisto, so it starts out empty. Your preferences, recent projects, dictionaries and window layout can be copied across now.
first-run-from = Copy from
first-run-to = Copy to
first-run-copy-note = Nothing is moved or removed. Skribisto keeps every one of its own settings, and goes on working exactly as before.
first-run-import = Import settings
first-run-start-fresh = Start fresh
first-run-import-failed = Some settings could not be imported: { $error }

# ── Work ▸ Punctuation — the project's typographic house style ──────────────
settings-page-punctuation = Punctuation defaults
settings-group-punctuation = Smart punctuation
settings-page-work-punctuation = Punctuation
settings-punctuation-override = Give this project its own punctuation rules
settings-punctuation-override-hint = Off: the project follows the application preference. These rules travel inside the .skrib, so a co-author opening the file writes with the same typography.
settings-punctuation-dashes = Turn -- into an en dash, --- into an em dash
settings-punctuation-ellipsis = Turn ... into an ellipsis
settings-punctuation-quotes = Curl quotation marks and apostrophes
settings-quote-style = Quotation marks
settings-quote-style-locale = Language default
settings-quote-style-curly = “Double”
settings-quote-style-curly-single = ‘Single’
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = „Low-high“
settings-punctuation-spacing = Space before ; : ! ?
settings-punctuation-spacing-hint = French typography sets a narrow no-break space before ; ! ? and a full one before :. It applies only to text written in French. The space inside guillemets « » comes with the guillemet quotes themselves.
settings-punctuation-sample = Your language gives
settings-punctuation-app-hint = What every project does unless it takes rules of its own in Work ▸ Punctuation.
settings-punctuation-dialogue = Open a paragraph typed as "- " with a dialogue dash
settings-punctuation-dialogue-hint = For languages that mark speech with a dash rather than quotation marks — French, Spanish, Russian and others. It only fires at the very start of a paragraph.

## Go to (jump to any item)
statusbar-go-to = Go to…
go-to-placeholder = Search the binder
go-to-no-matches = No item matches that search.
menu-go-to = &Go to…

# ── Distraction-free themes (Settings ▸ Editor ▸ Distraction-free themes) ──
settings-page-distraction-free-themes = Distraction-free themes
settings-themes-builtin = Built-in themes
settings-themes-builtin-badge = Built-in
settings-themes-user = My themes
settings-themes-editor-group = Edit theme
settings-themes-editor-empty = Pick a theme under "My themes" to edit it.
settings-themes-use = Use
settings-themes-duplicate = Duplicate
settings-themes-edit = Edit
settings-themes-delete = Delete
settings-themes-export = Export…
settings-themes-import = Import…
settings-themes-copy-suffix = copy
settings-themes-json-filter = Theme (JSON)
settings-themes-imported = Theme imported
settings-themes-import-failed = Could not import that theme
settings-themes-exported = Theme exported
settings-themes-export-failed = Could not export that theme
# Shown on a theme whose text and page are below the WCAG AA contrast floor.
settings-themes-low-contrast = low contrast
# Shown when the page and the text are fine but the caret band is what hides
# the prose — a different fault, and one the row's own swatches do not show.
settings-themes-low-contrast-band = highlight hides the text
settings-themes-field-name = Name
settings-themes-field-paper = Page
settings-themes-field-ink = Text
settings-themes-field-general = Background
settings-themes-field-widget-text = Control strip text
# The shading drawn around the caret — the sentence or paragraph being written,
# per Editor ▸ "Highlight around the caret". This field is the color it uses in
# distraction-free mode.
settings-themes-field-caret-band = Highlight around the caret

# The distraction-free strip's quick-settings gear, and the way through to the
# full theme library from inside it.
statusbar-focus-settings = Distraction-free settings
statusbar-focus-manage-themes = Manage themes…

## Comments — the two docks, the thread cards, and their actions.
comments-title = Comments
comments-document-title = This document
comments-empty-project = No comments in this project yet.
comments-empty-document = No comments on this document.
comments-filter-all = All
comments-filter-open = Open
comments-filter-resolved = Resolved
# Always shown, including at zero: an orphan nobody looks for is an orphan nobody fixes.
comments-filter-orphaned = Orphaned
comments-status-open = Open
comments-status-resolved = Resolved
comments-status-orphaned = Lost its text
comments-orphan-snippet = (the commented text is gone)
# A comment that resolved successfully yet has no live range to point at — an
# import from a format with no text-position concept for it (a heading, a blank
# paragraph, a table). Distinct from "Lost its text": nothing went missing, it
# never had a position to begin with.
comments-status-unplaced = No text position
comments-unplaced-snippet = (not anchored to any text)
comments-reply-count = { $count ->
    [one] { $count } reply
   *[other] { $count } replies
}
comments-menu-resolve = Resolve
comments-menu-reopen = Reopen
comments-menu-delete = Delete
comments-sort-document = In document order
comments-sort-newest = Newest first
comments-menu-add = Add comment
comments-menu-add-paragraph = Comment on this paragraph
overview-col-comments = Comments
overview-col-total-comments = Total comments
comments-card-placeholder = Write a comment…
comments-card-unknown-author = Unknown author
comments-unsigned-toast = Your comments are unsigned — no name is set for you on this computer.
comments-unsigned-action = Set your name
comments-card-reply = Reply
comments-card-reply-placeholder = Reply…
comments-card-actions = Comment actions
comments-reply-actions = Reply actions
comments-menu-delete-reply = Delete reply
comments-menu-delete-all = Delete all comments here
comments-deleted-toast = Comment deleted
comments-reply-deleted-toast = Reply deleted
comments-deleted-all-toast = { $count ->
    [one] { $count } comment deleted
   *[other] { $count } comments deleted
}
comments-undo = Undo

# ── Analysis (Book container segment) ────────────────────────────────────────
# House rule for every string here: describe, never judge. No "too many", no
# "weak", no "should". Every comparison is against the book's own median or its
# own distribution — never a genre norm, and never a target value.
analysis-segment = Analysis
analysis-scope-book = Analyzing this book
analysis-run = Run analysis
analysis-stale = Changed since this ran
analysis-not-run = Not analyzed yet.
analysis-running = Reading the manuscript…
analysis-failed = The analysis could not finish.
analysis-no-scenes = No scenes in this book yet.

analysis-shape = Shape

# ── How the text arrived ──────────────────────────────────────────────────────
# A fact about input, never about authorship. Nothing here weighs one route
# against another, and there is no total to reach and nothing to score.
analysis-arrivals = Arrivals
analysis-arrivals-explainer = How text reached this project while it has been open. It says which route the characters came down, and nothing at all about who wrote them: a writer who drafts elsewhere and pastes has pasted, and one who dictates has dictated. There is no number to aim for here.
# The two scope caveats, said plainly rather than left to be assumed. Everything
# else on this bar is about one book and the whole of its text; this is about the
# whole project and only since it was opened.
analysis-arrivals-scope = This whole project, not just this book.
analysis-arrivals-session = Since you opened it. Closing the project starts the count again.
analysis-arrivals-nothing = No text has arrived yet in this session.
analysis-arrivals-typed = Typed
analysis-arrivals-pasted = Pasted
analysis-arrivals-dictated = Dictated
analysis-arrivals-imported = Imported
analysis-arrivals-programmatic = Inserted for you
analysis-arrivals-count = { $count ->
    [one] { $count } character
   *[other] { $count } characters
}
# Said where a route contributed nothing, so a reader is not left wondering
# whether it was measured at all.
analysis-arrivals-none = none

analysis-words-per-scene = Words per scene
analysis-median-words = This book's median scene runs { $count ->
    [one] { $count } word.
   *[other] { $count } words.
}
analysis-median-line = Median: { $count ->
    [one] { $count } word
   *[other] { $count } words
}
analysis-dialogue = Dialogue
# Said instead of showing 0%, which would read as "there is no dialogue here".
analysis-dialogue-unsupported = Dialogue is not measured for this language yet.

analysis-footnote-words = Footnote words
# Kept apart from the manuscript total on purpose — see the module doc on
# AnalysisViewModel::run for why a footnote is authored prose but must not be folded
# into how far along the story reads as being.
analysis-footnote-words-count = { $count ->
    [one] { $count } word is in this book's footnotes, kept apart from the manuscript total.
   *[other] { $count } words are in this book's footnotes, kept apart from the manuscript total.
}
# The figure comes from its own operation and can still be catching up even once the
# rest of this report is ready — said plainly rather than shown as a misleading 0.
analysis-footnote-words-pending = Counting the footnotes…

# Shape's charts draw one bar per text, so an outlined-but-unwritten book is mostly gaps.
analysis-ignore-empty = Ignore texts with nothing written yet
analysis-empty-hidden = { $count } empty { $count ->
        [one] text
       *[other] texts
    } hidden.
analysis-all-texts-empty = Every text in this book is still empty.

# ── Binder filter feedback ───────────────────────────────────────────────────
# The binder's search field lives in a popover, so once it is dismissed nothing
# on screen says a filter is still narrowing the tree. These strings are what
# say it — without them a filtered-to-nothing binder is indistinguishable from
# an empty project.
binder-filter-count = { $shown } of { $total } shown
binder-filter-clear = Clear
binder-filter-none = Nothing matches “{ $query }”.

## Settings: paratext structures
settings-paratext-intro = The front and back matter a new project can start with. Each structure belongs to a publishing tradition, and its page titles are written in that tradition's own language — rename them freely once a project is created.
settings-paratext-structures = Structures
settings-paratext-broken = Could not be read
settings-paratext-edit = Edit
settings-paratext-duplicate = Duplicate
settings-paratext-delete = Delete
settings-paratext-new = New structure
settings-paratext-save = Save
settings-paratext-editor-hint = A name, the pages to create before the manuscript, and the pages to create after it. Where they end up is yours: they are ordinary items once the project exists.

## Images

image-insert = Insert i&mage…
image-no-project = Open a project before inserting an image.
image-choose-title = Choose an image
image-filter-label = Images
image-large-title = This is a large image
image-large-text =
    { $name } is { $megapixels } megapixels ({ $width }×{ $height }).
    Keeping it as it is stores your original file in the project — it travels
    with every backup and every export. Optimising stores a smaller copy
    instead, up to 2560 pixels on its longest side.
image-large-keep = Keep original
image-large-downscale = Optimise
image-large-remember = Do this from now on, don't ask again
image-not-recorded = The image was saved but could not be recorded in the project.
image-describe-title = Describe the image
image-describe-explain = What the picture shows, for a reader who cannot see it. It is not part of the manuscript: it is never counted, searched or exported as prose.
image-describe-placeholder = a lighthouse against a gray sky
image-resize-title = Resize the image
image-resize-explain = A percentage of the size it is shown at now. 100 leaves it as it is.
image-resize-invalid = Enter a number between 1 and 1000.
image-menu-describe = &Describe the image…
image-menu-resize = &Resize the image…
image-menu-reset-size = Original si&ze

# The book's cover — chosen from the book, not typed into a scene.
cover-choose = Book co&ver…
cover-clear = &Remove the cover
cover-choose-title = Choose a cover
cover-set = The cover is set.
cover-cleared = The cover has been removed. The picture is still in the project.
# ── Outline row card ────────────────────────────────────────────────────────
# The hover card on an outline row: what a row is, without opening it.
card-label = Label
card-exportable = Exported
card-numbered = Numbered
card-created = Created
card-modified = Modified
card-yes = yes
card-no = no
card-type = Type
card-position = Position
card-children = Children
card-goal = Goal
card-words = Words
card-synopsis = Synopsis
card-point-of-view = Point of view
card-aliases = Also known as

export-orphan-footnotes-title = Footnotes with no reference
export-comments-dropped =
    { $count ->
        [one] One comment could not be placed in the exported text and was left out.
       *[other] { $count } comments could not be placed in the exported text and were left out.
    }
export-orphan-footnotes =
    { $count ->
        [one] One footnote is no longer referenced anywhere in the manuscript. Its text will not appear in the exported book.
       *[other] { $count } footnotes are no longer referenced anywhere in the manuscript. Their text will not appear in the exported book.
    }

## Footnotes

menu-footnotes = Foot&notes
footnotes-title = Footnotes
footnotes-insert = Insert &footnote
footnotes-empty = No footnotes yet. Put the cursor in a scene, then use + above — or Ctrl+Alt+F.
footnotes-filter-all = All
footnotes-filter-document = This document
footnotes-filter-orphaned = Orphaned
footnotes-orphaned = Nothing points at this note any more
footnotes-untitled-home = Untitled
footnotes-body-placeholder = the note itself
footnotes-insert-tooltip = Insert a footnote at the cursor (Ctrl+Alt+F)
footnotes-actions = Footnote actions
footnotes-delete = &Delete note and its reference
footnotes-no-project = Open a project before inserting a footnote.
footnotes-no-caret = Put the cursor in a scene's text to insert a footnote there.
footnotes-not-created = The footnote could not be added to the project.
footnotes-deleted-toast = Note deleted
footnotes-undo-delete = Undo

# ── Import documents (Markdown / Word / ODT / plain text) ─────────────────────
import-document-title = Import documents
import-document-close = Close
import-document-step-files = Files
import-document-step-review = Review
import-document-step-destination = Destination
import-document-step-reconcile = Merge
import-document-reconcile-hint = Some of these rows are ones you already have. Say what should happen to each.
import-document-reconcile-all-new = Nothing in this file matches your project — every row will be added as new.
import-document-col-stray-prose = Its text
import-document-stray-as-paratext = Keep as paratext
import-document-stray-discard = Drop the text
import-document-col-current = In your project
import-document-col-incoming = In this file
import-document-col-status = Status
import-document-col-action = What to do
import-document-status-identical = Same
import-document-status-editor-edited = They edited it
import-document-status-you-edited = You edited it
import-document-status-conflict = Both edited it
import-document-status-different = Differs
import-document-status-new = New
import-document-status-missing = Not in this file
import-document-status-moved = Moved
import-document-action-comments-only = Comments only
import-document-action-take-import = Take this version
import-document-action-keep-current = Keep mine
import-document-action-create-new = Add as new
import-document-action-ignore = Skip
import-document-compare = Compare
import-document-compare-legend = Your version compared with this file's. Read-only.
import-document-compare-close = Close
import-document-drop-title = Drop documents here
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) and plain text (.txt)
import-document-browse = Browse…
import-document-move-up = Move up
import-document-move-down = Move down
import-document-remove-file = Remove
import-document-file-count = { $count ->
    [one] 1 file
   *[other] { $count } files
}
import-document-no-files = No files chosen yet.
import-document-col-included = Import
import-document-col-title = Title
import-document-col-type = Type
import-document-col-words = Words
import-document-col-breaks = Breaks
import-document-col-comments = Comments
import-document-col-epigraph = Epigraph
import-document-col-source = Source
import-document-level-rules = Heading levels
import-document-level-n = Heading { $level }
import-document-add-top-level = Add top level
import-document-add-top-level-tooltip = Insert a Book above every analyzed row — for chapter files that have no book heading
import-document-destination = Destination
import-document-destination-hint = Choose a binder or item — new rows land inside a folder, or after a scene.
import-document-destination-empty = No binders yet — create one first.
import-document-plan-empty = Nothing to import yet.
import-document-summary = { $rows ->
    [one] 1 row
   *[other] { $rows } rows
} · { $breaks ->
    [one] 1 scene break
   *[other] { $breaks } scene breaks
}
import-document-back = Back
import-document-cancel = Cancel
import-document-analyse = Next
import-document-analysing = Reading documents…
import-document-step-analysing = Reading
import-document-cancel-analysis = Stop reading
import-document-analyse-failed = The documents could not be read.
import-document-details = Details
import-document-import = Import
import-document-done = { $count ->
    [one] 1 item imported
   *[other] { $count } items imported
}
import-document-undo = Undo
# ── Import diagnostics ────────────────────────────────────────────────────────
# One per `document_ingest::ImportDiagnostic::key()`. The variant's data arrives
# as arguments; the sentence is assembled here, per locale. A row-scoped
# diagnostic gets its { $title } and { $kind } from the row it names, never from
# the wire.
import-diagnostic-file-unreadable = “{ $path }” could not be read: { $detail }. The other files still import.
import-diagnostic-lossy-decode = { $count ->
    [one] One character in “{ $path }” did not decode. Re-save the file as UTF-8 to keep it.
   *[other] { $count } characters in “{ $path }” did not decode. Re-save the file as UTF-8 to keep them.
}
import-diagnostic-decoded-from-bom = “{ $path }” was decoded as { $detail }, not UTF-8.
import-diagnostic-empty-file = “{ $path }” is empty.
import-diagnostic-no-headings = “{ $path }” has no headings, so it arrives as one item.
import-diagnostic-unsupported-format = Nothing reads “.{ $detail }” files, so “{ $path }” was skipped.
import-diagnostic-front-matter-not-flat = Front matter in “{ $path }”: “{ $detail }” is not a simple value and was skipped.
import-diagnostic-footnotes-degraded = { $count ->
    [one] One footnote in “{ $path }” arrives as plain text — footnotes are not read from Markdown.
   *[other] { $count } footnotes in “{ $path }” arrive as plain text — footnotes are not read from Markdown.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] One block of raw HTML in “{ $path }” was dropped.
   *[other] { $count } blocks of raw HTML in “{ $path }” were dropped.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] One scene break inside a quote or list in “{ $path }” was dropped. Only a break on its own line is kept.
   *[other] { $count } scene breaks inside quotes or lists in “{ $path }” were dropped. Only a break on its own line is kept.
}
import-diagnostic-image-not-ingested = “{ $path }” refers to the image “{ $detail }”. The reference arrives as text; the picture itself is not copied in.
import-diagnostic-duplicate-title = “{ $title }” appears { $count } times. If you have imported these files before, this will duplicate them.
import-diagnostic-heading-level-jump = “{ $title }” jumps from heading level { $from } to { $to }; it is placed one level under its parent.
import-diagnostic-illegal-combination = “{ $title }” carries prose, but a { $kind } cannot hold any. Import is held until you change its type or leave it out — nothing is imported at all otherwise.
import-diagnostic-tracked-changes-flattened = { $path } was mid-revision: { $count ->
    [one] { $count } tracked change was accepted
   *[other] { $count } tracked changes were accepted
}, and deletions dropped. That is the final text — but check it is the version you meant.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } holds { $count } text box. Its text sits outside the document's flow, so where it belongs in a manuscript cannot be answered — it is not imported.
   *[other] { $path } holds { $count } text boxes. Their text sits outside the document's flow, so where it belongs in a manuscript cannot be answered — they are not imported.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } holds { $count } embedded object — a chart, an equation or similar. There is nothing in a manuscript that could hold it.
   *[other] { $path } holds { $count } embedded objects — a chart, an equation or similar. There is nothing in a manuscript that could hold them.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } holds { $count } field — a page number, a cross-reference, a date. It keeps the text it was last showing and will not update again.
   *[other] { $path } holds { $count } fields — a page number, a cross-reference, a date. Each keeps the text it was last showing and will not update again.
}
import-diagnostic-unknown-style-level = { $path } uses the style “{ $detail }”, which looks like a heading but names no level. Those paragraphs are imported as prose rather than guessed at a depth.
import-diagnostic-comment-unanchored = The comment “{ $detail }” in { $path } could not be attached to the words it was about. It is kept on its item, where you can move it.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } reply in { $path } named a comment that is not in the file, so it arrives as a comment of its own.
   *[other] { $count } replies in { $path } named a comment that is not in the file, so they arrive as comments of their own.
}
import-diagnostic-epigraph-not-carried = “{ $title }” is headed by an epigraph, but a { $kind } cannot hold one. The quotation is kept at the top of its text instead.
import-diagnostic-epigraph-placement-ambiguous = An epigraph sits between “{ $title }” and “{ $below }” and could head either. It was given to “{ $title }”, which is where an epigraph usually goes.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 thing to know
           *[other] { $warnings } things to know
        }
   *[other] { $errors ->
            [one] 1 file could not be read
           *[other] { $errors } files could not be read
        }
}
import-document-diagnostics-none = Nothing to report.

# ── Versions dock ──
versions-title = Versions
versions-scope-synopsis = Synopsis
versions-scope-prose = Text
versions-loading = Looking through your backups…
versions-empty = No earlier version of this yet
versions-error = Couldn't read your backups — nothing has been lost, but this list may be incomplete
versions-did-not-exist = Didn't exist yet on { $date }
versions-deleted-after = Deleted some time after { $date }
versions-unreadable = { $count ->
    [one] { $count } backup couldn't be read
   *[other] { $count } backups couldn't be read
}
# Counts what the project's own history dropped, which is why it names it: a
# backup may still hold one of those states, and then it is in the list above.
versions-thinned = { $count ->
    [one] Older versions thin out as they age — the project's own history has already dropped { $count } earlier state of this text.
   *[other] Older versions thin out as they age — the project's own history has already dropped { $count } earlier states of this text.
}
versions-source-backup = From a backup
versions-source-project = From the project's own history
versions-list-caption = One entry per change, not per backup
versions-pick-a-version = Pick a version to see what changed
versions-earliest = The earliest version on record. There's nothing older to compare it with.
versions-no-change = Nothing changed in this part of the row
versions-formatting-only = Only the formatting changed here — the words are the same
versions-show-unchanged = Show unchanged
versions-hide-unchanged = Hide unchanged
versions-next-change = Next change
versions-near = near "{ $text }"
versions-words-added = { $count ->
    [one] { $count } word added
   *[other] { $count } words added
}
versions-words-removed = { $count ->
    [one] { $count } word removed
   *[other] { $count } words removed
}
versions-blocks-moved = { $count ->
    [one] { $count } paragraph moved
   *[other] { $count } paragraphs moved
}
# Names the backup, not the version: a pin is held against a file path, so it
# keeps the whole snapshot this row's text was read out of — see versions-pin-note.
versions-pin = Pin the backup this version came from — automatic cleanup will never delete it
versions-unpin = Unpin the backup this version came from — automatic cleanup may delete it again
versions-pinned-only = Show only pinned versions
versions-pin-note = Only versions from a backup can be pinned — a pin keeps a file, and the project's own history isn't one.
versions-pinned-empty = Nothing here is pinned yet
# The same sentence with the reason, for a list that actually holds a version no
# pin can reach. On an all-backup list the reason is true and irrelevant, and
# reads as an explanation for an emptiness it did not cause.
versions-pinned-empty-log = Nothing here is pinned yet. Only versions from a backup can be pinned — a pin keeps a file, and the project's own history isn't one.
versions-range-filter = Show only versions between two dates
versions-filtered-empty = No version matches the filters you've set
versions-clear-filters = Clear the filters
# Also used by the Timeline band's filter row, like versions-error above it.
versions-last-30-days = Last 30 days
versions-restore-button = Restore this version
versions-restore-confirm-title = Replace this text with the version from { $date }?
versions-restore-confirm-text = What you have now will be replaced by the text this row had on { $date }.
versions-restore-confirm-with-comments = What you have now will be replaced by the text this row had on { $date }. { $count ->
    [one] { $count } comment is anchored in the current text and may be left orphaned.
   *[other] { $count } comments are anchored in the current text and may be left orphaned.
}
versions-restore-confirm-undo-note = A backup is made first, and Ctrl+Z undoes this in one step.
versions-restored-toast = Restored the version from { $date }
versions-undo = Undo
versions-restore-row-gone = That row is no longer in this project
versions-restore-no-home = This row has changed type since then, and the old text has nowhere to go in it
versions-restore-no-safety-copy = Your safety backup didn't run, so nothing was changed
versions-restore-failed = The restore failed: { $error }
versions-restore-backup-busy = A backup is already running — try again in a moment
versions-restore-in-backup-file = You're looking at a backup file; open the project itself to restore into it
versions-restore-no-project = No project is open
# ── Bringing a deleted row back ──
versions-recreate-button = Bring this back…
versions-recreate-picker-title = Where should it go?
versions-recreate-picker-empty = This project has no binder to put it in
versions-recreate-picker-confirm = Bring it back here
versions-recreate-picker-cancel = Cancel
versions-recreate-untitled = this row
versions-recreate-confirm-title = Bring “{ $item }” back?
versions-recreate-confirm-text = It will be added to { $destination }, with the text it had on { $date }.
# NOT "Ctrl+Z": that undoes the focused editor's *document*, and this creates a
# binder row. The way back is the Undo on the toast, as it is for trash and
# comments — see versions-recreated-toast.
versions-recreate-confirm-undo-note = Undo, on the message that follows, takes it straight back out.
versions-recreated-toast = “{ $item }” is back in your project
versions-recreated-partial-toast = { $count ->
    [one] “{ $item }” is back, but one of its texts couldn't be read
   *[other] “{ $item }” is back, but { $count } of its texts couldn't be read
}
versions-recreate-already-here = That row is in your project already
versions-recreate-no-destination = Pick somewhere in the binder to put it
versions-recreate-unreadable = That backup couldn't be read, so nothing was added
versions-recreate-failed = Couldn't bring it back: { $error }
versions-changed-percent = { $percent }% of this changed
versions-hidden-paragraphs = { $count ->
    [one] … { $count } unchanged paragraph …
   *[other] … { $count } unchanged paragraphs …
}

# ── Timeline band ──
timeline-title = Go back in time
timeline-coverage = { $count ->
    [one] { $count } version recorded, going back to { $oldest }
   *[other] { $count } versions recorded, going back to { $oldest }
}
timeline-loading = Looking through your project's past…
timeline-empty = No version of this project has been recorded yet
timeline-no-changes = Nothing has changed since then
timeline-no-text-changes = No text has changed since then
timeline-slider-label = Recorded version
timeline-series-name = Project size
timeline-bars-caption = Each bar is a recorded version, as tall as the project was then. The highlighted one is what you're looking at.
timeline-bars-caption-periods = Too many versions to show one by one, so each bar is a { $period }, as tall as the project was by the end of it.
timeline-unit-hour = hour
timeline-unit-day = day
timeline-unit-week = week
timeline-unit-month = month
timeline-range-filter = Show only versions between two dates
timeline-range-empty = No version was recorded in those dates
timeline-open-period = Open this period
timeline-show-all = Show the whole history
timeline-changed-since = { $count ->
    [one] { $count } item differs between { $date } and your project now
   *[other] { $count } items differ between { $date } and your project now
}
timeline-kind-added = Written since
timeline-kind-removed = No longer in the project
timeline-kind-changed = Edited since
timeline-kind-moved = Moved since
timeline-not-yet-written = This didn't exist yet at that point
timeline-no-text-of-its-own = This has no text of its own — it's a heading for what's inside it
timeline-reader-close = Close
timeline-reader-stamp = As it was on { $date }
timeline-reader-loading = Opening the recorded version…
timeline-reader-unreadable = This recording couldn't be read — the backup may have been moved, deleted, or be on a drive that isn't connected.
timeline-reader-compared = As it was on { $date }, set against what it says now
timeline-reader-diff-legend = Struck through has gone since; underlined has been added.
timeline-reader-view-label = What to show
timeline-reader-view-diff = Changes
timeline-reader-view-text = Text
timeline-reader-deleted = This is no longer in your project. You can read it and copy it out here.
timeline-prose-only-record = This point comes from the project's own history, which keeps text and nothing else — so what was deleted or moved since it can't be shown. Pick a backup for that.

# The Settings tree's section for pages an extension contributed.
settings-sec-extensions = Extensions

# Writing games pane — self-imposed drafting constraints
settings-group-games-forward = Always forward
settings-games-forward-toggle = Play "Always forward"
settings-games-forward-hint =
    While you play, nothing you have written can be taken back: Backspace, Delete,
    Cut, drag-and-drop and Undo are all disabled in the surfaces you choose below.
    You can still type, paste, format and move around freely — the draft only grows.
settings-games-session-warning =
    This is a per-session choice: it is never saved. Closing the project, or quitting
    Skribisto, always ends the game — and everything you wrote while playing becomes
    undoable again the moment it does.
settings-group-games-scope = Where it applies
settings-games-in-prose = Manuscript prose
settings-games-in-synopsis = Synopses
settings-games-scope-hint =
    Comments, footnotes and titles are never frozen: they are where you note the fix
    you have just forbidden yourself from making.
settings-games-inert-warning =
    "Always forward" is on but applies to nothing — tick at least one surface above,
    or it changes nothing as you write.

# Writing games dock (leading rail) + the status-bar warning while a game is on
games-title = Writing games
games-forward-name = Always forward
games-forward-blurb =
    Draft without taking anything back. Deleting, cutting and undoing are disabled
    while you play — write the next sentence instead of fixing the last one.
games-forward-playing = Playing — deleting is disabled
games-forward-idle = Not playing
games-session-note = Ends when you close the project.
games-scope-prose-and-synopsis = Applies to your prose and synopses.
games-scope-prose = Applies to your prose.
games-scope-synopsis = Applies to your synopses.
games-scope-nothing = Applies to nothing yet — choose a surface in Settings.
games-settings-link = Writing game settings…
statusbar-games-forward = Always forward
statusbar-games-forward-tooltip =
    "Always forward" is on: deleting, cutting and undoing are disabled while you
    draft. Click to stop playing.
# ── Word / character targets ──────────────────────────────────────────────────
# The shared vocabulary every surface that shows a target uses: the Inspector, the
# Overview column, the status bar, a container page and the Distribute preview. Plurals
# select on the bare number (`$g` / `$n`) while the visible figure is the grouped string,
# because a count printed with thin spaces is no longer a number Fluent can select on.
goal-progress-words = { $g ->
    [one] { $count } of { $goal } word
   *[other] { $count } of { $goal } words
}
goal-progress-characters = { $g ->
    [one] { $count } of { $goal } character
   *[other] { $count } of { $goal } characters
}
goal-count-words = { $n ->
    [one] { $count } word
   *[other] { $count } words
}
goal-count-characters = { $n ->
    [one] { $count } character
   *[other] { $count } characters
}
goal-unit-words = Words
goal-unit-characters = Characters

# The Inspector's per-item target.
inspector-goal = Target
inspector-goal-none = No target

# The Overview's target column, and the note on a row the export leaves out.
overview-col-goal = Target
overview-excluded-from-export = Left out of the export, so it counts toward no total

# A container's own page: what the targets set inside it add up to. Deliberately not a
# target itself, and worded so it cannot be read as one.
goal-subtree-total-words = { $count ->
    [one] { $items } target inside adds up to { $words } words
   *[other] { $items } targets inside add up to { $words } words
}
goal-subtree-total-characters = { $count ->
    [one] { $items } target inside adds up to { $words } characters
   *[other] { $items } targets inside add up to { $words } characters
}

# The New Work panel's counting-unit picker, beside the language it is seeded from.
new-work-goal-unit = Count in
new-work-goal-unit-hint = Used for every word or character target in this project. Changeable later.

# Settings ▸ Work ▸ Structure: the project's counting unit, and the warning shown before
# switching it. No conversion happens, in either direction.
settings-group-goal-unit = Targets
settings-goal-unit-switch-title = Switch counting unit
settings-goal-unit-switch-text = The targets and milestones already set for this project were entered in { $from }. Switching to { $to } does not convert them, so every existing number will now be read as { $to }.
settings-goal-unit-switch-informative = Nothing is lost. Switch back at any time to restore the original reading, then update the targets you want to keep.
settings-goal-unit-switch-confirm = Switch

# Distribute: sharing a container's target across the pieces inside it.
goal-distribute-action = Distribute…
distribute-title = Distribute the target
distribute-empty = There is nothing inside this to share the target out to.
distribute-weight = Share out
distribute-weight-length = By length
distribute-weight-rows = By pieces
distribute-weight-even = Evenly
distribute-overwrite = Replace targets that are already set
distribute-col-item = Item
distribute-col-current = Now
distribute-col-proposed = After
distribute-total = These add up to { $total }, against a target of { $goal }.
distribute-over-budget = The targets already set inside come to { $over } more than this container's own. Raise its target, lower theirs, or replace them.
distribute-apply = Distribute
distribute-cancel = Cancel

# Milestones on the Book's pace: a dated waypoint, of one of two kinds.
milestone-target-gone = Target deleted
milestone-no-target = No target
milestone-add = Add
milestone-add-label = What should be reached

# The writing-plan summary shown once when a project with an active plan opens.
pace-summary-title = Where the book stands
pace-summary-remaining = { $words } to go
pace-summary-open = Open the plan
pace-summary-close = Close
pace-summary-dont-show = Do not show this when opening
# Work menu row that opens the same card on demand — greyed out when the project has
# no active writing plan.
pace-summary-menu = Writing plan…

# ── Help ─────────────────────────────────────────────────────────────────────
# The Help window (crates/teksilo_ui/src/help/), its table of contents and the
# keyboard-shortcuts sheet. Topic *bodies* are not here: they are Djot documents
# under crates/teksilo_ui/help/<locale>/, one file per topic per locale.
menu-help-topics = Help &Topics
menu-help-shortcuts = &Keyboard Shortcuts…
menu-help-website = Skribisto on the &Web
menu-help-report = &Report a Problem…
menu-command-palette = &Command Palette…

# The Learn pane's rows in the Launcher. Separate keys from the Help menu's rows
# on purpose: a menu label carries an `&` mnemonic, and a Button renders that
# ampersand literally ("Help &Topics"), which is what shipped for one build.
learn-help-topics = Help topics
learn-shortcuts = Keyboard shortcuts
learn-website = Skribisto on the web

help-window-title = Help
help-filter-topics = Filter topics
help-no-matching-topic = No topic matches.
help-topic-missing = This topic is no longer available.
help-back = Back
help-not-translated = This page has not been translated yet, so it is shown in English.

help-section-getting-started = Getting started
help-section-writing = Writing
help-section-reviewing = Reviewing
help-section-exchanging = Getting words in and out
help-section-keeping = Keeping your work safe
help-section-extensions = Extensions

help-topic-getting-started = Your first project
help-topic-writing-model = How a book is put together
help-topic-drafts-and-old-versions = Keeping an older draft
help-topic-goals-and-pace = Targets and pace
help-topic-comments = Comments
help-topic-round-trip = Sending your book to a reader
help-topic-export = Exporting
help-topic-import-documents = Importing documents
help-topic-import-projects = Bringing a whole project across
help-topic-backups-and-versions = Backups and versions

help-shortcuts-title = Keyboard shortcuts
help-shortcuts-filter = Filter shortcuts
help-shortcuts-no-matches = No shortcut matches.
help-shortcuts-rebind = Change shortcuts…
help-shortcuts-close = Close

command-palette-placeholder = Type a command

# ── Shortcut names ───────────────────────────────────────────────────────────
# The user-visible name of every rebindable `Shortcut` (`Shortcut::name`,
# teksilo-core) — reactive to a locale switch via `LocalizedString`/
# `Prop<String>`. Read by Settings ▸ Keymap, the Help ▸ Keyboard shortcuts
# sheet and the command palette. Kept in lockstep with the `Shortcut::new(...)`
# calls in `app/commands/*.rs`, `binder/dock.rs` and `welcome/panel.rs`. Where
# the same command already carries a menu label above, that exact wording is
# reused here so the menu and the shortcut list agree.
shortcut-name-binder-duplicate = Duplicate
shortcut-name-comments-add = Add Comment
shortcut-name-comments-add-paragraph = Comment on Paragraph
shortcut-name-spellcheck-toggle = Check Spelling
shortcut-name-editor-tab-close = Close Tab
shortcut-name-editor-tab-pin = Pin or Unpin Tab
shortcut-name-editor-save = Save
shortcut-name-work-export = Export…
shortcut-name-work-new = New Work
shortcut-name-work-open = Open Work
shortcut-name-window-new = New Window
shortcut-name-work-close = Close Work
shortcut-name-app-settings = Settings
shortcut-name-app-quit = Quit
shortcut-name-editor-insert-footnote = Insert Footnote
shortcut-name-format-scene-break = Insert Scene Break
shortcut-name-format-major-scene-break = Insert Major Scene Break
shortcut-name-format-link = Insert Link
shortcut-name-go-next = Next
shortcut-name-go-prev = Previous
shortcut-name-go-to = Go to
shortcut-name-outline-toggle = Toggle Outline
shortcut-name-preview-toggle = Toggle Preview Band
shortcut-name-view-fullscreen = Toggle Fullscreen
shortcut-name-view-focus-mode = Toggle Distraction-free Mode
shortcut-name-editor-size-increase = Increase Text Size
shortcut-name-editor-size-decrease = Decrease Text Size
shortcut-name-editor-size-reset = Reset Text Size
shortcut-name-editor-find = Find
shortcut-name-editor-replace = Replace
shortcut-name-editor-find-next = Next Match
shortcut-name-editor-find-prev = Previous Match
shortcut-name-search-show = Search in Project
shortcut-name-search-replace = Replace in Project
shortcut-name-outline-open-to-side = Open to the Side
shortcut-name-help-topics = Help Topics
shortcut-name-help-shortcuts = Keyboard Shortcuts
shortcut-name-help-website = Skribisto on the Web
shortcut-name-help-report = Report a Problem
shortcut-name-command-palette = Command Palette
help-section-reference = What things are

# Titles for the glossary entries whose concept is not something you can create, so
# there is no "＋ Create" label to reuse. See help.rs::concept_topics.
help-concept-scene-break = Scene break
help-concept-major-scene-break = Major scene break
help-concept-find-in-prose = Find in prose
help-concept-story-bible = Story bible
help-concept-goal-unit = Words or characters
help-concept-goal-progress = Progress
help-concept-manuscript-words = What counts as the manuscript
help-concept-exportable = Left out of the export
help-concept-distribute = Sharing a target out
help-concept-subtree-total = What the targets inside add up to
help-concept-milestone = Milestone
help-concept-pace-plan = Pace plan

# Glossary titles for the feature-concept web. See help.rs::concept_topics.
help-concept-tag = Tag
help-concept-label = Label
help-concept-point-of-view = Point of view
help-concept-epigraph = Epigraph
help-concept-footnote = Footnote
help-concept-chapter-mode = Chapter shape
help-concept-comment = Comment
help-concept-backup = Backup
help-concept-version = Version
help-concept-trash = Trash
help-concept-spellcheck = Spell checking
help-concept-search-replace = Search and replace
help-concept-note-template = Note template
help-concept-story-bible-entry = Story bible entry
help-concept-text-replacement = Text replacement
help-concept-smart-punctuation = Smart punctuation
help-concept-export-style = Export style
help-concept-round-trip-marks = Round-trip markers

# The epigraph's attribution control. See tabs/shared/panes.rs::attribution_control.
epigraph-mark-attribution = Source line
epigraph-mark-attribution-tip = Mark the line the caret is in as the quotation's source, so it prints as an attribution
## Margin lane — the strip beside the scrollbar that maps a document
margin-lane-name = Margin marks
margin-lane-provider-comments = Comments
margin-lane-provider-story-bible = Story bible entry
margin-lane-provider-story-bible-hint = On a note's In prose reading, where that entry is named, and the scenes told through its eyes. Also marks those names in the prose itself.
margin-lane-mark-point-of-view = Told from here
margin-lane-mark-named-here = Named here
margin-lane-provider-comments-hint = Where a note is attached to the text
margin-lane-provider-search = Search hits
margin-lane-provider-search-hint = Every hit for what you last searched for, wherever it is in the document
margin-lane-provider-boundaries = Where each document starts
margin-lane-provider-boundaries-hint = A rule at the top of every scene in a stream, so you can see where you are
margin-lane-provider-spelling = Spelling
margin-lane-provider-spelling-hint = Where the spell checker has flagged a word. Off unless you ask for it: it marks a machine's opinion of your prose
margin-lane-spelling = { $word }, possible misspelling
margin-lane-search-hit = { $text }, match { $index } of { $total }
margin-lane-search-current = { $text }, match { $index } of { $total }, the one you are on
margin-lane-boundary = Start of { $title }
margin-lane-boundary-untitled = an untitled document
settings-page-margin-lane = Margin marks
settings-desc-margin-lane = The strip beside the scroll bar, and what it shows
settings-margin-lane-enabled = Show the margin lane
settings-margin-lane-enabled-hint = The same switch as View ▸ Margin marks.
settings-margin-lane-enabled-more = It never says anything is wrong: it shows you where things are, and leaves what to make of that to you.
settings-group-margin-lane-marks = What it marks
settings-margin-lane-no-providers = Nothing marks the lane yet.
settings-group-margin-lane-texture = Dialogue texture
settings-margin-lane-texture = Show the dialogue texture
settings-margin-lane-texture-hint = One bar per paragraph: how long it is, and how much of it is spoken.
settings-margin-lane-texture-more = Not measured for languages with no curated convention, where the bar shows length alone.
settings-group-margin-lane-surfaces = Where it appears
settings-margin-lane-surface-editor = Text editor
settings-margin-lane-surface-stream = Streams
settings-margin-lane-surface-search-preview = Search preview
menu-margin-lane = &Margin marks

## Statuses — the project's workflow ladder.
## A rung's name is project DATA: these are resolved once, when a ladder is seeded, and
## stored literally from then on. They are not re-translated afterwards, and the writer is
## free to rename any of them.
status-preset-drafting = Drafting
status-preset-passes = Revision passes
status-preset-plume = Plume Creator
status-todo = To do
status-draft = Draft
status-revised = Revised
status-final = Final
status-outline = Outline
status-first-edit = 1st edit
status-second-edit = 2nd edit
status-done = Done
status-plume-draft-1 = 1st draft
status-plume-draft-2 = 2nd draft
status-plume-draft-3 = 3rd draft
status-plume-edit-1 = 1st edit
status-plume-edit-2 = 2nd edit
status-plume-edit-3 = 3rd edit
status-plume-proofread = Proofread
status-plume-finished = Finished
new-work-statuses = Workflow
new-work-statuses-hint = The stages you move a scene through. You can rename, reorder or add to these at any time.
status-none = No status
inspector-status = Status
overview-col-status = Status
overview-status-mixed = the parts below disagree
help-concept-status = Status
status-completion-title = Where the book stands
status-completion-headline = { $done } of { $total } scenes finished
status-completion-empty = No scenes yet — this reads the manuscript, so it fills in as you write.
status-completion-open = Where the book stands…

## Settings ▸ Work ▸ Statuses — the ladder editor.
## The writer owns a rung's NAME and the ladder's ORDER; the app owns its category, and
## the category owns the glyph and the colour. Only the category names below are
## translated — a rung's name is the writer's own text and stays as they typed it.
settings-page-statuses = Statuses
settings-desc-statuses = Rename, reorder, add and remove the stages you move a scene through
settings-statuses-add = Add status
settings-statuses-add-placeholder = New status name
settings-statuses-desc =
    The stages a scene moves through, in order — from least finished at the top to most
    finished at the bottom. That order is what "less finished than" means everywhere else
    in the app, so put them the way your process actually runs.
settings-statuses-apply-preset = Apply a preset…
settings-statuses-preset-applied = { $added ->
    [one] 1 status added
   *[other] { $added } statuses added
}
settings-statuses-preset-refused = This project already has a ladder. Presets only fill an empty one — delete the rungs you don't want first.
settings-statuses-duplicate = "{ $name }" is already on this ladder
settings-statuses-added = Added "{ $name }"
settings-statuses-move-up = Move up (less finished)
settings-statuses-move-down = Move down (more finished)
settings-statuses-details-placeholder = What this stage means (optional)
settings-statuses-delete = Delete "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Delete "{ $name }" — 1 item is on it and will lose its status
   *[other] Delete "{ $name }" — { $count } items are on it and will lose their status
}
settings-statuses-deleted = Deleted "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Deleted "{ $name }". 1 item no longer has a status.
   *[other] Deleted "{ $name }". { $count } items no longer have a status.
}
settings-statuses-empty-title = No workflow yet
settings-statuses-empty-body = A status says how far along a scene is. Add one above, or start from a preset.
status-category-planned = Planned
status-category-drafting = Drafting
status-category-needs-work = Needs work
status-category-revised = Revised
status-category-final = Final
