# Skribisto — UI strings (source locale).
# A single `&` marks the keyboard mnemonic of a menu label; `&&` is a literal `&`.

## Menu bar — File
menu-file = &File
menu-new-work = &New Work
menu-open-work = &Open Work…
menu-import-from = &Import from
menu-import-plume = &Plume Creator (.plume)…
menu-save = &Save
menu-save-as-file = Save as single &file…
menu-save-as-folder = Save as fol&der…
menu-backup = &Back up now
menu-close-work = &Close Work
menu-welcome = &Welcome…
menu-settings = S&ettings
menu-quit = &Quit

## Menu bar — View
menu-view = &View
menu-outline = &Outline

## Binder context menu
ctx-add = &Add
ctx-new-item = &New Item
ctx-new-folder = New &Folder
ctx-rename = &Rename
ctx-duplicate = &Duplicate
ctx-trash = Move to &Trash

## Create recommendations — logical type labels (SplitButton title + Add ▸ rows)
create-book = Book
create-part = Part
create-chapter = Chapter
create-scene = Scene
create-note = Note
create-note-folder = Note folder
create-folder = Folder
create-book-end = End of Book

## Create recommendations — rich tooltips ({ $kind } = type, { $target } = anchor title)
create-tooltip-child = Add a new { $kind } inside “{ $target }”.
create-tooltip-sibling = Add a new { $kind } after “{ $target }”.
create-tooltip-parent-sibling = Add a new { $kind } after the enclosing “{ $target }”.
create-tooltip-top = Add a new { $kind } at the top level.

## Promote — convert a binder item to its paired type
ctx-promote-to = Promote to { $target }
promote-chapter-folder = Chapter folder
promote-flat-chapter = Flat chapter
promote-blocked-title = Chapter not empty
promote-blocked-text = This chapter still holds { $count } item(s). Move or trash them before converting it to a flat chapter.

## Inspector (trailing dock) + status-bar dock toggles
inspector = Inspector
inspector-empty = Open an item to inspect it.
inspector-promote-to = Promote to { $target }
statusbar-toggle-outline = Toggle the binder
statusbar-toggle-inspector = Toggle the inspector

## Settings
language = Language
theme = Theme
english = English
french = French
light = Light
dark = Dark
settings-text-width = Text width
settings-autosave = Autosave to disk
settings-show-welcome = Show the Welcome screen at startup

## Settings window — chrome
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

## Settings window — categories
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
settings-page-dictionaries = Dictionaries
settings-page-autosave = Autosave
settings-page-export = Export Formats
settings-page-keymap = Keymap

## Settings window — fields
settings-group-typography = Typography
settings-group-writing-column = Writing column
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
settings-field-app-theme = Theme
settings-field-text-scale = Interface text size
settings-field-language = Interface language
settings-synopsis-pane = Show synopsis pane above the manuscript
settings-typewriter = Typewriter scrolling (keep caret line centred)
settings-highlight-sentence = Highlight the current sentence
settings-autosave-hint = Changes are written to disk automatically as you write.

## Settings — Work (the open project)
settings-sec-work = Work
settings-page-structure = Structure
settings-group-chapters = Chapters
settings-chapter-flat = Write directly in chapters
settings-chapter-flat-hint = On: each chapter is a single writing surface (flat). Off: chapters are folders that hold scenes. New chapters follow this; existing ones change via Promote.

## Welcome
welcome-title = Welcome to Skribisto
welcome-close = Close
welcome-search = Search works
welcome-open = Open
welcome-new-work = New Work
welcome-recent-works = Recent Works
welcome-empty-recents = No recent works yet.
welcome-learn-soon = Guides and tips are coming soon.
welcome-about-blurb = Skribisto — a Rust + Bastyde rewrite of the writing app.
welcome-tagline = A quiet place to write long things.
welcome-show-at-startup = Show at startup
nav-works = Works
nav-examples = Examples
nav-learn = Learn
nav-about = About

## Editor tabs & panes
synopsis = Synopsis
corkboard = Corkboard
overview = Overview
text-heading = Text
no-content = This item has no editable content.
untitled = Untitled
placeholder-title = Title…
placeholder-subtitle = Subtitle…
placeholder-chapter-title = Chapter title…

## Full Chapter view
full-chapter = Full Chapter
rename = Rename
set-label = Set label
insert-scene = Insert scene
split-scene = Split scene
move-up = Move up
move-down = Move down
merge-with-previous = Merge with previous
move-to-trash = Move to trash
rename-chapter = Rename chapter
add-scene = Add new scene
placeholder-scene-name = Scene name…
menu-cut = Cut
menu-copy = Copy
menu-paste = Paste
menu-paste-unformatted = Paste Unformatted
menu-select-all = Select All

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
close-question = Save changes before closing?
unsaved-changes = This work has unsaved changes.
tooltip-welcome = Welcome

## Toasts
could-not-open-work = Could not open work: { $error }
could-not-open-example = Could not open example: { $error }
could-not-create-work = Could not create work: { $error }
saving-as-file = Saving as { $target }…
saving-as-folder = Saving as { $target }/…
save-error = Could not save: { $error }
backup-error = Could not back up: { $error }
backing-up = Backing up…

## New-work dialog
new-work-title = New Work
new-work-close = Close
new-work-name = Work name
new-work-name-placeholder = Untitled
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
new-work-chapter-scene = Write directly in chapters
new-work-chapter-scene-tip = Each chapter becomes a single page you write straight into (a *ChapterScene*). Leave this off for the classic layout — every chapter is a folder holding one empty scene, better when a chapter has several scenes.
new-work-chapter-scene-tip-more = Skribisto's binder tree is organisational only, so both layouts compile to the same book. A chapter *folder* is defined by the scenes inside it; a *ChapterScene* is the flat equivalent that opens the chapter and holds its prose in one row. You can mix the two freely later.
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
import-plume-name-exists = A file with this name already exists here — Import will confirm overwrite
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
# Error toast: a short reason in the body, the full technical chain behind Details
import-plume-error-title = Could not import the project
import-plume-error-details = Details
