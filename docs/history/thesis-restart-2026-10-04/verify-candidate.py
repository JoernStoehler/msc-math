#!/usr/bin/env python3
"""Check retained candidate, source and evidence bytes without rebuilding.

This verifies identity, not mathematics, layout, human acceptance or reproduction.
Run from any directory; the repository is resolved from this file's location.
"""

import hashlib
import json
from pathlib import Path
import sys


def main():
    record = Path(__file__).resolve().parent
    repo = record.parents[2]
    manifest = json.loads((record / "candidate-manifest.json").read_text())
    mismatches = []

    def check(relative, expected):
        path = repo / relative
        if not path.resolve().is_relative_to(repo) or not path.is_file():
            mismatches.append(f"Missing or out-of-repository file: {relative}")
            return
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != expected:
            mismatches.append(f"Changed: {relative}")

    sources = manifest["source_files_sha256"]
    for relative, expected in sorted(sources.items()):
        check(relative, expected)
    identity = "".join(
        f"{relative}\0{digest}\n" for relative, digest in sorted(sources.items())
    )
    if hashlib.sha256(identity.encode()).hexdigest() != manifest["source_tree_sha256"]:
        mismatches.append("Source identity does not match the manifest's file hashes")
    check(manifest["pdf_path"], manifest["pdf_sha256"])
    check(manifest["build_log_path"], manifest["build_log_sha256"])
    evidence = json.loads((record / "verification-manifest.json").read_text())
    for relative, expected in evidence["files_sha256"].items():
        check(str(record.relative_to(repo) / relative), expected)
    acceptance = json.loads((record / "acceptance-manifest.json").read_text())
    check(
        acceptance["source"]["candidate_manifest_path"],
        acceptance["source"]["candidate_manifest_sha256"],
    )
    for relative, expected in acceptance["reports_sha256"].items():
        check(str(record.relative_to(repo) / relative), expected)
    for relative, expected in acceptance["numerical_changes_sha256"].items():
        check(relative, expected)
    if acceptance["pdf"]["sha256"] != manifest["pdf_sha256"]:
        mismatches.append("Acceptance refers to a different PDF")
    if acceptance["source"]["tree_sha256"] != manifest["source_tree_sha256"]:
        mismatches.append("Acceptance refers to different thesis sources")
    rebuild = json.loads((record / "source-only-rebuild.json").read_text())
    if (rebuild["retained_pdf_sha256"] != manifest["pdf_sha256"]
            or rebuild["source_tree_sha256"] != manifest["source_tree_sha256"]):
        mismatches.append("Source-only rebuild refers to a different candidate")
    if mismatches:
        print("FAIL: candidate identity differs from this checkout", file=sys.stderr)
        for mismatch in mismatches:
            print(mismatch, file=sys.stderr)
        return 1
    print(
        f"PASS: {len(sources)} thesis-source hashes, PDF, build log and "
        f"{len(evidence['files_sha256'])} retained evidence files match"
    )
    print("Acceptance reports, numerical sources and source-only rebuild bind this candidate.")
    print(f"PDF SHA-256: {manifest['pdf_sha256']}")
    print("Identity only; consult acceptance reports for checked scope and limitations.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
