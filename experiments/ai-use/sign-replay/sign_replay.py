#!/usr/bin/env python3
"""Replay four projection-solver regressions against a frozen base."""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

BASE_COMMIT = "f3d36cc968716132af582282dbe6c137a2857ec4"
SOURCE_REL = Path("crates/library/src/kkt/projection_solver.rs")
REPLAY_TARGET = Path(tempfile.gettempdir()) / "sign-replay-cargo-target"
GOOD_COEFFICIENT = "let coeff = -pi.dot(&b_prime) / eigenvalues[i];"
BAD_COEFFICIENT = "let coeff = pi.dot(&b_prime) / eigenvalues[i];"
INSERT_MARKER = "    /// Q is constant when H = 0: Q = 0 for all beta in the constraint set."


@dataclass(frozen=True)
class Case:
    label: str
    model: str
    prompt: str
    thread_id: str
    test_name: str
    kind: str


CASES = (
    Case("55-min", "GPT-5.5", "minimal repair contract",
         "019f580d-b500-7573-ae5f-0d90472186cf",
         "reduced_gradient_sign_gives_stationary_point",
         "two-variable-diag-2-1"),
    Case("55-ver", "GPT-5.5", "verifier-first repair contract",
         "019f580e-0ce8-79c3-80de-774115351de7",
         "retained_modes_use_negative_reduced_gradient",
         "two-variable-diag-neg-2-neg-8"),
    Case("56-min", "GPT-5.6-sol", "minimal repair contract",
         "019f580e-2677-7b12-b041-fd0854163903",
         "one_free_variable",
         "existing-one-free-variable-vector-assertion"),
    Case("56-ver", "GPT-5.6-sol", "verifier-first repair contract",
         "019f580e-4820-73c0-ab7d-bd9824559697",
         "reduced_stationarity_uses_negative_gradient",
         "two-variable-diag-1-3"),
)

REGRESSIONS = {
    "55-min": r'''
    /// Reconstructed 55-min regression.
    #[test]
    fn reduced_gradient_sign_gives_stationary_point() {
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 1.0]);
        let d = DVector::from_element(1, 1.0);
        let h = DMatrix::from_diagonal(&DVector::from_column_slice(&[2.0, 1.0]));
        let qp = QP { c, d, h };
        let sol = solve_projected(&qp);
        assert_eq!(sol.verdict, Verdict::True);
        assert!((sol.beta[0] - 1.0 / 3.0).abs() < 1e-10, "beta = {:?}", sol.beta);
        assert!((sol.beta[1] - 2.0 / 3.0).abs() < 1e-10, "beta = {:?}", sol.beta);
        let beta = DVector::from_column_slice(&sol.beta);
        let tangent = DVector::from_column_slice(&[1.0, -1.0]);
        let reduced_gradient = tangent.dot(&(&qp.h * beta));
        assert!(reduced_gradient.abs() < 1e-10, "gradient = {reduced_gradient}");
    }

''',
    "55-ver": r'''
    /// Reconstructed 55-ver regression.
    #[test]
    fn retained_modes_use_negative_reduced_gradient() {
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 1.0]);
        let d = DVector::from_column_slice(&[1.0]);
        let h = DMatrix::from_diagonal(&DVector::from_column_slice(&[-2.0, -8.0]));
        let qp = QP { c, d, h };
        let sol = solve_projected(&qp);
        assert_eq!(sol.verdict, Verdict::True);
        assert!((sol.beta[0] - 0.8).abs() < 1e-10, "beta = {:?}", sol.beta);
        assert!((sol.beta[1] - 0.2).abs() < 1e-10, "beta = {:?}", sol.beta);
        let beta = DVector::from_column_slice(&sol.beta);
        let tangent = DVector::from_column_slice(&[1.0, -1.0]);
        let derivative = tangent.dot(&(&qp.h * beta));
        assert!(derivative.abs() < 1e-10, "derivative = {derivative}");
    }

''',
    "56-ver": r'''
    /// Reconstructed 56-ver regression.
    #[test]
    fn reduced_stationarity_uses_negative_gradient() {
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 1.0]);
        let d = DVector::from_element(1, 1.0);
        let h = DMatrix::from_diagonal(&DVector::from_column_slice(&[1.0, 3.0]));
        let qp = QP { c, d, h };
        let sol = solve_projected(&qp);
        assert!((sol.beta[0] - 0.75).abs() < 1e-10, "beta = {:?}", sol.beta);
        assert!((sol.beta[1] - 0.25).abs() < 1e-10, "beta = {:?}", sol.beta);
        let tangent_gradient = sol.beta[0] - 3.0 * sol.beta[1];
        assert!(tangent_gradient.abs() < 1e-10, "gradient = {tangent_gradient}");
    }

''',
}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[3]


