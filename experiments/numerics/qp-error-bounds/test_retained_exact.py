"""Mutation regressions: semantic corruption must fail validation."""
from __future__ import annotations

import copy
import json
import subprocess
import sys
import tempfile
from pathlib import Path


def must_fail(validator: Path, target: Path, label: str) -> None:
    result = subprocess.run([sys.executable, str(validator), str(target)], capture_output=True, text=True)
    if result.returncode == 0:
        raise SystemExit(f"mutation regression failed: {label} validated")


def write_case(target: Path, rows: list[dict]) -> None:
    (target / "raw_rows.jsonl").write_text("\n".join(json.dumps(row) for row in rows) + "\n")


def main() -> None:
    source = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/retained-exact")
    validator = Path(__file__).with_name("validate_retained_exact.py")
    rows = [json.loads(line) for line in (source / "raw_rows.jsonl").read_text().splitlines()]
    with tempfile.TemporaryDirectory(prefix="retained-exact-mutation-") as temp:
        root = Path(temp)
        base = root / "base"
        base.mkdir()
        (base / "manifest.json").write_text((source / "manifest.json").read_text())
        (base / "analysis.json").write_text((source / "analysis.json").read_text())
        (base / "summary.md").write_text((source / "summary.md").read_text())

        truncated = root / "truncated"
        truncated.mkdir()
        for name in ("manifest.json", "analysis.json", "summary.md"):
            (truncated / name).write_text((source / name).read_text())
        write_case(truncated, rows[:-1])
        must_fail(validator, truncated, "truncated artifact")

        bad_action = root / "bad-action"
        bad_action.mkdir()
        for name in ("manifest.json", "analysis.json", "summary.md"):
            (bad_action / name).write_text((source / name).read_text())
        mutated = copy.deepcopy(rows)
        candidate = next(c for c in mutated[0]["candidates"] if c["retained_exact_status"] == "accepted")
        candidate["exact_action"] = "1/1"
        write_case(bad_action, mutated)
        must_fail(validator, bad_action, "corrupted exact action")

        bad_agreement = root / "bad-agreement"
        bad_agreement.mkdir()
        for name in ("manifest.json", "analysis.json", "summary.md"):
            (bad_agreement / name).write_text((source / name).read_text())
        mutated = copy.deepcopy(rows)
        mutated[1]["window_agreement_current_vs_retained"] = True
        write_case(bad_agreement, mutated)
        must_fail(validator, bad_agreement, "corrupted semantic agreement")
    print("truncation and semantic mutation regressions passed")


if __name__ == "__main__":
    main()
