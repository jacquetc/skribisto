#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""
Compare every translation locale against the source locale (en-US).

Skribisto's `tr!` macro validates keys against `en-US` *only* — a translation
locale is loaded at runtime and silently falls back per-key. That is exactly
the failure mode this script exists to catch: a key missing from `fr-FR`
renders in English with no compile error, a key that only exists in `fr-FR` is
dead weight nobody will ever see, and a `{ $variable }` renamed on one side
alone breaks the interpolation at runtime.

The locale set is discovered from the locales directory. Both layouts are
supported: one directory per locale holding several topic files
(`locales/fr-FR/main.ftl`, the current layout — all files in a locale are
merged, since that is how `compile_in` loads them), and one flat file per
locale (`locales/fr-FR.ftl`).

Checks, in two severities. Errors fail CI; warnings are reported and only fail
under `--strict`.

  ERRORS
    missing        key present in the base locale, absent from this one
    extra          key present here, absent from the base locale
    duplicate      key defined twice within one locale
    placeholder    the set of `{ $vars }` differs from the base entry
    selector       a `{ $x -> }` selector with no `*[default]` variant
    empty          key whose value is empty
    reference      `{ other-key }` / `{ -term }` pointing at a key that does
                   not exist in that locale
    tooltip-link   `[label](:key)` whose target key does not exist in that
                   locale (the wm-*-more tooltips cascade through these)
    unregistered   an .ftl file on disk the loader never compiles in (as a
                   literal `include_str!` or through `compile_in_locales!`),
                   so it is compiled into nothing
    syntax         a line that is not valid Fluent entry/continuation syntax

  WARNINGS
    attribute      the set of `.attributes` differs from the base entry
    mnemonic       the `&` menu mnemonic is present on one side only, or
                   declared more than once. A mnemonic is glued to the letter
                   it marks (`&File`); `X & Y` is prose and `&&` is a literal
                   ampersand, so neither counts.
    placement      key lives in a different .ftl file than it does in base
    plural         a variant key that is not a CLDR plural category for that
                   language
    untranslated   value byte-identical to the base value *and* several words
                   long, so probably a stub rather than a cognate (opt-in:
                   --check-untranslated, threshold --untranslated-min-words)
    unused         base key never referenced from Rust (opt-in: --check-unused;
                   a heuristic over `tr!(ident)` and "kebab-case" literals)

Usage:
    tools/check_locales.py                     # check, human-readable report
    tools/check_locales.py --format github     # CI annotations
    tools/check_locales.py --strict            # warnings fail too
    tools/check_locales.py --stubs fr-FR       # paste-ready stubs for missing
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent

DEFAULT_LOCALES_DIR = "crates/teksilo_ui/locales"
DEFAULT_BASE_LOCALE = "en-US"
DEFAULT_RUST_SRC = "crates/teksilo_ui/src"
DEFAULT_LOADER = "crates/teksilo_ui/src/startup.rs"

# --- Fluent syntax -----------------------------------------------------------

# A message (`foo = value`) or a term (`-foo = value`) at column 0.
ENTRY_RE = re.compile(r"^(-?[A-Za-z][A-Za-z0-9_-]*)[ \t]*=(.*)$")
# An attribute (`    .label = value`), indented under its entry.
ATTR_RE = re.compile(r"^[ \t]+\.([A-Za-z][A-Za-z0-9_-]*)[ \t]*=(.*)$")

