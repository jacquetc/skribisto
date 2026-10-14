#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
#
# Refuse a release whose Cargo.lock would not resolve from crates.io alone.
#
# Why this runs as cargo-release's pre-release hook: `cargo release` bumps the
# version and rewrites Cargo.lock before any hook runs, and it does that with
# whatever `.cargo/config.toml` cargo finds. A developer's `[patch.crates-io]`
# overlay (see relock-crates-io.sh) then strips the `source` and `checksum`
# lines of every patched crate, and the release commit and tag would carry a
# lockfile that only resolves on that machine. v3.0.4's Flatpak build died of
# exactly that, after the tag was cut. A hook cannot undo the rewrite, but it
# runs before the commit, so failing here leaves no commit and no tag; the
# bumped files are left in the working tree for `git checkout .`.
#
# Refuses when:
#   - a `.cargo/config.toml` (or `.cargo/config`) in the workspace or any
#     directory above it, or in $CARGO_HOME, holds a `[patch` table. Its mere
#     presence is not enough: such a file may hold other settings.
#   - Cargo.lock, at HEAD or in the working tree, has a package that is not a
#     workspace member and has no `source` line. HEAD matters on its own: with
#     the overlay moved aside, cargo-release re-resolves a stripped entry to the
#     newest release the manifest allows, without a word, so a stripped
#     lockfile must go through relock-crates-io.sh and be committed first.
#
# A dry run (`cargo release` without `--execute`) runs the same checks, so the
# refusal shows before anything is written.
#
# cargo-release hands the hook WORKSPACE_ROOT; by hand it is found through git.
# Usage: scripts/release-preflight.sh
# Requires: git, awk.

set -euo pipefail

ROOT="${WORKSPACE_ROOT:-$(git -C "$(dirname "$0")" rev-parse --show-toplevel)}"
cargo_home="${CARGO_HOME:-$HOME/.cargo}"
failed=0

# Cargo reads `.cargo/config.toml` from the directory it runs in and from
# every directory above it, and `$CARGO_HOME/config.toml` from anywhere.
patching_configs() {
    local dir="$ROOT" config
    while :; do
        for config in "$dir/.cargo/config.toml" "$dir/.cargo/config"; do
            if [[ -f "$config" ]] && grep -q '^[[:space:]]*\[patch' "$config"; then
                echo "$config"
            fi
        done
        [[ "$dir" == "/" ]] && break
        dir="$(dirname "$dir")"
    done
    for config in "$cargo_home/config.toml" "$cargo_home/config"; do
        if [[ -f "$config" ]] && grep -q '^[[:space:]]*\[patch' "$config"; then
            echo "$config"
        fi
    done
}

# The package name of every workspace member, one per line.
member_names() {
    awk '
        /^members[[:space:]]*=/ { in_members = 1 }
        in_members {
            line = $0
            while (match(line, /"[^"]+"/)) {
                print substr(line, RSTART + 1, RLENGTH - 2)
                line = substr(line, RSTART + RLENGTH)
            }
            if ($0 ~ /\]/) exit
        }
    ' "$ROOT/Cargo.toml" | while read -r member; do
        awk '
            /^\[/ { in_package = ($0 == "[package]"); next }
            in_package && /^name[[:space:]]*=/ { gsub(/.*=[[:space:]]*"|".*/, ""); print; exit }
        ' "$ROOT/$member/Cargo.toml"
    done
}

# Every `[[package]]` of a lockfile read on stdin that has no `source` line
# and is not a workspace member (the names in the file given as $1).
sourceless_packages() {
    awk '
        function flush() { if (in_package && name != "" && !has_source && !(name in member)) print "  " name " " version }
        FNR == NR { member[$0] = 1; next }
        /^\[/ { flush(); in_package = ($0 == "[[package]]"); name = ""; version = ""; has_source = 0; next }
        in_package && /^name = "/ { name = $3; gsub(/"/, "", name) }
        in_package && /^version = "/ { version = $3; gsub(/"/, "", version) }
        in_package && /^source = / { has_source = 1 }
        END { flush() }
    ' "$1" -
}

configs="$(patching_configs)"
if [[ -n "$configs" ]]; then
    {
        echo "error: cargo would patch crates for this release through:"
        echo "$configs" | sed 's/^/  /'
        echo "cargo-release has already rewritten Cargo.lock through that patch, so the"
        echo "release would commit a lockfile that resolves on this machine only."
        echo "Move each [patch] table aside, put the working tree back (git checkout .),"
        echo "and run cargo release again."
    } >&2
    failed=1
fi

members="$(mktemp)"
trap 'rm -f "$members"' EXIT
member_names >"$members"

if git -C "$ROOT" cat-file -e HEAD:Cargo.lock 2>/dev/null; then
    stripped="$(git -C "$ROOT" show HEAD:Cargo.lock | sourceless_packages "$members")"
    if [[ -n "$stripped" ]]; then
        {
            echo "error: the committed Cargo.lock does not say where these crates come from:"
            echo "$stripped"
            echo "Released from it, cargo would lock each one to the newest release on"
            echo "crates.io rather than the version tested. With the [patch] table moved"
            echo "aside, run scripts/relock-crates-io.sh, commit the lockfile, and release again."
        } >&2
        failed=1
    fi
fi

if [[ -f "$ROOT/Cargo.lock" ]]; then
    stripped="$(sourceless_packages "$members" <"$ROOT/Cargo.lock")"
    if [[ -n "$stripped" ]]; then
        {
            echo "error: Cargo.lock in the working tree does not say where these crates come from:"
            echo "$stripped"
        } >&2
        failed=1
    fi
fi

if ((failed)); then
    exit 1
fi
echo "Cargo.lock resolves from crates.io and no [patch] is active."
