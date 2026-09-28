#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
#
# Rewrite Cargo.lock so it resolves from crates.io alone.
#
# Why this exists: teksilo, teksilo-charts, teksilo-text and text-document are
# pinned by version in the workspace manifest, and a developer's gitignored
# `.cargo/config.toml` patches them to the sibling checkouts through
# `[patch.crates-io]`. Cargo records a patched package as a path package and
# drops the `source` and `checksum` lines from its Cargo.lock entry, so every
# build with the patch active leaves a lockfile that resolves on that machine
# and nowhere else.
#
# Committing that is silent: no build, test or check notices, because each one
# re-resolves from crates.io on the fly. The one step that cannot is the
# Flatpak vendoring, which passes `--locked` because flatpak-builder builds
# offline. That is where v3.0.4's release died, after the tag was cut. The
# `lockfile` job in ci.yml fails such a lockfile; this script puts it right.
#
# Cargo reads `.cargo/config.toml` from the directory it runs in and from
# every directory above that one, not from the manifest's, so the resolve
# below runs from an empty scratch directory. That also covers a worktree
# nested inside a checkout whose `.cargo/config.toml` patches it, which moving
# the worktree's own file aside would miss, and it never touches the
# developer's file. `$CARGO_HOME/config.toml` is read from anywhere, so a
# patch kept there is refused rather than worked around.
#
# What it keeps. The stripped lockfile names each patched crate at the version
# of the checkout that was tested. Once the patch is out of the way that entry
# matches nothing on crates.io, so cargo drops it and resolves the crate again
# to the newest release the manifest allows: a newer release would be taken
# without a word, and a checkout ahead of its last release would be locked
# back down to that release. The script therefore sets every crate the resolve
# moved back to its tested version, then checks that the new lockfile differs
# from the old one by added `source` and `checksum` lines and nothing else.
# Where that cannot hold, most often because a checkout is ahead of its last
# release, it puts Cargo.lock back as it was, names each crate, and fails.
#
# Requires: git, cargo, awk, diff and network access.
# Usage, from anywhere in the repository: scripts/relock-crates-io.sh
# Its test: scripts/relock-crates-io-test.sh
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
manifest="$repo_root/Cargo.toml"
lockfile="$repo_root/Cargo.lock"

if [[ ! -f "$lockfile" ]]; then
  echo "error: $lockfile does not exist, so there is nothing to relock." >&2
  exit 1
fi

cargo_home="${CARGO_HOME:-$HOME/.cargo}"
for config in "$cargo_home/config.toml" "$cargo_home/config"; do
  if [[ -f "$config" ]] && grep -q '^[[:space:]]*\[patch' "$config"; then
    echo "error: $config patches crates wherever cargo runs." >&2
    echo "Move its [patch] table aside, run this again, then put it back." >&2
    exit 1
  fi
done

scratch="$(mktemp -d)"
tested_lock="$scratch/Cargo.lock.tested"
cp "$lockfile" "$tested_lock"

# From here on, any failure puts the tested lockfile back.
finish() {
  local status=$?
  if [[ $status -ne 0 ]]; then
    cp "$tested_lock" "$lockfile"
    echo "Cargo.lock is left as it was." >&2
  fi
  rm -rf "$scratch"
  exit "$status"
}
trap finish EXIT
cd "$scratch"

# Every locked crate as `name version`, one line each, sorted. Only
# `[[package]]` tables count: a `[[patch.unused]]` entry locks nothing.
locked_versions() {
  awk '
    /^\[/ { in_package = ($0 == "[[package]]"); name = ""; next }
    in_package && /^name = "/ { name = $3; gsub(/"/, "", name) }
    in_package && /^version = "/ { version = $3; gsub(/"/, "", version); print name " " version }
  ' "$1" | LC_ALL=C sort
}

# `name tested resolved` for each crate locked at a single version in both
# lists, at two different versions: the ones a pin can put back.
moved_crates() {
  awk '
    FNR == NR { tested_count[$1]++; tested[$1] = $2; next }
    { resolved_count[$1]++; resolved[$1] = $2 }
    END {
      for (name in tested)
        if (tested_count[name] == 1 && resolved_count[name] == 1 && tested[name] != resolved[name])
          print name " " tested[name] " " resolved[name]
    }
  ' "$1" "$2" | LC_ALL=C sort
}

