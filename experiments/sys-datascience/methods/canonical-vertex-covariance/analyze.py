#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""Canonical-extreme-vertex covariance symplectic eccentricity diagnostic.

The input contract is deliberately narrower than the producer's dual-facet
representation: this script uses the exact primal `vertices_rational` payload,
after rational-value deduplication, and verifies that every retained point is an
extreme point of its full-dimensional convex hull.  It therefore does not turn
an arbitrary list of sampled boundary/interior points into a feature.
"""

from __future__ import annotations

import argparse
from collections import defaultdict
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import statistics
import subprocess
from typing import Any, Iterable

import numpy as np
from scipy import linalg, stats
from scipy.spatial import ConvexHull, QhullError


HERE = Path(__file__).resolve().parent
SYS_DATASCIENCE = HERE.parents[1]
EXPERIMENTS = SYS_DATASCIENCE.parent
PRODUCE = EXPERIMENTS / "polytope-datasets"
PREPARE = EXPERIMENTS / "polytope-invariant-table"

# Coordinates are (q1, q2, p1, p2), the project-wide convention.
J = np.array(
    [[0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0],
     [-1.0, 0.0, 0.0, 0.0], [0.0, -1.0, 0.0, 0.0]],
    dtype=float,
)


class VertexContractError(ValueError):
    """The supplied points are not canonical distinct extreme vertices."""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts" / "current")
    parser.add_argument("--self-test", action="store_true")
    return parser.parse_args()


def read_jsonl(path: Path) -> Iterable[dict[str, Any]]:
    with path.open() as handle:
        for number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            if number == 1 and line.startswith("version https://git-lfs.github.com/spec/"):
                raise VertexContractError(f"{path} is an unhydrated Git LFS pointer")
            row = json.loads(line)
            if not isinstance(row, dict):
                raise VertexContractError(f"expected JSON object in {path}:{number}")
            yield row


def canonical_extreme_vertices(vertices_rational: Any) -> np.ndarray:
    """Decode, exactly deduplicate, and validate a 4D extreme-vertex list."""
    if not isinstance(vertices_rational, list):
        raise VertexContractError("missing vertices_rational list")
    exact: dict[tuple[Fraction, Fraction, Fraction, Fraction], np.ndarray] = {}
    for item in vertices_rational:
        if not isinstance(item, list) or len(item) != 4 or not all(isinstance(x, str) for x in item):
            raise VertexContractError("vertices_rational must contain four rational strings per vertex")
        try:
            key = tuple(Fraction(x) for x in item)
            point = np.array([float(x) for x in key], dtype=float)
        except (ValueError, ZeroDivisionError) as error:
            raise VertexContractError(f"invalid rational vertex: {item!r}") from error
        if not np.all(np.isfinite(point)):
            raise VertexContractError("non-finite decoded vertex")
        exact[key] = point
    points = np.stack(list(exact.values())) if exact else np.empty((0, 4))
    if len(points) < 5:
        raise VertexContractError("fewer than five distinct vertices cannot define a 4-polytope")
    centered = points - points.mean(axis=0)
    scale = max(float(np.linalg.norm(centered, ord=2)), 1.0)
    if np.linalg.matrix_rank(centered, tol=1e-10 * scale) != 4:
        raise VertexContractError("vertices are not numerically full-dimensional")
    try:
        hull = ConvexHull(points)
    except QhullError as error:
        raise VertexContractError("convex-hull validation failed") from error
    if set(hull.vertices) != set(range(len(points))):
        raise VertexContractError("input contains a non-extreme vertex")
    return points


def covariance_rho(points: np.ndarray) -> dict[str, float | int]:
    """Return rho=nu_2/nu_1 for centered uniform extreme-vertex covariance."""
    if points.ndim != 2 or points.shape[1] != 4:
        raise VertexContractError("expected an n by 4 canonical vertex array")
    centered = points - points.mean(axis=0)
    covariance = centered.T @ centered / len(points)
    ordinary = np.linalg.eigvalsh(covariance)
    condition = float(ordinary[-1] / ordinary[0])
    # The raw random generators occasionally create extremely elongated
    # polytopes.  A stricter cutoff than mere positive-definiteness prevents a
    # visually finite f64 spectrum from being mistaken for a stable feature.
    if ordinary[0] <= max(ordinary[-1], 1.0) * 1e-12 or condition > 1e10:
        raise VertexContractError("covariance is not numerically positive definite")
    eigenvalues = np.linalg.eigvals(1j * J @ covariance)
    if np.max(np.abs(eigenvalues.imag)) > 1e-8 * max(1.0, np.max(np.abs(eigenvalues.real))):
        raise VertexContractError("symplectic spectrum has unresolved imaginary numerical error")
    paired = np.sort(np.abs(eigenvalues.real))
    nu1 = float((paired[0] + paired[1]) / 2.0)
    nu2 = float((paired[2] + paired[3]) / 2.0)
    pair_error = max(abs(paired[0] - paired[1]), abs(paired[2] - paired[3]))
    if nu1 <= 0.0 or pair_error > 1e-7 * max(nu2, 1.0):
        raise VertexContractError("symplectic eigenvalue pairing is numerically ill-conditioned")
    return {
        "rho": nu2 / nu1,
        "nu1": nu1,
        "nu2": nu2,
        "vertex_count": int(len(points)),
        "covariance_condition": condition,
        "symplectic_pair_error": float(pair_error),
    }


def metric_from_rationals(vertices_rational: Any) -> dict[str, float | int]:
    return covariance_rho(canonical_extreme_vertices(vertices_rational))


def random_sp4(seed: int) -> np.ndarray:
    rng = np.random.default_rng(seed)
    h = rng.normal(size=(4, 4))
    h = (h + h.T) / 2.0
    return linalg.expm(J @ h)


def assert_close(left: float, right: float, label: str) -> None:
    if not np.isclose(left, right, rtol=2e-9, atol=2e-10):
        raise AssertionError(f"{label}: {left} != {right}")


def run_self_tests() -> None:
    # Uniform points +/- (2,0,0,0), ..., +/- (0,0,0,8) have
    # C=diag(1,4,9,16), hence symplectic eigenvalues (3,8).
    fixture = np.array(
        [[2, 0, 0, 0], [-2, 0, 0, 0], [0, 4, 0, 0], [0, -4, 0, 0],
         [0, 0, 6, 0], [0, 0, -6, 0], [0, 0, 0, 8], [0, 0, 0, -8]], dtype=float
    )
    base = covariance_rho(fixture)
    assert_close(float(base["nu1"]), 3.0, "analytic nu1")
    assert_close(float(base["nu2"]), 8.0, "analytic nu2")
    assert_close(float(base["rho"]), 8.0 / 3.0, "analytic rho")
    assert_close(float(covariance_rho(fixture[[3, 6, 1, 7, 0, 4, 2, 5]])["rho"]), float(base["rho"]), "permutation")
    assert_close(float(covariance_rho(fixture + np.array([7.0, -2.0, 3.0, 5.0]))["rho"]), float(base["rho"]), "translation")
    assert_close(float(covariance_rho(3.5 * fixture)["rho"]), float(base["rho"]), "scale")
    assert_close(float(covariance_rho((random_sp4(20260712) @ fixture.T).T)["rho"]), float(base["rho"]), "Sp(4)")
    rational = [[str(int(x)) for x in point] for point in fixture]
    rational_duplicate = ["4/2", "0/3", "0/4", "0/5"]
    assert_close(float(metric_from_rationals(rational + [rational_duplicate])["rho"]), float(base["rho"]), "duplicate canonicalization")
    try:
        canonical_extreme_vertices(rational + [["0", "0", "0", "0"]])
    except VertexContractError:
        pass
    else:
        raise AssertionError("non-extreme input was accepted")


def source_to_poly_id() -> dict[str, str]:
    result: dict[str, str] = {}
    for row in read_jsonl(PREPARE / "polytope-provenance-table.jsonl"):
        source_name, poly_id = row.get("source_name"), row.get("poly_id")
        if isinstance(source_name, str) and isinstance(poly_id, str):
            if source_name in result and result[source_name] != poly_id:
                raise VertexContractError(f"ambiguous source_name {source_name!r}")
            result[source_name] = poly_id
    return result


def prepared_sys() -> dict[str, float]:
    result: dict[str, float] = {}
    for row in read_jsonl(PREPARE / "polytope-table.jsonl"):
        if isinstance(row.get("poly_id"), str) and isinstance(row.get("sys"), int | float):
            result[row["poly_id"]] = float(row["sys"])
    return result


def bucket_for(row: dict[str, Any], kind: str) -> str:
    if kind == "random":
        return f"random:F{int(row['facet_count'])}"
    return f"random_product:{int(row['k'])}x{int(row['m'])}"


def spearman(values: list[float], targets: list[float]) -> float | None:
    if len(values) < 3 or len(set(values)) < 2 or len(set(targets)) < 2:
        return None
    value = float(stats.spearmanr(values, targets).statistic)
    return value if np.isfinite(value) else None


def pearson(values: list[float], targets: list[float]) -> float | None:
    if len(values) < 3 or np.std(values) == 0.0 or np.std(targets) == 0.0:
        return None
    value = float(stats.pearsonr(values, targets).statistic)
    return value if np.isfinite(value) else None


def summarize(rows: list[dict[str, Any]]) -> dict[str, Any]:
    by_bucket: dict[str, list[dict[str, Any]]] = defaultdict(list)
    by_vertex: dict[int, list[dict[str, Any]]] = defaultdict(list)
    by_bucket_vertex: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_bucket[row["bucket"]].append(row)
        by_vertex[int(row["vertex_count"])].append(row)
        by_bucket_vertex[f"{row['bucket']}|V={int(row['vertex_count'])}"].append(row)

    def group_summary(groups: dict[Any, list[dict[str, Any]]]) -> list[dict[str, Any]]:
        answer = []
        for label, group in sorted(groups.items(), key=lambda item: str(item[0])):
            rhos = [float(x["rho"]) for x in group]
            systems = [float(x["sys"]) for x in group]
            counts = [float(x["vertex_count"]) for x in group]
            answer.append({
                "group": str(label), "rows": len(group),
                "rho_min": min(rhos), "rho_median": float(statistics.median(rhos)), "rho_max": max(rhos),
                "sys_median": float(statistics.median(systems)),
                "vertex_count_values": sorted(set(int(x) for x in counts)),
                "spearman_rho_sys": spearman(rhos, systems),
                "spearman_rho_vertex_count": spearman(rhos, counts),
            })
        return answer

    # This deliberately named rank-residualized statistic is descriptive, not
    # a partial-Spearman estimator or a causal/combinatorial independence test.
    bucket_rank_residual_rho, bucket_rank_residual_sys, bucket_rank_residual_vertex = [], [], []
    for group in by_bucket.values():
        rho_ranks = stats.rankdata([float(x["rho"]) for x in group])
        sys_ranks = stats.rankdata([float(x["sys"]) for x in group])
        vertex_ranks = stats.rankdata([float(x["vertex_count"]) for x in group])
        bucket_rank_residual_rho.extend(rho_ranks - rho_ranks.mean())
        bucket_rank_residual_sys.extend(sys_ranks - sys_ranks.mean())
        bucket_rank_residual_vertex.extend(vertex_ranks - vertex_ranks.mean())
    stratum_spearmans = []
    joint_rank_residual_rho, joint_rank_residual_sys = [], []
    for group in by_bucket_vertex.values():
        rho_values = [float(x["rho"]) for x in group]
        sys_values = [float(x["sys"]) for x in group]
        stratum_spearmans.append({
            "stratum": f"{group[0]['bucket']}|V={int(group[0]['vertex_count'])}",
            "rows": len(group),
            "spearman_rho_sys": spearman(rho_values, sys_values),
        })
        rho_ranks = stats.rankdata(rho_values)
        sys_ranks = stats.rankdata(sys_values)
        joint_rank_residual_rho.extend(rho_ranks - rho_ranks.mean())
        joint_rank_residual_sys.extend(sys_ranks - sys_ranks.mean())
    rhos = [float(x["rho"]) for x in rows]
    systems = [float(x["sys"]) for x in rows]
    counts = [float(x["vertex_count"]) for x in rows]
    return {
        "rows": len(rows),
        "global": {
            "spearman_rho_sys": spearman(rhos, systems),
            "spearman_rho_vertex_count": spearman(rhos, counts),
            "spearman_vertex_count_sys": spearman(counts, systems),
        },
        "pooled_pearson_of_within_bucket_rank_residuals": {
            "rho_sys": pearson(bucket_rank_residual_rho, bucket_rank_residual_sys),
            "rho_vertex_count": pearson(bucket_rank_residual_rho, bucket_rank_residual_vertex),
            "vertex_count_sys": pearson(bucket_rank_residual_vertex, bucket_rank_residual_sys),
            "interpretation": "Pooled Pearson correlation after separately ranking and centering values within each bucket; descriptive only.",
        },
        "bucket_vertex_count_stratum_spearmans": {
            "eligible_strata": sum(1 for row in stratum_spearmans if row["spearman_rho_sys"] is not None),
            "all_strata": stratum_spearmans,
        },
        "pooled_pearson_of_within_bucket_vertex_count_rank_residuals": {
            "rows": len(joint_rank_residual_rho),
            "rho_sys": pearson(joint_rank_residual_rho, joint_rank_residual_sys),
            "interpretation": "Pooled Pearson correlation after separately ranking and centering values within each exact bucket-and-vertex-count stratum; descriptive only.",
        },
        "by_bucket": group_summary(by_bucket),
        "by_vertex_count": group_summary(by_vertex),
    }


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def repo_command(*args: str) -> str:
    root = HERE.parents[3]
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


def tracked_lfs_identity(relative_path: str) -> dict[str, str] | None:
    try:
        text = repo_command("show", f"HEAD:{relative_path}")
    except subprocess.CalledProcessError:
        return None
    lines = dict(line.split(" ", 1) for line in text.splitlines() if " " in line)
    oid = lines.get("oid", "")
    if not oid.startswith("sha256:"):
        return None
    return {"oid_sha256": oid.removeprefix("sha256:"), "size": lines.get("size", "unknown")}


def input_identity(path: Path) -> dict[str, Any]:
    root = HERE.parents[3]
    relative = str(path.relative_to(root))
    first = path.read_text(errors="replace")[:128]
    return {
        "path": relative,
        "sha256": sha256_file(path),
        "hydrated": not first.startswith("version https://git-lfs.github.com/spec/"),
        "tracked_lfs": tracked_lfs_identity(relative),
    }


def provenance(inputs: list[Path], out_dir: Path) -> dict[str, Any]:
    root = HERE.parents[3]
    relative_inputs = [str(path.relative_to(root)) for path in inputs]
    producer_relative = str(Path(__file__).resolve().relative_to(root))
    tracked_producer = subprocess.run(
        ["git", "-C", str(root), "ls-files", "--error-unmatch", "--", producer_relative],
        text=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    ).returncode == 0
    return {
        "producer": {
            "path": producer_relative,
            "sha256": sha256_file(Path(__file__).resolve()),
            "git_tracking": "tracked" if tracked_producer else "untracked",
        },
        "command": "uv run --script experiments/sys-datascience/methods/canonical-vertex-covariance/analyze.py",
        "self_test_command": "uv run --script experiments/sys-datascience/methods/canonical-vertex-covariance/analyze.py --self-test",
        "source_revision": repo_command("rev-parse", "HEAD"),
        "source_input_status": repo_command("status", "--porcelain", "--", *relative_inputs),
        "inputs": [input_identity(path) for path in inputs],
        "reproducibility_boundary": "All four retained inputs must be hydrated rather than Git-LFS pointer files. If any is a pointer, the script stops before treating it as evidence.",
        "artifacts": {
            "per_polytope.jsonl_sha256": sha256_file(out_dir / "per_polytope.jsonl"),
            "report.json_sha256": sha256_file(out_dir / "report.json"),
        },
    }


def main() -> None:
    args = parse_args()
    run_self_tests()
    if args.self_test:
        print("canonical-vertex covariance self-tests passed")
        return
    inputs = [
        PRODUCE / "random.jsonl",
        PRODUCE / "random-product.jsonl",
        PREPARE / "polytope-table.jsonl",
        PREPARE / "polytope-provenance-table.jsonl",
    ]
    ids = source_to_poly_id()
    sys_by_id = prepared_sys()
    rows: list[dict[str, Any]] = []
    rejected: list[dict[str, str]] = []
    for kind, path in (("random", PRODUCE / "random.jsonl"), ("random_product", PRODUCE / "random-product.jsonl")):
        for raw in read_jsonl(path):
            name = raw.get("name")
            if not isinstance(name, str) or name not in ids or ids[name] not in sys_by_id:
                raise VertexContractError(f"unjoinable retained producer row {name!r}")
            if not isinstance(raw.get("sys"), int | float) or not np.isclose(
                float(raw["sys"]), sys_by_id[ids[name]], rtol=0.0, atol=1e-12
            ):
                raise VertexContractError(f"prepared/producer sys mismatch for {name!r}")
            try:
                metric = metric_from_rationals(raw.get("vertices_rational"))
            except VertexContractError as error:
                rejected.append({
                    "source_name": name,
                    "bucket": bucket_for(raw, kind),
                    "sys": sys_by_id[ids[name]],
                    "reason": str(error),
                })
                continue
            rows.append({
                "poly_id": ids[name], "source_name": name, "bucket": bucket_for(raw, kind),
                "sys": sys_by_id[ids[name]], **metric,
            })
    args.out_dir.mkdir(parents=True, exist_ok=True)
    with (args.out_dir / "per_polytope.jsonl").open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n")
    report = {
        "question": "Does canonical-extreme-vertex covariance symplectic eccentricity retain association with sys within retained sample bucket and vertex-count strata?",
        "definition": "C is the centered uniform covariance of canonical distinct primal extreme vertices; rho=nu2/nu1 for the ordered symplectic eigenvalues of C.",
        "input_contract": "Exact vertices_rational payloads; rationally equal duplicates are canonicalized; numerical full-dimensionality and convex-hull extremeness are verified per row.",
        "selection_protocol": "rho was selected in a post-target exploratory method-surface scan after retained sys values already existed. All reported associations reuse the full retained table; no threshold, split, or candidate rule was frozen before inspecting sys.",
        "eligibility": {
            "producer_rows": len(rows) + len(rejected),
            "accepted_rows": len(rows),
            "rejected_rows": len(rejected),
            "rejected_examples": rejected[:10],
            "numerical_condition_cutoff": 1e10,
        },
        "provenance_file": "provenance.json",
        "observation": summarize(rows),
        "inference_boundary": "These post-target same-table correlations are exploratory redundancy diagnostics, not frozen proposer evaluation, mechanism, or evidence that rho predicts sys outside the retained generators. Per-stratum Spearmans and pooled rank-residualized controls are descriptive controls only; neither establishes independence from other combinatorics.",
        "status": "computed on retained data only; no capacity or sys evaluations were run; contract-ineligible rows are excluded rather than silently assigned rho",
    }
    with (args.out_dir / "report.json").open("w") as handle:
        json.dump(report, handle, indent=2, sort_keys=True)
        handle.write("\n")
    with (args.out_dir / "provenance.json").open("w") as handle:
        json.dump(provenance(inputs, args.out_dir), handle, indent=2, sort_keys=True)
        handle.write("\n")
    print(json.dumps({"rows": len(rows), "out_dir": str(args.out_dir)}, sort_keys=True))


if __name__ == "__main__":
    main()
