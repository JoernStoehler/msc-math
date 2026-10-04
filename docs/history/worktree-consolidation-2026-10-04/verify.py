#!/usr/bin/env python3
"""Check unchanged candidate evidence and explicitly recorded integration drift."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]


def verify():
    record = json.loads((HERE / "verification.json").read_text())
    original = REPO / "docs/history/thesis-continuation-2026-10-04"
    spec = importlib.util.spec_from_file_location("original_verifier", original / "verify-candidate.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    actual = sorted(set(module.verify(REPO, original)))
    expected = sorted("Changed: " + p for p in record["changed_repository_documentation_sha256"])
    assert actual == expected, ("Unexpected candidate/evidence drift", actual, expected)
    for name, digest in record["changed_repository_documentation_sha256"].items():
        assert name.endswith((".md", ".json", ".dot", ".svg", ".html", ".py")), name
        assert hashlib.sha256((REPO / name).read_bytes()).hexdigest() == digest, name
    print("PASS: original manuscript/PDF and acceptance evidence unchanged; recorded documentation drift matches.")
    print("Identity only; selected inclusion, human PASS and full reproduction remain unresolved.")


if __name__ == "__main__":
    verify()