VARIABLE_RE = re.compile(r"\$([A-Za-z][A-Za-z0-9_-]*)")
# Only a placeable whose *entire* content is an identifier is a reference.
# Anything richer (a selector, a function call) is deliberately not matched:
# precision matters more than recall for a check that fails the build.
REFERENCE_RE = re.compile(
    r"\{[ \t]*(-?[a-z][A-Za-z0-9_-]*)(?:\.[a-z][A-Za-z0-9_-]*)?[ \t]*\}"
)
VARIANT_RE = re.compile(r"^[ \t]*(\*?)[ \t]*\[[ \t]*([^\]]*?)[ \t]*\]", re.MULTILINE)
DEFAULT_VARIANT_RE = re.compile(r"^[ \t]*\*[ \t]*\[", re.MULTILINE)
# Rich-tooltip cross-link: `[label](:some-key)`, resolved by tooltip_registry.
TOOLTIP_LINK_RE = re.compile(r"\]\([ \t]*:([a-z][A-Za-z0-9_-]*)[ \t]*\)")
# A mnemonic marks the letter it precedes, so it is always glued to a
# non-space character: `&File`, `&New Work`. An ampersand followed by a space
# is prose — "Appearance & Behaviour", which French renders "Apparence et
# comportement" with no ampersand at all. Without that distinction every
# `X & Y` title would look like a dropped mnemonic. `&&` is the escape for a
# literal ampersand and is never a mnemonic.
MNEMONIC_RE = re.compile(r"(?<!&)&(?!&)(?=\S)")
# An unescaped literal ampersand (`&` followed by a space or end of string).
# Harmless in prose, but teksilo's parse_mnemonic marks whatever character
# follows the `&` — including a space — so in a menu label this silently eats
# the ampersand and binds a useless space mnemonic. Only flagged when the same
# label also carries a real mnemonic, which makes it unambiguously a mistake.
LOOSE_AMPERSAND_RE = re.compile(r"(?<!&)&(?!&)(?!\S)")

# CLDR plural categories per language subtag. A language that is absent is not
# checked (better silent than wrong); the point is to catch `[two]` in French,
# not to police every locale we might add later.
CLDR_CATEGORIES = {
    "ar": {"zero", "one", "two", "few", "many", "other"},
    "cs": {"one", "few", "many", "other"},
    "de": {"one", "other"},
    "en": {"one", "other"},
    "es": {"one", "many", "other"},
    "fr": {"one", "many", "other"},
    "it": {"one", "many", "other"},
    "ja": {"other"},
    "ko": {"other"},
    "nl": {"one", "other"},
    "pl": {"one", "few", "many", "other"},
    "pt": {"one", "many", "other"},
    "ru": {"one", "few", "many", "other"},
    "uk": {"one", "few", "many", "other"},
    "zh": {"other"},
}

# A value identical to the base one is usually fine, not a stub: French shares
# "Note", "Format", "Synopsis", "Destination", "Structure" with English, and
# "PDF", "GitHub", "LaTeX" are nobody's to translate. Word count is what
# separates those from a copy-pasted English sentence — a cognate is one word,
# a stub is a phrase. Counts alphabetic words only, ignoring placeables, so
# `×{ $count }` and `1½` score zero.
DEFAULT_UNTRANSLATED_MIN_WORDS = 3
PLACEABLE_RE = re.compile(r"\{[^{}]*\}")
# A file extension is not a translatable word: without this, the menu label
# "&Plume Creator (.plume)…" counts three and trips the threshold.
EXTENSION_RE = re.compile(r"\.[A-Za-z0-9]+")
WORD_RE = re.compile(r"[^\W\d_]{2,}", re.UNICODE)

# --- Loader (which .ftl files are compiled into the binary) ------------------

# The literal form: one `include_str!("relative/path.ftl")` per file.
INCLUDE_STR_RE = re.compile(r'include_str!\("([^"]+)"\)')
# The cross-product form. `compile_in_locales!` expands to one `include_str!`
# per (locale, file) pair, so no literal path appears in the source at all —
# reading only the literal form here is how every file on disk once reported
# itself unregistered while the app shipped all of them.
COMPILE_IN_LOCALES_RE = re.compile(
    r"compile_in_locales!\s*\(\s*"
    r"base\s*=\s*\"([^\"]*)\"\s*,\s*"
    r"locales\s*=\s*\[([^\]]*)\]\s*,\s*"
    r"files\s*=\s*\[([^\]]*)\]\s*,?\s*\)",
    re.DOTALL,
)
STRING_LITERAL_RE = re.compile(r'"([^"]*)"')

