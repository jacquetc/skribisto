#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

"""
Check or apply SPDX license headers across the repository.

Every source file tracked by git that matches a known extension must start
with the canonical two-line SPDX header (after a shebang or `<?xml ?>` PI
if present), using the comment style appropriate for the file:

    SPDX-License-Identifier: GPL-3.0-only
    SPDX-FileCopyrightText: <year> Cyril Jacquet

The year is the year of the file's first commit, derived from
`git log --diff-filter=A`. Files that are not yet committed fall back to
the current year (override with `--fallback-year YYYY`). Existing
headers whose year disagrees with git history are normalized.

Scope note (Skribisto-specific): the tool-managed Qleany manifest and the
bundled icon/example/packaging assets are deliberately excluded (see SKIP_*
below) — the manifest is generator-owned, and asset provenance is mixed, so
stamping a copyright line on them would be wrong.

Modes:
  --check          report missing / outdated headers, non-zero exit if any
                   (use this in CI)
  --fix            insert missing headers and rewrite outdated lines in
                   place

By default the script runs in `--fix` mode. With no extra arguments it walks
every git-tracked file in the repository whose extension is known.
"""

from __future__ import annotations

import argparse
import datetime as dt
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

LICENSE_ID = "GPL-3.0-only"
COPYRIGHT_HOLDER = "Cyril Jacquet"

# (line_prefix, line_suffix) — line_suffix is "" for line-comment styles
# and " -->" / " */" for block-comment styles where each header line is its
# own self-closing block comment.
LINE_HASH = ("# ", "")
LINE_SLASH = ("// ", "")
LINE_HTML = ("<!-- ", " -->")
LINE_CBLOCK = ("/* ", " */")

# Map of file extension (lowercase, with leading dot) to comment style.
#
# `.svg` is intentionally NOT listed: Skribisto's bundled icons have mixed
# provenance (some are third-party icon sets), so a "Cyril Jacquet" copyright
# line on them would be a false claim. Keep asset files out of scope.
EXT_STYLE: dict[str, tuple[str, str]] = {
    ".rs": LINE_SLASH,
    ".wgsl": LINE_SLASH,
    ".js": LINE_SLASH,
    ".ts": LINE_SLASH,
    ".tsx": LINE_SLASH,
    ".jsx": LINE_SLASH,
    ".c": LINE_SLASH,
    ".h": LINE_SLASH,
    ".cpp": LINE_SLASH,
    ".hpp": LINE_SLASH,
    ".toml": LINE_HASH,
    ".py": LINE_HASH,
    ".sh": LINE_HASH,
    ".bash": LINE_HASH,
    ".zsh": LINE_HASH,
    ".yml": LINE_HASH,
    ".yaml": LINE_HASH,
    ".ftl": LINE_HASH,
    ".cfg": LINE_HASH,
    ".ini": LINE_HASH,
    ".md": LINE_HTML,
    ".html": LINE_HTML,
    ".htm": LINE_HTML,
    ".css": LINE_CBLOCK,
}

# Filenames (basename match) that should also be treated as `#`-comment.
SPECIAL_BASENAMES: dict[str, tuple[str, str]] = {
    "Dockerfile": LINE_HASH,
    "Makefile": LINE_HASH,
    ".gitignore": LINE_HASH,
    ".gitattributes": LINE_HASH,
}

# Filenames that should never be touched even if their extension is known.
SKIP_BASENAMES: set[str] = {
    "Cargo.lock",
    "LICENSE",
    "LICENSE.md",
    "LICENSE.txt",
    "COPYING",
    "NOTICE",
    "CHANGELOG.md",
    "NEWS.yml",
    # Tool-managed manifest — Qleany owns this; a header comment would be
    # churned by the generator.
    "qleany.yaml",
    # AppStream metainfo is validated by the packaging lint; keep it pristine.
    "eu.skribisto.skribisto.metainfo.xml",
}

# Path prefixes (relative to repo root, POSIX) that are excluded wholesale.
# Matched with str.startswith — anchored, so these are repo-root paths and do
# not affect the `crates/*/src/` and `crates/*/tests/` Rust sources.
SKIP_PATH_PREFIXES: tuple[str, ...] = (
    "resources/",    # icons / example bundles / packaging assets
    "package/",      # packaging scaffolding
)

# Path substrings (relative to repo root) that mark vendored, generated, or
# asset trees we never rewrite, wherever they appear in the tree.
#
# `.claude/` holds agent tooling (skill definitions, project instructions),
# not licensable source. Its SKILL.md files open with YAML frontmatter that
# must be the very first line, so a header inserted above it breaks parsing.
# `assets/` covers per-crate bundled assets (e.g. crates/teksilo_ui/assets/).
SKIP_PATH_SUBSTRINGS: tuple[str, ...] = (
    "target/",
    "dist/",
    ".git/",
    ".claude/",
    "assets/",
    "vendor/",
    "node_modules/",
)

