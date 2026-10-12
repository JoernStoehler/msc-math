"""Evaluator-provenance tests requiring no frozen source or target execution."""
import unittest
from unittest.mock import patch

import analyze


class AnalysisIdentityTests(unittest.TestCase):
    def data(self, identity):
        return [dict(identity, candidate_id="fixture", sys=0.5)]

    def report(self, identity):
        estimate = {
            "bucket_effects": {bucket: {"effect": 0.1} for bucket in ("4x6", "6x6")},
            "equal_bucket_effect": 0.1,
        }
        with (
            patch.object(analyze, "estimand", return_value=estimate),
            patch.object(analyze, "bootstrap", return_value=[0.1]),
            patch.object(analyze, "permutation", return_value={}),
        ):
            return analyze.summarize(
                self.data(identity), {"fixture": {"memberships": []}}, "fixture-hash"
            )

    def test_changed_identity_is_not_relabelled_as_retained(self):
        identity = dict(
            analyze.EVALUATOR_IDENTITY,
            evaluator_git_commit="different-revision",
            evaluator_git_clean=False,
        )
        self.assertEqual(self.report(identity)["evaluator_identity"], identity)
        self.assertNotEqual(identity, analyze.EVALUATOR_IDENTITY)

    def test_retained_identity_is_unchanged(self):
        self.assertEqual(
            self.report(analyze.EVALUATOR_IDENTITY)["evaluator_identity"],
            analyze.EVALUATOR_IDENTITY,
        )

    def test_mixed_identities_are_rejected_before_resampling(self):
        data = self.data(analyze.EVALUATOR_IDENTITY)
        data += self.data(
            dict(analyze.EVALUATOR_IDENTITY, evaluator_backend_sha256="other")
        )
        with patch.object(analyze, "bootstrap") as bootstrap:
            with self.assertRaisesRegex(ValueError, "mixed target evaluator"):
                analyze.summarize(data, {}, "fixture-hash")
            bootstrap.assert_not_called()

    def test_empty_rows_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "empty target"):
            analyze.recorded_evaluator_identity([])


if __name__ == "__main__":
    unittest.main()
