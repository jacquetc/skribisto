# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# The cross toolchain for package/linux-aarch64/build.py.
#
# The image carries compilers and nothing else. Rust, the cargo registry and the
# workspace are mounted in from the host at run time, so this image does not go
# stale when the toolchain moves.
#
# The base distribution sets the glibc floor of everything built here: ubuntu
# 24.04 means the binary needs glibc 2.39 or newer, which is what the project's
# own x86_64 release already requires (it is built on ubuntu-latest). Rebuild
# with `--build-arg BASE=debian:bookworm` for a 2.36 floor, which reaches
# Raspberry Pi OS bookworm and Ubuntu 22.04.
ARG BASE=ubuntu:24.04
FROM ${BASE}

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      # Host compilers. Build scripts and proc macros are still compiled for
      # x86_64, and without `cc` the build dies at the first one of them.
      gcc g++ libc6-dev \
      # The repo's .cargo/config.toml (gitignored, per-machine) selects mold for
      # the host triple. That file is visible through the workspace mount, so a
      # container without mold fails at the first host link.
      mold \
      # The cross toolchain proper. rusqlite builds SQLite from source, and
      # zstd-sys and ring compile C and assembly, so a C cross compiler and the
      # aarch64 glibc are required, not optional.
      gcc-aarch64-linux-gnu g++-aarch64-linux-gnu libc6-dev-arm64-cross \
      binutils-aarch64-linux-gnu \
      # Runs the finished binary on this x86_64 host for the smoke test.
      qemu-user \
      # Build-script staples.
      pkg-config make perl git ca-certificates \
 && rm -rf /var/lib/apt/lists/*
