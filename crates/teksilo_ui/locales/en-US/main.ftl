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
menu-import-scrivener = &Scrivener (.scriv)…
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
# Three rows are either/or: the split, the move and the pin each build exactly
# one of their pair, so a letter may repeat across a pair but never within one
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
# file, so a project that has never been saved has nothing for it to open. A
# tooltip on the disabled row, not a menu label, hence no mnemonic.
ctx-tab-move-window-unsaved = Save this project first: a second window opens onto a file on disk
ctx-tab-pin = &Pin this tab
ctx-tab-unpin = &Unpin this tab
# Tab tooltip, not a menu row: no mnemonic. It names the two commands a pin
# actually protects against, because the tab itself shows only a glyph and a
# missing close button, neither of which says what the pin is for.
tab-pinned-tooltip = Pinned: "Close others" and "Close all" leave it open

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
## Data, not chrome. Resolved once at creation and then owned by the writer,
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
# convention), so give the unit alone. Never shown at 0, which reads
# `session-no-limit` instead.
session-time-limit-unit = min
session-no-goal = No goal
session-no-limit = No limit
session-reset = Reset session
session-readout = { $words ->
    [one] { $words } word
   *[other] { $words } words
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } word
   *[other] { $words } words
} · { $time } left
# Distraction-free mode's always-visible control strip (Increment 2).
statusbar-focus-exit = Exit distraction-free mode
# The strip's Next/Previous buttons (Increment 4) fire the same go.next/go.prev
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
settings-check-for-updates = Check for new versions
settings-check-for-updates-tip = Once a day at most, Skribisto asks the project's website which version is current. The request carries no identifier, and not even the version you are running: the comparison happens on this computer.

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
# One line per page, shown under that page's link on its parent's page and,
# for a parent, under its own title. Keep them to one line: they describe what
# the page holds, not how to use it.
settings-desc-sec-appearance-behaviour = How the application itself looks, and what it does when it starts.
settings-desc-sec-editor = The writing surface: how it looks, and what it does as you type.
settings-desc-sec-spelling = Spell-checking, and the dictionaries behind it.
settings-desc-sec-backup = How your work reaches the disk, and what copies are kept.
settings-desc-sec-compile = What leaves Skribisto, and in what shape.
settings-desc-sec-work = Settings that belong to this project and travel inside its file.
settings-desc-sec-extensions = Pages contributed by the extensions installed here.
settings-desc-group-typography = One page per kind of text: typeface, size, line height and spacing.
settings-desc-appearance = Interface language, theme, text size, and the launcher at startup.
settings-desc-notifications = Every message this session has shown, and the actions you can replay.
settings-desc-scene = How scene prose is set: typeface, size, line height, indents and spacing.
settings-desc-synopsis = How the synopsis pane is set, independently of the manuscript.
settings-desc-notes = How notes are set, independently of the manuscript.
settings-desc-corkboard = Card size, what a card shows, and how the board arranges them.
settings-desc-distraction-free = Typography, column width and control strip for full-screen writing.
settings-desc-distraction-free-themes = The theme library for full-screen writing: the shipped ones and your own.
settings-desc-editor-behavior = Text width, synopsis position, typewriter scrolling and the caret highlight.
settings-desc-punctuation = The typographic house style new projects start from. Every project follows this unless it sets its own.
settings-desc-goals = Word and character targets, and how words are counted.
settings-desc-games = Constraints you set yourself while drafting, such as Always forward.
settings-desc-spellcheck = The single switch that turns spell-checking on and off everywhere.
settings-desc-dictionaries = Install, remove and browse the spelling dictionaries on this machine.
settings-desc-autosave = Whether your edits are written to disk on their own.
settings-desc-backup = When copies are taken, where they are kept, and how many. Every project follows this unless it sets its own.
settings-desc-export = The styles every export compiles through: the shipped ones and your own.
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
settings-field-user-hint = Both are optional and apply to every project on this computer. Changing them signs the comments you write next. Comments already written keep the name they were written under.
settings-desc-author = The name that goes on this project.
settings-desc-structure = Whether this project's chapters are folders or single items.
settings-desc-language = The language this project's prose is checked against.
settings-desc-work-backup = This project's own backup policy, or the general one.
settings-desc-personal-dictionary = The words this project treats as correctly spelled.
settings-desc-tags = The color-coded tags this project marks its items with.
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
settings-group-updates = Updates
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
settings-distraction-free-width-hint = Applies only while distraction-free mode is on. Your Scene, Synopsis and Notes column widths are untouched.
settings-distraction-free-title = Keep the item's name
settings-distraction-free-word-count = Keep the word count
settings-distraction-free-session = Keep the writing session
settings-distraction-free-go-to = Keep the Go to… button
settings-distraction-free-go = Keep the Previous and Next buttons
settings-distraction-free-chrome-hint = The Exit button always stays, whatever you choose here. It is your way out if Escape is busy.
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
settings-field-author-hint = Appears on the compiled title page and in exported file metadata. Leave it blank to omit it. This is the book's byline. The name your comments are signed with is in Settings ▸ User.
settings-group-chapters = Chapters
tidy-titles-title = Tidy chapter titles
tidy-titles-none = No chapter or part title is merely repeating its own number.
tidy-titles-lead = { $count ->
        [one] One chapter or part is titled with nothing but its own number.
       *[other] { $count } chapters and parts are titled with nothing but their own number.
    }
tidy-titles-explain = Clearing those titles leaves each one named by the number the book already knows, the same number the export prints. Nothing else changes, and one undo puts them all back.
menu-document-tidy-titles = Tid&y chapter titles…
settings-group-numbering = Numbering
settings-number-chapters = Number chapters and parts
settings-number-chapters-tip = Chapters and parts carry a number worked out from where they sit in the book, shown beside their title here, and printed by the export.
settings-number-chapters-tip-more = The number is never stored in the title, so it stays right when you reorder, insert or delete. Turn this off and the export prints titles alone, whatever the export style asks for. To leave one chapter out (a prologue, an interlude), use the Inspector's Numbering switch instead: that keeps it in the book but stops it taking a number.
settings-part-resets-chapter = Restart chapter numbers at each part
settings-part-resets-chapter-tip = Off by default: chapters run straight on across the parts of a book, so "Part Two" opens on Chapter Eleven.
settings-part-resets-chapter-tip-more = That is the usual trade practice, and what a reader expects. Turn it on for a book whose parts are meant to read as separate volumes, where each one opens on Chapter One.
settings-chapter-flat = Flat chapters
settings-chapter-flat-hint = On: a chapter is a single item. You write into it, and it holds no scenes. Off: a chapter is a folder. You still write into it, but it can hold scenes as well. New chapters follow this setting; existing ones convert via Promote.

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
# Shown under the version in the launcher sidebar, only when a newer release exists.
welcome-update-available = Version { $version } is available
welcome-update-tip = Opens the download page in your browser.
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
corkboard-layout-hint = Nested shows a container's direct children. Open a folder card to go inside it. Flat shows every scene in the whole container at once.
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
corkboard-move-picker-empty = No binders yet. Create one first.
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
    [one] { $count } item
   *[other] { $count } items
}
overview-search-placeholder = Filter items…
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
# the tooltip is their only accessible name, not decoration.
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
# Pre-filled in the "New binder" dialog, so the writer can just press Enter.
# Stored literally from then on: a binder name is data, not a label.
binder-default-name = New binder
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
dialog-new-binder = New binder
dialog-set-label = Set label
dialog-new-scene = New scene
close-work-question = Save changes before closing the work?
quit-question = Save changes before quitting?
quit-save-work-question = Save changes to { $title } before quitting?
unsaved-changes = This work has unsaved changes.
# Shown by Quit when another open Work (not this window's own) still has
# unsaved edits. The design's "one dialog listing every dirty Work" (see
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
# A project saved by a newer Skribisto than this one: title and body.
# $written_by is the version that wrote it, $requires the lowest version that can open it
# (the two differ when the newer build happened to use nothing new), $supported the newest
# this build understands.
could-not-open-work-too-new = "{ $file }" needs a newer Skribisto
could-not-open-work-too-new-detail = Saved by Skribisto format { $written_by }; opening it needs format { $requires } or newer, and this build supports up to format { $supported }. Update Skribisto to open it.
could-not-open-work-importing = "{ $file }" is being written by an import
could-not-open-work-importing-detail = An import is writing over this project and has not finished. Open it once the import is done: opened now, it would show the project the import is replacing, and its next save would write that project back over the import.
could-not-open-example = Could not open example: { $error }
# The browser (or whatever handles http links) could not be started for one of
# the Welcome sidebar's links. $url is shown so the address can still be copied.
could-not-open-link = Could not open { $url }: { $error }
could-not-create-work = Could not create work: { $error }
saving-as-file = Saving as { $target }…
saving-as-folder = Saving as { $target }/…
saved-as = Saved to { $target }
save-error = Could not save: { $error }
save-flush-failed = The save went through, but these documents could not be handed to it and are still unsaved: { $titles } ({ $error })
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
backup-failed-close-error-text = No backup copy could be written before closing: { $error }. Fix it and retry, or exit without backing up.
backup-no-result = The backup has ended without saying whether it made a copy. Check the list of backups to see if a new one is there.

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
new-work-chapter-scene-tip = Each chapter is a single item you write straight into, with no scenes under it. Leave this off for the classic layout, where a chapter is a folder: you write straight into that too, but it can hold scenes as well.
new-work-chapter-scene-tip-more = You write into a chapter either way. The only difference is whether it can *contain* scenes. Skribisto's binder tree is organizational only, so both layouts compile to the same book, you can mix them freely, and Promote converts a chapter between the two without losing a word.
# Field validation
new-work-name-required = Enter a name for the work
new-work-name-invalid = This name has no usable characters
new-work-location-required = Choose a location
new-work-location-missing = This folder does not exist
new-work-location-not-folder = This path is not a folder
new-work-location-readonly = This folder is not writable
new-work-target-exists = A project with this name is already in this folder. Creating the work will ask before replacing it
new-work-target-is-folder = A folder with this name is already in this location. Choose another name
new-work-overwrite-title = Replace the existing project?
new-work-overwrite-text = “{ $name }” already exists in this folder. Replace it with the new project?
new-work-target-open-title = This project is open
new-work-target-open-text = “{ $name }” is open in Skribisto. Creating a new project in its place would replace the project you are working on, and your next save would write that project back over the new one. Choose another name or folder, or close the project first.
target-importing-title = An import is writing this project
target-importing-text = An import is still writing “{ $name }”. Wait until it has finished, or choose another name.
backup-restore-importing-text = An import is still writing “{ $name }”, the project this backup would be restored over. Restore the backup once the import has finished.

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
import-plume-source = Plume project
import-plume-source-hint = Choose a .plume or .plume_backup file (any Plume Creator version).
import-plume-trash-warning = ⚠ Trashed / deleted items are not migrated.
# Field validation
import-plume-source-required = Choose a Plume project file
import-plume-source-missing = This file does not exist
import-plume-source-not-file = This path is not a file
# Binder names passed to the backend (which can't do i18n)
import-plume-manuscript-binder = Manuscript
import-plume-story-bible-binder = Story Bible
import-plume-notes-of = { $name } (notes)
# The title of the scene that takes a Plume Creator folder's own text, after the folder's
# name. Stored in the new project as written.
import-plume-content-of = { $name } (text)
# Progress toast (the import runs as a long operation)
import-plume-progress-title = Importing Plume project…
# Result
import-plume-done = Imported { $imported ->
    [one] { $imported } item.
   *[other] { $imported } items.
} { $skipped ->
    [one] { $skipped } trashed item was not migrated.
   *[other] { $skipped } trashed items were not migrated.
}
# Why a Plume Creator project was not imported. $part is the part's own name
# inside the project (tree, attendance, info); $limit is the ceiling, a number.
import-plume-nested-too-deep = The project was not imported: its “{ $part }” part is nested more than { $limit } levels deep. No Plume Creator project comes near that, and reading it could crash Skribisto. Nothing was written.
# The project was refused unread because one of its parts declares XML
# entities, which could expand into more memory than the computer has. $part
# is the part's own name (tree, attendance, info).
import-plume-declares-entities = The project was not imported: its “{ $part }” part declares XML entities, which no Plume Creator project does. Expanding them could take more memory than the computer has and crash Skribisto. Nothing was written.
import-plume-no-outline = This file is a compressed archive, but not a Plume Creator project: it has no outline. Nothing was written.
import-plume-empty-outline = The outline file of this Plume Creator project is empty, so there is nothing to import. Nothing was written.
# $tag is the name of the document the part turned out to be.
import-plume-wrong-document = This is not a Plume Creator project: its “{ $part }” part is a “{ $tag }” document. Nothing was written.
# $version is the part's own version, $supported the newest this importer reads.
import-plume-too-new = The “{ $part }” part of this project was written by a newer Plume Creator (version { $version }) than Skribisto can read (up to { $supported }). Nothing was written.

