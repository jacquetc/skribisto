# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Words the exporter writes INTO the book, in the book's own language.
#
# Not the app's interface: these files are chosen by the language of the
# manuscript (a scene's language, or the export style's fixed heading
# language), never by the language the app is shown in. A French writer using
# an English interface exports "Chapitre 3". See
# crates/skribisto_compiler/src/manuscript.rs.
#
# One file per language, named by its primary subtag (fr, pt, nb). A language
# with no file, or a message missing from one, falls back to this file.
#
# $n is the number ALREADY rendered in the export style's digits (3, or ٣ in
# an Eastern Arabic style), so it is text: put it where your language puts it
# ("Chapter 3", "第3章", "3. fejezet").

# A numbered structural heading.
heading-book = Book { $n }
heading-part = Part { $n }
heading-chapter = Chapter { $n }

# The bare word, for an untitled and unnumbered row in the binder.
heading-word-book = Book
heading-word-part = Part
heading-word-chapter = Chapter

# The line under the title on a title page. $author is the writer's name as
# they typed it. Languages that print the name alone write just { $author }.
byline = by { $author }

# The title page's word-count estimate (Shunn: rounded, never exact).
# $count is the rounded number already grouped and in the style's digits. It
# is always a multiple of a hundred, so one plural form fits every value.
word-count-note = about { $count } words

# What separates groups of three digits in $count: "90,000", "90 000".
# A quoted string so a space or a no-break space survives.
digit-group-separator = { "," }
