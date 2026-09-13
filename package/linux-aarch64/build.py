#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Cross-build the Linux ARM64 (aarch64) release of Skribisto from an x86_64 host.

Produces, into ``dist/`` by default:

    skribisto                                  the aarch64 release binary
    Skribisto-<version>-aarch64-linux.tar.gz   with --tarball: the same layout
                                               the Linux release job ships

The compile runs inside a throwaway Docker image, built from the Dockerfile
beside this script. A cross toolchain is a system package, and installing one on
the host is not this script's to do. Nothing else about the host changes: the
rust toolchain, the cargo registry and the workspace are mounted in, and the
build directory defaults to ``~/.cache/skribisto-cross-aarch64`` so the host's
own ``target/`` is left alone.

Prerequisites:

    docker, usable without sudo (`docker info` has to succeed)
    rustup    (the aarch64 std is added for you if it is missing)

A C cross compiler is a real requirement rather than a convenience: rusqlite
builds SQLite from source, and zstd-sys and ring compile C and assembly as well.
Wayland, X11 and xkbcommon are opened with dlopen at run time, so they have to
be present on the machine that runs the binary, not on the one that builds it.

Three things fail quietly here rather than loudly, so the finished binary is
inspected instead of trusted:

  * Drop ``--target`` and cargo still succeeds, having built for the host. An
    x86-64 binary in ``dist/`` looks exactly like an aarch64 one until somebody
    runs it, so the ELF machine type is checked.
  * The glibc floor is inherited from the image's base distribution, and a
    binary needing a newer glibc than the target machine has fails at startup
    inside the loader. The floor is always reported, and ``--max-glibc`` turns
    an expectation about it into a check.
  * A dependency that links a shared library instead of dlopening it adds a
    NEEDED entry no tarball carries. Anything outside the base set is refused,
    and ``--allow-needed`` accepts one deliberately.

The binary is then run under qemu-user with a ``--style`` value it has to
reject, which fails in the argument parser before any attempt to open a window.
That is the difference between a file that exists and a binary that runs.
``--no-smoke-test`` skips it, ``--check-only`` runs the inspection and the smoke
test against ``--binary`` and stops there.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import textwrap
from pathlib import Path

# package/linux-aarch64/build.py -> package/linux-aarch64 -> package -> <repo root>
HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]

DOCKERFILE = HERE / "Dockerfile"
DEFAULT_IMAGE = "skribisto-cross-aarch64"

TARGET = "aarch64-unknown-linux-gnu"
CARGO_PACKAGE = "teksilo_ui"
BIN_NAME = "skribisto"

# ELF facts this build has to hold to. EM_AARCH64 is 183; the host's own x86-64
# is 62, which is the value that shows up when --target goes missing.
EM_AARCH64 = 183
ELF_MACHINES = {62: "x86-64", 183: "aarch64", 40: "arm (32-bit)", 243: "riscv"}
EXPECTED_INTERP = "/lib/ld-linux-aarch64.so.1"

# What a correct build links against. Everything the app needs from the desktop
# (wayland, xkbcommon, x11) is dlopened by winit, so it is absent from here on
# purpose: a name appearing in this list that is not in that set is a new hard
# dependency on a library the target machine may not have.
BASE_NEEDED = frozenset(
    {"libc.so.6", "libm.so.6", "libgcc_s.so.1", "ld-linux-aarch64.so.1"}
)

# Icon sizes the release stages. Missing ones are skipped, as in release.yml.
ICON_SIZES = (16, 32, 64, 72, 96, 192, 256, 512)


class BuildError(Exception):
    """Anything that should stop the build with a readable message."""


# --------------------------------------------------------------------------
# small helpers


def log(msg: str) -> None:
    print(f"==> {msg}", flush=True)


def run(cmd: list[str], *, cwd: Path | None = None) -> None:
    log(" ".join(cmd))
    result = subprocess.run(cmd, cwd=str(cwd) if cwd else None)
    if result.returncode != 0:
        raise BuildError(f"{Path(cmd[0]).name} exited with {result.returncode}")


