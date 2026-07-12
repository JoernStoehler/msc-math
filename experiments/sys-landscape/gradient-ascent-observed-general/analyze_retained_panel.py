#!/usr/bin/env python3
"""Validate and retain the fixed observed-ascent panel without rewriting rows."""

import argparse
import json
from pathlib import Path
from statistics import fmean


EXPECTED_SEEDS = list(range(42, 54))
TRACE_STOP = "trace_iteration_cap"
ENDPOINT_STATUS = "not_evaluable_trace_did_not_stop"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input-dir", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    return parser.parse_args()


def load_one(path: Path) -> tuple[bytes, dict]:
    payload = path.read_bytes()
    lines = payload.splitlines(keepends=True)
    if len(lines) != 1 or not lines[0].endswith(b"\n"):
        raise ValueError(f"{path}: expected exactly one newline-terminated JSONL row")
    return payload, json.loads(lines[0])


def fail(message: str) -> None:
    raise ValueError(message)


def main() -> None:
    args = parse_args()
    raw_rows: list[bytes] = []
    records: list[dict] = []
    common_schema = None
    common_config = None
    naming_defects: list[str] = []
    operational_failures: list[dict] = []

    for seed in EXPECTED_SEEDS:
        path = args.input_dir / f"seed-{seed}.jsonl"
        if not path.is_file():
            fail(f"missing required input {path}")
        payload, row = load_one(path)
        raw_rows.append(payload)

        if row.get("schema") is None:
            fail(f"seed {seed}: missing schema")
        if common_schema is None:
            common_schema = row["schema"]
        elif row["schema"] != common_schema:
            fail(f"seed {seed}: schema differs from seed {EXPECTED_SEEDS[0]}")

        configuration = row.get("configuration")
        if not isinstance(configuration, dict):
            fail(f"seed {seed}: missing configuration object")
        if configuration.get("seeds") != [seed]:
            fail(f"seed {seed}: configuration.seeds must be [{seed}]")
        comparable_config = dict(configuration)
        comparable_config.pop("seeds", None)
        if common_config is None:
            common_config = comparable_config
        elif comparable_config != common_config:
            fail(f"seed {seed}: run parameters differ from seed {EXPECTED_SEEDS[0]}")

        if row.get("status") != "completed" or row.get("failure") is not None:
            fail(f"seed {seed}: run did not complete cleanly")
        trace = row.get("trace")
        if not isinstance(trace, list) or len(trace) != configuration.get("trace_iteration_cap"):
            fail(f"seed {seed}: trace length does not equal configured cap")
        if not all(item.get("accepted") is True for item in trace):
            fail(f"seed {seed}: not every trace move was accepted")
        endpoint = row.get("endpoint")
        if endpoint.get("trace_termination") != TRACE_STOP:
            fail(f"seed {seed}: unexpected trace termination")
        if endpoint.get("endpoint_condition_status") != ENDPOINT_STATUS:
            fail(f"seed {seed}: endpoint condition was unexpectedly evaluable")
        if endpoint.get("threshold_improvement_found") is not True:
            fail(f"seed {seed}: endpoint scan did not find above-threshold move")

        start_sys = row["start"]["sys"]
        final_sys = row["final_state"]["sys"]
        delta = final_sys - start_sys
        budget = row["compute_budget"]
        all_attempts = [attempt for step in trace for attempt in step["attempts"]]
        all_attempts.extend(endpoint["attempts"])
        bad_attempts = [
            attempt
            for attempt in all_attempts
            if attempt["status"]
            not in {"accepted", "below_threshold", "above_threshold", "positive_below_threshold", "nonpositive"}
        ]
        if bad_attempts:
            operational_failures.append({"seed": seed, "attempts": bad_attempts})

        run_id = row.get("run_id", "")
        purpose = row.get("purpose")
        if "smoke" in run_id or row["schema"].endswith("smoke_v1"):
            naming_defects.append(
                f"seed {seed}: raw run_id/schema retain smoke terminology while purpose is {purpose!r}"
            )
        if purpose != "retained_mode_one_seed_preflight":
            fail(f"seed {seed}: unexpected raw purpose {purpose!r}")
        records.append(
            {
                "seed": seed,
                "source": path.name,
                "run_id": run_id,
                "purpose": purpose,
                "start_sys": start_sys,
                "final_sys": final_sys,
                "delta_sys": delta,
                "trace_length": len(trace),
                "accepted_moves": sum(step["accepted"] for step in trace),
                "trace_termination": endpoint["trace_termination"],
                "endpoint_condition_status": endpoint["endpoint_condition_status"],
                "endpoint_threshold_improvement_found": endpoint["threshold_improvement_found"],
                "elapsed_ms": budget["elapsed_ms"],
                "finite_step_evaluations": budget["finite_step_evaluations"],
                "capacity_orbit_iterations": budget["capacity_orbit_iterations"],
            }
        )

    actual_files = sorted(path.name for path in args.input_dir.glob("seed-*.jsonl"))
    expected_files = [f"seed-{seed}.jsonl" for seed in EXPECTED_SEEDS]
    if actual_files != expected_files:
        fail("input directory contains a missing, extra, or misnamed seed JSONL file")

    deltas = [record["delta_sys"] for record in records]
    args.out_dir.mkdir(parents=True, exist_ok=True)
    (args.out_dir / "retained-panel.jsonl").write_bytes(b"".join(raw_rows))
    summary = {
        "schema": "gradient_ascent_observed_general_retained_panel_summary_v1",
        "input_directory": str(args.input_dir),
        "source_files_in_order": expected_files,
        "validated_seeds": EXPECTED_SEEDS,
        "raw_schema": common_schema,
        "common_configuration_except_per_seed_list": common_config,
        "run_count": len(records),
        "completed_runs": len(records),
        "failed_runs": 0,
        "all_trace_moves_accepted": all(record["accepted_moves"] == record["trace_length"] for record in records),
        "trace_lengths": sorted({record["trace_length"] for record in records}),
        "accepted_moves_per_run": sorted({record["accepted_moves"] for record in records}),
        "trace_termination_counts": {TRACE_STOP: len(records)},
        "endpoint_condition_status_counts": {ENDPOINT_STATUS: len(records)},
        "endpoint_scans_with_above_threshold_move": sum(
            record["endpoint_threshold_improvement_found"] for record in records
        ),
        "mean_delta_sys": fmean(deltas),
        "min_delta_sys": min(deltas),
        "max_delta_sys": max(deltas),
        "total_elapsed_ms": sum(record["elapsed_ms"] for record in records),
        "total_finite_step_evaluations": sum(record["finite_step_evaluations"] for record in records),
        "total_capacity_orbit_iterations": sum(record["capacity_orbit_iterations"] for record in records),
        "operational_failures": operational_failures,
        "operational_failure_count": len(operational_failures),
        "naming_defects": naming_defects,
        "per_seed": records,
        "interpretation_boundary": "The panel records systematic finite-step ascent progress and a bounded observed cost profile. All runs stopped at the iteration cap, and every endpoint scan found an above-threshold move; it therefore does not support a heuristic endpoint condition or local-maximality claim at cap 8.",
    }
    with (args.out_dir / "summary.json").open("w", encoding="utf-8") as handle:
        json.dump(summary, handle, indent=2, sort_keys=True)
        handle.write("\n")


if __name__ == "__main__":
    main()
