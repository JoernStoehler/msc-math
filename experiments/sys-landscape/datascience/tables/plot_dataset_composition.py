#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""Plot retained datascience table composition by dataset source."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Any


HERE = Path(__file__).resolve().parent


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=HERE)
    parser.add_argument(
        "--out",
        type=Path,
        default=HERE / "dataset-composition.png",
        help="PNG output path.",
    )
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            row = json.loads(line)
            if not isinstance(row, dict):
                raise SystemExit(f"Expected JSON object in {path}:{line_number}")
            rows.append(row)
    return rows


def source_labels(provenance_rows: list[dict[str, Any]]) -> dict[str, str]:
    by_poly_id: dict[str, set[str]] = defaultdict(set)
    for row in provenance_rows:
        poly_id = str(row.get("poly_id", ""))
        dataset = str(row.get("dataset", ""))
        if poly_id and dataset:
            by_poly_id[poly_id].add(dataset)
    return {
        poly_id: ", ".join(sorted(datasets)) if datasets else "-"
        for poly_id, datasets in by_poly_id.items()
    }


def main() -> None:
    args = parse_args()
    polytope_rows = load_jsonl(args.tables_dir / "polytope-table.jsonl")
    provenance_rows = load_jsonl(args.tables_dir / "polytope-provenance-table.jsonl")
    labels = source_labels(provenance_rows)

    summary: dict[str, dict[str, float | int | None]] = {}
    for row in polytope_rows:
        poly_id = str(row.get("poly_id", ""))
        sys_value = row.get("sys")
        if not isinstance(sys_value, int | float):
            raise SystemExit(f"Missing or non-numeric `sys` for poly_id={poly_id}")
        label = labels.get(poly_id, "-")
        entry = summary.setdefault(label, {"rows": 0, "sys_gt_1": 0, "max_sys": None})
        entry["rows"] = int(entry["rows"] or 0) + 1
        if float(sys_value) > 1.0:
            entry["sys_gt_1"] = int(entry["sys_gt_1"] or 0) + 1
        max_sys = entry["max_sys"]
        if max_sys is None or float(sys_value) > float(max_sys):
            entry["max_sys"] = float(sys_value)

    ordered = sorted(summary.items(), key=lambda item: int(item[1]["rows"] or 0))

    print("# dataset-composition")
    print()
    print(f"- polytope rows: `{len(polytope_rows)}`")
    print(f"- provenance rows: `{len(provenance_rows)}`")
    print(f"- datasets: `{len(ordered)}`")
    print()
    print("| dataset | rows | sys > 1 | max sys |")
    print("| --- | ---: | ---: | ---: |")
    for label, entry in reversed(ordered):
        print(
            f"| {label} | `{entry['rows']}` | `{entry['sys_gt_1']}` | "
            f"`{entry['max_sys']}` |"
        )

    import matplotlib.pyplot as plt

    names = [label for label, _ in ordered]
    counts = [int(entry["rows"] or 0) for _, entry in ordered]

    height = max(3.5, 0.55 * len(names) + 1.4)
    fig, ax = plt.subplots(figsize=(9.0, height))
    bars = ax.barh(names, counts, color="#4c78a8")
    ax.set_xlabel("retained polytope rows")
    ax.set_title("Sys-landscape datascience table composition")
    ax.bar_label(bars, labels=[str(count) for count in counts], padding=4)

    ax.set_xlim(0, max(counts) * 1.18 if counts else 1)
    ax.spines[["top", "right"]].set_visible(False)
    fig.tight_layout()
    args.out.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(args.out, dpi=160)
    print()
    print(f"Wrote `{args.out}`")


if __name__ == "__main__":
    main()
