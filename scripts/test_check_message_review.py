"""Focused tests for the exact-message receipt contract (stdlib only)."""

import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("check-message-review.py")
SPEC = importlib.util.spec_from_file_location("message_review", SCRIPT)
review = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(review)


class MessageReviewTests(unittest.TestCase):
    def setUp(self):
        self.message = "The café result is **provisional**.\n\n[review: /root/reviewer]"
        self.context = "User asks for the actual uncertainty."
        self.receipt = {
            "schema_version": 1,
            "verdict": "APPROVE",
            "reviewer": "/root/reviewer",
            "approved_text": self.message,
            "message_sha256": review.text_sha256(self.message),
            "context_sha256": review.text_sha256(self.context),
            "reviewed_at": "2026-10-08T12:00:00Z",
            "checks": {name: {"status": "PASS", "reason": "Checked against supplied context."}
                       for name in review.REQUIRED_CHECKS},
        }

    def test_exact_text_and_whitespace_nfc_variations_pass(self):
        varied = "  The\tcafe\u0301 result is **provisional**.\r\n[review: /root/reviewer]\n"
        self.assertEqual(review.validate(varied, self.receipt, "/root", self.context), "/root/reviewer")

    def test_substantive_change_and_markdown_change_reject(self):
        for altered in (self.message.replace("provisional", "proven"),
                        self.message.replace("**", ""),
                        self.message + " New unreviewed sentence."):
            with self.subTest(altered=altered), self.assertRaises(ValueError):
                review.validate(altered, self.receipt, "/root", self.context)

    def test_missing_failed_or_empty_reason_checks_reject(self):
        for name in review.REQUIRED_CHECKS:
            for replacement in (None, {"status": "FAIL", "reason": "Wrong scope."},
                                {"status": "PASS", "reason": " "}):
                receipt = copy.deepcopy(self.receipt)
                receipt["checks"][name] = replacement
                with self.subTest(name=name, replacement=replacement), self.assertRaises(ValueError):
                    review.validate(self.message, receipt, "/root", self.context)

    def test_self_review_rejects(self):
        with self.assertRaises(ValueError):
            review.validate(self.message, self.receipt, " /root/reviewer ", self.context)

    def test_context_binding_requires_matching_context(self):
        review.validate(self.message, self.receipt, "/root", self.context)
        for context in (None, "User asks for a proof."):
            with self.subTest(context=context), self.assertRaises(ValueError):
                review.validate(self.message, self.receipt, "/root", context)
        del self.receipt["context_sha256"]
        with self.assertRaises(ValueError):
            review.validate(self.message, self.receipt, "/root", self.context)
        with self.assertRaises(ValueError):
            review.validate(self.message, self.receipt, "/root")

    def test_digest_tampering_and_invalid_receipt_reject(self):
        for key, value in (("message_sha256", "0" * 64), ("verdict", "REJECT"),
                           ("schema_version", True), ("reviewed_at", "2026-10-08"),
                           ("reviewer", "")):
            receipt = copy.deepcopy(self.receipt)
            receipt[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                review.validate(self.message, receipt, "/root", self.context)
        with self.assertRaises(ValueError):
            review.validate(self.message, [], "/root", self.context)

    def test_visible_attribution_required_even_if_text_and_hash_match(self):
        message = "The café result is **provisional**."
        self.receipt.update(approved_text=message, message_sha256=review.text_sha256(message))
        with self.assertRaises(ValueError):
            review.validate(message, self.receipt, "/root", self.context)

    def test_cli_exit_status(self):
        with tempfile.TemporaryDirectory() as folder:
            message_file = Path(folder) / "message.txt"
            receipt_file = Path(folder) / "receipt.json"
            context_file = Path(folder) / "context.txt"
            context_file.write_text(self.context, encoding="utf-8")
            message_file.write_text(self.message, encoding="utf-8")
            receipt_file.write_text(json.dumps(self.receipt), encoding="utf-8")
            command = [sys.executable, str(SCRIPT), str(message_file), str(receipt_file), "--sender", "/root", "--context-file", str(context_file)]
            approved = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(approved.returncode, 0, approved.stderr)
            missing_context = subprocess.run(command[:-2], capture_output=True, text=True)
            self.assertEqual(missing_context.returncode, 2)
            self.assertIn("--context-file", missing_context.stderr)
            message_file.write_text(self.message + " Changed.", encoding="utf-8")
            rejected = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(rejected.returncode, 1)
            self.assertIn("REJECT:", rejected.stderr)


if __name__ == "__main__":
    unittest.main()
