#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet
"""Build and package the Windows release of Skribisto — from Linux or Windows.

Produces, into ``dist/`` by default:

    skribisto.exe            the release binary
    Skribisto-portable.zip   that binary, zipped
    Skribisto-setup.exe      the NSIS installer

On a non-Windows host the Rust build goes through `cargo-xwin`, which supplies
the MSVC CRT and Windows SDK so the *msvc* target (not mingw) can be linked
with `lld-link`. On Windows it calls plain `cargo build`. Everything after the
compile is identical on both hosts.

Prerequisites, Debian/Ubuntu:

    sudo apt-get install -y nsis clang lld llvm
    rustup target add x86_64-pc-windows-msvc
    cargo install --locked cargo-xwin

On first use cargo-xwin downloads the MSVC CRT and Windows SDK from Microsoft
and asks you to accept their licence; export ``XWIN_ACCEPT_LICENSE=1`` to answer
that ahead of time, which is what CI does. This script deliberately does not set
it for you — accepting a licence is yours to do.

Prerequisites, Windows: NSIS, and the MSVC toolchain rustup already uses.

Two failure modes here are silent rather than loud, so this script checks the
finished binary for both rather than trusting the toolchain:

  * cargo-xwin ignores ``rustflags`` set in ``.cargo/config.toml``
    (rust-cross/cargo-xwin#36), so ``-C target-feature=+crt-static`` is passed
    through the ``RUSTFLAGS`` environment variable instead. Lose it and the exe
    silently gains a vcruntime140.dll dependency it cannot satisfy on a clean
    Windows machine.
  * build.rs downgrades a resource-embedding failure to a cargo warning, so a
    missing `llvm-rc` would yield an exe with no icon and no VERSIONINFO.

`--check-only` runs just those checks against an existing binary.
"""

from __future__ import annotations

import argparse
import os
import platform
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path

# package/windows/build.py -> package/windows -> package -> <repo root>
HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]

NSI = HERE / "setup.nsi"
ICON = REPO / "resources" / "windows" / "skribisto.ico"
LICENSE = REPO / "COPYING"

DEFAULT_TARGET = "x86_64-pc-windows-msvc"
CRT_STATIC = "-C target-feature=+crt-static"

# A dependency on any of these means the static-CRT flag did not take, and the
# app will refuse to start without the Visual C++ redistributable installed.
DYNAMIC_CRT_DLLS = ("vcruntime", "msvcp", "msvcr")

IS_WINDOWS = os.name == "nt"


class BuildError(Exception):
    """Anything that should stop the build with a readable message."""


# --------------------------------------------------------------------------
# small helpers


def log(msg: str) -> None:
    print(f"==> {msg}", flush=True)


def run(cmd: list[str], *, env: dict[str, str] | None = None, cwd: Path | None = None) -> None:
    log(" ".join(cmd))
    result = subprocess.run(cmd, env=env, cwd=str(cwd) if cwd else None)
    if result.returncode != 0:
        raise BuildError(f"{cmd[0]} exited with {result.returncode}")