LICENSE_LINE_RE = re.compile(r"SPDX-License-Identifier:\s*(\S+)")
COPYRIGHT_LINE_RE = re.compile(
    r"SPDX-FileCopyrightText:\s*(?P<years>\d{4}(?:\s*-\s*\d{4})?)\s+(?P<holder>.+?)\s*$"
)


@dataclass
class Issue:
    path: Path
    kind: str  # "missing" | "outdated"
    detail: str


def style_for(path: Path) -> tuple[str, str] | None:
    if path.name in SPECIAL_BASENAMES:
        return SPECIAL_BASENAMES[path.name]
    return EXT_STYLE.get(path.suffix.lower())


def should_skip(rel_posix: str, basename: str) -> bool:
    if basename in SKIP_BASENAMES:
        return True
    if any(rel_posix.startswith(p) for p in SKIP_PATH_PREFIXES):
        return True
    return any(seg in rel_posix for seg in SKIP_PATH_SUBSTRINGS)


def git_tracked_files(repo: Path) -> list[Path]:
    out = subprocess.run(
        ["git", "-C", str(repo), "ls-files", "-z"],
        check=True,
        capture_output=True,
    )
    files = [p for p in out.stdout.decode("utf-8").split("\x00") if p]
    return [repo / p for p in files]


def collect_first_commit_years(repo: Path) -> dict[str, int]:
    """
    Map every path that has ever been added to git → year of the commit
    that first added it. Single git subprocess. Renames are not followed
    (`--follow` doesn't compose with multi-path log) — a rename shows up
    as the new path being added in the rename commit, which is good
    enough in practice.
    """
    try:
        out = subprocess.run(
            [
                "git", "-C", str(repo), "log",
                "--diff-filter=A",
                "--name-only",
                "--format=__SPDX_COMMIT__ %ad",
                "--date=format:%Y",
                "--reverse",
                "-z",
            ],
            check=True,
            capture_output=True,
        )
    except subprocess.CalledProcessError:
        return {}
    # `-z` separates *records* with NUL but headers and filenames are
    # newline-separated within a record. Walk linearly.
    text = out.stdout.decode("utf-8", errors="replace").replace("\x00", "\n")
    years: dict[str, int] = {}
    current_year: int | None = None
    for line in text.splitlines():
        line = line.strip()
        if not line:
            continue
        if line.startswith("__SPDX_COMMIT__ "):
            try:
                current_year = int(line[len("__SPDX_COMMIT__ "):])
            except ValueError:
                current_year = None
        elif current_year is not None:
            years.setdefault(line, current_year)
    return years


def expected_header_lines(style: tuple[str, str], year: int) -> list[str]:
    prefix, suffix = style
    return [
        f"{prefix}SPDX-License-Identifier: {LICENSE_ID}{suffix}",
        f"{prefix}SPDX-FileCopyrightText: {year} {COPYRIGHT_HOLDER}{suffix}",
    ]


def split_preamble(lines: list[str]) -> tuple[list[str], list[str]]:
    """
    Split off shebang / xml PI / encoding-declaration lines that must remain
    at the very top of the file. Returns (preamble, rest).
    """
    preamble: list[str] = []
    i = 0
    if i < len(lines) and lines[i].startswith("#!"):
        preamble.append(lines[i])
        i += 1
    if i < len(lines) and lines[i].lstrip().startswith("<?xml"):
        preamble.append(lines[i])
        i += 1
    # Python encoding declaration must be on line 1 or 2.
    if (
        i < len(lines)
        and len(preamble) <= 1
        and re.match(r"#.*coding[:=]", lines[i])
    ):
        preamble.append(lines[i])
        i += 1
    return preamble, lines[i:]


def find_existing_header(
    rest: list[str], style: tuple[str, str]
) -> tuple[int, int, str | None, str | None] | None:
    """
    Locate the SPDX header in `rest` (after preamble). Returns
    (start, end_exclusive, license_id, copyright_year_str) when both
    SPDX lines are found within the first ~10 lines of `rest`. Lines may
    be in either order and may have arbitrary leading whitespace inside
    the comment.
    """
    license_idx: int | None = None
    license_id: str | None = None
    copyright_idx: int | None = None
    year_str: str | None = None

    scan_limit = min(len(rest), 10)
    for i in range(scan_limit):
        line = rest[i]
        m_lic = LICENSE_LINE_RE.search(line)
        if m_lic and license_idx is None:
            license_idx = i
            license_id = m_lic.group(1)
        m_copy = COPYRIGHT_LINE_RE.search(line)
        if m_copy and copyright_idx is None:
            copyright_idx = i
            year_str = m_copy.group("years")
        if license_idx is not None and copyright_idx is not None:
            break

    if license_idx is None or copyright_idx is None:
        return None
    start = min(license_idx, copyright_idx)
    end = max(license_idx, copyright_idx) + 1
    return (start, end, license_id, year_str)


