#!/usr/bin/env python3
"""Validate and summarize the first paired score-comparison smoke."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import time
from pathlib import Path

EXPECTED = {
    "mechanism": (
        "f6be75d99a357735276fc4b6eb36b0549c823dd75faeedb4fc7506903da2f1b8",
        "narrow_gap",
        0,
    ),
    "ordinary": (
        "3daddfde522cb04777d651814d7f88a31f6ec20c1b7ac8fc960efc3e4534104e",
        "large_gap",
        2,
    ),
    "equality": (
        "43d2432913e3f665557c74ae146711b03fbbdb4182479852672cf1db98dec8cc",
        "narrow_gap",
        1,
    ),
}
EPS = 2.0e-12
ALLOWED_WITNESS_ADMISSIBILITY = {"AdmissibleF64", "AdmissibleExact"}


def rows(path: Path):
    with path.open() as f:
        yield from (json.loads(line) for line in f if line.strip())


def finite(value):
    return isinstance(value, (int, float)) and math.isfinite(value)


def sha256(path: Path) -> str:
    # blake3 is intentionally not a Python dependency for this smoke; SHA-256
    # is only an analyzer identity aid, while producer provenance has BLAKE3.
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def rank_indices(values):
    # Producer ranks by stable row order after descending total_cmp. Values in
    # this smoke are finite and non-tied, but index order makes ties explicit.
    return {
        index: rank
        for rank, (index, _value) in enumerate(
            sorted(enumerate(values), key=lambda item: (-item[1], item[0])), 1
        )
    }


def analyze_shard(role: str, shard: Path):
    fixture = list(rows(shard / "fixture-selection.jsonl"))
    expected_id, expected_label, expected_rank = EXPECTED[role]
    assert len(fixture) == 1, (role, "fixture count", len(fixture))
    selected = fixture[0]
    assert selected["poly_id"] == expected_id, (role, "wrong selected ID", selected["poly_id"])
    assert selected["degeneracy_label"] == expected_label
    assert selected["selection_rank_within_label"] == expected_rank

    audit = list(rows(shard / "step-ranking-audit.jsonl"))
    assert audit, (role, "empty audit")
    assert all(row["status"] == "ok" for row in audit), (role, "producer failure")
    assert all(row["poly_id"] == expected_id and row["iteration"] == 0 for row in audit)
    assert all(row["audit_step_policy"] == "fixed" for row in audit)
    assert all(row.get("move_key") for row in audit)
    assert len({row["move_key"] for row in audit}) == len(audit), (role, "duplicate move")
    assert len(audit) <= 6, (role, "shard evaluation cap", len(audit))

    for row in audit:
        for key in ("observed_delta_sys", "target_sys", "base_sys",
                    "near_active_predicted_delta_sys",
                    "candidate_window_predicted_delta_sys"):
            assert finite(row.get(key)), (role, key, row)
        assert abs((row["target_sys"] - row["base_sys"]) - row["observed_delta_sys"]) <= EPS
        # The guarded score must expose its witness, including the action
        # interval and numerical guards, whenever a prediction is present.
        for key in (
            "candidate_window_witness_action",
            "candidate_window_witness_action_lower",
            "candidate_window_witness_action_upper",
            "candidate_window_witness_relative_action_gap",
            "candidate_window_witness_q_error_bound",
            "candidate_window_witness_beta_margin",
        ):
            assert finite(row.get(key)), (role, "missing witness", key)
        assert row["candidate_window_witness_admissibility"] in ALLOWED_WITNESS_ADMISSIBILITY
        assert row["candidate_window_witness_action_lower"] - EPS <= row["candidate_window_witness_action"]
        assert row["candidate_window_witness_action"] <= row["candidate_window_witness_action_upper"] + EPS
        assert row["candidate_window_witness_relative_action_gap"] >= -EPS
        assert row["candidate_window_witness_relative_action_gap"] <= 0.01 + EPS
        assert row["candidate_window_witness_q_error_bound"] >= 0.0
        assert row["candidate_window_witness_beta_margin"] >= -EPS

    observed = [row["observed_delta_sys"] for row in audit]
    near = [row["near_active_predicted_delta_sys"] for row in audit]
    guarded = [row["candidate_window_predicted_delta_sys"] for row in audit]
    computed = {
        "observed": rank_indices(observed),
        "near_active": rank_indices(near),
        "candidate_window": rank_indices(guarded),
    }
    for index, row in enumerate(audit):
        assert row["observed_rank_desc"] == computed["observed"][index]
        assert row["near_active_rank_desc"] == computed["near_active"][index]
        assert row["candidate_window_rank_desc"] == computed["candidate_window"][index]

    best_index = max(range(len(audit)), key=lambda i: observed[i])
    result = {
        "role": role,
        "poly_id": expected_id,
        "source_label": selected["selection_buckets"],
        "source_datasets": selected["datasets"],
        "move_rows": len(audit),
        "exact_evaluations": len({row["exact_evaluation_order"] for row in audit}),
        "observed_best_move": audit[best_index]["move_key"],
        "observed_best_delta_sys": observed[best_index],
        "near_active_selected_move": audit[min(computed["near_active"], key=lambda i: computed["near_active"][i])]["move_key"],
        "candidate_window_selected_move": audit[min(computed["candidate_window"], key=lambda i: computed["candidate_window"][i])]["move_key"],
        "near_active_selected_observed_delta_sys": observed[min(computed["near_active"], key=lambda i: computed["near_active"][i])],
        "candidate_window_selected_observed_delta_sys": observed[min(computed["candidate_window"], key=lambda i: computed["candidate_window"][i])],
        "near_active_regret_sys": observed[best_index] - observed[min(computed["near_active"], key=lambda i: computed["near_active"][i])],
        "candidate_window_regret_sys": observed[best_index] - observed[min(computed["candidate_window"], key=lambda i: computed["candidate_window"][i])],
        "candidate_window_witness_rows": sum(
            row["candidate_window_witness_action"] is not None for row in audit
        ),
    }
    return result, audit


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--mechanism-dir", type=Path, required=True)
    parser.add_argument("--ordinary-dir", type=Path, required=True)
    parser.add_argument("--equality-dir", type=Path, required=True)
    args = parser.parse_args()
    started = time.monotonic()
    shards = {
        "mechanism": args.mechanism_dir,
        "ordinary": args.ordinary_dir,
        "equality": args.equality_dir,
    }
    results = []
    total_evaluations = 0
    identities = {}
    for role, shard in shards.items():
        result, _audit = analyze_shard(role, shard)
        results.append(result)
        total_evaluations += result["exact_evaluations"]
        for name in ("run-provenance.json", "compute-budget-report.json", "summary.json"):
            path = shard / name
            assert path.is_file(), (role, "missing", name)
            identities[f"{role}/{name}"] = sha256(path)
    assert total_evaluations <= 18, total_evaluations
    args.out_dir.joinpath("comparison.jsonl").write_text(
        "".join(json.dumps(result, sort_keys=True) + "\n" for result in results)
    )
    summary = {
        "packet": "first_optimizer_score_comparison",
        "roles": [result["role"] for result in results],
        "total_exact_evaluations": total_evaluations,
        "per_role_exact_evaluations": {
            result["role"]: result["exact_evaluations"] for result in results
        },
        "source_artifact_sha256": identities,
        "validation": {
            "same_exact_move_rows": True,
            "requested_cases_present": True,
            "rank_and_regret_recomputed": True,
            "candidate_window_witness_guards_visible": True,
        },
        "elapsed_analysis_ms": 1000.0 * (time.monotonic() - started),
        "caveat": "selected iteration-0 smoke; not a population estimate or thesis evidence",
    }
    args.out_dir.joinpath("summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