def git(*args: str) -> str | None:
    """Trimmed stdout of a git command, or None if it fails or is empty."""
    try:
        out = subprocess.run(
            ["git", *args],
            cwd=str(REPO),
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError:
        return None
    if out.returncode != 0:
        return None
    return out.stdout.strip() or None


# --------------------------------------------------------------------------
# tool discovery


def find_makensis(explicit: str | None) -> str:
    if explicit:
        return explicit
    found = shutil.which("makensis")
    if found:
        return found
    # NSIS does not add itself to PATH on Windows.
    for base in (r"C:\Program Files (x86)\NSIS", r"C:\Program Files\NSIS"):
        candidate = Path(base) / "makensis.exe"
        if candidate.is_file():
            return str(candidate)
    raise BuildError(
        "makensis not found. Install it with `sudo apt-get install nsis` "
        "(Debian/Ubuntu) or from https://nsis.sourceforge.io, or pass --makensis."
    )


def find_resource_compiler() -> str | None:
    """Locate `llvm-rc` for winresource when cross-compiling.

    winresource shells out to `llvm-rc` for an msvc target on a Unix host and
    honours RC_PATH. Distros version the binary (llvm-rc-18, …) and often ship
    no unsuffixed alias, so look for both. Returns None on a Windows host,
    where the SDK's rc.exe is used instead.
    """
    if IS_WINDOWS:
        return None
    if os.environ.get("RC_PATH"):
        return os.environ["RC_PATH"]
    names = ["llvm-rc"] + [f"llvm-rc-{v}" for v in range(21, 13, -1)]
    for name in names:
        found = shutil.which(name)
        if found:
            return found
    return None


# --------------------------------------------------------------------------
# PE inspection
#
# Just enough of the format to read the section table and the names of the
# DLLs the binary imports. Avoids depending on objdump/dumpbin, which differ
# between the two hosts this script has to run on.


def _u16(data: bytes, off: int) -> int:
    return int.from_bytes(data[off : off + 2], "little")


def _u32(data: bytes, off: int) -> int:
    return int.from_bytes(data[off : off + 4], "little")


def inspect_pe(path: Path) -> tuple[list[str], list[str]]:
    """Return (section names, imported DLL names) for a PE image."""
    data = path.read_bytes()
    if data[:2] != b"MZ":
        raise BuildError(f"{path} is not a PE image (no MZ signature)")

    pe = _u32(data, 0x3C)
    if data[pe : pe + 4] != b"PE\0\0":
        raise BuildError(f"{path} is not a PE image (no PE signature)")

    coff = pe + 4
    n_sections = _u16(data, coff + 2)
    opt_size = _u16(data, coff + 16)
    opt = coff + 20
    # 0x20b = PE32+ (64-bit); its data directories sit 16 bytes further in.
    dir_off = opt + (112 if _u16(data, opt) == 0x20B else 96)
    import_rva = _u32(data, dir_off + 8)  # DataDirectory[1] = imports

    sections = []
    table = opt + opt_size
    for i in range(n_sections):
        entry = table + i * 40
        sections.append(
            (
                data[entry : entry + 8].rstrip(b"\0").decode("ascii", "replace"),
                _u32(data, entry + 12),  # VirtualAddress
                _u32(data, entry + 8),  # VirtualSize
                _u32(data, entry + 20),  # PointerToRawData
                _u32(data, entry + 16),  # SizeOfRawData
            )
        )

    def to_offset(rva: int) -> int | None:
        for _, vaddr, vsize, raw, raw_size in sections:
            if vaddr <= rva < vaddr + max(vsize, raw_size):
                return raw + (rva - vaddr)
        return None

    imports: list[str] = []
    off = to_offset(import_rva) if import_rva else None
    while off is not None:
        entry = data[off : off + 20]
        # The import directory is terminated by an all-zero descriptor.
        if len(entry) < 20 or not any(entry):
            break
        name_off = to_offset(_u32(entry, 12))
        if name_off is None:
            break
        end = data.find(b"\0", name_off)
        imports.append(data[name_off:end].decode("ascii", "replace"))
        off += 20

    return [s[0] for s in sections], imports


def check_exe(exe: Path) -> None:
    """Fail on either of the two silent regressions described in the docstring."""
    sections, imports = inspect_pe(exe)

    dynamic = [
        dll for dll in imports if dll.lower().startswith(DYNAMIC_CRT_DLLS)
    ]
    if dynamic:
        raise BuildError(
            f"{exe.name} imports {', '.join(dynamic)}, so it was NOT linked against "
            f"the static CRT and will not start without the Visual C++ "
            f"redistributable.\nEnsure RUSTFLAGS carries '{CRT_STATIC}' — note that "
            f"cargo-xwin ignores rustflags set in .cargo/config.toml."
        )

    if ".rsrc" not in sections:
        raise BuildError(
            f"{exe.name} has no .rsrc section, so the icon and VERSIONINFO were not "
            f"embedded. build.rs only warns when this fails; re-run with a working "
            f"resource compiler (llvm-rc on Linux, set RC_PATH to override) and look "
            f"for a 'failed to embed Windows resources' warning in the cargo output."
        )

    log(f"checked {exe.name}: static CRT, resources embedded ({len(imports)} imports)")


# --------------------------------------------------------------------------
# steps


def resolve_version(args: argparse.Namespace) -> tuple[str, str]:
    """Return (installer version, SKRIBISTO_GIT_DESCRIBE value).

    The release version is a git fact, not a Cargo one — every crate shares the
    placeholder 0.0.1 — so it comes from the tag. A trailing -N-ghash from a
    non-tagged build is harmless: setup.nsi derives the numeric VERSIONINFO
    quad by truncating at the first dash.
    """
    describe = args.git_describe or git("describe", "--tags", "--always", "--abbrev=9")

    version = args.version
    if not version:
        version = describe.lstrip("v") if describe else "0.0.0-dev"

    return version, (describe or f"v{version}")


def build_exe(args: argparse.Namespace, describe: str) -> Path:
    cargo = shutil.which("cargo")
    if not cargo:
        raise BuildError("cargo not found on PATH")

    # cargo-xwin supplies the MSVC CRT + Windows SDK that the msvc target needs
    # and which a non-Windows host has no other way to get.
    if IS_WINDOWS:
        cmd = [cargo, "build"]
    else:
        if not shutil.which("cargo-xwin"):
            raise BuildError(
                "cargo-xwin not found. Install it with "
                "`cargo install --locked cargo-xwin` (it needs clang, lld and llvm)."
            )
        cmd = [cargo, "xwin", "build"]

    cmd += ["--release", "--target", args.target, "-p", "teksilo_ui"]
    if args.features:
        cmd += ["--features", args.features]

    env = os.environ.copy()
    env["SKRIBISTO_GIT_DESCRIBE"] = describe

    # Must go through the environment: cargo-xwin ignores target rustflags from
    # .cargo/config.toml. --target is always passed, so this applies to the
    # Windows artifacts only and never to host build scripts or proc-macros.
    rustflags = env.get("RUSTFLAGS", "").strip()
    if "crt-static" not in rustflags:
        env["RUSTFLAGS"] = f"{rustflags} {CRT_STATIC}".strip()

    rc = find_resource_compiler()
    if rc:
        env["RC_PATH"] = rc
        log(f"resource compiler: {rc}")
    elif not IS_WINDOWS:
        raise BuildError(
            "llvm-rc not found, so the icon and version metadata cannot be embedded "
            "(build.rs would only warn, and the exe would ship unbranded).\n"
            "Install it with `sudo apt-get install llvm`, or set RC_PATH."
        )

    run(cmd, env=env, cwd=REPO)

    exe = REPO / "target" / args.target / "release" / "skribisto.exe"
    if not exe.is_file():
        raise BuildError(f"expected {exe} after the build, but it is missing")
    return exe


def build_installer(args: argparse.Namespace, exe: Path, out_dir: Path) -> Path:
    makensis = find_makensis(args.makensis)
    installer = out_dir / "Skribisto-setup.exe"

    # POSIX makensis takes a dash where the Windows build takes a slash, and on
    # both, defines must precede the script path (they are processed in order).
    dash = "/" if IS_WINDOWS else "-"
    cmd = [
        makensis,
        f"{dash}V3",
        f"{dash}DAPP_VERSION={args.version_string}",
        f"{dash}DSRC_EXE={exe}",
        f"{dash}DSRC_ICO={ICON}",
        f"{dash}DSRC_LICENSE={LICENSE}",
        f"{dash}DOUT_FILE={installer}",
        str(NSI),
    ]
    run(cmd)

    if not installer.is_file():
        raise BuildError(f"makensis reported success but {installer} is missing")
    return installer


def make_zip(exe: Path, out_dir: Path) -> Path:
    archive = out_dir / "Skribisto-portable.zip"
    log(f"writing {archive.name}")
    with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.write(exe, arcname="skribisto.exe")
    return archive


# --------------------------------------------------------------------------


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=__doc__.splitlines()[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--version",
        help="release version for the installer, e.g. 3.0.0. Defaults to `git describe` "
        "with any leading v stripped.",
    )
    parser.add_argument(
        "--git-describe",
        help="value for SKRIBISTO_GIT_DESCRIBE, which build.rs stamps into the binary. "
        "Defaults to `git describe`. Set this in CI, where a shallow checkout may "
        "leave git unable to name the tag itself.",
    )
    parser.add_argument("--target", default=DEFAULT_TARGET, help=f"(default: {DEFAULT_TARGET})")
    parser.add_argument(
        "--features", default="pdf", help="cargo features for teksilo_ui (default: pdf)"
    )
    parser.add_argument(
        "--out-dir", default=str(REPO / "dist"), help="where to put the artifacts (default: dist/)"
    )
    parser.add_argument("--makensis", help="path to makensis, if it is not on PATH")
    parser.add_argument(
        "--exe", help="use this prebuilt skribisto.exe instead of compiling one"
    )
    parser.add_argument("--skip-installer", action="store_true", help="do not run makensis")
    parser.add_argument("--no-zip", action="store_true", help="do not write the portable zip")
    parser.add_argument(
        "--check-only",
        action="store_true",
        help="only run the static-CRT and embedded-resource checks on --exe, then exit",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)

    try:
        if args.check_only:
            if not args.exe:
                raise BuildError("--check-only needs --exe")
            check_exe(Path(args.exe).resolve())
            return 0

        version, describe = resolve_version(args)
        args.version_string = version
        log(f"host {platform.system()}, target {args.target}")
        log(f"version {version} (SKRIBISTO_GIT_DESCRIBE={describe})")

        out_dir = Path(args.out_dir).resolve()
        out_dir.mkdir(parents=True, exist_ok=True)

        if args.exe:
            exe = Path(args.exe).resolve()
            if not exe.is_file():
                raise BuildError(f"{exe} does not exist")
        else:
            exe = build_exe(args, describe)

        check_exe(exe)

        staged = out_dir / "skribisto.exe"
        if staged != exe:
            shutil.copy2(exe, staged)

        artifacts = [staged]
        if not args.no_zip:
            artifacts.append(make_zip(staged, out_dir))
        if not args.skip_installer:
            artifacts.append(build_installer(args, staged, out_dir))

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
