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
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documents (Markdown, text)…
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
menu-timeline = &Timeline
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
promote-lossy-text = A { $target } has nowhere to keep: { $kinds }. Move or clear that text first, then convert.
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
promote-blocked-text = This chapter still holds { $count } item(s). Move or trash them before converting it to a flat chapter.

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
settings-reset-confirm-body = This restores every setting on all pages to its factory value. It can't be undone.
settings-empty-title = No settings here yet
settings-empty-hint = This section will gain options in a future update.

## Settings window: categories
settings-sec-appearance-behaviour = Appearance & Behaviour
settings-sec-editor = Editor
settings-sec-spelling = Spelling
settings-sec-backup = Backup & Sync
settings-sec-compile = Compile & Export
settings-page-appearance = Appearance
settings-page-menus = Menus & Toolbars
settings-page-notifications = Notifications
settings-page-scene = Scene
settings-page-synopsis = Synopsis
settings-page-notes = Notes
settings-page-editor-behavior = Editor Behavior
settings-page-goals = Goals & Word Count
settings-page-corkboard = Corkboard
settings-page-distraction-free = Distraction-free
settings-page-dictionaries = Dictionaries
settings-page-autosave = Autosave
settings-page-export = Export Formats
settings-page-paratext = Paratext Structures
settings-page-keymap = Keymap
# Filter box on the Keymap page (filters the ShortcutSettings list by name / id / category).
settings-keymap-filter = Filter shortcuts

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
settings-field-author-hint = Appears on the compiled title page and in exported file metadata. Leave it blank to omit it.
settings-group-chapters = Chapters
tidy-titles-title = Tidy chapter titles
tidy-titles-none = No chapter or part title is merely repeating its own number.
tidy-titles-lead = { $count ->
        [one] One chapter or part is titled with nothing but its own number.
       *[other] { $count } chapters and parts are titled with nothing but their own number.
    }