## Import Manuskript project dialog
import-manuskript-title = Import Manuskript project
import-manuskript-source = Manuskript project
# A Manuskript project is a .msk file plus, in its usual mode, a folder of the
# same name beside it. Either one gets you there, and so does the folder itself.
import-manuskript-source-hint = Choose the .msk file or the project folder (any Manuskript version).
import-manuskript-source-file = Choose file…
import-manuskript-source-folder = Choose folder…
# Field validation
import-manuskript-source-required = Choose a Manuskript project
import-manuskript-source-missing = This file or folder does not exist
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
import-manuskript-notes-of = { $title } (notes)
# Progress toast (the import is a long operation)
import-manuskript-progress-title = Importing Manuskript project…
# Result
import-manuskript-done = { $imported ->
    [one] { $imported } item imported.
   *[other] { $imported } items imported.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } earlier version came with it.
   *[other] { $revisions } earlier versions came with them.
}
# Why a Manuskript project was not imported. $part is that file's name inside the
# project (world.opml, plots.xml…); $limit is the ceiling, a number.
import-manuskript-nested-too-deep = The project was not imported: its file “{ $part }” is nested more than { $limit } levels deep. No Manuskript project comes near that, and reading it could crash Skribisto. Nothing was written.
# One of its files sits in folders nested deeper than the app will read
# (Manuskript keeps a project's outline as folders, one per level).
import-manuskript-folders-too-deep = The project was not imported: one of its files sits more than { $limit } folders deep. No Manuskript project comes near that, and reading it could crash Skribisto. Nothing was written.
# The project was refused unread because one of its files declares XML
# entities, which could expand into more memory than the computer has. $part
# is that file's name inside the project (world.opml, plots.xml…).
import-manuskript-declares-entities = The project was not imported: its file “{ $part }” declares XML entities, which no Manuskript project does. Expanding them could take more memory than the computer has and crash Skribisto. Nothing was written.
import-manuskript-not-a-project-file = “{ $file }” is neither a Manuskript project archive nor the small .msk file that points at a project folder. Nothing was written.
import-manuskript-folder-missing = “{ $stub }” points at a project folder named “{ $folder }” beside it, and that folder is not there. A Manuskript project in folder mode is both, and they have to travel together. Nothing was written.
import-manuskript-not-a-project = This does not look like a Manuskript project: it has no Manuskript marker and no outline. Nothing was written.
import-manuskript-format-too-new = This project is in Manuskript format { $version }, newer than Skribisto can read (up to { $supported }). Nothing was written.
import-manuskript-too-many-files = The project was not imported: it holds more than { $limit } files, far more than any Manuskript project. Nothing was written.
import-manuskript-too-large = The project was not imported: it is larger than the { $limit } MB Skribisto reads from a Manuskript project. Nothing was written.
project-import-close = Close
project-import-location = Destination folder
# The title of the folder dialog, like the dialog-*-title keys: word it as a window title
# (in many languages an infinitive or a noun phrase), not as the instruction
# project-import-location-required gives under the field.
project-import-location-title = Choose a destination folder
project-import-name = File name
project-import-name-placeholder = Project name
project-import-will-create = Will create
project-import-cancel = Cancel
project-import-import = Import
# Field validation
project-import-location-required = Choose a destination folder
project-import-location-missing = This folder does not exist
project-import-location-not-folder = This path is not a folder
project-import-location-readonly = This folder is not writable
project-import-name-required = Enter a file name
project-import-name-exists = A file with this name already exists here. Import will confirm overwrite
project-import-name-is-folder = A folder with this name is already here, perhaps a project kept as a folder. An import cannot replace it: choose another name
# Overwrite confirmation
project-import-overwrite-title = Replace existing file?
project-import-overwrite-text = “{ $name }” already exists. Replace it with the imported project?
# Projects are imported one at a time, whichever importer asks.
project-import-busy-title = Another project is being imported
project-import-busy-text = Projects are imported one at a time. Wait until the import under way has finished, or cancel it, then start this one again.
# Spoken by a screen reader once per phase, never shown: the toast's title, then
# the phase's name.
project-import-progress-announcement = { $title } { $phase }
# The notice once Cancel is pressed, until the converter has stopped: it may be
# finishing a file it cannot leave half written.
project-import-stopping = Stopping the import…
project-import-cancelled = Import canceled
project-import-open-now = Open now
project-import-details = Details
# The import's report: what the writer should know about what came across.
project-import-report = { $count ->
    [one] 1 thing to know about this import
   *[other] { $count } things to know about this import
}
project-import-report-title = About this import
# Error toast: a short reason in the body, the full technical chain behind Details.
project-import-error-title = Could not import the project
# Why a project was not imported, whichever importer read it. A $file is a file
# name, a $folder a full path (it is the folder the writer has to act on), a
# $reason what the system or a parser said, a $limit a number.
project-import-failed-cancelled = The import was canceled. Nothing was written.
project-import-failed-target-exists = “{ $file }” already exists, and the import was not allowed to replace it. Nothing was written.
project-import-failed-target-folder = “{ $file }” is a folder, perhaps a project kept as a folder, and an import cannot replace it. Choose another name. Nothing was written.
project-import-failed-not-written = “{ $file }” could not be written in “{ $folder }” ({ $reason }). Nothing was written.
project-import-failed-disk-full = There is not enough space left in “{ $folder }” to write “{ $file }”. Free some space there, then import again. Nothing was written.
project-import-failed-source-missing = “{ $file }” is no longer there. Nothing was written.
project-import-failed-unreadable = Skribisto could not read “{ $file }” ({ $reason }). Nothing was written.
project-import-failed-archive-damaged = “{ $file }” is a compressed file that could not be read, probably because it is damaged ({ $reason }). Nothing was written.
# $entries and $limit are numbers of files; $entries is always more than $limit.
project-import-failed-archive-entries = The project was not imported: its archive lists { $entries ->
    [one] { $entries } file
   *[other] { $entries } files
}, more than the { $limit } a project may hold, which only a file built to exhaust memory does. Nothing was written.
project-import-failed-archive-member = The project was not imported: its part “{ $part }” unpacks to more than { $limit } MB, which only a file built to exhaust memory does. Nothing was written.
project-import-failed-archive-size = The project was not imported: it unpacks to more than { $limit } MB, which only a file built to exhaust memory does. Nothing was written.
project-import-failed-nested = The project was not imported: its part “{ $part }” is nested more than { $limit } levels deep, and reading it could crash Skribisto. Nothing was written.
project-import-failed-entities = The project was not imported: its part “{ $part }” declares XML entities. Expanding them could take more memory than the computer has and crash Skribisto. Nothing was written.
project-import-failed-folders = The project was not imported: one of its files sits more than { $limit } folders deep, and reading it could crash Skribisto. Nothing was written.
project-import-failed-malformed = The project was not imported: its part “{ $part }” is damaged and cannot be read ({ $reason }). Nothing was written.
# A Scrivener project the importer cannot bring across (a Scrivener 1 for Mac project, a
# damaged binder file). Why is the Scrivener importer's own to say; Details holds its key.
project-import-failed-scrivener-refused = Skribisto cannot import this Scrivener project. Nothing was written.
# The Scrivener project asks something its review did not answer: a project or a binder
# file appeared in it since. Shown with no button to press.
project-import-failed-scrivener-question = The Scrivener project asks a question its review did not answer. Start its import again and answer it, then import. Nothing was written.
# The Scrivener project needs the writer to tick a box before it imports (to import it
# without changes made on an iPhone or iPad, or with its documents empty), and the import
# was started without it. Shown with no button to press.
project-import-failed-scrivener-not-accepted = Before it can be imported, the Scrivener project needs you to tick a box on the Project step, and it was not ticked. Start its import again and tick it, then import. Nothing was written.
# The Scrivener project changed after the writer reviewed it. Shown with no button to press.
project-import-failed-scrivener-changed = The Scrivener project changed after you reviewed it. Start its import again and review it, then import. Nothing was written.
# An error nothing above accounts for; $detail is its technical description.
project-import-failed-internal = The import stopped on an error Skribisto did not expect: { $detail }

## Project import, shared by the Plume Creator and Manuskript dialogs
# The chosen destination is a project open in a window, here or in another
# running copy of Skribisto. The import is refused rather than written over it.
import-target-open-title = This project is open
import-target-open-text = “{ $name }” is open in Skribisto. Importing over it would replace the project you are working on, and your next save would write that project back over the import. Choose another file name, or close the project first.
# Spoken by a screen reader when an import's report notice appears, never shown.
# $result is the import's own result line ("Imported 12 items."), $notice the
# notice's title ("2 things to know about this import"). The notice is read out
# over the result, so it says both.
import-warnings-announcement = { $result } { $notice }.
# Quitting while an import runs. $name is the file the import is writing.
quit-import-running-title = An import is still running
quit-import-running-text = Skribisto is still importing “{ $name }”. Quitting now stops the import: nothing is written, and the project you are importing from is left as it was.
quit-import-stop = Stop the import and quit

## Import a Scrivener project: the wizard
import-scrivener-title = Import a Scrivener project
import-scrivener-step-project = Project
import-scrivener-step-review = Review
import-scrivener-step-save = Save
import-scrivener-back = Back
import-scrivener-next = Next
import-scrivener-import = Import
import-scrivener-cancel = Cancel
import-scrivener-help = Help

## Project step: choosing the project
import-scrivener-drop-title = Drop a Scrivener project here
import-scrivener-drop-hint = A .scriv project, the .scrivx file inside it, a backup .zip or a .scrivtemplate
import-scrivener-source = Scrivener project
import-scrivener-source-placeholder = Path to the project
import-scrivener-choose-file = Choose a file…
import-scrivener-choose-folder = Choose a folder…
# Windows and Linux show a .scriv project as an ordinary folder.
import-scrivener-source-hint = Choose the project’s .scriv folder, or open it and choose the .scrivx file inside. A backup .zip works too.
# macOS shows a .scriv project as a single file when Scrivener is installed, as a folder when it is not.
import-scrivener-source-hint-mac = If the project shows as a single file, use “Choose a file…”. If it shows as a folder, use “Choose a folder…”. A backup .zip works too.
# File-dialog titles and filter label, resolved when the dialog opens.
import-scrivener-dialog-file-title = Choose a Scrivener project
import-scrivener-dialog-folder-title = Choose a Scrivener project folder
import-scrivener-dialog-filter = Scrivener project
import-scrivener-source-missing = Nothing is at this path
# A path whose name is not Unicode: the field shows it with its unreadable letters replaced.
import-scrivener-source-unreadable-name = This name cannot be read. Rename the project in your file manager, then choose it again.
import-scrivener-empty-hint = Choose a project to see what Skribisto finds in it.
import-scrivener-reading = Reading the project…
import-scrivener-stop-reading = Stop reading
import-scrivener-read-failed = The project could not be read
# Shown above the two buttons it names: import-scrivener-check-again and
# import-scrivener-details. Quote them exactly as you translated them.
import-scrivener-read-failed-body = Something went wrong while reading it. Choose “Read the project again” to try once more, or “Details” to see what went wrong.
import-scrivener-details = Details
# The wizard's button that reads the Scrivener project from disk again, as it is now:
# after a refusal whose cause may go away, or a reading that failed. Not the action of
# import-scrivener-review-again: keep the two labels clearly different.
import-scrivener-check-again = Read the project again
# A refusal this version does not know how to word.
import-scrivener-refusal-unknown-title = This project cannot be imported
import-scrivener-refusal-unknown-body = Skribisto does not know how to bring it across. The details name the reason.

## Project step: the project card
import-scrivener-made-with-mac = Last saved by Scrivener { $version } for macOS
import-scrivener-made-with-windows = Last saved by Scrivener { $version } for Windows
import-scrivener-made-with-linux = Last saved by Scrivener { $version } for Linux
import-scrivener-made-with-ios = Last saved by Scrivener { $version } for iPhone and iPad
import-scrivener-made-with-version = Last saved by Scrivener { $version }
import-scrivener-made-with-unknown = Scrivener project, format { $format }
import-scrivener-saved-on = Saved on { $date } on “{ $device }”
import-scrivener-saved-on-no-device = Saved on { $date }
# $documents is how many documents (texts, not folders) the project holds; $manuscript is
# how many of them are in the manuscript (Scrivener's Draft folder).
import-scrivener-card-counts = { $documents ->
    [one] 1 document
   *[other] { $documents } documents
}, { $manuscript } in the manuscript

## Review step
import-scrivener-review-hint = This is how your binder will come across. Nothing is written until you import, and many items can change kind afterwards in the binder, with Convert to.
import-scrivener-choices-structure = Levels
import-scrivener-top-level = Top-level folders
import-scrivener-choices-text = Text
import-scrivener-choice-containers = Folders at level { $depth }
import-scrivener-choice-texts = Documents at level { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 item
   *[other] { $count } items
}
import-scrivener-choice-count-example = { $count ->
    [1] 1 item: “{ $example }”
   *[other] { $count } items, such as “{ $example }”
}
import-scrivener-rung-part = Parts
import-scrivener-rung-chapter = Chapters
import-scrivener-rung-plain = Plain folders
import-scrivener-texts-scenes = Scenes
import-scrivener-texts-chapters = Chapters
import-scrivener-choice-books = Books in the manuscript
import-scrivener-books-one = One book
import-scrivener-books-several = One book per folder
import-scrivener-choice-split = { $count ->
    [1] Split “{ $title }” at its heading
   *[other] Split “{ $title }” at its { $count } headings
}
import-scrivener-root-items = { $count ->
    [one] 1 item
   *[other] { $count } items
}
# Under a top-level folder's name on the Review page: how many items it holds, and that it
# reads like manuscript.
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 item. This folder reads like manuscript.
   *[other] { $count } items. This folder reads like manuscript.
}
import-scrivener-dest-notes = Notes
import-scrivener-dest-story-bible = Story bible
import-scrivener-dest-manuscript = Another book
import-scrivener-dest-leave-out = Leave out
import-scrivener-choice-trash = { $count ->
    [one] Bring the Trash across (1 item)
   *[other] Bring the Trash across ({ $count } items)
}
import-scrivener-trash-detail = { $count ->
    [0] None of them holds any writing.
    [one] 1 of them holds writing.
   *[other] { $count } of them hold writing.
}
import-scrivener-choice-blank-lines = Blank lines become scene breaks
import-scrivener-choice-ornaments = Lines of ornaments become scene breaks
import-scrivener-preview-label = Structure of the new project
import-scrivener-col-title = Title
import-scrivener-col-becomes = Becomes
import-scrivener-col-exports = In exports
import-scrivener-in-exports-yes = Yes
import-scrivener-in-exports-no = No
# What a row the binder has no creation command for becomes.
import-scrivener-becomes-item = Item
import-scrivener-becomes-trashed = In the Trash
import-scrivener-before-title = Before you import
import-scrivener-review-error-unreadable = The review could not be prepared. Choose the project again.
import-scrivener-review-error-choice = This choice does not fit the project. Choose the project again.
import-scrivener-review-error-unmappable = With this choice, the binder could not be brought across without losing something. Choose differently, or report the problem.

## Save step
import-scrivener-save-hint = The import makes a new Skribisto project. Your Scrivener project is only read, never changed.
import-scrivener-title-field = Title
import-scrivener-author = Author
import-scrivener-language = Language
import-scrivener-location-inside-project = Choose a folder outside the Scrivener project. Scrivener keeps only its own files in there.
import-scrivener-copy-research-label = Copy research files beside the new project
import-scrivener-copy-research = { $count ->
    [one] 1 research file Skribisto cannot open, such as a PDF or a sound, goes into a folder beside the project. Unticked, its note says where it is in your Scrivener project.
   *[other] { $count } research files Skribisto cannot open, such as PDFs or sounds, go into a folder beside the project. Unticked, their notes say where they are in your Scrivener project.
}
# The file name proposed when the project's name holds nothing a file name can.
import-scrivener-default-name = Scrivener project

