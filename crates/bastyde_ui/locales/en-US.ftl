# Skribisto — UI strings (source locale).
# A single `&` marks the keyboard mnemonic of a menu label; `&&` is a literal `&`.

## Menu bar — File
menu-file = &File
menu-new-work = &New Work
menu-open-work = &Open Work…
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
ctx-new-item = &New Item
ctx-new-folder = New &Folder
ctx-rename = &Rename
ctx-duplicate = &Duplicate
ctx-trash = Move to &Trash

## Settings
language = Language
theme = Theme
english = English
french = French
light = Light
dark = Dark
settings-text-width = Text width
settings-autosave = Autosave to disk
settings-show-welcome = Show Welcome at startup

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

## Binder / recents
binder = Binder
no-work = No work
no-recent-works = No recent works

## Dialogs
dialog-rename = Rename
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