ERROR_CHECKS = {
    "missing",
    "extra",
    "duplicate",
    "placeholder",
    "selector",
    "empty",
    "reference",
    "tooltip-link",
    "unregistered",
    "syntax",
}


@dataclass
class Entry:
    """One parsed Fluent message or term."""

    name: str
    file: Path
    line: int
    value: str = ""
    attributes: dict[str, str] = field(default_factory=dict)
    # Line buffers, filled while parsing and folded into `value` /
    # `attributes` when the entry is closed.
    value_lines: list[str] = field(default_factory=list, repr=False)
    attr_lines: dict[str, list[str]] = field(default_factory=dict, repr=False)

    @property
    def full_text(self) -> str:
        """Value plus every attribute value — what the checks scan."""
        return "\n".join([self.value, *self.attributes.values()])

    def variables(self) -> set[str]:
        return set(VARIABLE_RE.findall(self.full_text))

    def references(self) -> set[str]:
        return set(REFERENCE_RE.findall(self.full_text))

    def tooltip_links(self) -> set[str]:
        return set(TOOLTIP_LINK_RE.findall(self.full_text))

    def mnemonics(self) -> int:
        return len(MNEMONIC_RE.findall(self.value))

    def loose_ampersands(self) -> int:
        return len(LOOSE_AMPERSAND_RE.findall(self.value))

    def word_count(self) -> int:
        """Alphabetic words in the value, ignoring placeables, digits and
        file extensions."""
        text = PLACEABLE_RE.sub(" ", self.value)
        return len(WORD_RE.findall(EXTENSION_RE.sub(" ", text)))

    def is_empty(self) -> bool:
        return not self.value.strip() and not any(
            v.strip() for v in self.attributes.values()
        )


@dataclass
class Problem:
    locale: str
    check: str
    key: str
    message: str
    file: Path | None = None
    line: int = 0

    @property
    def severity(self) -> str:
        return "error" if self.check in ERROR_CHECKS else "warning"


@dataclass
class ParsedLocale:
    locale: str
    entries: dict[str, Entry] = field(default_factory=dict)
    problems: list[Problem] = field(default_factory=list)
    files: list[Path] = field(default_factory=list)


# --- Parsing -----------------------------------------------------------------


def scan_braces(line: str, depth: int) -> int:
    """Advance the placeable nesting depth across one line.

    Braces inside a Fluent string literal (`{ "{" }`) are literal text, so the
    scan has to honour quoting — otherwise a string containing a brace would
    desynchronise every entry after it.
    """
    in_string = False
    escaped = False
    for ch in line:
        if in_string:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
        elif ch == "{":
            depth += 1
        elif ch == "}":
            depth = max(0, depth - 1)
    return depth


