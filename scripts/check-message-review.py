#!/usr/bin/env python3
"""Historical schema/hash verifier; not required by the current message-review skill.

Check an independent review receipt against the complete outgoing message.

This is an explicit pre-send check, not a platform send hook. A receipt records
review; it cannot authenticate who wrote it. The coordinator must verify that
the named reviewer is the native subagent that returned the receipt.
"""

import argparse
import hashlib
import json
from pathlib import Path
import re
import sys
import unicodedata
from datetime import datetime, timezone


REQUIRED_CHECKS = (
    "answer_relevance", "user_effort", "evidence_scope", "continuation",
    "communication",
)


def normalize_text(text):
    """Allow only NFC and whitespace variation, preserving all punctuation."""
    return re.sub(r"\s+", " ", unicodedata.normalize("NFC", text)).strip()


def text_sha256(text):
    return hashlib.sha256(normalize_text(text).encode("utf-8")).hexdigest()


def validate(message, receipt, sender, context=None):
    """Raise ValueError on an invalid receipt; never create an approval."""
    if not isinstance(receipt, dict):
        raise ValueError("receipt must be a JSON object")
    if type(receipt.get("schema_version")) is not int or receipt["schema_version"] != 1:
        raise ValueError("schema_version must be 1")
    if receipt.get("verdict") != "APPROVE":
        raise ValueError("reviewer verdict is not APPROVE")
    reviewer = receipt.get("reviewer")
    if not isinstance(reviewer, str) or not reviewer.strip():
        raise ValueError("reviewer must be a nonempty native subagent name or UUID")
    if not sender.strip() or normalize_text(reviewer) == normalize_text(sender):
        raise ValueError("independent reviewer required; self-review is invalid")
    approved = receipt.get("approved_text")
    if not isinstance(approved, str) or not normalize_text(approved):
        raise ValueError("approved_text must contain the full message")
    if normalize_text(message) != normalize_text(approved):
        raise ValueError("message differs from the exact reviewed text")
    if receipt.get("message_sha256") != text_sha256(approved):
        raise ValueError("message_sha256 does not match reviewed text")
    if normalize_text(f"[review: {reviewer}]") not in normalize_text(message):
        raise ValueError("message must include [review: REVIEWER] attribution")
    checks = receipt.get("checks")
    if not isinstance(checks, dict):
        raise ValueError("checks must be an object")
    for name in REQUIRED_CHECKS:
        check = checks.get(name)
        if not isinstance(check, dict) or check.get("status") != "PASS":
            raise ValueError(f"{name}: a PASS check is required")
        reason = check.get("reason")
        if not isinstance(reason, str) or not reason.strip():
            raise ValueError(f"{name}: a review reason is required")
    reviewed_at = receipt.get("reviewed_at")
    if not isinstance(reviewed_at, str):
        raise ValueError("reviewed_at must be an ISO UTC timestamp")
    try:
        timestamp = datetime.fromisoformat(reviewed_at.replace("Z", "+00:00"))
    except ValueError as exc:
        raise ValueError("reviewed_at must be an ISO UTC timestamp") from exc
    if "T" not in reviewed_at or timestamp.tzinfo is None or timestamp.utcoffset() != timezone.utc.utcoffset(timestamp):
        raise ValueError("reviewed_at must be an ISO UTC timestamp")
    context_hash = receipt.get("context_sha256")
    if not isinstance(context_hash, str) or not re.fullmatch(r"[0-9a-f]{64}", context_hash):
        raise ValueError("context_sha256 must be a SHA-256 hex digest")
    if context is None:
        raise ValueError("review receipt requires --context-file")
    if context_hash != text_sha256(context):
        raise ValueError("context differs from reviewed context")
    return reviewer


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("message_file", type=Path)
    parser.add_argument("receipt_file", type=Path)
    parser.add_argument("--sender", required=True)
    parser.add_argument("--context-file", type=Path, required=True)
    args = parser.parse_args()
    try:
        message = args.message_file.read_text(encoding="utf-8")
        receipt = json.loads(args.receipt_file.read_text(encoding="utf-8"))
        context = args.context_file.read_text(encoding="utf-8")
        reviewer = validate(message, receipt, args.sender, context)
    except (OSError, UnicodeError, ValueError) as exc:
        print(f"REJECT: {exc}", file=sys.stderr)
        return 1
    print(f"APPROVE: exact text reviewed by {reviewer}; native identity must be verified by coordinator")
    return 0


if __name__ == "__main__":
    sys.exit(main())
