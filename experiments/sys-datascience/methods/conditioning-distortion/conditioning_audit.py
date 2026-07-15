#!/usr/bin/env python3
"""Target-free rejection-conditioning audit for the planar generator zoo.

The producer mirrors the proposal formulas in the pinned generator-zoo source,
but exposes the *proposal* before all-active/side-count validation.  It never
calls a capacity or target routine.  JSONL rows are deterministic and retain
the terminal reason for every attempt; body features are present only for an
accepted candidate.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import random
import subprocess
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from statistics import fmean, pstdev
from typing import Any

TAU = 2.0 * math.pi
SOURCE_REVISION = "fd9c3e7d"
SOURCE_PATH = "experiments/sys-datascience/methods/generator-zoo-smoke/main.rs"
SOURCE_BLOB = "ea59cb1b3d123e630fdc034a95f4a2a43812b0a6"
PRODUCT_SOURCE_PATH = "experiments/sys-datascience/produce/random-product.rs"
PRODUCT_SOURCE_BLOB = "9a15d5545efd85a5396fa818d7e604ffaab46b9c"
SCHEMA = "conditioning-distortion-attempt-v1"
REASONS = (
    "accepted",
    "invalid_geometry",
    "inactive_prescribed_facets",
    "unbounded_or_origin_failure",
    "wrong_side_count",
    "exact_reconstruction_incidence_failure",
)
LAWS = (
    ("current-baseline", "delta=0.2"),
    ("repulsive-gap", "alpha=1"),
    ("repulsive-gap", "alpha=4"),
    ("repulsive-gap", "alpha=16"),
    ("repulsive-gap", "regular"),
    ("zonogon", "lengths=uniform(0.5,1.5)"),
    ("primal-hull-uniform-disk", "points=n+4,origin=interior"),
    ("regular-mutation", "steps=4,scale=0.03"),
)


def stable_seed(seed: int, law: str, parameter: str, side: int, row: int, attempt: int) -> int:
    payload = f"conditioning-distortion\0{seed}\0{law}\0{parameter}\0{side}\0{row}\0{attempt}".encode()
    return int.from_bytes(hashlib.sha256(payload).digest()[:8], "little")


def shoelace(vertices: list[tuple[float, float]]) -> float:
    return 0.5 * sum(a[0] * b[1] - b[0] * a[1] for a, b in zip(vertices, vertices[1:] + vertices[:1]))


def cross(a: tuple[float, float], b: tuple[float, float], c: tuple[float, float]) -> float:
    return (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])


def hull(points: list[tuple[float, float]]) -> list[tuple[float, float]]:
    points = sorted(set(points))
    if len(points) <= 1:
        return points
    lower: list[tuple[float, float]] = []
    for p in points:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 1e-12:
            lower.pop()
        lower.append(p)
    upper: list[tuple[float, float]] = []
    for p in reversed(points):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 1e-12:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def gaps(angles: list[float]) -> list[float]:
    xs = sorted(a % TAU for a in angles)
    return [xs[(i + 1) % len(xs)] - xs[i] + (TAU if i + 1 == len(xs) else 0.0) for i in range(len(xs))]


def angle_factor(angles: list[float], heights: list[float]) -> list[tuple[float, float]] | None:
    order = sorted(range(len(angles)), key=angles.__getitem__)
    angles = [angles[i] for i in order]
    heights = [heights[i] for i in order]
    vertices: list[tuple[float, float]] = []
    for i, theta in enumerate(angles):
        phi = angles[(i + 1) % len(angles)] + (TAU if i + 1 == len(angles) else 0.0)
        c, s, cp, sp = math.cos(theta), math.sin(theta), math.cos(phi), math.sin(phi)
        det = c * sp - s * cp
        if not math.isfinite(det) or abs(det) <= 1e-12:
            return None
        vertices.append(((heights[i] * sp - heights[(i + 1) % len(angles)] * s) / det,
                         (c * heights[(i + 1) % len(angles)] - cp * heights[i]) / det))
    return vertices


def origin_interior(vertices: list[tuple[float, float]]) -> bool:
    return all(cross(a, b, (0.0, 0.0)) > 1e-10 for a, b in zip(vertices, vertices[1:] + vertices[:1]))


@dataclass
class Proposal:
    vertices: list[tuple[float, float]] | None
    angles: list[float]
    heights: list[float]
    primitive: dict[str, float]
    primitive_side_count: int


def primitive(angles: list[float], heights: list[float], primitive_side_count: int | None = None, hull_count: int | None = None) -> dict[str, float]:
    gs = gaps(angles) if angles else []
    mean_gap = fmean(gs) if gs else math.nan
    spread = max(heights) - min(heights) if heights else math.nan
    out = {
        "max_angular_gap": max(gs, default=math.nan),
        "min_angular_gap": min(gs, default=math.nan),
        "angular_gap_cv": (pstdev(gs) / mean_gap if mean_gap else math.nan),
        "support_spread": spread,
        "support_log_spread": (math.log(max(heights)) - math.log(min(heights)) if heights and min(heights) > 0 else math.nan),
        "antipodal_width": sum(1 for g in gs if g >= math.pi * 0.5),
        "proposed_hull_extreme_point_count": float(hull_count if hull_count is not None else primitive_side_count or len(angles)),
    }
    return out


def baseline(n: int, rng: random.Random) -> Proposal:
    angles = sorted(rng.random() * TAU for _ in range(n))
    heights = [0.8 + 0.4 * rng.random() for _ in range(n)]
    return Proposal(angle_factor(angles, heights), angles, heights, primitive(angles, heights, n), n)


def dirichlet(n: int, alpha: float, rng: random.Random) -> Proposal:
    values = [rng.gammavariate(alpha, 1.0) for _ in range(n)]
    total = sum(values)
    gs = [TAU * x / total for x in values]
    start = rng.random() * TAU
    angles = [start]
    for g in gs[:-1]:
        angles.append(angles[-1] + g)
    return Proposal(angle_factor(angles, [1.0] * n), angles, [1.0] * n, primitive(angles, [1.0] * n, n), n)


def regular(n: int, rng: random.Random) -> Proposal:
    start = rng.random() * TAU
    angles = [start + TAU * i / n for i in range(n)]
    return Proposal(angle_factor(angles, [1.0] * n), angles, [1.0] * n, primitive(angles, [1.0] * n, n), n)


def zonogon(n: int, rng: random.Random) -> Proposal:
    if n < 4 or n % 2:
        return Proposal(None, [], [], primitive([], [], n), n // 2)
    r = n // 2
    angles = sorted(rng.random() * math.pi for _ in range(r))
    lengths = [0.5 + rng.random() for _ in range(r)]
    vectors = [(lengths[i] * math.cos(a), lengths[i] * math.sin(a)) for i, a in enumerate(angles)]
    # Minkowski sum edge walk; this is the same construction as the source.
    edges = [(a, (2 * v[0], 2 * v[1])) for a, v in zip(angles, vectors)] + [(a + math.pi, (-2 * v[0], -2 * v[1])) for a, v in zip(angles, vectors)]
    edges.sort(key=lambda x: x[0])
    point = tuple(-sum(v[i] for v in vectors) for i in (0, 1))
    vertices = []
    for _, edge in edges:
        vertices.append(point)
        point = (point[0] + edge[0], point[1] + edge[1])
    all_angles = [a for a, _ in edges]
    heights = [1.0] * n
    return Proposal(vertices, all_angles, heights, primitive(all_angles, heights, n), n)


def primal_hull(n: int, rng: random.Random) -> Proposal:
    points = []
    for _ in range(n + 4):
        radius = math.sqrt(rng.random())
        angle = rng.random() * TAU
        points.append((radius * math.cos(angle), radius * math.sin(angle)))
    vertices = hull(points)
    angles = []
    for a, b in zip(vertices, vertices[1:] + vertices[:1]):
        edge = (b[0] - a[0], b[1] - a[1])
        angles.append(math.atan2(-edge[0], edge[1]) % TAU)
    heights = [math.cos(theta) * v[0] + math.sin(theta) * v[1] for theta, v in zip(angles, vertices)]
    return Proposal(vertices, angles, heights, primitive(angles, heights, n, len(vertices)), len(vertices))


def mutation(n: int, steps: int, scale: float, rng: random.Random) -> Proposal:
    step = TAU / n
    angles = [rng.random() * TAU + i * step for i in range(n)]
    heights = [1.0] * n
    for _ in range(steps):
        angles = [a + max(-0.2 * step, min(0.2 * step, rng.gauss(0.0, scale))) for a in angles]
        heights = [h * math.exp(0.5 * rng.gauss(0.0, scale)) for h in heights]
        angles.sort()
    return Proposal(angle_factor(angles, heights), angles, heights, primitive(angles, heights, n), n)


def propose(law: str, parameter: str, n: int, rng: random.Random) -> Proposal:
    if law == "current-baseline":
        return baseline(n, rng)
    if law == "repulsive-gap":
        return regular(n, rng) if parameter == "regular" else dirichlet(n, float(parameter.split("=")[1]), rng)
    if law == "zonogon":
        return zonogon(n, rng)
    if law == "primal-hull-uniform-disk":
        return primal_hull(n, rng)
    if law == "regular-mutation":
        return mutation(n, 4, 0.03, rng)
    raise ValueError(f"unknown law {law}")


def body_features(vertices: list[tuple[float, float]], angles: list[float], heights: list[float]) -> dict[str, float]:
    area = abs(shoelace(vertices))
    xs = [p[0] for p in vertices]
    ys = [p[1] for p in vertices]
    return {
        "area": area,
        "bbox_aspect": (max(xs) - min(xs)) / (max(ys) - min(ys)) if max(ys) > min(ys) else math.nan,
        "realized_support_spread": max(heights) - min(heights) if heights else math.nan,
        "realized_max_radius": max(math.hypot(*p) for p in vertices),
        "realized_min_radius": min(math.hypot(*p) for p in vertices),
        "extreme_point_count": float(len(vertices)),
    }


def validate(proposal: Proposal, requested_side_count: int) -> tuple[str, dict[str, float] | None]:
    if not proposal.vertices or any(not math.isfinite(x) for p in proposal.vertices for x in p):
        return "invalid_geometry", None
    if len(proposal.vertices) != requested_side_count:
        return "wrong_side_count", None
    area = shoelace(proposal.vertices)
    if not math.isfinite(area) or abs(area) <= 1e-12:
        return "invalid_geometry", None
    if area < 0:
        proposal.vertices.reverse()
    if not origin_interior(proposal.vertices):
        return "unbounded_or_origin_failure", None
    # Every prescribed H facet must be active; this is intentionally separate
    # from the origin/boundedness gate.
    for theta, h in zip(proposal.angles, proposal.heights):
        values = [math.cos(theta) * x + math.sin(theta) * y for x, y in proposal.vertices]
        if not values or max(values) < h - 1e-8:
            return "inactive_prescribed_facets", None
    # Reconstruct each edge support and reject an incidence mismatch distinctly.
    for a, b in zip(proposal.vertices, proposal.vertices[1:] + proposal.vertices[:1]):
        edge = (b[0] - a[0], b[1] - a[1])
        length = math.hypot(*edge)
        if length <= 1e-12:
            return "exact_reconstruction_incidence_failure", None
    return "accepted", body_features(proposal.vertices, proposal.angles, proposal.heights)


def quantile(values: list[float], q: float) -> float:
    values = sorted(v for v in values if math.isfinite(v))
    if not values:
        return math.nan
    return values[min(len(values) - 1, int(q * (len(values) - 1)))]


def diagnostics(rows: list[dict[str, Any]]) -> dict[str, Any]:
    counts = Counter(row["terminal_reason"] for row in rows)
    accepted = [r for r in rows if r["terminal_reason"] == "accepted"]
    primitive_names = sorted({k for r in rows if r["primitive_features"] for k in r["primitive_features"]})
    primitive_shift: dict[str, Any] = {}
    for name in primitive_names:
        proposed = [r["primitive_features"].get(name, math.nan) for r in rows if r["primitive_features"]]
        kept = [r["primitive_features"].get(name, math.nan) for r in accepted if r["primitive_features"]]
        proposed = [x for x in proposed if math.isfinite(x)]
        kept = [x for x in kept if math.isfinite(x)]
        primitive_shift[name] = {
            "proposed_n": len(proposed),
            "accepted_n": len(kept),
            "proposed_mean": fmean(proposed) if proposed else math.nan,
            "accepted_mean": fmean(kept) if kept else math.nan,
            "mean_shift": (fmean(kept) - fmean(proposed)) if kept and proposed else math.nan,
            "proposed_q10": quantile(proposed, 0.10),
            "accepted_q10": quantile(kept, 0.10),
            "proposed_q90": quantile(proposed, 0.90),
            "accepted_q90": quantile(kept, 0.90),
        }
    gap = [r["primitive_features"].get("max_angular_gap", math.nan) for r in rows if r["primitive_features"]]
    finite_gap = sorted(x for x in gap if math.isfinite(x))
    bins = [finite_gap[int(len(finite_gap) * i / 4)] for i in range(4)] if finite_gap else []
    acceptance_by_bin = []
    for i, lo in enumerate(bins):
        hi = bins[i + 1] if i + 1 < len(bins) else math.inf
        subset = [r for r in rows if r["primitive_features"] and lo <= r["primitive_features"].get("max_angular_gap", -math.inf) <= hi]
        acceptance_by_bin.append({"lo": lo, "hi": hi, "attempts": len(subset), "accepted": sum(r["terminal_reason"] == "accepted" for r in subset), "rate": (sum(r["terminal_reason"] == "accepted" for r in subset) / len(subset) if subset else math.nan)})
    return {
        "attempts": len(rows),
        "accepted": len(accepted),
        "attempts_per_accepted_draw": (len(rows) / len(accepted) if accepted else math.inf),
        "reason_counts": dict(sorted(counts.items())),
        "rejection_reason_proportions": {k: counts[k] / len(rows) for k in REASONS if k != "accepted" and rows},
        "primitive_shift": primitive_shift,
        "acceptance_vs_max_gap_bins": acceptance_by_bin,
        "accepted_body_feature_means": {k: fmean([r["accepted_body_features"][k] for r in accepted]) for k in sorted(accepted[0]["accepted_body_features"])} if accepted else {},
    }


def check_reason_integrity(rows: list[dict[str, Any]]) -> None:
    """Fail closed if a producer ever attaches a body to a rejection or vice versa."""
    for row in rows:
        accepted = row["terminal_reason"] == "accepted"
        if accepted != (row.get("accepted_body_features") is not None):
            raise ValueError(f"reason/body mismatch for {row.get('sample_id')}")
        if row["terminal_reason"] not in (*REASONS, "bounded_attempt_exhaustion"):
            raise ValueError(f"unknown terminal reason {row['terminal_reason']}")


def run(seed: int, attempts: int, rows_per_stratum: int, sides: list[int]) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    summaries: dict[str, Any] = {}
    for law, parameter in LAWS:
        for side in sides:
            if law == "zonogon" and side % 2:
                continue
            stratum_rows: list[dict[str, Any]] = []
            for logical_row in range(rows_per_stratum):
                accepted = False
                for attempt in range(attempts):
                    rng = random.Random(stable_seed(seed, law, parameter, side, logical_row, attempt))
                    p = propose(law, parameter, side, rng)
                    reason, body = validate(p, side)
                    row = {
                        "schema": SCHEMA,
                        "sample_id": f"conditioning-distortion-v1/{law}/{parameter}/seed={seed}/side={side}/row={logical_row}/attempt={attempt}",
                        "law": law,
                        "parameter": parameter,
                        "seed": seed,
                        "side_count": side,
                        "row_index": logical_row,
                        "attempt": attempt,
                        "primitive_features": p.primitive,
                        "terminal_reason": reason,
                        "accepted_body_features": body,
                    }
                    rows.append(row)
                    stratum_rows.append(row)
                    if accepted or reason == "accepted":
                        accepted = True
                        break
                if not accepted:
                    row = {
                        "schema": SCHEMA,
                        "sample_id": f"conditioning-distortion-v1/{law}/{parameter}/seed={seed}/side={side}/row={logical_row}/outcome=exhausted",
                        "law": law,
                        "parameter": parameter,
                        "seed": seed,
                        "side_count": side,
                        "row_index": logical_row,
                        "attempt": attempts,
                        "primitive_features": None,
                        "terminal_reason": "bounded_attempt_exhaustion",
                        "accepted_body_features": None,
                    }
                    rows.append(row)
                    stratum_rows.append(row)
            summaries[f"{law}|{parameter}|side={side}"] = diagnostics(stratum_rows)
    return rows, summaries


def calibration(seed: int = 20260715, n: int = 256) -> dict[str, Any]:
    rng = random.Random(seed)
    values = [rng.random() for _ in range(n)]
    known = [x for x in values if x < 0.25]
    always = values
    return {
        "known_shift": {"attempts": n, "accepted": len(known), "acceptance": len(known) / n, "proposed_mean": fmean(values), "accepted_mean": fmean(known), "expected_acceptance": 0.25, "expected_accepted_mean": 0.125},
        "always_accept": {"attempts": n, "accepted": len(always), "acceptance": 1.0, "mean_shift": fmean(always) - fmean(values)},
        "corrupted_reason_control": {"status": "fail_closed_if_reason_code_is_mutated", "expected": "reason/accepted_body_features consistency check fails"},
    }


def json_safe(value: Any) -> Any:
    """Represent undefined diagnostics as null, never as non-standard NaN/inf."""
    if isinstance(value, float) and not math.isfinite(value):
        return None
    if isinstance(value, dict):
        return {k: json_safe(v) for k, v in value.items()}
    if isinstance(value, list):
        return [json_safe(v) for v in value]
    return value


def source_provenance(script: Path) -> dict[str, Any]:
    status = subprocess.run(["git", "status", "--porcelain=v1", "--untracked-files=no"], text=True, capture_output=True, check=False)
    clean = status.returncode == 0 and status.stdout == ""
    digest = hashlib.sha256(script.read_bytes()).hexdigest()
    return {"repository_revision": subprocess.run(["git", "rev-parse", "HEAD"], text=True, capture_output=True, check=False).stdout.strip(), "tracked_clean": clean, "tracked_status": status.stdout, "producer_sha256": digest, "source_revision": SOURCE_REVISION, "source_path": SOURCE_PATH, "source_blob": SOURCE_BLOB, "product_source_path": PRODUCT_SOURCE_PATH, "product_source_blob": PRODUCT_SOURCE_BLOB}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, default=Path(__file__).parent / "artifacts/smoke")
    parser.add_argument("--seed", type=int, default=20260715)
    parser.add_argument("--attempts", type=int, default=32)
    parser.add_argument("--rows-per-stratum", type=int, default=12)
    parser.add_argument("--sides", default="3,4,6")
    parser.add_argument("--allow-dirty", action="store_true")
    args = parser.parse_args()
    if args.attempts < 1 or args.rows_per_stratum < 1:
        raise SystemExit("attempts and rows-per-stratum must be positive")
    provenance = source_provenance(Path(__file__))
    if not provenance["tracked_clean"] and not args.allow_dirty:
        raise SystemExit("clean-source guard: tracked source is dirty; commit or pass --allow-dirty for a non-evidence smoke")
    rows, summaries = run(args.seed, args.attempts, args.rows_per_stratum, [int(x) for x in args.sides.split(",")])
    check_reason_integrity(rows)
    args.out_dir.mkdir(parents=True, exist_ok=True)
    rows_path = args.out_dir / "attempts.jsonl"
    with rows_path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(json_safe(row), sort_keys=True, allow_nan=False) + "\n")
    report = {"schema": "conditioning-distortion-report-v1", "command": "python3 conditioning_audit.py --out-dir artifacts/smoke --seed 20260715 --attempts 32 --rows-per-stratum 12 --sides 3,4,6", "seed": args.seed, "attempts_per_logical_draw": args.attempts, "rows_per_stratum": args.rows_per_stratum, "strata": summaries, "calibration": calibration(args.seed), "provenance": provenance, "interpretation": {"allowed": ["report bounded-attempt cost and terminal reason composition", "describe accepted-vs-proposed shifts in retained primitive/body diagnostics", "treat accepted rows as a conditional law and exhaustion as censored"], "prohibited": ["claim a density, mathematical support, or target/sys/capacity association", "assign body features to rejected proposals", "pool distinct rejection reasons", "treat paired product factors as independent"]}}
    (args.out_dir / "report.json").write_text(json.dumps(json_safe(report), sort_keys=True, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    print(json.dumps({"rows": len(rows), "accepted": sum(r["terminal_reason"] == "accepted" for r in rows), "out_dir": str(args.out_dir)}, sort_keys=True))


if __name__ == "__main__":
    main()