tidy-titles-explain = Clearing those titles leaves each one named by the number the book already knows — the same number the export prints. Nothing else changes, and one undo puts them all back.
menu-document-tidy-titles = Tidy chapter titles…
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
welcome-new-from-documents = From documents…
welcome-recent-works = Recent Works
welcome-empty-recents = No recent works yet.
# Shown in place of the recents list when the search matched none of them,
# distinct from having no recent works at all.
welcome-no-matches = No recent work matches your search.
welcome-learn-soon = Guides and tips are coming soon.
welcome-about-blurb = Skribisto, a Rust + Bastyde rewrite of the writing app.
# The *…* is inline markup, not decoration: it italicises the line (the widget
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
pace-daily-target-line = Even pace: { $count } words/day
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
binder-item-count = { $count } items
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
backup-partial = Backup finished: { $ok } saved, { $failed } destination(s) failed
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
settings-page-backup = Backups
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
new-work-template-notebook = Notebook
new-work-cancel = Cancel
new-work-create = Create Work
# Format tile descriptions
new-work-single-file-desc = One .skrib archive (zip). Portable, easy to back up.
new-work-bundle-desc = A folder holding every text & asset. Friendlier to version control.
# Template row trailing counts
new-work-template-none-count = empty binder
new-work-template-empty-novel-count = binders, no chapters
new-work-template-light-novel-count = 15 chapters
new-work-template-novel-count = 20 chapters
new-work-template-notebook-count = free-form notes
# ChapterScene toggle (novel templates)
new-work-chapter-scene = Flat chapters
new-work-chapter-scene-tip = Each chapter is a single row you write straight into, with no scenes under it. Leave this off for the classic layout, where a chapter is a folder: you write straight into that too, but it can hold scenes as well.
new-work-chapter-scene-tip-more = You write into a chapter either way. The only difference is whether it can *contain* scenes. Skribisto's binder tree is organisational only, so both layouts compile to the same book, you can mix them freely, and Promote converts a chapter between the two without losing a word.
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
new-work-chapter = Chapter
new-work-scene = Scene
new-work-note = Note
new-work-front-matter = Front matter
new-work-back-matter = Back matter
new-work-paratext = Book structure
new-work-paratext-none = No structure
new-work-paratext-hint = The front and back matter a tradition opens and closes a book with. You can move, rename or delete any of it afterwards.

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
import-plume-cancelled = Import cancelled
# Result
import-plume-done = Imported { $imported } items. { $skipped } trashed items were not migrated.
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
export-section-style = Style preset
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
export-cancelled = Export cancelled
export-done = Exported { $count } item(s)
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
search-preserve-case = Preserve case
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
search-tip-folder = Folders: plain organising folders and separators.
search-tip-paratext = A text that belongs to the book but not to its story — a preface, a dedication, an afterword. Never counted in the manuscript.
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
search-occurrences = ×{ $count }
search-field-body = Body
search-field-title = Title
search-field-synopsis = Synopsis
search-field-label = Label
search-field-epigraph = Epigraph
search-field-comment = Comment
search-field-comment-reply = Reply
search-field-footnote = Footnote
search-include-in-replace = Include in Replace All
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
dict-accept-first = Accept the licence before downloading { $name }
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
dict-view-license = View licence
dict-approx-size = ~{ $size }
dict-personal-empty = No personal words in this project yet.
dict-personal-add = Add
dict-personal-placeholder = Add a word…
# Missing-dictionary prompt after opening a project
dict-missing-toast = This project uses { $count } dictionaries you don't have installed
dict-missing-action = Get dictionaries
# Licence modal
dict-license-title = { $name } licence
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
ctx-number = Number this chapter
ctx-unnumber = Do not number this chapter
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
settings-user-dict-imported = Imported { $count } words ({ $duplicates } already present).
settings-user-dict-import-failed = Could not read the word list: { $error }
settings-user-dict-exported = Saved { $count } words.
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
editor-dict-added-multi = Added { $count } words to your dictionary.
toast-undo = Undo

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
trash-empty-confirm-text = { $count } trashed entries will be permanently deleted. This can't be undone (a short grace period lets you undo right after).
trash-emptied-title = Trash emptied
trash-emptied-body = Everything in the trash was permanently deleted.
trash-empty-no-project = No project is open, so there is no trash to empty.
trash-delete-forever-confirm-title = Delete forever?
trash-delete-forever-confirm-text = { $count } item(s) will be permanently deleted. This can't be undone (a short grace period lets you undo right after).
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
format-group-history = History
format-group-marks = Text
format-group-block = Paragraph
format-group-lists = Lists
format-group-tables = Table
format-group-breaks = Scene breaks
# Button tooltips. Icon-only buttons, so each tooltip is that button's only
# accessible name — not decoration.
format-undo = Undo
format-redo = Redo
format-superscript = Superscript
format-subscript = Subscript
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
format-align-center = Centre
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
menu-format-align-center = &Centre
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
menu-format-undo = U&ndo
menu-format-redo = &Redo

## About panel

menu-about = &About Skribisto…
about-title = About Skribisto
about-version = Version { $version }
about-tagline = A novel-writing app for long-form fiction, written in Rust with the Bastyde toolkit.
about-license = Released under the GNU General Public License, version 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Close

# ── Work ▸ Punctuation — the project's typographic house style ──────────────
settings-page-punctuation = Punctuation
settings-group-punctuation = Smart punctuation
settings-punctuation-override = Give this project its own punctuation rules
settings-punctuation-override-hint = Off: the project follows the application preference. These rules travel inside the .skrib, so a co-author opening the file writes with the same typography.
settings-punctuation-dashes = Turn -- into an en dash, --- into an em dash
settings-punctuation-ellipsis = Turn ... into an ellipsis
settings-punctuation-quotes = Curl quotation marks and apostrophes
settings-quote-style = Quotation marks
settings-quote-style-locale = Language default
settings-quote-style-curly = “Curly”
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
# per Editor ▸ "Highlight around the caret". This field is the colour it uses in
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
comments-reply-count = { $count } replies
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
comments-card-reply = Reply
comments-card-reply-placeholder = Reply…
comments-card-actions = Comment actions
comments-reply-actions = Reply actions
comments-menu-delete-reply = Delete reply
comments-menu-delete-all = Delete all comments here
comments-deleted-toast = Comment deleted
comments-reply-deleted-toast = Reply deleted
comments-deleted-all-toast = { $count } comments deleted
comments-undo = Undo

