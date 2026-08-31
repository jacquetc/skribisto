#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""
Tests for check_locales.py.

Run with `python3 -m unittest discover -s tools -p 'test_*.py'` (no third-party
dependency, so CI needs nothing but a Python interpreter).

Every check gets a positive case (it fires on a broken locale) and, where a
false positive is plausible, a negative case (it stays quiet on a correct one).
A checker that only ever reports zero is indistinguishable from a checker that
does nothing, so the negative cases matter as much as the positive ones.
"""

from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

import check_locales as cl

BASE = """\
hello = Hello
menu-file = &File
word-count = { $count ->
    [one] { $count } word
   *[other] { $count } words
}
greeting = Hello, { $name }!
multi =
    First line
    second line
section-title = Backup & Sync
"""

TRANSLATION = """\
hello = Bonjour
menu-file = &Fichier
word-count = { $count ->
    [one] { $count } mot
   *[other] { $count } mots
}
greeting = Bonjour, { $name } !
multi =
    Première ligne
    deuxième ligne
section-title = Sauvegarde et synchronisation
"""


def write_locales(base: str, translation: str) -> tempfile.TemporaryDirectory:
    """Build a throwaway locales/ tree with an en-US and an fr-FR locale."""
    tmp = tempfile.TemporaryDirectory()
    root = Path(tmp.name)
    (root / "en-US").mkdir()
    (root / "fr-FR").mkdir()
    (root / "en-US" / "main.ftl").write_text(base, encoding="utf-8")
    (root / "fr-FR" / "main.ftl").write_text(translation, encoding="utf-8")
    return tmp


def run_checks(
    base: str,
    translation: str,
    untranslated: bool = False,
    min_words: int = cl.DEFAULT_UNTRANSLATED_MIN_WORDS,
):
    """Parse both locales and return every problem, as (check, key) pairs."""
    with write_locales(base, translation) as name:
        discovered = cl.discover_locales(Path(name))
        base_parsed = cl.load_locale("en-US", discovered["en-US"])
        fr = cl.load_locale("fr-FR", discovered["fr-FR"])
        problems = list(base_parsed.problems) + list(fr.problems)
        problems += cl.check_locale_internals(base_parsed, True)
        problems += cl.check_locale_internals(fr, True)
        problems += cl.check_against_base(fr, base_parsed, untranslated,
                                          min_words)
        return [(p.check, p.key) for p in problems]


class TestParser(unittest.TestCase):
    def test_parses_every_entry(self):
        with write_locales(BASE, TRANSLATION) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        self.assertEqual(
            set(parsed.entries),
            {"hello", "menu-file", "word-count", "greeting", "multi",
             "section-title"},
        )
        self.assertEqual(parsed.problems, [])

    def test_selector_closing_brace_at_column_zero_continues_the_entry(self):
        """The `}` closing a selector sits at column 0 in the real files.

        A parser that ended the entry at the first unindented line would treat
        it as junk and lose every variant.
        """
        with write_locales(BASE, TRANSLATION) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        entry = parsed.entries["word-count"]
        self.assertIn("[one]", entry.value)
        self.assertIn("*[other]", entry.value)
        self.assertEqual(entry.variables(), {"count"})

    def test_multiline_value_is_joined(self):
        with write_locales(BASE, TRANSLATION) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        # The empty segment left by `multi =` is stripped; the indented
        # continuation lines survive as the value.
        self.assertEqual(parsed.entries["multi"].value, "First line\nsecond line")

    def test_blank_line_inside_a_pattern_does_not_end_the_entry(self):
        source = "foo =\n    one\n\n    two\nbar = Bar\n"
        with write_locales(source, source) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        self.assertEqual(set(parsed.entries), {"foo", "bar"})
        self.assertIn("two", parsed.entries["foo"].value)
        self.assertEqual(parsed.problems, [])

    def test_braces_inside_string_literals_do_not_shift_depth(self):
        source = 'foo = { "{" } literal brace\nbar = Bar\n'
        with write_locales(source, source) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        self.assertEqual(set(parsed.entries), {"foo", "bar"})

    def test_attributes_are_captured(self):
        source = "foo = Value\n    .label = Label\n    .tip = Tip\n"
        with write_locales(source, source) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        self.assertEqual(set(parsed.entries["foo"].attributes), {"label", "tip"})

    def test_comments_do_not_become_entries(self):
        source = "# a comment = not an entry\n## group\nfoo = Foo\n"
        with write_locales(source, source) as name:
            parsed = cl.load_locale("en-US", [Path(name) / "en-US" / "main.ftl"])
        self.assertEqual(set(parsed.entries), {"foo"})
        self.assertEqual(parsed.problems, [])


class TestCleanTree(unittest.TestCase):
    def test_matching_locales_report_nothing(self):
        self.assertEqual(run_checks(BASE, TRANSLATION), [])

    def test_literal_ampersand_is_not_a_missing_mnemonic(self):
        """`Backup & Sync` -> `Sauvegarde et synchronisation` is correct.

        A mnemonic is glued to its letter; an ampersand followed by a space is
        prose, and French drops it entirely.
        """
        self.assertNotIn(
            "mnemonic", [check for check, _ in run_checks(BASE, TRANSLATION)]
        )


class TestChecks(unittest.TestCase):
    def test_missing_key(self):
        translation = TRANSLATION.replace("hello = Bonjour\n", "")
        self.assertIn(("missing", "hello"), run_checks(BASE, translation))

    def test_extra_key(self):
        translation = TRANSLATION + "orphan = Orphelin\n"
        self.assertIn(("extra", "orphan"), run_checks(BASE, translation))

    def test_duplicate_key(self):
        translation = TRANSLATION + "hello = Salut\n"
        self.assertIn(("duplicate", "hello"), run_checks(BASE, translation))

    def test_renamed_variable(self):
        translation = TRANSLATION.replace("{ $name }", "{ $nom }")
        problems = run_checks(BASE, translation)
        self.assertIn(("placeholder", "greeting"), problems)

    def test_dropped_variable(self):
        translation = TRANSLATION.replace("Bonjour, { $name } !", "Bonjour !")
        self.assertIn(("placeholder", "greeting"), run_checks(BASE, translation))

    def test_selector_without_default_variant(self):
        translation = TRANSLATION.replace("*[other]", " [other]")
        self.assertIn(("selector", "word-count"), run_checks(BASE, translation))

    def test_empty_value(self):
        translation = TRANSLATION.replace("hello = Bonjour", "hello =")
        self.assertIn(("empty", "hello"), run_checks(BASE, translation))

    def test_unknown_message_reference(self):
        base = BASE + "refers = See { hello }\n"
        translation = TRANSLATION + "refers = Voir { bonjour }\n"
        self.assertIn(("reference", "refers"), run_checks(base, translation))

    def test_known_message_reference_is_quiet(self):
        base = BASE + "refers = See { hello }\n"
        translation = TRANSLATION + "refers = Voir { hello }\n"
        self.assertNotIn(
            "reference", [check for check, _ in run_checks(base, translation)]
        )

    def test_dangling_tooltip_link(self):
        base = BASE + "wm-a = See [b](:wm-b).\nwm-b = Bee.\n"
        translation = TRANSLATION + "wm-a = Voir [b](:wm-b).\n"
        problems = run_checks(base, translation)
        self.assertIn(("tooltip-link", "wm-a"), problems)

    def test_missing_mnemonic(self):
        translation = TRANSLATION.replace("menu-file = &Fichier",
                                          "menu-file = Fichier")
        self.assertIn(("mnemonic", "menu-file"), run_checks(BASE, translation))

    def test_two_mnemonics_in_one_label(self):
        translation = TRANSLATION.replace("menu-file = &Fichier",
                                          "menu-file = &Fi&chier")
        self.assertIn(("mnemonic", "menu-file"), run_checks(BASE, translation))

    def test_mnemonic_mixed_with_unescaped_literal_ampersand(self):
        base = BASE + "menu-x = &Copy & Paste\n"
        translation = TRANSLATION + "menu-x = &Copier & Coller\n"
        self.assertIn(("mnemonic", "menu-x"), run_checks(base, translation))

    def test_escaped_ampersand_is_not_a_mnemonic(self):
        base = BASE + "menu-x = Copy && Paste\n"
        translation = TRANSLATION + "menu-x = Copier && Coller\n"
        self.assertNotIn(
            "mnemonic", [check for check, _ in run_checks(base, translation)]
        )

    def test_attribute_mismatch(self):
        base = BASE + "btn = Go\n    .tip = Tip\n"
        translation = TRANSLATION + "btn = Allez\n"
        self.assertIn(("attribute", "btn"), run_checks(base, translation))

    def test_non_cldr_plural_category(self):
        """French has no `two` category — CLDR gives it one/many/other."""
        translation = TRANSLATION.replace("    [one] { $count } mot",
                                          "    [two] { $count } mot")
        self.assertIn(("plural", "word-count"), run_checks(BASE, translation))

    def test_numeric_variant_is_allowed(self):
        translation = TRANSLATION.replace("    [one] { $count } mot",
                                          "    [0] aucun mot")
        self.assertNotIn(
            "plural", [check for check, _ in run_checks(BASE, translation)]
        )

    def test_untranslated_is_opt_in(self):
        base = BASE + "stub = This sentence was never translated.\n"
        translation = TRANSLATION + "stub = This sentence was never translated.\n"
        self.assertNotIn(
            "untranslated", [c for c, _ in run_checks(base, translation)]
        )
        self.assertIn(
            ("untranslated", "stub"),
            run_checks(base, translation, untranslated=True),
        )

    def test_untranslated_ignores_short_cognates(self):
        """"Note", "Format", "PDF" are identical in French on purpose.

        Flagging those buries the one string that is genuinely a stub, so the
        check only fires past a word-count threshold.
        """
        base = BASE + "fmt = Format\nnote = Note\npdf = PDF\n"
        translation = TRANSLATION + "fmt = Format\nnote = Note\npdf = PDF\n"
        self.assertNotIn(
            "untranslated",
            [c for c, _ in run_checks(base, translation, untranslated=True)],
        )

    def test_untranslated_threshold_of_one_reports_cognates_too(self):
        base = BASE + "note = Note\n"
        translation = TRANSLATION + "note = Note\n"
        self.assertIn(
            ("untranslated", "note"),
            run_checks(base, translation, untranslated=True, min_words=1),
        )

    def test_untranslated_ignores_placeables_and_symbols(self):
        """`×{ $count }` and `~{ $size }` have no translatable words at all."""
        base = BASE + "occ = ×{ $count }\nsize = ~{ $size }\nhalf = 1½\n"
        translation = TRANSLATION + "occ = ×{ $count }\nsize = ~{ $size }\nhalf = 1½\n"
        self.assertNotIn(
            "untranslated",
            [c for c, _ in run_checks(base, translation, untranslated=True,
                                      min_words=1)],
        )

    def test_untranslated_ignores_file_extensions(self):
        """"&Plume Creator (.plume)…" is a product name, not a stub."""
        base = BASE + "menu-import = &Plume Creator (.plume)…\n"
        translation = TRANSLATION + "menu-import = &Plume Creator (.plume)…\n"
        self.assertNotIn(
            "untranslated",
            [c for c, _ in run_checks(base, translation, untranslated=True)],
        )

    def test_syntax_junk(self):
        translation = TRANSLATION + "this is not fluent\n"
        self.assertIn("syntax", [check for check, _ in run_checks(BASE,
                                                                  translation)])

    def test_placement_differs(self):
        with write_locales(BASE, TRANSLATION) as name:
            root = Path(name)
            # Move `hello` into a second topic file on the fr-FR side only.
            (root / "fr-FR" / "main.ftl").write_text(
                TRANSLATION.replace("hello = Bonjour\n", ""), encoding="utf-8"
            )
            (root / "fr-FR" / "extra.ftl").write_text(
                "hello = Bonjour\n", encoding="utf-8"
            )
            discovered = cl.discover_locales(root)
            base_parsed = cl.load_locale("en-US", discovered["en-US"])
            fr = cl.load_locale("fr-FR", discovered["fr-FR"])
            problems = cl.check_against_base(fr, base_parsed, False)
        self.assertIn(("placement", "hello"), [(p.check, p.key) for p in problems])


class TestRegistration(unittest.TestCase):
    def test_unregistered_file_is_reported(self):
        with write_locales(BASE, TRANSLATION) as name:
            root = Path(name)
            (root / "en-US" / "tooltips.ftl").write_text("wm-a = A\n",
                                                         encoding="utf-8")
            loader = root / "main.rs"
            loader.write_text(
                'include_str!("../locales/en-US/main.ftl");\n'
                'include_str!("../locales/fr-FR/main.ftl");\n',
                encoding="utf-8",
            )
            problems = cl.check_registration(loader, cl.discover_locales(root))
        checks = [(p.check, p.locale) for p in problems]
        self.assertIn(("unregistered", "en-US"), checks)
        self.assertEqual(len(problems), 1, "only tooltips.ftl is unregistered")

    def test_fully_registered_tree_is_quiet(self):
        with write_locales(BASE, TRANSLATION) as name:
            root = Path(name)
            loader = root / "main.rs"
            loader.write_text(
                'include_str!("../locales/en-US/main.ftl");\n'
                'include_str!("../locales/fr-FR/main.ftl");\n',
                encoding="utf-8",
            )
            problems = cl.check_registration(loader, cl.discover_locales(root))
        self.assertEqual(problems, [])


class TestSeverity(unittest.TestCase):
    def test_error_checks_are_errors_and_the_rest_are_warnings(self):
        error = cl.Problem("fr-FR", "missing", "k", "m")
        warning = cl.Problem("fr-FR", "mnemonic", "k", "m")
        self.assertEqual(error.severity, "error")
        self.assertEqual(warning.severity, "warning")


class TestRealLocales(unittest.TestCase):
    """The shipped locales must stay clean — this is the CI gate in test form."""

    def test_repository_locales_have_no_errors(self):
        locales_dir = cl.REPO_ROOT / cl.DEFAULT_LOCALES_DIR
        if not locales_dir.is_dir():
            self.skipTest(f"no locales directory at {locales_dir}")
        discovered = cl.discover_locales(locales_dir)
        base = cl.load_locale(cl.DEFAULT_BASE_LOCALE,
                              discovered[cl.DEFAULT_BASE_LOCALE])
        problems = list(base.problems) + cl.check_locale_internals(base, True)
        for locale, files in discovered.items():
            if locale == cl.DEFAULT_BASE_LOCALE:
                continue
            parsed = cl.load_locale(locale, files)
            problems += parsed.problems
            problems += cl.check_locale_internals(parsed, True)
            problems += cl.check_against_base(parsed, base, False)
        errors = [f"{p.locale} {p.check} {p.key}: {p.message}"
                  for p in problems if p.severity == "error"]
        self.assertEqual(errors, [])


if __name__ == "__main__":
    unittest.main()
