import copy
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("s0_analyze", HERE / "analyze.py")
ANALYZE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ANALYZE)


def payload(sys):
    return {
        "capacity_result": {
            "min_action": 2.0,
            "min_action_lower": 2.0,
            "min_action_upper": 2.0,
            "iterations": 10,
            "orbits": [
                {"sigma": [0, 1, 2], "admissibility": "AdmissibleExact", "action": 2.0, "action_lower": 2.0, "action_upper": 2.0},
                {"sigma": [1, 2, 0], "admissibility": "IndeterminateF64", "action": 2.1, "action_lower": 2.05, "action_upper": 2.15},
            ],
        },
        "volume": 1.0,
        "sys": sys,
    }


def target(candidate_id, arm, replicate, attempt, sys, key, **extra):
    row = {
        "candidate_id": candidate_id,
        "arm": arm,
        "replicate": replicate,
        "attempt_index": attempt,
        "evaluation_status": "success",
        "cache_status": "miss",
        "polytope_key": key,
        "poly_id": f"poly-{key}",
        "sys": sys,
        "capacity": 2.0,
        "volume": 1.0,
        "capacity_iterations": 10,
        "raw_returned_word_count": 2,
        "raw_admissible_word_count": 1,
        "distinct_cyclic_class_count": 1,
        "support_lengths": [3],
        "wall_time_ms": 1.0,
        "construction_rejections_before": 0,
        "construction_attempt": 0,
        "construction_sequence_index": attempt - 1,
        "generation": None,
        "trajectory": None,
        "iteration": None,
        "proposal_index": attempt - 1,
        "role": "iid",
        "parent_candidate_id": None,
        "elite_set_id": None,
        "became_next_state": False,
        "product_chart": {"q_gap_logits": [0.0] * 4, "q_centered_log_radii": [0.0] * 5, "p_gap_logits": [0.0] * 4, "p_centered_log_radii": [0.0] * 5, "relative_phase": 0.0, "near_tie": False},
    }
    row.update(extra)
    row["candidate_id"] = ANALYZE.expected_candidate_id(row, "fixture target")
    return row


def candidate(arm_index, replicate, number):
    return f"s0v1-{arm_index:02x}{replicate:02x}{number:020x}"


def dual_vertices(serial):
    return [[str(serial), "0", "0", "0"] for _ in range(10)]


def exact_key(serial):
    return "|".join(",".join(vertex) for vertex in dual_vertices(serial))


def smoke_artifacts():
    rows, cache, lineages = [], [], []
    cem = []
    for arm_index, arm in enumerate(ANALYZE.ARMS):
        for replicate in range(3):
            for attempt in range(1, 257):
                generation = (attempt - 1) // 64 if arm == "diagonal_cem" else None
                local = arm == "multistart_branch_local_phase0"
                serial = arm_index * 10_000 + replicate * 1_000 + attempt
                row = target("", arm, replicate, attempt, 0.7 + arm_index * 0.03 + attempt / 10000, exact_key(serial), generation=generation, trajectory=attempt - 1 if local else None, iteration=None, proposal_index=(attempt - 1) % 64 if generation is not None else attempt - 1, construction_attempt=(attempt - 1) % 64 if generation is not None else 0, role="cem_population" if generation is not None else ("local_start" if local else "iid"))
                rows.append(row)
                cache.append({"arm": arm, "replicate": replicate, "polytope_key": row["polytope_key"], "poly_id": row["poly_id"], "dual_vertices_rational": dual_vertices(serial), "facet_count": 10, **payload(row["sys"])})
                lineages.append({"candidate_id": row["candidate_id"], "parent_kind": "none", "parent_candidate_id": None, "elite_set_id": None})
            if arm == "diagonal_cem":
                for generation in range(4):
                    members = [row for row in rows if row["arm"] == arm and row["replicate"] == replicate and row["generation"] == generation]
                    elite = sorted(members, key=lambda row: (-row["sys"], row["candidate_id"]))[:16]
                    elite_set_id = ANALYZE.expected_elite_set_id([row["candidate_id"] for row in elite])
                    parent = None if generation == 0 else cem[-1]["elite_set_id"]
                    for member in members:
                        member["elite_set_id"] = parent
                        lineage = next(item for item in lineages if item["candidate_id"] == member["candidate_id"])
                        lineage["elite_set_id"] = parent
                        lineage["parent_kind"] = "none" if parent is None else "distribution"
                    cem.append({"replicate": replicate, "generation": generation, "elite_set_id": elite_set_id, "parent_elite_set_id": parent, "member_candidate_ids": [row["candidate_id"] for row in members], "elite_candidate_ids": [row["candidate_id"] for row in elite], "distribution": {"mean": [0.0] * 17, "variance": [0.0] * 17, "generation_zero_variance": [0.0] * 17}, "complete": True, "construction_attempts": 64, "construction_rejections": 0})
    arm_runs = []
    for arm in ANALYZE.ARMS:
        for replicate in range(3):
            group = [row for row in rows if row["arm"] == arm and row["replicate"] == replicate]
            arm_runs.append({"arm": arm, "replicate": replicate, "target_attempts": 256, "successful_new_computations": 256, "cache_hits": 0, "failed_new_computations": 0, "construction_attempts": 256, "construction_rejections": 0, "target_wall_time_ms": 256.0, "total_wall_time_ms": 256.0, "status": "complete"})
    local_trajectories = [{"arm": "multistart_branch_local_phase0", "replicate": row["replicate"], "trajectory": row["trajectory"], "start_candidate_id": row["candidate_id"], "final_candidate_id": row["candidate_id"], "start_sys": row["sys"], "final_sys": row["sys"], "accepted_iterations": 0, "stop": "no_direction", "complete": True} for row in rows if row["arm"] == "multistart_branch_local_phase0"]
    run_status = {"packet_version": ANALYZE.FROZEN_CONFIG["packet_version"], "complete": True, "charged_target_attempts": 2304, "overall_wall_time_ms": 2304.0}
    return rows, cache, cem, lineages, [], arm_runs, local_trajectories, run_status


