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
