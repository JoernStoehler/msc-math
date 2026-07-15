import unittest
from analyze import rank_average, spearman

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

if __name__ == "__main__": unittest.main()
