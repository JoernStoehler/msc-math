#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: run a low-friction smoke consumer over the sys-landscape dataset tables.
Input Artifacts: None (reads ad hoc paths passed on the CLI)
Output Artifacts: None (writes ad hoc paths passed on the CLI)
"""

import argparse
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import FIGSIZE_SINGLE, setup

setup()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset-dir", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def main() -> None:
    args = parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)

    states = load_jsonl(args.dataset_dir / "states.jsonl")
    polytopes = {
        row["poly_id"]: row for row in load_jsonl(args.dataset_dir / "polytopes.jsonl")
    }
    capacities = {
        row["poly_id"]: row
        for row in load_jsonl(args.dataset_dir / "capacity_results.jsonl")
    }

    polytope_features = {
        row["poly_id"]: row
        for row in load_jsonl(args.dataset_dir / "polytope-features.jsonl")
    }

    dataset_counts: Counter[str] = Counter()
    sys_by_dataset: defaultdict[str, list[float]] = defaultdict(list)
    x_by_dataset: defaultdict[str, list[float]] = defaultdict(list)
    y_by_dataset: defaultdict[str, list[float]] = defaultdict(list)
    dual_vertex_counts: list[int] = []

    for state in states:
        poly = polytopes[state["poly_id"]]
        cap = capacities[state["poly_id"]]
        dataset = state["dataset"]
        dataset_counts[dataset] += 1
        sys_by_dataset[dataset].append(float(cap["sys"]))
        x_by_dataset[dataset].append(float(poly["facet_count"]))
        y_by_dataset[dataset].append(float(cap["sys"]))
        feature_row = polytope_features.get(state["poly_id"])
        if feature_row is not None:
            dual_vertex_counts.append(int(feature_row["dual_vertex_count"]))

    figure_path = args.out_dir / "eda_sys_vs_facet_count.png"
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    for dataset in sorted(x_by_dataset):
        ax.scatter(
            x_by_dataset[dataset],
            y_by_dataset[dataset],
            label=dataset,
            alpha=0.8,
            s=30,
        )
    ax.set_xlabel(r"facet count")
    ax.set_ylabel(r"$\mathrm{sys}(K)$")
    ax.set_title("Smoke EDA: sys vs facet count")
    ax.legend(frameon=False, fontsize=8)
    fig.savefig(figure_path)
    plt.close(fig)

    summary = {
        "n_states": len(states),
        "datasets": dict(sorted(dataset_counts.items())),
        "max_sys_by_dataset": {
            dataset: max(values) for dataset, values in sorted(sys_by_dataset.items())
        },
        "dual_vertex_count_mean": (
            float(np.mean(dual_vertex_counts)) if dual_vertex_counts else None
        ),
    }
    summary_path = args.out_dir / "eda_summary.json"
    summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {figure_path}")
    print(f"Wrote {summary_path}")


if __name__ == "__main__":
    main()
