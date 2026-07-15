#!/usr/bin/env python3
"""Audit retained generator rows for sampling-law and unit-lineage fidelity.

The analyzer is deliberately target-free.  It reports a failed or unavailable
diagnostic rather than treating a small non-rejection as evidence that a law is
correct.  Only the standard library is used so the packet can run in the base
devcontainer without an undeclared statistical stack.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import random
import statistics
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Iterable


TAU = math.tau
ROOT = Path(__file__).resolve().parents[4]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def git(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args], cwd=ROOT, text=True, capture_output=True, check=False
    )
    return completed.stdout.strip() if completed.returncode == 0 else "unknown"


def source_provenance() -> dict[str, Any]:
    status = git("status", "--porcelain=v1", "--untracked-files=no")
    return {
        "repository_revision": git("rev-parse", "HEAD"),
        "repository_tree": git("rev-parse", "HEAD^{tree}"),
        "tracked_clean": status == "",
        "tracked_status": status,
        "analyzer_path": str(Path(__file__).relative_to(ROOT)),
        "analyzer_sha256": sha256(Path(__file__)),
    }


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    text = path.read_text(encoding="utf-8")
    if text.startswith("version https://git-lfs.github.com/spec/v1"):
        raise ValueError(f"{path} is an LFS pointer; run git lfs pull for this input")
    return [json.loads(line) for line in text.splitlines() if line.strip()]


def write_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + "\n", encoding="utf-8")


def parse_bucket(bucket: str) -> tuple[int, int]:
    left, right = bucket.split("x", 1)
    return int(left), int(right)


def normals_from_vertices(vertices: list[list[float]]) -> list[float]:
    """Return sorted outward-normal arguments of a CCW polygon cycle."""
    angles = []
    for index, point in enumerate(vertices):
        nxt = vertices[(index + 1) % len(vertices)]
        dx, dy = nxt[0] - point[0], nxt[1] - point[1]
        length = math.hypot(dx, dy)
        if not math.isfinite(length) or length <= 1e-12:
            raise ValueError("degenerate polygon edge")
        angles.append(math.atan2(-dx / length, dy / length) % TAU)
    return sorted(angles)


def cyclic_gaps(angles: list[float]) -> list[float]:
    if len(angles) < 3:
        raise ValueError("need at least three angles")
    return [
        angles[(index + 1) % len(angles)] - angle + (TAU if index + 1 == len(angles) else 0.0)
        for index, angle in enumerate(angles)
    ]


def circular_diagnostics(values: Iterable[float]) -> dict[str, float]:
    xs = sorted((value / TAU) % 1.0 for value in values)
    if not xs:
        return {"n": 0, "mean_resultant": math.nan, "kuiper": math.nan}
    n = len(xs)
    mean_cos = sum(math.cos(TAU * x) for x in xs) / n
    mean_sin = sum(math.sin(TAU * x) for x in xs) / n
    d_plus = max((index + 1) / n - x for index, x in enumerate(xs))
    d_minus = max(x - index / n for index, x in enumerate(xs))
    return {
        "n": n,
        "mean_resultant": math.hypot(mean_cos, mean_sin),
        "kuiper": d_plus + d_minus,
    }


def rank(values: list[float]) -> list[float]:
    ordered = sorted(enumerate(values), key=lambda item: item[1])
    result = [0.0] * len(values)
    position = 0
    while position < len(ordered):
        end = position + 1
        while end < len(ordered) and ordered[end][1] == ordered[position][1]:
            end += 1
        average = (position + end - 1) / 2.0
        for original, _ in ordered[position:end]:
            result[original] = average
        position = end
    return result


def pearson(x: list[float], y: list[float]) -> float:
    if len(x) != len(y) or len(x) < 2:
        return math.nan
    mx, my = statistics.fmean(x), statistics.fmean(y)
    numerator = sum((a - mx) * (b - my) for a, b in zip(x, y))
    x_norm = math.sqrt(sum((a - mx) ** 2 for a in x))
    y_norm = math.sqrt(sum((b - my) ** 2 for b in y))
    return numerator / (x_norm * y_norm) if x_norm and y_norm else math.nan


def spearman(x: list[float], y: list[float]) -> float:
    return pearson(rank(x), rank(y))


def beta_continued_fraction(a: float, b: float, x: float) -> float:
    """Lentz evaluation used by the regularized incomplete beta CDF."""
    tiny, eps = 1e-300, 3e-14
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c = 1.0
    d = 1.0 - qab * x / qap
    d = tiny if abs(d) < tiny else d
    d = 1.0 / d
    h = d
    for step in range(1, 201):
        twice = 2 * step
        aa = step * (b - step) * x / ((qam + twice) * (a + twice))
        d = 1.0 + aa * d
        d = tiny if abs(d) < tiny else d
        c = 1.0 + aa / c
        c = tiny if abs(c) < tiny else c
        d = 1.0 / d
        h *= d * c
        aa = -(a + step) * (qab + step) * x / ((a + twice) * (qap + twice))
        d = 1.0 + aa * d
        d = tiny if abs(d) < tiny else d
        c = 1.0 + aa / c
        c = tiny if abs(c) < tiny else c
        d = 1.0 / d
        delta = d * c
        h *= delta
        if abs(delta - 1.0) < eps:
            return h
    raise ValueError("beta continued fraction did not converge")


def regularized_beta(x: float, a: float, b: float) -> float:
    if x <= 0.0:
        return 0.0
    if x >= 1.0:
        return 1.0
    log_front = a * math.log(x) + b * math.log1p(-x) - math.lgamma(a) - math.lgamma(b)
    front = math.exp(log_front)
    if x < (a + 1.0) / (a + b + 2.0):
        return front * beta_continued_fraction(a, b, x) / a
    return 1.0 - front * beta_continued_fraction(b, a, 1.0 - x) / b


def ks_uniform(values: list[float]) -> float:
    if not values:
        return math.nan
    xs = sorted(values)
    n = len(xs)
    return max(
        max((index + 1) / n - value for index, value in enumerate(xs)),
        max(value - index / n for index, value in enumerate(xs)),
    )


def dirichlet_diagnostics(gap_rows: list[list[float]], alpha: float, side_count: int) -> dict[str, Any]:
    """Check each polygon simplex separately, then pool exchangeable marginals."""
    proportions_by_row = [[gap / TAU for gap in gaps] for gaps in gap_rows]
    proportions = [value for row in proportions_by_row for value in row]
    positive = all(math.isfinite(value) and value > 0.0 for value in proportions)
    sum_error = max((abs(sum(row) - 1.0) for row in proportions_by_row), default=math.nan)
    if not positive or alpha <= 0.0:
        return {"positivity": positive, "max_simplex_sum_error": sum_error, "pit_ks": math.nan}
    beta_b = (side_count - 1) * alpha
    pit = [regularized_beta(value, alpha, beta_b) for value in proportions]
    expected_variance = (side_count - 1) / (side_count**2 * (side_count * alpha + 1.0))
    return {
        "positivity": positive,
        "max_simplex_sum_error": sum_error,
        "marginal_mean": statistics.fmean(proportions),
        "marginal_variance": statistics.fmean(
            (value - 1.0 / side_count) ** 2 for value in proportions
        ),
        "expected_marginal_variance": expected_variance,
        "pit_ks": ks_uniform(pit),
        "pit_mean": statistics.fmean(pit),
        "n_gaps": len(proportions),
    }


def stable_row(row: dict[str, Any]) -> dict[str, Any]:
    """Fields expected to match in an exact replay; wall-clock observations excluded."""
    return {
        key: value
        for key, value in row.items()
        if key not in {"generation_ms", "validation_ms", "target_ms"}
    }


def check_replay(
    left: list[dict[str, Any]], right: list[dict[str, Any]], left_path: Path, right_path: Path
) -> dict[str, Any]:
    """Compare two separately retained producer executions, never one file to itself."""
    if left_path.resolve() == right_path.resolve():
        raise ValueError("deterministic replay requires two distinct resolved row paths")
    if sha256(left_path) == sha256(right_path):
        raise ValueError("deterministic replay inputs have identical byte identity; retain two distinct executions")
    left_rows = [stable_row(row) for row in left]
    right_rows = [stable_row(row) for row in right]
    return {
        "name": "deterministic_replay",
        "status": "pass" if left_rows == right_rows else "fail",
        "left_rows": len(left_rows),
        "right_rows": len(right_rows),
        "comparison": "exact JSON row order after removing wall-clock fields",
    }


def input_identity(path: Path, row_count: int | None = None) -> dict[str, Any]:
    return {
        "path": str(path.relative_to(ROOT)) if path.is_relative_to(ROOT) else str(path),
        "resolved_path": str(path.resolve()),
        "sha256": sha256(path),
        "bytes": path.stat().st_size,
        **({"row_count": row_count} if row_count is not None else {}),
    }


def replay_evidence(args: argparse.Namespace) -> dict[str, Any] | None:
    if args.replay_left is None:
        return None
    left_path, right_path = args.replay_left, args.replay_right
    left_report_path, right_report_path = args.replay_left_report, args.replay_right_report
    assert right_path is not None and left_report_path is not None and right_report_path is not None
    if left_report_path.resolve() == right_report_path.resolve():
        raise ValueError("deterministic replay requires two distinct resolved producer report paths")
    left_rows, right_rows = read_jsonl(left_path), read_jsonl(right_path)
    left_report = json.loads(left_report_path.read_text())
    right_report = json.loads(right_report_path.read_text())
    if left_report.get("command") != args.replay_left_command or right_report.get("command") != args.replay_right_command:
        raise ValueError("replay producer command must exactly match the command retained in its producer report")
    if args.replay_left_command == args.replay_right_command:
        raise ValueError("deterministic replay requires distinct producer command identities")
    result = check_replay(left_rows, right_rows, left_path, right_path)
    result["runs"] = {
        "left": {
            "rows": input_identity(left_path, len(left_rows)),
            "producer_report": input_identity(left_report_path),
            "producer_command": args.replay_left_command,
            "producer_provenance": {
                key: left_report.get(key)
                for key in ("source_revision", "source_tree", "source_dirty", "seed", "max_attempts_per_row", "all_requested_rows_terminal", "status_counts")
            },
        },
        "right": {
            "rows": input_identity(right_path, len(right_rows)),
            "producer_report": input_identity(right_report_path),
            "producer_command": args.replay_right_command,
            "producer_provenance": {
                key: right_report.get(key)
                for key in ("source_revision", "source_tree", "source_dirty", "seed", "max_attempts_per_row", "all_requested_rows_terminal", "status_counts")
            },
        },
    }
    result["distinct_run_contract"] = {
        "resolved_row_paths_distinct": True,
        "resolved_producer_report_paths_distinct": True,
        "row_byte_identities_distinct": True,
        "recorded_commands_distinct": True,
        "note": "The two runs deliberately share producer source revision and seed; distinct artifacts, report paths, and recorded out-dir commands establish separate sequential executions.",
    }
    return result


def factor_fingerprint(row: dict[str, Any]) -> str:
    # Exact duplicate geometry is the high-signal accidental duplication case.
    payload = json.dumps(row.get("vertices_ccw"), separators=(",", ","), sort_keys=True)
    return hashlib.sha256(payload.encode()).hexdigest()


def lineage_checks(factors: list[dict[str, Any]], products: list[dict[str, Any]], report: dict[str, Any]) -> list[dict[str, Any]]:
    checks: list[dict[str, Any]] = []
    ids = [row.get("sample_id") for row in factors] + [row.get("sample_id") for row in products]
    checks.append({
        "name": "sample_id_collision",
        "status": "pass" if len(ids) == len(set(ids)) else "fail",
        "rows": len(ids),
        "duplicate_ids": len(ids) - len(set(ids)),
    })
    q_rows = [row for row in factors if row.get("factor_role") == "q"]
    q_seed_key = lambda row: (row.get("law"), row.get("parameter"), row.get("seed"), row.get("pair_bucket"), row.get("row_index"), row.get("attempt"))
    q_seed_counts = Counter(q_seed_key(row) for row in q_rows)
    checks.append({
        "name": "duplicate_q_seed_lineage",
        "status": "pass" if all(count == 1 for count in q_seed_counts.values()) else "fail",
        "effective_unit": "one q factor per accepted product logical row",
        "duplicate_seed_tuples": sum(count - 1 for count in q_seed_counts.values() if count > 1),
        "scope_note": "The paired p factor intentionally shares the product seed tuple and is excluded here.",
    })
    malformed_ids = [
        row.get("sample_id")
        for row in factors
        if not all(token in row.get("sample_id", "") for token in (
            f"seed={row.get('seed')}", f"row={row.get('row_index')}",
            f"attempt={row.get('attempt')}", row.get("pair_bucket", ""),
        ))
    ]
    checks.append({
        "name": "sample_id_field_lineage",
        "status": "pass" if not malformed_ids else "fail",
        "mismatching_ids": len(malformed_ids),
    })
    product_key = lambda row: (row.get("law"), row.get("parameter"), row.get("seed"), row.get("pair_bucket"), row.get("row_index"))
    product_counts = Counter(product_key(row) for row in products)
    checks.append({
        "name": "one_terminal_product_outcome_per_logical_row",
        "status": "pass" if all(count == 1 for count in product_counts.values()) else "fail",
        "logical_rows": len(product_counts),
        "duplicate_logical_rows": sum(count - 1 for count in product_counts.values() if count > 1),
    })
    expected_seed = report.get("seed")
    bad_seed = [row["sample_id"] for row in products if expected_seed is not None and row.get("seed") != expected_seed]
    checks.append({
        "name": "batch_seed_lineage",
        "status": "pass" if not bad_seed else "fail",
        "declared_seed": expected_seed,
        "mismatching_rows": len(bad_seed),
    })
    terminal = []
    for row in products:
        accepted = row.get("accepted")
        exhausted = (not accepted) and "no accepted product" in (row.get("rejection_reason") or "")
        terminal.append(bool(accepted) or exhausted)
    checks.append({
        "name": "terminal_acceptance_or_exhaustion",
        "status": "pass" if all(terminal) else "fail",
        "terminal_rows": sum(terminal),
        "rows": len(products),
        "contract_note": "v1 uses accepted=false plus a bounded-exhaustion reason; it does not retain rejected proposals.",
    })
    fingerprints = Counter(factor_fingerprint(row) for row in q_rows)
    checks.append({
        "name": "duplicate_independent_q_unit_geometry",
        "status": "pass" if all(count == 1 for count in fingerprints.values()) else "fail",
        "effective_unit": "one q factor per accepted product logical row",
        "duplicate_geometries": sum(count - 1 for count in fingerprints.values() if count > 1),
    })
    order_failures = 0
    for _, rows in grouped(q_rows, lambda row: (row["law"], row["parameter"], row["seed"], row["pair_bucket"])).items():
        indices = [row["row_index"] for row in rows]
        if indices != sorted(indices):
            order_failures += 1
    checks.append({
        "name": "row_order_monotone_within_declared_stratum",
        "status": "pass" if order_failures == 0 else "fail",
        "strata_out_of_order": order_failures,
        "interpretation": "serialization consistency only; monotonic row order is not a sampling-law test.",
    })
    checks.append({
        "name": "proposal_vs_accepted_conditioning",
        "status": "not_auditable_from_retained_schema",
        "missing_fields": ["one record per rejected proposal", "proposal_status", "proposal_seed_or_counter"],
        "cheapest_producer_amendment": "Emit a compact proposal ledger keyed by law/parameter/bucket/row/attempt with terminal acceptance reason; geometry payload is unnecessary for rejected proposals.",
    })
    return checks


def grouped(rows: Iterable[dict[str, Any]], key: Any) -> dict[Any, list[dict[str, Any]]]:
    result: dict[Any, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        result[key(row)].append(row)
    return result


def zoo_law_arms(factors: list[dict[str, Any]]) -> list[dict[str, Any]]:
    q_rows = [row for row in factors if row.get("factor_role") == "q"]
    arms = []
    for (law, parameter, bucket, side_count), rows in sorted(
        grouped(q_rows, lambda r: (r["law"], r["parameter"], r["pair_bucket"], r["side_count"])).items()
    ):
        angles = []
        gap_rows = []
        for row in rows:
            normal_angles = normals_from_vertices(row["vertices_ccw"])
            angles.extend(normal_angles)
            gap_rows.append(cyclic_gaps(normal_angles))
        diagnostics: dict[str, Any] = {
            "geometry_valid_rows": len(rows),
            "circular": circular_diagnostics(angles),
        }
        status = "observed_small_panel"
        interpretation = "Descriptive diagnostic only; this five-unit smoke cannot accept a null sampling law."
        if law == "current-baseline":
            diagnostics["declared_iid_uniform_angle"] = True
            diagnostics["effective_unit_contract"] = "one q factor from each accepted product row; q/p pairing is not counted twice"
        if law == "repulsive-gap":
            if parameter == "regular":
                diagnostics["dirichlet"] = {
                    "status": "inapplicable_exact_regular_limit",
                    "reason": "regular is a deterministic limiting control, not finite-alpha Dirichlet data",
                    "simplex_sum_error": max(abs(sum(cyclic_gaps(normals_from_vertices(row["vertices_ccw"]))) - TAU) for row in rows),
                }
            else:
                alpha = float(parameter.split("=", 1)[1])
                diagnostics["dirichlet"] = dirichlet_diagnostics(gap_rows, alpha, side_count)
        arms.append({
            "source": "generator-zoo-factor-shapes-v1",
            "law": law,
            "arm": parameter,
            "side_count": side_count,
            "pair_bucket": bucket,
            "sample_count": len(rows),
            "effective_independent_unit": "accepted product logical row, represented by its q factor only",
            "status": status,
            "diagnostics": diagnostics,
            "interpretation": interpretation,
        })
    return arms


def natural_law_arms(rows: list[dict[str, Any]], report: dict[str, Any] | None) -> list[dict[str, Any]]:
    if not rows:
        return [{
            "source": "generator-natural-law-expansion-v1",
            "law": "shared-latent",
            "arm": "all",
            "side_count": None,
            "sample_count": 0,
            "effective_independent_unit": "unavailable",
            "status": "not_auditable_from_retained_schema",
            "diagnostics": [{
                "name": "retained_natural_rows",
                "missing_fields": ["smoke-rows.jsonl"],
                "cheapest_producer_amendment": "Retain the existing smoke-rows.jsonl alongside its batch report.",
            }],
            "interpretation": "Source was inspected, but no retained natural-law row panel was supplied to this analyzer.",
        }]
    arms = []
    for (law, parameter, bucket), group in sorted(grouped(rows, lambda r: (r["law"], r["parameter"], r["pair_bucket"])).items()):
        statuses = Counter(row.get("status") for row in group)
        diagnostic: list[dict[str, Any]] = [{
            "name": "terminal_status",
            "status": "pass" if all(s in {"survived", "exhausted"} for s in statuses) else "fail",
            "statuses": dict(statuses),
        }]
        if law == "shared-latent":
            diagnostic.append({
                "name": "rho_latent_correlation",
                "status": "not_auditable_from_retained_schema",
                "missing_fields": ["q_gap_logits", "p_gap_logits", "q_log_supports", "p_log_supports", "or q/p factor vertices"],
                "cheapest_producer_amendment": "Retain centered pre-softmax gap logits and centered log supports for both factors (or factor vertices sufficient to recover them) in accepted shared-latent rows.",
                "reason": "CV summaries do not identify the intended coordinatewise logistic-normal correlation, including rho endpoints.",
            })
            diagnostic.append({
                "name": "shared_seed_duplicate_detection",
                "status": "not_auditable_from_retained_schema",
                "missing_fields": ["factor-level latent or geometry fingerprint"],
                "cheapest_producer_amendment": "Retain factor-level latent vectors or a deterministic factor fingerprint in each accepted row.",
            })
        arms.append({
            "source": "generator-natural-law-expansion-row-v1",
            "law": law,
            "arm": parameter,
            "side_count": parse_bucket(bucket)[0],
            "pair_bucket": bucket,
            "sample_count": len(group),
            "effective_independent_unit": "one accepted or exhausted requested product row; q/p are a paired unit",
            "status": "schema_limited" if any(item.get("status") == "not_auditable_from_retained_schema" for item in diagnostic) else "observed_small_panel",
            "diagnostics": diagnostic,
            "interpretation": "Terminal and lineage fields are auditable. Aggregate CV witnesses are insufficient for a logistic-normal joint-law acceptance claim.",
        })
    if report is not None:
        for arm in arms:
            arm["producer_report_terminal_contract"] = report.get("all_requested_rows_terminal")
    return arms


def order_bias(values: list[float]) -> dict[str, Any]:
    n = len(values)
    correlation = spearman(list(range(n)), values) if n >= 2 else math.nan
    return {
        "n": n,
        "spearman_row_index_vs_value": correlation,
        "flagged": bool(n >= 16 and math.isfinite(correlation) and abs(correlation) > 0.75),
    }


def latent_correlation(q: list[float], p: list[float], expected_rho: float) -> dict[str, Any]:
    observed = pearson(q, p)
    return {
        "coordinates": len(q),
        "observed_pearson": observed,
        "declared_rho": expected_rho,
        "absolute_error": abs(observed - expected_rho),
        "flagged": bool(math.isfinite(observed) and abs(observed - expected_rho) > 0.25),
    }


def synthetic_calibrations() -> list[dict[str, Any]]:
    rng = random.Random(20260715)
    duplicate = [{"vertices_ccw": [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]}] * 2
    duplicate_counts = Counter(factor_fingerprint(row) for row in duplicate)
    duplicate_caught = any(count > 1 for count in duplicate_counts.values())
    biased_angles = [rng.random() * (TAU / 8.0) for _ in range(256)]
    uniform_angles = [rng.random() * TAU for _ in range(256)]
    biased = circular_diagnostics(biased_angles)
    uniform = circular_diagnostics(uniform_angles)
    # Independent Gamma coordinates give a Dirichlet(alpha=4) row; declaring
    # alpha=1 deliberately turns its transformed ranks non-uniform.
    wrong_alpha_gap_rows = []
    for _ in range(128):
        gamma = [rng.gammavariate(4.0, 1.0) for _ in range(6)]
        total = sum(gamma)
        wrong_alpha_gap_rows.append([TAU * value / total for value in gamma])
    wrong_alpha = dirichlet_diagnostics(wrong_alpha_gap_rows, 1.0, 6)
    ordered = sorted(rng.random() for _ in range(64))
    order = order_bias(ordered)
    q = [rng.gauss(0.0, 1.0) for _ in range(256)]
    p = [rng.gauss(0.0, 1.0) for _ in range(256)]  # wrong rho=0 arm, declared .8
    rho = latent_correlation(q, p, 0.8)
    return [
        {"control": "duplicated_independent_units", "passed": duplicate_caught, "diagnostic": {"duplicate_geometries": 1}},
        {"control": "biased_angular_sector", "passed": biased["mean_resultant"] > 0.7 and uniform["mean_resultant"] < 0.2, "diagnostic": {"biased": biased, "uniform_reference": uniform}},
        {"control": "wrong_dirichlet_alpha", "passed": wrong_alpha["pit_ks"] > 0.12, "diagnostic": wrong_alpha},
        {"control": "law_ordered_truncation", "passed": order["flagged"], "diagnostic": order},
        {"control": "wrong_correlation_arm", "passed": rho["flagged"], "diagnostic": rho},
    ]


def audit(args: argparse.Namespace) -> dict[str, Any]:
    factors = read_jsonl(args.zoo_factors)
    products = read_jsonl(args.zoo_products)
    zoo_report = json.loads(args.zoo_report.read_text())
    natural_rows = read_jsonl(args.natural_rows) if args.natural_rows else []
    natural_report = json.loads(args.natural_report.read_text()) if args.natural_report and args.natural_report.exists() else None
    inputs = [args.zoo_factors, args.zoo_products, args.zoo_report]
    if args.natural_rows and args.natural_rows.exists():
        inputs.append(args.natural_rows)
    if args.natural_report and args.natural_report.exists():
        inputs.append(args.natural_report)
    replay = replay_evidence(args)
    if replay is not None:
        inputs.extend([
            args.replay_left, args.replay_right,
            args.replay_left_report, args.replay_right_report,
        ])
    unique_inputs = list(dict.fromkeys(path.resolve() for path in inputs))
    calibrations = synthetic_calibrations()
    return {
        "schema": "generator-law-fidelity-report-v1",
        "question": "Do retained generator rows support the declared law and effective-independent-unit assumptions needed before later distribution comparisons?",
        "source_provenance": source_provenance(),
        "producer_provenance": {
            "generator_zoo_report": {
                key: zoo_report.get(key)
                for key in ("law_version", "source_revision", "source_dirty", "seed", "max_attempts_per_row", "status_counts")
            },
            "natural_law_report": None if natural_report is None else {
                key: natural_report.get(key)
                for key in ("law_version", "source_revision", "source_tree", "source_dirty", "seed", "max_attempts_per_row", "all_requested_rows_terminal", "status_counts")
            },
        },
        "inputs": [input_identity(path) for path in unique_inputs],
        "calibrations": calibrations,
        "calibration_status": "pass" if all(item["passed"] for item in calibrations) else "fail",
        "global_checks": lineage_checks(factors, products, zoo_report),
        "law_arms": zoo_law_arms(factors) + natural_law_arms(natural_rows, natural_report),
        "cross_seed_same_law": {
            "status": "not_auditable_from_retained_schema" if len({row.get("seed") for row in products}) < 2 else "available_for_two_sample_diagnostic",
            "observed_seeds": sorted({row.get("seed") for row in products}),
            "interpretation": "A same-law cross-seed comparison is a consistency check only, never proof that the declared law is correct.",
            "cheapest_next_input": "At least two retained panels per law/parameter/side-count with distinct declared master seeds.",
        },
        **({"deterministic_replay": replay} if replay is not None else {}),
        "interpretation_boundary": "Passes establish calibration of these diagnostic implementations and schema/lineage observations only. They do not prove a null law, population IID behavior after conditioning, or target transfer.",
    }


def main() -> None:
    default = ROOT / "experiments/sys-datascience/methods"
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--zoo-factors", type=Path, default=default / "generator-zoo-smoke/artifacts/factor-shapes.jsonl")
    parser.add_argument("--zoo-products", type=Path, default=default / "generator-zoo-smoke/artifacts/product-smoke.jsonl")
    parser.add_argument("--zoo-report", type=Path, default=default / "generator-zoo-smoke/artifacts/batch-report.json")
    parser.add_argument("--natural-rows", type=Path)
    parser.add_argument("--natural-report", type=Path)
    parser.add_argument("--out", type=Path, default=Path(__file__).parent / "artifacts/report.json")
    parser.add_argument("--replay-left", type=Path)
    parser.add_argument("--replay-right", type=Path)
    parser.add_argument("--replay-left-report", type=Path)
    parser.add_argument("--replay-right-report", type=Path)
    parser.add_argument("--replay-left-command")
    parser.add_argument("--replay-right-command")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        failed = [item for item in synthetic_calibrations() if not item["passed"]]
        if failed:
            raise SystemExit(f"synthetic calibrations failed: {failed}")
        print("synthetic calibrations: pass")
        return
    replay_args = [
        args.replay_left, args.replay_right, args.replay_left_report,
        args.replay_right_report, args.replay_left_command, args.replay_right_command,
    ]
    if any(value is not None for value in replay_args) and any(value is None for value in replay_args):
        parser.error("replay requires left/right rows, producer reports, and exact producer commands")
    report = audit(args)
    write_json(args.out, report)
    print(json.dumps({"out": str(args.out), "calibration_status": report["calibration_status"], "law_arms": len(report["law_arms"])}, sort_keys=True))


if __name__ == "__main__":
    main()