def parse_ftl(path: Path, locale: str, parsed: ParsedLocale) -> None:
    """Parse one .ftl file into `parsed`, recording duplicates and junk.

    This is a structural parser, not a validating one: it recovers entry names,
    line numbers, values and attributes. Nesting depth is tracked so that a
    selector's closing `}` at column 0 — which the source locale uses freely —
    reads as a continuation rather than as the end of the entry.
    """
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()

    current: Entry | None = None
    sink: list[str] | None = None  # where continuation lines accumulate
    current_attr: str | None = None
    pending_blanks: list[str] = []
    depth = 0

    def flush() -> None:
        nonlocal current, sink, current_attr
        if current is None:
            return
        # Fluent trims the whitespace around a pattern, so `foo = Bar` and
        # `foo =\n    Bar` carry the same value.
        current.value = "\n".join(current.value_lines).strip()
        for name, buf in current.attr_lines.items():
            current.attributes[name] = "\n".join(buf).strip()
        existing = parsed.entries.get(current.name)
        if existing is not None:
            parsed.problems.append(
                Problem(
                    locale,
                    "duplicate",
                    current.name,
                    f"defined again here; first defined at "
                    f"{rel(existing.file)}:{existing.line}",
                    current.file,
                    current.line,
                )
            )
        else:
            parsed.entries[current.name] = current
        current, sink, current_attr = None, None, None

    for index, raw in enumerate(lines, start=1):
        if depth == 0:
            if not raw.strip():
                # A blank line only ends the entry if no indented continuation
                # follows it; Fluent allows blank lines *inside* a pattern.
                pending_blanks.append("")
                depth = scan_braces(raw, depth)
                continue

            if raw.startswith("#"):
                flush()
                pending_blanks.clear()
                depth = scan_braces(raw, depth)
                continue

            entry_match = ENTRY_RE.match(raw)
            if entry_match and not raw[0].isspace():
                flush()
                pending_blanks.clear()
                name, first = entry_match.group(1), entry_match.group(2)
                current = Entry(name=name, file=path, line=index,
                                value_lines=[first])
                sink = current.value_lines
                current_attr = None
                depth = scan_braces(first, depth)
                continue

            attr_match = ATTR_RE.match(raw)
            if attr_match and current is not None:
                pending_blanks.clear()
                current_attr = attr_match.group(1)
                current.attr_lines[current_attr] = [attr_match.group(2)]
                sink = current.attr_lines[current_attr]
                depth = scan_braces(attr_match.group(2), depth)
                continue

            if raw[0].isspace() and sink is not None:
                sink.extend(pending_blanks)
                pending_blanks.clear()
                sink.append(raw.strip())
                depth = scan_braces(raw, depth)
                continue

            # Nothing matched at depth 0: this line is junk Fluent would reject.
            flush()
            pending_blanks.clear()
            parsed.problems.append(
                Problem(
                    locale,
                    "syntax",
                    "",
                    f"not a valid Fluent entry or continuation: {raw.strip()!r}",
                    path,
                    index,
                )
            )
            depth = scan_braces(raw, depth)
            continue

        # depth > 0: inside a placeable, every line continues the current value.
        if sink is not None:
            sink.extend(pending_blanks)
            sink.append(raw.strip())
        pending_blanks.clear()
        depth = scan_braces(raw, depth)

    flush()
    parsed.files.append(path)


def discover_locales(locales_dir: Path) -> dict[str, list[Path]]:
    """Map locale name -> its .ftl files, for either directory or flat layout."""
    found: dict[str, list[Path]] = {}
    for child in sorted(locales_dir.iterdir()):
        if child.is_dir():
            files = sorted(child.glob("*.ftl"))
            if files:
                found[child.name] = files
        elif child.suffix == ".ftl":
            found.setdefault(child.stem, []).append(child)
    return found


def load_locale(locale: str, files: list[Path]) -> ParsedLocale:
    parsed = ParsedLocale(locale=locale)
    for path in files:
        parse_ftl(path, locale, parsed)
    return parsed


# --- Checks ------------------------------------------------------------------