def capture(cmd: list[str], *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        capture_output=True,
        text=True,
        check=False,
    )


def git(*args: str) -> str | None:
    """Trimmed stdout of a git command, or None if it fails or is empty."""
    try:
        out = capture(["git", *args], cwd=REPO)
    except OSError:
        return None
    if out.returncode != 0:
        return None
    return out.stdout.strip() or None


def which(name: str, hint: str) -> str:
    found = shutil.which(name)
    if not found:
        raise BuildError(f"{name} not found on PATH. {hint}")
    return found


# --------------------------------------------------------------------------
# host prerequisites


def check_docker() -> str:
    docker = which("docker", "Install Docker, or build elsewhere.")
    probe = capture([docker, "info"])
    if probe.returncode != 0:
        raise BuildError(
            "docker is installed but not usable by this user. Start the daemon, or add "
            "yourself to the `docker` group and log in again. `docker info` said:\n"
            + (probe.stderr.strip() or probe.stdout.strip())
        )
    return docker


def ensure_rust_target() -> Path:
    """Make sure the aarch64 std is installed, and return the cargo to run.

    The container runs the host's rust through a mount, so the target's standard
    library has to exist on the host, and the check is against the exact binary
    the container will invoke rather than against whatever is on PATH here.
    Adding a target is one cheap, reversible, user-level command, so it is done
    rather than demanded.
    """
    cargo_home = Path(os.environ.get("CARGO_HOME") or Path.home() / ".cargo").resolve()
    cargo = cargo_home / "bin" / "cargo"
    if not cargo.is_file():
        raise BuildError(
            f"{cargo} does not exist. This script mounts the host's rust into the "
            "container, so cargo has to live under CARGO_HOME (rustup's default is "
            "~/.cargo). Install rust with rustup: https://rustup.rs"
        )
    rustup = shutil.which("rustup")
    if not rustup:
        raise BuildError(
            "rustup not found. This script mounts the host's rust into the container, "
            f"so the {TARGET} standard library has to be installed with rustup. A "
            "distribution-packaged cargo cannot provide it."
        )

    installed = capture([rustup, "target", "list", "--installed"])
    if installed.returncode == 0 and TARGET in installed.stdout.split():
        return cargo

    log(f"adding the {TARGET} standard library")
    run([rustup, "target", "add", TARGET])
    return cargo


def external_path_deps(cargo: Path) -> list[Path]:
    """Workspace roots of path dependencies that live outside this repo.

    Sibling checkouts of teksilo and text-document are addressed with a leading
    ``..`` in the crate manifests, exactly the shape .github/actions/strip-path-deps
    removes for a crates.io build. They have to be mounted at their own absolute
    paths, or cargo resolves them to nothing inside the container.

    The list comes from ``cargo metadata`` rather than from reading this repo's
    manifests, because it has to be transitive: teksilo has a sibling of its own
    (text-typeset), and a mount list built from this repo alone stops one
    repository short, with the failure arriving only inside the container.

    What gets mounted is the outermost enclosing directory holding a Cargo.toml,
    not the crate directory: a leaf manifest saying ``workspace = true`` cannot
    be read without its workspace root.
    """
    out = capture([str(cargo), "metadata", "--format-version", "1"], cwd=REPO)
    if out.returncode != 0:
        raise BuildError(
            "cargo metadata failed, so the list of directories to mount cannot be "
            "built:\n" + (out.stderr.strip() or out.stdout.strip())
        )

    home = Path.home().resolve()
    roots: list[Path] = []
    for package in json.loads(out.stdout).get("packages", []):
        # A registry or git dependency carries a source. A path one does not.
        if package.get("source"):
            continue
        crate = Path(package["manifest_path"]).resolve().parent
        if crate == REPO or REPO in crate.parents:
            continue
        root = crate
        for parent in crate.parents:
            # Capped at the home directory so a stray Cargo.toml further up can
            # never turn into a mount of /home or /.
            if parent in (home, home.parent) or parent == Path(parent.root):
                break
            if (parent / "Cargo.toml").is_file():
                root = parent
        if root not in roots:
            roots.append(root)
    return sorted(roots)


