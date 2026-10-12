#!/usr/bin/env python3
"""Check builder adapters on frozen rows; never runs a capacity producer."""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

FIXTURE = Path(__file__).resolve().parent / "fixtures" / "builder-smoke"
TABLES = ("polytope-table.jsonl", "polytope-provenance-table.jsonl")


def run(binary: Path, arguments: list[str], out: Path, diagnostic: str | None = None) -> None:
    result = subprocess.run(
        [str(binary), *arguments, "--out-dir", str(out)],
        capture_output=True, text=True, env={**os.environ, "RAYON_NUM_THREADS": "1"},
    )
    if diagnostic is None:
        assert result.returncode == 0, result.stderr
    else:
        assert result.returncode != 0, f"malformed input accepted: {diagnostic}"
        assert diagnostic in result.stderr, result.stderr
        assert not out.exists(), f"failed adapter created output: {out}"


def check(binary_dir: Path) -> None:
    canonical = binary_dir / "sys-table-builder"
    with tempfile.TemporaryDirectory(prefix="sys-builder-fixture-") as scratch:
        scratch = Path(scratch)
        for label, input_dir, baseline, input_format, compatibility in (
            ("retained", "retained-input", "baseline-retained-output", "retained", "sys-dataset"),
            ("historical", "historical-unknown-input", "baseline-historical-unknown-output", "retained", "sys-dataset"),
            ("run-local", "run-local-input", "baseline-run-local-output", "run-local", "sys-datascience-prepare"),
        ):
            for program in (canonical, binary_dir / compatibility):
                out = scratch / f"{label}-{program.name}"
                args = ["--produce-dir", str(FIXTURE / input_dir)]
                if program == canonical:
                    args = ["--input-format", input_format, *args]
                run(program, args, out)
                for name in TABLES:
                    assert (out / name).read_bytes() == (FIXTURE / baseline / name).read_bytes(), (label, program.name, name)
        for name, diagnostic in (
            ("missing-payload", "missing computed payload"),
            ("mismatched-metadata", "disagrees with payload sys"),
            ("duplicate-provenance", "duplicate provenance_id"),
        ):
            run(canonical, ["--input-format", "run-local", "--produce-dir", str(FIXTURE / "failure-probes" / name / "input")], scratch / name, diagnostic)
        for label, filename, field, value, diagnostic in (
            ("product-row-facets", "random-product-samples.jsonl", "facet_count", 7, "facet_count disagrees with payload"),
            ("product-source-facets", "random-product-samples.jsonl", "source.k", 4, "source k+m disagrees with payload"),
            ("random-source-index", "random-samples.jsonl", "source.sample_index", None, "source must include sample_index"),
            ("product-source-index", "random-product-samples.jsonl", "source.sample_index", None, "source must include sample_index"),
        ):
            produce = scratch / f"{label}-input"
            shutil.copytree(FIXTURE / "run-local-input", produce)
            path = produce / filename
            rows = [json.loads(line) for line in path.read_text().splitlines()]
            container = rows[0]
            parts = field.split(".")
            for part in parts[:-1]:
                container = container[part]
            container[parts[-1]] = value
            path.write_text("".join(json.dumps(row) + "\n" for row in rows))
            run(canonical, ["--input-format", "run-local", "--produce-dir", str(produce)], scratch / label, diagnostic)
    print("builder fixture: six byte-identical routes; seven malformed inputs rejected before output")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary-dir", type=Path, default=Path("target/debug"))
    check(parser.parse_args().binary_dir.resolve())
