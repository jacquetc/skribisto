#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
# Rewrite Cargo.lock so it resolves from crates.io alone.
#
# WHY THIS EXISTS: teksilo*/text-document are pinned by version in the workspace
# manifest, and a developer's gitignored `.cargo/config.toml` overlays them onto
# the local sibling checkouts through `[patch.crates-io]`. Cargo records a
# patched package as a PATH package, which means it strips the `source` and
# `checksum` lines out of that package's Cargo.lock entry. So every local build
# with the overlay active rewrites the lock into a form that resolves on this
# machine and nowhere else.
#
# Committing that is silent. Nothing in a normal build, test or CI leg fails,
# because they all re-resolve from crates.io on the fly. The one thing that
# cannot is the Flatpak vendoring step, which passes `--locked` (flatpak-builder
# builds offline, so the whole crate graph must be vendored ahead of time). That
# is where v3.0.4's release died -- after the tag was cut, with every other
# artifact already building.
#
# Run this before committing a Cargo.lock touched by a build that had the
# overlay active. The `lockfile` job in ci.yml is the backstop if you forget.
#
# Requires: cargo and network access.
# Usage (from anywhere in the repo):  ./scripts/relock-crates-io.sh
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

overlay=".cargo/config.toml"
overlay_bak=".cargo/config.toml.relock-bak"

cleanup() {
  if [[ -f "$overlay_bak" ]]; then
    mv -f "$overlay_bak" "$overlay"
    echo "==> Restored $overlay"
  fi
}
trap cleanup EXIT

if [[ -f "$overlay" ]]; then
  echo "==> Parking local $overlay so siblings resolve from crates.io"
  mv "$overlay" "$overlay_bak"
fi

# Deliberately a plain resolve, NOT `cargo generate-lockfile` and NOT
# `cargo update`. Both re-resolve every dependency to the newest compatible
# version; this one keeps every already-locked version and only fills in what
# the parked overlay left unresolved -- the `source` and `checksum` lines of the
# sibling crates. A release lock should differ from the tested one by exactly
# that and nothing else.
echo "==> Resolving Cargo.lock against crates.io"
cargo metadata --format-version 1 >/dev/null

echo "==> Verifying the result needs no further change"
cargo metadata --locked --format-version 1 >/dev/null

echo "==> Done. Review and commit Cargo.lock:"
git --no-pager diff --stat Cargo.lock