# --------------------------------------------------------------------------
# ELF inspection
#
# Enough of the format to answer three questions: which machine is this built
# for, which shared libraries does it need, and which symbol versions does it
# demand from them. Reading the bytes here keeps the check on the host, where
# binutils for the target is not installed.


def _u16(data: bytes, off: int) -> int:
    return int.from_bytes(data[off : off + 2], "little")


def _u32(data: bytes, off: int) -> int:
    return int.from_bytes(data[off : off + 4], "little")


def _u64(data: bytes, off: int) -> int:
    return int.from_bytes(data[off : off + 8], "little")


def _cstr(data: bytes, off: int) -> str:
    end = data.find(b"\0", off)
    return data[off:end].decode("utf-8", "replace")


class ElfInfo:
    def __init__(self, path: Path) -> None:
        data = path.read_bytes()
        if data[:4] != b"\x7fELF":
            raise BuildError(f"{path} is not an ELF image")
        if data[4] != 2 or data[5] != 1:
            raise BuildError(f"{path} is not a little-endian 64-bit ELF image")

        self.path = path
        self.machine = _u16(data, 0x12)
        self.interpreter: str | None = None
        self.needed: list[str] = []
        # (library, version, is_weak) for every symbol version required.
        self.version_needs: list[tuple[str, str, bool]] = []

        phoff, phentsize, phnum = _u64(data, 0x20), _u16(data, 0x36), _u16(data, 0x38)
        for i in range(phnum):
            entry = phoff + i * phentsize
            if _u32(data, entry) == 3:  # PT_INTERP
                self.interpreter = _cstr(data, _u64(data, entry + 8))

        shoff, shentsize = _u64(data, 0x28), _u16(data, 0x3A)
        shnum, shstrndx = _u16(data, 0x3C), _u16(data, 0x3E)
        if shnum == 0:
            raise BuildError(
                f"{path} has no section table, so it cannot be checked. It was "
                "stripped with --strip-sections or by a tool that removes them."
            )

        sections = []
        for i in range(shnum):
            entry = shoff + i * shentsize
            sections.append(
                {
                    "name_off": _u32(data, entry),
                    "offset": _u64(data, entry + 24),
                    "size": _u64(data, entry + 32),
                    "link": _u32(data, entry + 40),
                }
            )
        names_at = sections[shstrndx]["offset"]
        for section in sections:
            section["name"] = _cstr(data, names_at + section["name_off"])
        by_name = {section["name"]: section for section in sections}

        dynamic, dynstr = by_name.get(".dynamic"), by_name.get(".dynstr")
        if dynamic and dynstr:
            strings = dynstr["offset"]
            for off in range(dynamic["offset"], dynamic["offset"] + dynamic["size"], 16):
                tag, value = _u64(data, off), _u64(data, off + 8)
                if tag == 0:  # DT_NULL
                    break
                if tag == 1:  # DT_NEEDED
                    self.needed.append(_cstr(data, strings + value))

        verneed = by_name.get(".gnu.version_r")
        if verneed and dynstr:
            link = verneed["link"]
            strings = sections[link]["offset"] if link < len(sections) else dynstr["offset"]
            off = verneed["offset"]
            while True:
                count = _u16(data, off + 2)
                library = _cstr(data, strings + _u32(data, off + 4))
                aux = off + _u32(data, off + 8)
                nxt = _u32(data, off + 12)
                for _ in range(count):
                    flags = _u16(data, aux + 4)
                    name = _cstr(data, strings + _u32(data, aux + 8))
                    # VER_FLG_WEAK (0x2) makes the requirement optional for the
                    # loader. Without it a missing version is a hard failure,
                    # even when every symbol using it is itself weak.
                    self.version_needs.append((library, name, bool(flags & 0x2)))
                    aux_next = _u32(data, aux + 12)
                    if not aux_next:
                        break
                    aux += aux_next
                if not nxt:
                    break
                off += nxt

    @property
    def machine_name(self) -> str:
        return ELF_MACHINES.get(self.machine, f"unknown (e_machine={self.machine})")

    def glibc_floor(self) -> tuple[int, ...] | None:
        """Highest non-optional GLIBC_x.y this binary demands, as a tuple."""
        versions = [
            tuple(int(part) for part in name.removeprefix("GLIBC_").split("."))
            for _, name, weak in self.version_needs
            if name.startswith("GLIBC_") and not weak and name[6:].replace(".", "").isdigit()
        ]
        return max(versions) if versions else None


