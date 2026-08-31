#!/bin/bash
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2025 Cyril Jacquet
set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status()  { echo -e "${GREEN}✓${NC} $1"; }
print_info()    { echo -e "${BLUE}ℹ${NC} $1"; }
print_warning() { echo -e "${YELLOW}⚠${NC} $1"; }

echo "🚀 Setting up the Skribisto development environment..."

print_info "Toolchain:"
echo "  $(rustc --version)"
echo "  $(cargo --version)"

# teksilo is a sibling path dependency; without it nothing in the workspace
# resolves, so fail loudly rather than let `cargo build` produce a confusing
# "failed to load manifest" much later.
if [ -d /workspaces/teksilo ]; then
    print_status "teksilo found at /workspaces/teksilo"
else
    print_warning "teksilo NOT found at /workspaces/teksilo"
    echo "    Skribisto depends on it through a path dependency. Clone it next to"
    echo "    skribisto on the host and rebuild the container:"
    echo "        git clone https://github.com/ferntech-eu/teksilo"
fi

print_info "Warming the dependency cache (cargo fetch)..."
if cargo fetch --locked; then
    print_status "Dependencies fetched"
else
    print_warning "cargo fetch failed — check the teksilo checkout above"
fi

echo
print_info "Build and run with:"
echo "    cargo build -p teksilo_ui"
echo "    cargo run   -p teksilo_ui"
echo "    cargo test"
