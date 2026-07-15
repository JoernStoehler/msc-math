#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# ///
"""Fail-closed paired summaries for the target-free alignment ladder."""
import argparse
import hashlib
import json
import math
from collections import defaultdict
from pathlib import Path

THETA_ORDER = ["0", "pi_over_4", "pi_over_2", "3pi_over_4", "pi"]
EPS = 1e-9
REPRODUCTION_COMMAND = (
    "cargo run -p exp-sys-landscape --release "
    "--bin sys-datascience-generator-alignment-ladder -- "
    "--out-dir experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel"
)
VOLATILE_ROW_FIELDS = {"generation_ms", "reconstruction_ms"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_rows(path: Path) -> list[dict]:
    rows = [json.loads(line) for line in path.read_text().splitlines() if line]
    if not rows:
        raise ValueError("rows artifact is empty")
    return rows


def vector_delta(row: dict) -> list[float]:
    base = row["base_signature"]["symplectic_gram_upper"]
    response = row["response_signature"]["symplectic_gram_upper"]
    if len(base) != len(response):
        raise ValueError(f"signature length mismatch: {row['id']}")
    return [b - a for a, b in zip(base, response)]


def cosine(left: list[float], right: list[float]) -> float | None:
    ln = math.sqrt(sum(x * x for x in left))
    rn = math.sqrt(sum(x * x for x in right))
    if ln <= EPS or rn <= EPS:
        return None
    return sum(x * y for x, y in zip(left, right)) / (ln * rn)


def classify(group: list[dict]) -> dict:
    by_theta = {row["theta_label"]: row for row in group}
    ordered = [by_theta[label] for label in THETA_ORDER]
    values = [row["symplectic_gram_l2_change"] for row in ordered]
    monotone = all(right + EPS >= left for left, right in zip(values, values[1:]))
    reverse_symmetric = all(
        abs(by_theta[left]["symplectic_gram_l2_change"] - by_theta[right]["symplectic_gram_l2_change"])
        <= EPS
        for left, right in [("pi_over_4", "3pi_over_4"), ("0", "pi")]
    )
    endpoint_controlled = values[-1] + EPS >= max(values)
    vectors = [vector_delta(row) for row in ordered[1:]]
    nonzero_cosines = [
        abs(value)
        for value in (cosine(left, right) for left, right in zip(vectors, vectors[1:]))
        if value is not None
    ]
    multi_directional = bool(nonzero_cosines) and min(nonzero_cosines) < 1.0 - 1e-6
    return {
        "base_id": ordered[0]["base_id"],
        "bucket": ordered[0]["bucket"],
        "monotone_non_decreasing_l2": monotone,
        "reverse_theta_symmetric_l2": reverse_symmetric,
        "endpoint_controlled_l2": endpoint_controlled,
        "genuinely_multi_directional_signature_response": multi_directional,
        "successive_abs_cosines": nonzero_cosines,
    }


def write_tsv(path: Path, header: list[str], rows: list[list[object]]) -> None:
    lines = ["\t".join(header)]
    lines.extend("\t".join(str(value) for value in row) for row in rows)
    path.write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rows", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument(
        "--orientation-rows",
        type=Path,
        help="hydrated generator-orientation-smoke rows.jsonl for exact base-ID comparison",
    )
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    rows = load_rows(args.rows)
    report = json.loads(args.report.read_text())
    if not report.get("all_requested_rows_passed") or not report.get("formula_controls_passed"):
        raise ValueError("producer report is not a passing formula/reconstruction packet")
    if report.get("source_dirty"):
        raise ValueError("producer report was made from tracked-dirty source")
    if report.get("command") != REPRODUCTION_COMMAND:
        raise ValueError("report command is not the stable repo-relative reproduction command")
    comparison = {
        "status": "not_run_external_orientation_rows_unavailable",
        "orientation_source_revision": report.get("orientation_source_revision"),
        "orientation_rows_lfs_oid": report.get("orientation_rows_lfs_oid"),
    }
    if len(rows) != 40 or report.get("observed_rows") != 40:
        raise ValueError("expected exactly 40 rows: eight bases times five angles")
    groups: dict[str, list[dict]] = defaultdict(list)
    for row in rows:
        required = [
            "determinant", "orthogonality_residual", "symplectic_residual",
            "anti_symplectic_residual", "condition_number", "exact_base_volume",
            "exact_response_volume", "source_incidence_preserved", "base_signature",
            "response_signature", "euclidean_gram_max_abs_change",
        ]
        if row["exact_reconstruction_status"] != "reconstructed" or row["failures"]:
            raise ValueError(f"nonterminal or failed row: {row['id']}")
        if VOLATILE_ROW_FIELDS & row.keys():
            raise ValueError(f"volatile timing field retained in deterministic row: {row['id']}")
        if any(row.get(key) is None for key in required):
            raise ValueError(f"missing required response field: {row['id']}")
        if row["coordinate_order"] != "q1,q2,p1,p2":
            raise ValueError(f"coordinate convention mismatch: {row['id']}")
        if not row["source_incidence_preserved"] or abs(row["relative_volume_change"]) > EPS:
            raise ValueError(f"exact incidence/volume contract failed: {row['id']}")
        if row["orthogonality_residual"] > EPS or abs(row["determinant"] - 1.0) > EPS:
            raise ValueError(f"SO(4) contract failed: {row['id']}")
        if row["euclidean_gram_max_abs_change"] > EPS:
            raise ValueError(f"Euclidean control changed: {row['id']}")
        expected_kappa = math.sin(row["theta_radians"] / 2.0) ** 2
        if abs(row["kahler_departure_sin_sq_half_theta"] - expected_kappa) > EPS:
            raise ValueError(f"Kähler coordinate mismatch: {row['id']}")
        groups[row["base_id"]].append(row)
    if len(groups) != 8 or any({row["theta_label"] for row in group} != set(THETA_ORDER) for group in groups.values()):
        raise ValueError("each of eight bases must have exactly the five requested angles")
    # Same U1,U2 must be reused only within a base, the controlled-change contract.
    for base_id, group in groups.items():
        if len({(row["left_u2_seed"], row["right_u2_seed"]) for row in group}) != 1:
            raise ValueError(f"U(2) pair varied within base {base_id}")
    if args.orientation_rows:
        expected_oid = report["orientation_rows_lfs_oid"].removeprefix("sha256:")
        observed_oid = sha256(args.orientation_rows)
        if observed_oid != expected_oid:
            raise ValueError("orientation rows do not match report's pinned LFS object")
        orientation_rows = load_rows(args.orientation_rows)
        orientation_bases = {
            row["base_id"]: row["base_geometry_id"]
            for row in orientation_rows
            if row.get("base_geometry_id") is not None
        }
        alignment_bases = {
            row["base_id"]: row["base_geometry_id"]
            for row in rows
            if row.get("base_geometry_id") is not None
        }
        if len(orientation_bases) != 8 or len(alignment_bases) != 8:
            raise ValueError("expected eight unique hydrated orientation and alignment bases")
        if orientation_bases != alignment_bases:
            raise ValueError("orientation base IDs or exact geometry IDs do not match")
        comparison = {
            "status": "verified_exact_base_id_and_geometry_id_match",
            "orientation_source_revision": report["orientation_source_revision"],
            "orientation_rows_lfs_oid": report["orientation_rows_lfs_oid"],
            "orientation_rows_sha256": observed_oid,
            "bases": len(alignment_bases),
        }
    labels = {row["theta_label"]: row for row in rows if row["base_id"] == next(iter(groups))}
    if labels["0"]["symplectic_residual"] > EPS or labels["pi"]["anti_symplectic_residual"] > EPS:
        raise ValueError("endpoint semantic controls failed")
    classifications = [classify(group) for _, group in sorted(groups.items())]
    args.out_dir.mkdir(parents=True, exist_ok=True)
    paired_rows = []
    theta_values: dict[str, list[float]] = defaultdict(list)
    for base_id, group in sorted(groups.items()):
        classification = next(item for item in classifications if item["base_id"] == base_id)
        by_theta = {row["theta_label"]: row for row in group}
        for label in THETA_ORDER:
            row = by_theta[label]
            theta_values[label].append(row["symplectic_gram_l2_change"])
            paired_rows.append([
                base_id, row["bucket"], label, row["theta_radians"],
                row["kahler_departure_sin_sq_half_theta"], row["symplectic_gram_l2_change"],
                row["symplectic_gram_max_abs_change"], row["euclidean_gram_max_abs_change"],
                classification["monotone_non_decreasing_l2"],
                classification["reverse_theta_symmetric_l2"],
                classification["endpoint_controlled_l2"],
                classification["genuinely_multi_directional_signature_response"],
            ])
    write_tsv(args.out_dir / "paired-by-base.tsv", [
        "base_id", "bucket", "theta_label", "theta_radians", "kahler_departure", "symplectic_l2_change",
        "symplectic_max_abs_change", "euclidean_max_abs_change", "base_monotone", "base_reverse_symmetric",
        "base_endpoint_controlled", "base_multi_directional",
    ], paired_rows)
    theta_rows = []
    for label in THETA_ORDER:
        values = theta_values[label]
        theta_rows.append([label, len(values), min(values), sum(values) / len(values), max(values)])
    write_tsv(args.out_dir / "paired-by-theta.tsv", ["theta_label", "bases", "min_l2", "mean_l2", "max_l2"], theta_rows)
    outcome = {
        "schema": "alignment-ladder-analysis-v1",
        "rows_sha256": sha256(args.rows),
        "report_sha256": sha256(args.report),
        "analyzer_sha256": sha256(Path(__file__)),
        "bases": len(groups),
        "rows": len(rows),
        "classification_counts": {
            key: sum(bool(item[key]) for item in classifications)
            for key in ["monotone_non_decreasing_l2", "reverse_theta_symmetric_l2", "endpoint_controlled_l2", "genuinely_multi_directional_signature_response"]
        },
        "orientation_base_comparison": comparison,
        "per_base": classifications,
        "interpretation": "Finite-panel, target-free direct symplectic-signature responses after exact reconstruction. These labels do not show a capacity dose-response, population law, quotient parameterization, or target transfer.",
    }
    (args.out_dir / "analysis.json").write_text(json.dumps(outcome, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