def check_binary(binary: Path, allow_needed: set[str], max_glibc: tuple[int, ...] | None) -> None:
    """Refuse a binary that is the wrong shape, and report the glibc floor."""
    elf = ElfInfo(binary)

    if elf.machine != EM_AARCH64:
        raise BuildError(
            f"{binary} is a {elf.machine_name} binary, not aarch64. A cargo build that "
            f"loses --target {TARGET} still succeeds, and produces exactly this."
        )
    if elf.interpreter and elf.interpreter != EXPECTED_INTERP:
        raise BuildError(
            f"{binary} asks for the interpreter {elf.interpreter}, not {EXPECTED_INTERP}."
        )

    unexpected = [lib for lib in elf.needed if lib not in BASE_NEEDED and lib not in allow_needed]
    if unexpected:
        raise BuildError(
            f"{binary.name} links {', '.join(unexpected)}, which is outside the set this "
            "build is meant to need. A dependency is now linking a shared library "
            "instead of dlopening it, and the machine running the binary has to supply "
            "it. Accept it with --allow-needed <name> once that is understood."
        )

    floor = elf.glibc_floor()
    floor_text = ".".join(str(part) for part in floor) if floor else "none"
    log(f"checked {binary.name}: aarch64, needs {', '.join(elf.needed)}")
    log(f"glibc floor: {floor_text}")

    optional = sorted(
        {name for _, name, weak in elf.version_needs if weak and name.startswith("GLIBC_")}
    )
    if optional:
        log(f"optional glibc versions (loader tolerates their absence): {', '.join(optional)}")

    if max_glibc and floor and floor > max_glibc:
        wanted = ".".join(str(part) for part in max_glibc)
        raise BuildError(
            f"{binary.name} requires glibc {floor_text}, above the {wanted} asked for with "
            "--max-glibc. The floor comes from the image's base distribution: rebuild the "
            "image with an older one, for example --build-arg BASE=debian:bookworm for 2.36."
        )


def smoke_test(docker: str, image: str, binary: Path) -> None:
    """Prove the binary runs, without a display.

    An unknown --style is rejected in the argument parser, before the app tries
    to open a window, so this exercises real aarch64 code start to finish and
    still works in a container with no compositor.
    """
    mount = binary.parent
    result = capture(
        [
            docker, "run", "--rm",
            "--user", f"{os.getuid()}:{os.getgid()}",
            "-e", "HOME=/tmp",
            "-v", f"{mount}:{mount}:ro",
            "-w", "/tmp",
            image,
            "qemu-aarch64", "-L", "/usr/aarch64-linux-gnu",
            str(binary),
            "--style", "no-such-style",
        ]
    )
    output = (result.stderr + result.stdout).strip()
    if "unknown style" not in output:
        raise BuildError(
            "the binary did not run under qemu-aarch64. Expected it to reject "
            f"`--style no-such-style`, but it exited with {result.returncode} and said:\n"
            f"{output or '(no output)'}"
        )
    log(f"smoke test: ran under qemu-aarch64 and rejected the bad argument ({output})")


# --------------------------------------------------------------------------
# steps


