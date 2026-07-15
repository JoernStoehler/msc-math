#!/usr/bin/env python3
"""Discriminating exact controls for the bounded quotient-distance packet."""

from __future__ import annotations

import copy
import importlib.util
import itertools
import sys
import unittest
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).parent
SPEC = importlib.util.spec_from_file_location("quotient_distance", HERE / "quotient_distance.py")
assert SPEC and SPEC.loader
quotient_distance = importlib.util.module_from_spec(SPEC)
sys.modules["quotient_distance"] = quotient_distance
SPEC.loader.exec_module(quotient_distance)


class QuotientDistanceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.configurations = quotient_distance.smoke_configurations()
        cls.base = cls.configurations["base"]

    def distance(self, left, right):
        result = quotient_distance.quotient_distance(left, right)
        self.assertEqual(result.status, "exact")
        self.assertTrue(result.exact)
        self.assertEqual(result.evaluated_permutations, 40320)
        return result

    def test_identical_labeled_and_permuted_zero(self):
        labeled = quotient_distance.squared_frobenius(
            quotient_distance.gram(self.base.duals),
            quotient_distance.gram(self.base.duals),
        )
        self.assertEqual(labeled, 0)
        self.assertEqual(self.distance(self.base, self.base).squared_distance, 0)
        permuted = self.configurations["permuted"]
        self.assertNotEqual(
            quotient_distance.gram(self.base.duals),
            quotient_distance.gram(permuted.duals),
        )
        self.assertEqual(self.distance(self.base, permuted).squared_distance, 0)

    def test_nonorthogonal_symplectic_zero(self):
        symplectic = quotient_distance.matrix(
            (
                (2, 0, 0, 0),
                (0, Fraction(1, 3), 0, 0),
                (0, 0, Fraction(1, 2), 0),
                (0, 0, 0, 3),
            )
        )
        self.assertTrue(quotient_distance.is_symplectic(symplectic))
        self.assertFalse(quotient_distance.is_orthogonal(symplectic))
        self.assertEqual(
            self.distance(self.base, self.configurations["nonorthogonal_symplectic"]).squared_distance,
            0,
        )

    def test_translation_and_positive_scale_normalization(self):
        transformed = self.configurations["translated_scaled"]
        self.assertEqual(transformed.exact_volume, Fraction(81))
        self.assertEqual(transformed.volume_quarter_root, Fraction(3))
        self.assertEqual(self.base.duals, transformed.duals)
        self.assertEqual(self.distance(self.base, transformed).squared_distance, 0)

    def test_so4_outside_u2_and_nonsymplectic_gl_are_nonzero(self):
        so4 = self.configurations["so4_outside_u2"]
        gl = self.configurations["nonsymplectic_gl"]
        self.assertGreater(self.distance(self.base, so4).squared_distance, 0)
        self.assertGreater(self.distance(self.base, gl).squared_distance, 0)

    def test_unequal_facet_counts_fail_closed(self):
        simplex_rows = tuple(
            quotient_distance.vector(row)
            for row in (
                (1, 0, 0, 0),
                (0, 1, 0, 0),
                (0, 0, 1, 0),
                (0, 0, 0, 1),
                (-1, -1, -1, -1),
            )
        )
        simplex = quotient_distance.NormalizedConfiguration(
            quotient_distance.validate_normalized_configuration(simplex_rows),
            "caller-certified analytic center",
            "caller-certified volume one",
            Fraction(1),
            Fraction(1),
        )
        with self.assertRaisesRegex(quotient_distance.ContractError, "unequal_facet_counts"):
            quotient_distance.quotient_distance(self.base, simplex)

    def test_redundant_and_degenerate_inputs_are_rejected(self):
        duplicate = self.base.duals + (self.base.duals[0],)
        with self.assertRaisesRegex(quotient_distance.ContractError, "duplicate"):
            quotient_distance.validate_normalized_configuration(duplicate)
        degenerate = tuple(
            row for row in self.base.duals if row[3] == 0
        )
        with self.assertRaisesRegex(
            quotient_distance.ContractError, "span|bounded|facets"
        ):
            quotient_distance.validate_normalized_configuration(degenerate)

        presentation = quotient_distance.base_cube()
        malformed = quotient_distance.Presentation(
            presentation.facets[:-1] + (presentation.facets[0],),
            presentation.center,
        )
        with self.assertRaisesRegex(quotient_distance.ContractError, "opposite|duplicate"):
            quotient_distance.normalize_parallelotope(malformed)

    def test_near_symmetry_and_timeout_statuses_are_explicit(self):
        near_configuration = self.configurations["near_symmetry"]
        near = self.distance(near_configuration, near_configuration)
        self.assertEqual(near.squared_distance, 0)
        self.assertTrue(near.near_symmetry)
        self.assertTrue(near.multiple_minimizers)

        timeout = quotient_distance.quotient_distance(
            self.base, self.base, timeout_seconds=0
        )
        self.assertEqual(timeout.status, "timeout")
        self.assertFalse(timeout.exact)
        self.assertIsNone(timeout.squared_distance)
        self.assertLess(timeout.evaluated_permutations, timeout.total_permutations)

    def test_bound_above_eight_facets_fails_closed(self):
        over_bound = copy.copy(self.base)
        over_bound = quotient_distance.NormalizedConfiguration(
            over_bound.duals + (quotient_distance.vector((1, 1, 1, 1)),),
            over_bound.center_convention,
            over_bound.scale_convention,
            over_bound.exact_volume,
            over_bound.volume_quarter_root,
        )
        with self.assertRaisesRegex(quotient_distance.ContractError, "exact_bound"):
            quotient_distance.quotient_distance(over_bound, over_bound)

    def test_triangle_inequality_on_enumerated_fixture_set(self):
        names = ("base", "so4_outside_u2", "nonsymplectic_gl")
        pair_distances = {}
        for left, right in itertools.combinations(names, 2):
            result = self.distance(self.configurations[left], self.configurations[right])
            self.assertIsNotNone(result.squared_distance)
            pair_distances[frozenset((left, right))] = result.squared_distance

        for left, middle, right in itertools.permutations(names, 3):
            self.assertTrue(
                quotient_distance.triangle_holds(
                    pair_distances[frozenset((left, middle))],
                    pair_distances[frozenset((middle, right))],
                    pair_distances[frozenset((left, right))],
                )
            )


if __name__ == "__main__":
    unittest.main()
