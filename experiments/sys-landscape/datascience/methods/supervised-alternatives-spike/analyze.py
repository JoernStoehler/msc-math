#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""
Goal: Run the DS-I005 supervised-alternatives spike for cheap standard
      regressors/classifiers on the hostile-landscape datascience tables.
Input Artifacts:
  - dataset directory passed by `--dataset-dir`, containing
    `polytope-table.jsonl` and `observation-table.jsonl`
Output Artifacts:
  - experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/summary.json
    as an existing historical sidecar, not a default requirement for new methods
  - experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/REPORT.md
"""

from __future__ import annotations

import argparse
import json
import math
import platform
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import numpy as np
from sklearn.base import clone
from sklearn.ensemble import ExtraTreesClassifier, ExtraTreesRegressor, HistGradientBoostingRegressor
from sklearn.feature_extraction import DictVectorizer
from sklearn.impute import SimpleImputer
from sklearn.linear_model import ElasticNet, Lasso
from sklearn.metrics import (
    accuracy_score,
    balanced_accuracy_score,
    mean_squared_error,
    r2_score,
    roc_auc_score,
)
from sklearn.model_selection import GroupKFold
from sklearn.neighbors import KNeighborsClassifier, KNeighborsRegressor
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import MaxAbsScaler, StandardScaler
from sklearn.utils import shuffle

try:
    from sklearn.model_selection import StratifiedGroupKFold
except ImportError:  # pragma: no cover - old sklearn fallback.
    StratifiedGroupKFold = None


EXPERIMENT_DIR = Path(__file__).resolve().parent
SUMMARY_JSON = EXPERIMENT_DIR / "summary.json"
REPORT_MD = EXPERIMENT_DIR / "REPORT.md"

ENDPOINT_DATASETS = {
    "gradient_ascent_general",
    "gradient_ascent_products",
    "variable_f_ascent",
}
RANDOM_DATASETS = {
    "random_sample",
    "random_product_sample",
}

EXPECTED_POLY_ROWS = 282
EXPECTED_OBS_ROWS = 282
EXPECTED_MAX_SYS = 0.906316153431123
EXPECTED_SYS_GT_ONE = 0

EXCLUDED_POLY_KEYS = {
    "poly_id",
    "dual_vertices_rational",
    "dual_vertices_f64",
    "dual_vertices_flat_f64",
    "capacity",
    "capacity_source",
    "volume",
    "sys",
    "sigmas",
}
ORBIT_SEARCH_KEYS = {
    "orbit_kkt_available",
    "orbit_search_scalar_available",
    "orbit_result_iterations_log1p",
    "orbit_result_returned_orbit_count",
    "orbit_best_beta_margin",
    "orbit_best_q_error_bound",
    "orbit_best_has_mu",
    "orbit_best_has_xi",
    "orbit_best_is_admissible_exact",
    "orbit_best_is_indeterminate_f64",
}


@dataclass(frozen=True)
class Row:
    observation_id: str
    poly_id: str
    regime: str
    group_id: str
    sys: float
    intrinsic_features: dict[str, float]
    intrinsic_without_orbit_search: dict[str, float]
    metadata_features: dict[str, str | float]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--dataset-dir",
        type=Path,
        required=True,
        help="Frozen dataset directory with polytope and observation JSONL tables.",
    )
    parser.add_argument(
        "--permutations",
        type=int,
        default=50,
        help="Number of cheap random-to-endpoint label permutations.",
    )
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def classify_regime(dataset: str) -> str:
    if dataset in ENDPOINT_DATASETS:
        return "endpoint"
    if dataset in RANDOM_DATASETS:
        return "random"
    raise ValueError(f"unexpected dataset {dataset!r}")


def grouped_split_id(observation: dict[str, Any], regime: str) -> str:
    if observation.get("root_group_id"):
        return str(observation["root_group_id"])
    if regime == "endpoint" and observation.get("source_name"):
        return str(observation["source_name"])
    return str(observation.get("lineage_id") or observation["observation_id"])


def is_plain_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(float(value))


def intrinsic_feature_dict(poly: dict[str, Any], *, include_orbit_search: bool) -> dict[str, float]:
    features: dict[str, float] = {}
    for key, value in poly.items():
        if key in EXCLUDED_POLY_KEYS:
            continue
        if not include_orbit_search and key in ORBIT_SEARCH_KEYS:
            continue
        if is_plain_number(value):
            features[key] = float(value)
    return features


def metadata_feature_dict(observation: dict[str, Any], poly: dict[str, Any]) -> dict[str, str | float]:
    return {
        "facet_count": float(poly["facet_count"]),
        "dataset": str(observation["dataset"]),
        "family": str(observation["family"]),
        "role": str(observation["role"]),
        "search_space": str(observation["search_space"]),
        "optimizer": str(observation["optimizer"]),
        "backend": str(observation["backend"]),
    }


def load_rows(dataset_dir: Path) -> tuple[list[Row], dict[str, Any]]:
    poly_path = dataset_dir / "polytope-table.jsonl"
    obs_path = dataset_dir / "observation-table.jsonl"
    polytopes = load_jsonl(poly_path)
    observations = load_jsonl(obs_path)
    by_poly_id = {row["poly_id"]: row for row in polytopes}

    max_sys = max(float(row["sys"]) for row in polytopes)
    sys_gt_one = sum(1 for row in polytopes if float(row["sys"]) > 1.0)
    checks = {
        "polytope_rows": len(polytopes),
        "observation_rows": len(observations),
        "unique_polytope_ids": len(by_poly_id),
        "unique_observation_poly_ids": len({row["poly_id"] for row in observations}),
        "max_sys": max_sys,
        "sys_gt_one_count": sys_gt_one,
        "expected_polytope_rows": EXPECTED_POLY_ROWS,
        "expected_observation_rows": EXPECTED_OBS_ROWS,
        "expected_max_sys": EXPECTED_MAX_SYS,
        "expected_sys_gt_one_count": EXPECTED_SYS_GT_ONE,
        "passed": (
            len(polytopes) == EXPECTED_POLY_ROWS
            and len(observations) == EXPECTED_OBS_ROWS
            and abs(max_sys - EXPECTED_MAX_SYS) <= 1e-15
            and sys_gt_one == EXPECTED_SYS_GT_ONE
        ),
    }
    if not checks["passed"]:
        raise ValueError(f"dataset guard mismatch: {checks}")
    if sys_gt_one:
        raise ValueError(f"stop condition hit: sys > 1 count is {sys_gt_one}")

    rows: list[Row] = []
    for observation in observations:
        poly = by_poly_id[observation["poly_id"]]
        regime = classify_regime(str(observation["dataset"]))
        rows.append(
            Row(
                observation_id=str(observation["observation_id"]),
                poly_id=str(observation["poly_id"]),
                regime=regime,
                group_id=grouped_split_id(observation, regime),
                sys=float(poly["sys"]),
                intrinsic_features=intrinsic_feature_dict(poly, include_orbit_search=True),
                intrinsic_without_orbit_search=intrinsic_feature_dict(
                    poly, include_orbit_search=False
                ),
                metadata_features=metadata_feature_dict(observation, poly),
            )
        )
    return rows, checks


def regression_models() -> dict[str, Any]:
    return {
        "lasso": make_pipeline(
            SimpleImputer(strategy="median"),
            StandardScaler(),
            Lasso(alpha=0.001, max_iter=20000, random_state=0),
        ),
        "elastic_net": make_pipeline(
            SimpleImputer(strategy="median"),
            StandardScaler(),
            ElasticNet(alpha=0.001, l1_ratio=0.5, max_iter=20000, random_state=0),
        ),
        "hist_gradient_boosting": make_pipeline(
            SimpleImputer(strategy="median"),
            HistGradientBoostingRegressor(max_iter=80, learning_rate=0.05, random_state=0),
        ),
        "extra_trees": make_pipeline(
            SimpleImputer(strategy="median"),
            ExtraTreesRegressor(
                n_estimators=250,
                min_samples_leaf=2,
                random_state=0,
                n_jobs=-1,
            ),
        ),
        "knn": make_pipeline(
            SimpleImputer(strategy="median"),
            MaxAbsScaler(),
            KNeighborsRegressor(n_neighbors=7, weights="distance"),
        ),
    }


def classification_models() -> dict[str, Any]:
    return {
        "extra_trees": make_pipeline(
            SimpleImputer(strategy="median"),
            ExtraTreesClassifier(
                n_estimators=250,
                min_samples_leaf=2,
                class_weight="balanced",
                random_state=0,
                n_jobs=-1,
            ),
        ),
        "knn": make_pipeline(
            SimpleImputer(strategy="median"),
            MaxAbsScaler(),
            KNeighborsClassifier(n_neighbors=7, weights="distance"),
        ),
    }


def vectorize(feature_dicts: list[dict[str, Any]]) -> tuple[DictVectorizer, Any]:
    vectorizer = DictVectorizer(sparse=False)
    return vectorizer, vectorizer.fit_transform(feature_dicts)


def transform(vectorizer: DictVectorizer, feature_dicts: list[dict[str, Any]]) -> Any:
    return vectorizer.transform(feature_dicts)


def score_regression(y_true: np.ndarray, y_pred: np.ndarray) -> dict[str, float]:
    return {
        "r2": float(r2_score(y_true, y_pred)),
        "rmse": float(math.sqrt(mean_squared_error(y_true, y_pred))),
    }


def score_classification(y_true: np.ndarray, y_prob: np.ndarray) -> dict[str, float]:
    y_pred = (y_prob >= 0.5).astype(int)
    return {
        "accuracy": float(accuracy_score(y_true, y_pred)),
        "balanced_accuracy": float(balanced_accuracy_score(y_true, y_pred)),
        "roc_auc": float(roc_auc_score(y_true, y_prob)),
    }


def feature_dict(row: Row, block: str) -> dict[str, Any]:
    if block == "intrinsic_numeric":
        return dict(row.intrinsic_features)
    if block == "intrinsic_no_orbit_search":
        return dict(row.intrinsic_without_orbit_search)
    if block == "metadata_caveat":
        return dict(row.metadata_features)
    raise ValueError(f"unknown feature block {block}")


def evaluate_grouped_regression(
    rows: list[Row], block: str, model_name: str, model: Any
) -> dict[str, Any]:
    y = np.asarray([row.sys for row in rows], dtype=float)
    groups = np.asarray([row.group_id for row in rows])
    unique_groups = sorted(set(groups.tolist()))
    n_splits = min(5, len(unique_groups))
    if n_splits < 2:
        return {"r2": math.nan, "rmse": math.nan, "folds": 0, "groups": len(unique_groups)}

    preds = np.zeros_like(y)
    splitter = GroupKFold(n_splits=n_splits)
    if block == "null_mean":
        for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
            preds[test_idx] = float(np.mean(y[train_idx]))
        metrics = score_regression(y, preds)
        return {**metrics, "folds": n_splits, "groups": len(unique_groups)}

    dicts = [feature_dict(row, block) for row in rows]
    for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
        vectorizer, x_train = vectorize([dicts[i] for i in train_idx])
        x_test = transform(vectorizer, [dicts[i] for i in test_idx])
        fitted = clone(model)
        fitted.fit(x_train, y[train_idx])
        preds[test_idx] = fitted.predict(x_test)
    metrics = score_regression(y, preds)
    return {**metrics, "folds": n_splits, "groups": len(unique_groups)}


def evaluate_transfer(
    train_rows: list[Row], test_rows: list[Row], block: str, model_name: str, model: Any
) -> dict[str, Any]:
    y_train = np.asarray([row.sys for row in train_rows], dtype=float)
    y_test = np.asarray([row.sys for row in test_rows], dtype=float)
    if block == "null_mean":
        preds = np.full_like(y_test, float(np.mean(y_train)))
        metrics = score_regression(y_test, preds)
        return {**metrics, "train_rows": len(train_rows), "test_rows": len(test_rows)}

    train_dicts = [feature_dict(row, block) for row in train_rows]
    test_dicts = [feature_dict(row, block) for row in test_rows]
    vectorizer, x_train = vectorize(train_dicts)
    x_test = transform(vectorizer, test_dicts)
    fitted = clone(model)
    fitted.fit(x_train, y_train)
    preds = fitted.predict(x_test)
    metrics = score_regression(y_test, preds)
    return {**metrics, "train_rows": len(train_rows), "test_rows": len(test_rows)}


def permutation_transfer_null(
    random_rows: list[Row],
    endpoint_rows: list[Row],
    block: str,
    model_name: str,
    model: Any,
    permutations: int,
) -> dict[str, float]:
    rng = np.random.default_rng(0)
    y_random = np.asarray([row.sys for row in random_rows], dtype=float)
    y_endpoint = np.asarray([row.sys for row in endpoint_rows], dtype=float)
    train_dicts = [feature_dict(row, block) for row in random_rows]
    test_dicts = [feature_dict(row, block) for row in endpoint_rows]
    vectorizer, x_train = vectorize(train_dicts)
    x_test = transform(vectorizer, test_dicts)
    r2s: list[float] = []
    for _ in range(permutations):
        fitted = clone(model)
        fitted.fit(x_train, rng.permutation(y_random))
        preds = fitted.predict(x_test)
        r2s.append(float(r2_score(y_endpoint, preds)))
    arr = np.asarray(r2s, dtype=float)
    return {
        "permutations": permutations,
        "r2_p05": float(np.quantile(arr, 0.05)),
        "r2_median": float(np.quantile(arr, 0.5)),
        "r2_p95": float(np.quantile(arr, 0.95)),
    }


def make_classification_splitter(y: np.ndarray, groups: np.ndarray):
    class_counts = np.bincount(y.astype(int))
    n_splits = min(5, len(set(groups.tolist())), int(class_counts.min()))
    if n_splits < 2:
        raise ValueError("need at least two groups and class members for grouped classification")
    if StratifiedGroupKFold is not None:
        return StratifiedGroupKFold(n_splits=n_splits, shuffle=True, random_state=0), n_splits
    return GroupKFold(n_splits=n_splits), n_splits


def evaluate_grouped_classification(
    rows: list[Row], block: str, model_name: str, model: Any
) -> dict[str, Any]:
    y = np.asarray([1 if row.regime == "endpoint" else 0 for row in rows], dtype=int)
    groups = np.asarray([row.group_id for row in rows])
    splitter, n_splits = make_classification_splitter(y, groups)
    probs = np.zeros(len(rows), dtype=float)

    if block == "null_rate":
        for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
            probs[test_idx] = float(np.mean(y[train_idx]))
        metrics = score_classification(y, probs)
        return {**metrics, "folds": n_splits, "groups": len(set(groups.tolist()))}

    dicts = [feature_dict(row, block) for row in rows]
    for train_idx, test_idx in splitter.split(np.zeros(len(rows)), y, groups):
        vectorizer, x_train = vectorize([dicts[i] for i in train_idx])
        x_test = transform(vectorizer, [dicts[i] for i in test_idx])
        fitted = clone(model)
        fitted.fit(x_train, y[train_idx])
        if hasattr(fitted, "predict_proba"):
            probs[test_idx] = fitted.predict_proba(x_test)[:, 1]
        else:
            probs[test_idx] = fitted.predict(x_test)
    metrics = score_classification(y, probs)
    return {**metrics, "folds": n_splits, "groups": len(set(groups.tolist()))}


def best_by(results: list[dict[str, Any]], metric: str, *, surface: str, blocks: set[str]) -> dict[str, Any]:
    candidates = [
        result
        for result in results
        if result["surface"] == surface and result["feature_block"] in blocks and math.isfinite(result[metric])
    ]
    return max(candidates, key=lambda result: result[metric])


def summarize(rows: list[Row], dataset_dir: Path, checks: dict[str, Any], permutations: int) -> dict[str, Any]:
    random_rows = [row for row in rows if row.regime == "random"]
    endpoint_rows = [row for row in rows if row.regime == "endpoint"]
    regression_blocks = ["null_mean", "intrinsic_no_orbit_search", "intrinsic_numeric", "metadata_caveat"]
    classification_blocks = ["null_rate", "intrinsic_no_orbit_search", "intrinsic_numeric", "metadata_caveat"]
    regressors = regression_models()
    classifiers = classification_models()

    regression_results: list[dict[str, Any]] = []
    for model_name, model in regressors.items():
        for block in regression_blocks:
            for surface, surface_rows in [
                ("within_random", random_rows),
                ("within_endpoint", endpoint_rows),
            ]:
                metrics = evaluate_grouped_regression(surface_rows, block, model_name, model)
                regression_results.append(
                    {
                        "surface": surface,
                        "model": model_name,
                        "feature_block": block,
                        **metrics,
                    }
                )
            transfer = evaluate_transfer(random_rows, endpoint_rows, block, model_name, model)
            regression_results.append(
                {
                    "surface": "random_to_endpoint",
                    "model": model_name,
                    "feature_block": block,
                    **transfer,
                }
            )

    permutation_results: list[dict[str, Any]] = []
    for model_name, model in regressors.items():
        for block in ["intrinsic_no_orbit_search", "intrinsic_numeric"]:
            metrics = permutation_transfer_null(
                random_rows, endpoint_rows, block, model_name, model, permutations
            )
            real = next(
                result
                for result in regression_results
                if result["surface"] == "random_to_endpoint"
                and result["model"] == model_name
                and result["feature_block"] == block
            )
            permutation_results.append(
                {
                    "surface": "random_to_endpoint",
                    "model": model_name,
                    "feature_block": block,
                    "real_r2": real["r2"],
                    **metrics,
                }
            )

    classification_results: list[dict[str, Any]] = []
    for model_name, model in classifiers.items():
        for block in classification_blocks:
            metrics = evaluate_grouped_classification(rows, block, model_name, model)
            classification_results.append(
                {
                    "surface": "endpoint_vs_random",
                    "model": model_name,
                    "feature_block": block,
                    **metrics,
                }
            )

    claim_blocks = {"intrinsic_no_orbit_search", "intrinsic_numeric"}
    best_transfer = best_by(
        regression_results,
        "r2",
        surface="random_to_endpoint",
        blocks=claim_blocks,
    )
    best_within_random = best_by(
        regression_results,
        "r2",
        surface="within_random",
        blocks=claim_blocks,
    )
    best_within_endpoint = best_by(
        regression_results,
        "r2",
        surface="within_endpoint",
        blocks=claim_blocks,
    )
    best_classification = best_by(
        classification_results,
        "balanced_accuracy",
        surface="endpoint_vs_random",
        blocks=claim_blocks,
    )

    verdict = "negative"
    evidence_strength = "medium"
    implementation_trust = "medium"
    thesis_use = "supporting/caveat only"
    caveat = (
        "Frozen 282-row table only; feature matrix excludes target/capacity, raw arrays, ids, "
        "and observation provenance for claim-bearing blocks; the `intrinsic_numeric` block "
        "still includes cached orbit-search scalar features, so `intrinsic_no_orbit_search` is "
        "the cleaner geometry-side sensitivity. The method panel is small and cheap."
    )
    if best_transfer["r2"] > 0:
        verdict = "conjectured-positive"
        evidence_strength = "low"
        thesis_use = "Jorn decision needed"

    return {
        "idea_id": "DS-I005",
        "generated_at_utc": datetime.now(timezone.utc).isoformat(),
        "command": (
            "uv run --script experiments/sys-landscape/datascience/methods/"
            "supervised-alternatives-spike/analyze.py --dataset-dir "
            f"{dataset_dir} --permutations {permutations}"
        ),
        "dataset": {
            "path": str(dataset_dir),
            "producer_command": (
                "cargo run -p exp-sys-landscape --bin sys-dataset -- "
                "--out-dir /tmp/sys-ds-pilot1-tables-tH33Hr"
            ),
            "checks": checks,
        },
        "row_counts": {
            "total": len(rows),
            "random": len(random_rows),
            "endpoint": len(endpoint_rows),
            "groups": len({row.group_id for row in rows}),
            "random_groups": len({row.group_id for row in random_rows}),
            "endpoint_groups": len({row.group_id for row in endpoint_rows}),
            "unique_polytope_rows": len({row.poly_id for row in rows}),
        },
        "feature_scope": {
            "claim_bearing_blocks": ["intrinsic_no_orbit_search", "intrinsic_numeric"],
            "caveat_comparison_block": "metadata_caveat",
            "excluded_polytope_keys": sorted(EXCLUDED_POLY_KEYS),
            "excluded_observation_provenance": [
                "observation_id",
                "dataset",
                "family",
                "role",
                "search_space",
                "optimizer",
                "backend",
                "source_name",
                "root_group_id",
                "lineage_id",
                "seed_index",
            ],
            "intrinsic_numeric_feature_count": len(rows[0].intrinsic_features),
            "intrinsic_no_orbit_search_feature_count": len(
                rows[0].intrinsic_without_orbit_search
            ),
        },
        "split_policy": {
            "regression_cv": "GroupKFold by root_group_id, endpoint source fallback, lineage fallback",
            "classification_cv": "StratifiedGroupKFold when available, same groups",
            "transfer": "train all random rows and score all endpoint rows; this is the load-bearing search-usefulness surface",
            "unique_polytope_note": (
                "The frozen snapshot has one observation per polytope, but grouping is still used "
                "because the producer lineage/root fields can couple rows through common starts or sources."
            ),
        },
        "model_panel": {
            "regression": sorted(regressors.keys()),
            "classification": sorted(classifiers.keys()),
            "excluded": ["neural_net", "bayesian_optimization"],
        },
        "regression_results": regression_results,
        "permutation_transfer_null": permutation_results,
        "classification_results": classification_results,
        "key_results": {
            "best_random_to_endpoint_intrinsic": best_transfer,
            "best_within_random_intrinsic": best_within_random,
            "best_within_endpoint_intrinsic": best_within_endpoint,
            "best_endpoint_vs_random_intrinsic": best_classification,
        },
        "verdict": verdict,
        "evidence_strength": evidence_strength,
        "implementation_trust": implementation_trust,
        "thesis_use": thesis_use,
        "caveat": caveat,
        "reopen_trigger": (
            "Reopen if refreshed tables add materially more random/endpoint rows, if new non-provenance "
            "features make random-to-endpoint R^2 nonnegative under grouped transfer, or if a reviewer "
            "finds a leakage bug in the feature exclusion or group policy."
        ),
        "environment": {
            "python": platform.python_version(),
            "platform": platform.platform(),
        },
    }


def fmt(value: float) -> str:
    if math.isnan(value):
        return "nan"
    return f"{value:.4f}"


def table_rows(rows: list[dict[str, Any]], metric: str, surfaces: list[str]) -> list[str]:
    lines = [
        "| surface | model | feature block | " + metric + " | secondary |",
        "| --- | --- | --- | ---: | ---: |",
    ]
    for surface in surfaces:
        candidates = [row for row in rows if row["surface"] == surface]
        candidates = sorted(candidates, key=lambda row: row.get(metric, -math.inf), reverse=True)[:6]
        for row in candidates:
            secondary_name = "rmse" if "rmse" in row else "roc_auc"
            lines.append(
                f"| `{row['surface']}` | `{row['model']}` | `{row['feature_block']}` | "
                f"{fmt(float(row[metric]))} | {fmt(float(row[secondary_name]))} |"
            )
    return lines


def write_report(summary: dict[str, Any]) -> None:
    key = summary["key_results"]
    checks = summary["dataset"]["checks"]
    lines = [
        "# DS-I005 Supervised Alternatives Spike",
        "",
        "## Command And Provenance",
        "",
        f"- command: `{summary['command']}`",
        f"- dataset path: `{summary['dataset']['path']}`",
        f"- producer command: `{summary['dataset']['producer_command']}`",
        f"- generated at UTC: `{summary['generated_at_utc']}`",
        "",
        "## Dataset Snapshot And Guards",
        "",
        f"- polytope rows: `{checks['polytope_rows']}`",
        f"- observation rows: `{checks['observation_rows']}`",
        f"- max `sys`: `{checks['max_sys']}`",
        f"- `sys > 1` count: `{checks['sys_gt_one_count']}`",
        f"- guard status: `{'passed' if checks['passed'] else 'failed'}`",
        f"- random rows/groups: `{summary['row_counts']['random']}` / `{summary['row_counts']['random_groups']}`",
        f"- endpoint rows/groups: `{summary['row_counts']['endpoint']}` / `{summary['row_counts']['endpoint_groups']}`",
        "",
        "## Method",
        "",
        "Observation: the claim-bearing matrix uses numeric scalar columns from `polytope-table.jsonl`.",
        "It excludes `sys`, capacity, raw vertex/sigma arrays, ids, and observation provenance.",
        "The `metadata_caveat` block is reported only as a provenance comparison, not as a geometry-based search heuristic.",
        "",
        f"- regression panel: `{', '.join(summary['model_panel']['regression'])}`",
        f"- classification panel: `{', '.join(summary['model_panel']['classification'])}`",
        f"- claim-bearing feature counts: `intrinsic_no_orbit_search={summary['feature_scope']['intrinsic_no_orbit_search_feature_count']}`, `intrinsic_numeric={summary['feature_scope']['intrinsic_numeric_feature_count']}`",
        "- split policy: grouped CV by root/lineage/source fields; random-to-endpoint transfer trains on random rows and scores endpoint rows.",
        "",
        "## Observations",
        "",
        "Best claim-bearing regression surfaces:",
        "",
        f"- random-to-endpoint: `{key['best_random_to_endpoint_intrinsic']['model']}` on `{key['best_random_to_endpoint_intrinsic']['feature_block']}` with `R^2={fmt(key['best_random_to_endpoint_intrinsic']['r2'])}`, `RMSE={fmt(key['best_random_to_endpoint_intrinsic']['rmse'])}`",
        f"- within-random: `{key['best_within_random_intrinsic']['model']}` on `{key['best_within_random_intrinsic']['feature_block']}` with `R^2={fmt(key['best_within_random_intrinsic']['r2'])}`, `RMSE={fmt(key['best_within_random_intrinsic']['rmse'])}`",
        f"- within-endpoint: `{key['best_within_endpoint_intrinsic']['model']}` on `{key['best_within_endpoint_intrinsic']['feature_block']}` with `R^2={fmt(key['best_within_endpoint_intrinsic']['r2'])}`, `RMSE={fmt(key['best_within_endpoint_intrinsic']['rmse'])}`",
        "",
        "Top regression rows:",
        "",
        *table_rows(
            summary["regression_results"],
            "r2",
            ["random_to_endpoint", "within_random", "within_endpoint"],
        ),
        "",
        "Random-to-endpoint permutation null for claim-bearing blocks:",
        "",
        "| model | feature block | real R^2 | permuted p05 | permuted median | permuted p95 |",
        "| --- | --- | ---: | ---: | ---: | ---: |",
    ]
    for row in summary["permutation_transfer_null"]:
        lines.append(
            f"| `{row['model']}` | `{row['feature_block']}` | {fmt(row['real_r2'])} | "
            f"{fmt(row['r2_p05'])} | {fmt(row['r2_median'])} | {fmt(row['r2_p95'])} |"
        )
    lines.extend(
        [
            "",
            "Endpoint-vs-random classification:",
            "",
            f"- best claim-bearing classifier: `{key['best_endpoint_vs_random_intrinsic']['model']}` on `{key['best_endpoint_vs_random_intrinsic']['feature_block']}` with balanced accuracy `{fmt(key['best_endpoint_vs_random_intrinsic']['balanced_accuracy'])}` and ROC AUC `{fmt(key['best_endpoint_vs_random_intrinsic']['roc_auc'])}`.",
            "",
            *table_rows(summary["classification_results"], "balanced_accuracy", ["endpoint_vs_random"]),
            "",
            "## Inference",
            "",
            "The cheap supervised alternatives do not change the M011 search-usefulness story under the load-bearing random-to-endpoint surface. The best claim-bearing random-to-endpoint `R^2` remains negative, even when flexible tree and kNN alternatives are allowed. Within-regime fits can be positive, but they do not transfer from random samples to endpoint rows.",
            "",
            "For the M012-style regime question, non-provenance numeric polytope features can still separate endpoint and random regimes. That is a table/regime observation, not a generator rule for finding new high-`sys` candidates; the metadata/provenance baseline is kept only as a caveat comparison.",
            "",
            "## Verdict",
            "",
            f"- verdict: `{summary['verdict']}`",
            f"- evidence_strength: `{summary['evidence_strength']}`",
            f"- implementation_trust: `{summary['implementation_trust']}`",
            f"- thesis_use: `{summary['thesis_use']}`",
            f"- caveat: {summary['caveat']}",
            f"- reopen trigger: {summary['reopen_trigger']}",
        ]
    )
    REPORT_MD.write_text("\n".join(lines) + "\n")


def main() -> None:
    args = parse_args()
    rows, checks = load_rows(args.dataset_dir)
    summary = summarize(rows, args.dataset_dir, checks, args.permutations)
    SUMMARY_JSON.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    write_report(summary)
    print(json.dumps(summary["key_results"], indent=2, sort_keys=True))
    print(f"wrote {SUMMARY_JSON}")
    print(f"wrote {REPORT_MD}")


if __name__ == "__main__":
    main()
