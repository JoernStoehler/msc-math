#!/usr/bin/env python3

import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("distribution_contract", HERE / "contract.py")
contract = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(contract)
MAKE_SPEC = importlib.util.spec_from_file_location("make_fixtures", HERE / "make_fixtures.py")
make_fixtures = importlib.util.module_from_spec(MAKE_SPEC)
assert MAKE_SPEC.loader is not None
MAKE_SPEC.loader.exec_module(make_fixtures)
ADAPT_SPEC = importlib.util.spec_from_file_location("adapt_orientation", HERE / "adapt_orientation.py")
adapt_orientation = importlib.util.module_from_spec(ADAPT_SPEC)
assert ADAPT_SPEC.loader is not None
ADAPT_SPEC.loader.exec_module(adapt_orientation)


class ContractTests(unittest.TestCase):
    def fixture(self):
        temporary = tempfile.TemporaryDirectory()
        path = Path(temporary.name) / "records"
        make_fixtures.make(path)
        return temporary, path

    def test_calibration_laws_and_units(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        report = contract.audit(contract.load_bundle(path))
        laws = {row["law_run_id"]: row for row in report["laws"]}
        self.assertEqual(laws["run/null-law/default"]["independent_units"], 12)
        self.assertEqual(laws["run/null-law/default"]["heldout_new_signature_rate"], 0.0)
        self.assertEqual(laws["run/collapsed-law/default"]["exact_duplicate_groups"], 1)
        self.assertEqual(laws["run/collapsed-law/default"]["exact_duplicate_multiplicities"], [8])
        self.assertEqual(laws["run/rare-mixture/default"]["singleton_signature_count"], 0)
        self.assertEqual(laws["run/rare-mixture/default"]["doubleton_signature_count"], 1)
        self.assertEqual(laws["run/censored-law/default"]["acceptance_rate"], None)
        self.assertIn("unknown/censored", " ".join(laws["run/censored-law/default"]["diagnostics"]))
        self.assertEqual(laws["run/paired-law/default"]["recorded_event_rows"], 8)
        self.assertIsNone(laws["run/paired-law/default"]["attempt_count"])
        self.assertIsNone(laws["run/paired-law/default"]["acceptance_rate"])
        self.assertIsNone(laws["run/paired-law/default"]["cost_ms_per_attempt"])
        self.assertEqual(laws["run/paired-law/default"]["independent_units"], 4)
        self.assertIn("dependent block", " ".join(laws["run/paired-law/default"]["diagnostics"]))
        self.assertIn("attempt log incomplete", " ".join(laws["run/missing-provenance/default"]["diagnostics"]))
        self.assertIsNone(laws["run/missing-provenance/default"]["unseen_signature_mass_good_turing"])
        # Accepted-only provenance cannot support Good--Turing even when every
        # unit has one visible signature (mutate a complete null run).
        run_rows = [json.loads(line) for line in (path / "law_runs.jsonl").read_text().splitlines()]
        null_run = next(row for row in run_rows if row["law_run_id"] == "run/null-law/default")
        null_run["attempt_log_status"] = "accepted_only"
        (path / "law_runs.jsonl").write_text("".join(json.dumps(row) + "\n" for row in run_rows))
        unavailable = contract.audit(contract.load_bundle(path))
        null_report = next(row for row in unavailable["laws"] if row["law_run_id"] == "run/null-law/default")
        self.assertIsNone(null_report["unseen_signature_mass_good_turing"])
        self.assertEqual(null_report["unseen_signature_mass_status"], "not-identifiable-attempt-provenance")
        # Same law_id is intentionally represented by two non-pooled runs.
        stratified = [row for row in report["laws"] if row["law_id"] == "stratified-law"]
        self.assertEqual({row["law_run_id"] for row in stratified}, {"run/stratified-law/a", "run/stratified-law/b"})
        self.assertEqual([row["recorded_event_rows"] for row in stratified], [3, 3])
        self.assertEqual([row["independent_units"] for row in stratified], [3, 3])

    def test_forbidden_target_field_fails_closed(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        events = path / "sampling_events.jsonl"
        rows = [json.loads(line) for line in events.read_text().splitlines()]
        rows[0]["sys"] = None
        events.write_text("".join(json.dumps(row) + "\n" for row in rows))
        with self.assertRaisesRegex(contract.ContractError, "forbidden target field"):
            contract.load_bundle(path)

    def test_input_order_does_not_change_report(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        first = contract.audit(contract.load_bundle(path))
        events = path / "sampling_events.jsonl"
        rows = [json.loads(line) for line in events.read_text().splitlines()]
        events.write_text("".join(json.dumps(row, sort_keys=True) + "\n" for row in reversed(rows)))
        second = contract.audit(contract.load_bundle(path))
        self.assertEqual(first, second)

    def test_truncated_record_and_unknown_join_fail(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        events = path / "sampling_events.jsonl"
        original = events.read_text()
        events.write_text(original + '{"schema":"generator-distribution-sampling-event-v1"\n')
        with self.assertRaisesRegex(contract.ContractError, "invalid JSON"):
            contract.load_bundle(path)
        events.write_text(original)
        rows = [json.loads(line) for line in events.read_text().splitlines()]
        rows[0]["geometry_id"] = "does-not-exist"
        events.write_text("".join(json.dumps(row) + "\n" for row in rows))
        with self.assertRaisesRegex(contract.ContractError, "unknown geometry"):
            contract.load_bundle(path)

    def test_malformed_incidence_is_rejected(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        geometry_path = path / "geometry_views.jsonl"
        rows = [json.loads(line) for line in geometry_path.read_text().splitlines()]
        for malformed in ([], [[]], [[99]]):
            rows[0]["incidence_signature"] = malformed
            geometry_path.write_text("".join(json.dumps(row) + "\n" for row in rows))
            with self.subTest(malformed=malformed), self.assertRaisesRegex(contract.ContractError, "incidence_signature"):
                contract.load_bundle(path)

    def test_cost_missingness_and_unit_kind_conflict_fail_closed(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        events_path = path / "sampling_events.jsonl"
        rows = [json.loads(line) for line in events_path.read_text().splitlines()]
        # Cost denominator is not silently treated as zero.
        censored = contract.audit(contract.load_bundle(path))
        censor = next(row for row in censored["laws"] if row["law_run_id"] == "run/censored-law/default")
        self.assertIsNone(censor["mean_processing_cost_ms_per_accepted_exact_event"])
        self.assertEqual(censor["mean_processing_cost_ms_per_accepted_exact_event_status"], "unidentifiable_missing_cost_or_exact_geometry")
        rejection = next(row for row in censored["laws"] if row["law_run_id"] == "run/rejection-cost-law/default")
        self.assertEqual(rejection["mean_processing_cost_ms_per_accepted_exact_event"], 1.0)
        self.assertEqual(rejection["total_recorded_sampling_cost_ms_per_accepted_exact_result"], 101.0)
        self.assertEqual(rejection["total_recorded_sampling_cost_ms_per_accepted_exact_result_status"], "identifiable_complete_uncensored_iid_log_and_cost")
        # A unit cannot change from IID to paired within a report group.
        target = next(row for row in rows if row["law_run_id"] == "run/paired-law/default")
        target["independent_unit_kind"] = "iid_draw"
        events_path.write_text("".join(json.dumps(row) + "\n" for row in rows))
        with self.assertRaisesRegex(contract.ContractError, "inconsistent kinds"):
            contract.audit(contract.load_bundle(path))

    def test_iid_attempt_and_unit_repeats_fail_closed(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        events_path = path / "sampling_events.jsonl"
        rows = [json.loads(line) for line in events_path.read_text().splitlines()]
        null_rows = [row for row in rows if row["law_run_id"] == "run/null-law/default"]
        null_rows[1]["attempt_id"] = null_rows[0]["attempt_id"]
        events_path.write_text("".join(json.dumps(row) + "\n" for row in rows))
        with self.assertRaisesRegex(contract.ContractError, "repeats attempt_id"):
            contract.load_bundle(path)
        null_rows[1]["attempt_id"] = "null-unit-01/attempt=0"
        null_rows[1]["independent_unit_id"] = null_rows[0]["independent_unit_id"]
        events_path.write_text("".join(json.dumps(row) + "\n" for row in rows))
        with self.assertRaisesRegex(contract.ContractError, "more than one sampling event"):
            contract.load_bundle(path)

    def test_metric_join_checked_even_when_accepted_geometry_missing(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        events_path = path / "sampling_events.jsonl"
        rows = [json.loads(line) for line in events_path.read_text().splitlines()]
        target = next(row for row in rows if row["event_id"] == "missing-0")
        target["metric_view_id"] = "not-present"
        events_path.write_text("".join(json.dumps(row) + "\n" for row in rows))
        with self.assertRaisesRegex(contract.ContractError, "unknown metric_view_id"):
            contract.load_bundle(path)

    def test_declared_view_and_event_metric_joins_fail_closed(self):
        temporary, path = self.fixture()
        self.addCleanup(temporary.cleanup)
        geometry_path = path / "geometry_views.jsonl"
        geometry_rows = [json.loads(line) for line in geometry_path.read_text().splitlines()]
        geometry_rows[0]["view_id"] = "wrong-view"
        geometry_path.write_text("".join(json.dumps(row) + "\n" for row in geometry_rows))
        with self.assertRaisesRegex(contract.ContractError, "unknown metric/preprocessing view"):
            contract.load_bundle(path)
        # Restore geometry and make an event disagree with its declared view.
        make_fixtures.make(path)
        events_path = path / "sampling_events.jsonl"
        event_rows = [json.loads(line) for line in events_path.read_text().splitlines()]
        event_rows[0]["metric_view_id"] = "wrong-view"
        # Add a valid metric row so this exercises agreement, not unknown-ID.
        metrics_path = path / "metric_preprocessing.jsonl"
        metrics = [json.loads(line) for line in metrics_path.read_text().splitlines()]
        metrics.append({"schema": contract.SCHEMAS["metrics"], "metric_view_id": "wrong-view", "metric_version": "v1", "preprocessing_id": "none", "fit_reference": "none", "fit_split": "none"})
        metrics_path.write_text("".join(json.dumps(row) + "\n" for row in metrics))
        events_path.write_text("".join(json.dumps(row) + "\n" for row in event_rows))
        with self.assertRaisesRegex(contract.ContractError, "disagrees with geometry view_id"):
            contract.load_bundle(path)

    def test_outside_source_gets_explicit_stable_provenance(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / "copied-source.jsonl"
            source.write_text("{}\n")
            label, revision = adapt_orientation._source_provenance(source, HERE.parents[3])
        self.assertTrue(label.startswith("/tmp/") or label.startswith("/private/tmp/"))
        self.assertEqual(revision, "outside-repository/not-recorded")
        label, revision = adapt_orientation._source_provenance(source, HERE.parents[3], "supplied-r1")
        self.assertEqual(revision, "supplied-r1")

    def test_adapter_run_semantics_conflict_fails_closed(self):
        base = {
            "map_variant": "identity", "map_family": "identity", "map_mode": "identity",
            "bucket": "3x3", "coordinate_order": "q1,q2,p1,p2", "q_sides": 3,
            "p_sides": 3, "base_facet_count": 6, "base_vertex_count": 9,
            "facet_count": 6, "vertex_count": 9,
        }
        other = dict(base, map_mode="corrupt")
        with self.assertRaisesRegex(ValueError, "map_mode"):
            adapt_orientation._check_run_semantics(base, other, "run/example")


if __name__ == "__main__":
    unittest.main()
