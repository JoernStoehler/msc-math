#!/usr/bin/env python3

import importlib.util
from pathlib import Path
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check_no_git_lfs.py")
SPEC = importlib.util.spec_from_file_location("check_no_git_lfs", SCRIPT)
assert SPEC and SPEC.loader
check_no_git_lfs = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(check_no_git_lfs)


class CheckNoGitLfsTests(unittest.TestCase):
    def test_ordinary_files_are_accepted(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = Path(raw) / "data.jsonl"
            path.write_text('{"value": 1}\n', encoding="utf-8")
            self.assertEqual(check_no_git_lfs.find_problems([path]), [])

    def test_lfs_attribute_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = Path(raw) / ".gitattributes"
            path.write_text("*.jsonl filter=lfs diff=lfs merge=lfs -text\n")
            problems = check_no_git_lfs.find_problems([path])
            self.assertTrue(any("active Git LFS attribute" in item for item in problems))

    def test_lfs_pointer_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            path = Path(raw) / "data.jsonl"
            path.write_text(
                "version https://git-lfs.github.com/spec/v1\n"
                "oid sha256:" + "0" * 64 + "\n"
                "size 1\n",
                encoding="utf-8",
            )
            problems = check_no_git_lfs.find_problems([path])
            self.assertTrue(any("Git LFS pointer" in item for item in problems))


if __name__ == "__main__":
    unittest.main()