def resolve_version(args: argparse.Namespace) -> tuple[str, str]:
    """Return (version, SKRIBISTO_GIT_DESCRIBE value).

    The release version is a git fact, not a Cargo one: every crate shares the
    same placeholder, so the tag is the only thing that names a release.
    """
    describe = args.git_describe or git("describe", "--tags", "--always", "--abbrev=9")
    version = args.version or (describe.lstrip("v") if describe else "0.0.0-dev")
    return version, (describe or f"v{version}")


def build_image(docker: str, args: argparse.Namespace) -> None:
    if args.skip_image:
        log(f"reusing the existing image {args.image}")
        return
    cmd = [docker, "build", "-t", args.image, "-f", str(DOCKERFILE)]
    if args.base_image:
        cmd += ["--build-arg", f"BASE={args.base_image}"]
    cmd.append(str(HERE))
    run(cmd)


def build_binary(docker: str, cargo: Path, args: argparse.Namespace, describe: str) -> Path:
    target_dir = Path(args.target_dir).resolve()
    target_dir.mkdir(parents=True, exist_ok=True)

    cargo_home = cargo.parents[1]
    rustup_home = Path(os.environ.get("RUSTUP_HOME") or Path.home() / ".rustup").resolve()

    externals = external_path_deps(cargo)
    if externals:
        log("sibling path dependencies: " + ", ".join(str(path) for path in externals))
    mounts: list[Path] = [REPO, *externals, cargo_home, rustup_home, target_dir]

    seen: set[Path] = set()
    mount_args: list[str] = []
    for path in mounts:
        if path in seen:
            continue
        seen.add(path)
        # Same path inside as outside: the manifests address each other with
        # relative paths, and only identical layouts keep those resolvable.
        mount_args += ["-v", f"{path}:{path}"]

    env = {
        "HOME": str(Path.home()),
        "CARGO_HOME": str(cargo_home),
        "RUSTUP_HOME": str(rustup_home),
        "CARGO_TARGET_DIR": str(target_dir),
        "PATH": f"{cargo_home / 'bin'}:/usr/local/bin:/usr/bin:/bin",
        # The target's linker and C compiler. The host triple is left alone, so
        # the .cargo/config.toml choice of mold still applies to build scripts.
        f"CARGO_TARGET_{TARGET.upper().replace('-', '_')}_LINKER": "aarch64-linux-gnu-gcc",
        f"CC_{TARGET.replace('-', '_')}": "aarch64-linux-gnu-gcc",
        f"CXX_{TARGET.replace('-', '_')}": "aarch64-linux-gnu-g++",
        f"AR_{TARGET.replace('-', '_')}": "aarch64-linux-gnu-ar",
        # Read by crates/teksilo_ui/build.rs. Set here so the stamp does not
        # depend on git being usable inside the container.
        "SKRIBISTO_GIT_DESCRIBE": describe,
        "SKRIBISTO_CHANNEL": args.channel,
    }
    env_args: list[str] = []
    for key, value in env.items():
        env_args += ["-e", f"{key}={value}"]

    cmd = [
        docker, "run", "--rm",
        # Build as the invoking user, so nothing in the workspace, the registry
        # or the build directory comes back owned by root.
        "--user", f"{os.getuid()}:{os.getgid()}",
        *mount_args,
        *env_args,
        "-w", str(REPO),
        args.image,
        # An absolute path, because the container resolves argv[0] itself and
        # the rustup shims live in the mounted CARGO_HOME.
        str(cargo),
        "build", "--release", "--target", TARGET, "-p", CARGO_PACKAGE,
    ]
    if args.features:
        cmd += ["--features", args.features]

    run(cmd)

    binary = target_dir / TARGET / "release" / BIN_NAME
    if not binary.is_file():
        raise BuildError(f"expected {binary} after the build, but it is missing")
    return binary


