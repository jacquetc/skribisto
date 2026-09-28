#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
#
# Test scripts/relock-crates-io.sh against small throwaway workspaces.
#
# Each case builds a git repository with one crate, writes the lockfile a
# `[patch.crates-io]` overlay would leave behind, runs the relock script in
# it and checks the result. The overlay is either a real one pointing at a
# stand-in crate, or a lockfile resolved from crates.io with the `source` and
# `checksum` lines of the patched crates removed: cargo writes the two
# identically. The crates are small ones with long release histories (itoa,
# indexmap, hashbrown, equivalent, ryu). Every published version named stays
# on crates.io, which never deletes a release, and itoa 1.9999.0 stands for a
# release that does not exist.
#
# Requires: git, cargo, awk, diff and network access.
# Usage: scripts/relock-crates-io-test.sh
set -euo pipefail

relock="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/relock-crates-io.sh"
work="$(mktemp -d)"
case_name="setting up"
finished=0
# A case's setup is not a check: when one of its cargo calls fails, `set -e`
# ends the run here, and this says where and what cargo said.
on_exit() {
  local status=$?
  if ((status != 0 && finished == 0)); then
    echo "error: setting up \"$case_name\" failed. What cargo said:" >&2
    tail -n 20 "$work/cargo.log" >&2 || true
  fi
  rm -rf "$work"
}
trap on_exit EXIT
: >"$work/cargo.log"
# Cargo reads `.cargo/config.toml` from the working directory upward, so every
# cargo call that must see no overlay runs from here.
cd "$work"

failures=0
fail() {
  echo "not ok - $case_name: $*" >&2
  failures=$((failures + 1))
}
pass() {
  echo "ok - $case_name"
}

# new_repo <name> <dependency line>...: a git repository holding one library
# crate with those dependencies, and no lockfile yet.
new_repo() {
  local repo="$work/$1"
  shift
  mkdir -p "$repo/src"
  git init -q "$repo"
  {
    printf '[package]\nname = "relock-case"\nversion = "0.0.0"\nedition = "2021"\npublish = false\n\n'
    printf '[dependencies]\n'
    printf '%s\n' "$@"
    printf '\n[workspace]\n'
  } >"$repo/Cargo.toml"
  : >"$repo/src/lib.rs"
  echo "$repo"
}