## Progress and result
import-scrivener-progress-title = Importing Scrivener project…
import-scrivener-done-title = Scrivener project imported
# How many rows the new project holds: the documents and folders that came across, and
# the rows the import made for them (a document's notes, the project notes).
import-scrivener-done = { $items ->
    [one] The new project holds 1 item.
   *[other] The new project holds { $items } items.
}
# The same, with how many kinds of thing could not be read or need checking.
import-scrivener-done-check = { $items ->
    [one] The new project holds 1 item.
   *[other] The new project holds { $items } items.
} { $findings ->
    [one] 1 thing needs a look.
   *[other] { $findings } things need a look.
}
# The button of the notice a failed import shows: it opens the import wizard again on the
# same Scrivener project, with what the writer chose for the import that failed. Not the
# action of import-scrivener-check-again. Keep it short: it shares one line of the notice
# with “Details”.
import-scrivener-review-again = Review the import again
## The import's report (the dialog See report opens)
import-scrivener-see-report = See report
import-scrivener-report-from = Read from { $path }
import-scrivener-report-to = Written to { $path }
import-scrivener-report-came-across = What came across
import-scrivener-report-nothing-counted = Nothing was counted.
import-scrivener-report-items = { $count ->
    [one] 1 item
   *[other] { $count } items
}
import-scrivener-report-words = { $count ->
    [one] 1 word
   *[other] { $count } words
}
import-scrivener-report-synopses = { $count ->
    [one] 1 synopsis
   *[other] { $count } synopses
}
import-scrivener-report-document-notes = { $count ->
    [one] 1 document’s notes
   *[other] { $count } documents’ notes
}
import-scrivener-report-comments = { $count ->
    [one] 1 comment
   *[other] { $count } comments
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 footnote
   *[other] { $count } footnotes
}
import-scrivener-report-pictures = { $count ->
    [one] 1 picture
   *[other] { $count } pictures
}
import-scrivener-report-templates = { $count ->
    [one] 1 note template
   *[other] { $count } note templates
}
import-scrivener-report-tags = { $count ->
    [one] 1 tag
   *[other] { $count } tags
}
import-scrivener-report-statuses = { $count ->
    [one] 1 status
   *[other] { $count } statuses
}
import-scrivener-report-trashed = { $count ->
    [one] 1 item in the Trash
   *[other] { $count } items in the Trash
}
import-scrivener-report-research-files = { $count ->
    [one] 1 research file
   *[other] { $count } research files
}
import-scrivener-report-choices = What you chose
import-scrivener-report-accepted-mobile = You imported the project without the changes made in Scrivener for iPhone or iPad that were not in it yet.
import-scrivener-report-accepted-no-text = You imported this copy with its documents empty, as none of their text was in it.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = yes
import-scrivener-report-no = no
import-scrivener-report-research-copied = Research files Skribisto cannot open were copied into “{ $folder }”, beside the new project.
import-scrivener-report-research-not-copied = Research files Skribisto cannot open were not copied. Their notes say where they are in your Scrivener project.
import-scrivener-report-research-copied-except = { $count ->
    [one] Research files Skribisto cannot open were copied into “{ $folder }”, beside the new project, except 1 that stays in your Scrivener project. Its note says where it is.
   *[other] Research files Skribisto cannot open were copied into “{ $folder }”, beside the new project, except { $count } that stay in your Scrivener project. Their notes say where they are.
}
# The writer asked for research files to be copied and none was: $count is how many
# stay in the Scrivener project, which is all of them.
import-scrivener-report-research-none-copied = { $count ->
    [1] You asked for research files to be copied, but the only one could not be. It stays in your Scrivener project, and its note says where it is.
   *[other] You asked for research files to be copied, but none of the { $count } could be. They stay in your Scrivener project, and their notes say where they are.
}
import-scrivener-report-chose-project = You chose to import “{ $project }”, one of several projects in the folder or archive you picked.
import-scrivener-report-chose-manifest = You chose the binder file “{ $file }”.
import-scrivener-report-chose-enclosing = You picked something inside “{ $project }”, so the whole project was imported.
import-scrivener-report-chose-upgraded = You imported “{ $current }”, the project you went on writing in, rather than the copy Scrivener kept before an update.
import-scrivener-report-chose-backup = You imported “{ $backup }”, the older copy Scrivener kept before an update.
import-scrivener-report-title-placeholder-rejected = The title in the project’s settings was a template placeholder, so Skribisto proposed the project’s name instead.
import-scrivener-report-author-placeholder-rejected = The author’s name in the project’s settings was a template placeholder, so Skribisto proposed none.
import-scrivener-report-title-from-compile-settings = Skribisto proposed the title the project’s compile settings state: the name the project had when it was created, unless a title was set there in Scrivener.
import-scrivener-report-title-from-template = Skribisto proposed the template’s own title.
import-scrivener-report-title-from-folder-name = Skribisto proposed the project’s name as its title.
import-scrivener-report-title-from-project-properties = Skribisto proposed the title the project’s properties state.
import-scrivener-report-author-from-project = Skribisto proposed the author’s name the project’s settings state. A template’s author can linger there, and it would print on a title page.
import-scrivener-report-language-not-stated = The project did not state its language, so Skribisto proposed the one it shows itself in. It decides how words are counted and quotes are written.
import-scrivener-report-language-from-script = Skribisto proposed the language from the script the manuscript is written in ({ $tag }).
# A proposal of the Save page the writer replaced with their own. $hint is the sentence
# saying what Skribisto proposed: a title, an author's name or a language.
import-scrivener-report-you-changed-it = { $hint } You replaced this proposal.
import-scrivener-report-content-salvaged-text = The text of “{ $item }” was missing from the project. Its words come from Scrivener’s search index, as plain text and perhaps from an older version: check them.
import-scrivener-report-content-salvaged-notes = The notes of “{ $item }” were missing from the project. Their words come from Scrivener’s search index, as plain text and perhaps from an older version: check them.
import-scrivener-report-content-salvaged-synopsis = The synopsis of “{ $item }” was missing from the project. Its words come from Scrivener’s search index, as plain text and perhaps from an older version: check them.
# A finding of the document importer found in several places: its sentence about the
# first, then how many more the list under it names.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } The same happens in 1 more place, listed below.
   *[other] { $sentence } The same happens in { $others } more places, listed below.
}
# An item of the report, by its path in Scrivener's binder, with which of its writings
# the line is about when it is not the text.
import-scrivener-report-item-notes = { $path } (document notes)
import-scrivener-report-item-synopsis = { $path } (synopsis)
import-scrivener-report-item-project-note = { $path } (project note)
import-scrivener-report-item-recovered = { $path } (recovered text)
# An item with no name in Scrivener.
import-scrivener-report-untitled = Untitled
import-scrivener-report-no-findings = The import has nothing more to report.
import-scrivener-report-group-unread = Could not be read
import-scrivener-report-group-check = Check these
import-scrivener-report-group-changed = Brought across differently
import-scrivener-report-group-left = Left behind
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 item
   *[other] { $count } items
}
# The same, as a screen reader names it: with the line whose items it lists.
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 item: { $line }
   *[other] { $count } items: { $line }
}
# A button that shows the names a report line is about, as the project wrote them: labels
# or keywords no document wears, collections, statuses, tags or custom fields.
import-scrivener-report-names-toggle = { $count ->
    [one] 1 name
   *[other] { $count } names
}
# The same, as a screen reader hears it, naming the line it belongs to.
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 name: { $line }
   *[other] { $count } names: { $line }
}
# Under a list of items or names (import-scrivener-report-items-toggle,
# import-scrivener-report-names-toggle) that shows only the first ones: how many more the
# list holds that are not shown. Word it so it fits either kind of list.
import-scrivener-report-more = { $count ->
    [one] And 1 more.
   *[other] And { $count } more.
}
import-scrivener-report-open-item = Open in the editor
import-scrivener-report-links-hint = Open the new project, then show this report again from the command palette: each item listed here then opens with a click.
import-scrivener-copy-report = Copy report
import-scrivener-report-copied = Report copied
import-scrivener-report-copy-failed = The report could not be copied.
import-scrivener-report-close = Close
import-scrivener-report-none-title = No import report
import-scrivener-report-none = No Scrivener import has finished since Skribisto started. Skribisto keeps the reports of the latest imports until it closes.
# $count is how many import reports Skribisto keeps (16 at present).
import-scrivener-report-gone = { $count ->
    [one] The report of this import is no longer kept. Skribisto keeps the report of the last { $count } import until it closes.
   *[other] The report of this import is no longer kept. Skribisto keeps the reports of the last { $count } imports until it closes.
}

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manuscript
import-scrivener-notes-binder = Notes
import-scrivener-notes-of = { $title } (notes)
import-scrivener-project-notes = Project notes
import-scrivener-recovered = Recovered texts
import-scrivener-untitled-template = Untitled template
# The folder beside the new project its research files are copied into. A folder name on
# disk: keep it free of the characters a file name cannot hold (/ \ : * ? " < > |).
import-scrivener-research-folder = { $name } research files
# Between two authors' names.
import-scrivener-list-separator = {", "}
# Here and in the lines about labels below, “label” is Scrivener's Label: the colored label
# each document can wear in Scrivener, which the import turns into a tag. It is not
# Skribisto's own Label (overview-col-label, the short note under an item's title): if
# your language has one word for both, use the word Scrivener uses for its Label, or one
# that does not clash.
import-scrivener-label-details = From the “{ $field }” field in Scrivener
import-scrivener-status-details = From the “{ $field }” field in Scrivener
import-scrivener-unnamed-label = Label { $number }
import-scrivener-unnamed-status = Status { $number }
import-scrivener-label-tag = { $name } (label)
import-scrivener-untitled-tag = Untitled
# The tags, the pace plan and the "From Scrivener" folder the import makes from what a
# Scrivener project holds beyond its labels and status.
import-scrivener-keyword-details = The keyword “{ $path }” in Scrivener
import-scrivener-unnamed-keyword = Keyword { $number }
import-scrivener-keyword-tag = { $name } (keyword)
import-scrivener-collection-details = A collection in Scrivener
import-scrivener-collection-tag = { $name } (collection)
import-scrivener-field-details = From the “{ $field }” field in Scrivener
# A tag made from one option of a list field.
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (field)
# The day off a pace plan takes on its deadline.
import-scrivener-deadline-day = Deadline
import-scrivener-from-scrivener = From Scrivener
import-scrivener-note-project = Project
import-scrivener-note-project-bookmarks = Project bookmarks
import-scrivener-note-replacements = Compile replacements
import-scrivener-note-auto-complete = Auto-complete entries
import-scrivener-note-saved-searches = Saved searches
import-scrivener-note-custom-metadata = Custom metadata
import-scrivener-note-targets = Targets
import-scrivener-note-bookmarks = Bookmarks
import-scrivener-abbreviated-title = Abbreviated title
import-scrivener-subjects = Subjects
import-scrivener-company = Company
import-scrivener-session-target = Session target
import-scrivener-session-target-calculated = Session target, calculated by Scrivener from the deadline
import-scrivener-writing-days = Writing days
import-scrivener-weekday-monday = Monday
import-scrivener-weekday-tuesday = Tuesday
import-scrivener-weekday-wednesday = Wednesday
import-scrivener-weekday-thursday = Thursday
import-scrivener-weekday-friday = Friday
import-scrivener-weekday-saturday = Saturday
import-scrivener-weekday-sunday = Sunday
import-scrivener-draft-target = Draft target
import-scrivener-deadline = Deadline
# The unit of a target, in parentheses after its number in the note that lists the
# project's targets: “80000 (words)”. A label for the unit, not part of a sentence, and no
# count reaches it: use the form your language gives a unit standing alone, as in a
# column heading. The same for the two keys below.
import-scrivener-unit-words = words
import-scrivener-unit-characters = characters
import-scrivener-unit-pages = pages
# The labels of one saved search, in the note that lists them, each followed by a colon and
# its value: the words searched for (quoted), where Scrivener searched (“titles, text”),
# and how the words had to match (“any word”, “all words”, “exact phrase”).
import-scrivener-search-query = Searched for
import-scrivener-search-in = Searched in
import-scrivener-search-operator = Match
# What a saved search looks in, by the kind Scrivener names.
import-scrivener-search-in-all = everything
import-scrivener-search-in-title = titles
import-scrivener-search-in-text = text
import-scrivener-search-in-notes = notes
import-scrivener-search-in-synopsis = synopses
import-scrivener-search-in-keywords = keywords
import-scrivener-search-in-label = labels
import-scrivener-search-in-status = statuses
import-scrivener-search-in-section-type = section types
import-scrivener-search-in-custom-metadata = custom metadata
# How a saved search's words combine.
import-scrivener-search-any = any word
import-scrivener-search-all = all words
import-scrivener-search-exact = exact phrase
# One line of a listing: a bookmark's place and its note, or a link's words and its address.
import-scrivener-label-value = { $label }: { $value }
# Said of a compile replacement (a “replace this with that” rule of Scrivener's compile),
# in parentheses after it: “`teh` → `the` (regular expression, switched off)”. Agree with
# your word for “replacement” if your language needs to. The same for the key below.
import-scrivener-replacement-regex = regular expression
import-scrivener-replacement-off = switched off
# In the notes that list custom metadata and bookmarks, said after the path of a document
# that became a note: its values or bookmarks are listed in that note's own text.
import-scrivener-in-own-note = Listed in the note this document became.


## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

# A refusal on one line: its title, then what to do.
import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
## A body that ends “then read the project again” is shown in the wizard beside the button
## import-scrivener-check-again, so it uses the same words. A failed import's notice shows
## the same body beside import-scrivener-review-again, which reads the project again too:
## the body quotes no button.
import-scrivener-refusal-missing-title = Nothing is at this path
import-scrivener-refusal-missing-body = “{ $file }” is not there any more. Choose the project again.
import-scrivener-refusal-unreadable-title = This project cannot be read
import-scrivener-refusal-unreadable-body = Skribisto could not read “{ $file }” ({ $reason }). Check that you can open it, then try again.
import-scrivener-refusal-not-a-scrivener-project-title = No Scrivener project here
import-scrivener-refusal-not-a-scrivener-project-body = Choose the project’s .scriv folder, the .scrivx file inside it, a backup .zip or a .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = The documents are not in this copy
import-scrivener-refusal-manifest-without-files-body = “{ $manifest }” is here, but not the Files folder that holds the documents it lists. A sync service may not have finished copying the project, or only the .scrivx file was copied. If you chose the .scrivx file, choose the project’s folder instead with “Choose a folder…”.
import-scrivener-refusal-search-too-wide-title = This folder holds too much to search
import-scrivener-refusal-search-too-wide-body = It has more than { $limit } folders to look through. Choose the Scrivener project itself.
import-scrivener-refusal-scrivener-one-for-mac-title = This project was made by Scrivener 1 for Mac
import-scrivener-refusal-scrivener-one-for-mac-body = Skribisto reads projects from Scrivener 2 and later. Open it once in Scrivener 2 or 3 to update it, or compile it to a Word document and import that.
# A small file Scrivener for iPhone or iPad leaves in place of a project it holds: a
# stand-in for the project, not one of the template placeholders other keys mention
# (codes such as <$projecttitle>).
import-scrivener-refusal-ios-stub-without-mobile-data-title = This is only a placeholder from Scrivener for iPhone or iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = The project itself is not here. Sync it from your iPhone or iPad, or share it from there as a .zip, and import that.
import-scrivener-refusal-newer-title = This project was made by a newer Scrivener
import-scrivener-refusal-unknown-format-version-body = Its format, { $version }, is not one Skribisto knows. Check for a Skribisto update. Until then, compile it to a Word document in Scrivener and import that.
import-scrivener-refusal-newer-format-stamp-body = Its files are marked as version { $stamp }, and Skribisto reads up to version { $newest }. Check for a Skribisto update. Until then, compile it to a Word document in Scrivener and import that.
import-scrivener-refusal-version-stamp-unreadable-title = The project’s version cannot be read
import-scrivener-refusal-version-stamp-unreadable-body = Its Files/version.txt file holds no version number, so Skribisto cannot tell whether it reads this project. Open the project once in Scrivener, then import it again.
import-scrivener-refusal-damaged-manifest-title = The binder file cannot be read
import-scrivener-refusal-damaged-manifest-body = “{ $manifest }” is damaged. { $spare_copies ->
    [0] Open the project in Scrivener, which can repair it, then import it again.
    [1] Scrivener keeps a spare copy of it inside the project, which Skribisto does not read. Open the project in Scrivener, which can repair it, then import it again.
   *[other] Scrivener keeps { $spare_copies } spare copies of it inside the project, which Skribisto does not read. Open the project in Scrivener, which can repair it, then import it again.
}
# The empty file Scrivener writes for a new project before its first save: not damage.
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = “{ $manifest }” is the empty file Scrivener writes for a project it has not saved yet. Open the project in Scrivener and let it save, then import it again.
import-scrivener-refusal-manifest-too-large-title = This binder file is too large
import-scrivener-refusal-manifest-too-large-body = “{ $manifest }” is larger than the { $limit } MB Skribisto reads. No real project comes near that, so the file may be damaged.
import-scrivener-refusal-no-item-identifiers-title = The binder lists no document Skribisto can find
import-scrivener-refusal-no-item-identifiers-body = None of its items carries an identifier, so their files cannot be found. Open the project once in Scrivener, then import it again.
import-scrivener-refusal-incomplete-copy-title = The documents are missing from this copy
import-scrivener-refusal-incomplete-copy-body = The Files folder is here, but the documents in it are not. A sync service may not have finished copying the project. Wait for it to finish, or copy the whole .scriv folder again, then read the project again.
import-scrivener-refusal-archive-title = This archive cannot be opened
import-scrivener-refusal-archive-unreadable-body = “{ $file }” could not be read ({ $reason }). It may be damaged. Unpack it yourself, then choose the .scriv folder inside it.
import-scrivener-refusal-archive-unsafe-name-title = This archive cannot be opened safely
# A name with `..` in it, or an absolute one.
import-scrivener-refusal-archive-unsafe-name-body = “{ $file }” holds a file named “{ $member }”, which would land outside the archive, so Skribisto does not open it.
import-scrivener-refusal-archive-unsafe-name-empty-body = “{ $file }” holds a file with no name, so Skribisto does not open it.
import-scrivener-refusal-archive-unsafe-name-control-character-body = “{ $file }” holds a file whose name contains a control character, so Skribisto does not open it.
import-scrivener-refusal-archive-too-large-title = This archive is too large to open safely
import-scrivener-refusal-archive-too-many-members-body = It holds more than { $limit } files. Unpack it yourself, then choose the .scriv folder inside it.
import-scrivener-refusal-archive-member-too-large-body = “{ $member }” unpacks to more than { $limit } MB. Unpack the archive yourself, then choose the .scriv folder inside it.
import-scrivener-refusal-archive-too-large-body = It unpacks to more than { $limit } MB. Unpack it yourself, then choose the .scriv folder inside it.
import-scrivener-refusal-archive-encrypted-title = This archive is protected by a password
import-scrivener-refusal-archive-encrypted-body = Unpack “{ $file }” yourself, then choose the .scriv folder inside it.
import-scrivener-refusal-archive-unsupported-compression-title = This archive is packed in a way Skribisto cannot read
import-scrivener-refusal-archive-unsupported-compression-body = “{ $member }” is compressed with { $method }. Unpack the archive yourself, then choose the .scriv folder inside it.
import-scrivener-refusal-duplicate-archive-members-title = This archive lists some files twice
import-scrivener-refusal-duplicate-archive-members-body = “{ $file }” holds more than one file under the same name, so Skribisto cannot tell which one belongs to the project. Unpack it yourself, then choose the .scriv folder inside it.
# What a file cut short looks like; a file too large, one that is not a template and one
# encoded another way have their own words.
import-scrivener-refusal-template-undecodable-title = This template file is incomplete
import-scrivener-refusal-template-undecodable-too-large-title = This template file is too large to open
import-scrivener-refusal-template-undecodable-not-a-template-title = This file is not a Scrivener template
import-scrivener-refusal-template-undecodable-unknown-encoding-title = This template file is stored in a form Skribisto does not know
import-scrivener-refusal-template-undecodable-body = “{ $file }” cannot be decoded: it may end before the project inside it does. Download it again, or open it in Scrivener and make a project from it.
import-scrivener-refusal-template-undecodable-too-large-body = “{ $file }” is larger than { $limit } MB. Open it in Scrivener, make a project from it, then import that project.
import-scrivener-refusal-template-undecodable-not-a-template-body = “{ $file }” does not hold a Scrivener project. Choose the template file Scrivener saved, or a project made from it.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = “{ $file }” stores its project in a way Skribisto cannot decode. Open it in Scrivener, make a project from it, then import that project.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 file of this project is still in the cloud
   *[other] { $count } files of this project are still in the cloud
}
# Said under import-scrivener-refusal-cloud-placeholders-title. $count is the number of
# files that title gives; $examples names a few of them, or the only one.
import-scrivener-refusal-cloud-placeholders-body = { $count ->
    [1] It has not been downloaded to this computer: { $examples }. Make the project available offline in your sync app, then read the project again.
   *[other] They have not been downloaded to this computer, for example { $examples }. Make the project available offline in your sync app, then read the project again.
}
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] 1 file of this project cannot be read
   *[other] { $count } files of this project cannot be read
}
import-scrivener-refusal-unreadable-members-body = For example { $examples }. Check that you are allowed to read the whole project and that the disk is healthy, then read the project again.
import-scrivener-refusal-needed-member-is-link-title = This project points outside its own folder
import-scrivener-refusal-needed-member-is-link-body = “{ $member }” is a link to a place elsewhere on this computer, and Skribisto does not follow links. Copy the project so that every file is inside it, then try again.
import-scrivener-refusal-project-changed-during-read-title = The project changed while it was being read
import-scrivener-refusal-project-changed-during-read-body = Scrivener or a sync service is still writing to it. Close it in Scrivener, wait for the sync to finish, then read the project again.
import-scrivener-refusal-too-many-members-title = This project holds too many files
import-scrivener-refusal-too-many-members-body = It has more than { $limit } files, more than any real project. It may not be a Scrivener project, or it may be damaged.
import-scrivener-refusal-too-large-title = This project is too large
import-scrivener-refusal-too-large-body = Its writing comes to more than { $limit } MB, more than Skribisto can import safely.
import-scrivener-refusal-structure-unmappable-title = Skribisto cannot map this binder
import-scrivener-refusal-structure-unmappable-body = The binder could not be brought across without losing or misplacing something. This is a fault in Skribisto, not in your project. Please report it with what “Details” shows.

