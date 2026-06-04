#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""
Goal: Rerun the DS-I004 PCA / clustering / anomaly scan as a repo-owned
      source-truth artifact over a sys-landscape dataset table.
Input Artifacts:
  - a dataset directory passed by `--dataset-dir` containing
    `polytope-table.jsonl` and `observation-table.jsonl`
Output Artifacts:
  - experiments/sys-landscape/datascience/methods/pca-cluster-spike/report.md
"""

from __future__ import annotations

import argparse
import json
import math
import subprocess
from collections import Counter
from pathlib import Path
from typing import Any

import numpy as np
from sklearn.cluster import KMeans
from sklearn.decomposition import PCA
from sklearn.ensemble import IsolationForest
from sklearn.metrics import silhouette_score
from sklearn.preprocessing import StandardScaler

EXPERIMENT_DIR = Path(__file__).resolve().parent
DEFAULT_DATASET_DIR = EXPERIMENT_DIR.parent.parent / "dataset"
EXPECTED_POLYTOPE_ROWS = 282
EXPECTED_OBSERVATION_ROWS = 282
EXPECTED_MAX_SYS = 0.906316153431123
EXPECTED_SYS_GT_ONE = 0
RANDOM_STATE = 20260430

POLYTOPE_EXCLUDED_EXACT = {
    "poly_id",
    "capacity",
    "capacity_source",
    "sys",
    "sigma_gap_cutoff",
    "sigmas",
    "dual_vertices_rational",
    "dual_vertices_f64",
    "dual_vertices_flat_f64",
}
POLYTOPE_EXCLUDED_PREFIXES = (
    "orbit_sigma_",
    "orbit_kkt_",
    "orbit_search_",
    "orbit_result_",
    "orbit_best_",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--dataset-dir",
        type=Path,
        default=DEFAULT_DATASET_DIR,
        help="Dataset directory. Defaults to experiments/sys-landscape/datascience/dataset.",
    )
    parser.add_argument("--out-dir", type=Path, default=EXPERIMENT_DIR)
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def git_commit() -> str | None:
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            check=True,
            capture_output=True,
            text=True,
        )
    except (subprocess.CalledProcessError, FileNotFoundError):
        return None
    return result.stdout.strip()


def is_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def include_polytope_feature(key: str, value: Any) -> bool:
    if key in POLYTOPE_EXCLUDED_EXACT:
        return False
    if key.startswith(POLYTOPE_EXCLUDED_PREFIXES):
        return False
    return is_number(value) or isinstance(value, bool)


def validate_dataset(
    polytope_rows: list[dict[str, Any]], observation_rows: list[dict[str, Any]]
) -> dict[str, Any]:
    max_sys = max(float(row["sys"]) for row in polytope_rows)
    sys_gt_one = sum(1 for row in polytope_rows if float(row["sys"]) > 1.0)
    checks = {
        "polytope_rows": len(polytope_rows),
        "observation_rows": len(observation_rows),
        "max_sys": max_sys,
        "sys_gt_one_count": sys_gt_one,
        "expected_polytope_rows": EXPECTED_POLYTOPE_ROWS,
        "expected_observation_rows": EXPECTED_OBSERVATION_ROWS,
        "expected_max_sys": EXPECTED_MAX_SYS,
        "expected_sys_gt_one_count": EXPECTED_SYS_GT_ONE,
        "max_sys_abs_error": abs(max_sys - EXPECTED_MAX_SYS),
    }
    errors: list[str] = []
    if checks["polytope_rows"] != EXPECTED_POLYTOPE_ROWS:
        errors.append("polytope row count mismatch")
    if checks["observation_rows"] != EXPECTED_OBSERVATION_ROWS:
        errors.append("observation row count mismatch")
    if not math.isclose(max_sys, EXPECTED_MAX_SYS, rel_tol=0.0, abs_tol=1e-15):
        errors.append("max sys mismatch")
    if sys_gt_one != EXPECTED_SYS_GT_ONE:
        errors.append("sys > 1 count mismatch")
    checks["passed"] = not errors
    checks["errors"] = errors
    return checks


def build_joined_rows(
    polytope_rows: list[dict[str, Any]], observation_rows: list[dict[str, Any]]
) -> list[dict[str, Any]]:
    by_poly_id = {row["poly_id"]: row for row in polytope_rows}
    if len(by_poly_id) != len(polytope_rows):
        raise ValueError("polytope-table.jsonl has duplicate poly_id values")
    joined: list[dict[str, Any]] = []
    for observation in observation_rows:
        poly_id = observation["poly_id"]
        if poly_id not in by_poly_id:
            raise ValueError(f"missing polytope row for observation poly_id {poly_id}")
        joined.append({"poly": by_poly_id[poly_id], "observation": observation})
    return joined


def feature_matrix(joined_rows: list[dict[str, Any]]) -> tuple[np.ndarray, list[str], dict[str, Any]]:
    candidate_keys = sorted(
        {
            key
            for row in joined_rows
            for key, value in row["poly"].items()
            if include_polytope_feature(key, value)
        }
    )
    dropped_missing: list[str] = []
    dropped_constant: list[str] = []
    feature_names: list[str] = []
    columns: list[list[float]] = []
    for key in candidate_keys:
        values: list[float] = []
        ok = True
        for row in joined_rows:
            value = row["poly"].get(key)
            if isinstance(value, bool):
                values.append(1.0 if value else 0.0)
            elif is_number(value):
                values.append(float(value))
            else:
                ok = False
                break
        if not ok:
            dropped_missing.append(key)
            continue
        if max(values) == min(values):
            dropped_constant.append(key)
            continue
        feature_names.append(key)
        columns.append(values)

    if not columns:
        raise ValueError("no nonconstant numeric non-provenance features remain")

    matrix = np.asarray(columns, dtype=float).T
    excluded_poly_keys = sorted(
        key
        for key in joined_rows[0]["poly"]
        if key in POLYTOPE_EXCLUDED_EXACT or key.startswith(POLYTOPE_EXCLUDED_PREFIXES)
    )
    excluded_observation_keys = sorted(joined_rows[0]["observation"].keys())
    feature_info = {
        "feature_count": len(feature_names),
        "feature_names": feature_names,
        "candidate_numeric_polytope_features": len(candidate_keys),
        "dropped_constant_features": dropped_constant,
        "dropped_missing_or_nonnumeric_features": dropped_missing,
        "excluded_polytope_columns": excluded_poly_keys,
        "excluded_observation_columns": excluded_observation_keys,
        "excluded_column_classes": [
            "target and capacity columns: sys, capacity, capacity_source",
            "raw vertex arrays and id columns",
            "sigma/orbit-search witness columns tied to capacity search",
            "all observation metadata, endpoint labels, dataset identity, and optimizer provenance",
        ],
    }
    return matrix, feature_names, feature_info


def regime_label(observation: dict[str, Any]) -> str:
    role = str(observation.get("role", ""))
    if "endpoint" in role:
        return "endpoint"
    return "random"


def top_loadings(pca: PCA, feature_names: list[str], component_index: int, limit: int = 8) -> list[dict[str, Any]]:
    loadings = pca.components_[component_index]
    order = np.argsort(np.abs(loadings))[::-1][:limit]
    return [
        {
            "feature": feature_names[int(index)],
            "loading": float(loadings[int(index)]),
            "abs_loading": float(abs(loadings[int(index)])),
        }
        for index in order
    ]


def pca_scan(x_scaled: np.ndarray, sys_values: np.ndarray, feature_names: list[str]) -> dict[str, Any]:
    n_components = min(8, x_scaled.shape[1], x_scaled.shape[0])
    pca = PCA(n_components=n_components, random_state=RANDOM_STATE)
    scores = pca.fit_transform(x_scaled)
    components: list[dict[str, Any]] = []
    for index in range(n_components):
        score = scores[:, index]
        corr = float(np.corrcoef(score, sys_values)[0, 1])
        abs_score = np.abs(score)
        high_score = abs_score >= np.quantile(abs_score, 0.8)
        components.append(
            {
                "component": index + 1,
                "explained_variance_ratio": float(pca.explained_variance_ratio_[index]),
                "corr_with_sys": corr,
                "abs_corr_with_sys": abs(corr),
                "top_abs_score_count": int(np.sum(high_score)),
                "top_abs_score_mean_sys": float(np.mean(sys_values[high_score])),
                "top_abs_score_max_sys": float(np.max(sys_values[high_score])),
                "top_loadings": top_loadings(pca, feature_names, index),
            }
        )
    best = max(components, key=lambda row: row["abs_corr_with_sys"])
    return {
        "components": components,
        "best_abs_corr_component": best,
        "total_explained_variance_first_8": float(np.sum(pca.explained_variance_ratio_)),
    }


def cluster_scan(
    x_scaled: np.ndarray, sys_values: np.ndarray, observations: list[dict[str, Any]]
) -> dict[str, Any]:
    high_sys_threshold = float(np.quantile(sys_values, 0.9))
    candidates: list[dict[str, Any]] = []
    for k in range(2, 9):
        model = KMeans(n_clusters=k, n_init=50, random_state=RANDOM_STATE)
        labels = model.fit_predict(x_scaled)
        silhouette = float(silhouette_score(x_scaled, labels))
        clusters: list[dict[str, Any]] = []
        for label in range(k):
            mask = labels == label
            datasets = Counter(str(observations[i]["dataset"]) for i, flag in enumerate(mask) if flag)
            regimes = Counter(regime_label(observations[i]) for i, flag in enumerate(mask) if flag)
            cluster_sys = sys_values[mask]
            clusters.append(
                {
                    "label": label,
                    "count": int(np.sum(mask)),
                    "mean_sys": float(np.mean(cluster_sys)),
                    "max_sys": float(np.max(cluster_sys)),
                    "high_sys_rate_top_decile": float(np.mean(cluster_sys >= high_sys_threshold)),
                    "dominant_dataset": datasets.most_common(1)[0][0],
                    "dominant_dataset_share": float(datasets.most_common(1)[0][1] / np.sum(mask)),
                    "regime_counts": dict(sorted(regimes.items())),
                }
            )
        candidates.append(
            {
                "k": k,
                "silhouette": silhouette,
                "clusters": clusters,
                "best_mean_sys_cluster": max(clusters, key=lambda row: row["mean_sys"]),
                "best_high_sys_rate_cluster": max(clusters, key=lambda row: row["high_sys_rate_top_decile"]),
            }
        )
    best = max(candidates, key=lambda row: row["silhouette"])
    all_clusters = [cluster for run in candidates for cluster in run["clusters"]]
    return {
        "high_sys_threshold_top_decile": high_sys_threshold,
        "runs": candidates,
        "selected_by_silhouette": best,
        "highest_mean_sys_cluster_across_k": max(all_clusters, key=lambda row: row["mean_sys"]),
        "highest_high_sys_rate_cluster_across_k": max(
            all_clusters, key=lambda row: row["high_sys_rate_top_decile"]
        ),
    }


def anomaly_scan(
    x_scaled: np.ndarray, sys_values: np.ndarray, observations: list[dict[str, Any]]
) -> dict[str, Any]:
    forest = IsolationForest(contamination=0.1, random_state=RANDOM_STATE)
    labels = forest.fit_predict(x_scaled)
    anomaly_mask = labels == -1
    normal_mask = ~anomaly_mask
    datasets = Counter(str(observations[i]["dataset"]) for i, flag in enumerate(anomaly_mask) if flag)
    regimes = Counter(regime_label(observations[i]) for i, flag in enumerate(anomaly_mask) if flag)
    return {
        "method": "IsolationForest(contamination=0.1)",
        "anomaly_count": int(np.sum(anomaly_mask)),
        "anomaly_mean_sys": float(np.mean(sys_values[anomaly_mask])),
        "normal_mean_sys": float(np.mean(sys_values[normal_mask])),
        "anomaly_max_sys": float(np.max(sys_values[anomaly_mask])),
        "normal_max_sys": float(np.max(sys_values[normal_mask])),
        "anomaly_dominant_dataset": datasets.most_common(1)[0][0],
        "anomaly_dominant_dataset_share": float(datasets.most_common(1)[0][1] / np.sum(anomaly_mask)),
        "anomaly_regime_counts": dict(sorted(regimes.items())),
    }


def decide_verdict(summary: dict[str, Any]) -> dict[str, str]:
    checks = summary["dataset_checks"]
    if not checks["passed"]:
        return {
            "verdict": "bug-redo",
            "evidence_strength": "low",
            "implementation_trust": "medium",
            "thesis_use": "omit before submission",
            "caveat": "Dataset guards failed; do not use the scan result.",
            "reopen_trigger": "Rerun after the dataset path matches the packet row counts, max sys, and sys > 1 count.",
        }
    best_cluster = summary["cluster"]["selected_by_silhouette"]["best_mean_sys_cluster"]
    anomaly = summary["anomaly"]
    best_pc = summary["pca"]["best_abs_corr_component"]
    if checks["sys_gt_one_count"] > 0:
        return {
            "verdict": "positive-escalate",
            "evidence_strength": "high",
            "implementation_trust": "high",
            "thesis_use": "Jorn decision needed",
            "caveat": "The dataset contains sys > 1; stop DS-I004 and inspect the producer result directly.",
            "reopen_trigger": "After Jorn/lead resolves the sys > 1 row provenance.",
        }
    if (
        best_pc["abs_corr_with_sys"] >= 0.85
        and best_cluster["high_sys_rate_top_decile"] >= 0.5
        and best_cluster["dominant_dataset_share"] <= 0.5
        and anomaly["anomaly_mean_sys"] > anomaly["normal_mean_sys"]
    ):
        return {
            "verdict": "conjectured-positive",
            "evidence_strength": "medium",
            "implementation_trust": "high",
            "thesis_use": "Jorn decision needed",
            "caveat": "This is only a generator hypothesis from intrinsic table features; it still needs a search/falsification follow-up.",
            "reopen_trigger": "Run a generator-side search rule derived without sys, endpoint labels, dataset identity, or optimizer provenance.",
        }
    return {
        "verdict": "negative",
        "evidence_strength": "medium",
        "implementation_trust": "high",
        "thesis_use": "supporting/caveat only",
        "caveat": "This is a 282-row retained-dataset scan over nonconstant intrinsic numeric polytope features; it excludes observation provenance and capacity/search witness columns, and it tests only PCA, KMeans k=2..8, and IsolationForest at 10 percent contamination.",
        "reopen_trigger": "Reopen if a larger or fresher table adds sys > 1, changes the row guards, or a sampling rule is proposed that can sample a feature-space region before inspecting sys, endpoint labels, dataset identity, or optimizer provenance.",
    }


def write_report(summary: dict[str, Any], path: Path) -> None:
    checks = summary["dataset_checks"]
    feature_info = summary["feature_info"]
    best_pc = summary["pca"]["best_abs_corr_component"]
    selected_cluster = summary["cluster"]["selected_by_silhouette"]
    best_cluster = selected_cluster["best_mean_sys_cluster"]
    highest_mean_cluster = summary["cluster"]["highest_mean_sys_cluster_across_k"]
    highest_rate_cluster = summary["cluster"]["highest_high_sys_rate_cluster_across_k"]
    anomaly = summary["anomaly"]
    verdict = summary["verdict"]

    lines = [
        "# DS-I004 PCA / Clustering / Anomaly Source-Truth Repair",
        "",
        "## Command / Provenance",
        "",
        f"- Script: `{summary['script']}`",
        f"- Dataset dir: `{summary['dataset_dir']}`",
        f"- Output dir: `{summary['out_dir']}`",
        f"- Producer command recorded in packet: `{summary['producer_command']}`",
        f"- Git commit: `{summary['git_commit']}`",
        f"- Random state: `{RANDOM_STATE}`",
        "",
        "## Dataset Snapshot And Guards",
        "",
        f"- Polytope rows: `{checks['polytope_rows']}` expected `{checks['expected_polytope_rows']}`.",
        f"- Observation rows: `{checks['observation_rows']}` expected `{checks['expected_observation_rows']}`.",
        f"- Max `sys`: `{checks['max_sys']:.15f}` expected `{checks['expected_max_sys']:.15f}`.",
        f"- `sys > 1` count: `{checks['sys_gt_one_count']}` expected `{checks['expected_sys_gt_one_count']}`.",
        f"- Guard status: `{'passed' if checks['passed'] else 'failed'}`.",
        "",
        "## Feature Scope",
        "",
        f"- Fitted feature count: `{feature_info['feature_count']}` nonconstant numeric columns.",
        f"- Candidate numeric polytope features before constant drop: `{feature_info['candidate_numeric_polytope_features']}`.",
        f"- Constant columns dropped: `{len(feature_info['dropped_constant_features'])}`.",
        "- Excluded classes:",
        *[f"  - {item}" for item in feature_info["excluded_column_classes"]],
        "- The fitted matrix uses no observation-table columns.",
        "",
        "## Observations",
        "",
        f"- PCA: component `{best_pc['component']}` has the largest absolute correlation with `sys`, `|corr| = {best_pc['abs_corr_with_sys']:.3f}`; PC1 explains `{summary['pca']['components'][0]['explained_variance_ratio']:.3f}` of standardized feature variance.",
        f"- PCA top absolute-score rows for that component have mean `sys = {best_pc['top_abs_score_mean_sys']:.3f}` and max `sys = {best_pc['top_abs_score_max_sys']:.3f}`.",
        f"- KMeans: silhouette selects `k = {selected_cluster['k']}` with silhouette `{selected_cluster['silhouette']:.3f}`.",
        f"- KMeans best mean-`sys` cluster has `{best_cluster['count']}` rows, mean `sys = {best_cluster['mean_sys']:.3f}`, max `sys = {best_cluster['max_sys']:.3f}`, top-decile high-`sys` rate `{best_cluster['high_sys_rate_top_decile']:.3f}`, dominant dataset `{best_cluster['dominant_dataset']}` at share `{best_cluster['dominant_dataset_share']:.3f}`, and regime counts `{best_cluster['regime_counts']}`.",
        f"- Across `k = 2..8`, the highest mean-`sys` cluster has `{highest_mean_cluster['count']}` rows, mean `sys = {highest_mean_cluster['mean_sys']:.3f}`, dominant dataset `{highest_mean_cluster['dominant_dataset']}` at share `{highest_mean_cluster['dominant_dataset_share']:.3f}`, and regime counts `{highest_mean_cluster['regime_counts']}`.",
        f"- Across `k = 2..8`, the highest top-decile high-`sys` rate is `{highest_rate_cluster['high_sys_rate_top_decile']:.3f}` in a `{highest_rate_cluster['count']}`-row cluster with dominant dataset `{highest_rate_cluster['dominant_dataset']}` at share `{highest_rate_cluster['dominant_dataset_share']:.3f}`.",
        f"- IsolationForest: `{anomaly['anomaly_count']}` anomalies have mean `sys = {anomaly['anomaly_mean_sys']:.3f}` versus normal mean `sys = {anomaly['normal_mean_sys']:.3f}`; anomaly max `sys = {anomaly['anomaly_max_sys']:.3f}`.",
        "",
        "## Inference",
        "",
        "The PCA/clustering/anomaly methods see structure in the retained dataset, but the fitted structure is not by itself a candidate-proposer. The strongest PCA diagnostic is a correlation with `sys`, which is an audit statistic rather than a sampling rule. The silhouette-selected cluster split is broad rather than a targeted high-`sys` rule; higher-k clusters with stronger high-`sys` concentration are endpoint/dataset-heavy when inspected after fitting. The anomaly rule does not enrich for high `sys` relative to the rest of the table. A positive follow-up would need to turn intrinsic feature loadings or cluster geometry into a sampling rule specified before inspecting `sys`, endpoint labels, dataset identity, or optimizer provenance.",
        "",
        "## Verdict",
        "",
        f"- `verdict`: `{verdict['verdict']}`",
        f"- `evidence_strength`: `{verdict['evidence_strength']}`",
        f"- `implementation_trust`: `{verdict['implementation_trust']}`",
        f"- `thesis_use`: `{verdict['thesis_use']}`",
        f"- `caveat`: {verdict['caveat']}",
        f"- `reopen_trigger`: {verdict['reopen_trigger']}",
        "",
        "## Reproducibility",
        "",
        "```bash",
        f"uv run --script {summary['script']} --dataset-dir {summary['dataset_dir']} --out-dir {summary['out_dir']}",
        "```",
    ]
    path.write_text("\n".join(lines) + "\n")


def main() -> None:
    args = parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    polytope_rows = load_jsonl(args.dataset_dir / "polytope-table.jsonl")
    observation_rows = load_jsonl(args.dataset_dir / "observation-table.jsonl")
    checks = validate_dataset(polytope_rows, observation_rows)
    if checks["sys_gt_one_count"] > 0:
        raise SystemExit("stop condition hit: sys > 1 found in dataset")
    if not checks["passed"]:
        raise SystemExit(f"dataset guard mismatch: {checks['errors']}")

    joined = build_joined_rows(polytope_rows, observation_rows)
    x, feature_names, feature_info = feature_matrix(joined)
    scaler = StandardScaler()
    x_scaled = scaler.fit_transform(x)
    sys_values = np.asarray([float(row["poly"]["sys"]) for row in joined], dtype=float)
    observations = [row["observation"] for row in joined]

    summary: dict[str, Any] = {
        "idea_id": "DS-I004",
        "script": str(Path(__file__).relative_to(Path.cwd())),
        "dataset_dir": str(args.dataset_dir),
        "out_dir": str(args.out_dir),
        "producer_command": "experiments/sys-landscape/datascience/build-dataset.sh",
        "git_commit": git_commit(),
        "dataset_checks": checks,
        "feature_info": feature_info,
        "pca": pca_scan(x_scaled, sys_values, feature_names),
        "cluster": cluster_scan(x_scaled, sys_values, observations),
        "anomaly": anomaly_scan(x_scaled, sys_values, observations),
    }
    summary["verdict"] = decide_verdict(summary)

    report_path = args.out_dir / "report.md"
    write_report(summary, report_path)
    print(f"Wrote {report_path}")
    print(json.dumps(summary["verdict"], sort_keys=True))


if __name__ == "__main__":
    main()
