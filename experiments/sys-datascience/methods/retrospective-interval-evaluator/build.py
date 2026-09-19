#!/usr/bin/env python3
"""Build only this adapter and bind executable bytes to observed build inputs."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess


def source_identity(packet):
    root=packet.parents[3]
    paths=[packet/'Cargo.toml',packet/'Cargo.lock',*packet.glob('src/*.rs')]
    for crate in ('symplectic','euclidean-polytopes','algebraic-numbers'):
        paths.extend((root/'crates'/crate).rglob('*.rs'))
        paths.append(root/'crates'/crate/'Cargo.toml')
    paths.append(root/'Cargo.toml')
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}


def main():
    p=argparse.ArgumentParser(__doc__)
    p.add_argument('--target-dir',type=Path,required=True)
    p.add_argument('--receipt',type=Path,required=True)
    args=p.parse_args()
    packet=Path(__file__).resolve().parent
    before=source_identity(packet)
    subprocess.run(['cargo','build','--offline','--locked','--release','--manifest-path',str(packet/'Cargo.toml')],
                   env=dict(os.environ,CARGO_BUILD_JOBS='2',CARGO_TARGET_DIR=str(args.target_dir.resolve())),check=True,timeout=300)
    after=source_identity(packet)
    if before!=after:
        raise SystemExit('sources changed during build; no receipt produced')
    binary=args.target_dir/'release/current-body-evaluator'
    args.receipt.write_text(json.dumps({'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),
                                       'build_source_sha256':after},indent=2)+'\n')


if __name__=='__main__':
    main()