## When there is a choice to make
# $count is 2 or more: how many Scrivener projects the chosen folder or archive holds.
import-scrivener-roots-title = { $count ->
    [one] { $count } project is here. Which one?
   *[other] { $count } projects are here. Which one?
}
import-scrivener-roots-hint = This folder or archive holds more than one Scrivener project. Choose the one to import.
# $count is 2 or more: how many binder files (.scrivx) the project holds.
import-scrivener-manifests-title = { $count ->
    [one] This project has { $count } binder file
   *[other] This project has { $count } binder files
}
import-scrivener-manifests-hint = Scrivener keeps the binder, the list of every document and folder, in a .scrivx file. The extra ones were most likely left by a sync service. Choose the one you last worked in.
import-scrivener-manifests-hint-preselected = Scrivener keeps the binder, the list of every document and folder, in a .scrivx file. The extra ones were most likely left by a sync service. The one selected is the one you most likely worked in last: check it before going on.
# A binder file to choose, on one line: its file name, when it was saved, and how many
# items its binder lists. $documents counts every item, folders included.
import-scrivener-manifest-row = { $file } · saved on { $date } · { $documents ->
    [one] 1 item
   *[other] { $documents } items
}
# The same, when the binder file does not say when it was saved.
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 item
   *[other] { $documents } items
}
import-scrivener-manifest-row-unread = { $file } · cannot be read
import-scrivener-enclosing-title = This is part of the Scrivener project “{ $project }”
import-scrivener-enclosing-body = You chose a file or folder inside a project. Skribisto imports the whole project.
import-scrivener-enclosing-use = Import “{ $project }”
import-scrivener-upgrade-title = This is the copy Scrivener kept before an update
import-scrivener-upgrade-body = When Scrivener updates a project to a newer version, it keeps the old one as “{ $backup }”. The project you went on writing in is “{ $current }”, beside it.
import-scrivener-upgrade-body-unclear = When Scrivener updates a project to a newer version, it keeps the old one as “{ $backup }”. The project beside it, “{ $current }”, could not be read cleanly, so choose which one to import.
import-scrivener-upgrade-use-current = Import “{ $current }”, the project you went on writing in
import-scrivener-upgrade-keep-backup = Import this older copy, “{ $backup }”

## What must be accepted before going on
import-scrivener-ios-pending-title = Changes made in Scrivener for iPhone or iPad are not in this copy yet
import-scrivener-ios-pending-body = Scrivener on a computer brings those changes in the next time it opens the project. Open the project once in Scrivener on your computer, then import it again.
import-scrivener-ios-pending-accept = Import it without those changes
import-scrivener-no-text-title = { $count ->
    [one] 1 document, and not one word of text in this copy
   *[other] { $count } documents, and not one word of text in this copy
}
import-scrivener-no-text-body = A sync service may still be copying the project, or the copy lost its documents. Imported now, every document arrives empty.
import-scrivener-no-text-accept = Import the documents empty

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = The project was found inside the folder you chose
import-scrivener-notice-project-found-below-pick-body = It is at “{ $path }”.
import-scrivener-notice-source-is-template-title = This is a Scrivener template
import-scrivener-notice-source-is-template-named = This is the Scrivener template “{ $title }”
import-scrivener-notice-source-is-template-body = Importing it makes a new project from what the template holds, its instructions included.
import-scrivener-notice-source-is-backup-title = This looks like a backup Scrivener made
import-scrivener-notice-source-is-backup-body = It holds the project as it was when the backup was made. If you went on writing after that, choose the project itself.
import-scrivener-notice-importing-pre-upgrade-copy-title = You are importing the copy Scrivener kept before an update
import-scrivener-notice-importing-pre-upgrade-copy-body = The project you went on writing in is beside it, and holds anything written since.
import-scrivener-notice-imported-from-ios-stub-title = This project was made in Scrivener for iPhone or iPad
import-scrivener-notice-imported-from-ios-stub-body = It has never been opened on a computer, so it is read from the copy the iPhone or iPad keeps.
import-scrivener-notice-stale-mobile-binder-ignored-title = An old binder from an iPhone or iPad is set aside
import-scrivener-notice-stale-mobile-binder-ignored-body = Scrivener on a computer already brought its changes in.
# $count is 2 or more: every binder file (.scrivx) of the project is the same.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] This project has { $count } identical binder file
   *[other] This project has { $count } identical binder files
}
import-scrivener-notice-identical-manifest-copies-body = They hold the same thing, so any of them gives the same project.
import-scrivener-notice-manifest-copies-ignored-title = Extra copies of the binder file are set aside
import-scrivener-notice-manifest-copies-ignored-body = “{ $chosen }” is the one Scrivener last worked in. { $count ->
    [one] 1 copy left by a sync service is set aside.
   *[other] { $count } copies left by a sync service are set aside.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = The binder file you chose looks like a sync copy
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = “{ $name }” is named like a copy of “{ $original }”, which is beside it. If the import looks old, choose “{ $original }” instead.
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] 1 other binder file is beside the one you chose
   *[other] { $count } other binder files are beside the one you chose
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Only the one you chose is read.
import-scrivener-notice-legacy-v1-files-beside-title = Files from an older Scrivener are set aside
import-scrivener-notice-legacy-v1-files-beside-body = They were left behind when the project was updated. Only the files of the current version are read.
import-scrivener-notice-mixed-payload-trees-title = Files of two Scrivener versions are in this project
import-scrivener-notice-mixed-payload-trees-body = The documents are read where the binder file says they are.
import-scrivener-notice-version-disagreement-title = This project’s files do not all agree on its version
import-scrivener-notice-id-scheme-mismatch-body = The binder file says version { $version } but names its documents the way another version does. They are read where it points. If the import looks wrong, open the project once in Scrivener and import it again.
import-scrivener-notice-version-stamp-mismatch-body = It is read the way its binder file describes it. If the import looks wrong, open the project once in Scrivener and import it again.
import-scrivener-notice-unrecognised-writer-title = This project was saved by a program Skribisto does not recognise
import-scrivener-notice-unrecognised-writer-named = It names itself “{ $creator }”. It is read as a Scrivener project.
import-scrivener-notice-unrecognised-writer-body = It does not name itself. It is read as a Scrivener project.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 binder item carries an identifier that is not one
   *[other] { $count } binder items carry an identifier that is not one
}
import-scrivener-notice-invalid-item-identifier-body = An identifier like that is set aside, and the item that carries it is kept.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 binder item has no identifier
   *[other] { $count } binder items have no identifier
}
import-scrivener-notice-items-without-identifier-body = Without an identifier, an item’s files cannot be found, so it arrives empty.
import-scrivener-notice-duplicate-item-key-title = Two binder items share one identifier: “{ $item }”
import-scrivener-notice-duplicate-item-key-body = The first one keeps the documents. The second keeps its place and arrives empty.
import-scrivener-notice-lock-here-title = Scrivener has this project open on this computer
import-scrivener-notice-lock-here-body = Close it in Scrivener first, so that every change is saved.
import-scrivener-notice-lock-title = Scrivener had this project open on “{ $host }”
import-scrivener-notice-lock-elsewhere-title = Scrivener had this project open on another computer
import-scrivener-notice-lock-body = If it is still open there, close it first: the files may not hold the latest changes yet. If Scrivener was closed long ago, you can import it as it is.
import-scrivener-notice-archived-while-open-title = This archive was made while Scrivener had the project open
import-scrivener-notice-archived-while-open-body = Changes made after that moment may be missing from it.
import-scrivener-notice-content-missing-notes = The notes of “{ $item }” are missing
import-scrivener-notice-content-missing-synopsis = The synopsis of “{ $item }” is missing
import-scrivener-notice-content-missing-text = The text of “{ $item }” is missing
# Said under one of the three titles above: $place is what is missing, “notes” (the
# document's notes), “synopsis” or “text”. Make each variant agree with it.
import-scrivener-notice-content-missing-body = { $place ->
    [notes] Their file is not in this copy, so they arrive empty. A sync service may still be copying the project.
    [synopsis] Its file is not in this copy, so it arrives empty. A sync service may still be copying the project.
   *[text] Its file is not in this copy, so it arrives empty. A sync service may still be copying the project.
}
# The same, when Scrivener's search index still held the words: $place as above.
import-scrivener-notice-content-missing-salvaged = { $place ->
    [notes] Their file is not in this copy, but Scrivener’s search index still held their words. They are kept as plain text, perhaps from an older version.
    [synopsis] Its file is not in this copy, but Scrivener’s search index still held its words. They are kept as plain text, perhaps from an older version.
   *[text] Its file is not in this copy, but Scrivener’s search index still held its words. They are kept as plain text, perhaps from an older version.
}
import-scrivener-notice-content-truncated-notes = The notes of “{ $item }” are empty on disk
import-scrivener-notice-content-truncated-synopsis = The synopsis of “{ $item }” is empty on disk
import-scrivener-notice-content-truncated-text = The text of “{ $item }” is empty on disk
# Said under one of the three titles above: $place is what is empty, “notes”,
# “synopsis” or “text”. Make each variant agree with it.
import-scrivener-notice-content-truncated-body = { $place ->
    [notes] Scrivener recorded them with text, but their file holds none: a sync may have stopped halfway. They arrive empty.
    [synopsis] Scrivener recorded it with text, but its file holds none: a sync may have stopped halfway. It arrives empty.
   *[text] Scrivener recorded it with text, but its file holds none: a sync may have stopped halfway. It arrives empty.
}
import-scrivener-notice-missing-media-title = The file of “{ $item }” is missing
import-scrivener-notice-missing-media-body = The picture or research file it holds is not in this copy.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] 1 sync copy of a document was found
   *[other] { $count } sync copies of documents were found
}
# $missing is how many of the copies are of a document the project no longer holds.
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] Sync copies come across as recovered texts, in a folder of their own, so you can compare them with the documents.
    [one] Sync copies come across as recovered texts, in a folder of their own. { $missing } of them is a copy of a document no longer in the project.
   *[other] Sync copies come across as recovered texts, in a folder of their own. { $missing } of them are copies of documents no longer in the project.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] 1 document no binder item lists was found
   *[other] { $count } documents no binder item lists were found
}
import-scrivener-notice-recovered-unreferenced-text-body = Documents like these come across as recovered texts, in a folder of their own.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] 1 extra copy left by a sync service is set aside
   *[other] { $count } extra copies left by a sync service are set aside
}
import-scrivener-notice-sync-debris-ignored-body = Scrivener does not read such copies either. Only the project’s own files are read.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] 1 file that is not part of a Scrivener project is set aside
   *[other] { $count } files that are not part of a Scrivener project are set aside
}
import-scrivener-notice-examples = For example { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] 1 link inside the project is not followed
   *[other] { $count } links inside the project are not followed
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] 1 file the import does not need could not be read
   *[other] { $count } files the import does not need could not be read
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] 1 file still in the cloud is skipped
   *[other] { $count } files still in the cloud are skipped
}
import-scrivener-notice-cloud-placeholders-skipped-body = Nothing the import needs is still in the cloud.
import-scrivener-notice-no-text-found-title = { $count ->
    [1] 1 document, and it holds no text
   *[other] { $count } documents, and none holds any text
}
import-scrivener-notice-no-text-found-body = This project looks empty. It is imported as it is.
import-scrivener-notice-metadata-unreadable-title = “{ $member }” could not be read
import-scrivener-notice-metadata-unreadable-body = What Scrivener kept in it does not come across.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 reference points to a keyword the project does not define
   *[other] { $count } references point to keywords the project does not define
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 reference points to a label the project does not define
   *[other] { $count } references point to labels the project does not define
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 reference points to a status the project does not define
   *[other] { $count } references point to statuses the project does not define
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 reference points to a section type the project does not define
   *[other] { $count } references point to section types the project does not define
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 reference points to a custom metadata field the project does not define
   *[other] { $count } references point to custom metadata fields the project does not define
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 reference points to a document the project does not define
   *[other] { $count } references point to documents the project does not define
}
import-scrivener-notice-dangling-references-body = A reference to something the project does not define is left out.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 snapshot stays in your Scrivener project: Skribisto does not bring snapshots across.
   *[other] { $count } snapshots stay in your Scrivener project: Skribisto does not bring snapshots across.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] Scrivener’s record of words written each day, 1 day, is not brought across. Skribisto starts its own count from the first save.
   *[other] Scrivener’s record of words written each day, { $days } days, is not brought across. Skribisto starts its own count from the first save.
}
# A template placeholder is one of Scrivener's codes, such as <$projecttitle>, which
# Scrivener replaces with text when it compiles.
import-scrivener-notice-title-placeholder-rejected-hint = The title in the project’s settings is a template placeholder, so the project’s name is proposed instead.
import-scrivener-notice-author-placeholder-rejected-hint = The author’s name in the project’s settings is a template placeholder, so none is proposed.
import-scrivener-notice-title-from-project-hint-compile-settings = This title is the one the project’s compile settings state. Until a title is set there, Scrivener keeps the name the project had when it was created.
import-scrivener-notice-title-from-project-hint-template = This title is the template’s own.
import-scrivener-notice-title-from-project-hint-folder-name = This title is the project’s name.
import-scrivener-notice-title-from-project-hint-project-properties = This title is the one the project’s properties state.
import-scrivener-notice-author-from-project-hint = This name is the one the project’s settings state. A template’s author can linger there, and it would print on a title page.
import-scrivener-notice-language-not-stated-hint = The project does not state its language, so the language Skribisto shows itself in is proposed. It decides how words are counted and quotes are written.
import-scrivener-notice-language-from-script-hint = Proposed from the script the manuscript is written in ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [1] { $documents ->
        [1] The manuscript’s only document is written in script mode. Skribisto keeps its words, not its screenplay layout.
       *[other] 1 of the { $documents } manuscript documents is written in script mode. Skribisto keeps its words, not its screenplay layout.
    }
   *[other] { $script } of the { $documents } manuscript documents are written in script mode. Skribisto keeps their words, not their screenplay layout.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [1] A manuscript document holds its chapters as headings.
   *[other] { $count } manuscript documents hold their chapters as headings.
}
# The number in digits, in Roman numerals or in words. “Chapter II” and “Part One” are
# examples: translate them.
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 chapter or part title also includes its number, such as “Chapter II” or “Part One”, so an export may print the number twice.
   *[other] { $count } chapter or part titles also include their number, such as “Chapter II” or “Part One”, so an export may print the number twice.
}
# An item whose title or section type says it opens a book, a part or a chapter, inside an
# item that already opens one of the same level (a chapter inside a chapter).
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 item looks like a book, a part or a chapter, but it sits inside an item already at that level, so it comes across at a lower level.
   *[other] { $count } items look like books, parts or chapters, but each sits inside an item already at that level, so they come across at a lower level.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 document that follows a chapter becomes the end of that chapter.
   *[other] { $count } documents that follow a chapter become the end of that chapter.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 top-level folder kept in the notes reads like manuscript. You can send it to the manuscript as another book.
   *[other] { $count } top-level folders kept in the notes read like manuscript. You can send them to the manuscript as other books.
}
# “In prose” is the page of a story bible entry that lists where the text names it
# (segment-note-in-prose): use your translation of it. “Hero” is an example: translate it.
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 story bible entry still has a template’s title, such as “Hero”. Skribisto looks for that title in your text and lists every place it appears under In prose: rename the entry if that word is common in your book.
   *[other] { $count } story bible entries still have a template’s title, such as “Hero”. Skribisto looks for those titles in your text and lists every place they appear under In prose: rename the entries if those words are common in your book.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 title only named its level and is left blank: the binder and exports number it.
   *[other] { $count } titles only named their level and are left blank: the binder and exports number them.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 item had no title in Scrivener and stays untitled.
   *[other] { $count } items had no title in Scrivener and stay untitled.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 title held line breaks or tabs, which are removed.
   *[other] { $count } titles held line breaks or tabs, which are removed.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 folder has text of its own, which moves to a first document inside it.
   *[other] { $count } folders have text of their own, which moves to a first document inside them.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 document with documents inside it becomes a folder.
   *[other] { $count } documents with documents inside them become folders.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 empty folder stays a plain folder, so no export prints a heading Scrivener never printed.
   *[other] { $count } empty folders stay plain folders, so no export prints a heading Scrivener never printed.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 book inside a book becomes a part.
   *[other] { $count } books inside a book become parts.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prologue or epilogue is left out of the chapter numbering.
   *[other] { $count } prologues or epilogues are left out of the chapter numbering.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 front or back matter item from outside the manuscript goes inside the book as paratext, left out of exports until you include it.
   *[other] { $count } front or back matter items from outside the manuscript go inside the book as paratext, left out of exports until you include them.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 manuscript document becomes paratext (front or back matter), so { $words ->
        [one] { $words } word no longer counts
       *[other] { $words } words no longer count
    } as manuscript.
   *[other] { $documents } manuscript documents become paratext (front or back matter), so { $words ->
        [one] { $words } word no longer counts
       *[other] { $words } words no longer count
    } as manuscript.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] The notes of 1 document become a note beside it.
   *[other] The notes of { $count } documents become notes beside them.
}
import-scrivener-structure-draft-content-as-note-line = The manuscript folder’s own text becomes a note at the top of the manuscript.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 character or place folder becomes a story bible tag.
   *[other] { $count } character or place folders become story bible tags.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 document template becomes a note template.
   *[other] { $count } document templates become note templates.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 template folder inside another is flattened: each text in it becomes a template.
   *[other] { $count } template folders inside others are flattened: each text in them becomes a template.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 template holds what a note template cannot, and is kept as a note.
   *[other] { $count } templates hold what a note template cannot, and are kept as notes.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 text no binder item listed comes across in a folder of its own.
   *[other] { $count } texts no binder item listed come across in a folder of their own.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 project note comes across in a folder of its own.
   *[other] { $count } project notes come across in a folder of their own.
}
# $items is how many items the Trash holds, $writing how many of them hold writing.
import-scrivener-structure-trash-brought-across-line = { $items ->
    [1] The Trash comes across: 1 item, which you can restore from the Trash.
   *[other] The Trash comes across: { $items } items, { $writing ->
        [0] none of them with writing
        [one] 1 of them with writing
       *[other] { $writing } of them with writing
    }, which you can restore from the Trash.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 item Scrivener left out of its compile is left out of exports too.
   *[other] { $count } items Scrivener left out of its compile are left out of exports too.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 picture or file in the manuscript becomes a note, since a book cannot hold it.
   *[other] { $count } pictures or files in the manuscript become notes, since a book cannot hold them.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] 1 top-level folder is left out, with { $items ->
        [one] 1 item
       *[other] { $items } items
    }.
   *[other] { $count } top-level folders are left out, with { $items ->
        [one] 1 item
       *[other] { $items } items
    }.
}
# $items is how many items the Trash holds, $writing how many of them hold writing.
import-scrivener-structure-trash-left-out-line = { $items ->
    [1] The Trash is left behind: 1 item.
   *[other] The Trash is left behind: { $items } items, { $writing ->
        [0] none of them with writing
        [one] 1 of them with writing
       *[other] { $writing } of them with writing
    }.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] 1 index card picture on a manuscript item is left behind: it would print in the book.
   *[other] { $count } index card pictures on manuscript items are left behind: they would print in the book.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] 1 folder had a default template for the new documents in it. In Skribisto, only a story bible tag can carry a template, so that link is not kept.
   *[other] { $count } folders had a default template for the new documents in them. In Skribisto, only a story bible tag can carry a template, so those links are not kept.
}
# “Page Break Before” is Scrivener's own label for this setting, quoted as Scrivener shows
# it: use the label of Scrivener's interface in your language if Scrivener is translated
# into it, otherwise keep the English label.
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 “Page Break Before” setting has no place in Skribisto and is not kept.
   *[other] { $count } “Page Break Before” settings have no place in Skribisto and are not kept.
}
# “Compile As-Is” is Scrivener's own label for this setting, quoted as Scrivener shows it:
# use the label of Scrivener's interface in your language if Scrivener is translated into
# it, otherwise keep the English label.
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 “Compile As-Is” setting has no counterpart in Skribisto and is not kept.
   *[other] { $count } “Compile As-Is” settings have no counterpart in Skribisto and are not kept.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] 1 status Skribisto does not recognise is given a kind by its place in the list. Check it in Settings ▸ Work ▸ Statuses.
   *[other] { $count } statuses Skribisto does not recognise are given a kind by their place in the list. Check them in Settings ▸ Work ▸ Statuses.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] The dates of 1 item could not be read: it carries the date of the import.
   *[other] The dates of { $count } items could not be read: they carry the date of the import.
}
# The labels are Scrivener's (see import-scrivener-label-details). $rows is how many items
# wear the new tags.
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] The labels become 1 tag, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
   *[other] The labels become { $tags } tags, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
}
# $rungs is how many statuses Scrivener's status list holds; $rows how many items wear one.
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] The status list comes across as the project’s list of statuses: { $rungs } status, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
   *[other] The status list comes across as the project’s list of statuses: { $rungs } statuses, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [1] The project had no status list, so it starts with Skribisto’s default status.
   *[other] The project had no status list, so it starts with Skribisto’s { $count } default statuses.
}
import-scrivener-metadata-label-renamed-line = The Label field was called “{ $title }” in Scrivener. That name is in each tag’s details.
import-scrivener-metadata-status-renamed-line = The Status field was called “{ $title }” in Scrivener. That name is in each status’s details.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] The label “{ $name }”, kept beside Scrivener’s own empty label under the same name, is read as no label on 1 item.
   *[other] The label “{ $name }”, kept beside Scrivener’s own empty label under the same name, is read as no label on { $count } items.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] The status “{ $name }”, kept beside Scrivener’s own empty status under the same name, is read as no status on 1 item.
   *[other] The status “{ $name }”, kept beside Scrivener’s own empty status under the same name, is read as no status on { $count } items.
}
import-scrivener-metadata-unnamed-label-line = A label with no name is named after its number, { $id }.
import-scrivener-metadata-unnamed-status-line = A status with no name is named after its number, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 tag takes a new name, because another tag already holds its own.
   *[other] { $count } tags take a new name, because other tags already hold theirs.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 status has the same name as another, so it takes a number after its name. Check it in Settings ▸ Work ▸ Statuses.
   *[other] { $count } statuses have the same name as another, so each takes a number after its name. Check them in Settings ▸ Work ▸ Statuses.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] 1 label no document wears is not imported.
   *[other] { $count } labels no document wears are not imported.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] The label of 1 item with no place in the new project does not come across.
   *[other] The labels of { $count } items with no place in the new project do not come across.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] The status of 1 item with no place in the new project does not come across.
   *[other] The statuses of { $count } items with no place in the new project do not come across.
}

