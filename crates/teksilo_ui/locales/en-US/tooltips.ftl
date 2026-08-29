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
wm-folder = An organizing folder.
wm-folder-more = A container for arranging items in the binder however suits you. It carries a [synopsis](:wm-synopsis) but no prose of its own. The items inside hold the writing.
wm-end-of-book = Marks where a book ends.
wm-end-of-book-more = Every book shares one continuous list, so a book's end is not decided by nesting. This marker says the [book](:wm-book) stops here. Anything after it belongs to the next book.
wm-story-bible-entry = A tagged, aliased note about your story.
wm-story-bible-entry-more = A [note](:wm-note) about a character, a location, or anything else worth filing, created with a name, tags, aliases and a starting template all in one step. Nothing on the page distinguishes it afterward, since it stays an ordinary note you can rename, retag or convert like any other.
wm-synopsis = A summary of a writing item.
wm-synopsis-more = A summary attached to any writing item, usually a short paragraph but as long as you like. Use it to plan and navigate before the prose exists. It sits beside the main text in the editor.

scene-break-minor = An ordinary scene break: a shift of time, place or viewpoint within a chapter.
scene-break-minor-more =
    Marks a break *where you put it*, including in the middle of a [scene](:wm-scene). Splitting prose into two items is an organizational choice, so it never creates a break on its own.

    Typed into the prose as three asterisks separated by spaces. How it prints is decided by the export style, not by what you type: a Shunn manuscript sets it as a single #, a trade paperback as a dinkus, and most French, German, Spanish, Russian and Italian publishing as a bare gap with no mark at all.
scene-break-major = A stronger division: a large time skip, or a decisive change of viewpoint.
scene-break-major-more =
    The same idea as an ordinary scene break, one step up. Use it when a plain break would understate the jump.

    Typed as # # #, and printed differently from the ordinary tier by the export style. Shunn's standard manuscript format sets a single # against # # # for exactly this distinction. Where a tradition has no stronger mark, both tiers print the same.
wm-find-in-prose = Skribisto looks for this item in your prose.
wm-find-in-prose-more = Turn this on for tags that name things you write about: characters, places, objects. Any item carrying such a tag is matched against your prose by its title and by the other names you give it, so each [scene](:wm-scene) lists who and what appears in it without you linking anything by hand. Leave it off for tags that describe an item rather than name one, such as a draft status or a reminder to check continuity. Every item it matches also joins your [story bible](:wm-story-bible).
wm-story-bible = Every character, place and other tagged entry, gathered in one grid.
wm-story-bible-more = Open from any notes folder's own tab. Cards group by tag, and each names how many aliases an entry answers to and how many scenes it has turned up in, drawn from every [find in prose](:wm-find-in-prose) tag in the project. Filtering by book only narrows which cards show; it never decides which entries exist.

tooltip-go-to = Jump to any item in the binder (Ctrl+G)
synopsis-collapse-tooltip = Hide the synopsis column

wm-paratext = A text that is not part of the story.
wm-paratext-more = A preface, a dedication, an afterword, a colophon: writing that belongs to the book but not to its body. Exported wherever you put it, and never counted in the manuscript's word count. Where it goes is up to you: conventions differ by country and publisher.
wm-paratext-folder = A folder for paratexts.
wm-paratext-folder-more = Somewhere to keep prefaces and afterwords so they do not clutter the binder. Organizing only: it carries a [synopsis](:wm-synopsis) but adds nothing to the exported book, not even its own name.

