#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
#
# cargo-release's pre-release hook for teksilo_ui. A package takes a single
# hook command, so this runs the two steps in order: refuse a lockfile that
# does not resolve from crates.io (release-preflight.sh), then write the
# metainfo's release notes (news-to-metainfo.sh). Both read cargo-release's
# environment themselves.

set -euo pipefail

here="$(dirname "$0")"
"$here/release-preflight.sh"
exec "$here/news-to-metainfo.sh" "$@"
