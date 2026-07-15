import copy
import json
import tempfile
import unittest
from pathlib import Path

from analyze import ArtifactError, verify


class AnalyzerCorruptionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.directory = Path(self.temp.name)
        config = {
            "packet_version": "ams-readiness-smoke-v1",
            "master_seed": 1,
            "replicate": 0,
            "initial_particles": 16,
            "levels": 2,
            "survivors_per_level": 8,
            "clones_per_level": 8,
            "mutation_steps_per_clone": 2,
            "iid_requests": 16,
            "construction_retry_cap": 64,
            "abort_wall_time_seconds": 600,
            "gap_logit_scale": 0.08,
            "centered_log_radius_scale": 0.04,
            "phase_scale": 0.08,
            "tie_rule": "sys_desc_candidate_id_asc",
            "clone_assignment": "seeded_uniform_with_replacement",
            "acceptance_rule": "successful_sys_at_least_frozen_level_threshold",
            "factor_exchange_quotiented": False,
        }
        import analyze

        self.manifest = {
            "artifact_kind": "synthetic_target_free",
            "config_identity": analyze.sha256(analyze.compact_json(config)),
            "exact_config": config,
            "source": {
                "git_revision": "test",
                "source_tree_clean": False,
                "executable_sha256": "0" * 64,
                "cargo_lock_sha256": "1" * 64,
                "production_target": False,
            },
            "adaptive_budget": 48,
            "iid_budget": 16,
            "target_probability_estimate": None,
            "factor_exchange_quotiented": False,
        }

    def tearDown(self):
        self.temp.cleanup()

    def write_minimal_corrupt_packet(self):
        (self.directory / "manifest.json").write_text(json.dumps(self.manifest))
        for name in (
            "target-evaluations.jsonl",
            "cache.jsonl",
            "construction-rejections.jsonl",
            "mutation-transitions.jsonl",
            "levels.jsonl",
            "arm-runs.jsonl",
        ):
            (self.directory / name).write_text("")

    def test_missing_fixed_budget_rows_fails_closed(self):
        self.write_minimal_corrupt_packet()
        with self.assertRaisesRegex(ArtifactError, "complete smoke"):
            verify(self.directory)

    def test_probability_claim_is_rejected(self):
        self.manifest["target_probability_estimate"] = 0.01
        self.write_minimal_corrupt_packet()
        with self.assertRaisesRegex(ArtifactError, "probability estimate"):
            verify(self.directory)

    def test_dirty_production_manifest_is_rejected(self):
        self.manifest["artifact_kind"] = "production_target"
        self.manifest["source"]["production_target"] = True
        self.manifest["source"]["source_tree_clean"] = False
        self.write_minimal_corrupt_packet()
        with self.assertRaisesRegex(ArtifactError, "dirty source tree"):
            verify(self.directory)

    def test_config_corruption_is_detected(self):
        self.write_minimal_corrupt_packet()
        changed = copy.deepcopy(self.manifest)
        changed["exact_config"]["phase_scale"] = 9.0
        (self.directory / "manifest.json").write_text(json.dumps(changed))
        with self.assertRaisesRegex(ArtifactError, "config_identity"):
            verify(self.directory)


if __name__ == "__main__":
    unittest.main()
