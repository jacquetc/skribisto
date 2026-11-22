# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Resources: files and web links attached to binder items.

## The dock

resources-title = Resources
shortcut-name-resources-show = Show the resources
resources-add-files = Add files
resources-add-files-title = Add files to this item
resources-add-link = Add a web link
resources-drop-hint = Drop files or web addresses to attach them
resources-no-item = Open an item to see its resources.
resources-empty = Nothing is attached to this item yet. Add files or web links with the buttons above, or drop them here.
resources-actions = Actions
resources-not-found = Not found
resources-row-linked = Linked file
resources-row-linked-size = Linked file, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, not found

## File types, as a screen reader hears them

resources-kind-picture = picture
resources-kind-pdf = PDF document
resources-kind-audio = sound recording
resources-kind-video = video
resources-kind-document = document
resources-kind-ebook = e-book
resources-kind-archive = archive
resources-kind-file = file
resources-kind-web-link = web link

## A row's menu

resources-open = Open
resources-show-in-folder = Show in folder
resources-locate = Locate…
resources-insert-into-text = Insert into the text
resources-insert-picture = Insert the picture into the text
resources-insert-link = Insert a link into the text
resources-edit = Rename and describe…
resources-detach = Detach from this item
resources-remove = Remove from the project…

## The two dialogs

resources-link-dialog-title = Add a web link
resources-link-dialog-address = Address
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Title
resources-link-dialog-name-placeholder = The name of the site, if left empty
resources-link-dialog-add = Add
resources-edit-dialog-title = Rename and describe
resources-edit-dialog-name = Title
resources-edit-dialog-description = Description
resources-edit-dialog-description-placeholder = What this is, and why you kept it
resources-edit-dialog-save = Save
resources-dialog-cancel = Cancel

## Adding

resources-large-title = A large file
resources-large-text = “{ $name }” is { $size }. Stored in the project, it travels with every copy and every backup of it. Linked, it stays where it is, and the project only remembers where to find it.
resources-large-text-many = “{ $name }” is { $size }, and { $others ->
    [one] one other file is
   *[other] { $others } other files are
} large too. Stored in the project, they travel with every copy and every backup of it. Linked, they stay where they are, and the project only remembers where to find them.
resources-large-store = Store it in the project
resources-large-link = Link to it instead
resources-large-remember = Don't ask again
resources-storing-title = Adding files to the project
resources-store-cancel = Cancel
resources-store-cancelled = No files were added.
resources-store-busy = Files are still being added. Try again when they are in.
resources-store-failed = The files could not be added: { $error }
resources-no-project = Open a project before attaching files.
resources-moved-away = The project moved while the file was being copied.
resources-some-failed = { $count ->
    [one] One file could not be added: { $names }
   *[other] { $count } files could not be added: { $names }
}
resources-add-failed = The resources could not be added: { $error }
resources-added = { $count ->
    [one] One resource added
   *[other] { $count } resources added
}
resources-dropped-added = { $count ->
    [one] Added as a resource of this item, and linked in the text
   *[other] { $count } files added as resources of this item, and linked in the text
}
resources-undo = Undo
resources-link-refused = “{ $url }” is not a web address Skribisto can keep. Use an address that starts with http:// or https://, without a user name or password.

## The single-file project notice

resources-folder-title = This project is getting large
resources-folder-text = Its stored files now take { $size }. A project kept as a single file is written whole at every save, its files too. Kept as a folder, it saves only what changed. You can save it as a folder now, and keep working in the folder from then on.
resources-folder-switch = Save as a folder…
resources-folder-not-now = Not now
resources-folder-never = Don't ask again for this project

## Opening

resources-not-found-toast = “{ $name }” is not where the project expects it.
resources-unsafe-title = Open “{ $name }”?
resources-unsafe-text = Skribisto does not know this kind of file to be safe. Opening it could start a program on your computer, and a project can come from someone else. Show it in its folder to see what it is first.
resources-open-anyway = Open anyway
resources-open-failed = The file could not be opened: { $error }
resources-no-data-folder = this system gives Skribisto no data folder to open a copy in
resources-link-gone = This link points to a resource the project no longer has.
resources-locate-title = Where is “{ $name }” now?
resources-relinked-more = { $count ->
    [one] One more missing file was found in the same folder, and linked again.
   *[other] { $count } more missing files were found in the same folder, and linked again.
}
resources-changed-elsewhere = “{ $name }” was changed in another app. The project keeps the new version.

## Editing

resources-edit-failed = The resource could not be changed: { $error }
resources-remove-title = Remove “{ $title }” from the project?
resources-remove-text = It is taken off every item it is attached to. A file stored in the project goes with it; a linked file and a web page stay where they are. Undo brings it back.
resources-remove-confirm = Remove
resources-remove-failed = The resource could not be removed: { $error }

## Into the text

resources-no-text = Put the cursor in a text first, then insert the resource there.
resources-drop-insert-picture = Insert the picture
resources-drop-insert-link = Insert a link to the resource

## Progress

progress-resources-storing = Copying the files
progress-resources-stored = Files copied
