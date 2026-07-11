#!/usr/bin/env python3
"""Join and audit the retained first-boundary transition artifacts.

This analyzer is deliberately limited to the reliable single-boundary surface.
It does not read the repeated-crossing stress artifact and therefore cannot
produce repeated-transition rates.
"""

from __future__ import annotations

import ast
import hashlib
import json
import textwrap
from collections import Counter, defaultdict
from pathlib import Path
from statistics import median


HERE = Path(__file__).resolve().parent
ANATOMY_PATH = HERE / "combinatorial-boundaries-anatomy.jsonl"
CROSSING_PATH = HERE / "combinatorial-boundaries-crossing.jsonl"
GRADIENT_PATH = HERE / "combinatorial-boundaries-gradient.jsonl"
SUMMARY_PATH = HERE / "first-boundary-transition-summary.json"
EXCEPTIONS_PATH = HERE / "first-boundary-transition-exceptions.json"
REPORT_PATH = HERE / "first-boundary-transition-report.md"

KEY_FIELDS = ("polytope_name", "direction_type", "direction_index")
ANGLE_THRESHOLDS_DEG = (0.01, 0.1, 1.0, 10.0)
ANGLE_EXCEPTION_DEG = 0.1
KNOWN_POLYTOPES = {"simplex", "hypercube", "hko_pentagon"}
DEFAULT_CROSSING_EPS_FRACTION = 1e-4
EPS_FLOOR = 1e-8

# The epsilon policy and exception interpretations below were reviewed against
# these exact producer outputs. A refreshed input must update these hashes only
# after rechecking the producer policy and the generated exceptions.
EXPECTED_INPUT_SHA256 = {
    "combinatorial-boundaries-anatomy.jsonl": (
        "899894a23876f3869841e72fb7c4ff795c2be3e651263398f686c4a376e139a4"
    ),
    "combinatorial-boundaries-crossing.jsonl": (
        "fcffb630ada26435410f3c6c9ee94d758fc5a799c71857da3931fe3f2e254896"
    ),
    "combinatorial-boundaries-gradient.jsonl": (
        "42fe7d52e91e19c87a0064e85b7a50c6f7fb63d7c862179afc5c518e33f80c05"
    ),
}


def load_jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def file_identity(path: Path, rows: list[dict]) -> dict:
    payload = path.read_bytes()
    return {
        "path": str(path.relative_to(HERE.parent.parent.parent)),
        "sha256": hashlib.sha256(payload).hexdigest(),
        "bytes": len(payload),
        "rows": len(rows),
    }


def require_reviewed_input(identity: dict) -> None:
    name = Path(identity["path"]).name
    expected = EXPECTED_INPUT_SHA256.get(name)
    if expected != identity["sha256"]:
        raise ValueError(
            f"unreviewed input identity for {name}: {identity['sha256']}; "
            "recheck the producer epsilon policy, selected-sigma semantics, "
            "and exception ledger before updating EXPECTED_INPUT_SHA256"
        )


def key(row: dict) -> tuple[str, str, int]:
    return tuple(row[field] for field in KEY_FIELDS)  # type: ignore[return-value]


def require_unique(name: str, rows: list[dict]) -> dict[tuple, dict]:
    indexed: dict[tuple, dict] = {}
    duplicates = []
    for row in rows:
        row_key = key(row)
        if row_key in indexed:
            duplicates.append(row_key)
        indexed[row_key] = row
    if duplicates:
        raise ValueError(f"{name} has duplicate join keys: {duplicates[:5]}")
    return indexed


def parse_selected_sigma(raw: str) -> tuple[int, ...]:
    value = ast.literal_eval(raw)
    if not isinstance(value, list) or not all(isinstance(item, int) for item in value):
        raise ValueError(f"unexpected selected-sigma string: {raw!r}")
    return tuple(value)


def cyclic_selected_sigma(raw: str) -> tuple[int, ...]:
    sigma = parse_selected_sigma(raw)
    if not sigma:
        return ()
    return min(sigma[index:] + sigma[:index] for index in range(len(sigma)))


def dihedral_selected_sigma(raw: str) -> tuple[int, ...]:
    sigma = parse_selected_sigma(raw)
    if not sigma:
        return ()
    reverse = tuple(reversed(sigma))
    variants = [sigma[index:] + sigma[:index] for index in range(len(sigma))]
    variants.extend(reverse[index:] + reverse[:index] for index in range(len(reverse)))
    return min(variants)


