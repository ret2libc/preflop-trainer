#!/usr/bin/env python3
"""Package GUI release artifacts and assets into a single zip.

Usage: python scripts/package_gui.py --crate crates/preflop-trainer-gui --assets assets
"""
from pathlib import Path
import argparse
import zipfile
import os
import platform
import sys


def find_first_existing(paths):
    for p in paths:
        pp = Path(p)
        if pp.exists():
            return pp
    return None


def is_executable(p: Path):
    if p.suffix.lower() == '.exe':
        return True
    try:
        return os.access(str(p), os.X_OK) and p.is_file()
    except Exception:
        return False


def is_library(p: Path):
    return p.suffix.lower() in {'.dll', '.so', '.dylib'}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--crate', default='crates/preflop-trainer-gui')
    parser.add_argument('--assets', default='assets')
    parser.add_argument('--out', default=None)
    parser.add_argument('--single', action='store_true', help='Package only the main executable')
    args = parser.parse_args()

    crate = args.crate
    candidates = [
        f"{crate}/target/x86_64-unknown-linux-musl/release",
        f"{crate}/target/release",
        "target/x86_64-unknown-linux-musl/release",
        "target/release",
        f"{crate}/target/debug",
        "target/debug",
    ]

    base = find_first_existing(candidates)
    if base is None:
        print('No build directory found among: {}'.format(','.join(candidates)), file=sys.stderr)
        return 2

    runner_os = os.getenv('RUNNER_OS') or platform.system()
    out_path = Path(args.out) if args.out else Path(f'gui-{runner_os}.zip')

    assets_path = Path(args.assets)
    single_only = args.single

    # Determine binary name from the crate's Cargo.toml if possible
    crate_toml = Path(args.crate) / 'Cargo.toml'
    bin_name = None
    if crate_toml.exists():
        try:
            for line in crate_toml.read_text(encoding='utf-8').splitlines():
                line = line.strip()
                if line.startswith('name') and '=' in line:
                    # crude parse: name = "preflop-trainer-gui"
                    parts = line.split('=', 1)
                    name_val = parts[1].strip().strip('"')
                    bin_name = name_val
                    break
        except Exception:
            bin_name = None

    files_added = 0
    with zipfile.ZipFile(out_path, 'w', zipfile.ZIP_DEFLATED) as z:
        if single_only:
            # Find the main executable in the build dir
            exe_found = None
            for p in base.iterdir():
                if not p.is_file():
                    continue
                name = p.name
                # exact match for windows exe
                if bin_name and (name == f"{bin_name}.exe" or name == bin_name):
                    exe_found = p
                    break
                # fallback: first .exe or executable file in dir
                if p.suffix.lower() == '.exe':
                    exe_found = p
                    break
                if is_executable(p) and exe_found is None:
                    exe_found = p

            if exe_found is None:
                print(f'No executable found in {base}', file=sys.stderr)
            else:
                z.write(exe_found, arcname=exe_found.name)
                files_added += 1
        else:
            # include assets (preserve top-level 'assets/' path in the zip)
            if assets_path.exists():
                for p in assets_path.rglob('*'):
                    if p.is_file():
                        arc = Path('assets') / p.relative_to(assets_path)
                        z.write(p, arcname=str(arc))
                        files_added += 1

            # include interesting build artifacts
            for p in base.rglob('*'):
                if not p.is_file():
                    continue
                rel = p.relative_to(base)
                include = False
                if is_executable(p) or is_library(p):
                    include = True
                elif p.suffix.lower() in {'.json', '.svg', '.pak', '.dat', '.txt', '.ini'}:
                    include = True
                else:
                    try:
                        if p.stat().st_size < 5 * 1024 * 1024:
                            include = True
                    except Exception:
                        include = True

                if include:
                    z.write(p, arcname=str(rel))
                    files_added += 1

    if files_added == 0:
        try:
            out_path.unlink()
        except Exception:
            pass
        print(f'No files added to {out_path}', file=sys.stderr)
        return 3

    print(str(out_path))
    return 0


if __name__ == '__main__':
    sys.exit(main())
