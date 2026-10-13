#!/usr/bin/env python3
"""Check bootstrap modes without installing tools or touching credentials."""

import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("bootstrap-cloud.sh")
BASH = shutil.which("bash")
STUB = '''#!/bin/sh
name=${0##*/}
printf '%s' "$name" >> "$BOOTSTRAP_TEST_LOG"
for arg do printf ' <%s>' "$arg" >> "$BOOTSTRAP_TEST_LOG"; done
printf '\\n' >> "$BOOTSTRAP_TEST_LOG"
case "$name" in
  uname) printf '%s\\n' "${BOOTSTRAP_TEST_ARCH:-x86_64}" ;;
  python3) exit "${BOOTSTRAP_TEST_PYTHON_EXIT:-0}" ;;
  rustc) printf '%s\\n' 'rustc 1.94.0 (test stub)' ;;
esac
'''


class BootstrapModes(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="msc-math-bootstrap-")
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.home = self.root / "home"
        self.home.mkdir()
        self.log = self.root / "commands.log"
        for name in (
            "uname", "uv", "rustup", "python3", "rustc", "rustfmt",
            "cargo", "rust-analyzer", "sudo", "install", "rclone", "chmod",
            "latexmk",
        ):
            stub = self.bin / name
            stub.write_text(STUB)
            stub.chmod(0o755)
        self.env = os.environ.copy()
        self.env.pop("MSC_MATH_R2_ACCESS_KEY_ID", None)
        self.env.pop("MSC_MATH_R2_SECRET_ACCESS_KEY", None)
        self.env.update(
            HOME=str(self.home),
            PATH=f"{self.bin}:/usr/bin:/bin",
            BOOTSTRAP_TEST_LOG=str(self.log),
            BOOTSTRAP_TEST_ARCH="x86_64",
            BOOTSTRAP_TEST_PYTHON_EXIT="0",
        )

    def run_setup(self, *args):
        result = subprocess.run(
            [BASH, str(SCRIPT), *args], cwd=self.root, env=self.env,
            capture_output=True, text=True, timeout=10,
        )
        lines = self.log.read_text().splitlines() if self.log.exists() else []
        return result, lines

    def assert_no_setup(self, commands):
        self.assertFalse(any(line.startswith(("sudo ", "install ", "rclone ", "cargo "))
                             for line in commands), commands)

    def test_without_r2_runs_core_setup_without_secrets(self):
        result, commands = self.run_setup("--without-r2")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("sudo <apt-get> <update>", commands)
        self.assertIn("cargo <fetch> <--locked>", commands)
        self.assertIn("latexmk <--version>", commands)
        self.assertIn("rclone <version>", commands)
        self.assertFalse(any(line.startswith(("install ", "chmod ", "rclone <config>", "rclone <lsf>"))
                             for line in commands), commands)

    def test_without_r2_ignores_supplied_secrets(self):
        config = self.home / ".config" / "rclone" / "rclone.conf"
        config.parent.mkdir(parents=True)
        config.write_text("existing test configuration\n")
        self.env.update(MSC_MATH_R2_ACCESS_KEY_ID="test-key",
                        MSC_MATH_R2_SECRET_ACCESS_KEY="test-secret")
        result, commands = self.run_setup("--without-r2")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("test-key", "\n".join(commands))
        self.assertNotIn("test-secret", "\n".join(commands))
        self.assertFalse(any(line.startswith(("install ", "chmod ", "rclone <config>", "rclone <lsf>"))
                             for line in commands), commands)
        self.assertEqual(config.read_text(), "existing test configuration\n")
        self.assertEqual(list(self.home.rglob("*")),
                         [self.home / ".config", config.parent, config])

    def test_default_requires_both_secrets_before_setup(self):
        for provided in ({}, {"MSC_MATH_R2_ACCESS_KEY_ID": "test-key"},
                         {"MSC_MATH_R2_SECRET_ACCESS_KEY": "test-secret"}):
            with self.subTest(provided=provided):
                self.log.unlink(missing_ok=True)
                self.env.pop("MSC_MATH_R2_ACCESS_KEY_ID", None)
                self.env.pop("MSC_MATH_R2_SECRET_ACCESS_KEY", None)
                self.env.update(provided)
                result, commands = self.run_setup()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("is required", result.stderr)
                self.assert_no_setup(commands)

    def test_default_still_configures_and_checks_r2(self):
        self.env.update(MSC_MATH_R2_ACCESS_KEY_ID="test-key",
                        MSC_MATH_R2_SECRET_ACCESS_KEY="test-secret")
        result, commands = self.run_setup()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(any(line.startswith("rclone <config> <create> <mscmath> <s3>")
                            for line in commands), commands)
        self.assertTrue(any(line.startswith("rclone <lsf> <mscmath:msc-math-artifacts/snapshots>")
                            for line in commands), commands)
        self.assertIn("cargo <fetch> <--locked>", commands)

    def test_invalid_arguments_and_help_do_no_work(self):
        for args in (("--unknown",), ("--without-r2", "extra"), ("",), ("--help",)):
            with self.subTest(args=args):
                self.log.unlink(missing_ok=True)
                result, commands = self.run_setup(*args)
                self.assertEqual(result.returncode == 0, args == ("--help",))
                self.assertEqual(commands, [])

    def test_opt_out_still_requires_supported_runtimes(self):
        for override in ({"BOOTSTRAP_TEST_ARCH": "aarch64"},
                         {"BOOTSTRAP_TEST_PYTHON_EXIT": "1"}):
            with self.subTest(override=override):
                self.log.unlink(missing_ok=True)
                self.env.update(BOOTSTRAP_TEST_ARCH="x86_64", BOOTSTRAP_TEST_PYTHON_EXIT="0")
                self.env.update(override)
                result, commands = self.run_setup("--without-r2")
                self.assertNotEqual(result.returncode, 0)
                self.assert_no_setup(commands)


if __name__ == "__main__":
    unittest.main()