import-scrivener-metadata-unnamed-keyword-line = A keyword with no name is named after its number, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] The keywords of 1 item with no place in the new project do not come across.
   *[other] The keywords of { $count } items with no place in the new project do not come across.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] The custom metadata of 1 item with no place in the new project does not come across.
   *[other] The custom metadata of { $count } items with no place in the new project does not come across.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] The bookmarks of 1 item with no place in the new project do not come across.
   *[other] The bookmarks of { $count } items with no place in the new project do not come across.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] The target of 1 item with no place in the new project does not come across.
   *[other] The targets of { $count } items with no place in the new project do not come across.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [1] The label or field value { $values } matches the story bible entry { $matches }. If it names the point of view, set it in the Inspector: the import links nothing.
   *[other] The labels or field values { $values } match the story bible entries { $matches }. If they name points of view, set them in the Inspector: the import links nothing.
}
import-scrivener-metadata-checkbox-default-unknown-line = The checkbox field “{ $field }” has a default Skribisto cannot read, so it cannot tell which documents are ticked by default. Only the documents ticked one by one wear its tag.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 goal is set in the other counting unit. It is kept, and shows once you switch the counting unit in Settings ▸ Work ▸ Structure.
   *[other] { $count } goals are set in the other counting unit. They are kept, and show once you switch the counting unit in Settings ▸ Work ▸ Structure.
}
import-scrivener-metadata-counting-differs-line = Scrivener counted the draft target over other documents than Skribisto does. The book’s goal counts the scenes included in exports.
import-scrivener-metadata-writing-days-not-read-line = The writing days “{ $raw }” cannot be read, so the book’s Pace counts every day as a writing day. Change them there.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 item was in script mode in Scrivener. Skribisto keeps its words as prose, without the screenplay layout.
   *[other] { $count } items were in script mode in Scrivener. Skribisto keeps their words as prose, without the screenplay layout.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 keyword becomes a tag, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
   *[other] { $tags } keywords become tags, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 keyword sat inside another in Scrivener. Tags are flat: its tag’s details say where it sat.
   *[other] { $count } keywords sat inside others in Scrivener. Tags are flat: each tag’s details say where its keyword sat.
}
# $fields list or checkbox fields become $tags tags; $rows is how many items wear those
# tags. Nest the selectors as your grammar needs.
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 list or checkbox field becomes { $tags ->
        [one] { $tags } tag
       *[other] { $tags } tags
    }, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
   *[other] { $fields } list and checkbox fields become { $tags ->
        [one] { $tags } tag
       *[other] { $tags } tags
    }, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 collection becomes a tag, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
   *[other] { $tags } collections become tags, on { $rows ->
        [one] { $rows } item
       *[other] { $rows } items
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] A tag holds no order, so the order of the documents in 1 collection is not kept.
   *[other] A tag holds no order, so the order of the documents in { $count } collections is not kept.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 bookmark becomes a link between two notes, or between a scene and a story bible entry: see Links and Cast.
   *[other] { $count } bookmarks become links between notes, or between scenes and story bible entries: see Links and Cast.
}
# A target is Scrivener's word or character count to reach for a document; a goal is
# Skribisto's. Where your language has one word for both, the sentence still reads as
# Scrivener's becoming Skribisto's.
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] The target Scrivener set on 1 document becomes its goal in Skribisto.
   *[other] The targets Scrivener set on { $count } documents become their goals in Skribisto.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] The draft target of 1 word becomes the book’s goal.
   *[other] The draft target of { $value } words becomes the book’s goal.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] The draft target of 1 character becomes the book’s goal.
   *[other] The draft target of { $value } characters becomes the book’s goal.
}
# “Pace” is the name of a book's Pace page (segment-pace), which plans the words to write
# each day: use your translation of segment-pace.
import-scrivener-metadata-pace-planned-line = The deadline, { $day }, sets up the book’s Pace, which plans the words of each writing day you chose in Scrivener, from the day of the import.
import-scrivener-metadata-pace-planned-day-off-line = The deadline, { $day }, sets up the book’s Pace, which plans the words of each writing day you chose in Scrivener, from the day of the import. As in Scrivener, the deadline itself is a day off.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 auto-complete word goes into the project’s personal dictionary, so it is not marked as misspelled.
   *[other] { $count } auto-complete words go into the project’s personal dictionary, so they are not marked as misspelled.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 detail of the project is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } details of the project are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 project bookmark is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } project bookmarks are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 compile replacement is listed in the note “{ $note }” of the “{ $folder }” folder. Skribisto does not make it.
   *[other] { $count } compile replacements are listed in the note “{ $note }” of the “{ $folder }” folder. Skribisto does not make them.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 auto-complete entry of several words is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } auto-complete entries of several words are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 saved search is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } saved searches are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 custom metadata value a tag cannot hold is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } custom metadata values a tag cannot hold are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 document target is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } document targets are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 bookmark no link of Skribisto shows is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] { $count } bookmarks no link of Skribisto shows are listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 document target sits on a note, whose writing no goal counts, so it does not become a goal.
   *[other] { $count } document targets sit on notes, whose writing no goal counts, so they do not become goals.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 document target sits on a document split at its headings, whose chapters no one goal counts together, so it does not become a goal.
   *[other] { $count } document targets sit on documents split at their headings, whose chapters no one goal counts together, so they do not become goals.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 bookmark leads to a document with no place in the new project, such as a document template. It is listed by where the document was in Scrivener.
   *[other] { $count } bookmarks lead to documents with no place in the new project, such as document templates. They are listed by where the documents were in Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] 1 keyword no document wears is not imported.
   *[other] { $count } keywords no document wears are not imported.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 value of the list field “{ $field }” names none of its options, and is not imported.
   *[other] { $count } values of the list field “{ $field }” name none of its options, and are not imported.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 value belongs to a custom field the project no longer defines, which Scrivener does not show either. It is not imported.
   *[other] { $count } values belong to custom fields the project no longer defines, which Scrivener does not show either. They are not imported.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 document of the collection “{ $name }” has no place in the new project, or is gone from the Scrivener project.
   *[other] { $count } documents of the collection “{ $name }” have no place in the new project, or are gone from the Scrivener project.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] 1 collection that holds no document is not imported.
   *[other] { $count } collections that hold no document are not imported.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 bookmark leads to a document the Scrivener project no longer holds, and is not imported.
   *[other] { $count } bookmarks lead to documents the Scrivener project no longer holds, and are not imported.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 document target counts pages, which Skribisto does not count, so it does not become a goal.
   *[other] { $count } document targets count pages, which Skribisto does not count, so they do not become goals.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] 1 target is not imported: its count is not a number, is below zero, or is more than a goal can hold.
   *[other] { $count } targets are not imported: their counts are not numbers, are below zero, or are more than a goal can hold.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] The draft target of 1 page does not become a goal: Skribisto counts words or characters. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The draft target of { $value } pages does not become a goal: Skribisto counts words or characters. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] The draft target of 1 word does not become a goal, because the manuscript makes several books, or none. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The draft target of { $value } words does not become a goal, because the manuscript makes several books, or none. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] The draft target of 1 character does not become a goal, because the manuscript makes several books, or none. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The draft target of { $value } characters does not become a goal, because the manuscript makes several books, or none. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