def selected_sigma_changed(row: dict, equivalence: str) -> bool:
    if equivalence == "raw":
        return row["orbit_before"] != row["orbit_after"]
    if equivalence == "cyclic":
        return cyclic_selected_sigma(row["orbit_before"]) != cyclic_selected_sigma(
            row["orbit_after"]
        )
    if equivalence == "dihedral_sensitivity":
        return dihedral_selected_sigma(
            row["orbit_before"]
        ) != dihedral_selected_sigma(row["orbit_after"])
    raise ValueError(equivalence)


def event_table(rows: list[dict]) -> dict:
    table = {}
    for event in sorted({row["event_type"] for row in rows}):
        event_rows = [row for row in rows if row["event_type"] == event]
        table[event] = {
            "rows": len(event_rows),
            "raw_selected_best_sigma_changes": sum(
                selected_sigma_changed(row, "raw") for row in event_rows
            ),
            "cyclic_selected_best_sigma_changes": sum(
                selected_sigma_changed(row, "cyclic") for row in event_rows
            ),
            "dihedral_sensitivity_selected_best_sigma_changes": sum(
                selected_sigma_changed(row, "dihedral_sensitivity")
                for row in event_rows
            ),
        }
    return table


def polytope_event_table(rows: list[dict]) -> dict:
    table = {}
    for event in sorted({row["event_type"] for row in rows}):
        event_rows = [row for row in rows if row["event_type"] == event]
        exposed = {row["polytope_name"] for row in event_rows}
        changed = {
            row["polytope_name"]
            for row in event_rows
            if selected_sigma_changed(row, "cyclic")
        }
        table[event] = {
            "polytopes_with_a_successful_probe": len(exposed),
            "polytopes_with_any_cyclic_selected_best_sigma_change": len(changed),
        }
    return table


def quantiles(values: list[float]) -> dict:
    ordered = sorted(values)
    if not ordered:
        return {"median": None, "p90": None, "p99": None, "max": None}

    def at(fraction: float) -> float:
        return ordered[int(fraction * (len(ordered) - 1))]

    return {
        "median": median(ordered),
        "p90": at(0.9),
        "p99": at(0.99),
        "max": ordered[-1],
    }


def epsilon_class(row: dict) -> str:
    if not row["construction_ok_after"]:
        return "crossing_failed"
    expected = max(DEFAULT_CROSSING_EPS_FRACTION * row["t_max"], EPS_FLOOR)
    tolerance = max(1e-14, 1e-9 * expected)
    if abs(row["eps_used"] - expected) <= tolerance:
        return "default_floor" if expected == EPS_FLOOR else "default_fraction"
    return "fallback"


def exception_record(crossing: dict, anatomy: dict, gradient: dict | None, flags: list[str]) -> dict:
    record = {
        "key": {field: crossing[field] for field in KEY_FIELDS},
        "flags": sorted(flags),
        "event_type": crossing["event_type"],
        "facet_count": crossing["facet_count"],
        "source_class": (
            "known" if crossing["polytope_name"] in KNOWN_POLYTOPES else "random"
        ),
        "t_max": crossing["t_max"],
        "eps_used": crossing["eps_used"],
        "epsilon_class": epsilon_class(crossing),
        "sys_before": crossing["sys_before"],
        "sys_after": crossing["sys_after"],
        "delta_sys": crossing["delta_sys"],
        "selected_best_sigma_before": crossing["orbit_before"],
        "selected_best_sigma_after": crossing["orbit_after"],
        "selected_best_sigma_changed_raw": selected_sigma_changed(crossing, "raw"),
        "selected_best_sigma_changed_cyclic": selected_sigma_changed(
            crossing, "cyclic"
        ),
        "selected_best_sigma_changed_dihedral_sensitivity": selected_sigma_changed(
            crossing, "dihedral_sensitivity"
        ),
        "base_cached_action_gap": anatomy["orbit_gap"],
        "event_vertex": anatomy["event_vertex"],
        "event_facet_new": anatomy["event_facet_new"],
        "event_facet_pair": anatomy["event_facet_pair"],
        "event_facet_degen": anatomy["event_facet_degen"],
    }
    if gradient is not None:
        record.update(
            {
                "gradient_angle_change_deg": gradient["gradient_angle_change_deg"],
                "directional_deriv_jump": gradient["directional_deriv_jump"],
                "gradient_norm_jump": gradient["gradient_norm_jump"],
            }
        )
    return record


