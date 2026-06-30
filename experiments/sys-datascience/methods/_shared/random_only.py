"""Shared helpers for random-only sys-datascience method packets."""

from __future__ import annotations

import json
from collections import defaultdict
from pathlib import Path
from typing import Any, Iterable

import numpy as np


METHODS_DIR = Path(__file__).resolve().parents[1]
SYS_DATASCIENCE_DIR = METHODS_DIR.parent
TABLES_DIR = SYS_DATASCIENCE_DIR / "prepare"

TRUSTED_DATASETS = {
    "random_sample",
    "random_product_sample",
}
REFERENCE_HOLDOUT_DATASETS = {
    "known_hko_reference",
}
EXCLUDED_DATASET_WORDS = ("ascent", "continuation", "gradient")
EXCLUDED_OPTIMIZER_WORDS = ("ascent", "gradient", "continuation")

ACTIVE_INVARIANT_NUMERIC_FEATURES = (
    "facet_count",
    "vertex_count",
    "edge_count",
    "ridge_count",
    "is_simple",
    "simple_vertex_fraction",
    "edge_density",
    "vertex_incident_facets_mean",
    "vertex_incident_facets_std",
    "vertex_incident_facets_min",
    "vertex_incident_facets_max",
    "vertex_degree_mean",
    "vertex_degree_std",
    "vertex_degree_min",
    "vertex_degree_max",
    "ridge_size_mean",
    "ridge_size_std",
    "ridge_size_min",
    "ridge_size_max",
    "facet_vertex_count_mean",
    "facet_vertex_count_std",
    "facet_vertex_count_min",
    "facet_vertex_count_max",
    "facet_neighbor_count_mean",
    "facet_neighbor_count_std",
    "facet_neighbor_count_min",
    "facet_neighbor_count_max",
    "ridge_symp_area_mean_over_volume_sqrt",
    "ridge_symp_area_std_over_volume_sqrt",
    "ridge_symp_area_min_over_volume_sqrt",
    "ridge_symp_area_max_over_volume_sqrt",
    "ridge_symp_area_q25_over_volume_sqrt",
    "ridge_symp_area_median_over_volume_sqrt",
    "ridge_symp_area_q75_over_volume_sqrt",
    "ridge_symp_area_q90_over_volume_sqrt",
    "ridge_symp_area_q95_over_volume_sqrt",
    "ridge_symp_area_sum_over_volume_sqrt",
    "ridge_symp_area_max_share",
    "ridge_symp_area_top3_share",
    "ridge_symp_area_le_1em3_over_volume_sqrt_fraction",
    "ridge_symp_area_le_1em2_over_volume_sqrt_fraction",
    "ridge_symp_area_le_1em1_over_volume_sqrt_fraction",
    "ridge_symp_area_entropy",
    "ridge_symp_area_effective_face_count",
    "ridge_symp_area_normalized_entropy",
)


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            if line_number == 1 and line.startswith("version https://git-lfs.github.com/spec/"):
                raise SystemExit(
                    f"{path} is a Git LFS pointer; hydrate retained experiment data with "
                    "git lfs checkout/pull or pass a run-local tables directory"
                )
            row = json.loads(line)
            if not isinstance(row, dict):
                raise SystemExit(f"Expected JSON object in {path}:{line_number}")
            rows.append(row)
    return rows


def iter_jsonl(path: Path) -> Iterable[dict[str, Any]]:
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            if line_number == 1 and line.startswith("version https://git-lfs.github.com/spec/"):
                raise SystemExit(
                    f"{path} is a Git LFS pointer; hydrate retained experiment data with "
                    "git lfs checkout/pull or pass a run-local tables directory"
                )
            row = json.loads(line)
            if not isinstance(row, dict):
                raise SystemExit(f"Expected JSON object in {path}:{line_number}")
            yield row


def write_jsonl(path: Path, rows: Iterable[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True, separators=(",", ":")))
            handle.write("\n")


