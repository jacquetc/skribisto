#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""
Tests for translation_report_triage.py.

Run with `python3 -m unittest discover -s tools -p 'test_*.py'`. The GitHub API is
a fake that records what the script asks of it; nothing touches the network.
"""

from __future__ import annotations

import unittest
from pathlib import Path

import translation_report_triage as tr

FIXTURE = Path(__file__).resolve().parent / "fixtures" / "translation_report_block.txt"
LOCALES = tr.supported_locales()
REPO = "example/skribisto"


def body_with(block: str) -> str:
    """An issue body as the form renders it: the block inside a text code block."""
    return f"### Language\n\nfr-FR\n\n### Report data\n\n```text\n{block}\n```\n"


def fixture_block() -> str:
    return FIXTURE.read_text(encoding="utf-8").strip()


def replace(block: str, key: str, value: str) -> str:
    lines = []
    for line in block.splitlines():
        if line.startswith(f"{key}:"):
            line = f"{key}: {value}" if value else f"{key}:"
        lines.append(line)
    return "\n".join(lines)


FR_MAIN_V331 = """\
menu-edit-undo = &Annuler
menu-edit-undo-target = A&nnuler { $target }
dialog-cancel = Annuler
undo-target-trash = la mise à la corbeille
"""
FR_MAIN_DEV = """\
menu-edit-undo = &Annuler
menu-edit-undo-target = &Défaire { $target }
undo-target-trash = la mise à la corbeille
"""
EN_MAIN = """\
menu-edit-undo = &Undo
menu-edit-undo-target = &Undo { $target }
"""


class FakeApi(tr.Api):
    def __init__(self) -> None:
        super().__init__("token", REPO)
        self.tags = {"v3.3.1"}
        self.framework_tags = {"v0.26.0"}
        self.commits: dict[str, str] = {}
        self.compare: dict[str, str] = {}
        self.files: dict[tuple[str, str, str], str] = {
            (REPO, "crates/teksilo_ui/locales/fr-FR/main.ftl", "v3.3.1"): FR_MAIN_V331,
            (REPO, "crates/teksilo_ui/locales/fr-FR/main.ftl", "dev"): FR_MAIN_DEV,
            (REPO, "crates/teksilo_ui/locales/en-US/main.ftl", "v3.3.1"): EN_MAIN,
        }
        self.existing_labels = {"translation", "needs-triage"}
        self.created: list[str] = []
        self.added: list[list[str]] = []
        self.issues: list[dict] = []
        self.posted: list[str] = []
        self.edited: list[tuple[int, str]] = []
        self.existing_comments: list[dict] = []

    def tag_exists(self, tag, repo=None):
        return tag in (self.framework_tags if repo == tr.FRAMEWORK_REPO else self.tags)

    def commit_sha(self, sha):
        return self.commits.get(sha)

    def compare_status(self, base, head):
        return self.compare.get(base)

    def file_text(self, path, ref, repo=None):
        return self.files.get((repo or REPO, path, ref))

    def labels(self):
        return set(self.existing_labels)

    def create_label(self, name, colour):
        self.created.append(name)
        self.existing_labels.add(name)

    def add_labels(self, number, labels):
        self.added.append(list(labels))

    def open_translation_issues(self):
        return list(self.issues)

    def comments(self, number):
        return list(self.existing_comments)

    def post_comment(self, number, body):
        self.posted.append(body)

    def edit_comment(self, comment_id, body):
        self.edited.append((comment_id, body))


def event(body: str, number: int = 7) -> dict:
    return {"action": "opened", "issue": {"number": number, "body": body}}


class TestBlock(unittest.TestCase):
    def test_the_fixture_the_application_writes_parses(self):
        block = tr.parse_block(body_with(fixture_block()), LOCALES)
        self.assertIsNotNone(block)
        self.assertEqual(block.locale, "fr-FR")
        self.assertEqual(block.catalogue, "app")
        self.assertEqual(block.file, "main.ftl")
        self.assertEqual(block.line, 3080)
        self.assertEqual(block.message, "menu-edit-undo-target")
        self.assertEqual(block.composed, "undo-target-trash")
        self.assertEqual(block.also, [])
        self.assertFalse(block.upstream)
        self.assertEqual(block.kind, "wrong-meaning")
        self.assertEqual(block.framework, "0.26.0")

    def test_a_body_without_the_marker_has_no_block(self):
        self.assertIsNone(tr.parse_block("A bug report about saving.", LOCALES))

    def test_a_block_pasted_into_a_blank_issue_parses(self):
        pasted = "I could not open the browser, so here it is:\n\n" + fixture_block() + "\n\nThanks!"
        self.assertIsNotNone(tr.parse_block(pasted, LOCALES))

    def test_a_report_of_a_text_not_found_is_valid(self):
        block = fixture_block()
        for key in ("catalogue", "file", "line", "message", "composed"):
            block = replace(block, key, "")
        block = replace(block, "area", "unknown")
        parsed = tr.parse_block(block, LOCALES)
        self.assertIsNotNone(parsed)
        self.assertFalse(parsed.found)

    def test_a_bare_hash_version_is_accepted(self):
        block = replace(fixture_block(), "version", "5c62c8cac")
        self.assertEqual(tr.parse_block(block, LOCALES).version, "5c62c8cac")

    def test_hostile_values_are_refused(self):
        cases = [
            ("locale", "x\nkind: other"),
            ("locale", "xx-YY"),
            ("message", "menu-edit-undo target"),
            ("message", "../etc"),
            ("file", "../../.github/workflows/ci.yml"),
            ("file", "fr-FR/main.ftl"),
            ("version", "3.3.1; rm -rf /"),
            ("kind", "wrong meaning"),
            ("line", "0"),
            ("upstream", "yes"),
        ]
        for key, value in cases:
            with self.subTest(key=key, value=value):
                with self.assertRaises(tr.InvalidBlock):
                    tr.parse_block(replace(fixture_block(), key, value), LOCALES)
        with self.assertRaises(tr.InvalidBlock):
            tr.parse_block(fixture_block() + "\nkind: other", LOCALES)

    def test_a_newline_in_a_value_ends_the_block_rather_than_adding_a_key(self):
        # The application never writes one; a hand-edited body might.
        block = fixture_block().replace("locale: fr-FR", "locale: fr-FR\nnot a key line")
        with self.assertRaises(tr.InvalidBlock):
            tr.parse_block(block, LOCALES)

    def test_only_a_framework_string_goes_upstream(self):
        with self.assertRaises(tr.InvalidBlock):
            tr.parse_block(replace(fixture_block(), "upstream", "teksilo"), LOCALES)


class TestLabels(unittest.TestCase):
    def test_a_valid_block_gets_every_family(self):
        block = tr.parse_block(fixture_block(), LOCALES)
        self.assertEqual(
            tr.labels_for(block),
            ["translation", "lang: fr-FR", "kind: wrong-meaning", "area: interface"],
        )
        self.assertEqual(tr.labels_for(None), ["needs-triage"])

    def test_each_family_has_its_colour(self):
        self.assertEqual(tr.label_colour("lang: ja-JP"), tr.LABEL_COLOURS["lang:"])
        self.assertEqual(tr.label_colour("upstream: teksilo"), tr.LABEL_COLOURS["upstream: teksilo"])


class TestRef(unittest.TestCase):
    def block(self, version: str) -> tr.Block:
        return tr.parse_block(replace(fixture_block(), "version", version), LOCALES)

    def test_a_release_is_quoted_at_its_tag(self):
        self.assertEqual(tr.choose_ref(FakeApi(), self.block("3.3.1")), "v3.3.1")

    def test_a_commit_on_dev_is_quoted_at_that_commit(self):
        api = FakeApi()
        full = "5c62c8cac" + "0" * 31
        api.commits["5c62c8cac"] = full
        api.compare[full] = "behind"
        self.assertEqual(tr.choose_ref(api, self.block("3.3.1+5c62c8cac")), full)

    def test_a_fork_commit_is_never_quoted(self):
        api = FakeApi()
        full = "abcdef1" + "0" * 33
        api.commits["abcdef1"] = full
        api.compare[full] = "diverged"
        self.assertEqual(tr.choose_ref(api, self.block("3.3.1+abcdef1")), "v3.3.1")

    def test_an_unknown_commit_falls_back_to_the_crate_tag_then_dev(self):
        api = FakeApi()
        self.assertEqual(tr.choose_ref(api, self.block("deadbeef")), "v3.3.1")
        api.tags.clear()
        self.assertEqual(tr.choose_ref(api, self.block("deadbeef")), "dev")


class TestQuote(unittest.TestCase):
    def test_a_fence_outlasts_the_backticks_inside(self):
        quoted = tr.quote("a ```` b")
        self.assertTrue(quoted.startswith("`````text\n"))
        self.assertTrue(quoted.endswith("\n`````"))

    def test_a_mention_stays_inside_the_code_block(self):
        quoted = tr.quote("@everyone look")
        self.assertIn("```text\n@everyone look\n```", quoted)


class TestTriage(unittest.TestCase):
    def test_a_report_is_labelled_located_and_commented(self):
        api = FakeApi()
        summary = tr.triage(api, event(body_with(replace(fixture_block(), "version", "3.3.1"))), LOCALES)
        self.assertIn("#7", summary)
        self.assertEqual(
            api.added,
            [["translation", "lang: fr-FR", "kind: wrong-meaning", "area: interface"]],
        )
        self.assertEqual(set(api.created), {"lang: fr-FR", "kind: wrong-meaning", "area: interface"})
        self.assertEqual(len(api.posted), 1)
        comment = api.posted[0]
        self.assertIn(tr.COMMENT_MARKER, comment)
        self.assertIn(
            f"https://github.com/{REPO}/blob/v3.3.1/crates/teksilo_ui/locales/fr-FR/main.ftl#L2",
            comment,
        )
        self.assertIn("A&nnuler { $target }", comment)
        self.assertIn("already changed", comment)
        self.assertIn("&Défaire { $target }", comment)
        self.assertIn("&Undo { $target }", comment)
        self.assertIn("Composed with `undo-target-trash`", comment)

    def test_a_second_run_edits_its_comment_in_place(self):
        api = FakeApi()
        api.existing_comments = [
            {"id": 99, "user": {"login": "github-actions[bot]"}, "body": tr.COMMENT_MARKER + "\nold"}
        ]
        tr.triage(api, event(body_with(fixture_block())), LOCALES)
        self.assertEqual(api.posted, [])
        self.assertEqual(api.edited[0][0], 99)

    def test_a_comment_by_someone_else_is_never_edited(self):
        api = FakeApi()
        api.existing_comments = [
            {"id": 5, "user": {"login": "mallory"}, "body": tr.COMMENT_MARKER}
        ]
        tr.triage(api, event(body_with(fixture_block())), LOCALES)
        self.assertEqual(api.edited, [])
        self.assertEqual(len(api.posted), 1)

    def test_an_invalid_block_only_needs_triage(self):
        api = FakeApi()
        summary = tr.triage(api, event(body_with(replace(fixture_block(), "kind", "nonsense"))), LOCALES)
        self.assertIn("invalid", summary)
        self.assertEqual(api.added, [["needs-triage"]])
        self.assertEqual(api.posted, [])

    def test_a_body_without_a_block_is_left_alone(self):
        api = FakeApi()
        tr.triage(api, event("Just a question."), LOCALES)
        self.assertEqual(api.added, [])
        self.assertEqual(api.posted, [])

    def test_a_duplicate_is_flagged(self):
        api = FakeApi()
        api.issues = [
            {"number": 3, "body": body_with(fixture_block())},
            {"number": 4, "body": body_with(replace(fixture_block(), "locale", "de-DE"))},
            {"number": 7, "body": body_with(fixture_block())},
            {"number": 5, "body": "no block"},
        ]
        tr.triage(api, event(body_with(fixture_block())), LOCALES)
        self.assertIn("possible duplicate", api.added[0])
        self.assertIn("Possible duplicate of #3", api.posted[0])
        self.assertNotIn("#4", api.posted[0])

    def test_a_framework_string_is_quoted_from_teksilo(self):
        api = FakeApi()
        api.files[(tr.FRAMEWORK_REPO, "crates/teksilo-widgets/locales/fr-FR.ftl", "v0.26.0")] = (
            "messagebox-btn-save = Enregistrer\n"
        )
        api.files[(tr.FRAMEWORK_REPO, "crates/teksilo-widgets/locales/en-US.ftl", "v0.26.0")] = (
            "messagebox-btn-save = Save\n"
        )
        block = fixture_block()
        for key, value in {
            "catalogue": "framework",
            "file": "fr-FR.ftl",
            "line": "1",
            "message": "messagebox-btn-save",
            "composed": "",
            "upstream": "teksilo",
        }.items():
            block = replace(block, key, value)
        tr.triage(api, event(body_with(block)), LOCALES)
        self.assertIn("upstream: teksilo", api.added[0])
        comment = api.posted[0]
        self.assertIn(
            f"https://github.com/{tr.FRAMEWORK_REPO}/blob/v0.26.0/crates/teksilo-widgets/locales/fr-FR.ftl#L1",
            comment,
        )
        self.assertIn("Save", comment)

    def test_a_help_paragraph_is_quoted_by_its_line(self):
        api = FakeApi()
        page = "## Titre\n\nPremière phrase.\nSuite.\n\nAutre paragraphe.\n"
        api.files[(REPO, "crates/teksilo_ui/help/fr-FR/getting-started.djot", "v3.3.1")] = page
        block = fixture_block()
        for key, value in {
            "catalogue": "help",
            "area": "help",
            "file": "getting-started.djot",
            "line": "3",
            "message": "help-getting-started",
            "composed": "",
        }.items():
            block = replace(block, key, value)
        tr.triage(api, event(body_with(block)), LOCALES)
        self.assertIn("Première phrase.\nSuite.", api.posted[0])
        self.assertNotIn("Autre paragraphe", api.posted[0])


if __name__ == "__main__":
    unittest.main()
