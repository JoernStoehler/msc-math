from __future__ import annotations

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import conditioning_audit as audit


def test_replay_is_byte_stable_and_reasons_are_separate() -> None:
    left, left_summary = audit.run(20260715, 12, 3, [3, 4, 6])
    right, right_summary = audit.run(20260715, 12, 3, [3, 4, 6])
    assert left == right
    assert left_summary == right_summary
    audit.check_reason_integrity(left)
    reasons = {row["terminal_reason"] for row in left}
    assert "accepted" in reasons
    assert "wrong_side_count" in reasons
    assert "unbounded_or_origin_failure" in reasons


def test_known_selection_shift_and_always_accept_control() -> None:
    result = audit.calibration(seed=91, n=20_000)
    known = result["known_shift"]
    assert abs(known["acceptance"] - 0.25) < 0.02
    assert abs(known["accepted_mean"] - 0.125) < 0.015
    assert abs(result["always_accept"]["mean_shift"]) < 1e-12


def test_rejected_rows_never_receive_body_metrics() -> None:
    rows, _ = audit.run(7, 8, 4, [3, 4])
    assert rows
    assert all(
        (row["accepted_body_features"] is not None)
        == (row["terminal_reason"] == "accepted")
        for row in rows
    )


def test_corrupted_reason_control_fails_closed() -> None:
    rows, _ = audit.run(7, 8, 2, [3])
    row = next(row for row in rows if row["terminal_reason"] == "accepted")
    row["terminal_reason"] = "wrong_side_count"
    try:
        audit.check_reason_integrity(rows)
    except ValueError as error:
        assert "reason/body mismatch" in str(error)
    else:
        raise AssertionError("corrupted reason code was accepted")


def test_no_nonfinite_payload_in_smoke_rows() -> None:
    rows, summaries = audit.run(20260715, 8, 2, [3, 4, 6])
    for row in rows:
        for value in row["primitive_features"].values() if row["primitive_features"] else ():
            assert not math.isinf(value)
    assert summaries


if __name__ == "__main__":
    for name, function in sorted(globals().items()):
        if name.startswith("test_"):
            function()
    print("conditioning-distortion tests: ok")