# ── Analysis (Book container segment) ────────────────────────────────────────
# House rule for every string here: describe, never judge. No "too many", no
# "weak", no "should". Every comparison is against the book's own median or its
# own distribution — never a genre norm, and never a target value.
analysis-segment = Analysis
analysis-scope-book = Analysing this book
analysis-run = Run analysis
analysis-stale = Changed since this ran
analysis-not-run = Not analysed yet.
analysis-running = Reading the manuscript…
analysis-failed = The analysis could not finish.
analysis-no-scenes = No scenes in this book yet.

analysis-shape = Shape
analysis-repetition = Repetition
analysis-synopsis = Synopsis
analysis-voice = Voice

analysis-words-per-scene = Words per scene
analysis-median-words = This book's median scene runs { $count } words.
analysis-median-line = Median: { $count } words
analysis-dialogue = Dialogue
# Said instead of showing 0%, which would read as "there is no dialogue here".
analysis-dialogue-unsupported = Dialogue is not measured for this language yet.

analysis-footnote-words = Footnote words
# Kept apart from the manuscript total on purpose — see the module doc on
# AnalysisViewModel::run for why a footnote is authored prose but must not be folded
# into how far along the story reads as being.
analysis-footnote-words-count = { $count } words are in this book's footnotes, kept apart from the manuscript total.
# The figure comes from its own operation and can still be catching up even once the
# rest of this report is ready — said plainly rather than shown as a misleading 0.
analysis-footnote-words-pending = Counting the footnotes…

analysis-echoes = Repeated words
# What the list is, before the list. Says the filtering out loud: a writer who notices
# "the" missing from a repetition report should know that was a decision.
analysis-echoes-explainer = Distinctive words you used twice within about a page of each other, closest pair first. Everyday words are left out. Repetition is not a fault — this is only where a reader is most likely to notice one. Pick a scene to open it.
analysis-no-echoes = No word repeats closely enough to stand out.
# The gap is the *tightest* pair, which is the number that decides whether a reader hears
# the repeat at all — so it is said as a relationship between two uses, not as a property
# of the word. "4 times, 12 words apart at the closest" left the reader to work out which
# two of the four were 12 words apart.
analysis-echo-row = “{ $word }” — { $count } times, two of them { $gap } words apart
# The tree's parent row: a text, and how many distinct words echo inside it. The count is
# what decides whether the row is worth opening, which is the whole point of starting closed.
analysis-repetition-text-tooltip =
    { $count ->
        [one] One distinctive word repeats closely in this text. Click to open it.
       *[other] { $count } distinctive words repeat closely in this text. Click to open it.
    }
# The two numbers on a word row, said as prose. The count is deliberately not "how many
# times the word appears": uses too far from any other use are not part of the finding, and
# a reader comparing the number against the scene would otherwise think it wrong.
analysis-repetition-word-tooltip = { $count } uses of “{ $word }” sit close enough together to be heard as a repeat — not necessarily every time it appears here. The closest two are { $gap } words apart, which is what decides whether a reader notices.
# The gap column, abbreviated. "w" for words: the column is read against the tooltip that
# spells it out, and a full "words" would double the column's width for no added meaning.
analysis-repetition-gap-short = { $gap }w
analysis-similar-scenes = Similar scenes
analysis-similar-explainer = Two scenes that share long runs of the same wording. Usually a scene that was copied and then edited, or one that was split in two and never grew apart.
analysis-no-similar-scenes = No two scenes share long stretches of wording.
analysis-similar-row = “{ $a }” and “{ $b }” share about { $percent }% of the shorter one's wording.
analysis-more-rows = { $count } more not shown.

