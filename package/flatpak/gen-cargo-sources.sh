#!/usr/bin/env bash
# Generate package/flatpak/cargo-sources.json for the offline Flatpak build.
#
# WHY: flatpak-builder builds with no network, so every crate must be vendored.
# flatpak-cargo-generator.py turns a Cargo.lock into a Flatpak `sources` list.
# Manifests pin teksilo*/text-document by version (local sibling checkouts are a
# gitignored `[patch.crates-io]` overlay in `.cargo/config.toml`), so this
# script vendors the crates.io graph. Those crates must be published before
# this can succeed.
#
# The COMMITTED Cargo.lock is what gets vendored. It is deliberately not
# regenerated: see the note above the verification step below.
#
# A local `.cargo/config.toml` overlay would redirect those crates back onto
# path checkouts, which cannot be vendored. This script parks that file for
# the duration of the run (and restores it on exit) so the lock is read as
# crates.io. CI has no overlay to park.
#
# Requires: python3 (with venv), cargo, and network access.
# Usage (from anywhere in the repo):  ./package/flatpak/gen-cargo-sources.sh
set -euo pipefail

# flatpak-builder-tools has no tagged releases, so this is pinned to a reviewed
# commit SHA. Tracking `master` is not merely a supply-chain risk here: the
# generator's own dependency set changed under us (`toml` -> `tomlkit`), which
# broke every run of this script until the pin caught up. Override with
# FBT_REF=master to test a newer generator, then move the pin.
FBT_REF="${FBT_REF:-1fc32195e3e60fe5c97f0af646dec7a99df5962b}"
GEN_URL="https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/${FBT_REF}/cargo/flatpak-cargo-generator.py"

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

overlay=".cargo/config.toml"
overlay_bak=".cargo/config.toml.vendoring-bak"
tmp=""
cleanup() {
  if [[ -n "$tmp" ]]; then
    rm -rf "$tmp"
  fi
  if [[ -f "$overlay_bak" ]]; then
    mv -f "$overlay_bak" "$overlay"
  fi
}
trap cleanup EXIT

if [[ -f "$overlay" ]]; then
  echo "==> Parking local $overlay so siblings resolve from crates.io"
  mv "$overlay" "$overlay_bak"
fi

# Deliberately NOT `cargo generate-lockfile`. That re-resolves every dependency
# to the newest compatible version, which has two consequences a release build
# must not have. The artifact would be built from a dependency set no test ever
# ran against, and that set is chosen by the HOST toolchain while the sandbox
# compiles with the SDK extension's rustc, which lags stable. The two agreeing is
# luck, and the day they diverge the lock can name crates the sandbox cannot
# build. `--locked` instead asserts the committed lock is already complete and
# needs no change, which is the only case the regeneration ever covered.
echo "==> Verifying the committed Cargo.lock resolves everything from crates.io"
cargo metadata --locked --format-version 1 >/dev/null

echo "==> Fetching flatpak-cargo-generator.py (ref: ${FBT_REF})"
tmp="$(mktemp -d)"
curl -fsSL "$GEN_URL" -o "$tmp/flatpak-cargo-generator.py"

# The dependency list is the generator's own PEP-723 header, not a guess:
# aiohttp, tomlkit and PyYAML. A throwaway venv rather than `pip install --user`,
# which a PEP-668 "externally managed" python (Debian/Ubuntu, and the GitHub
# runner image) refuses outright.
echo "==> Ensuring python deps in a throwaway venv (aiohttp, tomlkit, PyYAML)"
python3 -m venv "$tmp/venv"
"$tmp/venv/bin/pip" install --quiet aiohttp tomlkit PyYAML

echo "==> Generating package/flatpak/cargo-sources.json"
"$tmp/venv/bin/python" "$tmp/flatpak-cargo-generator.py" Cargo.lock -o package/flatpak/cargo-sources.json

echo "==> Done: package/flatpak/cargo-sources.json"
