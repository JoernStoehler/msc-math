#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""First-pass audit of declared identities and ridge-feature rank redundancy."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
from collections import defaultdict
from pathlib import Path
import sys

import numpy as np
from scipy import stats

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    active_invariant_numeric_feature_names,
    dataset_label,
    load_trusted_random_tables,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)

DEFAULT_OVERLAP = (
    HERE.parent
    / "ridge-mechanism-discriminator"
    / "artifacts"
    / "current"
    / "scalar_feature_overlap_matrix.tsv"
)
RIDGE_PREFIX = "ridge_symp_area_"
ORDERING_FIELDS = {
    "ridge_symp_area_ordered_face_count",
    "ridge_symp_area_ordering_failure_count",
    "ridge_symp_area_ordered_fraction",
}
IDENTITY_TOLERANCE = 2e-12
CORRELATION_THRESHOLD = 0.9


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--selection-overlap", type=Path, default=DEFAULT_OVERLAP)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    return parser.parse_args()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def max_abs_error(left: np.ndarray, right: np.ndarray) -> float:
    return float(np.max(np.abs(left.astype(float) - right.astype(float))))


def assert_identity(name: str, left: np.ndarray, right: np.ndarray) -> dict[str, object]:
    error = max_abs_error(left, right)
    if error > IDENTITY_TOLERANCE:
        raise AssertionError(f"identity {name!r} failed: max absolute error {error}")
    return {"identity": name, "max_abs_error": error, "assertion": "passed"}


def group_labels(
    rows: list[dict[str, object]], provenance_rows: list[dict[str, object]]
) -> tuple[list[str], list[bool]]:
    provenance = provenance_by_poly_id(provenance_rows)
    labels: list[str] = []
    product_mask: list[bool] = []
    for row in rows:
        row_provenance = provenance.get(str(row["poly_id"]), [])
        is_product = row.get("capacity_source") == "random_product_sample"
        if is_product:
            label = f"random_product_sample:{product_bucket(row_provenance)}"
        else:
            label = f"{dataset_label(row, row_provenance)}:facet_{int(row['facet_count'])}"
        labels.append(label)
        product_mask.append(is_product)
    return labels, product_mask


def grouped_ranks(values: np.ndarray, labels: list[str]) -> np.ndarray:
    groups: dict[str, list[int]] = defaultdict(list)
    for index, label in enumerate(labels):
        groups[label].append(index)
    ranks = np.zeros(len(values), dtype=float)
    for indices in groups.values():
        subset = values[np.array(indices)]
        ranks[np.array(indices)] = stats.rankdata(subset, method="average") / len(indices)
    return ranks


def spearman(left: np.ndarray, right: np.ndarray) -> float | None:
    if np.all(left == left[0]) or np.all(right == right[0]):
        return None
    value = float(stats.spearmanr(left, right).statistic)
    return value if math.isfinite(value) else None