# “Pace” is the name of a book's Pace page (segment-pace), which plans the words to write
# each day: use your translation of segment-pace.
import-scrivener-metadata-deadline-passed-line = The deadline, { $day }, has passed, so it does not set up the book’s Pace. It is listed in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-deadline-no-writing-day-line = No writing day is left from the day of the import to the deadline, { $day }, so it does not set up the book’s Pace. It is listed in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-deadline-template-line = The deadline, { $day }, was saved in the template, so it does not set up the book’s Pace. It is listed in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-deadline-no-word-goal-line = The deadline, { $day }, does not set up the book’s Pace, because the draft has no word target to plan towards. It is listed in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-deadline-character-goal-line = The deadline, { $day }, does not set up the book’s Pace, because the draft target counts characters and the Pace counts words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-deadline-no-book-line = The deadline, { $day }, does not set up any book’s Pace, because the manuscript makes several books, or none. It is listed in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-deadline-unreadable-line = The deadline cannot be read, so it does not set up the book’s Pace. It is listed as written in the note “{ $note }” of the “{ $folder }” folder.
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] The session target of 1 word is not imported: in Skribisto, the writing session’s goal is one setting for every project. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The session target of { $value } words is not imported: in Skribisto, the writing session’s goal is one setting for every project. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] The session target of 1 character is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The session target of { $value } characters is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] The session target of 1 page is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The session target of { $value } pages is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] The session target of 1 word, which Scrivener calculated from the deadline and the writing days, is not imported: in Skribisto, the writing session’s goal is one setting for every project. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The session target of { $value } words, which Scrivener calculated from the deadline and the writing days, is not imported: in Skribisto, the writing session’s goal is one setting for every project. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] The session target of 1 character, which Scrivener calculated from the deadline and the writing days, is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The session target of { $value } characters, which Scrivener calculated from the deadline and the writing days, is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] The session target of 1 page, which Scrivener calculated from the deadline and the writing days, is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
   *[other] The session target of { $value } pages, which Scrivener calculated from the deadline and the writing days, is not imported: in Skribisto, the writing session’s goal is one setting for every project, in words. It is listed in the note “{ $note }” of the “{ $folder }” folder.
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 item had a binder icon, which does not come across: Skribisto shows each item’s icon from its type.
   *[other] { $count } items had binder icons, which do not come across: Skribisto shows each item’s icon from its type.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] 1 text could not be read. It arrives empty.
   *[other] { $count } texts could not be read. They arrive empty.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] The comments and linked footnotes of 1 text could not be read. The words they marked are kept.
   *[other] The comments and linked footnotes of { $count } texts could not be read. The words they marked are kept.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 text was stored as plain text rather than formatted text, and is read as such.
   *[other] { $count } texts were stored as plain text rather than formatted text, and are read as such.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 inline note marker does not pair with another and is kept as text.
   *[other] { $count } inline note markers do not pair with another and are kept as text.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 style marker names a style the project does not define. It is removed, and the text keeps its formatting.
   *[other] { $count } style markers name a style the project does not define. They are removed, and the text keeps its formatting.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 paragraph may be a heading that lost its style when Scrivener 3 exported the project to its older format, and comes across as an ordinary paragraph. Check it, or import the project it was exported from.
   *[other] { $count } paragraphs may be headings that lost their style when Scrivener 3 exported the project to its older format, and come across as ordinary paragraphs. Check them, or import the project they were exported from.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] 1 comment or footnote had lost its content in Scrivener. The words it marked are kept, without it.
   *[other] { $count } comments or footnotes had lost their content in Scrivener. The words they marked are kept, without them.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 comment had no place left in its text and is attached to the first paragraph.
   *[other] { $count } comments had no place left in their text and are attached to the first paragraph.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 linked footnote had no place left in its text, so it is cited at the end of that text.
   *[other] { $count } linked footnotes had no place left in their texts, so each is cited at the end of its own text.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 comment from an older Scrivener was placed by another reading of its position. Check that it sits on the right words.
   *[other] { $count } comments from an older Scrivener were placed by another reading of their position. Check that they sit on the right words.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 comment from an older Scrivener was placed by counting the markers of its inline notes. Check that it sits on the right words.
   *[other] { $count } comments from an older Scrivener were placed by counting the markers of their inline notes. Check that they sit on the right words.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 comment from an older Scrivener was placed by leaving its pictures out. Check that it sits on the right words.
   *[other] { $count } comments from an older Scrivener were placed by leaving their pictures out. Check that they sit on the right words.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 comment from an older Scrivener fits no reading of its position exactly, and may sit a few characters off.
   *[other] { $count } comments from an older Scrivener fit no reading of their position exactly, and may sit a few characters off.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 link points at a document the import did not bring across. Its words stay.
   *[other] { $count } links point at documents the import did not bring across. Their words stay.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [1] 1 picture linked to a file on your disk, “{ $example }”, was not brought across.
   *[other] { $count } pictures linked to files on your disk, such as “{ $example }”, were not brought across.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 text opens with a heading that repeats its title, so an export may print the title twice.
   *[other] { $count } texts open with a heading that repeats their title, so an export may print the title twice.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 paragraph written in Markdown is kept as the text it is.
   *[other] { $count } paragraphs written in Markdown are kept as the text they are.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 paragraph holds citation keys for a bibliography program, kept as text.
   *[other] { $count } paragraphs hold citation keys for a bibliography program, kept as text.
}
# A placeholder here is one of Scrivener's codes, such as <$n> or <$projecttitle>, which
# Scrivener replaces with text when it compiles. Not the iPhone or iPad stand-in of
# import-scrivener-refusal-ios-stub-without-mobile-data-title.
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [1] 1 Scrivener placeholder, { $example }, is kept as plain text in what you export. Replace it by hand.
   *[other] { $count } Scrivener placeholders, such as { $example }, are kept as plain text in what you export. Replace them by hand.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [1] 1 Scrivener placeholder, { $example }, is kept as plain text in a text that is not exported.
   *[other] { $count } Scrivener placeholders, such as { $example }, are kept as plain text in texts that are not exported.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 centered line of ornaments may mark a scene break and is kept as written.
   *[other] { $count } centered lines of ornaments may mark scene breaks and are kept as written.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 text is very long, and may be slow to open.
   *[other] { $count } texts are very long, and may be slow to open.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 picture has no description for readers who cannot see it. You can add one in the editor.
   *[other] { $count } pictures have no description for readers who cannot see them. You can add one in the editor.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] Struck-through text in 1 text is kept. Scrivener’s compile left it out of the book, but an export from Skribisto prints it.
   *[other] Struck-through text in { $count } texts is kept. Scrivener’s compile left it out of the book, but an export from Skribisto prints it.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [1] Text in the style “{ $example }”, which Scrivener’s compile deleted, is kept as prose, and an export from Skribisto prints it.
   *[other] Text in { $count } styles Scrivener’s compile deleted, such as “{ $example }”, is kept as prose, and an export from Skribisto prints it.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [1] Text in the style “{ $example }”, which Scrivener’s compile passed on as code for another program, is kept as prose, and an export from Skribisto prints it.
   *[other] Text in { $count } styles Scrivener’s compile passed on as code for another program, such as “{ $example }”, is kept as prose, and an export from Skribisto prints it.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] 1 piece of a damaged file, after its end, is left out.
   *[other] { $count } pieces of damaged files, after their end, are left out.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] 1 header or footer is left out: it is not part of the text.
   *[other] { $count } headers and footers are left out: they are not part of the text.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] 1 picture used as a list bullet is left out.
   *[other] { $count } pictures used as list bullets are left out.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] 1 piece of a file no text holds is left out.
   *[other] { $count } pieces of files no text holds are left out.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 character could not be decoded and shows as a replacement mark (�) where it stood.
   *[other] { $count } characters could not be decoded and show as replacement marks (�) where they stood.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 inline note Scrivener had split where its formatting changed is joined back.
   *[other] { $count } inline notes Scrivener had split where their formatting changed are joined back.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [1] 1 inline note written one paragraph at a time is joined into a single note.
   *[other] { $count } inline notes written one paragraph at a time are joined into notes of several paragraphs.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] 1 empty inline note is left out.
   *[other] { $count } empty inline notes are left out.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 inline annotation became a comment.
   *[other] { $count } inline annotations became comments.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 document held nothing but an annotation. Its words are kept as prose.
   *[other] { $count } documents held nothing but an annotation. Their words are kept as prose.
}
# “Preserve Formatting” is Scrivener's own name for this format, quoted as Scrivener shows
# it: use the name of Scrivener's interface in your language if Scrivener is translated
# into it, otherwise keep the English name.
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 passage marked “Preserve Formatting” arrives as ordinary text.
   *[other] { $count } passages marked “Preserve Formatting” arrive as ordinary text.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 note inside a comment or a footnote is kept as text, where it stood.
   *[other] { $count } notes inside comments or footnotes are kept as text, where they stood.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [1] The style “{ $example }” has no counterpart in Skribisto. Its text keeps its words.
   *[other] { $count } styles, such as “{ $example }”, have no counterpart in Skribisto. Their text keeps its words.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 link to another document has no place in Skribisto. Its words stay.
   *[other] { $count } links to other documents have no place in Skribisto. Their words stay.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 link to another document is kept as a reference.
   *[other] { $count } links to other documents are kept as references.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 link to another project or to a file is kept as text.
   *[other] { $count } links to other projects or to files are kept as text.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] 1 line break inside a paragraph now starts a paragraph of its own.
   *[other] { $count } line breaks inside paragraphs now start paragraphs of their own.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] 1 indent typed at the start of a paragraph is removed.
   *[other] { $count } indents typed at the start of paragraphs are removed.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 paragraph indented from the margin loses its indent.
   *[other] { $count } paragraphs indented from the margin lose their indent.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] 1 empty paragraph between two others is removed.
   *[other] { $count } empty paragraphs between others are removed.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] 1 blank line between the stanzas of a poem is removed, so the stanzas run on.
   *[other] { $count } blank lines between the stanzas of a poem are removed, so the stanzas run on.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 blank line between paragraphs became a scene break.
   *[other] { $count } blank lines between paragraphs became scene breaks.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 line of ornaments became a scene break.
   *[other] { $count } lines of ornaments became scene breaks.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 table now opens with a header row.
   *[other] { $count } tables now open with a header row.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 table cell held several paragraphs, now joined into one.
   *[other] { $count } table cells held several paragraphs, now joined into one.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] Text color is removed in 1 text.
   *[other] Text color is removed in { $count } texts.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] Highlighting is removed in 1 text.
   *[other] Highlighting is removed in { $count } texts.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Small capitals show as ordinary letters in 1 text.
   *[other] Small capitals show as ordinary letters in { $count } texts.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] Text shown in capitals shows as it was typed in 1 text.
   *[other] Text shown in capitals shows as it was typed in { $count } texts.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] Hidden text is left out in 1 text.
   *[other] Hidden text is left out in { $count } texts.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] Formatting Skribisto has no place for is removed in 1 text.
   *[other] Formatting Skribisto has no place for is removed in { $count } texts.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 justified paragraph follows the export’s style instead.
   *[other] { $count } justified paragraphs follow the export’s style instead.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 comment loses its color.
   *[other] { $count } comments lose their color.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] 1 invisible control character is removed.
   *[other] { $count } invisible control characters are removed.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 footnote or comment on a chapter heading could not stay there: a comment moves to the chapter’s first paragraph, a footnote is left out.
   *[other] { $count } footnotes or comments on chapter headings could not stay there: a comment moves to the chapter’s first paragraph, a footnote is left out.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 research file Skribisto cannot open was copied beside the new project. Its note says where.
   *[other] { $count } research files Skribisto cannot open were copied beside the new project. Their notes say where.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 research file Skribisto cannot open stays in your Scrivener project. Its note names it.
   *[other] { $count } research files Skribisto cannot open stay in your Scrivener project. Their notes name them.
}

## When the import fails
## Shown in the notice of a failed import, above the button import-scrivener-review-again,
## which these sentences quote.
import-scrivener-error-refused = { $refusal } Nothing was written.
import-scrivener-error-question = The project now asks a question its review did not answer. Choose “Review the import again”, then import. Nothing was written.
# “Project” is the wizard's first step (import-scrivener-step-project); the box is
# import-scrivener-ios-pending-accept or import-scrivener-no-text-accept.
import-scrivener-error-not-accepted = Before it can be imported, the project needs you to tick a box on the Project step, and it was not ticked. Choose “Review the import again”, tick it, then import. Nothing was written.
import-scrivener-error-changed = The project changed after you reviewed it. Choose “Review the import again”, then import. Nothing was written.
import-scrivener-error-choice = The structure chosen in the review no longer fits this project. Choose “Review the import again”, then import. Nothing was written.

## Export dialog
export-title = Export
export-close = Close
export-scope-label = What
export-format-label = Format
export-style-label = Style
export-path-label = File
export-browse = Browse…
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
# Style-preset summary chips
export-chip-chapters-none = Chapters: none
export-chip-chapters-numbered = Chapters: numbered
export-chip-chapters-title = Chapters: title only
export-chip-chapters-both = Chapters: number + title
export-chip-scene-break-glyph = Scene break: { $glyph }
export-chip-scene-break-blank = Scene break: blank line
export-chip-scene-break-none = Scene break: none
export-chip-major-break-glyph = Major break: { $glyph }
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
search-tip-synopsis = Synopsis: search each writing item’s summary.
search-tip-label = Label: search the short note shown under a binder item’s title.
search-tip-comment = Comments: search the text of comment threads and their replies. A comment is about the manuscript rather than part of it, so it has its own switch, and a replace leaves comment matches unticked until you tick them.
search-tip-book = Books: the book container and its begin / end markers.
search-tip-part = Parts: part-level dividers.
search-tip-chapter = Chapters: chapters, in whichever way the project stores them.
search-tip-scene = Scenes: the items that hold your prose.
search-tip-note = Notes: free-form notes.
search-tip-folder = Folders: plain organizing folders and separators.
search-tip-paratext = A text that belongs to the book but not to its story: a preface, a dedication, an afterword. Never counted in the manuscript.
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
search-preview-footnote-prompt = This match is in a footnote's own text. Open it in the Footnotes dock to see and edit it.
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
inspector-numbered-tip-more = An unnumbered chapter stays in the book exactly as it was: its heading, its prose and its word count are untouched. It simply prints no number, and does not use one up: the chapter after a prologue is chapter one, not chapter two. Leaving it out of the export instead is a different switch, above, and that one removes the chapter from the book altogether.
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
undo-superseded-body = Something else has changed in this project since. Undo would take that back instead, so it hasn’t. Nothing this step did has been reversed. Use Edit ▸ Undo to step back through the history yourself.
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
undo-target-import-tags = importing tags
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
trash-restore-orphaned = This item's original spot is gone. Choose where to restore it.
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
trash-restore-picker-empty = No binders yet. Create one first.
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
# accessible name, not decoration.
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

## Formatting marks: the invisible characters a writer places by hand.
## Mnemonics are unique within the submenu (N, H, S, r, Z, W).
menu-format-typo-marks = &Formatting Mark
menu-format-typo-nbsp = &No-break Space
menu-format-typo-nbhyphen = No-break &Hyphen
menu-format-typo-soft-hyphen = &Soft Hyphen
menu-format-typo-nnbsp = Na&rrow No-break Space
menu-format-typo-zwsp = &Zero-width Space
menu-format-typo-word-joiner = &Word Joiner

## About panel

menu-about = &About Skribisto…
about-title = About Skribisto
about-version = Version { $version }
about-update-available = Version { $version } is available
about-update-see-notes = See what changed
about-update-flathub = Flathub keeps this copy up to date.
about-update-distro = Your distribution keeps this copy up to date.
about-tagline = A novel-writing app for long-form fiction, written in Rust with the Teksilo toolkit.
about-license = Released under the GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Close

## Native (macOS) menu bar
# Labels for the platform-standard App and Window menus mirrored into the macOS
# global menu bar. No `&` mnemonics here: macOS has none, and the native bridge
# resolves these without stripping one, so an ampersand would print literally.
# The application name is data (the running edition's own name), so it arrives
# as an argument: same rule as the window titles below.
native-menu-about = About { $app }
native-menu-hide = Hide { $app }
native-menu-quit = Quit { $app }
native-menu-settings = Settings…
native-menu-window = Window
native-menu-minimize = Minimize
native-menu-zoom = Zoom

# Window titles. The application name is data (the running edition's own name),
# so it arrives as an argument rather than being written into the value here.
window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Window { $n })
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

# ── Work ▸ Punctuation: the project's typographic house style ──────────────
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
settings-punctuation-dialogue-hint = For languages that mark speech with a dash rather than quotation marks: French, Spanish, Russian and others. It only fires at the very start of a paragraph.

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
# the prose: a different fault, and one the row's own swatches do not show.
settings-themes-low-contrast-band = highlight hides the text
settings-themes-field-name = Name
settings-themes-field-paper = Page
settings-themes-field-ink = Text
settings-themes-field-general = Background
settings-themes-field-widget-text = Control strip text
# The shading drawn around the caret: the sentence or paragraph being written,
# per Editor ▸ "Highlight around the caret". This field is the color it uses in
# distraction-free mode.
settings-themes-field-caret-band = Highlight around the caret

# The distraction-free strip's quick-settings gear, and the way through to the
# full theme library from inside it.
statusbar-focus-settings = Distraction-free settings
statusbar-focus-manage-themes = Manage themes…

## Comments: the two docks, the thread cards, and their actions.
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
# A comment that resolved successfully yet has no live range to point at: an
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
comments-sort-reader = Grouped by reader
comments-unsigned-author = Unsigned
# Shown on a thread that shares its passage with another reader's. Never at one:
# a lone remark needs no number, and "1 reader" beside every card would drown the
# two that say 3.
comments-agreement = { $count ->
    [one] { $count } reader
   *[other] { $count } readers
}
comments-menu-add = Add comment
comments-menu-add-paragraph = Comment on this paragraph
overview-col-comments = Comments
overview-col-total-comments = Total comments
comments-card-placeholder = Write a comment…
comments-card-unknown-author = Unknown author
comments-unsigned-toast = Your comments are unsigned: no name is set for you on this computer.
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
# own distribution, never a genre norm, and never a target value.
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
# Kept apart from the manuscript total on purpose. See the module doc on
# AnalysisViewModel::run for why a footnote is authored prose but must not be folded
# into how far along the story reads as being.
analysis-footnote-words-count = { $count ->
    [one] { $count } word is in this book's footnotes, kept apart from the manuscript total.
   *[other] { $count } words are in this book's footnotes, kept apart from the manuscript total.
}
# The figure comes from its own operation and can still be catching up even once the
# rest of this report is ready, said plainly rather than shown as a misleading 0.
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
# say it. Without them a filtered-to-nothing binder is indistinguishable from
# an empty project.
binder-filter-count = { $shown } of { $total } shown
binder-filter-clear = Clear
binder-filter-none = Nothing matches “{ $query }”.