# ── Word / character targets ──────────────────────────────────────────────────
# A second cascade web beside the writing-model one. It doubles as the app's only
# explanation of how counting works, since there is no Help surface anywhere else.
goal-target = How long this piece is meant to be.
goal-target-more = Set a target on anything: a [scene](:wm-scene), a [chapter](:wm-chapter), a [part](:wm-part), the whole [book](:wm-book). Each one is its own number and stands on its own, so setting a target on a chapter never changes its book's, and a book's target is never a sum of the chapters inside it. What does add up is the writing: a container's [progress](:goal-progress) is everything written beneath it. Leave a target at zero and it simply is not set. The number is counted in [words or characters](:goal-unit), whichever this project uses.
goal-unit = Whether this project counts in words or in characters.
goal-unit-more = A project-wide choice, because a length unit belongs to the manuscript and its market rather than to one section: German and French publishing measure in signs, Japanese in 400-character sheets, Chinese by the thousand characters, most English-language publishing in words. Both numbers are kept side by side and neither is ever converted into the other, so switching the unit points every [target](:goal-target) at the other figure and switching back restores the first. What it does not do is convert anything, so targets already entered will need updating by hand.
goal-progress = How much is written, against the target.
goal-progress-explained = A piece with prose of its own is measured against its own length; a container against [everything written beneath it](:goal-manuscript-words). The bar warms from red through amber to green as the writing comes in, and changes again once the piece has run well past what was planned, which is worth noticing when a chapter has a length to keep to.
goal-manuscript-words = What counts as the manuscript.
goal-manuscript-words-more = Only prose that would actually be exported: [scenes](:wm-scene) and [chapters](:wm-chapter) carrying their own text. A [note](:wm-note) or a [paratext](:wm-paratext) is not part of the manuscript and is never counted toward one. Anything in the trash drops out, and so does anything [left out of the export](:goal-exportable).
goal-exportable = This row is left out of the export.
goal-exportable-more = Its own length still shows, because the writing is still there, but it counts toward no total, exactly as it will appear in no exported book. Leaving a container out does not leave the pieces inside it out: each row answers for itself, which is what the "Apply to children" button beside the switch is for.
goal-distribute = Share this target out across what is inside.
goal-distribute-more = Splits a [book's](:wm-book) or [part's](:wm-part) target across its immediate children so the pieces add up to it exactly. By default it fills only the ones with no target yet and leaves your own numbers alone. What it writes are ordinary [targets](:goal-target), free to edit afterwards and free to drift apart: this is one action, not a standing arrangement.
goal-subtree-total = What the targets inside add up to.
goal-subtree-total-more = A fact, not a target. This project never treats a container's target as the sum of the ones inside it, because that is how a folder's figure ends up moving on its own when a scene inside it is given a number. This line simply tells you what your own numbers come to, so you can compare it with the container's [target](:goal-target) yourself, or [share one out](:goal-distribute).
goal-milestone = A date to reach something by.
goal-milestone-more = Two kinds. One says the [book](:wm-book) should stand at a given length by a date, a waypoint on its own curve. The other says a particular [chapter](:wm-chapter) or [part](:wm-part) should have reached [its own target](:goal-target) by then, and reads that number live rather than keeping a copy of it, so editing the target updates the milestone too. Both appear on the book's [pace plan](:pace-plan).
pace-plan = The book's writing schedule.
pace-plan-more = A [target](:goal-target) and a deadline, with the days you write on, turned into how much a day needs. It reads the book's own target, so setting it here and setting it in the Inspector are the same number in two places. Waypoints along the way are [milestones](:goal-milestone).

