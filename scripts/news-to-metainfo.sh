#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
#
# Write the <releases> block of eu.skribisto.skribisto.metainfo.xml from
# NEWS.yml, for one version, and refuse to when NEWS.yml does not describe that
# version as the release being made.
#
# Why this runs inside `cargo release` and nowhere else: a Linux software
# centre, and Flathub above all, reads the release notes from the metainfo of
# the TAG it builds. Until 3.2.0 a workflow regenerated the file after NEWS.yml
# reached master and committed it back from a bot, so what a tag held depended
# on which of the two landed first. v3.0.1 shipped 3.0.0's notes that way, and
# v3.1.0 reached Flathub as a "development" release. Run as cargo-release's
# pre-release hook, the regenerated file goes into the release commit itself,
# and the tag cannot exist without it.
#
# Usage:
#   scripts/news-to-metainfo.sh            # write; the version comes from cargo-release
#   scripts/news-to-metainfo.sh --check    # compare only; fails if the file differs
#   VERSION=3.3.0 scripts/news-to-metainfo.sh --check
#
# cargo-release hands the hook NEW_VERSION, DRY_RUN and WORKSPACE_ROOT; by
# hand, set VERSION instead. A dry run (`cargo release` without
# `--execute`) runs every check and prints the diff it would apply, but writes
# nothing. release.yml runs `--check` against the tag, as a last gate before
# any artifact is built.
#
# Checks, all against the FIRST block of NEWS.yml, which is the newest:
#   - its Version is the version being released;
#   - its Type is `development` for a pre-release (a version with a `-`) and
#     `stable` otherwise, the same rule as the GitHub Release's prerelease flag;
#   - when writing, its Date is today: the date is what the software centre
#     shows, and a block written days before the release keeps the day it was
#     written unless someone remembers to change it.
#
# Only the newest release is kept (`--limit=1`). news-to-metainfo replaces the
# <releases> element and leaves the rest of the file, including the translated
# description, byte for byte as it was.
#
# Requires: appstreamcli (AppStream 1.0 or later), awk, diff.

set -euo pipefail

MODE=write
case "${1:-}" in
    "") ;;
    --check) MODE=check ;;
    *)
        echo "usage: $0 [--check]" >&2
        exit 2
        ;;
esac

ROOT="${WORKSPACE_ROOT:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
NEWS="$ROOT/NEWS.yml"
METAINFO="$ROOT/eu.skribisto.skribisto.metainfo.xml"

VERSION="${VERSION:-${NEW_VERSION:-}}"
if [ -z "$VERSION" ]; then
    echo "error: no version. cargo-release sets NEW_VERSION; by hand, run VERSION=x.y.z $0" >&2
    exit 2
fi
if ! command -v appstreamcli >/dev/null 2>&1; then
    echo "error: appstreamcli is not installed (package \`appstream\`)." >&2
    exit 1
fi

# One field of the first NEWS.yml block, unquoted.
field() {
    awk -v key="$1" '
        /^---/ { exit }
        $0 ~ "^" key ":" {
            sub("^" key ":[[:space:]]*", "")
            gsub(/^"|"[[:space:]]*$/, "")
            sub(/[[:space:]]+$/, "")
            print
            exit
        }
    ' "$NEWS"
}

NEWS_VERSION="$(field Version)"
NEWS_TYPE="$(field Type)"
NEWS_DATE="$(field Date)"

fail=0
if [ "$NEWS_VERSION" != "$VERSION" ]; then
    echo "error: the newest block of NEWS.yml is ${NEWS_VERSION:-missing}, but the release is $VERSION." >&2
    echo "       Add a block for $VERSION at the top of NEWS.yml." >&2
    fail=1
fi
case "$VERSION" in
    *-*) expected_type=development ;;
    *) expected_type=stable ;;
esac
if [ "$NEWS_TYPE" != "$expected_type" ]; then
    echo "error: NEWS.yml gives $VERSION the type ${NEWS_TYPE:-missing}; a $VERSION release must be $expected_type." >&2
    fail=1
fi
if [ "$MODE" = write ]; then
    today="$(date +%F)"
    if [ "$NEWS_DATE" != "$today" ]; then
        echo "error: NEWS.yml dates $VERSION ${NEWS_DATE:-nowhere}; a release made today must say Date: $today." >&2
        fail=1
    fi
fi
[ "$fail" = 0 ] || exit 1

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
cp "$METAINFO" "$scratch/metainfo.xml"
appstreamcli news-to-metainfo --limit=1 "$NEWS" "$scratch/metainfo.xml" >/dev/null
appstreamcli validate --no-net --strict "$scratch/metainfo.xml" >/dev/null || {
    appstreamcli validate --no-net --strict "$scratch/metainfo.xml" >&2 || true
    echo "error: the regenerated metainfo does not validate; nothing was written." >&2
    exit 1
}

if cmp -s "$METAINFO" "$scratch/metainfo.xml"; then
    echo "metainfo already describes $VERSION as NEWS.yml does."
    exit 0
fi

if [ "$MODE" = check ]; then
    diff -u "$METAINFO" "$scratch/metainfo.xml" >&2 || true
    echo "error: eu.skribisto.skribisto.metainfo.xml is not what NEWS.yml regenerates to for $VERSION." >&2
    echo "       Run VERSION=$VERSION scripts/news-to-metainfo.sh and commit the result." >&2
    exit 1
fi

if [ "${DRY_RUN:-false}" = true ]; then
    echo "dry run: would rewrite the metainfo's releases for $VERSION:"
    diff -u "$METAINFO" "$scratch/metainfo.xml" || true
    exit 0
fi

cp "$scratch/metainfo.xml" "$METAINFO"
echo "metainfo releases written for $VERSION."
