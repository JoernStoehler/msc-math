"""Cheap negative checks for the artifact analyzers."""

import json
import shutil
import subprocess
import tempfile
from pathlib import Path


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[3]


def test_summary_mutation_is_rejected():
    with tempfile.TemporaryDirectory(prefix="adaptive-direction-summary-") as tmp:
        packet = Path(tmp)
        shutil.copytree(HERE / "artifacts", packet / "artifacts")
        shutil.copytree(HERE / "inputs", packet / "inputs")
        summary_path = packet / "artifacts" / "summary.json"
        summary = json.loads(summary_path.read_text())
        summary["trajectories"][0]["best_sys"] += 1.0
        summary_path.write_text(json.dumps(summary) + "\n")
        result = subprocess.run(
            ["python3", str(HERE / "analyze.py"), str(packet / "artifacts")],
            cwd=REPO,
            capture_output=True,
            text=True,
        )
        assert result.returncode != 0, result.stdout + result.stderr


if __name__ == "__main__":
    test_summary_mutation_is_rejected()
    print("summary mutation rejected")