# ── Feature concepts ───────────────────────────────────────────────────────────
# A third cascade web, beside the writing-model one and the word/character-target
# one above it. Each entry here teaches one project feature that otherwise has no
# page of its own: tags, the per-row label, point of view, epigraphs, footnotes,
# chapter mode, comments, backups, versions, trash, spell-check, search and
# replace, note templates, text replacement, smart punctuation, export styles and
# the round-trip markers an export can carry. Registered the same way as the two
# webs above, and cross-linking into them wherever a concept genuinely cites one.
concept-tag = A color-coded label you can put on any item, reusable across the whole project.
concept-tag-more = Tag an item to mark it: a status, a location, a thread you are tracking. The same tag can sit on any number of items, and one item can carry several. Turn on [find in prose](:wm-find-in-prose) for a tag that names something in your book, a character or a place, and Skribisto starts looking for it in your prose.
concept-label = A short note you write under an item's title, for your own eyes.
concept-label-more = Not a [tag](:concept-tag): a label belongs to one item alone, plain text with no color or catalog behind it, something like "1st plot point" or "needs a name". Set it from the row's context menu, or edit it inline in the Overview's Label column; it shows as a small subtitle under the title, in the outline and in the stream alike.
concept-point-of-view = Whose eyes a scene is narrated through: one or more of your story's cast.
concept-point-of-view-more = Set in the Inspector, beside the cast. Choosing someone not yet in the cast adds them there too. More than one viewpoint is a legal choice, useful for a shared scene or a change of eyes mid-book, not something Skribisto blocks. Candidates come from the items a [find in prose](:wm-find-in-prose) tag puts in your story bible.
concept-epigraph = A quotation set at the head of a part or a chapter.
concept-epigraph-more = Its own field, separate from the [chapter](:wm-chapter)'s or the [part](:wm-part)'s own prose; a [scene](:wm-scene) or a [note](:wm-note) carries none. Write one as an ordinary quotation; write several and each becomes its own blockquote, printed as its own epigraph. Its words are never counted as manuscript words.
concept-footnote = A citation marked in the prose, printing as a numbered note.
concept-footnote-more = The reference is a single mark inside your words, so it moves and deletes exactly as the words around it do. Its number is never stored: Skribisto works it out fresh from where every reference sits in the whole manuscript, so inserting one renumbers every note after it. A footnote's own words are counted apart from the manuscript, never folded into it.
concept-chapter-mode = Whether a new chapter is a folder of scenes or a single flat row.
concept-chapter-mode-more = Set in Settings, under Structure. You write directly into a [chapter](:wm-chapter) either way; the only difference is whether it can also contain scenes beneath it. Both encodings compile to the same book, and you may mix them freely in one project. Promote converts an existing chapter between the two without losing a word.
concept-comment = A note anchored to an exact stretch of prose, shown in the margin.
concept-comment-more = It remembers the words it points at and what surrounds them, not a fixed position, so it survives ordinary editing and a full reload. Reply underneath to keep a thread going. If the exact quote can no longer be found, the comment says so rather than silently drifting to the wrong sentence.
concept-backup = A point-in-time copy of the whole project, kept beside it.
concept-backup-more = Taken on a schedule you choose: on open, on close, on an interval, or any mix, with old copies pruned by age or by count. Opening one loads it in its own window: you can look and even edit, but only Save As or Restore keeps anything. Different from a [version](:concept-version), which tracks one row's own past.
concept-version = One row's own past: every change to its text or synopsis.
concept-version-more = Built from the project's own save history and its backups together, one entry per change rather than one per file. Pick an entry to see what changed and restore only that row's text to it; the rest of the project is untouched, and a safety copy is taken first. Different from a [backup](:concept-backup), which is a copy of everything.
concept-trash = A trashed item is hidden, not moved: it stays exactly where it was.
concept-trash-more = Trashing only flips one flag: the row keeps its place in the binder, marked inactive rather than removed. Restore it and it reappears in that exact spot, with no re-filing to do. Empty Trash removes trashed rows for good, and each of these is one undoable step, like any other project edit.
concept-spellcheck = Underlines words no installed dictionary recognizes, per project language.
concept-spellcheck-more = Each project has its own working language or languages; an item can override them for a scene written in another tongue. Add a word Skribisto does not know from the editor's context menu, into a personal dictionary that carries over to every project. Turning spell-check off stops it everywhere, until you turn it back on.
concept-search-replace = Finds a word or phrase everywhere in the project, not only the open document.
concept-search-replace-more = Looks through every scene, note, title, synopsis and label, and through comments and footnotes too. Case, whole-word matching and accent matching are three separate switches. Replace All rewrites every reviewed match in one pass and one undo step, so Ctrl+Z takes back the whole batch, never only the last change.
concept-note-template = A reusable piece of writing you insert into anything you are working on.
concept-note-template-more = A blank character sheet, a location profile, a beat sheet: write it once and drop it in wherever you need it, in any item, not only a note. Start from a built-in preset, import a .md or .djot file, or write something and choose Document ▸ Save as template. Stored in the project, so a co-author who opens it gets the same set.
concept-text-replacement = Your own shorthand, expanded automatically as you type.
concept-text-replacement-more = Set a trigger and its replacement, "btw" for "by the way", and it expands the moment you type a space or punctuation after it. Case follows what you typed: capitalize the trigger and the expansion capitalizes too. A Backspace right after an expansion undoes only that one substitution. Switched on per project, distinct from [smart punctuation](:concept-smart-punctuation).
concept-smart-punctuation = Locale-aware typography applied automatically as you type: quotes, dashes, ellipses.
concept-smart-punctuation-more = Curls straight quotes into typographic ones, turns -- into an en dash and --- into an em dash, turns ... into an ellipsis, and, where you turn it on, opens a paragraph typed as "- " with a dialogue dash. Set per project, and travels inside the .skrib so a co-author writes with the same rules. Distinct from [your own rules](:concept-text-replacement), which you write yourself.
concept-export-style = A named bundle of export choices: headings, spacing, what gets included.
concept-export-style-more = One style, reused across every export whatever format you choose. The built-in styles are read-only; duplicate one to get a starting point you can actually change. Among its choices is whether to include [round-trip markers](:concept-round-trip-marks), for a file bound for an editor and back.
concept-round-trip-marks = Invisible bookmarks that let an edited DOCX or ODT come home recognized.
concept-round-trip-marks-more = Written only into the two formats an editor can send back, DOCX and ODT; every other export carries none. A bookmark, not a custom attribute: LibreOffice strips custom attributes on save but leaves a bookmark intact. Turned on per [export style](:concept-export-style), under "Sending to an editor".
concept-status = Where an item stands in your own workflow: one stage at a time, moving forward.
concept-status-more = Not a [tag](:concept-tag): an item has exactly one status, the stages are ordered, and the project owns the list, so you can rename, reorder or add to it whenever you like. An asterisk beside a chapter or book means the parts below it do not all agree with it. Set it from the Inspector, a card, a stream row, or here.
