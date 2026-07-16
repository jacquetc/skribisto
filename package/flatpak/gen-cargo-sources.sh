#!/usr/bin/env bash
# Generate package/flatpak/cargo-sources.json for the offline Flatpak build.
#
# WHY: flatpak-builder builds with no network, so every crate must be vendored.
# flatpak-cargo-generator.py turns a Cargo.lock into a Flatpak `sources` list.
# But our committed Cargo.lock resolves the external siblings (bastyde*,
# text-document) from LOCAL PATHS, which cannot be vendored from crates.io. So we
# first strip the `../` path attrs (leaving each dep's `version =`) and
# regenerate the lockfile so those deps re-resolve from crates.io, THEN vendor.
# This is why bastyde*/bastyde-charts/text-document must be published (see the
# migration plan's prerequisites P1/P2) before this script can succeed.
#
# This MODIFIES Cargo.toml (strip) and Cargo.lock (regenerate) IN PLACE. In CI
# that is a throwaway checkout. For a LOCAL run, do it on a scratch clone, or run
#   git checkout -- Cargo.lock Cargo.toml 'crates/*/Cargo.toml'
# afterwards to restore the path deps for day-to-day development.
#
# Requires: python3 + pip (aiohttp, toml), cargo, and network access.
# Usage (from anywhere in the repo):  ./package/flatpak/gen-cargo-sources.sh
set -euo pipefail

# flatpak-builder-tools has no tagged releases — PIN this to a reviewed commit
# SHA of flatpak/flatpak-builder-tools before relying on it in a release.
FBT_REF="${FBT_REF:-master}"
GEN_URL="https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/${FBT_REF}/cargo/flatpak-cargo-generator.py"

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

echo "==> Stripping external ../ path deps (bastyde*/text-document -> crates.io)"
shopt -s nullglob
for f in Cargo.toml crates/*/Cargo.toml; do
  sed -i.bak -E \
    -e 's#, *path = "\.\.[^"]*"##g' \
    -e 's#path = "\.\.[^"]*", *##g' \
    "$f"
  rm -f "$f.bak"
done

echo "==> Regenerating Cargo.lock against crates.io"
cargo generate-lockfile

echo "==> Fetching flatpak-cargo-generator.py (ref: ${FBT_REF})"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
curl -fsSL "$GEN_URL" -o "$tmp/flatpak-cargo-generator.py"

echo "==> Ensuring python deps (aiohttp, toml)"
python3 -m pip install --quiet --user aiohttp toml

echo "==> Generating package/flatpak/cargo-sources.json"
python3 "$tmp/flatpak-cargo-generator.py" Cargo.lock -o package/flatpak/cargo-sources.json

echo "==> Done: package/flatpak/cargo-sources.json"
