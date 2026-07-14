#!/usr/bin/env python3
"""Write portable source and artifact identities for the resampling smoke."""

import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def version(*command):
    return subprocess.run(command, check=True, text=True, capture_output=True).stdout.strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=".")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    root = Path(args.root).resolve()
    owner = Path("experiments/sys-datascience/methods/product-bounce-active-resampling")
    paths = [
        Path("experiments/sys-datascience/produce/random-product.jsonl"),
        Path(
            "experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl"
        ),
        owner / "src/main.rs",
        owner / "src/bin/match_features.rs",
        owner / "select_bases.py",
        owner / "summarize.py",
        owner / "artifacts/match-features.jsonl",
        owner / "artifacts/bases.json",
        owner / "artifacts/proposals.jsonl",
        owner / "artifacts/runtime.txt",
        owner / "artifacts/match-runtime.txt",
        owner / "artifacts/summary.json",
    ]
    missing = [str(path) for path in paths if not (root / path).is_file()]
    if missing:
        raise ValueError(f"missing provenance inputs: {missing}")
    result = {
        "schema": "product-bounce-active-resampling/provenance/v1",
        "source_revision": version("git", "-C", str(root), "rev-parse", "HEAD"),
        "source_start_revision": "90c1bec29a593bf16323f41dda25545c682c59b5",
        "files": {
            str(path): {"sha256": sha256(root / path), "bytes": (root / path).stat().st_size}
            for path in paths
        },
        "commands": {
            "match_features": (
                "cargo build --manifest-path {o}/Cargo.toml --bin "
                "product-bounce-active-match-features && {o}/target/debug/"
                "product-bounce-active-match-features --input experiments/sys-datascience/"
                "produce/random-product.jsonl --output {o}/artifacts/match-features.jsonl"
            ).format(o=owner),
            "select_bases": (
                "python3 {o}/select_bases.py --raw experiments/sys-datascience/produce/"
                "random-product.jsonl --classes experiments/sys-datascience/methods/"
                "product-bounce-distribution/artifacts/class-minima.jsonl --ridge-features "
                "{o}/artifacts/match-features.jsonl --out {o}/artifacts/bases.json"
            ).format(o=owner),
            "evaluate": (
                "cargo build --release --manifest-path {o}/Cargo.toml --bin "
                "product-bounce-active-resampling && {o}/target/release/"
                "product-bounce-active-resampling --raw experiments/sys-datascience/produce/"
                "random-product.jsonl --classes experiments/sys-datascience/methods/"
                "product-bounce-distribution/artifacts/class-minima.jsonl --bases "
                "{o}/artifacts/bases.json --out {o}/artifacts/proposals.jsonl "
                "--accepted-per-base 16 --max-attempts-per-base-law 160"
            ).format(o=owner),
            "summarize": (
                "python3 {o}/summarize.py --proposals {o}/artifacts/proposals.jsonl "
                "--bases {o}/artifacts/bases.json --runtime {o}/artifacts/runtime.txt "
                "--out {o}/artifacts/summary.json"
            ).format(o=owner),
        },
        "tools": {
            "rustc": version("rustc", "--version"),
            "cargo": version("cargo", "--version"),
            "python": platform.python_version(),
        },
    }
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"out": str(out), "files": len(paths)}, indent=2))


if __name__ == "__main__":
    main()