def ridge_pair_rows(
    rows: list[dict[str, object]], labels: list[str], product_mask: list[bool]
) -> list[dict[str, object]]:
    names = sorted(
        name
        for name in active_invariant_numeric_feature_names(rows)
        if name.startswith(RIDGE_PREFIX) and name not in ORDERING_FIELDS
    )
    values = {name: np.array([float(row[name]) for row in rows]) for name in names}
    ranked = {name: grouped_ranks(value, labels) for name, value in values.items()}
    product_indices = np.array(product_mask, dtype=bool)
    output: list[dict[str, object]] = []
    for left_index, left in enumerate(names):
        for right in names[left_index + 1 :]:
            overall = spearman(values[left], values[right])
            within = spearman(ranked[left], ranked[right])
            product_within = spearman(
                ranked[left][product_indices], ranked[right][product_indices]
            )
            if not any(
                value is not None and abs(value) >= CORRELATION_THRESHOLD
                for value in (overall, within, product_within)
            ):
                continue
            if {left, right} == {
                "ridge_symp_area_mean_over_volume_sqrt",
                "ridge_symp_area_sum_over_volume_sqrt",
            }:
                interpretation = "exact rank equivalence in fixed-ridge-count product buckets"
            elif {left, right} == {
                "ridge_symp_area_entropy",
                "ridge_symp_area_effective_face_count",
            }:
                interpretation = "global monotone identity"
            elif set((left, right)).issubset(
                {
                    "ridge_symp_area_entropy",
                    "ridge_symp_area_effective_face_count",
                    "ridge_symp_area_normalized_entropy",
                }
            ):
                interpretation = "rank equivalence in fixed-ridge-count product buckets"
            else:
                interpretation = "high empirical rank correlation; not an identity"
            output.append(
                {
                    "left": left,
                    "right": right,
                    "spearman_overall": overall,
                    "spearman_pooled_within_source_bucket_fractional_ranks": within,
                    "spearman_pooled_within_product_bucket_fractional_ranks": product_within,
                    "interpretation": interpretation,
                }
            )
    return sorted(
        output,
        key=lambda row: (
            -abs(
                float(
                    row[
                        "spearman_pooled_within_product_bucket_fractional_ranks"
                    ]
                    or 0.0
                )
            ),
            str(row["left"]),
            str(row["right"]),
        ),
    )


