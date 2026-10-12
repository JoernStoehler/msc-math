#!/usr/bin/env python3
"""Synthetic producer-contract checks; no capacity computation or retained data writes.

Run: python3 -m unittest discover -s topics/empirical-studies/polytope-datasets/tests -v
"""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


VALIDATOR_PATH = Path(__file__).resolve().parents[1] / "validate-datascience-produced.py"
SPEC = importlib.util.spec_from_file_location("validate_datascience_produced", VALIDATOR_PATH)
assert SPEC is not None and SPEC.loader is not None
VALIDATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VALIDATOR)


def fixture() -> dict:
    payloads = []
    for poly_id, facets, backend in (("random", 8, "auto"), ("product", 7, "product"), ("reference", 10, "product")):
        payloads.append({
            "poly_id": poly_id,
            "facet_count": facets,
            "dual_vertices": [[1.0, 0.0, 0.0, 0.0]] * facets,
            "dual_vertices_rational": [["1", "0", "0", "0"]] * facets,
            "backend": backend,
            "sys": 0.5,
            "capacity": 1.0,
            "volume": 1.0,
            "capacity_lower": 0.99999999999,
            "capacity_upper": 1.00000000001,
            "capacity_exact": "1",
            "capacity_method": "certified-qp-minimizers-v1",
            "candidate_family": "general-hk" if backend == "auto" else "product-closure-vertex",
            "sigmas": [{"perm": [0, 1], "action": 1.0}],
        })
    source = {"h_min": 0.8, "h_max": 1.2, "seed": 42, "sample_index": 0, "attempt": 0}
    return {
        "computed-polytopes.jsonl": payloads,
        "random-samples.jsonl": [{"name": "random", "poly_id": "random", "sys": 0.5,
                                  "source": {**source, "producer": "random", "facet_count": 8}}],
        "random-product-samples.jsonl": [{"name": "product", "poly_id": "product", "sys": 0.5,
                                          "facet_count": 7,
                                          "source": {**source, "producer": "random-product", "k": 3, "m": 4, "bounces": 2}}],
        "reference-samples.jsonl": [{"name": "reference", "poly_id": "reference", "sys": 0.5,
                                    "source": {"producer": "known-hko-reference", "fixture": "hko_pentagon", "role": "reference_holdout"}}],
        "produce-stats.json": {"random_rows": 1, "random_product_rows": 1, "reference_rows": 1,
                               "computed_payload_rows": 3, "failures": 0, "cache_hits": 0,
                               "cache_misses": 3, "mode": "smoke", "max_sys": 0.5,
                               "producers": list(VALIDATOR.PRODUCER_FILES), "cache_miss_volume_ms": 0.0,
                               "cache_miss_capacity_ms": 0.0, "wall_time_ms": 1.0},
    }


class ProducerValidationTests(unittest.TestCase):
    def validate_fixture(self, data: dict) -> dict:
        with tempfile.TemporaryDirectory(prefix="producer-contract-") as directory:
            path = Path(directory)
            for filename, content in data.items():
                if filename.endswith(".jsonl"):
                    (path / filename).write_text("".join(json.dumps(row) + "\n" for row in content))
                else:
                    (path / filename).write_text(json.dumps(content))
            return VALIDATOR.validate(path, "smoke", ",".join(VALIDATOR.PRODUCER_FILES), 1, 1)

    def test_valid_all_producer_shapes(self) -> None:
        result = self.validate_fixture(fixture())
        self.assertEqual(result["computed_payload_rows"], 3)
        self.assertEqual(result["sys_gt_one"], 0)

    def test_reject_nonfinite_or_nonpositive_payload_values(self) -> None:
        for field in ("sys", "capacity", "volume", "capacity_lower", "capacity_upper"):
            for value in (float("inf"), float("-inf"), float("nan"), 0.0, -1.0, None, True, "1"):
                with self.subTest(field=field, value=value):
                    data = fixture()
                    data["computed-polytopes.jsonl"][0][field] = value
                    with self.assertRaisesRegex(SystemExit, field):
                        self.validate_fixture(data)

    def test_reject_nonfinite_sample_sys_and_action(self) -> None:
        for value in (float("inf"), float("-inf"), float("nan")):
            with self.subTest(value=value):
                data = fixture()
                data["random-samples.jsonl"][0]["sys"] = value
                with self.assertRaisesRegex(SystemExit, "sys"):
                    self.validate_fixture(data)
                data = fixture()
                data["computed-polytopes.jsonl"][0]["sigmas"][0]["action"] = value
                with self.assertRaisesRegex(SystemExit, "action"):
                    self.validate_fixture(data)

    def test_reject_invalid_capacity_bounds_and_action(self) -> None:
        for field, value in (("capacity_lower", 1.1), ("capacity_upper", 0.9)):
            with self.subTest(field=field):
                data = fixture()
                data["computed-polytopes.jsonl"][0][field] = value
                with self.assertRaisesRegex(SystemExit, "invalid capacity bounds"):
                    self.validate_fixture(data)
        data = fixture()
        data["computed-polytopes.jsonl"][0]["sigmas"][0]["action"] = 2.0
        with self.assertRaisesRegex(SystemExit, "outside its capacity bounds"):
            self.validate_fixture(data)

    def test_reject_facet_metadata_disagreement(self) -> None:
        cases = (
            ("random-samples.jsonl", "source", "facet_count", 9),
            ("random-product-samples.jsonl", None, "facet_count", 8),
            ("random-product-samples.jsonl", "source", "m", 5),
            ("computed-polytopes.jsonl", None, "dual_vertices", []),
            ("computed-polytopes.jsonl", None, "dual_vertices_rational", []),
        )
        for filename, nested, field, value in cases:
            with self.subTest(filename=filename, field=field):
                data = fixture()
                row = data[filename][0]
                (row[nested] if nested else row)[field] = value
                with self.assertRaisesRegex(SystemExit, "disagrees"):
                    self.validate_fixture(data)

    def test_reject_invalid_facet_counts(self) -> None:
        for value in (True, 4, -1, 8.0):
            with self.subTest(value=value):
                data = fixture()
                data["random-samples.jsonl"][0]["source"]["facet_count"] = value
                with self.assertRaisesRegex(SystemExit, "invalid facet_count"):
                    self.validate_fixture(data)
        for field in ("k", "m"):
            data = fixture()
            data["random-product-samples.jsonl"][0]["source"][field] = 2
            with self.assertRaisesRegex(SystemExit, f"invalid {field}"):
                self.validate_fixture(data)

    def test_reject_nonfinite_height_and_timings(self) -> None:
        for field in ("h_min", "h_max"):
            data = fixture()
            data["random-samples.jsonl"][0]["source"][field] = float("inf")
            with self.assertRaisesRegex(SystemExit, field):
                self.validate_fixture(data)
        for field in ("cache_miss_volume_ms", "cache_miss_capacity_ms", "wall_time_ms"):
            data = fixture()
            data["produce-stats.json"][field] = float("inf")
            with self.assertRaisesRegex(SystemExit, field):
                self.validate_fixture(data)


if __name__ == "__main__":
    unittest.main()
