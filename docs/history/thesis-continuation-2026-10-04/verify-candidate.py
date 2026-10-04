#!/usr/bin/env python3
"""Verify the retained continuation's byte identity without running producers.

Identity does not establish mathematical correctness, visual quality, remote
availability or human acceptance. Read the independently owned reports.
"""

import hashlib
import json
from pathlib import Path
import subprocess
import sys


def verify(repo: Path, record: Path) -> list[str]:
    mismatches = []

    def read_json(name):
        return json.loads((record / name).read_text())

    def check(relative, expected):
        path = repo / relative
        if not path.resolve().is_relative_to(repo) or not path.is_file():
            mismatches.append(f"Missing or out-of-repository file: {relative}")
        elif hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            mismatches.append(f"Changed: {relative}")

    manifest = read_json("candidate-manifest.json")
    sources = manifest["source_files_sha256"]
    for relative, expected in sorted(sources.items()):
        check(relative, expected)
    identity = "".join(f"{p}\0{h}\n" for p, h in sorted(sources.items()))
    if hashlib.sha256(identity.encode()).hexdigest() != manifest["source_tree_sha256"]:
        mismatches.append("Source identity differs from the declared file hashes")
    tracked = set(subprocess.check_output(
        ["git", "ls-files", "-z", "--", "thesis"], cwd=repo
    ).decode().rstrip("\0").split("\0"))
    if tracked != set(sources):
        mismatches.append("Tracked thesis file inventory differs from the candidate")
    check(manifest["pdf"]["path"], manifest["pdf"]["sha256"])
    check(str(record.relative_to(repo) / "thesis-build.log"), manifest["build_log_sha256"])

    evidence = read_json("verification-manifest.json")
    for relative, expected in evidence["repository_files_sha256"].items():
        check(relative, expected)

    acceptance = read_json("acceptance-manifest.json")
    source = acceptance["source"]
    check(source["candidate_manifest_path"], source["candidate_manifest_sha256"])
    for section in ("reports_sha256", "accepted_receipts_sha256"):
        for relative, expected in acceptance[section].items():
            check(str(record.relative_to(repo) / relative), expected)
    for relative, expected in acceptance.get("reviewed_repository_files_sha256", {}).items():
        check(relative, expected)
    if acceptance["pdf"]["sha256"] != manifest["pdf"]["sha256"]:
        mismatches.append("Acceptance binds a different PDF")
    if source["tree_sha256"] != manifest["source_tree_sha256"]:
        mismatches.append("Acceptance binds different thesis sources")
    return mismatches


def main():
    record = Path(__file__).resolve().parent
    repo = record.parents[2]
    try:
        mismatches = verify(repo, record)
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"FAIL: incomplete or invalid candidate record: {error}", file=sys.stderr)
        return 1
    if mismatches:
        print("FAIL: candidate identity differs from this checkout", file=sys.stderr)
        for mismatch in mismatches:
            print(mismatch, file=sys.stderr)
        return 1
    print("PASS: thesis inventory, source/PDF hashes, retained evidence and acceptance bindings match.")
    print("Identity only; consult acceptance reports for scope and limitations.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
