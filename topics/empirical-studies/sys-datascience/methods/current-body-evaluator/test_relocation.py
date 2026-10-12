#!/usr/bin/env python3
"""Relocated build/supervisor boundary fixtures; no body or capacity requests."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

PACKET = Path(__file__).resolve().parent
REPO = PACKET.parents[4]
spec = importlib.util.spec_from_file_location("adapter_build", PACKET / "build.py")
build = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build)


class RelocatedAdapterTests(unittest.TestCase):
    def test_build_identity_resolves_repository_and_adapter_sources(self):
        identity = build.source_identity(PACKET)
        self.assertIn("Cargo.toml", identity)
        self.assertIn("crates/symplectic/Cargo.toml", identity)
        self.assertIn("topics/empirical-studies/sys-datascience/methods/current-body-evaluator/Cargo.toml", identity)
        self.assertTrue(all((REPO / name).is_file() for name in identity))

    def test_empty_supervisor_panel_accepts_receipt_and_keeps_output_immutable(self):
        with tempfile.TemporaryDirectory() as directory:
            temp = Path(directory)
            binary = temp / "not-a-capacity-worker"
            binary.write_text("this fixture binary must never be launched")
            source = temp / "empty.jsonl"
            source.write_text("")
            output = temp / "output.jsonl"
            receipt = temp / "receipt.json"
            receipt.write_text(json.dumps({
                "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "build_source_sha256": build.source_identity(PACKET),
            }))
            command = [sys.executable, str(PACKET / "evaluate.py"), "--input", str(source),
                       "--output", str(output), "--binary", str(binary), "--build-receipt", str(receipt)]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(output.read_bytes(), b"")
            output.write_text("retained fixture")
            again = subprocess.run(command, capture_output=True, text=True)
            self.assertNotEqual(again.returncode, 0)
            self.assertEqual(output.read_text(), "retained fixture")


if __name__ == "__main__":
    unittest.main()
