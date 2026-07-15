#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""Target-free combinatorial occupancy and coverage diagnostics.

The adapter accepts either explicit ``vertex_facet_incidence`` matrices or the
retained rational ``vertices_rational``/``dual_vertices_rational`` records.
Incidence is reconstructed with :class:`fractions.Fraction`, so floating-point
near-equality never decides a combinatorial edge.  Canonical labels are exact
only when the bounded canonical search completes; capped labels are retained
as diagnostics and are never used as exact type evidence.
"""

from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from fractions import Fraction
import hashlib
import itertools
import json
import math
import subprocess
from pathlib import Path
from typing import Any

SEED = 20260715
ANALYZER_REPO_PATH = "experiments/sys-datascience/methods/generator-combinatorial-coverage/analyze.py"
SCHEMAS = {"combinatorial-row-v1", "factor-shape-row-v1"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def stable_key(seed: int, *parts: Any) -> bytes:
    payload = json.dumps([seed, *parts], sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(payload).digest()


def parse_exact(x: Any) -> Fraction:
    if isinstance(x, Fraction):
        return x
    if isinstance(x, int):
        return Fraction(x)
    if isinstance(x, float):
        return Fraction(str(x))
    if isinstance(x, str):
        return Fraction(x)
    raise ValueError(f"unsupported exact coordinate {x!r}")


def exact_matrix(row: dict[str, Any]) -> list[list[int]] | None:
    supplied = row.get("vertex_facet_incidence")
    if supplied is not None:
        matrix = [[int(bool(x)) for x in r] for r in supplied]
        if not matrix or not matrix[0] or any(len(r) != len(matrix[0]) for r in matrix):
            raise ValueError("vertex_facet_incidence must be a nonempty rectangular matrix")
        return matrix
    vertices = row.get("vertices_rational")
    facets = row.get("dual_vertices_rational")
    if vertices is None or facets is None:
        return None
    vv = [[parse_exact(x) for x in p] for p in vertices]
    ff = [[parse_exact(x) for x in p] for p in facets]
    if not vv or not ff or any(len(v) != len(vv[0]) for v in vv) or any(len(f) != len(vv[0]) for f in ff):
        raise ValueError("rational vertices and dual vertices have incompatible dimensions")
    # Producer convention is a_k^T x = 1 for an incident vertex.
    return [[int(sum(a * x for a, x in zip(f, v)) == 1) for f in ff] for v in vv]


def float_vertices(row: dict[str, Any]) -> list[list[float]] | None:
    raw = row.get("vertices") or row.get("vertices_ccw") or row.get("vertices_rational")
    if raw is None:
        return None
    return [[float(parse_exact(x)) for x in p] for p in raw]


def refine(partition: list[tuple[int, tuple[int, ...]]], matrix: list[list[int]]) -> list[tuple[int, tuple[int, ...]]]:
    """Color-preserving equitable refinement of a bipartite incidence graph."""
    n_v, n_f = len(matrix), len(matrix[0])
    cells = [list(cell) for _, cell in partition]
    colors = [c for c, _ in partition]
    while True:
        signatures: list[tuple[Any, ...]] = []
        for color, cell in zip(colors, cells):
            for i in cell:
                if color == 0:
                    counts = tuple(sum(matrix[i][j] for j in target) for target, tcolor in zip(cells, colors) if tcolor == 1)
                else:
                    counts = tuple(sum(matrix[j][i] for j in target) for target, tcolor in zip(cells, colors) if tcolor == 0)
                signatures.append((color, i, counts))
        new_cells: list[list[int]] = []
        new_colors: list[int] = []
        k = 0
        for cell, color in zip(cells, colors):
            groups: dict[tuple[Any, ...], list[int]] = defaultdict(list)
            for i in cell:
                sig = next(s[2] for s in signatures if s[0] == color and s[1] == i)
                groups[sig].append(i)
            for sig in sorted(groups, key=lambda x: repr(x)):
                new_cells.append(sorted(groups[sig])); new_colors.append(color); k += 1
        if new_cells == cells and new_colors == colors:
            return [(c, tuple(cell)) for c, cell in zip(colors, cells)]
        cells, colors = new_cells, new_colors


def canonical_incidence(matrix: list[list[int]], node_cap: int = 20000) -> tuple[str | None, str, int]:
    """Return an exact color-preserving canonical code or a capped status.

    The search enumerates all individualizations required by equitable
    refinement.  ``node_cap`` counts search nodes; hitting it returns no exact
    code (fail closed), while the caller can still retain a WL-style summary.
    """
    n_v, n_f = len(matrix), len(matrix[0])
    if n_v == 0 or n_f == 0:
        return None, "invalid", 0
    nodes = 0
    best: str | None = None

    def search(partition: list[tuple[int, tuple[int, ...]]]) -> None:
        nonlocal nodes, best
        nodes += 1
        if nodes > node_cap:
            return
        stable = refine(partition, matrix)
        choices = [i for i, (_, cell) in enumerate(stable) if len(cell) > 1]
        if not choices:
            v_order = [i for c, cell in stable if c == 0 for i in cell]
            f_order = [i for c, cell in stable if c == 1 for i in cell]
            code = f"{n_v}x{n_f}:" + ";".join("".join(str(matrix[i][j]) for j in f_order) for i in v_order)
            if best is None or code < best:
                best = code
            return
        # Largest cell first reduces branching for regular inputs.  Ties use
        # color and position, making the traversal independent of input labels.
        idx = max(choices, key=lambda i: (len(stable[i][1]), -stable[i][0], -i))
        color, cell = stable[idx]
        for chosen in cell:
            rest = tuple(x for x in cell if x != chosen)
            branch = list(stable[:idx]) + [(color, (chosen,))]
            if rest:
                branch.append((color, rest))
            branch.extend(stable[idx + 1:])
            search(branch)

    search([(0, tuple(range(n_v))), (1, tuple(range(n_f)))])
    if nodes > node_cap:
        return None, "capped", nodes
    return best, "exact", nodes


def wl_summary(matrix: list[list[int]], rounds: int = 4) -> str:
    labels = [f"v:{sum(r)}" for r in matrix] + [f"f:{sum(matrix[i][j] for i in range(len(matrix)))}" for j in range(len(matrix[0]))]
    n_v = len(matrix)
    for _ in range(rounds):
        nxt = []
        for i in range(len(labels)):
            if i < n_v:
                neigh = [labels[n_v + j] for j, x in enumerate(matrix[i]) if x]
            else:
                j = i - n_v; neigh = [labels[k] for k in range(n_v) if matrix[k][j]]
            nxt.append(hashlib.sha256((labels[i] + "|" + ",".join(sorted(neigh))).encode()).hexdigest()[:16])
        labels = nxt
    return hashlib.sha256("|".join(sorted(labels[:n_v])) .encode()).hexdigest()


def geometry_summary(vertices: list[list[float]] | None) -> dict[str, float | None]:
    if not vertices or len(vertices) < 2:
        return {"mean_pair_distance": None, "max_pair_distance": None}
    dists = []
    for a, b in itertools.combinations(vertices, 2):
        dists.append(math.sqrt(sum((x - y) ** 2 for x, y in zip(a, b))))
    scale = math.sqrt(sum(x * x for p in vertices for x in p) / len(vertices)) or 1.0
    return {"mean_pair_distance": sum(dists) / len(dists) / scale, "max_pair_distance": max(dists) / scale}


def normalize_row(raw: dict[str, Any], path: Path, line: int, cap: int) -> dict[str, Any]:
    matrix = exact_matrix(raw)
    vertices = float_vertices(raw)
    if matrix is None:
        raise ValueError(f"{path}:{line}: no explicit or rational incidence")
    if any(x not in (0, 1) for r in matrix for x in r):
        raise ValueError(f"{path}:{line}: incidence entries must be binary")
    identity, status, nodes = canonical_incidence(matrix, cap)
    incidence_code = json.dumps(matrix, separators=(",", ":"), sort_keys=True)
    population = raw.get("population", raw.get("law"))
    if not population:
        if raw.get("k") is not None and raw.get("m") is not None:
            population = f"product[{raw['k']}x{raw['m']}]"
        elif raw.get("name", "").startswith("random_"):
            population = "generic-random"
        else:
            population = raw.get("source", {}).get("family", "unknown")
    return {
        "sample_id": str(raw.get("sample_id", raw.get("name", f"{path.name}:{line}"))),
        "law": str(population),
        "facet_count": int(raw.get("facet_count", len(matrix[0]))),
        "vertex_count": len(matrix),
        "matrix": matrix,
        "exact_type": identity,
        "canonical_status": status,
        "wl_summary": wl_summary(matrix),
        "canonical_search_nodes": nodes,
        "vertices": vertices,
        "geometry": geometry_summary(vertices),
        "independence_group": str(raw.get("independence_group", raw.get("root_group_id", raw.get("sample_id", raw.get("name", line))))),
        "source_path": str(path),
        "source_line": line,
        "source_row": raw,
        "incidence_digest": hashlib.sha256(incidence_code.encode()).hexdigest(),
    }


def load_rows(paths: list[Path], cap: int, max_rows: int | None, facet_counts: set[int] | None = None) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    rows, rejects = [], []
    for path in paths:
        with path.open() as handle:
            for line, text in enumerate(handle, 1):
                if max_rows is not None and sum(r["source_path"] == str(path) for r in rows) >= max_rows:
                    break
                if not text.strip():
                    continue
                try:
                    raw = json.loads(text)
                    if facet_counts is not None and int(raw.get("facet_count", -1)) not in facet_counts:
                        continue
                    rows.append(normalize_row(raw, path, line, cap))
                except (ValueError, TypeError, KeyError, json.JSONDecodeError) as exc:
                    rejects.append({"path": str(path), "line": line, "reason": str(exc)})
    if not rows:
        raise ValueError("no usable rows; exact rational incidence is required")
    return rows, rejects


def exact_occupancy(members: list[dict[str, Any]], seed: int, rarefaction: int = 16) -> dict[str, Any]:
    exact = [r for r in members if r["canonical_status"] == "exact" and r["exact_type"] is not None]
    counts = Counter(r["exact_type"] for r in exact)
    n = len(exact)
    singleton = sum(c == 1 for c in counts.values()); doubleton = sum(c == 2 for c in counts.values())
    probs = [c / n for c in counts.values()] if n else []
    entropy = -sum(p * math.log(p) for p in probs) if probs else None
    collision = sum(p * p for p in probs) if probs else None
    order = sorted(exact, key=lambda r: stable_key(seed, r["sample_id"]))
    curve = []
    seen: set[str] = set()
    for k, row in enumerate(order, 1):
        seen.add(row["exact_type"])
        if k <= rarefaction or k in {n, max(1, n // 2), max(1, 3 * n // 4)}:
            curve.append({"prefix": k, "distinct_exact_types": len(seen), "new_type": int(row["exact_type"] not in {x["exact_type"] for x in order[:k-1]})})
    group_types: dict[str, set[str]] = defaultdict(set)
    for r in exact:
        group_types[r["independence_group"]].add(r["exact_type"])
    independent_types = set().union(*group_types.values()) if group_types else set()
    return {
        "rows_total": len(members), "rows_exact": n, "rows_capped": sum(r["canonical_status"] == "capped" for r in members),
        "distinct_exact_types": len(counts), "singleton_types": singleton, "doubleton_types": doubleton,
        "plugin_entropy_nats": entropy, "effective_number": math.exp(entropy) if entropy is not None else None,
        "collision_probability_observed": collision,
        "good_turing_unseen_mass_diagnostic": singleton / n if n else None,
        "good_turing_warning": "diagnostic only; small-n and dependence make it unsuitable as support proof",
        "independence_groups": len(group_types), "distinct_types_after_group_dedup": len(independent_types),
        "rarefaction_discovery": curve,
        "canonical_status_counts": dict(Counter(r["canonical_status"] for r in members)),
    }


def between(laws: dict[str, list[dict[str, Any]]], facet_count: int, budget: int, seed: int) -> list[dict[str, Any]]:
    out = []
    names = sorted(laws)
    for left, right in itertools.combinations(names, 2):
        a = [r for r in laws[left] if r["facet_count"] == facet_count and r["canonical_status"] == "exact"]
        b = [r for r in laws[right] if r["facet_count"] == facet_count and r["canonical_status"] == "exact"]
        pair_budget = min(len(a), len(b), budget) if budget else min(len(a), len(b))
        a = sorted(a, key=lambda r: stable_key(seed, left, r["sample_id"]))[:pair_budget]
        b = sorted(b, key=lambda r: stable_key(seed, right, r["sample_id"]))[:pair_budget]
        sa, sb = {r["exact_type"] for r in a}, {r["exact_type"] for r in b}
        if not a or not b:
            continue
        out.append({"facet_count": facet_count, "left_law": left, "right_law": right, "balanced_budget": min(len(a), len(b)),
                    "shared_exact_types": len(sa & sb), "left_types": len(sa), "right_types": len(sb),
                    "directed_mass_left_covered_by_right": sum(r["exact_type"] in sb for r in a) / len(a) if a else None,
                    "directed_mass_right_covered_by_left": sum(r["exact_type"] in sa for r in b) / len(b) if b else None,
                    "interpretation": "observed-panel coverage only; correlated/fixed rows are not independent draws"})
    return out


def incremental_yield(laws: dict[str, list[dict[str, Any]]], facet_count: int, seed: int) -> dict[str, Any] | None:
    """Stable balanced-budget discovery order; no law is treated as random."""
    panels = {
        law: sorted((r for r in members if r["facet_count"] == facet_count and r["canonical_status"] == "exact"),
                    key=lambda r: stable_key(seed, law, r["sample_id"]))
        for law, members in laws.items()
    }
    panels = {law: rows for law, rows in panels.items() if rows}
    if len(panels) < 2:
        return None
    budget = min(len(rows) for rows in panels.values())
    seen: set[str] = set(); rows = []
    for law in sorted(panels):
        selected = panels[law][:budget]
        before = len(seen)
        seen.update(r["exact_type"] for r in selected)
        rows.append({"law": law, "balanced_budget": budget, "new_exact_types": len(seen) - before,
                     "cumulative_exact_types": len(seen)})
    return {"balanced_budget": budget, "law_order": sorted(panels), "steps": rows,
            "interpretation": "order-sensitive observed-panel yield; not a generator ranking or support estimate"}


def geometry_by_type(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    groups: dict[tuple[str, int], list[dict[str, Any]]] = defaultdict(list)
    for r in rows:
        if r["canonical_status"] == "exact": groups[(r["law"], r["facet_count"])].append(r)
    out = []
    for (law, f), members in sorted(groups.items()):
        for typ, same in sorted(itertools.groupby(sorted(members, key=lambda x: x["exact_type"]), key=lambda x: x["exact_type"]), key=lambda x: x[0]):
            values = list(same); geo = [x["geometry"] for x in values if x["geometry"]["mean_pair_distance"] is not None]
            out.append({"law": law, "facet_count": f, "exact_type": typ, "rows": len(values),
                        "mean_within_type_pair_distance": sum(x["mean_pair_distance"] for x in geo) / len(geo) if geo else None,
                        "max_within_type_pair_distance": max((x["max_pair_distance"] for x in geo), default=None),
                        "interpretation": "geometry variation within an exact incidence type; normalization is only a Euclidean diagnostic"})
    return out


def git_clean() -> bool | None:
    try:
        return not bool(subprocess.run(["git", "status", "--porcelain", "--untracked-files=no", "--", ANALYZER_REPO_PATH], capture_output=True, text=True, check=True).stdout.strip())
    except (OSError, subprocess.SubprocessError):
        return None


def git_revision() -> str | None:
    try:
        return subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True).stdout.strip()
    except (OSError, subprocess.SubprocessError):
        return None


def run(args: argparse.Namespace) -> dict[str, Any]:
    paths = [Path(p) for p in args.input]
    rows, rejects = load_rows(paths, args.exact_node_cap, args.max_rows_per_input, getattr(args, "facet_counts", None))
    laws: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in rows: laws[row["law"]].append(row)
    occupancy = {}
    for (law, f), members in sorted({(law, f): [r for r in rows if r["law"] == law and r["facet_count"] == f] for law in laws for f in {r["facet_count"] for r in laws[law]}}.items()):
        occupancy[f"{law}|F={f}"] = exact_occupancy(members, args.seed)
    facets = sorted({r["facet_count"] for r in rows})
    budget = min((len([r for r in rows if r["law"] == law and r["facet_count"] == f and r["canonical_status"] == "exact"]) for law in laws for f in facets), default=0)
    report = {
        "schema": "generator-combinatorial-coverage-report-v1", "seed": args.seed,
        "inputs": [{"path": str(p), "sha256": sha256(p)} for p in paths],
        "analyzer_repo_path": ANALYZER_REPO_PATH, "analyzer_source_sha256": sha256(Path(__file__)),
        "source_revision": git_revision(),
        "source_clean_tracked_scope": git_clean(),
        "selection": {"max_rows_per_input": args.max_rows_per_input, "exact_node_cap": args.exact_node_cap,
                      "facet_counts": sorted(args.facet_counts) if getattr(args, "facet_counts", None) is not None else None,
                      "stable_order": "sha256(seed, law, sample_id) bytes", "balanced_budget": budget},
        "rows": {"accepted": len(rows), "rejected": len(rejects), "rejects": rejects[:100]},
        "exchangeability_contract": "Rows from one producer law and fixed F are exchangeable only conditional on the named source panel. Product rows sharing factor/root identifiers and transformed/fixed panels are correlated; no independent-draw inference is made.",
        "occupancy_by_law_and_facet_count": occupancy,
        "between_laws": {f"F={f}": between(laws, f, budget, args.seed) for f in facets},
        "incremental_new_type_yield": {f"F={f}": incremental_yield(laws, f, args.seed) for f in facets},
        "within_type_geometry": geometry_by_type(rows),
        "interpretation": {"allowed": ["observed exact-type occupancy and finite-panel directed coverage", "diagnostic rarefaction and collision summaries", "geometry variation conditional on an exact incidence type"],
                           "prohibited": ["target, sys, capacity, population-support, or all-combinatorics-reached claims", "Good-Turing/Chao support proof", "independence or natural-law ranking", "WL/capped labels as exact type certificates"]},
        "deferred": {"exact_canonicalization_above_cap": "bounded search is intentionally fail-closed; a nauty/traces-grade backend was not added", "full 4D source incidence sidecars": "adapter reconstructs exact incidence from retained rationals; future producers should emit the matrix directly", "formal affine/product equivalence": "geometry-within-type is a Euclidean diagnostic, not an affine classifier"},
    }
    return report


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", action="append", required=True, help="JSONL input; repeat for panels")
    parser.add_argument("--out-dir", required=True)
    parser.add_argument("--seed", type=int, default=SEED)
    parser.add_argument("--exact-node-cap", type=int, default=20000)
    parser.add_argument("--max-rows-per-input", type=int)
    parser.add_argument("--facet-counts", type=lambda text: {int(x) for x in text.split(",") if x},
                        help="optional comma-separated F strata to retain before the per-input cap")
    parser.add_argument("--allow-dirty", action="store_true", help="record but do not reject a tracked-dirty source")
    args = parser.parse_args()
    report = run(args)
    if report["source_clean_tracked_scope"] is False and not args.allow_dirty:
        raise SystemExit("tracked analyzer source is dirty; commit or pass --allow-dirty for a disposable run")
    out = Path(args.out_dir); out.mkdir(parents=True, exist_ok=True)
    (out / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    with (out / "summary.tsv").open("w") as handle:
        handle.write("law\tfacet_count\trows_total\trows_exact\tdistinct_exact_types\tsingletons\tdoubletons\teffective_number\tcollision_probability\n")
        for key, value in sorted(report["occupancy_by_law_and_facet_count"].items()):
            law, f = key.rsplit("|F=", 1)
            handle.write("\t".join(str(x) for x in [law, f, value["rows_total"], value["rows_exact"], value["distinct_exact_types"], value["singleton_types"], value["doubleton_types"], value["effective_number"], value["collision_probability_observed"]]) + "\n")


if __name__ == "__main__":
    main()