## Settings: paratext structures
settings-paratext-intro = The front and back matter a new project can start with. Each structure belongs to a publishing tradition, and its page titles are written in that tradition's own language. Rename them freely once a project is created.
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
    Keeping it as it is stores your original file in the project: it travels
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

# A Book's cover, set on the Book's own page. Each Book has its own: a project
# holding a trilogy holds three covers.
cover-heading = Cover
cover-pick = Choose…
cover-change = Change…
cover-remove = Remove
cover-none = No cover yet
cover-missing = The picture file is missing from the project
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
footnotes-empty = No footnotes yet. Put the cursor in a scene, then use + above, or Ctrl+Alt+F.
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
import-document-reconcile-hint = Some of these items are ones you already have. Say what should happen to each.
import-document-reconcile-all-new = Nothing in this file matches your project: every item will be added as new.
import-document-duplicate-returns = These files are more than one copy of the same manuscript coming back. Import them one at a time: matching only lines up one returning copy against your project, so importing several together would add a second copy of the book instead of merging it.
import-document-hunk-added = Take this new paragraph
import-document-hunk-removed = Drop this paragraph
import-document-hunk-changed = Take this rewrite
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
import-document-col-footnotes = Footnotes
import-document-col-epigraph = Epigraph
import-document-col-source = Source
import-document-level-rules = Heading levels
import-document-level-n = Heading { $level }
import-document-add-top-level = Add top level
import-document-add-top-level-tooltip = Insert a Book above every analyzed item, for chapter files that have no book heading
import-document-destination = Destination
import-document-destination-hint = Choose a binder or item: new items land inside a folder, or after a scene.
import-document-destination-empty = No binders yet. Create one first.
import-document-plan-empty = Nothing to import yet.
# How many pictures the items being imported bring into the project, under the
# review tree. Shown only when there is at least one.
import-document-pictures = { $count ->
    [1] One picture comes with these documents.
   *[other] { $count } pictures come with these documents.
}
import-document-summary = { $rows ->
    [one] 1 item
   *[other] { $rows } items
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
# A file refused unread because its XML nests deeper than the app will read.
# $limit is the ceiling, a number.
import-diagnostic-nested-too-deep = “{ $path }” was not read: its content is nested more than { $limit } levels deep. No real document comes near that, and reading it could crash Skribisto, so it is left out. The other files still import.
import-diagnostic-archive-too-large = “{ $path }” was not read: it is a compressed file built to unpack far larger than it is, which could exhaust memory and crash Skribisto, so it is left out. The other files still import.
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
    [one] One footnote in “{ $path }” arrives as plain text: footnotes are not read from Markdown.
   *[other] { $count } footnotes in “{ $path }” arrive as plain text: footnotes are not read from Markdown.
}
# The sibling of the line above, and deliberately a different sentence. That one
# says the reader cannot carry footnotes at all (Markdown); this one says it
# carried the rest and names what it could not: a note with no text behind it, or
# one whose reference sits where no prose block covers it, a chapter title being
# the ordinary case.
import-diagnostic-footnote-not-carried = { $count ->
    [one] One footnote in “{ $path }” could not be brought over. The others were.
   *[other] { $count } footnotes in “{ $path }” could not be brought over. The others were.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] One block of raw HTML in “{ $path }” was dropped.
   *[other] { $count } blocks of raw HTML in “{ $path }” were dropped.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] One scene break inside a quote or list in “{ $path }” was dropped. Only a break on its own line is kept.
   *[other] { $count } scene breaks inside quotes or lists in “{ $path }” were dropped. Only a break on its own line is kept.
}
# A picture the document shows that does not come with it, one sentence per reason.
# $detail is the picture as the document names it: a file name, a path inside the
# document, or a web address. Nothing is left in the text where it was.
import-diagnostic-image-not-ingested-format = In “{ $path }”, the picture “{ $detail }” is in a format Skribisto cannot show, so it was left out. Pictures in PNG, JPEG, WebP, GIF, TIFF or BMP come in; save it as one of those and insert it by hand.
import-diagnostic-image-not-ingested-remote = In “{ $path }”, the picture “{ $detail }” is on the web. Pictures are never downloaded during an import, so it was left out.
import-diagnostic-image-not-ingested-outside = In “{ $path }”, the picture “{ $detail }” is outside the document and its folder, so it was left out. Only pictures stored in the document, or in the document’s own folder, come in.
import-diagnostic-image-not-ingested-missing = In “{ $path }”, the picture “{ $detail }” could not be found, so it was left out.
import-diagnostic-image-not-ingested-too-large = In “{ $path }”, the picture “{ $detail }” is larger than a project accepts, so it was left out.
import-diagnostic-image-not-ingested-unreadable = In “{ $path }”, the picture “{ $detail }” could not be read as a picture, so it was left out.
import-diagnostic-image-not-ingested-heading = In “{ $path }”, the picture “{ $detail }” sits in a heading. Each heading becomes the title of an item in the binder, and a title holds no picture, so the picture was left out.
import-diagnostic-image-not-ingested-note = In “{ $path }”, the picture “{ $detail }” sits in a footnote, an endnote or a comment. Those keep their words and no picture, so it was left out.
import-diagnostic-image-not-ingested-html = In “{ $path }”, the picture “{ $detail }” is written as HTML, and an import leaves HTML out, so it was left out with it. Write it as a Markdown picture to bring it in.
import-diagnostic-image-not-ingested-document-full = In “{ $path }”, the picture “{ $detail }” was left out: the pictures of this document together pass what one import brings in. Import the document in parts, or insert the rest by hand.
import-diagnostic-picture-first-frame-only = In “{ $path }”, the animated picture “{ $detail }” arrives as its first frame. Pictures in a project do not move.
import-diagnostic-duplicate-title = “{ $title }” appears { $count } times. If you have imported these files before, this will duplicate them.
import-diagnostic-heading-level-jump = “{ $title }” jumps from heading level { $from } to { $to }; it is placed one level under its parent.
import-diagnostic-illegal-combination = “{ $title }” carries prose, but a { $kind } cannot hold any. Import is held until you change its type or leave it out. Nothing is imported at all otherwise.
# Two sentences, because the file may or may not say who. `$names` is a
# ready-formatted list the app never splits; empty when the source records no
# author, which is what an anonymised .docx looks like.
import-diagnostic-tracked-changes-flattened = { $path } was mid-revision: { $count ->
    [one] { $count } tracked change was accepted
   *[other] { $count } tracked changes were accepted
}, and deletions dropped. That is the final text. To keep any of your own wording instead, use Compare in the last step.
import-diagnostic-tracked-changes-flattened-by = { $path } was mid-revision: { $count ->
    [one] { $count } tracked change by { $names } was accepted
   *[other] { $count } tracked changes by { $names } were accepted
}, and deletions dropped. That is the final text. To keep any of your own wording instead, use Compare in the last step.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } holds { $count } text box. Its text sits outside the document's flow, so where it belongs in a manuscript cannot be answered. It is not imported.
   *[other] { $path } holds { $count } text boxes. Their text sits outside the document's flow, so where it belongs in a manuscript cannot be answered. They are not imported.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } holds { $count } embedded object: a chart, an equation or similar. There is nothing in a manuscript that could hold it.
   *[other] { $path } holds { $count } embedded objects: a chart, an equation or similar. There is nothing in a manuscript that could hold them.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } holds { $count } field: a page number, a cross-reference, a date. It keeps the text it was last showing and will not update again.
   *[other] { $path } holds { $count } fields: a page number, a cross-reference, a date. Each keeps the text it was last showing and will not update again.
}
import-diagnostic-unknown-style-level = { $path } uses the style “{ $detail }”, which looks like a heading but names no level. Those paragraphs are imported as prose rather than guessed at a depth.
import-diagnostic-comment-unanchored = The comment “{ $detail }” in { $path } could not be attached to the words it was about. It is kept on its item, where you can move it.
# The sibling of the line above, for the one case where there is no item to keep the
# comment on: the file brought no text at all, or only headings.
import-diagnostic-comment-not-carried = The comment “{ $detail }” in { $path } is not imported, because nothing in the file arrives as text it could be kept on. Copy it from the file if you need it.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } reply in { $path } named a comment that is not in the file, so it arrives as a comment of its own.
   *[other] { $count } replies in { $path } named a comment that is not in the file, so they arrive as comments of their own.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } paragraph in { $path } could not be brought over with its formatting exactly as written, so it arrives as plain text. Compare it with the original.
   *[other] { $count } paragraphs in { $path } could not be brought over with their formatting exactly as written, so they arrive as plain text. Compare them with the original.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] In { $path }, { $count } stretch of blank space was underlined or struck through, a line left to fill in for example. Skribisto keeps neither line on spaces alone, so it arrives as plain spaces, or not at all where it starts or ends a paragraph.
   *[other] In { $path }, { $count } stretches of blank space were underlined or struck through, lines left to fill in for example. Skribisto keeps neither line on spaces alone, so they arrive as plain spaces, or not at all where they start or end a paragraph.
}
# List items nested deeper than the importer keeps, placed at the deepest level it keeps.
# $count is how many items; $limit is that deepest level, a number.
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } list item in { $path } was nested more than { $limit } levels deep. It arrives at level { $limit }, the deepest Skribisto keeps, with its words and formatting.
   *[other] { $count } list items in { $path } were nested more than { $limit } levels deep. They arrive at level { $limit }, the deepest Skribisto keeps, with their words and formatting.
}
# Tables too lopsided to complete into a grid, one row far wider than many short ones,
# which arrive as the paragraphs of their cells. $count is how many tables.
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } table in { $path } has a row far wider than the rest, too lopsided to lay out as a grid. Its cells arrive as paragraphs, one after another, with their words and formatting.
   *[other] { $count } tables in { $path } have a row far wider than the rest, too lopsided to lay out as a grid. Their cells arrive as paragraphs, one after another, with their words and formatting.
}
# Runs of spaces longer than the importer keeps, cut to that length. $count is how many
# runs; $limit is the most spaces a run keeps, a number.
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } run of spaces in { $path } was longer than { $limit } spaces, and arrives shortened to { $limit }. The words around it are all there.
   *[other] { $count } runs of spaces in { $path } were longer than { $limit } spaces, and arrive shortened to { $limit }. The words around them are all there.
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
versions-error = Couldn't read your backups. Nothing has been lost, but this list may be incomplete
versions-did-not-exist = Didn't exist yet on { $date }
versions-deleted-after = Deleted some time after { $date }
versions-unreadable = { $count ->
    [one] { $count } backup couldn't be read
   *[other] { $count } backups couldn't be read
}
# Counts what the project's own history dropped, which is why it names it: a
# backup may still hold one of those states, and then it is in the list above.
versions-thinned = { $count ->
    [one] Older versions thin out as they age. The project's own history has already dropped { $count } earlier state of this text.
   *[other] Older versions thin out as they age. The project's own history has already dropped { $count } earlier states of this text.
}
versions-source-backup = From a backup
versions-source-project = From the project's own history
versions-list-caption = One entry per change, not per backup
versions-pick-a-version = Pick a version to see what changed
versions-earliest = The earliest version on record. There's nothing older to compare it with.
versions-no-change = Nothing changed in this part of the item
versions-formatting-only = Only the formatting changed here. The words are the same
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
# keeps the whole snapshot this row's text was read out of. See versions-pin-note.
versions-pin = Pin the backup this version came from. Automatic cleanup will never delete it
versions-unpin = Unpin the backup this version came from. Automatic cleanup may delete it again
versions-pinned-only = Show only pinned versions
versions-pin-note = Only versions from a backup can be pinned. A pin keeps a file, and the project's own history isn't one.
versions-pinned-empty = Nothing here is pinned yet
# The same sentence with the reason, for a list that actually holds a version no
# pin can reach. On an all-backup list the reason is true and irrelevant, and
# reads as an explanation for an emptiness it did not cause.
versions-pinned-empty-log = Nothing here is pinned yet. Only versions from a backup can be pinned. A pin keeps a file, and the project's own history isn't one.
versions-range-filter = Show only versions between two dates
versions-filtered-empty = No version matches the filters you've set
versions-clear-filters = Clear the filters
# Also used by the Timeline band's filter row, like versions-error above it.
versions-last-30-days = Last 30 days
versions-restore-button = Restore this version
versions-restore-confirm-title = Replace this text with the version from { $date }?
versions-restore-confirm-text = What you have now will be replaced by the text this item had on { $date }.
versions-restore-confirm-with-comments = What you have now will be replaced by the text this item had on { $date }. { $count ->
    [one] { $count } comment is anchored in the current text and may be left orphaned.
   *[other] { $count } comments are anchored in the current text and may be left orphaned.
}
versions-restore-confirm-undo-note = A backup is made first, and Ctrl+Z undoes this in one step.
versions-restored-toast = Restored the version from { $date }
versions-undo = Undo
versions-restore-row-gone = That item is no longer in this project
versions-restore-no-home = This item has changed type since then, and the old text has nowhere to go in it
versions-restore-no-safety-copy = Your safety backup didn't run, so nothing was changed
versions-restore-failed = The restore failed: { $error }
versions-restore-backup-busy = A backup is already running. Try again in a moment
versions-restore-in-backup-file = You're looking at a backup file; open the project itself to restore into it
versions-restore-no-project = No project is open
# ── Bringing a deleted row back ──
versions-recreate-button = Bring this back…
versions-recreate-picker-title = Where should it go?
versions-recreate-picker-empty = This project has no binder to put it in
versions-recreate-picker-confirm = Bring it back here
versions-recreate-picker-cancel = Cancel
versions-recreate-untitled = this item
versions-recreate-confirm-title = Bring “{ $item }” back?
versions-recreate-confirm-text = It will be added to { $destination }, with the text it had on { $date }.
# NOT "Ctrl+Z": that undoes the focused editor's *document*, and this creates a
# binder row. The way back is the Undo on the toast, as it is for trash and
# comments. See versions-recreated-toast.
versions-recreate-confirm-undo-note = Undo, on the message that follows, takes it straight back out.
versions-recreated-toast = “{ $item }” is back in your project
versions-recreated-partial-toast = { $count ->
    [one] “{ $item }” is back, but one of its texts couldn't be read
   *[other] “{ $item }” is back, but { $count } of its texts couldn't be read
}
versions-recreate-already-here = That item is in your project already
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
timeline-no-text-of-its-own = This has no text of its own. It's a heading for what's inside it
timeline-reader-close = Close
timeline-reader-stamp = As it was on { $date }
timeline-reader-loading = Opening the recorded version…
timeline-reader-unreadable = This recording couldn't be read. The backup may have been moved, deleted, or be on a drive that isn't connected.
timeline-reader-compared = As it was on { $date }, set against what it says now
timeline-reader-diff-legend = Struck through has gone since; underlined has been added.
timeline-reader-view-label = What to show
timeline-reader-view-diff = Changes
timeline-reader-view-text = Text
timeline-reader-deleted = This is no longer in your project. You can read it and copy it out here.
timeline-prose-only-record = This point comes from the project's own history, which keeps text and nothing else, so what was deleted or moved since it can't be shown. Pick a backup for that.

# The Settings tree's section for pages an extension contributed.
settings-sec-extensions = Extensions

# Writing games pane: self-imposed drafting constraints
settings-group-games-forward = Always forward
settings-games-forward-toggle = Play "Always forward"
settings-games-forward-hint =
    While you play, nothing you have written can be taken back: Backspace, Delete,
    Cut, drag-and-drop and Undo are all disabled in the surfaces you choose below.
    You can still type, paste, format and move around freely. The draft only grows.
settings-games-session-warning =
    This is a per-session choice: it is never saved. Closing the project, or quitting
    Skribisto, always ends the game, and everything you wrote while playing becomes
    undoable again the moment it does.
settings-group-games-scope = Where it applies
settings-games-in-prose = Manuscript prose
settings-games-in-synopsis = Synopses
settings-games-scope-hint =
    Comments, footnotes and titles are never frozen: they are where you note the fix
    you have just forbidden yourself from making.
settings-games-inert-warning =
    "Always forward" is on but applies to nothing. Tick at least one surface above,
    or it changes nothing as you write.