def selector_spans(text: str) -> list[str]:
    """Return the variant list of every `->` selector in `text`.

    Walks forward from each `->` until the brace that closes the placeable the
    selector lives in, so nested selectors each yield their own span.
    """
    spans: list[str] = []
    for match in re.finditer(r"->", text):
        start = match.end()
        depth = 0
        end = len(text)
        for i in range(start, len(text)):
            ch = text[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                if depth == 0:
                    end = i
                    break
                depth -= 1
        spans.append(text[start:end])
    return spans


def check_entry_pair(
    locale: str,
    base_locale: str,
    base: Entry,
    entry: Entry,
    problems: list[Problem],
) -> None:
    """Cross-check one translated entry against its base-locale counterpart."""
    base_vars, vars_ = base.variables(), entry.variables()
    if base_vars != vars_:
        missing = sorted(base_vars - vars_)
        extra = sorted(vars_ - base_vars)
        detail = []
        if missing:
            detail.append("missing " + ", ".join("$" + v for v in missing))
        if extra:
            detail.append("unexpected " + ", ".join("$" + v for v in extra))
        problems.append(
            Problem(
                locale,
                "placeholder",
                entry.name,
                "variables differ from en-US: " + "; ".join(detail),
                entry.file,
                entry.line,
            )
        )

    if set(base.attributes) != set(entry.attributes):
        missing = sorted(set(base.attributes) - set(entry.attributes))
        extra = sorted(set(entry.attributes) - set(base.attributes))
        detail = []
        if missing:
            detail.append("missing " + ", ".join("." + a for a in missing))
        if extra:
            detail.append("unexpected " + ", ".join("." + a for a in extra))
        problems.append(
            Problem(
                locale,
                "attribute",
                entry.name,
                "attributes differ from en-US: " + "; ".join(detail),
                entry.file,
                entry.line,
            )
        )

    base_mn, mn = base.mnemonics(), entry.mnemonics()
    complaint = None
    if base_mn and not mn:
        complaint = (
            f"the {base_locale} label declares an `&` mnemonic, this "
            f"translation has none — the menu item loses its Alt+key access"
        )
    elif mn and not base_mn:
        complaint = (
            f"declares an `&` mnemonic the {base_locale} label does not; "
            f"if it is a literal ampersand, escape it as `&&`"
        )
    elif mn > 1:
        complaint = (
            f"{mn} `&` mnemonics in one label; only the first binds "
            f"(write `&&` for a literal ampersand)"
        )
    elif mn and entry.loose_ampersands():
        complaint = (
            "mixes a mnemonic with an unescaped literal `&`; the literal one "
            "must be written `&&`"
        )
    if complaint:
        problems.append(
            Problem(locale, "mnemonic", entry.name, complaint, entry.file,
                    entry.line)
        )

    if base.file.name != entry.file.name:
        problems.append(
            Problem(
                locale,
                "placement",
                entry.name,
                f"lives in {entry.file.name} but in {base.file.name} for en-US",
                entry.file,
                entry.line,
            )
        )


def check_locale_internals(
    parsed: ParsedLocale, check_categories: bool
) -> list[Problem]:
    """Checks that need no base locale: shape, references, selectors."""
    problems: list[Problem] = []
    language = parsed.locale.split("-")[0].lower()
    categories = CLDR_CATEGORIES.get(language) if check_categories else None

    for entry in parsed.entries.values():
        if entry.is_empty():
            problems.append(
                Problem(
                    parsed.locale,
                    "empty",
                    entry.name,
                    "value is empty",
                    entry.file,
                    entry.line,
                )
            )

        for span in selector_spans(entry.full_text):
            if not DEFAULT_VARIANT_RE.search(span):
                problems.append(
                    Problem(
                        parsed.locale,
                        "selector",
                        entry.name,
                        "selector has no `*[default]` variant (Fluent will "
                        "refuse to parse it)",
                        entry.file,
                        entry.line,
                    )
                )
            if categories is None:
                continue
            for _, key in VARIANT_RE.findall(span):
                # Numeric variants (`[0]`, `[1]`) are exact matches, not
                # plural categories, and are always legal.
                if not key or key.replace(".", "", 1).isdigit():
                    continue
                if key not in categories:
                    problems.append(
                        Problem(
                            parsed.locale,
                            "plural",
                            entry.name,
                            f"variant [{key}] is not a CLDR plural category "
                            f"for '{language}' "
                            f"({', '.join(sorted(categories))})",
                            entry.file,
                            entry.line,
                        )
                    )

        for ref in entry.references():
            if ref not in parsed.entries:
                problems.append(
                    Problem(
                        parsed.locale,
                        "reference",
                        entry.name,
                        f"references {{ {ref} }}, which this locale does not "
                        f"define",
                        entry.file,
                        entry.line,
                    )
                )

        for target in entry.tooltip_links():
            if target not in parsed.entries:
                problems.append(
                    Problem(
                        parsed.locale,
                        "tooltip-link",
                        entry.name,
                        f"links to (:{target}), which this locale does not "
                        f"define",
                        entry.file,
                        entry.line,
                    )
                )

    return problems


def check_against_base(
    parsed: ParsedLocale,
    base: ParsedLocale,
    check_untranslated: bool,
    untranslated_min_words: int = DEFAULT_UNTRANSLATED_MIN_WORDS,
) -> list[Problem]:
    problems: list[Problem] = []
    base_keys, keys = set(base.entries), set(parsed.entries)

    for name in sorted(base_keys - keys):
        origin = base.entries[name]
        problems.append(
            Problem(
                parsed.locale,
                "missing",
                name,
                f"not translated (defined in {origin.file.name} for "
                f"{base.locale})",
                origin.file,
                origin.line,
            )
        )

    for name in sorted(keys - base_keys):
        entry = parsed.entries[name]
        problems.append(
            Problem(
                parsed.locale,
                "extra",
                name,
                f"not defined in {base.locale}; dead string or a typo'd key",
                entry.file,
                entry.line,
            )
        )

    for name in sorted(base_keys & keys):
        check_entry_pair(parsed.locale, base.locale, base.entries[name],
                         parsed.entries[name], problems)
        if check_untranslated:
            base_entry, entry = base.entries[name], parsed.entries[name]
            words = base_entry.word_count()
            if (
                base_entry.value.strip()
                and base_entry.value == entry.value
                and words >= untranslated_min_words
            ):
                problems.append(
                    Problem(
                        parsed.locale,
                        "untranslated",
                        name,
                        f"identical to {base.locale} and {words} words long — "
                        f"likely left untranslated rather than a cognate",
                        entry.file,
                        entry.line,
                    )
                )

    return problems


def loader_includes(loader: Path) -> set[str]:
    """Every `.ftl` path the loader compiles in, in either spelling.

    Paths are returned exactly as written, relative to the loader's own
    directory — which is what `include_str!` resolves against.
    """
    source = loader.read_text(encoding="utf-8")
    paths = set(INCLUDE_STR_RE.findall(source))
    for base, locales, files in COMPILE_IN_LOCALES_RE.findall(source):
        for locale in STRING_LITERAL_RE.findall(locales):
            for name in STRING_LITERAL_RE.findall(files):
                paths.add(f"{base}{locale}/{name}")
    return paths


def check_registration(
    loader: Path, discovered: dict[str, list[Path]]
) -> list[Problem]:
    """Every .ftl on disk must be `include_str!`d by the loader.

    Adding `foo.ftl` and forgetting the `include_str!` line compiles cleanly
    and ships nothing — the strings are simply never loaded.
    """
    if not loader.is_file():
        return []
    included = loader_includes(loader)
    included_names = {Path(p).name for p in included}
    included_paths = {"/".join(Path(p).parts[-2:]) for p in included}
    # `include_str!` resolves against the file it appears in, so the loader's
    # own directory is what a relative base like `../locales/` hangs off.
    included_resolved = {(loader.parent / p).resolve() for p in included}

    problems: list[Problem] = []
    for locale, files in discovered.items():
        for path in files:
            relative = f"{locale}/{path.name}"
            if (
                path.resolve() in included_resolved
                or relative in included_paths
                or path.name in included_names
            ):
                continue
            problems.append(
                Problem(
                    locale,
                    "unregistered",
                    "",
                    f"{rel(path)} is never compiled in by {rel(loader)} "
                    f"(neither include_str! nor compile_in_locales!); "
                    f"its strings do not reach the binary",
                    path,
                    0,
                )
            )
    return problems


def check_unused(base: ParsedLocale, rust_src: Path) -> list[Problem]:
    """Base keys never referenced from Rust — a heuristic, hence opt-in.

    Two reference styles exist: `tr!(some_key())`, where the identifier maps to
    the kebab-case key, and bare `"some-key"` literals (the tooltip registry).
    """
    if not rust_src.is_dir():
        return []
    referenced: set[str] = set()
    for path in rust_src.rglob("*.rs"):
        source = path.read_text(encoding="utf-8", errors="replace")
        for ident in re.findall(r"tr!\s*\(\s*([a-z][a-z0-9_]*)", source):
            referenced.add(ident.replace("_", "-"))
        referenced.update(re.findall(r'"([a-z][a-z0-9]*(?:-[a-z0-9]+)+)"', source))

    problems: list[Problem] = []
    for name, entry in sorted(base.entries.items()):
        if name not in referenced:
            problems.append(
                Problem(
                    base.locale,
                    "unused",
                    name,
                    "never referenced from Rust (heuristic; dynamic keys are "
                    "not detected)",
                    entry.file,
                    entry.line,
                )
            )
    return problems


# --- Reporting ---------------------------------------------------------------


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


def report_text(problems: list[Problem], stream) -> None:
    if not problems:
        print("All locales are in sync with the base locale.", file=stream)
        return

    by_locale: dict[str, list[Problem]] = {}
    for problem in problems:
        by_locale.setdefault(problem.locale, []).append(problem)

    for locale in sorted(by_locale):
        entries = by_locale[locale]
        errors = sum(1 for p in entries if p.severity == "error")
        warnings = len(entries) - errors
        print(f"\n{locale}: {errors} error(s), {warnings} warning(s)",
              file=stream)
        by_check: dict[str, list[Problem]] = {}
        for problem in entries:
            by_check.setdefault(problem.check, []).append(problem)
        for check in sorted(by_check):
            group = by_check[check]
            severity = group[0].severity
            print(f"  [{severity}] {check} ({len(group)})", file=stream)
            for problem in group:
                where = ""
                if problem.file is not None:
                    where = rel(problem.file)
                    if problem.line:
                        where += f":{problem.line}"
                label = problem.key or "-"
                print(f"    {label}  {problem.message}", file=stream)
                if where:
                    print(f"      at {where}", file=stream)


def report_github(problems: list[Problem], stream) -> None:
    for problem in problems:
        location = ""
        if problem.file is not None:
            location = f" file={rel(problem.file)}"
            if problem.line:
                location += f",line={problem.line}"
        title = f"{problem.locale}: {problem.check}"
        key = f"{problem.key}: " if problem.key else ""
        print(f"::{problem.severity}{location},title={title}::{key}"
              f"{problem.message}", file=stream)


def report_json(problems: list[Problem], stream) -> None:
    json.dump(
        [
            {
                "locale": p.locale,
                "check": p.check,
                "severity": p.severity,
                "key": p.key,
                "message": p.message,
                "file": rel(p.file) if p.file else None,
                "line": p.line,
            }
            for p in problems
        ],
        stream,
        indent=2,
        ensure_ascii=False,
    )
    print(file=stream)


def print_stubs(locale: str, parsed: ParsedLocale, base: ParsedLocale) -> None:
    """Emit paste-ready stub lines for every key this locale is missing."""
    missing = sorted(set(base.entries) - set(parsed.entries))
    if not missing:
        print(f"# {locale} defines every {base.locale} key.")
        return
    by_file: dict[str, list[str]] = {}
    for name in missing:
        by_file.setdefault(base.entries[name].file.name, []).append(name)
    for filename in sorted(by_file):
        print(f"\n# --- {locale}/{filename} ---")
        for name in by_file[filename]:
            source = base.entries[name].value
            for line in source.splitlines() or [""]:
                print(f"# {base.locale}: {line}")
            print(f"{name} = {source.splitlines()[0] if source else ''}")


# --- Entry point -------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Compare .ftl translation locales against the base locale."
    )
    parser.add_argument(
        "--locales-dir",
        default=DEFAULT_LOCALES_DIR,
        help=f"locales directory (default: {DEFAULT_LOCALES_DIR})",
    )
    parser.add_argument(
        "--base",
        default=DEFAULT_BASE_LOCALE,
        help=f"base/source locale (default: {DEFAULT_BASE_LOCALE})",
    )
    parser.add_argument(
        "--locale",
        action="append",
        dest="only",
        metavar="LOCALE",
        help="check only this locale (repeatable)",
    )
    parser.add_argument(
        "--format",
        choices=("text", "github", "json"),
        default="text",
        help="report format (default: text)",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="treat warnings as errors",
    )
    parser.add_argument(
        "--check-untranslated",
        action="store_true",
        help="warn on multi-word values identical to the base locale "
             "(likely stubs)",
    )
    parser.add_argument(
        "--untranslated-min-words",
        type=int,
        default=DEFAULT_UNTRANSLATED_MIN_WORDS,
        metavar="N",
        help="minimum word count for --check-untranslated to flag an "
             f"identical value (default: {DEFAULT_UNTRANSLATED_MIN_WORDS}; "
             "pass 1 to see every identical value, cognates included)",
    )
    parser.add_argument(
        "--check-unused",
        action="store_true",
        help="warn on base keys never referenced from Rust (heuristic)",
    )
    parser.add_argument(
        "--no-plural-check",
        action="store_true",
        help="skip the CLDR plural-category check",
    )
    parser.add_argument(
        "--loader",
        default=DEFAULT_LOADER,
        help=f"Rust file compiling the locale catalogues in "
             f"(default: {DEFAULT_LOADER})",
    )
    parser.add_argument(
        "--rust-src",
        default=DEFAULT_RUST_SRC,
        help=f"Rust source root for --check-unused (default: {DEFAULT_RUST_SRC})",
    )
    parser.add_argument(
        "--stubs",
        metavar="LOCALE",
        help="print stub lines for the keys this locale is missing, then exit",
    )
    args = parser.parse_args()

    locales_dir = Path(args.locales_dir)
    if not locales_dir.is_absolute():
        locales_dir = REPO_ROOT / locales_dir
    if not locales_dir.is_dir():
        print(f"error: no such locales directory: {locales_dir}",
              file=sys.stderr)
        return 2

    discovered = discover_locales(locales_dir)
    if args.base not in discovered:
        print(f"error: base locale '{args.base}' not found in {locales_dir} "
              f"(found: {', '.join(sorted(discovered)) or 'nothing'})",
              file=sys.stderr)
        return 2

    base = load_locale(args.base, discovered[args.base])

    if args.stubs:
        if args.stubs not in discovered:
            print(f"error: unknown locale '{args.stubs}'", file=sys.stderr)
            return 2
        print_stubs(args.stubs, load_locale(args.stubs, discovered[args.stubs]),
                    base)
        return 0

    targets = sorted(name for name in discovered if name != args.base)
    if args.only:
        unknown = [name for name in args.only if name not in discovered]
        if unknown:
            print(f"error: unknown locale(s): {', '.join(unknown)}",
                  file=sys.stderr)
            return 2
        targets = [name for name in args.only if name != args.base]

    problems: list[Problem] = list(base.problems)
    problems += check_locale_internals(base, not args.no_plural_check)
    if args.check_unused:
        rust_src = Path(args.rust_src)
        if not rust_src.is_absolute():
            rust_src = REPO_ROOT / rust_src
        problems += check_unused(base, rust_src)

    loader = Path(args.loader)
    if not loader.is_absolute():
        loader = REPO_ROOT / loader
    problems += check_registration(loader, discovered)

    for locale in targets:
        parsed = load_locale(locale, discovered[locale])
        problems += parsed.problems
        problems += check_locale_internals(parsed, not args.no_plural_check)
        problems += check_against_base(parsed, base, args.check_untranslated,
                                       args.untranslated_min_words)

    if args.format == "github":
        report_github(problems, sys.stdout)
        # Annotations land on the PR diff, not in the log; without this the
        # step body is blank on success and reads like it never ran.
        checked = ", ".join(sorted(discovered))
        print(f"Checked {checked} against {args.base}.")
    elif args.format == "json":
        report_json(problems, sys.stdout)
    else:
        report_text(problems, sys.stdout)

    errors = sum(1 for p in problems if p.severity == "error")
    warnings = len(problems) - errors

    if args.format == "text":
        print(
            f"\n{len(discovered)} locale(s) checked against {args.base}: "
            f"{errors} error(s), {warnings} warning(s)."
        )

    if errors or (args.strict and warnings):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