def selection_overlap_rows(path: Path) -> list[dict[str, object]]:
    with path.open(newline="") as handle:
        source = list(csv.DictReader(handle, delimiter="\t"))
    by_rule = {row["rule"]: row for row in source}
    requested = [
        ("ridge_symp_area_mean_over_volume_sqrt", "ridge_symp_area_sum_over_volume_sqrt"),
        ("ridge_symp_area_mean_over_volume_sqrt", "ridge_symp_area_max_over_volume_sqrt"),
        ("ridge_symp_area_mean_over_volume_sqrt", "ridge_symp_area_std_over_volume_sqrt"),
        ("ridge_symp_area_mean_over_volume_sqrt", "ridge_symp_area_q95_over_volume_sqrt"),
        ("ridge_symp_area_max_over_volume_sqrt", "ridge_symp_area_std_over_volume_sqrt"),
        ("ridge_symp_area_max_over_volume_sqrt", "ridge_symp_area_q95_over_volume_sqrt"),
        ("ridge_symp_area_std_over_volume_sqrt", "ridge_symp_area_q95_over_volume_sqrt"),
        ("ridge_symp_area_q90_over_volume_sqrt", "ridge_symp_area_q95_over_volume_sqrt"),
        ("facet_count", "vertex_count"),
        ("facet_count", "edge_count"),
        ("facet_count", "ridge_count"),
    ]
    output: list[dict[str, object]] = []
    for left, right in requested:
        cell = by_rule[left][right]
        intersection_text, jaccard_text = cell.split("|")
        intersection = int(intersection_text)
        jaccard = float(jaccard_text)
        if left in {"facet_count", "vertex_count", "edge_count", "ridge_count"}:
            interpretation = (
                "identical under configured global/per-bucket rules; "
                "counts are fixed inside each product bucket"
            )
        elif jaccard == 1.0:
            interpretation = "identical extreme selection"
        else:
            interpretation = "distinct extreme selection despite family correlation"
        output.append(
            {
                "left": left,
                "right": right,
                "intersection": intersection,
                "jaccard": jaccard,
                "selected_rows_each": int(by_rule[left]["selected_rows"]),
                "interpretation": interpretation,
            }
        )
    return output


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise AssertionError(f"refusing to write empty first-pass audit artifact {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=list(rows[0]),
            delimiter="\t",
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    labels, product_mask = group_labels(rows, provenance_rows)
    arrays = {
        name: np.array([float(row[name]) for row in rows])
        for name in active_invariant_numeric_feature_names(rows)
    }
    ordered_count = np.array(
        [float(row["ridge_symp_area_ordered_face_count"]) for row in rows]
    )
    positive_area = arrays["ridge_symp_area_sum_over_volume_sqrt"] > 0
    expected_effective_count = np.zeros(len(rows), dtype=float)
    expected_effective_count[positive_area] = np.exp(
        arrays["ridge_symp_area_entropy"][positive_area]
    )
    expected_normalized_entropy = np.zeros(len(rows), dtype=float)
    multiple_faces = ordered_count > 1
    expected_normalized_entropy[multiple_faces] = (
        arrays["ridge_symp_area_entropy"][multiple_faces]
        / np.log(ordered_count[multiple_faces])
    )

    identities = [
        assert_identity(
            "ridge area sum = mean * ordered face count",
            arrays["ridge_symp_area_sum_over_volume_sqrt"],
            arrays["ridge_symp_area_mean_over_volume_sqrt"] * ordered_count,
        ),
        assert_identity(
            "effective face count = exp(entropy)",
            arrays["ridge_symp_area_effective_face_count"],
            expected_effective_count,
        ),
        assert_identity(
            "normalized entropy = entropy / log(ordered face count)",
            arrays["ridge_symp_area_normalized_entropy"],
            expected_normalized_entropy,
        ),
        assert_identity(
            "edge density = 2E/(V(V-1))",
            arrays["edge_density"],
            2 * arrays["edge_count"] / (
                arrays["vertex_count"] * (arrays["vertex_count"] - 1)
            ),
        ),
        assert_identity(
            "vertex degree mean = 2E/V",
            arrays["vertex_degree_mean"],
            2 * arrays["edge_count"] / arrays["vertex_count"],
        ),
    ]
    all_simple = all(bool(row["is_simple"]) for row in rows)
    if all_simple:
        identities.extend(
            [
                assert_identity("E = 2V", arrays["edge_count"], 2 * arrays["vertex_count"]),
                assert_identity(
                    "R = V + F",
                    arrays["ridge_count"],
                    arrays["vertex_count"] + arrays["facet_count"],
                ),
                assert_identity(
                    "R * ridge size mean = 6V",
                    arrays["ridge_count"] * arrays["ridge_size_mean"],
                    6 * arrays["vertex_count"],
                ),
                assert_identity(
                    "F * facet vertex mean = 4V",
                    arrays["facet_count"] * arrays["facet_vertex_count_mean"],
                    4 * arrays["vertex_count"],
                ),
                assert_identity(
                    "F * facet neighbor mean = 2R",
                    arrays["facet_count"] * arrays["facet_neighbor_count_mean"],
                    2 * arrays["ridge_count"],
                ),
                assert_identity(
                    "facet neighbor mean = facet vertex mean / 2 + 2",
                    arrays["facet_neighbor_count_mean"],
                    arrays["facet_vertex_count_mean"] / 2 + 2,
                ),
                assert_identity(
                    "facet neighbor std = facet vertex std / 2",
                    arrays["facet_neighbor_count_std"],
                    arrays["facet_vertex_count_std"] / 2,
                ),
                assert_identity(
                    "facet neighbor min = facet vertex min / 2 + 2",
                    arrays["facet_neighbor_count_min"],
                    arrays["facet_vertex_count_min"] / 2 + 2,
                ),
                assert_identity(
                    "facet neighbor max = facet vertex max / 2 + 2",
                    arrays["facet_neighbor_count_max"],
                    arrays["facet_vertex_count_max"] / 2 + 2,
                ),
            ]
        )

    pair_rows = ridge_pair_rows(rows, labels, product_mask)
    overlap_rows = selection_overlap_rows(args.selection_overlap)
    overlap_logical_path = (
        "../ridge-mechanism-discriminator/artifacts/current/scalar_feature_overlap_matrix.tsv"
        if args.selection_overlap.resolve() == DEFAULT_OVERLAP.resolve()
        else "<selection-overlap>"
    )
    dataset_counts: dict[str, int] = defaultdict(int)
    for row in rows:
        dataset_counts[str(row["capacity_source"])] += 1
    bucket_counts: dict[str, int] = defaultdict(int)
    for label in labels:
        bucket_counts[label] += 1

    summary = {
        "schema": "sys-datascience.feature-family-quotient.v2",
        "question": (
            "Which declared current-schema identities hold, and which ridge features "
            "are rank-equivalent or strongly correlated on this slice?"
        ),
        "audit_scope": {
            "status": "first pass, not a complete algebraic or empirical quotient",
            "declared_identities": (
                "producer formulas plus simple-four-polytope identities selected "
                "before this run; other identities were not searched exhaustively"
            ),
            "empirical_pairs": (
                "ridge-feature pairs meeting the fixed rank-correlation threshold; "
                "non-ridge empirical pair discovery is outside this packet"
            ),
        },
        "inputs": {
            "polytope_table": {
                "path": "<tables-dir>/polytope-table.jsonl",
                "sha256": sha256(args.tables_dir / "polytope-table.jsonl"),
            },
            "provenance_table": {
                "path": "<tables-dir>/polytope-provenance-table.jsonl",
                "sha256": sha256(args.tables_dir / "polytope-provenance-table.jsonl"),
            },
            "selection_overlap": {
                "path": overlap_logical_path,
                "sha256": sha256(args.selection_overlap),
            },
        },
        "row_count": len(rows),
        "provenance": {
            "prepared_table_builder": "../../prepare/build-random-only-slice.sh full <tables-dir>",
            "prepared_table_contract": "../standard-baseline-p2/README.md",
            "selection_overlap_owner": "../ridge-mechanism-discriminator/README.md",
        },
        "dataset_counts": dict(sorted(dataset_counts.items())),
        "bucket_counts": dict(sorted(bucket_counts.items())),
        "all_rows_simple": all_simple,
        "all_two_face_orderings_complete": all(
            int(row["ridge_symp_area_ordering_failure_count"]) == 0
            and float(row["ridge_symp_area_ordered_fraction"]) == 1.0
            for row in rows
        ),
        "identity_tolerance": IDENTITY_TOLERANCE,
        "declared_identity_count": len(identities),
        "identity_assertions": identities,
        "rank_pair_threshold": CORRELATION_THRESHOLD,
        "rank_pair_count": len(pair_rows),
        "selection_overlap_pair_count": len(overlap_rows),
        "interpretation": {
            "ridge_groups": [
                "magnitude and upper-tail level",
                "concentration and evenness",
                "small-area threshold fractions (weak/discrete diagnostics)",
            ],
            "boundary": (
                "This first pass checks declared identities and ridge-feature rank "
                "redundancy; it is not a complete quotient of the engineered schema, "
                "does not establish a mechanism or complete the invariant feature map, "
                "and does not validate a sys>1 proposer."
            ),
            "pooled_fractional_rank_calculation": (
                "Each feature is converted to fractional ranks separately inside each "
                "source/facet or product bucket; Spearman correlation is then computed "
                "after pooling rows. This is row-weighted, not an aggregation of "
                "per-bucket correlations."
            ),
            "selection_boundary": (
                "Selection overlaps describe the frozen 100-of-100k random-product "
                "selection sets recorded by the upstream proposer packet."
            ),
        },
        "artifacts": {
            "rank_pairs": "feature-family-rank-pairs.tsv",
            "selection_overlap": "feature-family-selection-overlap.tsv",
        },
    }
    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_json(args.out_dir / "feature-family-quotient.json", summary)
    write_tsv(args.out_dir / "feature-family-rank-pairs.tsv", pair_rows)
    write_tsv(args.out_dir / "feature-family-selection-overlap.tsv", overlap_rows)
    print(f"rows={len(rows)} identities={len(identities)} rank_pairs={len(pair_rows)}")
    print(f"wrote={args.out_dir}")


if __name__ == "__main__":
    main()
