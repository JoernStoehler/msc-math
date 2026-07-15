"""Derive v2 formula observations without changing raw solver evidence."""
from __future__ import annotations

from fractions import Fraction
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path


def rat(text: str | None) -> Fraction | None:
    if text is None:
        return None
    n, d = text.split("/", 1)
    return Fraction(int(n), int(d))


def center(row: dict, name: str) -> dict | None:
    return next((x for x in row["centers"] if x["center_id"] == name), None)


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/soundness-v2")
    rows = [json.loads(x) for x in (out / "raw_rows.jsonl").read_text().splitlines() if x]
    registry = json.loads((out / "formula_registry.json").read_text())
    evals: list[dict] = []
    center_counts: Counter[str] = Counter()
    exact_status: Counter[str] = Counter()
    per_case: dict[str, dict] = {}
    for row in rows:
        exact_status[row["exact_positive_witness_status"]] += 1
        target_q = rat(row["exact_positive_witness_q"]) if row["exact_positive_witness_status"] == "exists" else None
        for c in row["centers"]:
            center_counts[c["center_id"]] += c["center_availability"] == "available"
            q = c["center_q_raw_f64"]
            if q is not None and target_q is not None:
                evals.append({
                    "formula_id": f"error_q_abs__{c['center_id']}_q_raw__to_exact_positive_witness_q",
                    "target_polytope_id": row["target_polytope_id"],
                    "sigma_active_reeb_word": row["sigma_active_reeb_word"],
                    "center": c["center_id"],
                    "exact_target": "exact_positive_witness_q",
                    "observed_error_abs": abs(q - float(target_q)),
                    "status": "observed_not_a_bound",
                })
        saddle = center(row, "saddle_eig_accepted")
        if saddle and saddle["center_availability"] == "available":
            margin = saddle["center_beta_margin_f64"]
            f64_pred = "true" if margin is not None and margin > 1e-9 else "indeterminate"
            evals.append({
                "formula_id": "predicate_beta_positive__saddle_eig_accepted__to_exact_positive_witness",
                "target_polytope_id": row["target_polytope_id"],
                "sigma_active_reeb_word": row["sigma_active_reeb_word"],
                "center": "saddle_eig_accepted",
                "exact_target": "exact_positive_witness_status",
                "f64_predicate": f64_pred,
                "exact_predicate": row["exact_positive_witness_status"],
                "sound": (f64_pred != "true" or row["exact_positive_witness_status"] == "exists"),
                "status": "heuristic_predicate_comparison",
            })
    for case_id in sorted({r["target_polytope_id"] for r in rows}):
        case_rows = [r for r in rows if r["target_polytope_id"] == case_id]
        per_case[case_id] = {
            "row_count": len(case_rows),
            "cohorts": sorted({r["target_input_kind"] for r in case_rows}),
            "exact_positive_witness_status": dict(Counter(r["exact_positive_witness_status"] for r in case_rows)),
            "saddle_retained_count": sum(r["f64_retained_by_saddle"] for r in case_rows),
            "complete_stream_contract": sorted({r["supplied_stream_completeness"] for r in case_rows}),
        }
    with (out / "formula_evaluations.jsonl").open("w") as f:
        for item in evals:
            f.write(json.dumps(item, sort_keys=True) + "\n")
    analysis = {
        "schema_version": "qp-soundness-analysis-v2",
        "raw_row_count": len(rows),
        "formula_registry_entry_count": len(registry),
        "available_center_counts": dict(center_counts),
        "exact_positive_witness_status": dict(exact_status),
        "cases": per_case,
        "interpretation_boundary": "Observed errors compare each named f64 center only to the same-sigma stored-rational exact positive witness. They do not establish a theorem, algebraic HKO behavior, candidate-family recall beyond the declared stream, or physical-orbit identity.",
    }
    (out / "analysis.json").write_text(json.dumps(analysis, indent=2, sort_keys=True) + "\n")
    lines = [
        "# QP soundness v2 interpretation boundary",
        "",
        "The raw rows compare named f64 centres with the exact rational KKT witness for the same supplied word. An unavailable exact witness is not silently treated as a negative-Q or beta-negative result.",
        "",
        "HKO rows target the stored binary64 rational coordinates only. They are deliberately post-selected regressions and do not transfer to the intended algebraic HKO object.",
        "",
        "Policy summaries distinguish f64 heuristic, retained-set exactness, selected-window exactness, and supplied-stream exactness. Active words are candidate words, not physical-orbit sets.",
    ]
    (out / "interpretation.md").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
