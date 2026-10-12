#!/usr/bin/env python3
"""Target-free cross-family relation scout for the retained 14,334-row join.

The script deliberately never loads ``sys``.  It compares canonical-vertex
covariance measurements with the retained combinatorial and normalized
symplectic two-face summaries, stratified by the 18 generator buckets.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
from pathlib import Path
from statistics import median
from typing import Any

from scipy import stats


COVARIANCE_FEATURES = (
    "rho",
    "nu1",
    "nu2",
    "covariance_condition",
)

# Metadata, diagnostics, and the target are excluded.  The remaining numeric
# columns are assigned to their substantive family by name.
EXCLUDED_HISTORICAL = {"poly_id", "sys", "capacity_source"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--historical", type=Path, required=True)
    parser.add_argument("--covariance", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    return parser.parse_args()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    rows = []
    with path.open() as handle:
        for line_number, line in enumerate(handle, 1):
            if not line.strip():
                continue
            row = json.loads(line)
            if not isinstance(row, dict):
                raise ValueError(f"expected object at {path}:{line_number}")
            rows.append(row)
    return rows


def finite_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(float(value))


def spearman(xs: list[float], ys: list[float]) -> float | None:
    if len(xs) < 20 or len(set(xs)) < 2 or len(set(ys)) < 2:
        return None
    value = float(stats.spearmanr(xs, ys).statistic)
    return value if math.isfinite(value) else None


def main() -> None:
    args = parse_args()
    historical_rows = read_jsonl(args.historical)
    covariance_rows = read_jsonl(args.covariance)
    historical = {row["poly_id"]: row for row in historical_rows}
    covariance = {row["poly_id"]: row for row in covariance_rows}
    joined_ids = sorted(historical.keys() & covariance.keys())
    if len(joined_ids) != 14334:
        raise ValueError(f"expected 14334 joined rows, found {len(joined_ids)}")

    # Assert target exclusion structurally: only named source fields below are
    # copied into the analysis rows.
    historical_features = sorted(
        key for key, value in historical[joined_ids[0]].items()
        if key not in EXCLUDED_HISTORICAL and finite_number(value)
    )
    combinatorial = [key for key in historical_features if not key.startswith("ridge_symp_area_")]
    symplectic_face = [key for key in historical_features if key.startswith("ridge_symp_area_")]
    if not combinatorial or not symplectic_face:
        raise ValueError("expected both historical feature families")

    rows = []
    for poly_id in joined_ids:
        hrow, crow = historical[poly_id], covariance[poly_id]
        row = {"poly_id": poly_id, "bucket": crow["bucket"]}
        for name in (*COVARIANCE_FEATURES, *historical_features):
            source = crow if name in COVARIANCE_FEATURES else hrow
            if not finite_number(source.get(name)):
                raise ValueError(f"nonfinite {name} for {poly_id}")
            row[name] = float(source[name])
        rows.append(row)

    buckets = sorted({row["bucket"] for row in rows})
    groups = {bucket: [row for row in rows if row["bucket"] == bucket] for bucket in buckets}
    candidate_rows: list[dict[str, Any]] = []
    for covariance_feature in COVARIANCE_FEATURES:
        for other_family, feature_names in (
            ("combinatorial", combinatorial),
            ("symplectic_face", symplectic_face),
        ):
            for other_feature in feature_names:
                bucket_values = []
                for bucket, group in groups.items():
                    coefficient = spearman(
                        [row[covariance_feature] for row in group],
                        [row[other_feature] for row in group],
                    )
                    if coefficient is not None:
                        bucket_values.append((bucket, coefficient))
                if not bucket_values:
                    continue
                coefficients = [value for _, value in bucket_values]
                generic = [value for bucket, value in bucket_values if bucket.startswith("random:F")]
                products = [value for bucket, value in bucket_values if bucket.startswith("random_product:")]
                nonzero = [value for value in coefficients if value != 0]
                sign_consistency = (
                    max(sum(value > 0 for value in nonzero), sum(value < 0 for value in nonzero)) / len(nonzero)
                    if nonzero else 0.0
                )
                candidate_rows.append({
                    "covariance_feature": covariance_feature,
                    "other_family": other_family,
                    "other_feature": other_feature,
                    "eligible_buckets": len(coefficients),
                    "median_spearman": median(coefficients),
                    "median_abs_spearman": median(abs(value) for value in coefficients),
                    "generic_median_spearman": median(generic) if generic else None,
                    "product_median_spearman": median(products) if products else None,
                    "sign_consistency": sign_consistency,
                    "min_spearman": min(coefficients),
                    "max_spearman": max(coefficients),
                    "bucket_spearmans": {bucket: value for bucket, value in bucket_values},
                })

    candidate_rows.sort(
        key=lambda row: (row["sign_consistency"], row["median_abs_spearman"]), reverse=True
    )
    # Select concrete, target-free counterpressure against the leading relation.
    # Rank differences are calculated only inside buckets, so bucket scale and
    # source composition cannot nominate an exemplar by themselves.
    discordant_exemplars = []
    for bucket, group in groups.items():
        rho_ranks = stats.rankdata([row["rho"] for row in group]) / len(group)
        ridge_ranks = stats.rankdata(
            [row["ridge_symp_area_sum_over_volume_sqrt"] for row in group]
        ) / len(group)
        ranked = sorted(
            (
                {
                    "poly_id": row["poly_id"],
                    "bucket": bucket,
                    "rho": row["rho"],
                    "ridge_symp_area_sum_over_volume_sqrt": row["ridge_symp_area_sum_over_volume_sqrt"],
                    "rho_rank_fraction": float(rho_rank),
                    "ridge_rank_fraction": float(ridge_rank),
                    "absolute_rank_disagreement": float(abs(rho_rank - ridge_rank)),
                }
                for row, rho_rank, ridge_rank in zip(group, rho_ranks, ridge_ranks)
            ),
            key=lambda row: row["absolute_rank_disagreement"],
            reverse=True,
        )
        discordant_exemplars.append(ranked[0])
    discordant_exemplars.sort(
        key=lambda row: row["absolute_rank_disagreement"], reverse=True
    )
    args.out_dir.mkdir(parents=True, exist_ok=True)
    with (args.out_dir / "relations.tsv").open("w", newline="") as handle:
        fields = [
            "covariance_feature", "other_family", "other_feature", "eligible_buckets",
            "median_spearman", "median_abs_spearman", "generic_median_spearman",
            "product_median_spearman", "sign_consistency", "min_spearman", "max_spearman",
        ]
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in candidate_rows:
            writer.writerow({key: row[key] for key in fields})

    summary = {
        "status": "exploratory target-free screen; not a theorem, validation, or capacity result",
        "question": "Which covariance measurements have stable within-generator relations with a substantively different retained feature family?",
        "target_exclusion": "The script never copies or analyzes sys; pairs are ranked only by within-bucket feature-to-feature Spearman coefficients.",
        "inputs": {
            "historical": {"path": str(args.historical), "sha256": sha256(args.historical), "rows": len(historical_rows)},
            "covariance": {"path": str(args.covariance), "sha256": sha256(args.covariance), "rows": len(covariance_rows)},
        },
        "joined_rows": len(rows),
        "buckets": {bucket: len(group) for bucket, group in groups.items()},
        "feature_families": {
            "covariance": list(COVARIANCE_FEATURES),
            "combinatorial": combinatorial,
            "symplectic_face": symplectic_face,
        },
        "transformation_boundary": {
            "rho": "translation- and scale-invariant; invariant under exact linear symplectic changes of the vertex representative",
            "nu1_nu2": "translation-invariant and symplectic-spectrum measurements, but homogeneous of degree two under common scaling",
            "covariance_condition": "Euclidean representative-dependent diagnostic; not symplectically invariant",
            "historical": "combinatorial invariants and volume-normalized symplectic two-face summaries under the owning feature contract",
        },
        "searched_cross_family_pairs": len(candidate_rows),
        "ranking": "descending sign consistency across buckets, then median absolute within-bucket Spearman",
        "top_relations": candidate_rows[:20],
        "leading_relation_discordant_exemplars": discordant_exemplars,
        "decision_rule": {
            "follow_up": "Only consider a relation for semantic reconstruction when sign consistency is at least 0.9, generic and product median signs agree, and median absolute Spearman is at least 0.5.",
            "otherwise": "Do not add model complexity; acquire a substantively different representation or controlled family.",
        },
    }
    (args.out_dir / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    print(json.dumps({
        "joined_rows": len(rows),
        "searched_cross_family_pairs": len(candidate_rows),
        "top": candidate_rows[:5],
    }, indent=2))


if __name__ == "__main__":
    main()