def stage_tarball(binary: Path, out_dir: Path, version: str) -> Path:
    """Write the tarball the Linux release ships, with this binary in it.

    The icons are installed under the names the .desktop file and the MIME entry
    look up (eu.skribisto.skribisto), not under their source name. An icon filed
    under a name nothing asks for is invisible, and silently so.
    """
    root = f"Skribisto-{version}-aarch64-linux"
    stage = out_dir / root
    if stage.exists():
        shutil.rmtree(stage)

    def place(src: Path, rel: str, mode: int = 0o644) -> None:
        dest = stage / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)
        dest.chmod(mode)

    icons = REPO / "resources" / "unix" / "icons" / "icons" / "hicolor"
    place(binary, BIN_NAME, 0o755)
    place(REPO / "COPYING", "COPYING")
    place(REPO / "README.md", "README.md")
    place(
        REPO / "resources/unix/applications/eu.skribisto.skribisto.desktop",
        "share/applications/eu.skribisto.skribisto.desktop",
    )
    place(
        REPO / "resources/unix/mime/eu.skribisto.skribisto.xml",
        "share/mime/packages/eu.skribisto.skribisto.xml",
    )
    svg = icons / "scalable" / "apps" / "skribisto.svg"
    place(svg, "share/icons/hicolor/scalable/apps/eu.skribisto.skribisto.svg")
    place(svg, "share/icons/hicolor/scalable/mimetypes/eu.skribisto.skribisto.project.svg")
    for size in ICON_SIZES:
        png = icons / f"{size}x{size}" / "apps" / "skribisto.png"
        if not png.is_file():
            continue
        place(png, f"share/icons/hicolor/{size}x{size}/apps/eu.skribisto.skribisto.png")
        place(
            png,
            f"share/icons/hicolor/{size}x{size}/mimetypes/eu.skribisto.skribisto.project.png",
        )

    # Exec=skribisto expects the binary on PATH, so unpacking share/ is not
    # enough on its own. Ten lines that do the user-local install beat a
    # paragraph telling somebody to do it by hand.
    installer = stage / "install.sh"
    installer.write_text(
        textwrap.dedent(
            """\
            #!/bin/sh
            # Install Skribisto for the current user (default prefix ~/.local).
            # Usage: ./install.sh [prefix]
            set -eu
            prefix="${1:-$HOME/.local}"
            here="$(cd "$(dirname "$0")" && pwd)"
            install -Dm755 "$here/skribisto" "$prefix/bin/skribisto"
            mkdir -p "$prefix/share"
            cp -r "$here/share/." "$prefix/share/"
            update-desktop-database "$prefix/share/applications" 2>/dev/null || true
            update-mime-database "$prefix/share/mime" 2>/dev/null || true
            gtk-update-icon-cache -f -t "$prefix/share/icons/hicolor" 2>/dev/null || true
            printf 'Installed to %s\\n' "$prefix"
            case ":$PATH:" in
              *":$prefix/bin:"*) ;;
              *) printf 'Note: %s/bin is not on your PATH.\\n' "$prefix" ;;
            esac
            """
        ),
        encoding="utf-8",
    )
    installer.chmod(0o755)

    archive = out_dir / f"{root}.tar.gz"
    log(f"writing {archive.name}")

    def reset(entry: tarfile.TarInfo) -> tarfile.TarInfo:
        # Ownership of a downloaded tarball means nothing to the machine that
        # unpacks it, and leaving the builder's uid in there is noise.
        entry.uid = entry.gid = 0
        entry.uname = entry.gname = "root"
        return entry

    with tarfile.open(archive, "w:gz") as tar:
        tar.add(stage, arcname=root, filter=reset)
    return archive


