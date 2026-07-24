import json
import unittest
from fractions import Fraction as F
from pathlib import Path

import analyze


ROOT = Path(__file__).parents[4]
RAW = ROOT / "experiments" / "polytope-datasets" / "random-product.jsonl"


class ExactTriangleTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        with RAW.open() as stream:
            row = json.loads(stream.readline())
        cls.duals = analyze.parse_duals(row)

    def test_barycentric_positive_and_closes(self):
        for offset in (0, 2):
            ids = range(3) if offset == 0 else range(3, 6)
            weights = analyze.barycentric(self.duals, offset)
            self.assertTrue(all(x > 0 for x in weights))
            self.assertEqual(sum(weights, F(0)), F(1))
            for d in (0, 1):
                self.assertEqual(
                    sum((weights[j] * self.duals[i][offset + d] for j, i in enumerate(ids)), F(0)), F(0)
                )

    def test_difference_body_formula_matches_halfspace_enumeration(self):
        self.assertEqual(analyze.a2_exact(self.duals), analyze.a2_exact_geometry(self.duals))

    def test_stored_first_word_reproduces_action(self):
        signs = [[1 if analyze.omega(self.duals[i], self.duals[3 + j]) > 0 else -1
                  for j in range(3)] for i in range(3)]
        words = analyze.cycle_words(signs)
        self.assertIn((0, 3, 2, 5, 1, 4), words)
        qbar = analyze.barycentric(self.duals, 0)
        pbar = analyze.barycentric(self.duals, 2)
        _, q, action = analyze.word_qp(self.duals, (0, 3, 2, 5, 1, 4), qbar, pbar)
        self.assertGreater(q, 0)
        self.assertAlmostEqual(float(action), 6.022362590631931, places=12)

    def test_zero_pairing_is_boundary_not_strict(self):
        d = [list(x) for x in self.duals]
        d[3] = tuple(d[3])
        # Make one p vertex exactly symplectically orthogonal to q_0 while
        # retaining a triangle fixture for sign-stratum handling.
        d[3] = (F(0), F(0), d[0][1], -d[0][0])
        signs = [[(analyze.omega(d[i], d[3 + j]) > 0) - (analyze.omega(d[i], d[3 + j]) < 0)
                  for j in range(3)] for i in range(3)]
        self.assertTrue(any(x == 0 for row in signs for x in row))
        self.assertFalse(all(x != 0 for row in signs for x in row))


if __name__ == "__main__":
    unittest.main()