# stand_in <name> <version> [<dependency line>...]: a local crate of that
# name and version, with those dependencies, for an overlay to patch in.
stand_in() {
  local dir="$work/stand-in-$1-$2"
  mkdir -p "$dir/src"
  printf '[package]\nname = "%s"\nversion = "%s"\nedition = "2021"\n' "$1" "$2" >"$dir/Cargo.toml"
  if (($# > 2)); then
    printf '\n[dependencies]\n' >>"$dir/Cargo.toml"
    printf '%s\n' "${@:3}" >>"$dir/Cargo.toml"
  fi
  : >"$dir/src/lib.rs"
  echo "$dir"
}

# overlay <repo> <name> <path> [<name> <path>]...: patch each crate to its
# path for cargo run inside the repository, the way a developer's gitignored
# `.cargo/config.toml` does.
overlay() {
  local repo="$1"
  shift
  mkdir -p "$repo/.cargo"
  {
    printf '[patch.crates-io]\n'
    while (($# >= 2)); do
      printf '%s = { path = "%s" }\n' "$1" "$2"
      shift 2
    done
  } >"$repo/.cargo/config.toml"
}

# lock_inside <repo>: resolve with the repository's own overlay active.
lock_inside() {
  (cd "$1" && cargo metadata --format-version 1 >/dev/null 2>>"$work/cargo.log")
}

# lock_outside <repo>: resolve from crates.io, no overlay.
lock_outside() {
  cargo metadata --manifest-path "$1/Cargo.toml" --format-version 1 >/dev/null 2>>"$work/cargo.log"
}

# pin <repo> <name> <version>: lock that crate at that version, from crates.io.
pin() {
  cargo update --manifest-path "$1/Cargo.toml" --package "$2" --precise "$3" >/dev/null 2>>"$work/cargo.log"
}

# strip <repo> <name>...: remove the `source` and `checksum` lines of those
# crates' lockfile entries, as an overlay patching them leaves them.
strip() {
  local repo="$1"
  shift
  awk -v names=" $* " '
    /^\[/ { stripped = 0 }
    /^name = "/ { name = $3; gsub(/"/, "", name); stripped = index(names, " " name " ") > 0 }
    !(stripped && /^(source|checksum) = /) { print }
  ' "$repo/Cargo.lock" >"$repo/Cargo.lock.stripped"
  mv "$repo/Cargo.lock.stripped" "$repo/Cargo.lock"
}

# field <repo> <name> <key>: the value of that key in the crate's
# `[[package]]` entry, or nothing when the entry has no such line.
field() {
  awk -v want="$2" -v key="$3" '
    /^\[/ { in_package = ($0 == "[[package]]"); name = ""; next }
    in_package && /^name = "/ { name = $3; gsub(/"/, "", name) }
    in_package && name == want && $1 == key { value = $3; gsub(/"/, "", value); print value; exit }
  ' "$1/Cargo.lock"
}

# versions <repo>: every locked `name version`, sorted.
versions() {
  awk '
    /^\[/ { in_package = ($0 == "[[package]]"); next }
    in_package && /^name = "/ { name = $3 }
    in_package && /^version = "/ { print name " " $3 }
  ' "$1/Cargo.lock" | LC_ALL=C sort
}

# only_gained_sources <before> <after>: the second lockfile differs from the
# first by added `source` and `checksum` lines and nothing else.
only_gained_sources() {
  local changes
  changes="$(diff "$1" "$2" | grep -E '^[<>] ' | grep -vE '^> (source|checksum) = ' || true)"
  [[ -z "$changes" ]]
}

# relock <repo>: run the script inside the repository, keeping what it printed.
relock() {
  (cd "$1" && bash "$relock") >"$1/relock.out" 2>&1
}

registry='registry+https://github.com/rust-lang/crates.io-index'

case_name="an overlay's lockfile keeps the version it was tested at"
repo="$(new_repo overlay 'itoa = "1.0.0"')"
overlay "$repo" itoa "$(stand_in itoa 1.0.0)"
lock_inside "$repo"
cp "$repo/Cargo.lock" "$work/overlay.before"
if [[ "$(field "$repo" itoa version)" != 1.0.0 || -n "$(field "$repo" itoa source)" ]]; then
  fail "the overlay did not leave itoa 1.0.0 without a source, so the case tests nothing"
elif ! relock "$repo"; then
  fail "the script failed: $(cat "$repo/relock.out")"
elif [[ "$(field "$repo" itoa version)" != 1.0.0 ]]; then
  fail "itoa moved from 1.0.0 to $(field "$repo" itoa version)"
elif [[ "$(field "$repo" itoa source)" != "$registry" || -z "$(field "$repo" itoa checksum)" ]]; then
  fail "itoa has no crates.io source or no checksum"
elif ! only_gained_sources "$work/overlay.before" "$repo/Cargo.lock"; then
  fail "the lockfile changed beyond its source and checksum lines: $(diff "$work/overlay.before" "$repo/Cargo.lock")"
elif ! cargo metadata --manifest-path "$repo/Cargo.toml" --locked --format-version 1 >/dev/null 2>&1; then
  fail "the result does not pass cargo metadata --locked"
else
  pass
fi

case_name="crates that depend on each other are all set back"
repo="$(new_repo chain 'indexmap = "2.0.0"')"
lock_outside "$repo"
pin "$repo" indexmap 2.0.0
pin "$repo" hashbrown 0.14.0
pin "$repo" equivalent 1.0.0
tested="$(versions "$repo")"
strip "$repo" indexmap hashbrown equivalent
cp "$repo/Cargo.lock" "$work/chain.before"
if ! relock "$repo"; then
  fail "the script failed: $(cat "$repo/relock.out")"
elif [[ "$(versions "$repo")" != "$tested" ]]; then
  fail "the locked versions moved: $(diff <(echo "$tested") <(versions "$repo"))"
elif ! only_gained_sources "$work/chain.before" "$repo/Cargo.lock"; then
  fail "the lockfile changed beyond its source and checksum lines: $(diff "$work/chain.before" "$repo/Cargo.lock")"
elif [[ "$(field "$repo" hashbrown source)" != "$registry" || "$(field "$repo" indexmap source)" != "$registry" ]]; then
  fail "hashbrown or indexmap has no crates.io source"
else
  pass
fi

case_name="a checkout ahead of its last release is refused"
repo="$(new_repo ahead 'itoa = "1"')"
overlay "$repo" itoa "$(stand_in itoa 1.9999.0)"
lock_inside "$repo"
cp "$repo/Cargo.lock" "$work/ahead.before"
if [[ "$(field "$repo" itoa version)" != 1.9999.0 ]]; then
  fail "the overlay did not lock itoa 1.9999.0, so the case tests nothing"
elif relock "$repo"; then
  fail "the script accepted it: $(cat "$repo/relock.out")"
elif ! cmp -s "$work/ahead.before" "$repo/Cargo.lock"; then
  fail "the lockfile was not put back: $(diff "$work/ahead.before" "$repo/Cargo.lock")"
elif ! grep -q 'itoa: tested at 1.9999.0' "$repo/relock.out"; then
  fail "the error does not name itoa and its tested version: $(cat "$repo/relock.out")"
else
  pass
fi

case_name="a manifest asking for an unreleased version is refused"
repo="$(new_repo unreleased 'itoa = "1.9999.0"')"
overlay "$repo" itoa "$(stand_in itoa 1.9999.0)"
lock_inside "$repo"
cp "$repo/Cargo.lock" "$work/unreleased.before"
if relock "$repo"; then
  fail "the script accepted it: $(cat "$repo/relock.out")"
elif ! cmp -s "$work/unreleased.before" "$repo/Cargo.lock"; then
  fail "the lockfile was not put back: $(diff "$work/unreleased.before" "$repo/Cargo.lock")"
elif ! grep -q 'does not resolve from crates.io alone' "$repo/relock.out"; then
  fail "the error does not say why: $(cat "$repo/relock.out")"
else
  pass
fi

case_name="a checkout that differs from the release of its version is refused"
# The stand-in claims itoa 1.0.0 but depends on equivalent, which the real
# 1.0.0 does not: every version matches after the relock, and the lockfile
# still is not the one that was tested.
repo="$(new_repo differs 'itoa = "1.0.0"' 'equivalent = "1.0.0"')"
overlay "$repo" itoa "$(stand_in itoa 1.0.0 'equivalent = "1"')"
lock_inside "$repo"
cp "$repo/Cargo.lock" "$work/differs.before"
if [[ "$(field "$repo" itoa version)" != 1.0.0 || -n "$(field "$repo" itoa source)" ]]; then
  fail "the overlay did not leave itoa 1.0.0 without a source, so the case tests nothing"
elif relock "$repo"; then
  fail "the script accepted it: $(cat "$repo/relock.out")"
elif ! cmp -s "$work/differs.before" "$repo/Cargo.lock"; then
  fail "the lockfile was not put back: $(diff "$work/differs.before" "$repo/Cargo.lock")"
elif ! grep -q 'changed in more than' "$repo/relock.out"; then
  fail "the error does not say what changed: $(cat "$repo/relock.out")"
else
  pass
fi

case_name="an unused patch beside a used one is dropped with nothing else"
# Cargo leaves a `[[patch.unused]]` table alone until it rewrites the
# lockfile for another reason, and the relock of a used patch is one.
repo="$(new_repo unused 'itoa = "1.0.0"')"
overlay "$repo" itoa "$(stand_in itoa 1.0.0)" ryu "$(stand_in ryu 1.0.0)"
lock_inside "$repo"
tested="$(versions "$repo")"
cp "$repo/Cargo.lock" "$work/unused.before"
if ! grep -qx '\[\[patch.unused\]\]' "$repo/Cargo.lock" || [[ -n "$(field "$repo" itoa source)" ]]; then
  fail "the overlay did not leave an unused patch and a stripped itoa, so the case tests nothing"
elif ! relock "$repo"; then
  fail "the script failed: $(cat "$repo/relock.out")"
elif grep -qx '\[\[patch.unused\]\]' "$repo/Cargo.lock"; then
  fail "the [[patch.unused]] table is still there"
elif [[ "$(versions "$repo")" != "$tested" ]]; then
  fail "the locked versions moved: $(diff <(echo "$tested") <(versions "$repo"))"
elif [[ "$(field "$repo" itoa source)" != "$registry" ]]; then
  fail "itoa has no crates.io source"
else
  pass
fi

case_name="a lockfile that already resolves is left alone"
repo="$(new_repo clean 'itoa = "1.0.0"')"
lock_outside "$repo"
pin "$repo" itoa 1.0.0
cp "$repo/Cargo.lock" "$work/clean.before"
if ! relock "$repo"; then
  fail "the script failed: $(cat "$repo/relock.out")"
elif ! cmp -s "$work/clean.before" "$repo/Cargo.lock"; then
  fail "the lockfile changed: $(diff "$work/clean.before" "$repo/Cargo.lock")"
else
  pass
fi

finished=1
if ((failures > 0)); then
  echo "$failures case(s) failed." >&2
  exit 1
fi
echo "All relock cases passed."
