#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "numpy==2.2.6",
#   "scipy==1.15.3",
#   "scikit-learn==1.6.1",
#   "ripser==0.6.12",
# ]
# ///
"""Calibrated topology and density-component diagnostics for factor clouds.

The input is the reviewed ``factor-shape-row-v1`` JSONL contract used by the
generator atlas.  Each fixed-side polygon is converted to a translation,
positive-scale, rotation, and cyclic-relabel quotient vector.  No side count
or population parameter is pooled.  Synthetic controls are intentionally
small and are used to decide whether the diagnostics survive obvious positive
and negative cases before touching the atlas rows.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.metadata
import json
import math
import platform
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Iterable

import numpy as np
from ripser import ripser
from scipy.spatial.distance import pdist, squareform
from sklearn.cluster import DBSCAN
from sklearn.neighbors import NearestNeighbors

SCHEMA = "factor-shape-row-v1"
REPORT_SCHEMA = "topology-density-report-v1"
RNG_SEED = 2026071501
MIN_STRATUM_N = 12
BOOTSTRAPS = 32
SCALE_FACTORS = (0.8, 1.0, 1.2)
UPSTREAM_ATLAS_COMMIT = "3f09eeeb"
UPSTREAM_GENERATOR_COMMIT = "a50e1e930e21541506df6228aaed16355d830372"
UPSTREAM_ATLAS_ROOT = Path(
    "/workspaces/msc-math/.worktrees/multiseed-atlas-confirmation/"
    "experiments/sys-datascience/methods/generator-distribution-atlas-confirmation/"
    "artifacts/raw"
)
SOURCE_FILES = (
    Path("experiments/sys-datascience/methods/topology-density/analyze.py"),
    Path("experiments/sys-datascience/methods/topology-density/README.md"),
    Path("experiments/sys-datascience/methods/topology-density/test_topology_density.py"),
)
EXPECTED_INPUT_HASHES = {
    "seed-20260716/core/factor-shapes.jsonl": "442a640629381cffdd215598521d05cc4ccc86c96d96be530a842572fe2d5cd1",
    "seed-20260716/core/factor-only-report.json": "384ba39bd1085f8cbf84a1ad1224d01efa66c728bc474ed78b98761b76638d48",
    "seed-20260716/zonogon/factor-shapes.jsonl": "6403831626b4a05781ee9e3d24320e72a9433b2b1fabf26eb1f1f986675a2c15",
    "seed-20260716/zonogon/factor-only-report.json": "ffb2a1c99149a72e75653a142a8c455f5d05df5c0f91ff7ecacbdbbcfcbc3c12",
    "seed-20260717/core/factor-shapes.jsonl": "2e22a87988c975b05f77eb20dd4cb6c466f088f8bee21fecb1561670e34b2ab0",
    "seed-20260717/core/factor-only-report.json": "b43f0bdd94785131a83940096a2b7f212bb5d689eebae6fe5d9536adc17b3b36",
    "seed-20260717/zonogon/factor-shapes.jsonl": "a78c71cd338e8bf4c7144935f25f40d538f8831cc2cdbfe82d2efb63214164d2",
    "seed-20260717/zonogon/factor-only-report.json": "4977d6456d4c3dd42156f804f14107f612fffaf704a2d6fae82445466906f8ba",
    "seed-20260718/core/factor-shapes.jsonl": "5aaf6b0a28abccac81df7a7a22cb74338555ba824209eef97f340fb181a36c72",
    "seed-20260718/core/factor-only-report.json": "f68777fe49a9f264d5b7142524dd366a445e35ff54c1702c1c2b9f5a22403d51",
    "seed-20260718/zonogon/factor-shapes.jsonl": "b653cd7a2776e130547f5e0d621b00857b6eef69306a8a5a468e31b871b65934",
    "seed-20260718/zonogon/factor-only-report.json": "1dcc4386cc3e989a16fba6a446ac6b319e4bcf10959409d8b31bc56e41a5abde",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def git_revision() -> str | None:
    try:
        return subprocess.run(
            ["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def git_source_revision() -> str | None:
    try:
        return subprocess.run(
            ["git", "log", "-1", "--format=%H", "--", *map(str, SOURCE_FILES)],
            capture_output=True,
            text=True,
            check=True,
            cwd=repo_root(),
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def git_source_tree(revision: str | None) -> str | None:
    if not revision:
        return None
    try:
        return subprocess.run(
            ["git", "rev-parse", f"{revision}^{{tree}}"],
            capture_output=True,
            text=True,
            check=True,
            cwd=repo_root(),
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def repo_root() -> Path:
    try:
        value = subprocess.run(
            ["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError) as exc:
        raise RuntimeError("cannot locate repository root") from exc
    return Path(value)


def git_source_dirty() -> bool:
    try:
        result = subprocess.run(
            ["git", "status", "--porcelain", "--", *map(str, SOURCE_FILES)],
            capture_output=True,
            text=True,
            check=True,
            cwd=repo_root(),
        )
    except (OSError, subprocess.CalledProcessError) as exc:
        raise RuntimeError("cannot establish clean source surface") from exc
    return bool(result.stdout.strip())


def source_manifest() -> dict[str, str]:
    if git_source_dirty():
        raise RuntimeError("declared analyzer/README/test source surface is dirty")
    root = repo_root()
    missing = [str(path) for path in SOURCE_FILES if not (root / path).is_file()]
    if missing:
        raise RuntimeError(f"declared source files are missing: {missing}")
    return {str(path): sha256(root / path) for path in SOURCE_FILES}


def input_manifest(paths: Iterable[Path]) -> list[dict[str, Any]]:
    packet_dir = Path(__file__).resolve().parent
    entries: list[dict[str, Any]] = []
    seen: set[str] = set()
    for path in paths:
        resolved = path.resolve()
        try:
            key = resolved.relative_to(packet_dir / "artifacts/input").as_posix()
        except ValueError as exc:
            raise RuntimeError(f"input must be under packet artifacts/input: {path}") from exc
        if key in seen or key not in EXPECTED_INPUT_HASHES:
            raise RuntimeError(f"input is outside the declared six-file surface: {path}")
        seen.add(key)
        observed = sha256(resolved)
        expected = EXPECTED_INPUT_HASHES[key]
        if observed != expected:
            raise RuntimeError(f"input hash mismatch for {key}: expected {expected}, got {observed}")
        owner = UPSTREAM_ATLAS_ROOT / key
        owner_available = owner.is_file()
        owner_hash = sha256(owner) if owner_available else None
        if owner_available and owner_hash != expected:
            raise RuntimeError(f"copied input disagrees with owner source {owner}: expected {expected}, got {owner_hash}")
        entries.append(
            {
                "relative_path": key,
                "sha256": observed,
                "expected_sha256": expected,
                "owner_source_path": str(owner),
                "owner_source_available": owner_available,
                "owner_source_sha256": owner_hash,
            }
        )
    if not entries:
        return []
    expected_shapes = {key for key in EXPECTED_INPUT_HASHES if key.endswith("factor-shapes.jsonl")}
    if seen != expected_shapes:
        raise RuntimeError(f"real run must use exactly the six declared shape files, got {sorted(seen)}")
    for entry in entries:
        if not entry["relative_path"].endswith("factor-only-report.json"):
            report_path = packet_dir / "artifacts/input" / Path(entry["relative_path"]).parent / "factor-only-report.json"
            if not report_path.is_file():
                raise RuntimeError(f"missing corresponding factor-only report for {entry['relative_path']}")
            report_key = report_path.relative_to(packet_dir / "artifacts/input").as_posix()
            report_hash = sha256(report_path)
            report_expected = EXPECTED_INPUT_HASHES[report_key]
            if report_hash != report_expected:
                raise RuntimeError(f"upstream report hash mismatch for {report_key}: expected {report_expected}, got {report_hash}")
            owner_report = UPSTREAM_ATLAS_ROOT / report_key
            owner_report_available = owner_report.is_file()
            owner_report_hash = sha256(owner_report) if owner_report_available else None
            if owner_report_available and owner_report_hash != report_expected:
                raise RuntimeError(f"copied report disagrees with owner source {owner_report}: expected {report_expected}, got {owner_report_hash}")
            report = _validate_upstream_report(report_path)
            entry["factor_only_report"] = {
                "relative_path": report_key,
                "sha256": report_hash,
                "expected_sha256": report_expected,
                "owner_source_path": str(owner_report),
                "owner_source_available": owner_report_available,
                "owner_source_sha256": owner_report_hash,
                "schema": report["schema"],
                "source_revision": report["source_revision"],
                "source_dirty": report["source_dirty"],
                "factor_rows": report.get("factor_rows"),
            }
    return entries


def _validate_upstream_report(report_path: Path) -> dict[str, Any]:
    try:
        report = json.loads(report_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise ValueError(f"cannot read upstream report {report_path}") from exc
    if report.get("schema") != "generator-zoo-factor-only-report-v1":
        raise ValueError(f"unexpected upstream report schema in {report_path}")
    if report.get("source_dirty") is not False or report.get("source_revision") != UPSTREAM_GENERATOR_COMMIT:
        raise ValueError(f"upstream report provenance is not clean in {report_path}")
    return report


def _canonical_vector(vertices: np.ndarray) -> np.ndarray:
    """Return a fixed-side quotient vector, retaining orientation.

    Translation is removed by the centroid, scale by RMS radius, rotation by
    making the first vertex real and positive, and cyclic relabelling by the
    lexicographically least cyclic representative.  Reflection is *not*
    quotiented.  A deterministic tie break is important for duplicate rows.
    """
    _validate_polygon(vertices)
    z = vertices[:, 0] + 1j * vertices[:, 1]
    z = z - np.mean(z)
    norm = float(np.sqrt(np.mean(np.abs(z) ** 2)))
    if not math.isfinite(norm) or norm <= 1e-14:
        raise ValueError("degenerate centered polygon")
    z = z / norm
    candidates: list[tuple[float, ...]] = []
    for shift in range(len(z)):
        w = np.roll(z, -shift)
        pivot = w[0]
        if abs(pivot) <= 1e-14:
            # A rare symmetric tie: use the first nonzero pivot.
            pivot = w[np.flatnonzero(np.abs(w) > 1e-14)[0]]
        w = w * np.exp(-1j * np.angle(pivot))
        candidates.append(tuple(float(x) for pair in zip(w.real, w.imag) for x in pair))
    return np.asarray(min(candidates), dtype=np.float64)


def _validate_polygon(vertices: np.ndarray) -> None:
    """Require a finite, nondegenerate, strictly convex CCW polygon."""
    if vertices.ndim != 2 or vertices.shape[1] != 2 or vertices.shape[0] < 3:
        raise ValueError("vertices must have shape (n,2), n>=3")
    if not np.isfinite(vertices).all():
        raise ValueError("polygon vertices must be finite")
    edges = np.roll(vertices, -1, axis=0) - vertices
    twice_area = float(np.sum(vertices[:, 0] * np.roll(vertices[:, 1], -1) - vertices[:, 1] * np.roll(vertices[:, 0], -1)))
    scale = max(1.0, float(np.max(np.abs(vertices))) ** 2)
    tolerance = 1e-12 * scale
    if twice_area <= tolerance:
        raise ValueError("polygon must be CCW with positive area")
    crosses = edges[:, 0] * np.roll(edges[:, 1], -1) - edges[:, 1] * np.roll(edges[:, 0], -1)
    if np.any(crosses <= tolerance):
        raise ValueError("polygon must be strictly convex in cyclic CCW order")


def _radial_vector(vertices: np.ndarray) -> np.ndarray:
    """A second, rotation-free view used only as a stability check."""
    z = vertices[:, 0] + 1j * vertices[:, 1]
    z -= np.mean(z)
    norm = float(np.sqrt(np.mean(np.abs(z) ** 2)))
    if norm <= 1e-14 or not math.isfinite(norm):
        raise ValueError("degenerate centered polygon")
    # Sorting radii loses cyclic adjacency, so this is deliberately a
    # diagnostic view and never the primary shape object.
    return np.sort(np.abs(z / norm)).astype(np.float64)


def _load_rows(paths: Iterable[Path]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    seen: set[str] = set()
    for path in paths:
        with path.open(encoding="utf-8") as handle:
            for line_number, line in enumerate(handle, 1):
                if not line.strip():
                    continue
                try:
                    row = json.loads(line)
                except json.JSONDecodeError as exc:
                    raise ValueError(f"{path}:{line_number}: invalid JSON: {exc}") from exc
                if row.get("schema") != SCHEMA:
                    raise ValueError(f"{path}:{line_number}: schema must be {SCHEMA!r}")
                sample_id = row.get("sample_id")
                if not isinstance(sample_id, str) or not sample_id or sample_id in seen:
                    raise ValueError(f"{path}:{line_number}: sample_id must be globally unique")
                seen.add(sample_id)
                side = row.get("side_count")
                vertices = np.asarray(row.get("vertices_ccw"), dtype=float)
                if isinstance(side, bool) or not isinstance(side, int) or side < 3 or vertices.shape != (side, 2):
                    raise ValueError(f"{path}:{line_number}: side_count/vertices mismatch")
                try:
                    _validate_polygon(vertices)
                except ValueError as exc:
                    raise ValueError(f"{path}:{line_number}: {exc}") from exc
                row["_canonical"] = _canonical_vector(vertices).tolist()
                row["_radial"] = _radial_vector(vertices).tolist()
                rows.append(row)
    if not rows:
        raise ValueError("no rows")
    return rows


def _distance_scale(x: np.ndarray) -> float:
    d = pdist(x)
    positive = d[d > 1e-12]
    return float(np.median(positive)) if len(positive) else 1.0


def _density_summary(x: np.ndarray, rng: np.random.Generator) -> dict[str, Any]:
    n = len(x)
    if n < 4:
        return {"status": "underpowered", "n": n}
    k = min(8, n - 1)
    nn = NearestNeighbors(n_neighbors=k + 1).fit(x)
    kth = nn.kneighbors(x, return_distance=True)[0][:, -1]
    base = float(np.median(kth))
    if base <= 1e-12:
        positive = kth[kth > 1e-12]
        base = float(np.median(positive)) if len(positive) else 1.0
    eps_grid = [base * factor for factor in SCALE_FACTORS]
    rows: list[dict[str, Any]] = []
    for scale_factor, eps in zip(SCALE_FACTORS, eps_grid):
        labels = DBSCAN(eps=eps, min_samples=k).fit_predict(x)
        components = len(set(labels)) - (1 if -1 in labels else 0)
        noise_fraction = float(np.mean(labels == -1))
        rows.append({"scale_factor": scale_factor, "eps": eps, "components": components, "noise_fraction": noise_fraction})
    unique_rows = int(len(np.unique(x, axis=0)))
    boot_counts: list[list[int]] = [[] for _ in eps_grid]
    bootstrap_unique_counts: list[int] = []
    for _ in range(BOOTSTRAPS):
        indices = rng.integers(0, n, size=n)
        # Keep repeated indices.  DBSCAN's min_samples then sees the actual
        # bootstrap multiplicity instead of an undocumented ~0.632n
        # without-replacement subsample.
        sample = x[indices]
        bootstrap_unique_counts.append(int(len(np.unique(indices))))
        for j, eps in enumerate(eps_grid):
            labels = DBSCAN(eps=eps, min_samples=k).fit_predict(sample)
            boot_counts[j].append(len(set(labels)) - (1 if -1 in labels else 0))
    for row, counts in zip(rows, boot_counts):
        row["bootstrap_n"] = len(counts)
        row["bootstrap_mode_components"] = int(Counter(counts).most_common(1)[0][0]) if counts else None
        row["bootstrap_mode_fraction"] = (max(Counter(counts).values()) / len(counts)) if counts else None
    return {
        "status": "ok",
        "k": k,
        "base_kth_distance": base,
        "input_unique_rows": unique_rows,
        "input_duplicate_fraction": float(1.0 - unique_rows / n),
        "resampling": "bootstrap_with_replacement_multiplicities_retained",
        "bootstrap_sample_size": n,
        "bootstrap_unique_rows_mean": float(np.mean(bootstrap_unique_counts)),
        "scales": rows,
    }


def _topology_summary(x: np.ndarray) -> dict[str, Any]:
    n = len(x)
    if n < 5:
        return {"status": "underpowered", "n": n}
    scale = _distance_scale(x)
    result = ripser(x, maxdim=1, thresh=3.0 * scale, metric="euclidean")
    diagrams = result["dgms"]
    summary: dict[str, Any] = {"status": "ok", "n": n, "distance_scale": scale}
    for dim in (0, 1):
        bars = diagrams[dim]
        persistence = bars[:, 1] - bars[:, 0] if len(bars) else np.empty(0)
        persistence = persistence[np.isfinite(persistence)]
        # The H1 threshold is deliberately more conservative than H0: finite
        # random samples from a filled disk create many short spurious loops,
        # while the circle and boundary controls retain a substantially longer
        # bar.  This is a calibration knob, not a theorem-level persistence
        # cutoff.
        threshold = 0.35 * scale if dim == 1 else 0.2 * scale
        summary[f"h{dim}_bars"] = int(len(bars))
        summary[f"h{dim}_significant_bars"] = int(np.sum(persistence > threshold))
        summary[f"h{dim}_max_persistence"] = float(np.max(persistence)) if len(persistence) else 0.0
        summary[f"h{dim}_threshold"] = threshold
    return summary


def summarize_cloud(x: np.ndarray, rng: np.random.Generator) -> dict[str, Any]:
    return {"n": len(x), "dimension": int(x.shape[1]), "topology": _topology_summary(x), "density": _density_summary(x, rng)}


def _synthetic_clouds() -> dict[str, tuple[np.ndarray, dict[str, Any]]]:
    rng = np.random.default_rng(RNG_SEED)
    t = np.linspace(0, 2 * np.pi, 160, endpoint=False)
    circle = np.column_stack((np.cos(t), np.sin(t))) + rng.normal(0, 0.025, (len(t), 2))
    u = rng.random(180)
    angle = rng.uniform(0, 2 * np.pi, len(u))
    disk = np.column_stack((np.sqrt(u) * np.cos(angle), np.sqrt(u) * np.sin(angle)))
    separated = np.vstack((rng.normal((-2.0, 0.0), 0.18, (90, 2)), rng.normal((2.0, 0.0), 0.18, (90, 2))))
    left = rng.normal((-1.8, 0.0), 0.28, (65, 2)); right = rng.normal((1.8, 0.0), 0.28, (65, 2)); bridge = np.column_stack((rng.uniform(-1.7, 1.7, 40), rng.normal(0, 0.08, 40)))
    dumbbell = np.vstack((left, right, bridge))
    duplicate_base = np.vstack((rng.normal((-1.5, 0), 0.2, (24, 2)), rng.normal((1.5, 0), 0.2, (24, 2))))
    duplicates = np.repeat(duplicate_base, 3, axis=0)
    anisotropic_line = np.column_stack((rng.uniform(-3, 3, 180), rng.normal(0, 0.025, 180)))
    boundary = np.vstack((
        np.column_stack((rng.uniform(-2, 2, 60), np.full(60, -2.0) + rng.normal(0, .03, 60))),
        np.column_stack((rng.uniform(-2, 2, 60), np.full(60, 2.0) + rng.normal(0, .03, 60))),
        np.column_stack((np.full(30, -2.0) + rng.normal(0, .03, 30), rng.uniform(-2, 2, 30))),
        np.column_stack((np.full(30, 2.0) + rng.normal(0, .03, 30), rng.uniform(-2, 2, 30))),
        rng.normal(0, .35, (12, 2)),
    ))
    return {
        "circle": (circle, {"h1": True}),
        "disk": (disk, {"h1": False}),
        "separated_mixture": (separated, {"components": 2}),
        "narrow_bridge_dumbbell": (dumbbell, {"scale_change": True}),
        "duplicates": (duplicates, {"duplicates": True}),
        "anisotropic_line": (anisotropic_line, {"h1": False, "line": True}),
        "boundary_heavy": (boundary, {"boundary": True}),
    }


def calibrate(rng: np.random.Generator) -> dict[str, Any]:
    results: dict[str, Any] = {}
    for name, (cloud, expected) in _synthetic_clouds().items():
        result = summarize_cloud(cloud, rng)
        topo = result["topology"]
        density = result["density"]
        scales = density.get("scales", [])
        passed = False
        reason = ""
        if name == "circle":
            passed = topo.get("h1_significant_bars", 0) >= 1
            reason = "H1 must retain a persistent loop"
        elif name == "disk":
            passed = topo.get("h1_significant_bars", 0) == 0
            reason = "filled disk must not retain a significant H1 loop"
        elif name == "separated_mixture":
            modes = [s.get("bootstrap_mode_components") for s in scales]
            passed = 2 in modes and max(s.get("bootstrap_mode_fraction", 0) or 0 for s in scales) >= 0.6
            reason = "a stable two-component density level must be visible"
        elif name == "narrow_bridge_dumbbell":
            counts = [s["components"] for s in scales]
            passed = len(set(counts)) >= 2
            reason = "component count should change with the declared neighbourhood scale"
        elif name == "duplicates":
            passed = (
                density.get("input_duplicate_fraction", 0.0) > 0.5
                and topo.get("h0_significant_bars", 0) <= 2
            )
            reason = "duplicate multiplicity must remain visible without creating extra significant H0 components"
        elif name == "anisotropic_line":
            passed = topo.get("h1_significant_bars", 0) == 0 and float(np.std(cloud[:, 0]) / np.std(cloud[:, 1])) > 20
            reason = "a thin line should have no loop and an obvious anisotropy diagnostic"
        elif name == "boundary_heavy":
            passed = topo.get("h1_significant_bars", 0) >= 1
            reason = "boundary-heavy square should expose a scale-sensitive hole"
        results[name] = {"expected": expected, "observed": result, "pass": bool(passed), "reason": reason}
    passed_count = sum(item["pass"] for item in results.values())
    return {"schema": "topology-density-calibration-v1", "controls": results, "pass_count": passed_count, "control_count": len(results), "method_selected": passed_count >= 5, "selection_rule": "at least 5 of 7 predeclared control signatures pass"}


def analyze_real(rows: list[dict[str, Any]], rng: np.random.Generator) -> dict[str, Any]:
    groups: dict[tuple[str, str, int], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        groups[(row["law"], row.get("population", row["law"]), row["side_count"])].append(row)
    strata = []
    for (law, population, side_count), group in sorted(groups.items()):
        seeds = sorted({int(r.get("seed")) for r in group if r.get("seed") is not None})
        item: dict[str, Any] = {"law": law, "population": population, "side_count": side_count, "n": len(group), "seed_count": len(seeds), "seeds": seeds}
        if len(group) < MIN_STRATUM_N or len(seeds) < 2:
            item["status"] = "underpowered"
            item["reason"] = f"requires n>={MIN_STRATUM_N} and at least two independent seeds"
            strata.append(item)
            continue
        views = {}
        for view in ("canonical", "radial"):
            x = np.asarray([r[f"_{view}"] for r in group], dtype=float)
            views[view] = summarize_cloud(x, rng)
        item["status"] = "descriptive"
        item["views"] = views
        item["interpretation"] = "finite-sample descriptive shape-cloud diagnostic; no law, support, target, or population claim"
        strata.append(item)
    return {"schema": "topology-density-real-v1", "strata": strata, "stratum_count": len(strata), "allowed": ["compare observed component/loop summaries within an explicitly named law/population/side stratum", "flag scale and view instability"], "prohibited": ["pool side counts or law knobs", "infer population topology or natural-law support", "use topology as a sys or capacity predictor", "claim target transfer or mechanism"]}


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def write_synthetic_fixture(path: Path) -> None:
    rows = []
    for name, (cloud, _) in _synthetic_clouds().items():
        rows.extend({"schema": "point-cloud-v1", "cloud": name, "index": i, "x": float(x), "y": float(y)} for i, (x, y) in enumerate(cloud))
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("".join(json.dumps(row, sort_keys=True) + "\n" for row in rows), encoding="utf-8")


def canonical_command(argv: list[str]) -> str:
    """Keep replay provenance independent of the chosen output directory."""
    normalized: list[str] = []
    i = 0
    while i < len(argv):
        token = argv[i]
        if token in {"--out-dir", "--write-synthetic-fixture"} and i + 1 < len(argv):
            normalized.extend((token, "<OUTPUT>" if token == "--out-dir" else "<FIXTURE>"))
            i += 2
            continue
        normalized.append(token)
        i += 1
    return " ".join(normalized)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", action="append", type=Path, default=[])
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--write-synthetic-fixture", type=Path)
    parser.add_argument("--calibration-only", action="store_true")
    args = parser.parse_args()
    if args.write_synthetic_fixture:
        write_synthetic_fixture(args.write_synthetic_fixture)
    args.out_dir.mkdir(parents=True, exist_ok=True)
    source_hashes = source_manifest()
    input_hashes = input_manifest(args.input)
    rng = np.random.default_rng(RNG_SEED)
    calibration = calibrate(rng)
    write_json(args.out_dir / "calibration.json", calibration)
    if not args.calibration_only and args.input:
        rows = _load_rows(args.input)
        real = analyze_real(rows, rng)
        write_json(args.out_dir / "real.json", real)
    elif not args.calibration_only and not args.input:
        raise SystemExit("provide --input or --calibration-only")
    source_revision = git_source_revision()
    provenance = {
        "schema": REPORT_SCHEMA,
        "command": canonical_command(sys.argv),
        "python": sys.version,
        "platform": platform.platform(),
        "packages": {name: importlib.metadata.version(name) for name in ("numpy", "scipy", "scikit-learn", "ripser")},
        "git_revision": source_revision,
        "git_tree": git_source_tree(source_revision),
        "source_dirty": False,
        "source_files": source_hashes,
        "input_manifest": input_hashes,
        "calibration_sha256": sha256(args.out_dir / "calibration.json"),
        "contract": {"shape_view": "centroided/RMS-scaled/cyclic-relabel-minimized/rotation-aligned; orientation retained", "side_stratified": True, "law_knobs_not_pooled": True, "bootstrap_count": BOOTSTRAPS, "scale_factors": SCALE_FACTORS},
        "upstream": {
            "accepted_atlas_commit": UPSTREAM_ATLAS_COMMIT,
            "generator_source_commit": UPSTREAM_GENERATOR_COMMIT,
            "owner_source_root": str(UPSTREAM_ATLAS_ROOT),
            "copied_input_contract": "all six factor-shapes files and corresponding generator-zoo-factor-only-report-v1 reports must match the embedded hashes; owner bytes are checked when available",
        },
        "interpretation": "Calibration is a feasibility gate. Real rows are descriptive finite-sample diagnostics only.",
    }
    if (args.out_dir / "real.json").exists():
        provenance["real_sha256"] = sha256(args.out_dir / "real.json")
    write_json(args.out_dir / "report.json", provenance)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
