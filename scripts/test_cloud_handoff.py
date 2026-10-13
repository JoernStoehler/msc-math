#!/usr/bin/env python3
"""Consequential scoped source handoff checks, with disposable local repositories."""

import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("cloud-handoff.py").resolve()


class SourceHandoff(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="msc-math-cloud-handoff-")
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.repo = self.root / "repo"
        self.repo.mkdir()
        self.git("init", "--quiet")
        self.git("config", "user.name", "Test")
        self.git("config", "user.email", "test@example.invalid")
        self.write(".gitignore", "private/\n*.secret\ncache/\n")
        self.write("src/run.sh", "#!/bin/sh\necho committed\n", 0o755)
        self.write(".agents/skills/demo/SKILL.md", "committed skill\n")
        (self.repo / ".claude/skills").mkdir(parents=True)
        (self.repo / ".claude/skills/demo").symlink_to("../../.agents/skills/demo")
        self.write("other/unselected.txt", "outside scope\n")
        self.git("add", ".")
        self.git("commit", "--quiet", "-m", "fixture")
        self.commit = self.git("rev-parse", "HEAD").stdout.strip()
        self.archive = self.root / "handoff.tar.gz"
        self.extracted = self.root / "relocated"
        self.extracted.mkdir()

    def git(self, *args):
        return subprocess.run(["git", "-C", str(self.repo), *args], text=True,
                              capture_output=True, check=True, timeout=10)

    def write(self, name, data, mode=0o644):
        path = self.repo / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(data)
        path.chmod(mode)

    def run_helper(self, *args, env=None, script=SCRIPT):
        return subprocess.run([sys.executable, str(script), *args],
                              text=True, capture_output=True, timeout=15,
                              cwd=self.root, env=env)

    def create(self, *scopes, revision=None):
        identity = ["--revision", revision] if revision else ["--working-tree"]
        return self.run_helper("create", "--repo", str(self.repo),
                               "--output", str(self.archive), *identity,
                               *(arg for scope in scopes for arg in ("--path", scope)))

    def extract(self):
        with tarfile.open(self.archive) as bundle:
            # Fixture contents are created by the helper under test. The normal
            # consumer contract uses POSIX tar, preserving relative symlinks.
            bundle.extractall(self.extracted, filter="fully_trusted")
        return json.loads((self.extracted / ".cloud-handoff/manifest.json").read_text())

    def verify(self):
        env = os.environ.copy()
        env["PATH"] = "/no-git-or-other-tools"
        return self.run_helper("verify", str(self.extracted), env=env,
                               script=self.extracted / ".cloud-handoff/verify.py")

    def test_working_bytes_addition_and_identity_are_distinct_from_base_commit(self):
        self.write("src/run.sh", "#!/bin/sh\necho local working edit\n", 0o755)
        self.write("src/new.py", "print('ordinary untracked addition')\n")
        self.write("src/token.secret", "private test bytes\n")
        self.write("private/credentials", "private test bytes\n")
        result = self.create("src")
        self.assertEqual(result.returncode, 0, result.stderr)
        manifest = self.extract()
        self.assertEqual(manifest["snapshot_kind"], "working-tree")
        self.assertEqual(manifest["base_commit"], self.commit)
        self.assertTrue(manifest["working_tree_dirty"])
        self.assertEqual((self.extracted / "src/new.py").read_text(),
                         "print('ordinary untracked addition')\n")
        self.assertIn("local working edit", (self.extracted / "src/run.sh").read_text())
        self.assertEqual(stat.S_IMODE((self.extracted / "src/run.sh").stat().st_mode), 0o755)
        self.assertFalse((self.extracted / "src/token.secret").exists())
        self.assertFalse((self.extracted / "private").exists())
        self.assertFalse((self.extracted / "other").exists())
        self.assertFalse((self.extracted / ".git").exists())
        self.assertTrue(manifest["omissions"]["untracked_ignored_not_enumerated"])
        self.assertEqual(self.verify().returncode, 0)

    def test_committed_revision_ignores_later_working_bytes_and_untracked_files(self):
        self.write("src/run.sh", "later dirty bytes\n", 0o600)
        self.write("src/new.py", "later addition\n")
        result = self.create("src/run.sh", revision=self.commit)
        self.assertEqual(result.returncode, 0, result.stderr)
        manifest = self.extract()
        self.assertEqual(manifest["snapshot_kind"], "committed-revision")
        self.assertIsNone(manifest["working_tree_dirty"])
        self.assertEqual((self.extracted / "src/run.sh").read_text(), "#!/bin/sh\necho committed\n")
        self.assertFalse((self.extracted / "src/new.py").exists())
        self.assertEqual(stat.S_IMODE((self.extracted / "src/run.sh").stat().st_mode), 0o755)
        self.assertEqual(self.verify().returncode, 0)

    def test_skill_discovery_link_remains_relative_and_works_after_relocation(self):
        result = self.create(".agents/skills/demo", ".claude/skills/demo")
        self.assertEqual(result.returncode, 0, result.stderr)
        manifest = self.extract()
        discovery = self.extracted / ".claude/skills/demo"
        self.assertTrue(discovery.is_symlink())
        self.assertEqual(os.readlink(discovery), "../../.agents/skills/demo")
        self.assertEqual((discovery / "SKILL.md").read_text(), "committed skill\n")
        self.assertEqual(manifest["omitted_symlink_targets"], [])
        shutil.rmtree(self.repo)  # Verification and discovery need no original checkout.
        self.assertEqual(self.verify().returncode, 0)

    def test_committed_skill_link_and_escape_rejection(self):
        result = self.create(".agents/skills/demo", ".claude/skills/demo", revision=self.commit)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.extract()
        self.assertTrue((self.extracted / ".claude/skills/demo").is_symlink())
        self.assertEqual(self.verify().returncode, 0)
        self.archive.unlink()
        (self.repo / "src/escape").symlink_to("../../external")
        self.git("add", "src/escape")
        self.git("commit", "--quiet", "-m", "external link fixture")
        result = self.create("src", revision="HEAD")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("escapes repository", result.stderr)
        self.assertFalse(self.archive.exists())

    def test_whole_selection_excludes_git_and_even_force_tracked_ignored_paths(self):
        self.write("private/credentials", "private test bytes\n")
        self.git("add", "--force", "private/credentials")
        result = self.create(".")
        self.assertEqual(result.returncode, 0, result.stderr)
        manifest = self.extract()
        self.assertFalse((self.extracted / ".git").exists())
        self.assertFalse((self.extracted / "private").exists())
        self.assertIn("private/credentials", manifest["omissions"]["ignored"])
        self.assertEqual(self.verify().returncode, 0)

    def test_omitted_internal_symlink_target_is_reported_and_not_copied(self):
        result = self.create(".claude/skills/demo")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("OMITTED/PARTIAL", result.stdout)
        manifest = self.extract()
        self.assertEqual(manifest["omitted_symlink_targets"], [{
            "link": ".claude/skills/demo", "resolved_target": ".agents/skills/demo"}])
        self.assertFalse((self.extracted / ".agents").exists())
        self.assertTrue((self.extracted / ".claude/skills/demo").is_symlink())
        self.assertEqual(self.verify().returncode, 0)

    def test_missing_intermediate_link_is_explicit_even_when_final_target_is_selected(self):
        (self.repo / "a").symlink_to("b")
        (self.repo / "b").symlink_to("c")
        self.write("c", "selected final target\n")
        result = self.create("a", "c")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("missing intermediate links: b", result.stdout)
        manifest = self.extract()
        self.assertFalse((self.extracted / "b").exists())
        self.assertEqual(manifest["omitted_symlink_targets"], [{
            "link": "a", "resolved_target": "c", "omitted_intermediate_links": ["b"]}])
        result = self.verify()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("missing intermediate links: b", result.stdout)

    def test_declared_output_parents_are_created(self):
        self.archive = self.root / "new/scratch/run/source.tar.gz"
        result = self.create("src")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(self.archive.is_file())
        self.extract()
        self.assertEqual(self.verify().returncode, 0)

    def test_outside_and_ignored_scopes_are_refused(self):
        self.write("private/credentials", "private test bytes\n")
        (self.root / "external").write_text("outside checkout\n")
        (self.repo / "src/escape").symlink_to("../../external")
        for scopes in (("src",), ("../external",), (str(self.root / "external"),),
                       ("private",), (".git",)):
            with self.subTest(scopes=scopes):
                result = self.create(*scopes)
                self.assertNotEqual(result.returncode, 0)
                self.assertFalse(self.archive.exists())

    def test_explicit_identity_selection_and_no_overwrite(self):
        result = self.run_helper("create", "--repo", str(self.repo),
                                 "--output", str(self.archive), "--path", "src")
        self.assertNotEqual(result.returncode, 0)
        result = self.run_helper("create", "--repo", str(self.repo),
                                 "--output", str(self.archive), "--working-tree")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.create("src").returncode, 0)
        original = self.archive.read_bytes()
        result = self.create("src")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("refusing to overwrite", result.stderr)
        self.assertEqual(self.archive.read_bytes(), original)

    def test_verify_refuses_changed_bytes_mode_link_missing_or_extra_files(self):
        self.assertEqual(self.create("src", ".agents/skills/demo", ".claude/skills/demo").returncode, 0)
        for mutation in ("bytes", "mode", "link", "missing", "extra"):
            with self.subTest(mutation=mutation):
                shutil.rmtree(self.extracted)
                self.extracted.mkdir()
                self.extract()
                run = self.extracted / "src/run.sh"
                if mutation == "bytes":
                    run.write_text(run.read_text().replace("committed", "corrupted"))
                elif mutation == "mode":
                    run.chmod(0o644)
                elif mutation == "link":
                    link = self.extracted / ".claude/skills/demo"
                    link.unlink()
                    link.symlink_to("../../src")
                elif mutation == "missing":
                    run.unlink()
                else:
                    (self.extracted / "extra.txt").write_text("not listed\n")
                result = self.verify()
                self.assertNotEqual(result.returncode, 0, result.stdout)
                if mutation == "bytes":
                    self.assertIn("hash mismatch", result.stderr)


if __name__ == "__main__":
    unittest.main()