def write_json(path: Path, row: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        json.dump(row, handle, indent=2, sort_keys=True)
        handle.write("\n")


def provenance_by_poly_id(provenance_rows: list[dict[str, Any]]) -> dict[str, list[dict[str, Any]]]:
    by_poly_id: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in provenance_rows:
        poly_id = str(row.get("poly_id", ""))
        if poly_id:
            by_poly_id[poly_id].append(row)
    return dict(by_poly_id)


def datasets_for(polytope_row: dict[str, Any], provenance_rows: list[dict[str, Any]]) -> set[str]:
    datasets = {str(row.get("dataset", "")) for row in provenance_rows if row.get("dataset")}
    capacity_source = str(polytope_row.get("capacity_source", ""))
    if capacity_source:
        datasets.add(capacity_source)
    return datasets


def has_excluded_optimizer(provenance_rows: list[dict[str, Any]]) -> bool:
    for row in provenance_rows:
        optimizer = str(row.get("optimizer", "")).lower()
        role = str(row.get("role", "")).lower()
        dataset = str(row.get("dataset", "")).lower()
        if any(word in optimizer for word in EXCLUDED_OPTIMIZER_WORDS):
            return True
        if any(word in role for word in EXCLUDED_OPTIMIZER_WORDS):
            return True
        if any(word in dataset for word in EXCLUDED_DATASET_WORDS):
            return True
    return False


def is_trusted_random_polytope(
    polytope_row: dict[str, Any], provenance_rows: list[dict[str, Any]]
) -> bool:
    datasets = datasets_for(polytope_row, provenance_rows)
    if not datasets:
        return False
    if not datasets.issubset(TRUSTED_DATASETS):
        return False
    return not has_excluded_optimizer(provenance_rows)


def is_reference_holdout_polytope(
    polytope_row: dict[str, Any], provenance_rows: list[dict[str, Any]]
) -> bool:
    datasets = datasets_for(polytope_row, provenance_rows)
    if not datasets:
        return False
    if not datasets.issubset(REFERENCE_HOLDOUT_DATASETS):
        return False
    roles = {str(row.get("role", "")) for row in provenance_rows if row.get("role")}
    return roles == {"reference_holdout"}


def load_trusted_random_tables(
    tables_dir: Path = TABLES_DIR,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    polytope_rows = load_jsonl(tables_dir / "polytope-table.jsonl")
    provenance_rows = load_jsonl(tables_dir / "polytope-provenance-table.jsonl")
    provenance = provenance_by_poly_id(provenance_rows)

    trusted_poly_ids = {
        str(row["poly_id"])
        for row in polytope_rows
        if is_trusted_random_polytope(row, provenance.get(str(row["poly_id"]), []))
    }
    trusted_polytopes = [row for row in polytope_rows if str(row["poly_id"]) in trusted_poly_ids]
    trusted_provenance = [
        row for row in provenance_rows if str(row.get("poly_id", "")) in trusted_poly_ids
    ]
    return trusted_polytopes, trusted_provenance


def load_reference_holdout_tables(
    tables_dir: Path = TABLES_DIR,
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    polytope_rows = load_jsonl(tables_dir / "polytope-table.jsonl")
    provenance_rows = load_jsonl(tables_dir / "polytope-provenance-table.jsonl")
    provenance = provenance_by_poly_id(provenance_rows)

    reference_poly_ids = {
        str(row["poly_id"])
        for row in polytope_rows
        if is_reference_holdout_polytope(row, provenance.get(str(row["poly_id"]), []))
    }
    reference_polytopes = [row for row in polytope_rows if str(row["poly_id"]) in reference_poly_ids]
    reference_provenance = [
        row for row in provenance_rows if str(row.get("poly_id", "")) in reference_poly_ids
    ]
    return reference_polytopes, reference_provenance


def dataset_label(polytope_row: dict[str, Any], provenance_rows: list[dict[str, Any]]) -> str:
    datasets = sorted(datasets_for(polytope_row, provenance_rows))
    return ", ".join(datasets) if datasets else "-"


def product_bucket(provenance_rows: list[dict[str, Any]]) -> str:
    explicit = sorted(
        {
            f"{int(row['product_k'])}x{int(row['product_m'])}"
            for row in provenance_rows
            if isinstance(row.get("product_k"), int) and isinstance(row.get("product_m"), int)
        }
    )
    if len(explicit) == 1:
        return explicit[0]
    if len(explicit) > 1:
        return "multi:" + ",".join(explicit)
    paths = sorted({str(row.get("path", "")) for row in provenance_rows if row.get("path")})
    for path in paths:
        if path.startswith("lp_"):
            return path.removeprefix("lp_")
    return "unknown"


def active_invariant_numeric_feature_names(
    rows: list[dict[str, Any]],
    min_present_fraction: float = 0.98,
    require_all: bool = True,
) -> list[str]:
    if not rows:
        return []
    ordering_failures = [
        row.get("poly_id", f"row:{index}")
        for index, row in enumerate(rows)
        if int(row.get("ridge_symp_area_ordering_failure_count", 0)) != 0
        or float(row.get("ridge_symp_area_ordered_fraction", 1.0)) != 1.0
    ]
    if ordering_failures:
        preview = ", ".join(str(poly_id) for poly_id in ordering_failures[:5])
        raise SystemExit(
            "active invariant feature schema requires complete two-face ordering; "
            f"{len(ordering_failures)} rows failed, examples: {preview}"
        )

    threshold = max(1, int(len(rows) * min_present_fraction))
    names: list[str] = []
    missing: list[str] = []
    for key in ACTIVE_INVARIANT_NUMERIC_FEATURES:
        present = 0
        for row in rows:
            value = row.get(key)
            if isinstance(value, int | float) and np.isfinite(float(value)):
                present += 1
        if present >= threshold:
            names.append(key)
        else:
            missing.append(key)
    if missing and require_all:
        raise SystemExit(
            "active invariant feature schema is missing required numeric fields: "
            + ", ".join(missing)
        )
    return names


def matrix_for(rows: list[dict[str, Any]], names: list[str]) -> list[list[float]]:
    matrix: list[list[float]] = []
    for row in rows:
        values: list[float] = []
        for name in names:
            value = row.get(name)
            values.append(
                float(value)
                if isinstance(value, int | float) and np.isfinite(float(value))
                else 0.0
            )
        matrix.append(values)
    return matrix
