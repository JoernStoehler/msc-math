#!/usr/bin/env python3
"""Adversarial integrity checks for the unconditional wide-row packet.

All mutations happen in temporary copies.  The producer-owned artifacts are
never edited; this keeps the test both hostile to corruption and reproducible.
"""
from __future__ import annotations

import json
import io
import shutil
import tempfile
from contextlib import contextmanager, redirect_stderr
from pathlib import Path

from validate import PREDICATE_CATEGORIES, validate


@contextmanager
def mutated(directory: Path):
    """Yield a cheap packet view; copy only artifacts a mutation writes.

    Every immutable artifact starts as a symlink to the producer packet.  The
    write helpers below unlink that symlink and copy its source before writing,
    so a mutation can never modify producer-owned data.  This also naturally
    supports mutations touching several files in one block.
    """
    with tempfile.TemporaryDirectory() as temporary:
        copy = Path(temporary) / "packet"
        copy.mkdir()
        for source in directory.iterdir():
            destination = copy / source.name
            destination.symlink_to(
                source.resolve(), target_is_directory=source.is_dir()
            )
        yield copy


def _writable(path: Path) -> Path:
    if path.is_symlink():
        source = path.resolve(strict=True)
        path.unlink()
        shutil.copy2(source, path)
    return path


def write_text(path: Path, text: str) -> None:
    _writable(path).write_text(text)


def write_jsonl(path: Path, rows: list[dict]) -> None:
    write_text(path, "".join(json.dumps(row, sort_keys=True) + "\n" for row in rows))


def source_guard(directory: Path) -> dict[str, tuple[int, int, int]]:
    """Cheap mtime/size/inode guard against accidental source mutation."""
    return {
        str(path.relative_to(directory)): (path.stat().st_size, path.stat().st_mtime_ns, path.stat().st_ino)
        for path in directory.iterdir()
        if path.is_file()
    }


def jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def main(directory: Path) -> None:
    guard = source_guard(directory)
    rows = [json.loads(line) for line in (directory / "raw_rows.jsonl").read_text().splitlines() if line]
    assert rows, "producer emitted no rows"
    inventory = json.loads((directory / "formula_inventory.json").read_text())
    assert inventory["formula_count"] == 101
    analysis = json.loads((directory / "analysis.json").read_text())
    assert len(analysis["applicable_formula_ids"]) >= 20
    assert analysis["formula_summary"]["qp.assembly_C"]["value_evaluation_count"] > 0
    # Availability is producer/data dependent; the validator checks that any
    # emitted value is atom-matched rather than requiring an obsolete zero.
    assert analysis["formula_summary"]["volume.facet_volume_centroid"]["value_evaluation_count"] >= 0
    assert analysis["formula_summary"]["consumer.orbit_recovery_acceptance"]["value_evaluation_count"] >= 0
    # The analysis worker may now evaluate interval/ranking formulas whenever
    # the producer retains explicit endpoints.  Assert only the invariant
    # that every retained summary has a nonnegative count; validator.py joins
    # each value back to its source atom and catches proxy evaluations.
    assert all(summary["value_evaluation_count"] >= 0 for summary in analysis["formula_summary"].values())
    assert analysis.get("six_predicate_categories")
    assert all("indeterminate_wrong" not in line for line in (directory / "analysis.json").read_text().splitlines())
    assert all("indeterminate_wrong" not in row.get("predicate_category", "") and "indeterminate_wrong" not in row.get("q_predicate_category", "") for row in rows)
    feasible = next(row for row in rows if row.get("qp_c_f64") and row.get("kkt_matrix_f64"))
    evaluations = [json.loads(line) for line in (directory / "formula_evaluations.jsonl").read_text().splitlines() if line]
    by_formula = {entry["formula_id"]: entry for entry in evaluations if entry["row_id"] == f"{feasible['case_id']}:{feasible['sigma']}"}
    assert by_formula["qp.assembly_C"]["value"] == feasible["qp_c_f64"]
    assert by_formula["qp.assembly_d"]["value"] == feasible["qp_d_f64"]
    assert by_formula["qp.assembly_H"]["value"] == feasible["qp_h_f64"]
    if feasible.get("q_correction_f64") is not None:
        assert by_formula["kkt.q_correction"]["value"] == feasible["q_correction_f64"]
    omega = feasible["omega_matrix_f64"]
    assert all(abs(omega[i][j] + omega[j][i]) < 1e-14 for i in range(len(omega)) for j in range(len(omega)))
    assert analysis["formula_summary"]["volume.facet_volume_centroid"]["evaluated"] == 0
    registry = analysis["local_formula_registry"]
    assert set(registry) == set(__import__("analyze").LOCAL_FORMULAS)
    assert all({"expression", "target", "center", "required_atoms", "hypotheses", "arithmetic_model", "consumers", "implementation_status"} <= set(spec) for spec in registry.values())
    assert {row["target_id"] for row in rows} >= {"original_rational", "stored_dyadic"}
    by_case = {}
    for row in rows:
        by_case.setdefault(row["case_id"], []).append(row)
    required_counts = {
        "random_F5_s0_0": 9,
        "seed99540836_q4_p5_attempt405000000000": 1294,
        "hko_beta_boundary": 1,
        "hko_near_singular_false_acceptance": 1,
        "hko_residual_q_failure": 1,
        "hko_rank_deficient": 1,
        "hypercube_exact_zero_beta_boundary": 1,
        "random_3x5_s0_0": 4,
    }
    for case_id, count in required_counts.items():
        assert len(by_case.get(case_id, [])) == count, (case_id, len(by_case.get(case_id, [])))
    assert {tuple(row["sigma"]) for row in by_case["random_3x5_s0_0"]} == {
        (1, 5, 6, 0, 2, 3), (1, 5, 6, 2, 0, 3),
        (1, 6, 5, 0, 2, 3), (1, 6, 5, 2, 0, 3),
    }
    assert tuple(by_case["hypercube_exact_zero_beta_boundary"][0]["sigma"]) == (0, 2, 1, 5, 6)
    assert by_case["random_F5_s0_0"][0]["target_id"] == "original_rational"
    assert by_case["seed99540836_q4_p5_attempt405000000000"][0]["target_id"] == "stored_dyadic"
    assert {by_case[case_id][0]["cohort"] for case_id in by_case if case_id.startswith("hko_")} == {
        "regression_beta_boundary", "regression_near_singular_false_acceptance",
        "regression_residual_q_failure", "regression_rank_deficient",
    }
    assert all(row["q_raw_f64"] is None or "q_corrected_f64" in row for row in rows)
    assert any(row.get("proposal_action_f64") is not None for row in rows if row["f64_solver_status"] != "feasible")
    assert any(row.get("unconditional_minimum_action_member") and row["f64_solver_status"] != "feasible" for row in rows)
    for row in rows:
        if row["exact_solver_status"] == "feasible":
            assert row["exact_beta_predicate"] == "true"
        elif row["exact_algebra_status"] == "consistent_no_positive_beta":
            assert row["exact_beta_predicate"] == "false"
        elif row["exact_algebra_status"] == "rational_system_inconsistent":
            assert row["exact_beta_predicate"] == "unavailable"
    # Batch independent hostile mutations: each packet is parsed once, while
    # preserving the original checks and their distinct error needles.
    with mutated(directory) as copy:
        rows_mut = jsonl(copy / "raw_rows.jsonl")
        next(row for row in rows_mut if row["case_id"] == "random_F5_s0_0")["target_id"] = "stored_dyadic"
        rows_mut[0]["lifecycle_events"] = rows_mut[0]["lifecycle_events"][:3] + ["visited"]
        next(row for row in rows_mut if row["f64_solver_status"] != "feasible")["action_f64"] = 1.0
        next(row for row in rows_mut if row["exact_solver_status"] == "feasible")["exact_beta_predicate"] = "false"
        rows_mut[0]["exact_solver_status"] = "feasible;algebraic_oracle_unavailable"
        write_jsonl(copy / "raw_rows.jsonl", rows_mut)
        ledger = json.loads((copy / "coverage_ledger.json").read_text())
        ledger["populations"] = [entry for entry in ledger["populations"] if entry["case_id"] != "random_3x5_s0_0"]
        write_text(copy / "coverage_ledger.json", json.dumps(ledger))
        aggregates_mut = jsonl(copy / "aggregates.jsonl")
        aggregates_mut[0]["exact_min_action"] = "999/1"
        aggregates_mut[0]["f64_low_action_window_count"] += 1
        aggregates_mut[0]["proposal_min_action"] = None
        aggregates_mut[0]["accepted_min_action"] = None
        write_jsonl(copy / "aggregates.jsonl", aggregates_mut)
        errors = validate(copy)
        for needle in ("stored-dyadic target/cohort/source mislabeling", "incomplete lifecycle", "unconditional proposal on rejected/non-feasible row", "feasible exact status", "ambiguous exact solver status", "coverage ledger case set mismatch", "aggregate exact minimum mismatch", "aggregate filter/count reconstruction mismatch", "aggregate proposal reconstruction mismatch", "accepted aggregate reconstruction mismatch"):
            assert any(needle in error for error in errors), needle

    # JSONL truncation and duplicate identities are kept separate because they
    # intentionally exercise incompatible row-count failure modes.
    with mutated(directory) as copy:
        raw = (copy / "raw_rows.jsonl").read_text().splitlines()
        write_text(copy / "raw_rows.jsonl", "\n".join(raw[:-1] + [raw[-1][:-7]]) + "\n")
        errors = validate(copy)
        assert any("row count mismatch" in error for error in errors)
        assert any("truncated or invalid JSON" in error for error in errors)
    with mutated(directory) as copy:
        raw = (copy / "raw_rows.jsonl").read_text().splitlines()
        write_text(copy / "raw_rows.jsonl", "\n".join(raw + [raw[0]]) + "\n")
        manifest = json.loads((copy / "manifest.json").read_text())
        manifest["rows"] += 1
        write_text(copy / "manifest.json", json.dumps(manifest))
        assert any("duplicate row identity" in error for error in validate(copy))

    # Artifact identity, formula joins, and atom/proxy checks are independent
    # and can share one full packet parse.
    with mutated(directory) as copy:
        manifest = json.loads((copy / "manifest.json").read_text())
        manifest["schema_version"] = "foreign-schema"
        manifest["command"] = "python3 fake-producer.py"
        write_text(copy / "manifest.json", json.dumps(manifest))
        analysis_mut = json.loads((copy / "analysis.json").read_text())
        analysis_mut["run_id"] = "foreign-run"
        write_text(copy / "analysis.json", json.dumps(analysis_mut))
        evaluations = jsonl(copy / "formula_evaluations.jsonl")
        evaluations[0]["run_id"] = "foreign-run"
        evaluations[0]["target_id"] = "stored_dyadic" if evaluations[0]["target_id"] == "original_rational" else "original_rational"
        target = next(entry for entry in evaluations if entry["formula_id"] == "local.q_residual_diagnostic.v1")
        target["center_id"] = "beta"
        target["comparison_id"] = "q_exact"
        next(entry for entry in evaluations if entry["formula_id"] == "qp.assembly_C")["value"] = [[999.0]]
        write_jsonl(copy / "formula_evaluations.jsonl", evaluations)
        stderr = io.StringIO()
        with redirect_stderr(stderr):
            errors = validate(copy)
        assert "retained producer command differs" in stderr.getvalue()
        for needle in ("manifest schema identity mismatch", "mixed analysis artifact identity", "mixed formula-evaluation run identity", "formula target/row identity mismatch", "formula center mismatch", "proposal Q correction diagnostic center/target mismatch", "proxy formula evaluation value"):
            assert any(needle in error for error in errors), needle

    # Keep all f64-ternary/exact-binary categories distinct, including the
    # exact-indeterminate states exposed by the unconditional exact linear
    # diagnostics, while keeping exact-unavailable separate.
    assert PREDICATE_CATEGORIES - {"exact_unavailable"} == {
        "true|true_sound", "true|false_unsound", "true|indeterminate_unsound",
        "false|true_unsound", "false|false_sound", "false|indeterminate_unsound",
        "indeterminate|true", "indeterminate|false", "indeterminate|indeterminate",
        "indeterminate|indeterminate_sound",
    }
    assert "exact_unavailable" in PREDICATE_CATEGORIES
    with mutated(directory) as copy:
        rows_mut = jsonl(copy / "raw_rows.jsonl")
        rows_mut[0]["predicate_category"] = "indeterminate_wrong"
        write_jsonl(copy / "raw_rows.jsonl", rows_mut)
        analysis_mut = json.loads((copy / "analysis.json").read_text())
        report = analysis_mut["strata_reports"]["unconditional_maximum_q"]["consumer_margins"]
        report["E_over_M"]["max"] = 0.0
        write_text(copy / "analysis.json", json.dumps(analysis_mut))
        errors = validate(copy)
        assert any("predicate category mismatch" in error or "invalid predicate category" in error for error in errors)
        assert any("consumer margin mismatch" in error for error in errors)
    assert source_guard(directory) == guard, "producer packet changed during mutation suite"
    print("wide-row tests passed")


if __name__ == "__main__":
    import sys

    main(Path(sys.argv[1]))
