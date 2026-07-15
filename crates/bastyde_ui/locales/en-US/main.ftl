# Skribisto — UI strings (source locale).
# A single `&` marks the keyboard mnemonic of a menu label; `&&` is a literal `&`.

## Menu bar — File
menu-file = &File
menu-new-work = &New Work
menu-open-work = &Open Work…
menu-import-from = &Import from
menu-import-plume = &Plume Creator (.plume)…
menu-export = &Export
menu-export-book = Export Book
menu-export-part = Export Part
menu-export-chapter = Export Chapter
menu-export-scene = Export Scene
menu-export-note = Export Note
menu-export-folder = Export Folder
menu-export-choose = Choose…
menu-export-none = Open a document to export
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
menu-search = &Search in Project
menu-search-preview = Search &Preview

## Binder context menu
ctx-add = &Add
ctx-new-item = &New Item
ctx-new-folder = New &Folder
ctx-rename = &Rename
ctx-duplicate = &Duplicate
ctx-trash = Move to &Trash
ctx-open-to-side = Open to the &Side

## Create recommendations — logical type labels (SplitButton title + Add ▸ rows)
create-book = Book
create-part = Part
create-chapter = Chapter
create-scene = Scene
create-note = Note
create-note-folder = Note folder
create-folder = Folder
create-book-end = End of Book

## Create recommendations — trailing placement hint on each row
placement-inside = inside
placement-after = after
placement-after-parent = after parent
placement-top-level = top level

# (Writing-model rich tooltips live in this locale's tooltips.ftl.)

## Promote — convert a binder item to its paired type
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
promote-blocked-title = Chapter not empty
promote-blocked-text = This chapter still holds { $count } item(s). Move or trash them before converting it to a flat chapter.

## Inspector (trailing dock) + status-bar dock toggles
inspector = Inspector
inspector-empty = Open an item to inspect it.
inspector-promote = Convert to…
statusbar-toggle-outline = Toggle the binder
statusbar-toggle-inspector = Toggle the inspector
# The save indicator (status bar, next to the binder toggle).
statusbar-save-unsaved = Unsaved changes — click to save
statusbar-save-saved = All changes saved
statusbar-save-autosave = Autosave is on — changes are saved as you write
statusbar-saving = Saving…

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
settings-show-welcome = Show the launcher at startup (otherwise, reopen the last project)

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
settings-chapter-flat = Flat chapters
settings-chapter-flat-hint = On: a chapter is a single row. You write into it, and it holds no scenes. Off: a chapter is a folder. You still write into it, but it can hold scenes as well. New chapters follow this setting; existing ones convert via Promote.

## Welcome
welcome-title = Welcome to Skribisto
# $version is stamped from the git tag at build time (see src/version.rs).
welcome-version = Version { $version }
welcome-search = Search works
welcome-open = Open
welcome-new-work = New Work
welcome-recent-works = Recent Works
welcome-empty-recents = No recent works yet.
# Shown in place of the recents list when the search matched none of them —
# distinct from having no recent works at all.
welcome-no-matches = No recent work matches your search.
welcome-learn-soon = Guides and tips are coming soon.
welcome-about-blurb = Skribisto — a Rust + Bastyde rewrite of the writing app.
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
corkboard = Corkboard
overview = Overview
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
unsaved-changes = This work has unsaved changes.
# Replacing the open work in this window (New Work, Open Work, "Open here", the
# import toast's "Open now") — the same guard as closing, since the open work is
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
tooltip-welcome = Welcome

## Toasts
could-not-open-work = Could not open work: { $error }
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
backup-failed-close-text = No backup copy could be written before closing — every destination failed (the drive may have been removed, or it may be full or write-protected). Fix it and retry, or exit without backing up.

