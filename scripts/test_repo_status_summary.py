#!/usr/bin/env python3
"""Exercise dated-check guidance in isolated Git repositories."""

from pathlib import Path
import os
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("repo-status-summary.sh").resolve()


class RepoStatusSummaryTests(unittest.TestCase):
    def setUp(self):
        scratch = Path("/tmp/msc-math/repo-status-summary")
        scratch.mkdir(parents=True, exist_ok=True)
        self.temp = tempfile.TemporaryDirectory(dir=scratch)
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.env = os.environ.copy()
        self.env.update(
            GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull,
            GIT_AUTHOR_NAME="Test", GIT_AUTHOR_EMAIL="test@example.invalid",
            GIT_COMMITTER_NAME="Test", GIT_COMMITTER_EMAIL="test@example.invalid",
        )
        self.git("init", "-q")
        (self.root / "scripts").mkdir()
        artifacts = self.root / "scripts/artifacts.py"
        artifacts.write_text("#!/bin/sh\nexit 0\n")
        artifacts.chmod(0o755)
        for name in ("README.md", "PLAN.md", "STATUS.md", "sample.rs"):
            (self.root / name).write_text("baseline\n")
        self.commit()
        baseline = self.git("rev-parse", "HEAD").strip()
        (self.root / "reference.md").write_text(
            f"- Commands listed below were run through commit `{baseline}`.\n"
        )
        self.commit()

    def git(self, *args):
        return subprocess.run(
            ["git", *args], cwd=self.root, env=self.env,
            check=True, capture_output=True, text=True, timeout=10,
        ).stdout

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-qm", "fixture")

    def summary(self):
        return subprocess.run(
            ["bash", str(SCRIPT), "reference.md"], cwd=self.root, env=self.env,
            check=True, capture_output=True, text=True, timeout=10,
        ).stdout

    def test_planning_changes_are_orientation_changes(self):
        (self.root / "PLAN.md").write_text("committed plan\n")
        self.commit()
        (self.root / "STATUS.md").write_text("local status\n")
        output = self.summary()
        self.assertIn("Orientation-affecting changed paths:\nPLAN.md\n", output)
        self.assertIn("Uncommitted orientation-affecting paths:\nSTATUS.md\n", output)

    def test_local_docs_do_not_hide_committed_check_warning(self):
        (self.root / "sample.rs").write_text("changed code\n")
        self.commit()
        (self.root / "README.md").write_text("local docs\n")
        output = self.summary()
        self.assertIn("Potentially check-affecting paths changed since", output)
        self.assertIn("Working tree is not clean", output)

    def test_local_and_committed_check_warnings_both_appear(self):
        (self.root / "sample.rs").write_text("committed code\n")
        self.commit()
        (self.root / "sample.rs").write_text("local code\n")
        output = self.summary()
        self.assertIn("uncommitted check-affecting paths. Inspect", output)
        self.assertIn("Potentially check-affecting paths changed since", output)

    def test_clean_unchanged_checks(self):
        output = self.summary()
        self.assertIn("Working tree:\nclean\n", output)
        self.assertIn("No code, data, build-contract", output)
        self.assertNotIn("Potentially check-affecting paths changed since", output)


if __name__ == "__main__":
    unittest.main()
