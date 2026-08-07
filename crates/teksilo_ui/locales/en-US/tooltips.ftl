# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Skribisto: writing-model rich tooltips (source locale).
# Registered in tooltip_registry.rs and attached by key from the "＋ Create" /
# "Convert to" menus. The wm-*-more bodies cascade to one another via
# [label](:key) links, so this file doubles as an in-place Help doc.

wm-book = A book in your project.
wm-book-more = A book is as simple or as layered as you like. A short story can be just a few [scenes](:wm-scene). A longer work can nest [parts](:wm-part) and [chapters](:wm-chapter) as deep as you want. The shape is yours.
wm-part = A part grouping chapters.
wm-part-more = A part gathers [chapters](:wm-chapter) under one heading. Like a chapter, it runs until the next part or the end of the [book](:wm-book).
wm-chapter = A chapter of the book.
wm-chapter-more = A chapter runs from here until the next chapter, the next [part](:wm-part), or the [end of the book](:wm-end-of-book). It comes in two shapes: a chapter folder that holds [scenes](:wm-scene), or a flat chapter that carries its own prose. Choose which one new chapters use in Settings, under Structure.
wm-scene = A scene of prose.
wm-scene-more = A scene holds your writing. Every writing item has two sides: the main text and a [synopsis](:wm-synopsis) to plan with. Both open together in the editor.
wm-note = A note.
wm-note-more = A note keeps text and a [synopsis](:wm-synopsis), like a [scene](:wm-scene), for research, asides, or reminders. Any item can be part of the compiled book, including a note sitting inside a [chapter](:wm-chapter).
wm-note-folder = A note that holds other items.
wm-note-folder-more = A [note](:wm-note) that also contains items beneath it, so you can group related notes together. It keeps its own [synopsis](:wm-synopsis), and the items inside keep their own text.
wm-folder = An organising folder.
wm-folder-more = A container for arranging items in the binder however suits you. It carries a [synopsis](:wm-synopsis) but no prose of its own. The items inside hold the writing.
wm-end-of-book = Marks where a book ends.
wm-end-of-book-more = Every book shares one continuous list, so a book's end is not decided by nesting. This marker says the [book](:wm-book) stops here. Anything after it belongs to the next book.
wm-synopsis = A summary of a writing item.
wm-synopsis-more = A summary attached to any writing item, usually a short paragraph but as long as you like. Use it to plan and navigate before the prose exists. It sits beside the main text in the editor.

scene-break-minor = An ordinary scene break — a shift of time, place or viewpoint within a chapter.
scene-break-minor-more =
    Marks a break *where you put it*, including in the middle of a [scene](:wm-scene) — splitting prose into two items is an organisational choice, so it never creates a break on its own.

    Typed into the prose as `* * *`. How it prints is decided by the export style, not by what you type: a Shunn manuscript sets it as `#`, a trade paperback as a dinkus, and most French, German, Spanish, Russian and Italian publishing as a bare gap with no mark at all.
scene-break-major = A stronger division — a large time skip, or a decisive change of viewpoint.
scene-break-major-more =
    The same idea as an ordinary scene break, one step up. Use it when a plain break would understate the jump.

    Typed as `# # #`, and printed differently from the ordinary tier by the export style — Shunn's standard manuscript format sets `#` against `# # #` for exactly this distinction. Where a tradition has no stronger mark, both tiers print the same.
wm-story-bible = Skribisto looks for this item in your prose.
wm-story-bible-more = Turn this on for tags that name things you write about: characters, places, objects. Any item carrying such a tag is matched against your prose by its title and by the other names you give it, so each [scene](:wm-scene) lists who and what appears in it without you linking anything by hand. Leave it off for tags that describe an item rather than name one, such as a draft status or a reminder to check continuity.

tooltip-go-to = Jump to any item in the binder (Ctrl+G)
synopsis-collapse-tooltip = Hide the synopsis column

wm-paratext = A text that is not part of the story.
wm-paratext-more = A preface, a dedication, an afterword, a colophon — writing that belongs to the book but not to its body. Exported wherever you put it, and never counted in the manuscript's word count. Where it goes is up to you: conventions differ by country and publisher.
wm-paratext-folder = A folder for paratexts.
wm-paratext-folder-more = Somewhere to keep prefaces and afterwords so they do not clutter the binder. Organising only — it carries a [synopsis](:wm-synopsis) but adds nothing to the exported book, not even its own name.
