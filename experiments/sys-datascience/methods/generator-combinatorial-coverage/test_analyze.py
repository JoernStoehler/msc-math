#!/usr/bin/env python3
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import analyze  # noqa: E402


def test_permutation_invariance_and_nonisomorphic_separation():
    rows, rejects = analyze.load_rows([HERE / "fixtures/synthetic.jsonl"], cap=20_000, max_rows=None)
    assert not rejects
    a = {r["exact_type"] for r in rows if r["law"] == "law-a"}
    b = {r["exact_type"] for r in rows if r["law"] == "law-b"}
    assert len(a) == 1, "labeled permutations and geometry changes must collapse"
    assert len(b) == 1
    assert a != b, "nonisomorphic incidence graphs must separate"


def test_duplicates_do_not_disappear_from_raw_but_group_dedup_is_visible():
    rows, _ = analyze.load_rows([HERE / "fixtures/synthetic.jsonl"], cap=20_000, max_rows=None)
    summary = analyze.exact_occupancy([r for r in rows if r["law"] == "law-a"], analyze.SEED)
    assert summary["rows_total"] == 4
    assert summary["distinct_exact_types"] == 1
    assert summary["independence_groups"] == 3
    assert summary["distinct_types_after_group_dedup"] == 1
    assert summary["collision_probability_observed"] == 1.0


def test_cap_fails_closed():
    rows, _ = analyze.load_rows([HERE / "fixtures/synthetic.jsonl"], cap=1, max_rows=None)
    capped = [r for r in rows if r["law"] == "law-cap"]
    assert capped and all(r["canonical_status"] == "capped" for r in capped)
    assert all(r["exact_type"] is None for r in capped)
    assert all(r["wl_summary"] for r in capped)


def test_report_is_json_serializable():
    parser = type("Args", (), {"input": [str(HERE / "fixtures/synthetic.jsonl")], "seed": analyze.SEED, "exact_node_cap": 20_000, "max_rows_per_input": None})
    report = analyze.run(parser)
    json.dumps(report)
    assert report["rows"]["accepted"] == 7


if __name__ == "__main__":
    test_permutation_invariance_and_nonisomorphic_separation()
    test_duplicates_do_not_disappear_from_raw_but_group_dedup_is_visible()
    test_cap_fails_closed()
    test_report_is_json_serializable()
    print("ok")