# Shape's charts draw one bar per text, so an outlined-but-unwritten book is mostly gaps.
analysis-ignore-empty = Ignore texts with nothing written yet
analysis-empty-hidden = { $count } empty { $count ->
        [one] text
       *[other] texts
    } hidden.
analysis-all-texts-empty = Every text in this book is still empty.

analysis-synopsis-drift = Synopsis and prose
analysis-no-synopses = No synopses written yet, so there is nothing to compare.
analysis-no-drift = Every synopsis tracks its scene about as closely as the others.
analysis-drift-row = “{ $title }” — its synopsis mentions { $terms }, and the prose does not.

analysis-vocabulary = Vocabulary
# Named after what it does, not after the statistic. The window is the part worth
# explaining: it is why a 200,000-word book is not automatically "more varied" than a
# novella, which is the trap a plain type-token ratio falls into.
analysis-vocabulary-explainer = How much the wording varies, measured over a sliding window so a long book is not scored higher for its length alone. It describes the writing, it does not judge it: plain prose scores lower than ornate prose by design, and neither is better.
analysis-words-measured = { $words } words, { $distinct } distinct.
analysis-mattr = Vocabulary variety: { $value }
analysis-mattr-scale = 0 would be one word repeated forever; 1 would be a book that never reuses a word. Real prose sits well inside those ends, and there is no target to reach.
analysis-not-enough-text = Not enough text yet to measure this.
# Surface forms only: an inflected language scores higher for reasons that have
# nothing to do with the writer, so the figure is comparable within one book and
# one language and nowhere else.
analysis-vocabulary-caveat = Comparable within this book only.

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

image-insert = &Insert image…
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
image-describe-placeholder = a lighthouse against a grey sky
image-resize-title = Resize the image
image-resize-explain = A percentage of the size it is shown at now. 100 leaves it as it is.
image-resize-invalid = Enter a number between 1 and 1000.
image-menu-describe = &Describe the image…
image-menu-resize = &Resize the image…
image-menu-reset-size = Original si&ze

# The book's cover — chosen from the book, not typed into a scene.
cover-choose = Book &cover…
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
export-orphan-footnotes =
    { $count ->
        [one] One footnote is no longer referenced anywhere in the manuscript. Its text will not appear in the exported book.
       *[other] { $count } footnotes are no longer referenced anywhere in the manuscript. Their text will not appear in the exported book.
    }

## Footnotes

menu-footnotes = Foo&tnotes
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

# ── Import documents (Markdown / plain text) ──────────────────────────────────
import-document-title = Import documents
import-document-close = Close
import-document-step-files = Files
import-document-step-review = Review
import-document-drop-title = Drop documents here
import-document-drop-hint = Markdown (.md) and plain text (.txt)
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
import-document-col-source = Source
import-document-level-rules = Heading levels
import-document-level-n = Heading { $level }
import-document-destination = Destination
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
import-diagnostic-illegal-combination = “{ $title }” carries prose, but a { $kind } cannot hold any. Its text would be dropped — change its type.
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
versions-pin = Pin this version — automatic cleanup will never delete it
versions-unpin = Unpin this version — automatic cleanup may delete it again
versions-pinned-only = Show only pinned versions
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
versions-changed-percent = { $percent }% of this changed
versions-hidden-paragraphs = { $count ->
    [one] … { $count } unchanged paragraph …
   *[other] … { $count } unchanged paragraphs …
}

# ── Timeline band ──
timeline-title = Timeline
timeline-coverage = { $count ->
    [one] { $count } version recorded, going back to { $oldest }
   *[other] { $count } versions recorded, going back to { $oldest }
}
timeline-loading = Looking through your project's past…
timeline-empty = No version of this project has been recorded yet
timeline-no-changes = Nothing has changed since then
timeline-slider-label = Recorded version
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
timeline-reader-deleted = This is no longer in your project. You can read it and copy it out here.
