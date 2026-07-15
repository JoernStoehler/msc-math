#!/usr/bin/env python3

from pathlib import Path
import subprocess
import sys
import tempfile

HERE = Path(__file__).resolve().parent
with tempfile.TemporaryDirectory(prefix="generator-map-distortion-") as directory:
    root = Path(directory)
    for name in ("a", "b"): subprocess.run([sys.executable, str(HERE / "analyze.py"), "--out-dir", str(root / name)], cwd=HERE, check=True, stdout=subprocess.PIPE, text=True)
    assert (root / "a/report.json").read_bytes() == (root / "b/report.json").read_bytes()
print("byte-identical generator-map distortion reproduction passed")
