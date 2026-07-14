"""Mutation regression: truncating raw evidence must fail validation."""
from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path


def main() -> None:
    source = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/retained-exact")
    validator = Path(__file__).with_name("validate_retained_exact.py")
    with tempfile.TemporaryDirectory(prefix="retained-exact-mutation-") as temp:
        target = Path(temp) / "artifact"
        target.mkdir()
        lines = (source / "raw_rows.jsonl").read_text().splitlines()
        (target / "raw_rows.jsonl").write_text("\n".join(lines[:-1]) + "\n")
        result = subprocess.run([sys.executable, str(validator), str(target)], capture_output=True, text=True)
        if result.returncode == 0:
            raise SystemExit("mutation regression failed: truncated artifact validated")
    print("mutation regression passed")


if __name__ == "__main__":
    main()
