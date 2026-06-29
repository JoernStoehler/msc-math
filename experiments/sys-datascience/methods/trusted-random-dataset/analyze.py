#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Build and audit the trusted random/product method input tables."""

from __future__ import annotations

import argparse
from collections import Counter
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    dataset_label,
    load_jsonl,
    load_trusted_random_tables,
    provenance_by_poly_id,
    write_json,
    write_jsonl,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument(
        "--write-filtered",
        action="store_true",
        help="Write filtered JSONL tables under --out-dir. Default retains only summary.json.",
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    polytope_rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    provenance = provenance_by_poly_id(provenance_rows)

    labels = [
        dataset_label(row, provenance.get(str(row["poly_id"]), [])) for row in polytope_rows
    ]
    label_counts = Counter(labels)
    sys_values = [float(row["sys"]) for row in polytope_rows]
    duplicate_poly_ids = len(polytope_rows) - len({str(row["poly_id"]) for row in polytope_rows})
    bad_labels = [
        label
        for label in labels
        if "gradient" in label or "ascent" in label or "continuation" in label
    ]

    summary = {
        "input_tables_dir": str(args.tables_dir),
        "trusted_polytope_rows": len(polytope_rows),
        "trusted_provenance_rows": len(provenance_rows),
        "dataset_counts": dict(sorted(label_counts.items())),
        "duplicate_polytope_rows": duplicate_poly_ids,
        "excluded_label_hits": len(bad_labels),
        "max_sys": max(sys_values) if sys_values else None,
        "sys_gt_one": sum(1 for value in sys_values if value > 1.0),
        "min_sys": min(sys_values) if sys_values else None,
    }

    if args.write_filtered:
        write_jsonl(args.out_dir / "trusted-polytope-table.jsonl", polytope_rows)
        write_jsonl(args.out_dir / "trusted-polytope-provenance-table.jsonl", provenance_rows)
    write_json(args.out_dir / "summary.json", summary)

    print("# trusted-random-dataset")
    print()
    print(f"- trusted polytope rows: `{summary['trusted_polytope_rows']}`")
    print(f"- trusted provenance rows: `{summary['trusted_provenance_rows']}`")
    print(f"- duplicate polytope rows: `{summary['duplicate_polytope_rows']}`")
    print(f"- excluded label hits: `{summary['excluded_label_hits']}`")
    print(f"- max `sys`: `{summary['max_sys']}`")
    print(f"- `sys > 1` rows: `{summary['sys_gt_one']}`")
    print("- dataset counts:")
    for label, count in summary["dataset_counts"].items():
        print(f"  - `{label}`: `{count}`")
    print()
    if args.write_filtered:
        print(f"Wrote summary and filtered JSONL tables under `{args.out_dir}`")
    else:
        print(f"Wrote `{args.out_dir / 'summary.json'}`")


if __name__ == "__main__":
    main()