def run(
    command: list[str], cwd: Path, *, env: dict[str, str] | None = None
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, cwd=cwd, text=True, capture_output=True, env=env)


def replace_coefficient(source: str, *, correct_sign: bool) -> str:
    desired = GOOD_COEFFICIENT if correct_sign else BAD_COEFFICIENT
    pattern = re.compile(r"^(?P<indent>[ \t]*)let coeff = -?pi\.dot\(&b_prime\) / eigenvalues\[i\];$", re.MULTILINE)
    matches = list(pattern.finditer(source))
    if len(matches) != 1:
        raise ValueError(f"expected one reduced-gradient coefficient, found {len(matches)}")
    match = matches[0]
    replacement = match.group("indent") + desired
    return source[: match.start()] + replacement + source[match.end() :]


def inject_regression(source: str, case: Case) -> str:
    if case.label == "56-min":
        needle = """            5.0 / 12.0
        );
    }

""" + INSERT_MARKER
        replacement = """            5.0 / 12.0
        );
        let expected = [1.0 / 6.0; 5]
            .into_iter()
            .chain([5.0 / 6.0])
            .collect::<Vec<_>>();
        for (i, (&actual, expected)) in sol.beta.iter().zip(expected).enumerate() {
            assert!((actual - expected).abs() < 1e-8, "beta[{i}] = {actual}, expected {expected}");
        }
    }

""" + INSERT_MARKER
        if source.count(needle) != 1:
            raise ValueError("one_free_variable insertion anchor is not unique")
        return source.replace(needle, replacement)
    if source.count(INSERT_MARKER) != 1:
        raise ValueError("test insertion marker is not unique")
    return source.replace(INSERT_MARKER, REGRESSIONS[case.label] + INSERT_MARKER, 1)


def expected_pass(case: Case, *, correct_sign: bool) -> bool:
    return correct_sign or case.label == "56-min"


def dry_run() -> list[dict[str, object]]:
    rows = []
    for case in CASES:
        for correct_sign in (False, True):
            rows.append({
                "case": case.label,
                "test": case.test_name,
                "sign": "correct-negative" if correct_sign else "bad-positive",
                "expected_pass": expected_pass(case, correct_sign=correct_sign),
                "command": ["cargo", "test", "--manifest-path",
                            "crates/library/Cargo.toml", case.test_name, "--lib"],
            })
    return rows


def replay_case(repo: Path, case: Case, *, correct_sign: bool) -> dict[str, object]:
    temp_parent = Path(tempfile.mkdtemp(prefix="sign-replay-"))
    checkout = temp_parent / "checkout"
    result: dict[str, object] = {
        "case": case.label,
        "test": case.test_name,
        "sign": "correct-negative" if correct_sign else "bad-positive",
        "expected_pass": expected_pass(case, correct_sign=correct_sign),
    }
    try:
        add = run(["git", "worktree", "add", "--detach", str(checkout), BASE_COMMIT], repo)
        if add.returncode:
            raise RuntimeError(f"git worktree add failed: {add.stdout}\n{add.stderr}")
        source_path = checkout / SOURCE_REL
        source = inject_regression(source_path.read_text(), case)
        source_path.write_text(replace_coefficient(source, correct_sign=correct_sign))
        command = ["cargo", "test", "--manifest-path", "crates/library/Cargo.toml",
                   case.test_name, "--lib"]
        env = dict(os.environ)
        env["CARGO_TARGET_DIR"] = str(REPLAY_TARGET)
        proc = run(command, checkout, env=env)
        result.update({
            "command": command,
            "returncode": proc.returncode,
            "observed_pass": proc.returncode == 0,
            "matched_expectation": (proc.returncode == 0) == result["expected_pass"],
            "output_tail": (proc.stdout + proc.stderr)[-2000:],
        })
        return result
    finally:
        remove = run(["git", "worktree", "remove", "--force", str(checkout)], repo)
        shutil.rmtree(temp_parent, ignore_errors=True)
        if remove.returncode:
            print(f"warning: worktree cleanup failed: {remove.stderr}", file=sys.stderr)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)
    rows = dry_run() if args.dry_run else [
        replay_case(repo_root(), case, correct_sign=correct_sign)
        for case in CASES for correct_sign in (False, True)
    ]
    if args.json:
        print(json.dumps(rows, indent=2, sort_keys=True))
    else:
        for row in rows:
            print(json.dumps(row, sort_keys=True))
    return 0 if args.dry_run or all(row.get("matched_expectation") for row in rows) else 1


if __name__ == "__main__":
    raise SystemExit(main())
