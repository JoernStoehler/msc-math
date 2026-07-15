import copy
import hashlib
import json
import os
import signal
import shutil
import subprocess
import tempfile
import time
import unittest
from pathlib import Path

import analyze
from analyze import ArtifactError, verify


HERE = Path(__file__).resolve().parent
EXECUTABLE = HERE / "target" / "debug" / "adaptive-multilevel-splitting"
CONFIG = HERE / "resolved-config.json"


class AnalyzerCorruptionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        subprocess.run(["cargo", "build"], cwd=HERE, check=True, capture_output=True)
        cls.fixtures = tempfile.TemporaryDirectory()
        root = Path(cls.fixtures.name)
        cls.normal = root / "normal"
        cls.hit = root / "hit"
        cls.timeout = root / "timeout"
        cls.large_response = root / "large-response"
        cls.final_failure = root / "final-failure"
        cls.final_hit = root / "final-hit"
        cls.run_packet(cls.normal)
        cls.run_packet(cls.hit, "--force-synthetic-hit")
        result = cls.run_packet(
            cls.timeout,
            "--synthetic-child-delay-ms",
            "100",
            "--synthetic-call-timeout-ms",
            "10",
            check=False,
        )
        if result.returncode == 0:
            raise AssertionError("slow child fixture unexpectedly completed")
        cls.run_packet(
            cls.large_response,
            "--force-synthetic-hit",
            "--synthetic-response-padding-bytes",
            "1000000",
        )
        if cls.run_packet(
            cls.final_failure, "--synthetic-fail-call", "48", check=False
        ).returncode == 0:
            raise AssertionError("final mutation failure fixture unexpectedly completed")
        cls.run_packet(cls.final_hit, "--synthetic-hit-call", "48")

    @classmethod
    def tearDownClass(cls):
        cls.fixtures.cleanup()

    @classmethod
    def run_packet(cls, directory, *extra, check=True):
        return subprocess.run(
            [
                EXECUTABLE,
                "synthetic",
                "--config",
                CONFIG,
                "--artifacts",
                directory,
                *extra,
            ],
            cwd=HERE,
            check=check,
            capture_output=True,
            text=True,
        )

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name) / "packet"
        shutil.copytree(self.normal, self.directory)

    def tearDown(self):
        self.temp.cleanup()

    def load(self, name):
        return json.loads((self.directory / name).read_text())

    def store(self, name, value):
        (self.directory / name).write_text(json.dumps(value, indent=2) + "\n")

    def load_jsonl(self, name):
        return [json.loads(line) for line in (self.directory / name).read_text().splitlines()]

    def store_jsonl(self, name, rows):
        (self.directory / name).write_text("".join(json.dumps(row, separators=(",", ":")) + "\n" for row in rows))

    def rehash(self):
        status = self.load("run-status.json")
        status["artifact_sha256"] = {
            name: hashlib.sha256((self.directory / name).read_bytes()).hexdigest()
            for name in status["artifact_sha256"]
        }
        self.store("run-status.json", status)

    def remove_final_successful_request(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        final = targets.pop()
        self.store_jsonl("target-evaluations.jsonl", targets)
        ledger = self.load_jsonl("charged-requests.jsonl")
        ledger.pop()
        self.store_jsonl("charged-requests.jsonl", ledger)
        caches = self.load_jsonl("cache.jsonl")
        if final["cache_status"] == "miss":
            caches.pop()
            self.store_jsonl("cache.jsonl", caches)
        runs = self.load_jsonl("arm-runs.jsonl")
        iid = runs[-1]
        iid["target_attempts"] -= 1
        if final["cache_status"] == "miss":
            iid["cache_misses"] -= 1
            iid["distinct_successful_keys"] -= 1
        else:
            iid["cache_hits"] -= 1
        iid["complete"] = False
        self.store_jsonl("arm-runs.jsonl", runs)
        status = self.load("run-status.json")
        status["iid_charged_requests"] -= 1
        status["total_charged_requests"] -= 1
        return final, status, runs

    def assert_corrupt(self, pattern):
        with self.assertRaisesRegex(ArtifactError, pattern):
            verify(self.directory)

    def production_refusal(self, arguments, *, validate_dirty_source=False):
        environment = os.environ.copy()
        environment["AMS_TEST_REFUSAL_ONLY"] = "1"
        if validate_dirty_source:
            environment["AMS_TEST_VALIDATE_DIRTY_SOURCE"] = "1"
        return subprocess.run(
            arguments,
            cwd=HERE,
            capture_output=True,
            text=True,
            env=environment,
        )

    def test_normal_synthetic_fixture_is_verified_but_never_production_ready(self):
        result = verify(self.directory)
        self.assertTrue(result["verified"])
        self.assertFalse(result["readiness_passed"])
        self.assertEqual(result["artifact_kind"], "synthetic_target_free")
        self.assertIsNone(result["probability_estimate"])

    def test_forced_hit_is_auditable_but_not_readiness(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.hit, self.directory)
        result = verify(self.directory)
        self.assertTrue(result["verified"])
        self.assertFalse(result["readiness_passed"])
        self.assertTrue(result["stopped_on_sys_gt_one"])

    def test_slow_child_timeout_is_charged_auditable_and_not_readiness(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.timeout, self.directory)
        result = verify(self.directory)
        self.assertEqual(result["disposition"], "timeout")
        self.assertEqual(result["adaptive_attempts"], 1)
        self.assertFalse(result["readiness_passed"])
        row = self.load_jsonl("target-evaluations.jsonl")[0]
        self.assertEqual(row["evaluation_status"], "timeout")
        self.assertEqual(len(row["dual_vertices_rational"]), 10)

    def test_response_larger_than_pipe_capacity_is_drained_without_deadlock(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.large_response, self.directory)
        result = verify(self.directory)
        self.assertEqual(result["disposition"], "sys_gt_one_stop")
        self.assertEqual(result["adaptive_attempts"], 1)

    def test_final_mutation_failure_is_auditable_without_completed_level_row(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.final_failure, self.directory)
        result = verify(self.directory)
        self.assertEqual(result["disposition"], "error")
        self.assertEqual(result["adaptive_attempts"], 48)
        self.assertEqual(result["post_level_distinct_states"].__len__(), 1)
        self.assertFalse(result["readiness_passed"])

    def test_final_mutation_hit_is_auditable_without_completed_level_row(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.final_hit, self.directory)
        result = verify(self.directory)
        self.assertEqual(result["disposition"], "sys_gt_one_stop")
        self.assertEqual(result["adaptive_attempts"], 48)
        self.assertEqual(len(result["post_level_distinct_states"]), 1)
        self.assertFalse(result["readiness_passed"])

    def test_finalized_terminal_failure_requires_terminal_transition(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.final_failure, self.directory)
        transitions = self.load_jsonl("mutation-transitions.jsonl")
        transitions.pop()
        self.store_jsonl("mutation-transitions.jsonl", transitions)
        self.rehash()
        self.assert_corrupt("transitions do not exactly reconcile")

    def test_transition_file_order_must_equal_mutation_target_order(self):
        transitions = self.load_jsonl("mutation-transitions.jsonl")
        transitions[0], transitions[1] = transitions[1], transitions[0]
        self.store_jsonl("mutation-transitions.jsonl", transitions)
        self.rehash()
        self.assert_corrupt("transition file order")

    def test_charged_ledger_is_required_one_for_one_when_finalized(self):
        ledger = self.load_jsonl("charged-requests.jsonl")
        ledger.pop()
        self.store_jsonl("charged-requests.jsonl", ledger)
        self.rehash()
        self.assert_corrupt("ledger does not reconcile")

    def test_cumulative_charge_times_must_be_ordered(self):
        ledger = self.load_jsonl("charged-requests.jsonl")
        ledger[1]["charged_monotonic_ms"] = -1
        self.store_jsonl("charged-requests.jsonl", ledger)
        self.rehash()
        self.assert_corrupt("ordered cumulative monotonic")

    def test_integer_schema_rejects_boolean_attempt_index(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        ledger = self.load_jsonl("charged-requests.jsonl")
        targets[0]["attempt_index"] = True
        ledger[0]["attempt_index"] = True
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.store_jsonl("charged-requests.jsonl", ledger)
        self.rehash()
        self.assert_corrupt("attempt index has a non-integer")

    def test_integer_schema_rejects_boolean_mutation_and_level_indices(self):
        transitions = self.load_jsonl("mutation-transitions.jsonl")
        transitions[0]["clone_index"] = False
        self.store_jsonl("mutation-transitions.jsonl", transitions)
        self.rehash()
        self.assert_corrupt("invalid mutation transition")
        shutil.rmtree(self.directory)
        shutil.copytree(self.normal, self.directory)
        levels = self.load_jsonl("levels.jsonl")
        levels[0]["level"] = False
        self.store_jsonl("levels.jsonl", levels)
        self.rehash()
        self.assert_corrupt("zero-based prefix")

    def test_target_intervals_must_be_sequential_and_nonoverlapping(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        previous_finish = targets[0]["cumulative_monotonic_ms"]
        targets[1]["started_monotonic_ms"] = previous_finish - 0.5
        targets[1]["wall_time_ms"] = (
            targets[1]["cumulative_monotonic_ms"] - targets[1]["started_monotonic_ms"]
        )
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("intervals overlap")

    def test_arm_intervals_must_be_sequential_and_nonoverlapping(self):
        runs = self.load_jsonl("arm-runs.jsonl")
        runs[1]["started_monotonic_ms"] = runs[0]["cumulative_monotonic_ms"] - 0.5
        runs[1]["wall_time_ms"] = runs[1]["cumulative_monotonic_ms"] - runs[1]["started_monotonic_ms"]
        self.store_jsonl("arm-runs.jsonl", runs)
        self.rehash()
        self.assert_corrupt("arm-run monotonic intervals overlap")

    def test_exact_row_schema_rejects_extra_key(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0]["unexpected"] = True
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("missing or extra fields")

    def test_terminal_error_schema_is_bound_to_final_failure(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.final_failure, self.directory)
        status = self.load("run-status.json")
        status["terminal_error"]["candidate_id"] = "forged"
        self.store("run-status.json", status)
        self.assert_corrupt("disagrees with final target")

    def test_exact_iid_construction_exhaustion_is_structurally_auditable(self):
        final, status, runs = self.remove_final_successful_request()
        identity = final["identity"]
        rejections = self.load_jsonl("construction-rejections.jsonl")
        for attempt in range(64):
            attempt_identity = copy.deepcopy(identity)
            attempt_identity["construction_attempt"] = attempt
            rejections.append(
                {
                    "candidate_id": analyze.expected_candidate_id(attempt_identity),
                    "identity": attempt_identity,
                    "arm": "iid",
                    "reason": "target-free construction exhaustion fixture",
                    "parent_candidate_id": None,
                    "root_candidate_id": None,
                    "raw_proposed_chart": None,
                }
            )
        self.store_jsonl("construction-rejections.jsonl", rejections)
        runs[-1]["construction_rejections"] += 64
        self.store_jsonl("arm-runs.jsonl", runs)
        first_identity = copy.deepcopy(identity)
        first_identity["construction_attempt"] = 0
        status.update(
            disposition="error",
            error="construction_exhaustion: target-free fixture",
            terminal_error={
                "kind": "construction_exhaustion",
                "arm": "iid",
                "global_request_index": None,
                "candidate_id": None,
                "evaluation_status": None,
                "failure_reason": None,
                "next_schedule_identity": first_identity,
                "level": None,
                "observed_distinct_geometry_keys": None,
                "required_distinct_geometry_keys": None,
            },
        )
        self.store("run-status.json", status)
        self.rehash()
        result = verify(self.directory)
        self.assertEqual(result["disposition"], "error")
        self.assertFalse(result["readiness_passed"])

    def test_wall_termination_before_next_charge_is_structurally_auditable(self):
        _, status, _ = self.remove_final_successful_request()
        targets = self.load_jsonl("target-evaluations.jsonl")
        final = targets[-1]
        status.update(
            disposition="error",
            error="wall_termination: target-free fixture",
            terminal_error={
                "kind": "wall_termination",
                "arm": "iid",
                "global_request_index": final["global_request_index"],
                "candidate_id": final["candidate_id"],
                "evaluation_status": None,
                "failure_reason": None,
                "next_schedule_identity": None,
                "level": None,
                "observed_distinct_geometry_keys": None,
                "required_distinct_geometry_keys": None,
            },
        )
        deadline_ms = self.load("manifest.json")["exact_config"]["abort_wall_time_seconds"] * 1000
        runs = self.load_jsonl("arm-runs.jsonl")
        active = runs[-1]
        active["cumulative_monotonic_ms"] = deadline_ms
        active["wall_time_ms"] = deadline_ms - active["started_monotonic_ms"]
        self.store_jsonl("arm-runs.jsonl", runs)
        status["total_monotonic_wall_time_ms"] = deadline_ms
        manifest = self.load("manifest.json")
        status["end_unix_ms"] = manifest["start_unix_ms"] + deadline_ms
        self.store("run-status.json", status)
        self.rehash()
        result = verify(self.directory)
        self.assertEqual(result["disposition"], "error")
        self.assertFalse(result["readiness_passed"])
        runs = self.load_jsonl("arm-runs.jsonl")
        runs[-1]["cumulative_monotonic_ms"] = deadline_ms - 200
        runs[-1]["wall_time_ms"] = (
            runs[-1]["cumulative_monotonic_ms"] - runs[-1]["started_monotonic_ms"]
        )
        self.store_jsonl("arm-runs.jsonl", runs)
        status = self.load("run-status.json")
        status["total_monotonic_wall_time_ms"] = deadline_ms - 200
        status["end_unix_ms"] = manifest["start_unix_ms"] + deadline_ms - 200
        self.store("run-status.json", status)
        self.rehash()
        self.assert_corrupt("frozen deadline")

    def test_diversity_terminal_payload_matches_final_completed_level(self):
        level = {
            "level": 0,
            "post_level_distinct_geometry_keys": 7,
        }
        target = {"global_request_index": 32, "candidate_id": "last"}
        terminal = {
            "kind": "post_level_diversity_gate",
            "arm": "adaptive",
            "global_request_index": 32,
            "candidate_id": "last",
            "evaluation_status": None,
            "failure_reason": None,
            "next_schedule_identity": None,
            "level": 0,
            "observed_distinct_geometry_keys": 7,
            "required_distinct_geometry_keys": 8,
        }
        analyze.verify_diversity_terminal_payload(terminal, [level], [target])
        terminal["observed_distinct_geometry_keys"] = 8
        with self.assertRaisesRegex(ArtifactError, "final completed level"):
            analyze.verify_diversity_terminal_payload(terminal, [level], [target])

    def test_strict_json_rejects_duplicate_keys_and_nonstandard_constants(self):
        with self.assertRaisesRegex(ValueError, "duplicate"):
            analyze.strict_json_loads('{"a":1,"a":2}')
        with self.assertRaisesRegex(ValueError, "nonstandard"):
            analyze.strict_json_loads('{"a":NaN}')

    def test_rational_spelling_is_canonical_reduced_positive_denominator(self):
        good = [["0/1", "1/2", "0/1", "0/1"] for _ in range(10)]
        analyze.parse_exact_vertices(good, "fixture")
        for bad in ("2/4", "1/-2", "01/2", "+1/2", "1.0", "-0/1", "0/2"):
            value = copy.deepcopy(good)
            value[0][0] = bad
            with self.assertRaisesRegex(ArtifactError, "noncanonical|invalid rational"):
                analyze.parse_exact_vertices(value, "fixture")

    def test_post_level_population_collapse_cannot_hide_behind_diverse_proposals(self):
        levels = self.load_jsonl("levels.jsonl")
        survivor = levels[0]["post_level_population_candidate_ids"][0]
        key = levels[0]["post_level_population_geometry_keys"][0]
        levels[0]["post_level_population_candidate_ids"] = [survivor] * 16
        levels[0]["post_level_population_geometry_keys"] = [key] * 16
        levels[0]["post_level_distinct_geometry_keys"] = 1
        self.store_jsonl("levels.jsonl", levels)
        self.rehash()
        self.assert_corrupt("population/assignment evidence|eight distinct actual")

    def test_sys_hit_without_stop_event_fails_closed(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        caches = self.load_jsonl("cache.jsonl")
        target = targets[-1]
        target["sys"] = 1.01
        target["capacity"] = (2 * target["volume"] * target["sys"]) ** 0.5
        target["diagnostics"]["action_lower"] = target["capacity"]
        target["diagnostics"]["action_upper"] = target["capacity"]
        for cache in caches:
            if cache["arm"] == target["arm"] and cache["exact_geometry_key"] == target["exact_geometry_key"]:
                cache["sys"] = target["sys"]
                cache["capacity"] = target["capacity"]
                cache["diagnostics"] = copy.deepcopy(target["diagnostics"])
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.store_jsonl("cache.jsonl", caches)
        self.rehash()
        self.assert_corrupt("without a stop event")

    def test_stop_event_must_exactly_match_final_hit(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.hit, self.directory)
        stop = self.load("stop-event.json")
        stop["action"] = "continue"
        self.store("stop-event.json", stop)
        self.rehash()
        self.assert_corrupt("does not exactly match")

    def test_clone_assignments_are_recomputed(self):
        levels = self.load_jsonl("levels.jsonl")
        current = levels[0]["clone_parent_candidate_ids"][0]
        levels[0]["clone_parent_candidate_ids"][0] = next(
            candidate for candidate in levels[0]["survivor_candidate_ids"] if candidate != current
        )
        self.store_jsonl("levels.jsonl", levels)
        self.rehash()
        self.assert_corrupt("population/assignment evidence")

    def test_charged_base_row_order_is_replayed(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0], targets[1] = targets[1], targets[0]
        for index, row in enumerate(targets, 1):
            row["global_request_index"] = index
        for attempt, row in enumerate((row for row in targets if row["arm"] == "adaptive"), 1):
            row["attempt_index"] = attempt
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("frozen global request schedule|charged ledger row")

    def test_mutation_step_cannot_precede_its_parent(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[16], targets[17] = targets[17], targets[16]
        for index, row in enumerate(targets, 1):
            row["global_request_index"] = index
        for attempt, row in enumerate((row for row in targets if row["arm"] == "adaptive"), 1):
            row["attempt_index"] = attempt
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("frozen global request schedule|does not follow its parent|charged ledger row")

    def test_sha_gaussian_mutation_is_recomputed_from_state_before(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        proposal = next(row for row in targets if row["identity"]["level"] is not None)
        proposal["raw_proposed_chart"]["relative_phase"] += 0.01
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("raw mutation|charged ledger row")

    def test_raw_mutation_is_bound_to_resulting_geometry_and_canonical_chart(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        transitions = self.load_jsonl("mutation-transitions.jsonl")
        transition = next(row for row in transitions if not row["accepted"])
        proposal = next(
            row for row in targets if row["candidate_id"] == transition["proposal_candidate_id"]
        )
        self.assertEqual(proposal["cache_status"], "miss")
        adaptive_keys = {
            row["exact_geometry_key"] for row in targets if row["arm"] == "adaptive"
        }
        donor = next(
            row
            for row in targets
            if row["arm"] == "iid"
            and row["evaluation_status"] == "success"
            and row["exact_geometry_key"] not in adaptive_keys
        )
        new_sys = max(0.1, transition["frozen_threshold"] - 0.05)
        new_capacity = (2.0 * donor["volume"] * new_sys) ** 0.5
        geometry_fields = (
            "exact_geometry_key", "geometry_identity", "dual_vertices_rational",
            "dual_vertices_f64", "facet_count", "product_chart",
        )
        for field in geometry_fields:
            proposal[field] = copy.deepcopy(donor[field])
        proposal["capacity"] = new_capacity
        proposal["volume"] = donor["volume"]
        proposal["sys"] = new_sys
        proposal["diagnostics"] = copy.deepcopy(donor["diagnostics"])
        proposal["diagnostics"]["action_lower"] = new_capacity
        proposal["diagnostics"]["action_upper"] = new_capacity
        original_key = next(
            row["exact_geometry_key"]
            for row in self.load_jsonl("charged-requests.jsonl")
            if row["candidate_id"] == proposal["candidate_id"]
        )
        ledger = self.load_jsonl("charged-requests.jsonl")
        charge = next(row for row in ledger if row["candidate_id"] == proposal["candidate_id"])
        for field in geometry_fields:
            charge[field] = copy.deepcopy(proposal[field])
        caches = self.load_jsonl("cache.jsonl")
        cache = next(
            row
            for row in caches
            if row["arm"] == "adaptive" and row["exact_geometry_key"] == original_key
        )
        for field in geometry_fields:
            cache[field] = copy.deepcopy(proposal[field])
        for field in ("capacity", "volume", "sys", "diagnostics", "audit_kind"):
            cache[field] = copy.deepcopy(proposal[field])
        transition["proposal_sys"] = new_sys
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.store_jsonl("charged-requests.jsonl", ledger)
        self.store_jsonl("cache.jsonl", caches)
        self.store_jsonl("mutation-transitions.jsonl", transitions)
        self.rehash()
        self.assert_corrupt("resulting geometry is not decoded from its raw mutation chart")

    def test_successful_retry_requires_all_preceding_rejections(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        target = targets[-1]
        old = target["candidate_id"]
        target["identity"]["construction_attempt"] = 1
        new = analyze.expected_candidate_id(target["identity"])
        target["candidate_id"] = new
        target["root_candidate_id"] = new
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("retry history|charged ledger row")

    def test_exact_product_geometry_is_independently_checked(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0]["dual_vertices_rational"][0][2] = "1/100"
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("product structure|charged ledger row")

    def test_success_volume_is_recomputed_from_exact_product_geometry(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        caches = self.load_jsonl("cache.jsonl")
        target = targets[0]
        target["volume"] *= 1.01
        target["sys"] = target["capacity"] ** 2 / (2 * target["volume"])
        for cache in caches:
            if cache["arm"] == target["arm"] and cache["exact_geometry_key"] == target["exact_geometry_key"]:
                cache["volume"] = target["volume"]
                cache["sys"] = target["sys"]
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.store_jsonl("cache.jsonl", caches)
        self.rehash()
        self.assert_corrupt("volume disagrees with exact product geometry")

    def test_failed_row_cannot_drop_exact_geometry(self):
        shutil.rmtree(self.directory)
        shutil.copytree(self.timeout, self.directory)
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0]["dual_vertices_rational"] = []
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("ten exact dual vertices|charged ledger row")

    def test_artifact_tampering_without_final_hash_refresh_is_rejected(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0]["wall_time_ms"] += 1
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.assert_corrupt("hash mismatch")

    def test_total_time_must_reconcile_with_target_rows(self):
        status = self.load("run-status.json")
        status["total_monotonic_wall_time_ms"] = 0
        self.store("run-status.json", status)
        self.assert_corrupt("target row wall times exceed|wall-clock and monotonic")

    def test_wall_and_monotonic_reconciliation_tolerance_is_bounded(self):
        status = self.load("run-status.json")
        manifest = self.load("manifest.json")
        rounded = round(status["total_monotonic_wall_time_ms"])
        status["end_unix_ms"] = manifest["start_unix_ms"] + rounded + 99
        self.store("run-status.json", status)
        self.assertTrue(verify(self.directory)["verified"])
        status["end_unix_ms"] = manifest["start_unix_ms"] + rounded + 101
        self.store("run-status.json", status)
        self.assert_corrupt("wall-clock and monotonic")

    def test_non_invariant_policy_and_no_probability_claim_are_frozen(self):
        manifest = self.load("manifest.json")
        manifest["tail_probability_supported"] = True
        self.store("manifest.json", manifest)
        self.rehash()
        self.assert_corrupt("claim boundary")

    def test_complete_status_requires_exact_fixed_budget(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        removed = targets.pop()
        self.store_jsonl("target-evaluations.jsonl", targets)
        if removed["cache_status"] == "miss":
            caches = self.load_jsonl("cache.jsonl")
            caches = [
                row
                for row in caches
                if not (
                    row["arm"] == removed["arm"]
                    and row["exact_geometry_key"] == removed["exact_geometry_key"]
                )
            ]
            self.store_jsonl("cache.jsonl", caches)
        arm_runs = self.load_jsonl("arm-runs.jsonl")
        iid = next(row for row in arm_runs if row["arm"] == "iid")
        iid["target_attempts"] -= 1
        iid["cache_misses"] -= int(removed["cache_status"] == "miss")
        iid["cache_hits"] -= int(removed["cache_status"] == "hit")
        remaining_keys = {
            row["exact_geometry_key"]
            for row in targets
            if row["arm"] == "iid" and row["evaluation_status"] == "success"
        }
        iid["distinct_successful_keys"] = len(remaining_keys)
        self.store_jsonl("arm-runs.jsonl", arm_runs)
        status = self.load("run-status.json")
        status["iid_charged_requests"] -= 1
        status["total_charged_requests"] -= 1
        self.store("run-status.json", status)
        self.rehash()
        self.assert_corrupt("48/16 charged budgets|ledger does not reconcile")

    def test_production_launch_refuses_missing_reviewed_commit(self):
        result = self.production_refusal(
            [EXECUTABLE, "production", "--config", CONFIG, "--artifacts", self.directory / "unused"],
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("requires --reviewed-commit", result.stderr)

    def test_production_launch_refuses_wrong_reviewed_commit(self):
        result = self.production_refusal(
            [
                EXECUTABLE,
                "production",
                "--config",
                CONFIG,
                "--artifacts",
                self.directory / "unused",
                "--reviewed-commit",
                "0" * 40,
            ],
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not equal reviewed commit", result.stderr)

    def test_production_launch_refuses_dirty_source(self):
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=HERE, check=True, capture_output=True, text=True
        ).stdout.strip()
        dirty_fixture = HERE / ".ams-dirty-source-refusal-test"
        try:
            dirty_fixture.write_text("intentional untracked dirtiness for refusal test\n")
            result = self.production_refusal(
                [
                    EXECUTABLE,
                    "production",
                    "--config",
                    CONFIG,
                    "--artifacts",
                    self.directory / "unused",
                    "--reviewed-commit",
                    revision,
                ],
                validate_dirty_source=True,
            )
        finally:
            dirty_fixture.unlink(missing_ok=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("dirty or untracked", result.stderr)
        self.assertFalse((self.directory / "unused").exists())

    def test_production_launch_refuses_synthetic_flags(self):
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=HERE, check=True, capture_output=True, text=True
        ).stdout.strip()
        result = self.production_refusal(
            [
                EXECUTABLE,
                "production",
                "--config",
                CONFIG,
                "--artifacts",
                self.directory / "unused",
                "--reviewed-commit",
                revision,
                "--force-synthetic-hit",
            ],
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("synthetic test flags", result.stderr)

    def test_private_refusal_test_guard_cannot_reach_a_target(self):
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=HERE, check=True, capture_output=True, text=True
        ).stdout.strip()
        result = self.production_refusal(
            [
                EXECUTABLE,
                "production",
                "--config",
                CONFIG,
                "--artifacts",
                self.directory / "unused",
                "--reviewed-commit",
                revision,
            ],
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("AMS_TEST_REFUSAL_ONLY safety guard", result.stderr)
        self.assertFalse((self.directory / "unused").exists())

    def test_private_guard_also_blocks_direct_production_child_endpoint(self):
        environment = os.environ.copy()
        environment["AMS_TEST_REFUSAL_ONLY"] = "1"
        payload = {
            "mode": "production",
            "exact_geometry_key": "guard-fixture",
            "dual_vertices_f64": [],
            "synthetic_force_hit": False,
            "synthetic_force_failure": False,
            "synthetic_validate_constructor": False,
            "synthetic_delay_ms": 0,
            "synthetic_response_padding_bytes": 0,
        }
        result = subprocess.run(
            [EXECUTABLE, "target-once"],
            cwd=HERE,
            input=json.dumps(payload),
            capture_output=True,
            text=True,
            env=environment,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("production target child disabled", result.stderr)

    def test_external_parent_termination_retains_charge_and_kills_child(self):
        artifacts = Path(self.temp.name) / "interrupted"
        process = subprocess.Popen(
            [
                EXECUTABLE,
                "synthetic",
                "--config",
                CONFIG,
                "--artifacts",
                artifacts,
                "--synthetic-child-delay-ms",
                "5000",
            ],
            cwd=HERE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        child_pid = None
        deadline = time.monotonic() + 5.0
        ledger = artifacts / "charged-requests.jsonl"
        while time.monotonic() < deadline:
            children_path = Path(f"/proc/{process.pid}/task/{process.pid}/children")
            if ledger.exists() and ledger.read_text().count("\n") == 1 and children_path.exists():
                children = children_path.read_text().split()
                if children:
                    child_pid = int(children[0])
                    break
            time.sleep(0.01)
        self.assertIsNotNone(child_pid, "synthetic child was not observed after durable charge")
        os.kill(process.pid, signal.SIGKILL)
        process.wait(timeout=5)
        process.communicate(timeout=1)
        assert child_pid is not None
        child_deadline = time.monotonic() + 2.0
        while time.monotonic() < child_deadline and Path(f"/proc/{child_pid}").exists():
            state = Path(f"/proc/{child_pid}/stat").read_text().split()[2]
            if state == "Z":
                break
            time.sleep(0.01)
        if Path(f"/proc/{child_pid}").exists():
            self.assertEqual(Path(f"/proc/{child_pid}/stat").read_text().split()[2], "Z")
        result = verify(artifacts)
        self.assertEqual(result["disposition"], "externally_interrupted")
        self.assertEqual(result["adaptive_attempts"], 0)
        self.assertEqual(result["ledger_charged_requests"], 1)
        self.assertEqual(result["outcome_unknown_requests"], 1)
        self.assertFalse(result["readiness_passed"])

    def test_reused_output_directory_is_fail_closed(self):
        result = self.run_packet(self.directory, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("already exists", result.stderr)


if __name__ == "__main__":
    unittest.main()
