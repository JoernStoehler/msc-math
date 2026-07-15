import copy
import hashlib
import json
import shutil
import subprocess
import tempfile
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

    def assert_corrupt(self, pattern):
        with self.assertRaisesRegex(ArtifactError, pattern):
            verify(self.directory)

    def test_normal_fixture_passes_only_readiness_gate(self):
        result = verify(self.directory)
        self.assertTrue(result["verified"])
        self.assertTrue(result["readiness_passed"])
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
        self.assert_corrupt("frozen global request schedule")

    def test_mutation_step_cannot_precede_its_parent(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[16], targets[17] = targets[17], targets[16]
        for index, row in enumerate(targets, 1):
            row["global_request_index"] = index
        for attempt, row in enumerate((row for row in targets if row["arm"] == "adaptive"), 1):
            row["attempt_index"] = attempt
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("frozen global request schedule|does not follow its parent")

    def test_sha_gaussian_mutation_is_recomputed_from_state_before(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        proposal = next(row for row in targets if row["identity"]["level"] is not None)
        proposal["raw_proposed_chart"]["relative_phase"] += 0.01
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("raw mutation")

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
        self.assert_corrupt("retry history")

    def test_exact_product_geometry_is_independently_checked(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0]["dual_vertices_rational"][0][2] = "1/100"
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.rehash()
        self.assert_corrupt("product structure")

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
        self.assert_corrupt("ten exact dual vertices")

    def test_artifact_tampering_without_final_hash_refresh_is_rejected(self):
        targets = self.load_jsonl("target-evaluations.jsonl")
        targets[0]["wall_time_ms"] += 1
        self.store_jsonl("target-evaluations.jsonl", targets)
        self.assert_corrupt("hash mismatch")

    def test_total_time_must_reconcile_with_target_rows(self):
        status = self.load("run-status.json")
        status["total_monotonic_wall_time_ms"] = 0
        self.store("run-status.json", status)
        self.assert_corrupt("target row wall times exceed")

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
        self.assert_corrupt("48/16 charged budgets")

    def test_production_launch_refuses_missing_reviewed_commit(self):
        result = subprocess.run(
            [EXECUTABLE, "production", "--config", CONFIG, "--artifacts", self.directory / "unused"],
            cwd=HERE,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("requires --reviewed-commit", result.stderr)

    def test_production_launch_refuses_wrong_reviewed_commit(self):
        result = subprocess.run(
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
            cwd=HERE,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not equal reviewed commit", result.stderr)

    def test_production_launch_refuses_dirty_source(self):
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=HERE, check=True, capture_output=True, text=True
        ).stdout.strip()
        result = subprocess.run(
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
            cwd=HERE,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("dirty or untracked", result.stderr)

    def test_production_launch_refuses_synthetic_flags(self):
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=HERE, check=True, capture_output=True, text=True
        ).stdout.strip()
        result = subprocess.run(
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
            cwd=HERE,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("synthetic test flags", result.stderr)

    def test_reused_output_directory_is_fail_closed(self):
        result = self.run_packet(self.directory, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("already exists", result.stderr)


if __name__ == "__main__":
    unittest.main()
