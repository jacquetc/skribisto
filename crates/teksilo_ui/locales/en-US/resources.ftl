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

## A folder's Resources page

segment-folder = Folder
resources-folder-count = { $count ->
    [one] One resource
   *[other] { $count } resources
}
resources-view-label = Show resources as
resources-view-list = List
resources-view-grid = Grid
resources-grid-label = Resources
resources-folder-drop-hint = Drop files or web addresses to attach them to this folder
resources-folder-empty = Nothing is attached to this folder or to anything in it yet. Add files or web links with the buttons above, or drop them here: they are attached to the folder.
resources-attached-to = On
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] and one more
   *[other] and { $count } more
}
resources-row-name-attached = { $name }, on { $items }
resources-open-item = Open “{ $title }”
resources-detach-from = Detach from “{ $title }”

## Picture resources in the Gallery

gallery-resource-badge = Resource
gallery-tile-resource = Picture resource of { $title }: { $description }

## The Work's Resources settings page

settings-page-resources = Resources
settings-desc-resources = The files and web links no item carries any more: attach them again, remove them, or export copies
settings-resources-description = A resource is attached to nothing once it is detached from its last item, or once the item it was on is deleted for good. It stays in the project, and a stored file travels with every copy and every backup of it, until you attach it again or remove it.
settings-resources-group-storage = Storage
settings-resources-group-unattached = Attached to nothing
settings-resources-stored = Files stored in this project: { $size }
settings-resources-folder-advice = A project kept as a single file is written whole at every save, its stored files too. Kept as a folder, it saves only what changed, which is faster.
settings-resources-count = { $count ->
    [one] One resource
   *[other] { $count } resources
}
settings-resources-empty = Every resource of this project is attached to an item.
resources-attach-to-items = Attach to items…
resources-export-copies = Export copies…
resources-export-copy = Export a copy…
resources-remove-many-title = Remove { $count } resources from the project?
resources-attach-title = Attach to items
resources-attach-hint = Choose the items to attach to. Ctrl+click or Shift+click chooses several.
resources-attach-empty = This project has no items yet.
resources-attach-confirm = Attach
resources-attached = { $count ->
    [one] One resource attached
   *[other] { $count } resources attached
} to { $items ->
    [one] one item
   *[other] { $items } items
}
resources-export-title = Export copies to a folder
resources-exported = { $count ->
    [one] One resource exported
   *[other] { $count } resources exported
}
resources-export-missing = { $count ->
    [one] One file is not where the project expects it, and was left out: { $names }
   *[other] { $count } files are not where the project expects them, and were left out: { $names }
}
resources-export-failed = { $count ->
    [one] One file could not be copied: { $names }
   *[other] { $count } files could not be copied: { $names }
}

## The large-file preference (Editor behavior)

settings-resources-large-file = Files over 10 MB
settings-resources-large-ask = Ask each time
settings-resources-large-store = Always store in the project
settings-resources-large-link = Always link to the file
settings-hint-resources-large-file = What happens to a file over 10 MB you attach as a resource. Stored in the project, it travels with every copy and every backup of it. Linked, it stays where it is, and the project only remembers where to find it.
