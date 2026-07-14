#!/usr/bin/env python3
"""Generate the compact decision summary for the bounded resampling smoke."""

import argparse
import hashlib
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from statistics import median


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def read_jsonl(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.strip()]


def numeric_range(rows, key):
    if not rows:
        return None
    values = [r["metrics"][key] for r in rows]
    return {
        "min": min(values),
        "median": median(values),
        "max": max(values),
        "span": max(values) - min(values),
    }


def summarize_group(rows):
    accepted = [r for r in rows if r["status"] == "accepted"]
    rejected = [r for r in rows if r["status"] != "accepted"]
    evaluated = [r for r in rows if r["target_evaluation_index"] is not None]
    bounce_labels = Counter(
        "+".join(map(str, r["metrics"]["global_bounce_labels"])) for r in accepted
    )
    outcome_states = Counter()
    takeover_identities = Counter()
    for row in accepted:
        metrics = row["metrics"]
        if not metrics["fixed_candidate_stream_present"]:
            state = "candidate_stream_missing"
        elif not metrics["fixed_geometrically_feasible_at_1e_8"]:
            state = "recovered_orbit_infeasible"
        elif metrics["fixed_global_minimal_exact"]:
            state = "recovered_orbit_survives_and_is_global"
        else:
            state = "recovered_orbit_survives_but_takeover"
        outcome_states[state] += 1
        for sigma in metrics["takeover_sigmas"]:
            takeover_identities["-".join(map(str, sigma))] += 1
    survives_global = [
        r
        for r in accepted
        if r["metrics"]["fixed_geometrically_feasible_at_1e_8"]
        and r["metrics"]["fixed_global_minimal_exact"]
    ]
    survives_takeover = [
        r
        for r in accepted
        if r["metrics"]["fixed_geometrically_feasible_at_1e_8"]
        and not r["metrics"]["fixed_global_minimal_exact"]
    ]
    return {
        "proposals": len(rows),
        "accepted": len(accepted),
        "proposal_acceptance_rate": len(accepted) / len(rows),
        "target_evaluations": len(evaluated),
        "rejection_reasons": dict(sorted(Counter(r["rejection_reason"] for r in rejected).items())),
        "fixed_action_exact_agreement": sum(
            r["metrics"]["fixed_action_exact_agrees_with_base"] for r in accepted
        ),
        "fixed_geometric_feasible": sum(
            r["metrics"]["fixed_geometrically_feasible_at_1e_8"] for r in accepted
        ),
        "fixed_candidate_stream_present": sum(
            r["metrics"]["fixed_candidate_stream_present"] for r in accepted
        ),
        "fixed_global_minimal": sum(
            r["metrics"]["fixed_global_minimal_exact"] for r in accepted
        ),
        "takeovers": sum(not r["metrics"]["fixed_global_minimal_exact"] for r in accepted),
        "outcome_state_counts": dict(sorted(outcome_states.items())),
        "takeover_identity_counts": dict(sorted(takeover_identities.items())),
        "global_bounce_label_counts": dict(sorted(bounce_labels.items())),
        "fixed_inactive_clearance": numeric_range(accepted, "fixed_inactive_clearance_min"),
        "log_volume_ratio_from_base": numeric_range(accepted, "log_volume_ratio_from_base"),
        "log_global_sys_ratio_from_base": numeric_range(accepted, "log_global_sys_ratio_from_base"),
        "log_fixed_branch_sys_ratio_from_base": numeric_range(
            accepted, "log_fixed_branch_sys_ratio_from_base"
        ),
        "global_sys": numeric_range(accepted, "global_sys"),
        "fixed_branch_sys": numeric_range(accepted, "fixed_branch_sys"),
        "survives_and_global": {
            "rows": len(survives_global),
            "log_volume_ratio_from_base": numeric_range(
                survives_global, "log_volume_ratio_from_base"
            ),
            "global_sys": numeric_range(survives_global, "global_sys"),
        },
        "survives_but_takeover": {
            "rows": len(survives_takeover),
            "log_volume_ratio_from_base": numeric_range(
                survives_takeover, "log_volume_ratio_from_base"
            ),
            "global_sys": numeric_range(survives_takeover, "global_sys"),
        },
        "constraint_diagnostics": {
            "available": len(accepted),
            "rank_counts": dict(
                sorted(Counter(str(r["metrics"]["fixed_constraint_rank_exact"]) for r in accepted).items())
            ),
            "kernel_dimension_counts": dict(
                sorted(
                    Counter(
                        str(r["metrics"]["fixed_constraint_kernel_dimension"]) for r in accepted
                    ).items()
                )
            ),
            "recovery_solution_dimension_counts": dict(
                sorted(
                    Counter(
                        str(r["metrics"]["fixed_recovery_solution_dimension"]) for r in accepted
                    ).items()
                )
            ),
        },
        "evaluation_wall_seconds_sum": sum(
            r["metrics"]["evaluation_wall_seconds"] for r in accepted
        ),
        "evaluation_wall_seconds_max": max(
            r["metrics"]["evaluation_wall_seconds"] for r in accepted
        ),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--proposals", required=True)
    ap.add_argument("--bases", required=True)
    ap.add_argument("--runtime", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    rows = read_jsonl(args.proposals)
    bases = json.loads(Path(args.bases).read_text(encoding="utf-8"))
    if not rows:
        raise ValueError("no proposal rows")
    if any(r["schema"] != "product-bounce-active-resampling/proposal/v1" for r in rows):
        raise ValueError("proposal schema mismatch")
    grouped = defaultdict(list)
    for row in rows:
        grouped[(row["base_name"], row["law"])].append(row)
    expected = {(b["name"], law) for b in bases["bases"] for law in ("fixed_ranks", "unlabeled_support")}
    if set(grouped) != expected:
        raise ValueError(f"base/law groups differ: {set(grouped) ^ expected}")
    groups = {
        f"{base}|{law}": summarize_group(rs)
        for (base, law), rs in sorted(grouped.items())
    }
    accepted = [r for r in rows if r["status"] == "accepted"]
    target_indices = [r["target_evaluation_index"] for r in rows if r["target_evaluation_index"] is not None]
    action_failures = [
        r["base_name"]
        for r in accepted
        if not r["metrics"]["fixed_action_exact_agrees_with_base"]
    ]
    summary = {
        "schema": "product-bounce-active-resampling/summary/v1",
        "purpose": "technical and mathematical feasibility/failure-mode gate; not a class-effect estimate",
        "inputs": {
            "proposals": {"path": args.proposals, "sha256": sha256(args.proposals)},
            "bases": {"path": args.bases, "sha256": sha256(args.bases)},
            "runtime": {"path": args.runtime, "sha256": sha256(args.runtime)},
            "base_source_inputs": bases["inputs"],
        },
        "design": {
            "base_is_inferential_unit": True,
            "bases": 4,
            "laws": ["fixed_ranks", "unlabeled_support"],
            "accepted_target_per_base_law": 16,
            "target_evaluation_cap": 128,
            "normalization": "none",
            "fixed_geometric_feasibility_tolerance": 1e-8,
        },
        "totals": {
            "proposal_rows": len(rows),
            "accepted": len(accepted),
            "target_evaluations": len(target_indices),
            "target_evaluation_indices_unique": len(target_indices) == len(set(target_indices)),
            "fixed_action_exact_failures": action_failures,
            "trusted_new_sys_over_one": [
                {
                    "base": r["base_name"],
                    "law": r["law"],
                    "proposal_index": r["proposal_index"],
                    "sys": r["metrics"]["global_sys"],
                }
                for r in accepted
                if r["metrics"]["global_sys"] > 1.0
            ],
        },
        "by_base_law": groups,
        "pooled_smoke_descriptives": summarize_group(rows),
        "runtime_text": Path(args.runtime).read_text(encoding="utf-8"),
    }
    if len(accepted) != 128 or len(target_indices) > 128 or action_failures:
        raise ValueError("bounded completed-run invariants failed")
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(summary["totals"], indent=2))


if __name__ == "__main__":
    main()