class SummaryReconciliationTests(unittest.TestCase):
    def test_smoke_artifacts_reconcile_and_material_thresholds_apply(self):
        summary = ANALYZE.summarize(*smoke_artifacts(), ANALYZE.FROZEN_CONFIG)
        self.assertEqual(summary["accounting"]["target_rows"], 2304)
        self.assertTrue(summary["material_ahead"]["diagonal_cem"]["is_materially_ahead"])
        self.assertIn("inferential_statistics", summary["unavailable_metrics"])

    def test_missing_cache_row_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        with self.assertRaisesRegex(ValueError, "absent arm-private cache"):
            ANALYZE.summarize(rows, cache[:-1], cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_cache_hit_before_private_miss_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        rows[1]["cache_status"] = "hit"
        with self.assertRaisesRegex(ValueError, "cache hit precedes"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_corrupt_cache_payload_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cache[0]["capacity_result"]["iterations"] = 11
        with self.assertRaisesRegex(ValueError, "capacity_iterations disagrees"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_duplicate_cem_generation_false_pass_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cem[1]["generation"] = 0
        with self.assertRaisesRegex(ValueError, "duplicate CEM generation"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_cem_elite_genealogy_false_pass_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cem[1]["parent_elite_set_id"] = "forged"
        with self.assertRaisesRegex(ValueError, "parent elite-set genealogy"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_lineage_parent_kind_and_id_false_pass_fail(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        lineages[64]["parent_kind"] = "candidate"
        lineages[64]["parent_candidate_id"] = lineages[63]["candidate_id"]
        with self.assertRaisesRegex(ValueError, "lineage parent fields disagree|CEM parent kind"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_failed_target_must_explicitly_mark_payload_unavailable(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        bad = rows[0]
        bad["evaluation_status"] = "failure"
        bad["cache_status"] = "failed_miss"
        with self.assertRaisesRegex(ValueError, "must explicitly mark capacity unavailable"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_resolved_config_change_fails_closed(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        config = copy.deepcopy(ANALYZE.FROZEN_CONFIG)
        config["cem"]["elites"] = 15
        with self.assertRaisesRegex(ValueError, "resolved-config identity/constants"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status, config)

    def test_rejection_detail_and_arm_run_accounting_false_pass_fail(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        rows[0]["construction_rejections_before"] = 1
        arm_runs[0]["construction_rejections"] = 1
        arm_runs[0]["construction_attempts"] = 257
        with self.assertRaisesRegex(ValueError, "detailed rejection rows disagree"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_artifact_config_must_be_byte_identical_to_selected_config(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            artifacts = root / "artifacts"
            artifacts.mkdir()
            source = root / "source-config.json"
            source.write_text(json.dumps(ANALYZE.FROZEN_CONFIG, indent=2) + "\n", encoding="utf-8")
            (artifacts / "resolved-config.json").write_text(json.dumps(ANALYZE.FROZEN_CONFIG, sort_keys=True) + "\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "byte-identical"):
                ANALYZE.identical_config_artifact(artifacts, source)

    def test_corrupt_generation_zero_distribution_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cem[0]["distribution"]["variance"][3] = 0.01
        with self.assertRaisesRegex(ValueError, "distribution variance\\[3\\]"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_corrupt_smoothed_distribution_update_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cem[1]["distribution"]["mean"][8] = 0.01
        with self.assertRaisesRegex(ValueError, "distribution mean\\[8\\]"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_run_status_underreports_sequential_wall_time_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        run_status["overall_wall_time_ms"] = 1.0
        with self.assertRaisesRegex(ValueError, "overall wall time is less"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_circular_zero_resultant_uses_frozen_generation_zero_id_fallback(self):
        first = target(candidate(2, 0, 2), "diagonal_cem", 0, 1, 0.8, "phase-a", generation=0, proposal_index=0, role="cem_population")
        second = target(candidate(2, 0, 1), "diagonal_cem", 0, 2, 0.8, "phase-b", generation=0, proposal_index=1, role="cem_population")
        first["product_chart"]["relative_phase"] = 0.0
        second["product_chart"]["relative_phase"] = 3.141592653589793
        mean, _ = ANALYZE.coordinate_moments([first, second], ANALYZE.chart_coordinates(second, "fallback")[16])
        self.assertAlmostEqual(mean[16], 3.141592653589793)

    def test_forged_local_start_parent_fails_even_with_matching_lineage(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        local_rows = [row for row in rows if row["arm"] == "multistart_branch_local_phase0" and row["replicate"] == 0]
        forged, parent = local_rows[1], local_rows[0]
        forged["parent_candidate_id"] = parent["candidate_id"]
        lineage = next(row for row in lineages if row["candidate_id"] == forged["candidate_id"])
        lineage.update({"parent_kind": "candidate", "parent_candidate_id": parent["candidate_id"]})
        with self.assertRaisesRegex(ValueError, "local start parent/index fields"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_forged_rejection_role_fails_after_exact_sequence_reconciliation(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        iid_rows = [row for row in rows if row["arm"] == "iid" and row["replicate"] == 0]
        for row in iid_rows:
            row["construction_sequence_index"] += 1
        accepted = iid_rows[0]
        old_id = accepted["candidate_id"]
        accepted["construction_attempt"] = 1
        accepted["construction_rejections_before"] = 1
        accepted["candidate_id"] = ANALYZE.expected_candidate_id(accepted, "accepted after rejection")
        next(row for row in lineages if row["candidate_id"] == old_id)["candidate_id"] = accepted["candidate_id"]
        arm_runs[0]["construction_rejections"] = 1
        arm_runs[0]["construction_attempts"] = 257
        rejection = {"arm": "iid", "replicate": 0, "generation": None, "trajectory": None, "iteration": None, "proposal_index": 0, "construction_attempt": 0, "construction_sequence_index": 0, "role": "iid", "reason": "fixture"}
        rejection["candidate_id"] = ANALYZE.expected_candidate_id(rejection, "fixture rejection")
        rejection["role"] = "overshoot"
        rejections.append(rejection)
        with self.assertRaisesRegex(ValueError, "IID rejection identity"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_mutated_rational_dual_payload_cannot_keep_its_key(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cache[0]["dual_vertices_rational"][0][0] = "999/7"
        with self.assertRaisesRegex(ValueError, "polytope_key does not exactly encode"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_forged_target_poly_ids_fail_against_cache(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        for row in rows:
            row["poly_id"] = "forged-poly-id"
        with self.assertRaisesRegex(ValueError, "poly_id disagrees with cache"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_orphan_overshoot_with_recomputed_identity_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        local_rows = [row for row in rows if row["arm"] == "multistart_branch_local_phase0" and row["replicate"] == 0]
        forged, foreign_parent = local_rows[1], local_rows[0]
        old_id = forged["candidate_id"]
        forged.update({"role": "overshoot", "iteration": 0, "proposal_index": 5, "parent_candidate_id": foreign_parent["candidate_id"]})
        forged["candidate_id"] = ANALYZE.expected_candidate_id(forged, "forged overshoot")
        lineage = next(row for row in lineages if row["candidate_id"] == old_id)
        lineage.update({"candidate_id": forged["candidate_id"], "parent_kind": "candidate", "parent_candidate_id": foreign_parent["candidate_id"]})
        with self.assertRaisesRegex(ValueError, "no preceding successful same-trajectory start|parent is not reachable within same trajectory"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_iid_rejection_proposal_999_with_recomputed_identity_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        rejection = {"arm": "iid", "replicate": 0, "generation": None, "trajectory": None, "iteration": None, "proposal_index": 999, "construction_attempt": 0, "construction_sequence_index": 256, "role": "iid", "reason": "forged"}
        rejection["candidate_id"] = ANALYZE.expected_candidate_id(rejection, "proposal 999")
        rejections.append(rejection)
        with self.assertRaisesRegex(ValueError, "IID rejection identity"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_fabricated_local_iteration_999_rejection_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        start = next(row for row in rows if row["arm"] == "multistart_branch_local_phase0" and row["replicate"] == 0 and row["trajectory"] == 0)
        rejection = {"arm": "multistart_branch_local_phase0", "replicate": 0, "generation": None, "trajectory": 0, "iteration": 999, "proposal_index": 0, "construction_attempt": 0, "construction_sequence_index": 0, "role": "within_step", "reason": "fabricated"}
        rejection["candidate_id"] = ANALYZE.expected_candidate_id(rejection, "fabricated local rejection")
        rejections.append(rejection)
        with self.assertRaisesRegex(ValueError, "step iterations are not contiguous|iteration exceeds accepted trajectory iterations"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_cem_rejection_after_64th_acceptance_fails_even_when_accounted_in_next_generation(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        cem_rows = [row for row in rows if row["arm"] == "diagonal_cem" and row["replicate"] == 0]
        for row in cem_rows:
            if row["construction_sequence_index"] >= 64:
                row["construction_sequence_index"] += 1
        next_generation_first = next(row for row in cem_rows if row["generation"] == 1 and row["proposal_index"] == 0)
        next_generation_first["construction_rejections_before"] = 1
        cem[1]["construction_rejections"] = 1
        cem[1]["construction_attempts"] = 65
        arm_runs[6]["construction_rejections"] = 1
        arm_runs[6]["construction_attempts"] = 257
        rejection = {"arm": "diagonal_cem", "replicate": 0, "generation": 0, "trajectory": None, "iteration": None, "proposal_index": 63, "construction_attempt": 64, "construction_sequence_index": 64, "role": "cem_population", "reason": "after complete"}
        rejection["candidate_id"] = ANALYZE.expected_candidate_id(rejection, "post-complete CEM rejection")
        rejections.append(rejection)
        with self.assertRaisesRegex(ValueError, "construction occurred after completed population"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)

    def test_local_complete_grid_with_missing_proposal_fails(self):
        rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status = smoke_artifacts()
        local = [row for row in rows if row["arm"] == "multistart_branch_local_phase0" and row["replicate"] == 0]
        start = local[0]
        for proposal_index, row in enumerate(local[1:5]):
            old_id = row["candidate_id"]
            row.update({"trajectory": 0, "iteration": 0, "proposal_index": proposal_index, "role": "within_step", "parent_candidate_id": start["candidate_id"]})
            row["candidate_id"] = ANALYZE.expected_candidate_id(row, "partial local grid")
            next(item for item in lineages if item["candidate_id"] == old_id).update({"candidate_id": row["candidate_id"], "parent_kind": "candidate", "parent_candidate_id": start["candidate_id"]})
        local_trajectories[:] = [record for record in local_trajectories if not (record["replicate"] == 0 and record["trajectory"] in {1, 2, 3, 4})]
        trajectory_zero = next(record for record in local_trajectories if record["replicate"] == 0 and record["trajectory"] == 0)
        trajectory_zero.update({"stop": "no_improvement", "complete": True, "accepted_iterations": 0, "final_candidate_id": start["candidate_id"], "final_sys": start["sys"]})
        with self.assertRaisesRegex(ValueError, "no-improvement terminal grid invalid"):
            ANALYZE.summarize(rows, cache, cem, lineages, rejections, arm_runs, local_trajectories, run_status)


if __name__ == "__main__":
    unittest.main()
