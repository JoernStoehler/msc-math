#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Promote reviewed LICCA random refresh outputs to canonical producer files."""

from __future__ import annotations

import argparse
import shutil
from datetime import datetime
from pathlib import Path


PROMOTIONS = [
    ("random-licca-refresh.jsonl", "random.jsonl", 4096),
    ("random-product-licca-refresh.jsonl", "random-product.jsonl", 10240),
    ("shared-cache-licca-random-refresh.jsonl", "shared-cache.jsonl", None),
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--produce-dir",
        type=Path,
        default=Path(__file__).resolve().parent,
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="Promote files. Without this flag, only print the planned changes.",
    )
    parser.add_argument(
        "--backup-dir",
        type=Path,
        help="Backup directory for current canonical files. Defaults to a timestamped directory.",
    )
    return parser.parse_args()


def line_count(path: Path) -> int:
    with path.open("rb") as handle:
        return sum(1 for _ in handle)


def main() -> None:
    args = parse_args()
    produce_dir = args.produce_dir
    missing_sources = [
        str(produce_dir / source)
        for source, _target, _expected_rows in PROMOTIONS
        if not (produce_dir / source).is_file()
    ]
    if missing_sources:
        raise SystemExit(
            "missing random refresh review targets:\n"
            + "\n".join(f"- {path}" for path in missing_sources)
        )

    backup_dir = args.backup_dir
    if backup_dir is None:
        stamp = datetime.now().strftime("%Y%m%d-%H%M%S")
        backup_dir = produce_dir / f"pre-random-refresh-promote-{stamp}"
    elif not backup_dir.is_absolute():
        backup_dir = produce_dir / backup_dir

    print("# LICCA Random Refresh Promotion")
    print()
    print(f"- produce dir: `{produce_dir}`")
    print(f"- backup dir: `{backup_dir}`")
    print(f"- mode: `{'write' if args.write else 'dry-run'}`")
    print()

    for source_name, target_name, expected_rows in PROMOTIONS:
        source = produce_dir / source_name
        target = produce_dir / target_name
        source_rows = line_count(source)
        target_rows = line_count(target) if target.exists() else None
        old = "missing" if target_rows is None else str(target_rows)
        expected = "any" if expected_rows is None else str(expected_rows)
        print(
            f"- `{source_name}` rows=`{source_rows}` expected=`{expected}` "
            f"-> `{target_name}` old_rows=`{old}`"
        )
        if expected_rows is not None and source_rows != expected_rows:
            raise SystemExit(
                f"{source_name} has {source_rows} rows, expected {expected_rows}; "
                "not promoting"
            )

    if not args.write:
        print()
        print("Dry run only. Pass `--write` to back up and promote canonical producer files.")
        return

    backup_dir.mkdir(parents=True, exist_ok=False)
    for _source_name, target_name, _expected_rows in PROMOTIONS:
        target = produce_dir / target_name
        if target.exists():
            shutil.copy2(target, backup_dir / target_name)

    for source_name, target_name, _expected_rows in PROMOTIONS:
        shutil.copy2(produce_dir / source_name, produce_dir / target_name)

    print()
    print("Promotion complete.")


if __name__ == "__main__":
    main()
