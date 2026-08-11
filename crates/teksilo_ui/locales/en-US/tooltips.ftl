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

# ── Word / character targets ──────────────────────────────────────────────────
# A second cascade web beside the writing-model one. It doubles as the app's only
# explanation of how counting works, since there is no Help surface anywhere else.
goal-target = How long this piece is meant to be.
goal-target-more = Set a target on anything: a [scene](:wm-scene), a [chapter](:wm-chapter), a [part](:wm-part), the whole [book](:wm-book). Each one is its own number and stands on its own — setting a target on a chapter never changes its book's, and a book's target is never a sum of the chapters inside it. What does add up is the writing: a container's [progress](:goal-progress) is everything written beneath it. Leave a target at zero and it simply is not set. The number is counted in [words or characters](:goal-unit), whichever this project uses.
goal-unit = Whether this project counts in words or in characters.
goal-unit-more = A project-wide choice, because a length unit belongs to the manuscript and its market rather than to one section: German and French publishing measure in signs, Japanese in 400-character sheets, Chinese by the thousand characters, most English-language publishing in words. Both numbers are kept side by side and neither is ever converted into the other, so switching the unit points every [target](:goal-target) at the other figure and switching back restores the first. What it does not do is convert anything, so targets already entered will need updating by hand.
goal-progress = How much is written, against the target.
goal-progress-explained = A piece with prose of its own is measured against its own length; a container against [everything written beneath it](:goal-manuscript-words). The bar warms from red through amber to green as the writing comes in, and changes again once the piece has run well past what was planned, which is worth noticing when a chapter has a length to keep to.
goal-manuscript-words = What counts as the manuscript.
goal-manuscript-words-more = Only prose that would actually be exported: [scenes](:wm-scene) and [chapters](:wm-chapter) carrying their own text. A [note](:wm-note) or a [paratext](:wm-paratext) is not part of the manuscript and is never counted toward one. Anything in the trash drops out, and so does anything [left out of the export](:goal-exportable).
goal-exportable = This row is left out of the export.
goal-exportable-more = Its own length still shows, because the writing is still there — but it counts toward no total, exactly as it will appear in no exported book. Leaving a container out does not leave the pieces inside it out: each row answers for itself, which is what the "Apply to children" button beside the switch is for.
goal-distribute = Share this target out across what is inside.
goal-distribute-more = Splits a [book's](:wm-book) or [part's](:wm-part) target across its immediate children so the pieces add up to it exactly. By default it fills only the ones with no target yet and leaves your own numbers alone. What it writes are ordinary [targets](:goal-target), free to edit afterwards and free to drift apart: this is one action, not a standing arrangement.
goal-subtree-total = What the targets inside add up to.
goal-subtree-total-more = A fact, not a target. This project never treats a container's target as the sum of the ones inside it, because that is how a folder's figure ends up moving on its own when a scene inside it is given a number. This line simply tells you what your own numbers come to, so you can compare it with the container's [target](:goal-target) yourself, or [share one out](:goal-distribute).
goal-milestone = A date to reach something by.
goal-milestone-more = Two kinds. One says the [book](:wm-book) should stand at a given length by a date, a waypoint on its own curve. The other says a particular [chapter](:wm-chapter) or [part](:wm-part) should have reached [its own target](:goal-target) by then, and reads that number live rather than keeping a copy of it, so editing the target updates the milestone too. Both appear on the book's [pace plan](:pace-plan).
pace-plan = The book's writing schedule.
pace-plan-more = A [target](:goal-target) and a deadline, with the days you write on, turned into how much a day needs. It reads the book's own target, so setting it here and setting it in the Inspector are the same number in two places. Waypoints along the way are [milestones](:goal-milestone).