def normalize_year(existing: str, target_year: int) -> str:
    """
    Return the canonical year string. We always emit the bare target
    year — `2026 Cyril Jacquet`, not `2024-2026 Cyril Jacquet` — so the
    header has one canonical form.
    """
    return str(target_year)


def process_file(
    path: Path, year: int, fix: bool
) -> Issue | None:
    style = style_for(path)
    if style is None:
        return None
    try:
        original = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, OSError):
        return None

    # Preserve trailing newline behavior of the original file.
    had_trailing_newline = original.endswith("\n")
    raw_lines = original.split("\n")
    if had_trailing_newline:
        raw_lines = raw_lines[:-1]

    preamble, rest = split_preamble(raw_lines)
    expected = expected_header_lines(style, year)

    found = find_existing_header(rest, style)
    if found is None:
        if not fix:
            return Issue(path, "missing", "no SPDX header")
        # Insert header at top of rest, with a blank line separating it
        # from existing content if there is content.
        new_rest = list(expected)
        if rest and rest[0].strip() != "":
            new_rest.append("")
        new_rest.extend(rest)
        new_lines = preamble + new_rest
        new_text = "\n".join(new_lines) + ("\n" if had_trailing_newline or new_lines else "")
        if not had_trailing_newline and new_lines:
            new_text = "\n".join(new_lines) + "\n"
        path.write_text(new_text, encoding="utf-8")
        return None

    start, end, license_id, year_str = found
    needs_update = False
    reasons: list[str] = []
    if license_id != LICENSE_ID:
        needs_update = True
        reasons.append(f"license is `{license_id}`, expected `{LICENSE_ID}`")
    canonical_year = normalize_year(year_str or "", year)
    if (year_str or "").strip() != canonical_year:
        needs_update = True
        reasons.append(f"year is `{year_str}`, expected `{canonical_year}`")
    # Also normalize when the lines are not literally what we expect
    # (e.g. wrong ordering, wrong holder, weird spacing).
    current_block = rest[start:end]
    if current_block != expected:
        if current_block != list(reversed(expected)):
            needs_update = True
            if not reasons:
                reasons.append("header text not canonical")
        else:
            # Same content, wrong order — still rewrite.
            needs_update = True
            if not reasons:
                reasons.append("header lines reversed")

    if not needs_update:
        return None
    if not fix:
        return Issue(path, "outdated", "; ".join(reasons))

    new_rest = rest[:start] + expected + rest[end:]
    new_lines = preamble + new_rest
    new_text = "\n".join(new_lines)
    if had_trailing_newline:
        new_text += "\n"
    path.write_text(new_text, encoding="utf-8")
    return None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument(
        "--check",
        action="store_true",
        help="report files needing changes; exit 1 if any (CI mode)",
    )
    mode.add_argument(
        "--fix",
        action="store_true",
        help="insert missing headers and update outdated years (default)",
    )
    parser.add_argument(
        "--fallback-year",
        type=int,
        default=dt.date.today().year,
        dest="fallback_year",
        help="year to stamp on files not yet in git history "
        "(default: current calendar year)",
    )
    parser.add_argument(
        "paths",
        nargs="*",
        type=Path,
        help="restrict to these paths (default: all git-tracked files)",
    )
    args = parser.parse_args()

    if not args.check and not args.fix:
        args.fix = True

    repo = Path(__file__).resolve().parent.parent
    if args.paths:
        candidates: list[Path] = []
        for p in args.paths:
            p = p.resolve()
            if p.is_dir():
                for root, _, files in os.walk(p):
                    for f in files:
                        candidates.append(Path(root) / f)
            elif p.is_file():
                candidates.append(p)
    else:
        candidates = git_tracked_files(repo)

    git_years = collect_first_commit_years(repo)

    issues: list[Issue] = []
    for path in candidates:
        try:
            rel = path.resolve().relative_to(repo)
        except ValueError:
            continue
        rel_posix = rel.as_posix() + ("/" if path.is_dir() else "")
        if should_skip(rel_posix, path.name):
            continue
        year = git_years.get(rel.as_posix(), args.fallback_year)
        issue = process_file(path, year, fix=args.fix)
        if issue is not None:
            issues.append(issue)

    if args.check:
        if issues:
            print(
                f"SPDX header check failed: {len(issues)} file(s) need updates",
                file=sys.stderr,
            )
            for issue in issues:
                rel = issue.path.resolve().relative_to(repo)
                print(f"  {issue.kind:9s} {rel}  ({issue.detail})", file=sys.stderr)
            print(
                "\nRun `python3 tools/check_spdx_headers.py --fix` to apply.",
                file=sys.stderr,
            )
            return 1
        print("SPDX header check passed (per-file years from git).")
        return 0

    print("SPDX headers normalized (per-file years from git).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
