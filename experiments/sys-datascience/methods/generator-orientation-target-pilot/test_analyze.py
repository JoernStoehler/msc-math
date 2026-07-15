import unittest
from analyze import primary_disposition, rank_average, spearman

class FrozenAnalyzerCalibration(unittest.TestCase):
    def test_positive_negative_and_ties(self):
        self.assertEqual(rank_average([1, 1, 2]), [1.5, 1.5, 3.0])
        self.assertAlmostEqual(spearman([1, 2, 3], [1, 2, 3]), 1.0)
        self.assertAlmostEqual(spearman([1, 2, 3], [3, 2, 1]), -1.0, places=12)
        self.assertLess(spearman([1, 1, 2], [3, 2, 1]), -0.8)

    def test_heterogeneous_sign_fixture(self):
        values = [1, -1, 1, -1, 1, -1, 1, -1]
        self.assertEqual(sum(v > 0 for v in values), 4)
        self.assertTrue(max(sum(v > 0 for v in values), sum(v < 0 for v in values)) < 7)

    def test_incomplete_fixture_is_not_a_complete_grid(self):
        self.assertNotEqual(len({"identity", "u2-haar"}), 3)

    def test_primary_positive_negative_and_u2_failure(self):
        self.assertEqual(primary_disposition(0.0, [0.02] * 6 + [0.0, 0.0]), "supports_material_alignment_role")
        self.assertEqual(primary_disposition(0.0, [0.001] * 8), "contradicts_material_role_on_frozen_maps")
        self.assertEqual(primary_disposition(2e-8, [0.2] * 8), "ambiguous_numerical_control_failure")

if __name__ == "__main__": unittest.main()