## Open-a-backup (choice modal + permanent banner + restore)
backup-choice-title = Backup file
backup-choice-heading = You've opened a backup copy
backup-choice-subtitle = This is a point-in-time backup of a project.
backup-choice-subtitle-dated = Backup taken { $date }.
backup-choice-body = You can open it and edit freely, but changes can only be kept with Save As — the original project file is not touched. Or restore this project to exactly this backup.
backup-choice-open = Open the backup
backup-choice-restore = Restore this project to this point…
backup-choice-not-a-backup = No, open it normally
backup-banner-title = Backup copy — changes can't be saved here
backup-banner-description = Use Save As to keep your edits in a new file, or Restore to replace the original project with this backup.
backup-banner-restore = Restore…
backup-banner-save-as = Save As…
restore-original-missing = Can't find the original project to restore over. Use Save As to keep this backup as a new project instead.
restore-close-elsewhere-title = Project open in another window
restore-close-elsewhere-text = The project you're restoring over is open in another window. Please close it there first, then retry.
restore-focus-window = Focus that window
restore-confirm-title = Restore this backup?
restore-confirm-text = The current version of the project will be copied aside as a safety backup before being replaced with this one.
restore-confirm-ok = Restore
restore-error = Could not restore: { $error }
restored-ok = Project restored.
restored-with-safety = Project restored. Your previous version was saved to { $path }.
close-backup-discard-title = Discard changes to this backup?
close-backup-discard-text = Changes to a backup can't be saved to it. Use Save As to keep them, or discard and close.
quit-backup-discard-title = Discard changes and quit?
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
settings-backup-dest-none = No destinations — backups are saved next to the project.
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
export-cancel = Cancel
export-export = Export
export-save-dialog-title = Export to file
# Output formats
export-format-docx = Word (.docx)
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
# Error toast: a short reason in the body, the full technical chain behind Details
export-error-title = Could not export
export-error-details = Details

## Backup scheduler (progress toast + failure/prune-warning details — backup review, T1-2/T1-7/T2-3/T2-8/T2-9)
backup-progress-start = Starting…
backup-progress-retention = Cleaning up old backups…
backup-progress-done = Done
backup-progress-destination = Destination { $i } of { $n }
backup-details = Details
backup-issues-title = Backup issues
backup-failed-title = Backup failed
backup-complete-prune-warning = Backup complete ({ $ok } saved, { $skipped } already current) — some old backups could not be removed

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
search-tip-case = Match case — treat uppercase and lowercase as different, so “Elena” and “elena” are separate matches.
search-tip-whole-word = Whole word — match only complete words, so “cat” is not found inside “category”.
search-tip-diacritics = Match accents — treat accented letters as distinct, so “cafe” does not match “café”.
search-tip-body = Body — search the prose of scenes and notes.
search-tip-title = Title — search the titles of binder items.
search-tip-synopsis = Synopsis — search each writing row’s summary.
search-tip-label = Label — search the short note shown under a binder item’s title.
search-tip-book = Books — the book container and its begin / end markers.
search-tip-part = Parts — part-level dividers.
search-tip-chapter = Chapters — chapters, in whichever way the project stores them.
search-tip-scene = Scenes — the rows that hold your prose.
search-tip-note = Notes — free-form notes.
search-tip-folder = Folders — plain organising folders and separators.
search-tip-replace = Replace — show the replacement field and Replace All.
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
dict-add-code-hint = A short tag of your choosing — this is what you pick as a document's language.
dict-add-aff = Affix file (.aff)
dict-add-dic = Word list (.dic)
dict-add-submit = Add
dict-add-cancel = Cancel
dict-add-name-required = Give the dictionary a name.
dict-add-code-required = Enter a language code.
dict-add-code-invalid = Use only letters, digits, and - _ .
dict-add-code-reserved = That is a built-in dictionary code — choose another.
dict-add-file-required = Choose a file.
dict-add-file-missing = That file does not exist.
dict-add-code-taken = A dictionary for that code is already installed.
dict-add-done = Added { $name }
dict-add-unusable = These files are not a usable dictionary: { $error }
dict-add-failed = Couldn't add the dictionary: { $error }
# The language pill field (Inspector + Settings)
inspector-dict-language = Language
settings-page-language = Language
settings-field-dict-language = Languages
dict-tradeoff-hint = Each extra language accepts more words, so fewer mistakes are caught.
lang-inherit-hint = Inherited — this scene uses the book's or project's languages.
lang-pill-list = Languages
lang-pill-add = Add a language
lang-pill-remove = Remove { $name }
lang-pill-mute = Turn spell-checking off for { $name }
lang-pill-unmute = Turn spell-checking on for { $name }
