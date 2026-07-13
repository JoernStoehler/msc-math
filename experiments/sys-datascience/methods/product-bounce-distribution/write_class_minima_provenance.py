#!/usr/bin/env python3
"""Write portable provenance for the class-minimum artifacts."""
import argparse
import hashlib
import json
import os
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def checked_relative(path):
    path = Path(path)
    if path.is_absolute():
        raise ValueError(f"provenance paths must be repository-relative: {path}")
    return path


def command(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--class-minima", required=True)
    ap.add_argument("--summary", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--availability-audit", default=None)
    ap.add_argument("--null-availability", default=None)
    args = ap.parse_args()
    input_path, minima_path, summary_path, out_path = map(
        checked_relative, (args.input, args.class_minima, args.summary, args.out)
    )
    audit_path = checked_relative(args.availability_audit) if args.availability_audit else None
    null_path = checked_relative(args.null_availability) if args.null_availability else None
    sources = [
        Path("experiments/sys-datascience/methods/product-bounce-distribution/class-minima.rs"),
        Path("experiments/sys-datascience/methods/product-bounce-distribution/audit-null-availability.rs"),
        Path("experiments/sys-datascience/methods/product-bounce-distribution/summarize_class_minima.py"),
        Path("experiments/sys-datascience/methods/product-bounce-distribution/write_class_minima_provenance.py"),
        Path("experiments/sys-landscape/Cargo.toml"),
    ]
    result = {
        "artifact": "product-bounce class minima",
        "paths": {"input": str(input_path), "class_minima": str(minima_path), "summary": str(summary_path)},
        "sha256": {
            "input": sha256(ROOT / input_path),
            "class_minima": sha256(ROOT / minima_path),
            "summary": sha256(ROOT / summary_path),
            "source_files": {str(path): sha256(ROOT / path) for path in sources},
        },
        "producer": {
            "command": "cargo run -p exp-sys-landscape --release --bin sys-datascience-product-bounce-class-minima -- --input experiments/sys-datascience/produce/random-product.jsonl --output experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl",
            "summary_command": "python3 experiments/sys-datascience/methods/product-bounce-distribution/summarize_class_minima.py --input experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl --out experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-summary.json",
            "source_revision": command("git", "rev-parse", "HEAD"),
            "source_status": command("git", "status", "--short", "--", *map(str, sources)),
            "environment": {
                "rustc": command("rustc", "--version"),
                "cargo": command("cargo", "--version"),
                "rayon_num_threads": os.environ.get("RAYON_NUM_THREADS", "unset"),
            },
        },
    }
    if audit_path is not None:
        result["paths"]["availability_audit"] = str(audit_path)
        result["sha256"]["availability_audit"] = sha256(ROOT / audit_path)
        result["producer"]["availability_audit_command"] = (
            "cargo run -p exp-sys-landscape --release --bin "
            "sys-datascience-product-bounce-null-audit -- "
            "--input experiments/sys-datascience/produce/random-product.jsonl "
            "--class-minima experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl "
            "--output experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima-null-availability.jsonl"
        )
    if null_path is not None:
        result["paths"]["null_availability"] = str(null_path)
        result["sha256"]["null_availability"] = sha256(ROOT / null_path)
    (ROOT / out_path).write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
