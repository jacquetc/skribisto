# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2025 Cyril Jacquet

FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# The winit/wgpu display-server libraries Skribisto needs to build on Linux.
# Kept in step with .github/actions/install-linux-deps/action.yml — if you add a
# package there, add it here too.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libxkbcommon-dev \
        libwayland-dev \
        libxcb1-dev \
        libx11-dev && \
    rm -rf /var/lib/apt/lists/*

CMD ["/bin/bash"]