def markdown_table(event_metrics: dict) -> str:
    lines = [
        "| Event | Successful rows | Raw selected-sigma changes | Cyclic selected-sigma changes | Dihedral sensitivity |",
        "| --- | ---: | ---: | ---: | ---: |",
    ]
    for event, metrics in event_metrics.items():
        lines.append(
            f"| {event} | {metrics['rows']} | "
            f"{metrics['raw_selected_best_sigma_changes']} | "
            f"{metrics['cyclic_selected_best_sigma_changes']} | "
            f"{metrics['dihedral_sensitivity_selected_best_sigma_changes']} |"
        )
    return "\n".join(lines)


def main() -> None:
    anatomy_rows = load_jsonl(ANATOMY_PATH)
    crossing_rows = load_jsonl(CROSSING_PATH)
    gradient_rows = load_jsonl(GRADIENT_PATH)

    input_identities = [
        file_identity(ANATOMY_PATH, anatomy_rows),
        file_identity(CROSSING_PATH, crossing_rows),
        file_identity(GRADIENT_PATH, gradient_rows),
    ]
    for identity in input_identities:
        require_reviewed_input(identity)

    anatomy = require_unique("anatomy", anatomy_rows)
    crossing = require_unique("crossing", crossing_rows)
    gradient = require_unique("gradient", gradient_rows)

    crossing_keys = set(crossing)
    anatomy_keys = set(anatomy)
    gradient_keys = set(gradient)
    if not crossing_keys <= anatomy_keys:
        raise ValueError("crossing contains keys missing from anatomy")
    if not gradient_keys <= crossing_keys:
        raise ValueError("gradient contains keys missing from crossing")

    successful_rows = [row for row in crossing_rows if row["construction_ok_after"]]
    successful_keys = {key(row) for row in successful_rows}
    if successful_keys != gradient_keys:
        raise ValueError(
            "gradient keys must equal successful crossing keys: "
            f"successful_only={len(successful_keys - gradient_keys)}, "
            f"gradient_only={len(gradient_keys - successful_keys)}"
        )

    raw_changed = {
        key(row) for row in successful_rows if selected_sigma_changed(row, "raw")
    }
    cyclic_changed = {
        key(row)
        for row in successful_rows
        if selected_sigma_changed(row, "cyclic")
    }
    dihedral_changed = {
        key(row)
        for row in successful_rows
        if selected_sigma_changed(row, "dihedral_sensitivity")
    }
    producer_raw_changed = {
        key(row) for row in successful_rows if row["orbit_changed"]
    }

    gradient_by_change = {}
    for changed, keys in ((False, successful_keys - cyclic_changed), (True, cyclic_changed)):
        values = [gradient[row_key]["gradient_angle_change_deg"] for row_key in keys]
        gradient_by_change[str(changed).lower()] = {
            "rows": len(values),
            "angle_change_deg": quantiles(values),
        }

    threshold_metrics = {}
    for threshold in ANGLE_THRESHOLDS_DEG:
        selected = [
            row
            for row in gradient_rows
            if row["gradient_angle_change_deg"] >= threshold
        ]
        threshold_metrics[str(threshold)] = {
            "rows": len(selected),
            "cyclic_selected_best_sigma_changes": sum(
                key(row) in cyclic_changed for row in selected
            ),
            "omega_flips": sum(row["event_type"] == "omega_flip" for row in selected),
        }

    random_successful = [
        row for row in successful_rows if row["polytope_name"] not in KNOWN_POLYTOPES
    ]
    facet_metrics = {}
    for facet_count in sorted({row["facet_count"] for row in successful_rows}):
        facet_metrics[str(facet_count)] = event_table(
            [row for row in successful_rows if row["facet_count"] == facet_count]
        )

    epsilon_counts = Counter(epsilon_class(row) for row in crossing_rows)
    exceptions: dict[tuple, set[str]] = defaultdict(set)
    for row in crossing_rows:
        row_key = key(row)
        if not row["construction_ok_after"]:
            exceptions[row_key].add("crossing_failed")
            continue
        changed = row_key in cyclic_changed
        grad = gradient[row_key]
        angle = grad["gradient_angle_change_deg"]
        if row["event_type"] == "incidence_flip" and changed:
            exceptions[row_key].add("incidence_selected_best_sigma_change")
        if angle >= ANGLE_EXCEPTION_DEG and not changed:
            exceptions[row_key].add("large_angle_without_selected_best_sigma_change")
        if changed and angle < ANGLE_EXCEPTION_DEG:
            exceptions[row_key].add("selected_best_sigma_change_below_angle_threshold")
        if epsilon_class(row) == "fallback":
            exceptions[row_key].add("epsilon_fallback")
        if epsilon_class(row) == "default_floor":
            exceptions[row_key].add("epsilon_floor")
        if (
            row["polytope_name"] in KNOWN_POLYTOPES
            and angle >= 90.0
            and not changed
        ):
            exceptions[row_key].add("symmetric_large_angle_anomaly")
        if selected_sigma_changed(row, "raw") != selected_sigma_changed(
            row, "cyclic"
        ):
            exceptions[row_key].add("raw_cyclic_equivalence_disagreement")
        if selected_sigma_changed(row, "cyclic") != selected_sigma_changed(
            row, "dihedral_sensitivity"
        ):
            exceptions[row_key].add("cyclic_dihedral_sensitivity_disagreement")

    exception_rows = [
        exception_record(
            crossing[row_key],
            anatomy[row_key],
            gradient.get(row_key),
            sorted(flags),
        )
        for row_key, flags in sorted(exceptions.items())
    ]

    summary = {
        "schema_version": 2,
        "question": (
            "Do first omega-sign and incidence crossings differ in the selected "
            "cyclic best sigma and gradient response?"
        ),
        "inputs": input_identities,
        "join_checks": {
            "key_fields": list(KEY_FIELDS),
            "anatomy_unique_keys": len(anatomy),
            "crossing_unique_keys": len(crossing),
            "gradient_unique_keys": len(gradient),
            "crossing_keys_missing_from_anatomy": 0,
            "gradient_keys_missing_from_crossing": 0,
            "successful_crossing_keys_equal_gradient_keys": True,
            "all_input_hashes_match_reviewed_identity": True,
        },
        "crossing_status": {
            "rows": len(crossing_rows),
            "successful": len(successful_rows),
            "failed": len(crossing_rows) - len(successful_rows),
            "epsilon_classes": dict(sorted(epsilon_counts.items())),
        },
        "selected_best_sigma_equivalence_audit": {
            "producer_orbit_changed_flag_vs_raw_selected_sigma_disagreements": len(
                producer_raw_changed ^ raw_changed
            ),
            "raw_selected_best_sigma_changes": len(raw_changed),
            "cyclic_selected_best_sigma_changes": len(cyclic_changed),
            "dihedral_sensitivity_selected_best_sigma_changes": len(
                dihedral_changed
            ),
            "raw_vs_cyclic_selected_sigma_disagreements": len(
                raw_changed ^ cyclic_changed
            ),
            "cyclic_vs_dihedral_selected_sigma_disagreements": len(
                cyclic_changed ^ dihedral_changed
            ),
            "interpretation": (
                "Cyclic rotation is the intended closed-cycle sensitivity. "
                "Reversal is reported only as a sensitivity check; this analyzer "
                "does not assert that reversal preserves the oriented characteristic."
            ),
        },
        "reviewed_analysis_contract": {
            "expected_input_sha256": EXPECTED_INPUT_SHA256,
            "default_crossing_epsilon_fraction": DEFAULT_CROSSING_EPS_FRACTION,
            "epsilon_floor": EPS_FLOOR,
            "angle_exception_threshold_deg": ANGLE_EXCEPTION_DEG,
            "known_polytope_names": sorted(KNOWN_POLYTOPES),
        },
        "all_successful_by_event": event_table(successful_rows),
        "random_only_by_event": event_table(random_successful),
        "all_successful_polytope_level_by_event": polytope_event_table(
            successful_rows
        ),
        "random_only_polytope_level_by_event": polytope_event_table(
            random_successful
        ),
        "by_facet_count_and_event": facet_metrics,
        "gradient_angle_by_cyclic_selected_best_sigma_change": gradient_by_change,
        "gradient_angle_threshold_sensitivity": threshold_metrics,
        "exception_counts": dict(
            sorted(Counter(flag for row in exception_rows for flag in row["flags"]).items())
        ),
        "claim_boundaries": [
            "Rows are multiple directions per polytope, not independent polytope draws.",
            "A small before/after delta does not prove continuity.",
            "A selected best-sigma change does not enumerate all tied minima and is not a mechanism or theorem.",
            "The cached action gap belongs to the starting polytope and returned branch set, not the immediate pre-boundary point or a global branch certificate.",
            "Gradient representatives can be unstable at symmetric or tied points.",
            "The repeated-crossing stress artifact is excluded; no repeated-transition rate is supported.",
        ],
    }

    SUMMARY_PATH.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    EXCEPTIONS_PATH.write_text(json.dumps(exception_rows, indent=2, sort_keys=True) + "\n")

    incidence_witnesses = [
        row
        for row in exception_rows
        if "incidence_selected_best_sigma_change" in row["flags"]
    ]
    if len(incidence_witnesses) == 1:
        incidence_key = incidence_witnesses[0]["key"]
        incidence_witness_text = textwrap.fill(
            "The lone retained incidence witness is "
            f"`{incidence_key['polytope_name']}/"
            f"{incidence_key['direction_type']}/"
            f"{incidence_key['direction_index']}`. Its full event facets, "
            "selected sigmas, gradient response, and epsilon are in the "
            "exception artifact; it prevents a universal claim that incidence "
            "flips are invisible.",
            break_on_hyphens=False,
        )
    else:
        incidence_witness_text = textwrap.fill(
            f"The retained inputs contain {len(incidence_witnesses)} incidence "
            "selected-best-sigma witnesses; inspect them in the exception "
            "artifact before making an incidence-boundary claim.",
            break_on_hyphens=False,
        )

    threshold_text = textwrap.fill(
        "The gradient response separates the same rows descriptively. A "
        "post-hoc 0.1-degree diagnostic threshold selects "
        f"{threshold_metrics['0.1']['rows']} rows; "
        f"{threshold_metrics['0.1']['cyclic_selected_best_sigma_changes']} "
        "change the cyclic selected best sigma and "
        f"{threshold_metrics['0.1']['omega_flips']} are omega flips.",
        break_on_hyphens=False,
    )
    epsilon_text = textwrap.fill(
        "All successful after-boundary constructions used either the "
        "producer's default 1e-4 relative epsilon or its declared absolute "
        f"1e-8 floor: {epsilon_counts.get('default_fraction', 0)} "
        "default-relative rows and "
        f"{epsilon_counts.get('default_floor', 0)} floor rows. There are "
        f"{epsilon_counts.get('fallback', 0)} successful fallback rows. The "
        "two failed crossings are retained separately.",
        break_on_hyphens=False,
    )

    report = f"""# First-Boundary Transition Atlas

Generated by `analyze_transition_atlas.py` from the three retained
boundary-characterization JSONL artifacts. Exact input hashes and join checks
are in `first-boundary-transition-summary.json`. The analyzer refuses changed
input hashes until the copied epsilon policy and exception assumptions are
reviewed again.

## Result

{markdown_table(summary['all_successful_by_event'])}

The raw, cyclic, and reversal-inclusive sensitivity counts agree on this
artifact. Cyclic canonicalization therefore does not explain any reported
change of the producer-selected best sigma. Reversal is only a sensitivity
check: this packet does not assert that reversing an oriented characteristic
is mathematical equivalence. These rows do not enumerate every tied minimizing
sigma.

After excluding simplex, hypercube, and hko_pentagon, the random-only table is:

{markdown_table(summary['random_only_by_event'])}

{threshold_text}
The threshold is a sensitivity summary, not a validated classifier.

## Exception Audit

`first-boundary-transition-exceptions.json` preserves all crossing failures,
epsilon-floor/fallback rows, the incidence-flip selected-best-sigma change,
large-angle rows without such a change, selected-best-sigma changes below 0.1
degrees, equivalence disagreements, and symmetric large-angle anomalies. The
exception counts are:

```json
{json.dumps(summary['exception_counts'], indent=2, sort_keys=True)}
```

{epsilon_text}

{incidence_witness_text}

In particular, the hypercube supplies the symmetric near-180-degree gradient
anomalies without a selected-best-sigma change. These prohibit interpreting
gradient angle alone as branch geometry without handling tied/symmetric
gradients.

## Interpretation

The retained first-boundary probes support a hypothesis-generation claim:
changes of the cyclically canonicalized, producer-selected best sigma and
sizeable gradient changes are concentrated at omega-sign crossings rather than
ordinary incidence crossings. Most omega crossings still do not change that
selected sigma, so event type alone is not a mechanism. The incidence exception
is a required falsifier/witness for any stronger proposed statement.

The exception field `base_cached_action_gap` is the action gap cached at the
starting polytope over the producer's returned branch set. It is neither the
gap immediately before the boundary nor a certificate over every branch, and
this packet makes no near-tie claim from it.

This packet does not establish continuity, independence, causal mechanism, or
a theorem. It deliberately excludes `multiple-crossings/`: its construction
failures and missing per-step selected-best-sigma identities prevent an
unbiased repeated-transition-rate claim.
"""
    REPORT_PATH.write_text(report)


if __name__ == "__main__":
    main()
