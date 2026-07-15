#!/usr/bin/env python3
"""Check byte-identical reports from a clean, source-bound analyzer surface."""

from __future__ import annotations

import argparse
from pathlib import Path
import subprocess
import sys
import tempfile


HERE = Path(__file__).resolve().parent


def run(out_dir: Path, factor_shapes: Path | None) -> None:
    command = [sys.executable, str(HERE / "analyze.py"), "--out-dir", str(out_dir)]
    if factor_shapes is not None:
        command.extend(["--factor-shapes", str(factor_shapes)])
    subprocess.run(command, cwd=HERE, check=True, stdout=subprocess.PIPE, text=True)


def compare(left: Path, right: Path, label: str) -> None:
    if left.read_bytes() != right.read_bytes():
        raise AssertionError(f"{label} reports are not byte-identical")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--factor-shapes", type=Path, required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="generator-distribution-dimension-") as directory:
        root = Path(directory)
        run(root / "calibration-a", None)
        run(root / "calibration-b", None)
        compare(root / "calibration-a/report.json", root / "calibration-b/report.json", "calibration")
        run(root / "smoke-a", args.factor_shapes)
        run(root / "smoke-b", args.factor_shapes)
        compare(root / "smoke-a/report.json", root / "smoke-b/report.json", "real smoke")
    print("byte-identical calibration and real-smoke reproduction passed")


if __name__ == "__main__":
    main()
