#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Check that pre-target JSONL artifacts do not expose target-like field names."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

FORBIDDEN_KEY_PARTS = ("sys", "capacity", "bounce", "target", "min_action")


def iter_key_paths(value: object, prefix: str = ""):
    if isinstance(value, dict):
        for key, child in value.items():
            path = f"{prefix}.{key}" if prefix else str(key)
            yield path
            yield from iter_key_paths(child, path)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            yield from iter_key_paths(child, f"{prefix}[{index}]")


def check_jsonl(path: Path) -> list[str]:
    violations: list[str] = []
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            row = json.loads(line)
            for key_path in iter_key_paths(row):
                key = key_path.rsplit(".", 1)[-1].split("[", 1)[0].lower()
                if any(part in key for part in FORBIDDEN_KEY_PARTS):
                    violations.append(f"{path}:{line_number}:{key_path}")
    return violations


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("jsonl", nargs="+", type=Path)
    args = parser.parse_args()

    violations = []
    for path in args.jsonl:
        violations.extend(check_jsonl(path))
    if violations:
        print("target-like keys found in pre-target artifacts:")
        for violation in violations:
            print(violation)
        raise SystemExit(1)
    print(f"checked {len(args.jsonl)} pre-target JSONL artifact(s): no forbidden keys")


if __name__ == "__main__":
    main()
