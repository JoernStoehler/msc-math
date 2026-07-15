#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy==1.26.4"]
# ///
"""Target-free multi-view coverage-directed novelty packet.

This analyzer keeps a deliberately small, explicit policy surface.  It never
reads a target or a capacity field: candidates are planar factor shapes only.
The two views are (A) a frame-adjusted cyclic support geometry and (B) a
permutation-invariant sorted chord vector.  They are reported separately; the
``offline_greedy_max`` frontier uses max-normalized view distances only as an
acquisition rule, not as a scientific score.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import platform
import subprocess
import time
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Callable

import numpy as np

SCHEMA = "coverage-directed-novelty-v1"
VIEW_NAMES = ("frame_geometry", "chord_invariant")
BASELINE = "current-baseline[delta=0.2]"
MAX_BUDGET = 24


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_id(sample_id: str) -> str:
    return "witness-" + hashlib.sha256(sample_id.encode()).hexdigest()[:20]


@dataclass(frozen=True)
class Shape:
    witness_id: str
    sample_id: str
    population: str
    law: str
    parameter: str
    side_count: int
    seed: int
    row_index: int
    attempt: int
    vertices: np.ndarray


def load(path: Path) -> list[Shape]:
    result: list[Shape] = []
    seen: set[str] = set()
    for line in path.read_text().splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        if row.get("schema") != "factor-shape-row-v1" or row.get("factor_role") != "single":
            raise ValueError(f"unexpected factor row in {path}: {row.get('schema')}")
        sample_id = str(row["sample_id"])
        wid = stable_id(sample_id)
        if wid in seen:
            raise ValueError(f"duplicate witness id {wid}")
        seen.add(wid)
        vertices = np.asarray(row["vertices_ccw"], dtype=float)
        if vertices.shape != (int(row["side_count"]), 2):
            raise ValueError(f"bad vertex shape for {sample_id}")
        result.append(Shape(wid, sample_id, str(row["population"]), str(row["law"]),
                            str(row["parameter"]), int(row["side_count"]), int(row["seed"]),
                            int(row["row_index"]), int(row["attempt"]), vertices))
    return sorted(result, key=lambda x: x.witness_id)


def polygon_area(vertices: np.ndarray) -> float:
    return float(np.sum(vertices[:, 0] * np.roll(vertices[:, 1], -1) -
                        np.roll(vertices[:, 0], -1) * vertices[:, 1]) / 2.0)


def normalized(vertices: np.ndarray) -> np.ndarray:
    v = vertices - np.mean(vertices, axis=0)
    area = abs(polygon_area(v))
    if not np.isfinite(area) or area <= 1e-14:
        raise ValueError("degenerate polygon")
    return v / np.sqrt(area)


def frame_view(vertices: np.ndarray) -> np.ndarray:
    """Cyclic/frame-adjusted support geometry, quotienting rigid frame choices."""
    v = normalized(vertices)
    candidates = []
    for reverse in (False, True):
        order = v[::-1] if reverse else v
        for start in range(len(order)):
            seq = np.roll(order, -start, axis=0)
            edge = seq[1] - seq[0]
            angle = np.arctan2(edge[1], edge[0])
            rot = np.array([[np.cos(angle), np.sin(angle)], [-np.sin(angle), np.cos(angle)]])
            candidates.append(((seq - seq[0]) @ rot.T).reshape(-1))
    return min(candidates, key=lambda x: tuple(np.round(x, 14)))


def chord_view(vertices: np.ndarray) -> np.ndarray:
    """Lossy invariant: sorted all-pairs chord lengths, scaled by RMS chord."""
    v = normalized(vertices)
    distances = [float(np.linalg.norm(v[i] - v[j])) for i in range(len(v)) for j in range(i + 1, len(v))]
    values = np.asarray(sorted(distances))
    rms = float(np.sqrt(np.mean(values * values)))
    return values / rms if rms else values


def representations(shapes: list[Shape]) -> dict[str, np.ndarray]:
    return {
        "frame_geometry": np.stack([frame_view(s.vertices) for s in shapes]),
        "chord_invariant": np.stack([chord_view(s.vertices) for s in shapes]),
    }


def pairwise(a: np.ndarray, b: np.ndarray) -> np.ndarray:
    return np.sqrt(np.maximum(0.0, np.sum((a[:, None, :] - b[None, :, :]) ** 2, axis=2)))


def distance_matrix(shapes_a: list[Shape], shapes_b: list[Shape], view: str) -> np.ndarray:
    return pairwise(representations(shapes_a)[view], representations(shapes_b)[view])


def grouped(shapes: list[Shape]) -> dict[str, list[Shape]]:
    out: dict[str, list[Shape]] = defaultdict(list)
    for shape in shapes:
        out[shape.population].append(shape)
    return {key: sorted(value, key=lambda x: x.witness_id) for key, value in sorted(out.items())}


def write_tsv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fields = sorted({field for row in rows for field in row})
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows({field: "NA" if row.get(field) is None else row[field] for field in fields} for row in rows)


def radius(selected: list[int], candidate_indices: list[int], distances: np.ndarray) -> float:
    if not candidate_indices:
        return 0.0
    if not selected:
        return float("inf")
    return float(np.max(np.min(distances[np.ix_(candidate_indices, selected)], axis=1)))


def radius_cross(selected_train: list[int], holdout_indices: list[int], distances: np.ndarray) -> float:
    """Cover holdout rows by selected train rows (rows=holdout, cols=train)."""
    if not holdout_indices:
        return 0.0
    if not selected_train:
        return float("inf")
    return float(np.max(np.min(distances[np.ix_(holdout_indices, selected_train)], axis=1)))


def cover_values(selected: list[int], evaluated: list[int], distances: np.ndarray) -> np.ndarray:
    """Nearest-cover distances for an evaluated set (max is only one display)."""
    if not evaluated:
        return np.zeros(0)
    if not selected:
        return np.full(len(evaluated), np.inf)
    return np.min(distances[np.ix_(evaluated, selected)], axis=1)


def cover_stats(selected: list[int], evaluated: list[int], distances: np.ndarray) -> dict[str, float]:
    values = cover_values(selected, evaluated, distances)
    if not len(values):
        return {"max": 0.0, "mean": 0.0, "q90": 0.0}
    return {"max": float(np.max(values)), "mean": float(np.mean(values)), "q90": float(np.quantile(values, 0.9))}


def arm_order(shapes: list[Shape]) -> list[str]:
    return sorted({s.population for s in shapes})


def baseline_refs(shapes: list[Shape], n: int = 4) -> list[Shape]:
    refs = [s for s in shapes if s.population == BASELINE]
    if len(refs) < n:
        raise ValueError(f"baseline has only {len(refs)} rows")
    return sorted(refs, key=lambda x: x.witness_id)[:n]


def frontier(train: list[Shape], holdout: list[Shape], mode: str, budget: int = MAX_BUDGET) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    if not train or not holdout or {s.side_count for s in train + holdout} != {train[0].side_count}:
        raise ValueError("frontier requires one nonempty side-count stratum")
    side_count = train[0].side_count
    refs = baseline_refs(train)
    ref_ids = {s.witness_id for s in refs}
    candidates = [s for s in train if s.witness_id not in ref_ids]
    ordered = sorted(candidates, key=lambda x: x.witness_id)
    all_eval = sorted(holdout, key=lambda x: x.witness_id)
    all_train = sorted(train, key=lambda x: x.witness_id)
    rep_train = representations(all_train)
    rep_hold = representations(all_eval)
    ref_idx = [all_train.index(s) for s in refs]
    candidate_indices = [all_train.index(s) for s in ordered]
    # Holdout rows are covered by the *training* frozen references and selected
    # witnesses.  Exclude the holdout baseline arm from the evaluated union so
    # it cannot receive free coverage from itself.
    hold_candidates = [i for i, s in enumerate(all_eval) if s.population != BASELINE]

    matrices_train = {v: pairwise(rep_train[v], rep_train[v]) for v in VIEW_NAMES}
    matrices_cross = {v: pairwise(rep_hold[v], rep_train[v]) for v in VIEW_NAMES}
    medians = {v: float(np.median(matrices_train[v][np.triu_indices(len(all_train), 1)])) or 1.0 for v in VIEW_NAMES}

    selected: list[int] = []
    rows: list[dict[str, Any]] = []
    disagreements: list[dict[str, Any]] = []
    # Use deterministic hashed passive allocation; offline policies are greedy
    # over the already generated candidate pool, not online sample allocation.
    if mode == "passive_coreset":
        selected = sorted(range(len(ordered)), key=lambda i: ordered[i].witness_id)[:budget]
    for step in range(budget):
        if mode == "passive_coreset":
            current = selected[: step + 1]
        else:
            remaining = [i for i in range(len(ordered)) if i not in selected]
            if not remaining:
                break
            scores: dict[str, dict[int, float]] = {v: {} for v in VIEW_NAMES}
            for v in VIEW_NAMES:
                selected_global = ref_idx + [candidate_indices[i] for i in selected]
                for i in remaining:
                    gi = candidate_indices[i]
                    before = radius(selected_global, candidate_indices, matrices_train[v])
                    after = radius(selected_global + [gi], candidate_indices, matrices_train[v])
                    scores[v][i] = (before - after) / medians[v]
            best_by_view = {v: max(remaining, key=lambda i: (scores[v][i], ordered[i].witness_id)) for v in VIEW_NAMES}
            if mode == "offline_greedy_frame":
                chosen = best_by_view["frame_geometry"]
            elif mode == "offline_greedy_chord":
                chosen = best_by_view["chord_invariant"]
            else:
                chosen = max(remaining, key=lambda i: (max(scores[v][i] for v in VIEW_NAMES), ordered[i].witness_id))
            disagreements.append({"side_count": side_count, "step": step + 1, "frame_witness_id": ordered[best_by_view["frame_geometry"]].witness_id, "chord_witness_id": ordered[best_by_view["chord_invariant"]].witness_id, "disagree": best_by_view["frame_geometry"] != best_by_view["chord_invariant"], "chosen_witness_id": ordered[chosen].witness_id, "mode": mode})
            selected.append(chosen)
            current = selected
        selected_global = ref_idx + [candidate_indices[i] for i in current]
        selected_train_global = ref_idx + [candidate_indices[i] for i in current]
        chosen = ordered[current[-1]]
        for view in VIEW_NAMES:
            train_before_stats = cover_stats(ref_idx + [candidate_indices[i] for i in current[:-1]], candidate_indices, matrices_train[view])
            train_after_stats = cover_stats(selected_global, candidate_indices, matrices_train[view])
            hold_before_stats = cover_stats(ref_idx + [candidate_indices[i] for i in current[:-1]], hold_candidates, matrices_cross[view])
            hold_after_stats = cover_stats(selected_train_global, hold_candidates, matrices_cross[view])
            row = {"side_count": side_count, "policy": mode, "step": step + 1, "budget": step + 1, "view": view, "witness_id": chosen.witness_id, "population": chosen.population, "seed": chosen.seed, "attempt": chosen.attempt}
            for metric in ("max", "mean", "q90"):
                row[f"train_cover_{metric}_before"] = train_before_stats[metric]
                row[f"train_cover_{metric}_after"] = train_after_stats[metric]
                row[f"train_cover_{metric}_incremental_reduction"] = train_before_stats[metric] - train_after_stats[metric]
                row[f"holdout_cover_{metric}_before"] = hold_before_stats[metric]
                row[f"holdout_cover_{metric}_after"] = hold_after_stats[metric]
                row[f"holdout_cover_{metric}_incremental_reduction"] = hold_before_stats[metric] - hold_after_stats[metric]
            rows.append(row)
    return rows, disagreements, [{"side_count": side_count, "policy": mode, "budget": len(selected), "selected_witnesses": len(selected), "selected_populations": json.dumps(Counter(ordered[i].population for i in selected), sort_keys=True), "holdout_candidate_count": len(hold_candidates)}]


def synthetic_calibration(out_dir: Path) -> dict[str, Any]:
    angles = np.linspace(0, 2 * np.pi, 6, endpoint=False)
    radii = np.array([1.0, 1.2, 0.8, 1.1, 0.9, 1.3])
    base = np.stack([radii * np.cos(angles), radii * np.sin(angles)], axis=1)
    cases: list[dict[str, Any]] = []
    # A broad law that occupies the same support region.
    for _ in range(8):
        cases.append({"case": "broad_no_new_support", "label": "broad", "vertices": base.tolist()})
    # Rare remote mode, and a known one-outlier contamination control.
    remote = base.copy(); remote[[0, 3]] *= 2.0
    cases.append({"case": "tiny_rare_remote_mode", "label": "rare_remote", "vertices": remote.tolist()})
    contamination = base.copy(); contamination[[0, 3]] *= 2.3
    cases.append({"case": "one_outlier_contamination", "label": "contamination_outlier", "vertices": contamination.tolist()})
    # Duplicated/dependent rows.
    for _ in range(4):
        cases.append({"case": "duplicated_dependent_rows", "label": "duplicate", "vertices": base.tolist()})
    # Raw representation disagrees under rotation; quotient should agree.
    rotated = base @ np.array([[0.0, -1.0], [1.0, 0.0]])
    cases.extend([{"case": "representation_disagreement", "label": "raw", "vertices": base.tolist()}, {"case": "representation_disagreement", "label": "rotated", "vertices": rotated.tolist()}])
    shapes = [normalized(np.asarray(c["vertices"])) for c in cases]
    quotient = np.stack([frame_view(v) for v in shapes])
    raw = np.stack([v.reshape(-1) for v in shapes])
    broad_indices = [i for i, c in enumerate(cases) if c["label"] == "broad"]
    novelty = np.array([min(float(np.linalg.norm(quotient[i] - quotient[j])) for j in broad_indices) for i in range(len(cases))])
    selected_indices = set(np.argsort(-novelty, kind="stable")[:2].tolist())
    rows = []
    for i, c in enumerate(cases):
        selected = i in selected_indices
        rows.append({"case": c["case"], "label": c["label"], "raw_distance_to_first": float(np.linalg.norm(raw[i] - raw[0])), "quotient_distance_to_first": float(np.linalg.norm(quotient[i] - quotient[0])), "selected_by_frontier": selected, "disposition": "remote-mode-discovered" if c["label"] == "rare_remote" and selected else ("known-contamination-selected; not a population region" if c["label"] == "contamination_outlier" and selected else ("known-contamination-not-selected" if c["label"] == "contamination_outlier" else "control"))})
    write_tsv(out_dir / "synthetic-calibrations.tsv", rows)
    remote_row = next(row for row in rows if row["label"] == "rare_remote")
    contamination_row = next(row for row in rows if row["label"] == "contamination_outlier")
    return {"schema": "coverage-directed-novelty-synthetic-v1", "cases": sorted(Counter(c["case"] for c in cases).items()), "remote_mode_discovered": remote_row["selected_by_frontier"], "contamination_not_population_region": contamination_row["disposition"].startswith("known-contamination"), "representation_raw_distance": float(np.linalg.norm(raw[-1] - raw[-2])), "representation_quotient_distance": float(np.linalg.norm(quotient[-1] - quotient[-2])), "limitation": "Geometry alone cannot distinguish a tiny remote mode from a contaminated outlier; this calibration selected both and retains the known contamination label rather than promoting it."}


def validate_pool_contract(train: list[Shape], holdout: list[Shape], expected_side_count: int | None = None) -> None:
    """Fail closed on independence or unbalanced explicit-arm strata."""
    train_ids, hold_ids = {s.sample_id for s in train}, {s.sample_id for s in holdout}
    if train_ids & hold_ids:
        raise ValueError("train and holdout sample IDs overlap")
    train_seeds, hold_seeds = {s.seed for s in train}, {s.seed for s in holdout}
    if train_seeds & hold_seeds:
        raise ValueError("train and holdout master seeds overlap")
    for label, pool in (("train", train), ("holdout", holdout)):
        counts = Counter((s.population, s.side_count) for s in pool)
        if not counts or len({count for count in counts.values()}) != 1:
            raise ValueError(f"{label} population/side-count strata are unbalanced")
        side_counts = {s.side_count for s in pool}
        if len(side_counts) != 1 or (expected_side_count is not None and side_counts != {expected_side_count}):
            raise ValueError(f"{label} is not one fixed requested side-count stratum")
    train_counts = Counter(s.population for s in train)
    hold_counts = Counter(s.population for s in holdout)
    if train_counts != hold_counts:
        raise ValueError("train and holdout population arms differ or are unbalanced")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--train", type=Path, action="append", required=True, help="one complete train panel per side count")
    parser.add_argument("--holdout", type=Path, action="append", required=True, help="one complete holdout panel per side count")
    parser.add_argument("--producer-report", type=Path, action="append", required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    if len(args.train) != len(args.holdout):
        raise SystemExit("provide one holdout panel for every train panel")
    train_pools, holdout_pools = [load(path) for path in args.train], [load(path) for path in args.holdout]
    panels: list[tuple[int, list[Shape], list[Shape]]] = []
    for train, holdout in zip(train_pools, holdout_pools):
        side_counts = {s.side_count for s in train + holdout}
        if len(side_counts) != 1:
            raise SystemExit("each input panel must contain exactly one side count")
        side_count = next(iter(side_counts))
        validate_pool_contract(train, holdout, side_count)
        if {s.population for s in train} != {s.population for s in holdout}:
            raise SystemExit(f"train/holdout population sets differ at side count {side_count}")
        panels.append((side_count, train, holdout))
    if len({side for side, _, _ in panels}) != len(panels):
        raise SystemExit("duplicate side-count panel")
    panels.sort(key=lambda item: item[0])
    start = time.perf_counter()
    policy_rows, disagreement, policy_summary = [], [], []
    for side_count, train, holdout in panels:
        for mode in ("passive_coreset", "offline_greedy_max", "offline_greedy_frame", "offline_greedy_chord"):
            rows, disagreements, summary = frontier(train, holdout, mode)
            policy_rows.extend(rows); disagreement.extend(disagreements); policy_summary.extend(summary)
    selection_ms = (time.perf_counter() - start) * 1000.0
    write_tsv(args.out_dir / "coreset-yield.tsv", policy_rows)
    write_tsv(args.out_dir / "view-disagreement.tsv", disagreement)
    write_tsv(args.out_dir / "policy-summary.tsv", policy_summary)
    arms = []
    for side_count, train, holdout in panels:
        train_seed = sorted({s.seed for s in train})
        holdout_seed = sorted({s.seed for s in holdout})
        for population in arm_order(train):
            rows = [s for s in train if s.population == population]
            panel_rows = [row for row in policy_rows if row["side_count"] == side_count]
            view_by_metric = {metric: {v: any(row["population"] == population and row["view"] == v and row[f"holdout_cover_{metric}_incremental_reduction"] > 1e-10 for row in panel_rows if row["policy"] == "offline_greedy_max") for v in VIEW_NAMES} for metric in ("mean", "q90")}
            arms.append({"side_count": side_count, "population": population, "train_rows": len(rows), "holdout_rows": len([s for s in holdout if s.population == population]), "train_seed": train_seed, "holdout_seed": holdout_seed, "attempt_max_train": max(s.attempt for s in rows), "offline_greedy_max_fixed_train_panel_selected_witnesses": {"policy": "offline_greedy_max", "selection_order": "deterministic greedy order over the complete fixed train panel", "retained_budget": MAX_BUDGET, "count_for_this_arm": sum(1 for row in panel_rows if row["policy"] == "offline_greedy_max" and row["population"] == population and row["view"] == "frame_geometry"), "not_intrinsic_arm_property": True}, "offline_greedy_max_fixed_train_panel_holdout_mean_nonredundant_views": {"policy": "offline_greedy_max", "train_panel": f"side_count={side_count}, seed={train_seed}", "holdout_panel": f"side_count={side_count}, seed={holdout_seed}", "retained_budget": MAX_BUDGET, "metric": "holdout mean nearest-cover distance", "view_dependent": view_by_metric["mean"], "not_intrinsic_arm_property": True}, "offline_greedy_max_fixed_train_panel_holdout_bulk_nonredundant_views": {"policy": "offline_greedy_max", "train_panel": f"side_count={side_count}, seed={train_seed}", "holdout_panel": f"side_count={side_count}, seed={holdout_seed}", "retained_budget": MAX_BUDGET, "metrics": ("mean", "q90"), "view_dependent_by_metric": view_by_metric, "not_intrinsic_arm_property": True}})
    generation_rows = []
    train_seed_set = {s.seed for pool in train_pools for s in pool}
    holdout_seed_set = {s.seed for pool in holdout_pools for s in pool}
    if train_seed_set & holdout_seed_set:
        raise SystemExit("producer report seed roles overlap")
    for path in args.producer_report:
        report = json.loads(path.read_text())
        report_seed = int(report["seed"])
        pool_label = "train" if report_seed in train_seed_set else ("holdout" if report_seed in holdout_seed_set else None)
        if pool_label is None:
            raise SystemExit(f"producer report seed is not assigned to a panel: {report_seed}")
        report_side_count = int(report["per_population"][0]["side_count"])
        pool_requested = pool_accepted = pool_exhausted = 0
        pool_generation_ms = 0.0
        for item in report["per_population"]:
            pool_requested += int(item["requested"]); pool_accepted += int(item["accepted"]); pool_exhausted += int(item["exhausted"]); pool_generation_ms += float(item["total_generation_ms"])
            generation_rows.append({"pool": pool_label, "population": f"{item['law']}[{item['parameter']}]", "side_count": item["side_count"], "requested": item["requested"], "accepted": item["accepted"], "exhausted": item["exhausted"], "generation_ms": item["total_generation_ms"], "generation_ms_per_accepted": (item["total_generation_ms"] / item["accepted"] if item["accepted"] else None), "source_revision": report.get("source_revision"), "source_dirty": report.get("source_dirty")})
        generation_rows.append({"pool": pool_label, "population": "__full_pool__", "side_count": report_side_count, "requested": pool_requested, "accepted": pool_accepted, "exhausted": pool_exhausted, "generation_ms": pool_generation_ms, "generation_ms_per_accepted": (pool_generation_ms / pool_accepted if pool_accepted else None), "source_revision": report.get("source_revision"), "source_dirty": report.get("source_dirty")})
    write_tsv(args.out_dir / "generation-cost.tsv", generation_rows)
    calibration = synthetic_calibration(args.out_dir)
    policy_results = []
    for side_count, _, _ in panels:
      for policy in ("passive_coreset", "offline_greedy_max", "offline_greedy_frame", "offline_greedy_chord"):
        for view in VIEW_NAMES:
            values = [row for row in policy_rows if row["side_count"] == side_count and row["policy"] == policy and row["view"] == view]
            first, last = values[0], values[-1]
            for metric in ("max", "mean", "q90"):
                initial = float(first[f"holdout_cover_{metric}_before"])
                final = float(last[f"holdout_cover_{metric}_after"])
                policy_results.append({"side_count": side_count, "policy": policy, "view": view, "metric": metric, "retained_budget": int(last["budget"]), "holdout_initial_cover": initial, "holdout_final_cover": final, "holdout_total_reduction": initial - final, "holdout_reduction_per_retained_witness": (initial - final) / int(last["budget"])})
    findings = []
    for side_count, _, _ in panels:
        panel_arms = [arm for arm in arms if arm["side_count"] == side_count]
        for view in VIEW_NAMES:
            for metric in ("mean", "q90"):
                lookup = lambda policy: next(row for row in policy_results if row["side_count"] == side_count and row["policy"] == policy and row["view"] == view and row["metric"] == metric)
                off, passive = lookup("offline_greedy_max"), lookup("passive_coreset")
                positive = sum(bool(arm["offline_greedy_max_fixed_train_panel_holdout_bulk_nonredundant_views"]["view_dependent_by_metric"][metric][view]) for arm in panel_arms)
                findings.append({"side_count": side_count, "view": view, "metric": metric, "offline_greedy_max_total_reduction": off["holdout_total_reduction"], "passive_coreset_total_reduction": passive["holdout_total_reduction"], "offline_greedy_max_reduction_per_retained_witness": off["holdout_reduction_per_retained_witness"], "positive_arm_contribution_count": positive, "interpretation": "stratum-local bulk diagnostic; do not compare side counts or select a best law/view"})
    inputs = {str(path): sha256(path) for path in [*args.train, *args.holdout, *args.producer_report]}
    source_files = [Path(__file__)]
    repo = Path(__file__).resolve().parents[4]
    revision = subprocess.run(["git", "rev-parse", "HEAD"], cwd=repo, capture_output=True, text=True, check=True).stdout.strip()
    dirty = subprocess.run(["git", "status", "--porcelain", "--", str(Path(__file__).relative_to(repo))], cwd=repo, capture_output=True, text=True, check=True).stdout.strip()
    producer_revision = json.loads(args.producer_report[0].read_text()).get("source_revision", "unknown")
    producer_source_blobs = {}
    if producer_revision != "unknown":
        for source_path in ("experiments/sys-datascience/methods/generator-zoo-smoke/main.rs", "experiments/sys-landscape/Cargo.toml", "Cargo.lock"):
            resolved = subprocess.run(["git", "rev-parse", f"{producer_revision}:{source_path}"], cwd=repo, capture_output=True, text=True)
            producer_source_blobs[source_path] = resolved.stdout.strip() if resolved.returncode == 0 else "unresolved"
    python_version, numpy_version = platform.python_version(), np.__version__
    selection_observation = {"schema": "coverage-directed-novelty-selection-observation-v1", "selection_cost_ms": selection_ms, "python_version": python_version, "numpy_version": numpy_version, "nondeterministic": True, "excluded_from_byte_replay": True, "note": "Wall-clock timing is an observation only; it is not part of deterministic evidence."}
    (args.out_dir / "selection-cost-observation.json").write_text(json.dumps(selection_observation, indent=2, sort_keys=True) + "\n")
    panel_metadata = [{"side_count": side_count, "train_rows": len(train), "holdout_rows": len(holdout), "train_seeds": sorted({s.seed for s in train}), "holdout_seeds": sorted({s.seed for s in holdout})} for side_count, train, holdout in panels]
    report = {
        "schema": SCHEMA,
        "question": "Do bulk holdout mean/q90 cover reductions and multi-law retained-witness contributions persist separately at side counts 4, 6, and 8?",
        "panels": panel_metadata,
        "side_counts": [item["side_count"] for item in panel_metadata],
        "environment": {"python_version": python_version, "numpy_version": numpy_version, "numpy_requirement": "numpy==1.26.4", "deterministic_replay_contract": "run with uv and the PEP-723 numpy pin; selection-cost-observation.json is excluded because it contains wall-clock timing"},
        "arms": arms,
        "policies": ["passive_coreset", "offline_greedy_max", "offline_greedy_frame", "offline_greedy_chord"],
        "retained_budget_per_side_count": MAX_BUDGET,
        "policy_results": policy_results,
        "stratum_findings": findings,
        "selection_cost_artifact": "selection-cost-observation.json",
        "generation_cost_artifact": "generation-cost.tsv",
        "view_contract": {"frame_geometry": "cyclic/frame-adjusted vertices after translation and area normalization; minimum over start and reversal", "chord_invariant": "sorted all-pairs chord lengths after translation and area normalization", "offline_greedy_max": "max of per-view median-normalized gains for offline coreset acquisition only; no scientific scalar ranking", "passive_coreset": "deterministic SHA-256 rank order is the reproducible passive retained-budget comparator"},
        "frozen_baseline": {"population": BASELINE, "n": 4, "selection": "lowest deterministic witness IDs separately within each side-count train panel"},
        "pool_contract": {"independent_master_seeds": True, "disjoint_sample_ids": True, "balanced_population_side_count_strata": True, "abandoned_or_capped_arms": [], "failure_mode": "analyze.py fails closed on overlap, imbalance, side-count drift, or duplicate side panels"},
        "synthetic_calibrations": calibration,
        "input_hashes": inputs,
        "source": {"revision": revision, "analyzer_sha256": sha256(Path(__file__)), "tracked_clean_for_analyzer": not bool(dirty), "producer_revision": producer_revision, "producer_source_blobs": producer_source_blobs},
        "provenance": {"producer_reports": [str(p) for p in args.producer_report], "seed_contract": "each side-count train/holdout pair uses independent master seeds; seed and attempt are retained in every witness row", "witness_id": "sha256(sample_id) prefix", "selection_contract": "all full train rows are generated before offline retained-witness selection separately within each side-count panel"},
        "interpretation": {"allowed": ["stratum-local holdout nearest-cover max/mean/q90 by named panel, population, and view", "which arms contribute retained witnesses under each explicitly qualified panel/policy/metric/view field", "measured full-pool generation and offline selection costs", "metric-disagreement cases and synthetic calibration behavior"], "prohibited": ["pooling side counts or selecting a best side count, law, or view", "online allocation, sample-efficiency, or per-generated-row claims", "reading arm contribution fields as intrinsic properties of a law or population", "sys or target exposure", "population support or density claims", "law quality ranking", "target transfer, inferential, causal, or post-selection claims"], "limitations": ["geometry alone cannot distinguish a rare remote mode from a contamination outlier", "three finite side-count panels and independent seed pairs are bounded confirmation evidence only", "offline greedy views are retained-coreset heuristics, not online policies"], "smallest_next_geometry_only_follow_up": "If a later decision needs more evidence, repeat one named side-count panel with another independent seed pair; do not pool these strata or choose a winning law/view.", "selected_witness_authorization": "geometry-only follow-up only"}
    }
    (args.out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"schema": SCHEMA, "train_rows": len(train), "holdout_rows": len(holdout), "selection_cost_artifact": str(args.out_dir / "selection-cost-observation.json")}))


if __name__ == "__main__":
    main()