# --------------------------------------------------------------------------


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--version",
        help="release version, e.g. 3.0.2. Defaults to `git describe` with any leading v "
        "stripped.",
    )
    parser.add_argument(
        "--git-describe",
        help="value for SKRIBISTO_GIT_DESCRIBE, which build.rs stamps into the binary. "
        "Defaults to `git describe`. Set this in CI, where a shallow checkout may leave "
        "git unable to name the tag itself.",
    )
    parser.add_argument(
        "--features", default="pdf", help="cargo features for teksilo_ui (default: pdf)"
    )
    parser.add_argument(
        "--channel",
        default="tarball",
        help="value for SKRIBISTO_CHANNEL, which tells the running app how it was "
        "installed and whether to offer updates (default: tarball)",
    )
    parser.add_argument(
        "--out-dir", default=str(REPO / "dist"), help="where to put the artifacts (default: dist/)"
    )
    parser.add_argument(
        "--target-dir",
        default=str(Path.home() / ".cache" / "skribisto-cross-aarch64"),
        help="cargo build directory, kept out of the repo so the host's own target/ is "
        "untouched (default: ~/.cache/skribisto-cross-aarch64)",
    )
    parser.add_argument("--image", default=DEFAULT_IMAGE, help=f"(default: {DEFAULT_IMAGE})")
    parser.add_argument(
        "--base-image",
        help="base distribution for the toolchain image, which sets the glibc floor "
        "(default: the Dockerfile's ubuntu:24.04, floor 2.39; debian:bookworm gives 2.36)",
    )
    parser.add_argument(
        "--skip-image", action="store_true", help="do not rebuild the toolchain image"
    )
    parser.add_argument(
        "--binary", help="use this prebuilt aarch64 binary instead of compiling one"
    )
    parser.add_argument("--tarball", action="store_true", help="also write the release tarball")
    parser.add_argument(
        "--allow-needed",
        action="append",
        default=[],
        metavar="LIB",
        help="accept a shared library outside the expected set, e.g. libfoo.so.1 "
        "(repeatable)",
    )
    parser.add_argument(
        "--max-glibc",
        metavar="X.Y",
        help="fail if the binary needs a glibc newer than this, e.g. 2.36",
    )
    parser.add_argument(
        "--no-smoke-test", action="store_true", help="do not run the binary under qemu-user"
    )
    parser.add_argument(
        "--check-only",
        action="store_true",
        help="only inspect and smoke-test --binary, then exit",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)

    try:
        max_glibc = None
        if args.max_glibc:
            if not re.fullmatch(r"\d+(\.\d+)*", args.max_glibc):
                raise BuildError(f"--max-glibc takes a version like 2.36, not {args.max_glibc!r}")
            max_glibc = tuple(int(part) for part in args.max_glibc.split("."))
        allow_needed = set(args.allow_needed)

        docker = check_docker()

        if args.check_only:
            if not args.binary:
                raise BuildError("--check-only needs --binary")
            binary = Path(args.binary).resolve()
            if not binary.is_file():
                raise BuildError(f"{binary} does not exist")
            check_binary(binary, allow_needed, max_glibc)
            if not args.no_smoke_test:
                build_image(docker, args)
                smoke_test(docker, args.image, binary)
            return 0

        version, describe = resolve_version(args)
        log(f"target {TARGET}, version {version} (SKRIBISTO_GIT_DESCRIBE={describe})")

        out_dir = Path(args.out_dir).resolve()
        out_dir.mkdir(parents=True, exist_ok=True)

        if args.binary:
            binary = Path(args.binary).resolve()
            if not binary.is_file():
                raise BuildError(f"{binary} does not exist")
            if not args.no_smoke_test:
                build_image(docker, args)
        else:
            cargo = ensure_rust_target()
            build_image(docker, args)
            binary = build_binary(docker, cargo, args, describe)

        check_binary(binary, allow_needed, max_glibc)
        if not args.no_smoke_test:
            smoke_test(docker, args.image, binary)

        staged = out_dir / BIN_NAME
        if staged.resolve() != binary:
            shutil.copy2(binary, staged)
            staged.chmod(0o755)

        artifacts = [staged]
        if args.tarball:
            artifacts.append(stage_tarball(staged, out_dir, version))

        log("done:")
        for path in artifacts:
            print(f"    {path}  ({path.stat().st_size:,} bytes)")
        return 0

    except BuildError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    except KeyboardInterrupt:
        return 130


if __name__ == "__main__":
    sys.exit(main())