# Writing games dock (leading rail) + the status-bar warning while a game is on
games-title = Writing games
games-forward-name = Always forward
games-forward-blurb =
    Draft without taking anything back. Deleting, cutting and undoing are disabled
    while you play. Write the next sentence instead of fixing the last one.
games-forward-playing = Playing: deleting is disabled
games-forward-idle = Not playing
games-session-note = Ends when you close the project.
games-scope-prose-and-synopsis = Applies to your prose and synopses.
games-scope-prose = Applies to your prose.
games-scope-synopsis = Applies to your synopses.
games-scope-nothing = Applies to nothing yet. Choose a surface in Settings.
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
# Work menu row that opens the same card on demand, greyed out when the project has
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
menu-help-check-updates = Check for &Updates
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
help-topic-import-scrivener = Coming from Scrivener
help-topic-backups-and-versions = Backups and versions

help-shortcuts-title = Keyboard shortcuts
help-shortcuts-filter = Filter shortcuts
help-shortcuts-no-matches = No shortcut matches.
help-shortcuts-rebind = Change shortcuts…
help-shortcuts-close = Close

command-palette-placeholder = Type a command

# ── Shortcut names ───────────────────────────────────────────────────────────
# The user-visible name of every rebindable `Shortcut` (`Shortcut::name`,
# teksilo-core), reactive to a locale switch via `LocalizedString`/
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
shortcut-name-work-import-scrivener = Import Scrivener Project…
shortcut-name-work-import-scrivener-report = Show Scrivener Import Report
shortcut-name-app-settings = Settings
shortcut-name-app-quit = Quit
shortcut-name-editor-insert-footnote = Insert Footnote
shortcut-name-format-scene-break = Insert Scene Break
shortcut-name-format-major-scene-break = Insert Major Scene Break
shortcut-name-format-link = Insert Link
shortcut-name-format-mark-nbsp = Insert No-break Space
shortcut-name-format-mark-nbhyphen = Insert No-break Hyphen
shortcut-name-format-mark-nnbsp = Insert Narrow No-break Space
shortcut-name-format-mark-zwsp = Insert Zero-width Space
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
shortcut-name-help-check-updates = Check for Updates
# Raised only by the Help menu command, which the reader asked for.
update-check-behind = Version { $version } is available.
update-check-current = You are running the newest version.
update-check-failed = Could not check for a newer version. { $error }
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

## Margin lane: the strip beside the scrollbar that maps a document
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

## Statuses: the project's workflow ladder.
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
status-completion-empty = No scenes yet. This reads the manuscript, so it fills in as you write.
status-completion-open = Where the book stands…

## Settings ▸ Work ▸ Statuses: the ladder editor.
## The writer owns a rung's NAME and the ladder's ORDER; the app owns its category, and
## the category owns the glyph and the colour. Only the category names below are
## translated: a rung's name is the writer's own text and stays as they typed it.
settings-page-statuses = Statuses
settings-desc-statuses = Rename, reorder, add and remove the stages you move a scene through
settings-statuses-add = Add status
settings-statuses-add-placeholder = New status name
settings-statuses-desc =
    The stages a scene moves through, in order, from least finished at the top to most
    finished at the bottom. That order is what "less finished than" means everywhere else
    in the app, so put them the way your process actually runs.
settings-statuses-apply-preset = Apply a preset…
settings-statuses-preset-applied = { $added ->
    [one] 1 status added
   *[other] { $added } statuses added
}
settings-statuses-preset-refused = This project already has a ladder. Presets only fill an empty one. Delete the rungs you don't want first.
settings-statuses-duplicate = "{ $name }" is already on this ladder
settings-statuses-added = Added "{ $name }"
settings-statuses-move-up = Move up (less finished)
settings-statuses-move-down = Move down (more finished)
settings-statuses-details-placeholder = What this stage means (optional)
settings-statuses-delete = Delete "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Delete "{ $name }": 1 item is on it and will lose its status
   *[other] Delete "{ $name }": { $count } items are on it and will lose their status
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

## Long-operation progress (shared::progress_text). The backend reports phase
## codes it cannot translate; these are their labels. `progress-percent` is the
## toast body on its own, `progress-percent-with-label` with the phase after it.
progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = Opening the project…
progress-import-outline = Reading the outline…
progress-import-converting = Converting chapters and scenes…
progress-import-writing = Writing the .skrib file…
progress-import-done = Done
progress-import-structure = Working out the structure…
progress-export-start = Starting…
progress-export-writing = Writing…
progress-project-reading = Reading the project…
# Settings: the open project's section title, "Work: <its title>".
settings-sec-work-titled = Work: { $title }

## Project-import notices (shared::import_warnings::notices): one line of the
## Plume / Manuskript warnings list each. $file, $folder, $title, $name, $field
## and $error are data (file names, titles, a parser's own message) and stay as
## they are. $container is "single-file" or "folder"; $kind is "scene", "note",
## "synopsis" or "title".
import-notice-flattened-rich-texts = { $count ->
    [one] One text was nested deeper than a Skribisto project can hold; its words were kept as plain text, without their formatting.
   *[other] { $count } texts were nested deeper than a Skribisto project can hold; their words were kept as plain text, without their formatting.
}
import-notice-free-text-badges = { $count ->
    [one] One scene badge looks like free text rather than a list of statuses, so it was kept as the item’s label instead of becoming a tag.
   *[other] { $count } different scene badges look like free text rather than a list of statuses, so they were kept as item labels instead of becoming tags.
}
import-notice-separator-text-dropped = The separator “{ $name }” held text, which has no place in a Skribisto project; the text was not imported.
import-notice-separator-without-scene = The separator “{ $name }” had no scene before it in its chapter to attach to, so it was not imported.
import-notice-chapter-without-scenes = The chapter “{ $name }” has no scenes, so its separators were not imported.
import-notice-rich-text-unconverted = Some formatted text could not be converted ({ $error }); it was not imported.
import-notice-unresolved-cross-link = A link to cast entry #{ $number } could not be followed: the entry is missing or was in the trash.
import-notice-read-copy = Read the { $container ->
    [single-file] single-file
   *[folder] folder
} copy of this project.
import-notice-read-copy-dated = Read the { $container ->
    [single-file] single-file
   *[folder] folder
} copy of this project, last changed { $day }.
import-notice-original-format = This project is in Manuskript’s original 2016 format. It was read in full; nothing in it was changed.
import-notice-newer-single-file-beside = “{ $file }” is newer than this folder ({ $file_day } against { $folder_day }). Manuskript leaves the old folder in place when a project moves to single-file mode, so this folder may be an abandoned copy; importing the .msk file instead would read the newer one.
import-notice-pickle-skipped = “{ $file }” was left unread. It is a Python pickle, and loading one from a project file was a security flaw in Manuskript itself (CVE-2021-35196); the settings it holds are display preferences, not writing.
import-notice-file-unreadable = “{ $file }” could not be read ({ $error }) and was skipped; everything else was imported.
import-notice-not-utf8 = “{ $file }” is not valid UTF-8; the characters that could not be read were replaced rather than stopping the import.
import-notice-table-unreadable = “{ $file }” could not be read ({ $error }); what it held was not imported.
import-notice-outline-missing = “{ $file }” is missing, so this project has no manuscript to import.
import-notice-outline-unreadable = “{ $file }” could not be read ({ $error }); no manuscript was imported.
import-notice-named-none-row = “{ $file }” starts with a named row (“{ $name }”) where Manuskript keeps its empty “none” entry. It was left out, so the rows after it keep the numbers the manuscript uses.
import-notice-unknown-field = “{ $file }” has a field this importer does not know, “{ $field }”. It was not imported.
import-notice-colorless-labels = { $count ->
    [one] “{ $file }” has a line with no color. It was imported without one; Manuskript stops loading the project on such a line.
   *[other] “{ $file }” has { $count } lines with no color. They were imported without one; Manuskript stops loading the project on such a line.
}
import-notice-settings-unreadable = “{ $file }” could not be read ({ $error }); the project’s language was not imported.
import-notice-revisions-unreadable = “{ $file }” could not be read ({ $error }); the project’s history was not imported, but nothing else was affected.
import-notice-foreign-files = { $count ->
    [one] This project holds a file that is not part of the Manuskript format and was not imported: { $files }.
   *[other] This project holds { $count } files that are not part of the Manuskript format and were not imported: { $files }.
}
import-notice-foreign-files-more = This project holds { $count } files that are not part of the Manuskript format and were not imported: { $files }, and { $more ->
    [one] one more
   *[other] { $more } more
}.
import-notice-not-a-character-file = “{ $file }” is not a Manuskript character file and was left out.
import-notice-not-a-text-item = “{ $file }” is not a Manuskript text item and was left out of the outline.
import-notice-missing-blank-line = “{ $file }” has no blank line after its header. Its last field was kept, although Manuskript loses it.
import-notice-folder-without-metadata = “{ $folder }” has no { $file }, so it was imported as a plain folder named after the directory. Manuskript skips such a directory and everything in it.
import-notice-rows-given-ids = { $count ->
    [one] One outline row had no ID of its own and was given one. Manuskript drops such a row, or stops loading the project when it is a folder.
   *[other] { $count } outline rows had no ID of their own and were given one. Manuskript drops such a row, or stops loading the project when it is a folder.
}
import-notice-plots-unreadable = “{ $file }” could not be read ({ $error }); the plots were not imported.
import-notice-world-unreadable = “{ $file }” could not be read ({ $error }); the world entries were not imported.
import-notice-world-not-opml = “{ $file }” is not an OPML document; the world entries were not imported.
import-notice-text-flattened = The text of “{ $title }” is nested deeper than a Skribisto project can hold; its words were kept as plain text, without their formatting.
import-notice-text-unconverted = The text of “{ $title }” could not be converted ({ $error }); it was kept exactly as it was written.
import-notice-text-without-file-name = The { $kind ->
    [scene] scene text
    [note] note text
    [synopsis] synopsis
   *[title] title
} of “{ $title }” could not be given a file name, so it was not imported.
import-notice-unknown-label = “{ $title }” has label { $index }, which is not in this project’s label list. It was imported without a label.
import-notice-unknown-status = “{ $title }” has status { $index }, which is not in this project’s status list. It was imported without a status.
import-notice-unknown-point-of-view = “{ $title }” is told through a character who is not in this project’s cast. Its point of view was not set.
import-notice-unresolved-references = The references in the text of “{ $title }” could not be resolved ({ $error }); the text was kept as it was converted, reference markers included.
import-notice-custom-icons = { $count ->
    [one] One row had a custom icon. Skribisto gives each row the icon of its type, so the icon was not imported; nothing else about the row changed.
   *[other] { $count } rows had a custom icon. Skribisto gives each row the icon of its type, so the icons were not imported; nothing else about those rows changed.
}
import-notice-character-colors = { $count ->
    [one] One character had a color beside their name. Skribisto colors tags rather than rows, so the color was not imported; the character itself was.
   *[other] { $count } characters had a color beside their name. Skribisto colors tags rather than rows, so the colors were not imported; the characters themselves all were.
}
import-notice-versions-kept = { $found ->
    [one] This project recorded one earlier version
   *[other] This project recorded { $found } earlier versions
}; { $kept ->
    [one] one was kept
   *[other] { $kept } were kept
}, thinned out the way Skribisto thins its own history. They are in the Versions panel.
import-notice-versions-unplaced = { $count ->
    [one] One earlier version belongs to a row that is no longer in the project and could not be placed.
   *[other] { $count } earlier versions belong to a row that is no longer in the project and could not be placed.
}

## Native file dialogs: the window title, and the label of the file-type filter.
dialog-open-work-title = Open Skribisto work
dialog-save-as-file-title = Save as a single .skrib file
dialog-save-as-folder-title = Choose a parent folder for the work
dialog-manuskript-file-title = Choose a Manuskript project
dialog-manuskript-folder-title = Choose a Manuskript project folder
dialog-plume-file-title = Choose a Plume Creator project
dialog-filter-skrib = Skribisto work
dialog-filter-manuskript = Manuskript project
dialog-filter-plume = Plume Creator project
dialog-filter-hunspell-aff = Hunspell affix file (.aff)
dialog-filter-hunspell-dic = Hunspell dictionary (.dic)
dialog-filter-export = Exported file
# Help ▸ Keyboard shortcuts / Settings ▸ Keymap / command palette.
shortcut-name-view-margin-lane = Toggle Margin Marks

## The built-in distraction-free themes' names (distraction_free::theme).
df-theme-paper = Paper
df-theme-sepia = Sepia
df-theme-night = Night
df-theme-high-contrast = High contrast
df-theme-midnight = Midnight

## Welcome ▸ Examples: the line under a bundled example's title.
welcome-example-byline = by { $author }

## Spoken by a screen reader only.
# Welcome window: the name of the section tab bar.
welcome-sections-a11y = Welcome sections
# A comment's anchored span in the editor: author, plain-text preview, replies.
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 reply
   *[other] { $count } replies
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 reply
   *[other] { $count } replies
})
# Margin marks beside the text: one mark, several merged into one, and the
# lane's own name when nothing more specific was given.
margin-lane-a11y-mark = { $label }, { $percent } percent through
margin-lane-a11y-marks = { $label }, { $count } marks, { $percent } percent through
margin-lane-a11y-fallback = { $count ->
    [one] Margin marks, 1 mark
   *[other] Margin marks, { $count } marks
}
# Export ▸ Choose: the row standing for a binder that has no name.
export-choose-unnamed-binder = Unnamed binder
# The built-in export styles named in words (the national manuscript presets
# keep their own language's name).
export-style-neutral = Neutral
export-style-manuscript-shunn = Standard Manuscript (Shunn)
export-style-ebook-clean = Clean e-book

## Dates. Put the parts in the order your language writes a short date; the
## parts arrive already zero-padded where that matters.
# Comment cards, the outline card's Created/Modified.
date-time-short = { $month }/{ $day }/{ $year } { $hour }:{ $minute }
# Pace chart: a day on the axis, a milestone's date, a holiday.
pace-day-label = { $month }/{ $day }

## File sizes (shared::byte_size). $whole and $tenth are the two sides of the
## decimal mark: write your language's mark between them and its unit symbol.
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole }.{ $tenth } MB
byte-size-gb = { $whole }.{ $tenth } GB
# A whole percentage (sliders, the Pace card, the text-size toast).
unit-percent = { $value }%
# A whole number of pixels (slider readouts).
unit-pixels = { $value } px
# Settings ▸ Paratext: a preset saved without a `name` line. `name` is the TOML
# key the writer types and stays as it is.
settings-paratext-error-no-name = A preset needs a `name`.
# Export: started with no project open (a menu raced a close).
export-error-no-project = No project is open to export.

## Shortcut groups (Settings ▸ Keymap, Help ▸ Keyboard shortcuts, the command
## palette): each menu's title, without its mnemonic.
shortcut-group-work = Work
shortcut-group-edit = Edit
shortcut-group-view = View
shortcut-group-document = Document
shortcut-group-format = Format
shortcut-group-go = Go
shortcut-group-tools = Tools
shortcut-group-help = Help

## Manuskript import: the headings of the notes it writes (a character's
## sheet, a world entry, a plot and its steps, the project details, the
## snowflake summary ladder) and the title of a project with no name. Written
## into the imported project once, in the interface language; the writer can
## rename them like any other text. Goal and Conflict reuse the note templates'
## own words.
import-manuskript-section-paragraph-summary = Paragraph summary
import-manuskript-section-full-summary = Full summary
import-manuskript-section-motivation = Motivation
import-manuskript-section-epiphany = Epiphany
import-manuskript-section-notes = Notes
import-manuskript-section-description = Description
import-manuskript-section-passion = Passion
import-manuskript-section-result = Result
import-manuskript-section-summary = Summary
import-manuskript-section-meta = Meta
import-manuskript-section-series = Series
import-manuskript-section-volume = Volume
import-manuskript-section-genre = Genre
import-manuskript-section-license = License
import-manuskript-section-email = Email
# The snowflake ladder's five tellings, shortest first: one situation, one
# sentence, one paragraph, one page, the full summary.
import-manuskript-section-situation = Situation
import-manuskript-section-sentence = One sentence
import-manuskript-section-paragraph = One paragraph
import-manuskript-section-page = One page
import-manuskript-section-full = Full summary
import-manuskript-untitled-project = Imported Manuskript project

## Plume import: the story-bible groups an old (0.3 and earlier) project
## implies without naming them, and the title of a project with no name.
## Written into the imported project once, in the interface language.
import-plume-group-characters = Characters
import-plume-group-items = Items
import-plume-group-places = Places
import-plume-untitled-project = Imported Plume project