# One line per crate whose locked versions differ between the two lists.
version_changes() {
  awk '
    FNR == NR { if ($1 in tested) tested[$1] = tested[$1] ", " $2; else tested[$1] = $2; next }
    { if ($1 in resolved) resolved[$1] = resolved[$1] ", " $2; else resolved[$1] = $2 }
    END {
      for (name in tested) {
        now = (name in resolved) ? resolved[name] : "nothing"
        if (tested[name] != now)
          print "  " name ": tested at " tested[name] ", crates.io gives " now
      }
      for (name in resolved)
        if (!(name in tested))
          print "  " name ": not in the tested lockfile, crates.io adds " resolved[name]
    }
  ' "$1" "$2" | LC_ALL=C sort
}

# The lockfile without its `[[patch.unused]]` tables, which record no
# resolve, and without blank lines, which only separate tables.
resolve_lines() {
  awk '
    /^\[/ { unused_patch = ($0 == "[[patch.unused]]") }
    !unused_patch && NF { print }
  ' "$1"
}

locked_versions "$tested_lock" >"$scratch/tested"

# A plain resolve, not `cargo generate-lockfile` and not `cargo update`: both
# move every crate to its newest compatible version. This one keeps every
# entry that still matches a crates.io release and re-resolves only the
# entries the patch stripped, which the pins below then set back.
echo "==> Resolving Cargo.lock against crates.io"
if ! cargo metadata --manifest-path "$manifest" --format-version 1 >/dev/null; then
  echo "error: Cargo.lock does not resolve from crates.io alone (cargo's reason is above)." >&2
  echo "A crate the manifest asks for may be newer than its last release on crates.io." >&2
  exit 1
fi

# A pin fails when crates.io does not have the tested version, or when another
# crate, still at its newer version, needs this one newer too. The second kind
# goes through once that crate is back, so a failed pin is tried again in the
# next round, for as long as a round pins something and at most one round
# more than the number of crates the resolve moved.
locked_versions "$lockfile" >"$scratch/resolved"
moved_crates "$scratch/tested" "$scratch/resolved" >"$scratch/moved"
cp "$scratch/moved" "$scratch/moved.by-resolve"
rounds_left=$(($(wc -l <"$scratch/moved") + 1))
: >"$scratch/pin.log"
while ! cmp -s "$scratch/tested" "$scratch/resolved" && ((rounds_left > 0)); do
  rounds_left=$((rounds_left - 1))
  pinned_this_round=0
  while read -r name tested_version resolved_version; do
    if cargo update --manifest-path "$manifest" \
      --package "$name@$resolved_version" --precise "$tested_version" \
      </dev/null >>"$scratch/pin.log" 2>&1; then
      pinned_this_round=1
    fi
  done <"$scratch/moved"
  locked_versions "$lockfile" >"$scratch/resolved"
  moved_crates "$scratch/tested" "$scratch/resolved" >"$scratch/moved"
  ((pinned_this_round)) || break
done

if ! cmp -s "$scratch/tested" "$scratch/resolved"; then
  {
    echo "error: crates.io cannot lock these crates at the versions Cargo.lock was tested with:"
    version_changes "$scratch/tested" "$scratch/resolved"
    echo "Most often a sibling checkout is ahead of its last release. Publish that"
    echo "release and run this again, or build and test against the version"
    echo "crates.io has. What cargo said when asked to keep each tested version:"
    sed 's/^/    /' "$scratch/pin.log"
  } >&2
  exit 1
fi

resolve_lines "$tested_lock" >"$scratch/tested.lines"
resolve_lines "$lockfile" >"$scratch/relocked.lines"
diff "$scratch/tested.lines" "$scratch/relocked.lines" >"$scratch/lines.diff" || [[ $? -eq 1 ]]
grep -E '^[<>] ' "$scratch/lines.diff" | grep -vE '^> (source|checksum) = ' >"$scratch/unexpected" || true
if [[ -s "$scratch/unexpected" ]]; then
  {
    echo "error: every crate keeps its tested version, but the lockfile changed in more than"
    echo "its source and checksum lines. The crates.io release of a patched crate does"
    echo "not match the checkout it was tested against; publish that checkout as a new"
    echo "release, or test against the release. Lines removed (<) and added (>):"
    head -n 40 "$scratch/unexpected" | sed 's/^/    /'
  } >&2
  exit 1
fi

echo "==> Checking that it needs no further change"
cargo metadata --manifest-path "$manifest" --locked --format-version 1 >/dev/null

if [[ -s "$scratch/moved.by-resolve" ]]; then
  echo "==> Set back to the versions they were tested at:"
  while read -r name tested_version resolved_version; do
    echo "    $name $tested_version (the resolve picked $resolved_version)"
  done <"$scratch/moved.by-resolve"
fi

echo "==> Done. Cargo.lock only gained source and checksum lines. Review and commit it:"
git -C "$repo_root" --no-pager diff --stat -- Cargo.lock
