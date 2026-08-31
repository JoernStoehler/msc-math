#!/usr/bin/env python3
"""Build the final source, registered release data, and checked thesis PDF."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import tempfile
import zipfile
from pathlib import Path


DOWNLOADED_SUBMIT_PREFIXES = (
    "submit/anmeldung-",
    "submit/erklaerung-",
    "submit/hinweis-",
)


def git(root: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(root), *args])


def tracked_paths(root: Path) -> list[str]:
    return sorted(
        item.decode() for item in git(root, "ls-files", "-z").split(b"\0") if item
    )


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def verify_zip(path: Path) -> None:
    with zipfile.ZipFile(path) as archive:
        manifest = json.loads(archive.read("ARCHIVE-FILE-MANIFEST.json"))
        expected = {entry["path"]: entry for entry in manifest["files"]}
        if set(archive.namelist()) != set(expected) | {"ARCHIVE-FILE-MANIFEST.json"}:
            raise ValueError("ZIP inventory differs from embedded manifest")
        for name, entry in expected.items():
            data = archive.read(name)
            if len(data) != entry["bytes"] or digest(data) != entry["sha256"]:
                raise ValueError(f"ZIP content mismatch: {name}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--expected-commit", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    root = Path(__file__).resolve().parent.parent
    head = git(root, "rev-parse", "HEAD").decode().strip()
    if not re.fullmatch(r"[0-9a-f]{40}", args.expected_commit):
        raise ValueError("--expected-commit must be a full 40-character SHA")
    if head != args.expected_commit:
        raise ValueError(f"expected reviewed commit {args.expected_commit}, found {head}")
    if git(root, "status", "--porcelain=v1", "--untracked-files=no"):
        raise ValueError("tracked working tree must be clean")
    if args.output.exists():
        raise ValueError(f"refusing to overwrite {args.output}")

    paths = tracked_paths(root)
    cleanup_blockers = [
        path
        for path in paths
        if path.startswith("papers/") or path.startswith(DOWNLOADED_SUBMIT_PREFIXES)
    ]
    if cleanup_blockers:
        preview = "\n  ".join(cleanup_blockers[:20])
        raise ValueError(
            "final third-party cleanup is incomplete; remaining paths:\n  " + preview
        )

    payloads: list[tuple[str, bytes, dict[str, object]]] = []
    for path in paths:
        data = git(root, "show", f"HEAD:{path}")
        entry = {
            "path": path,
            "bytes": len(data),
            "sha256": digest(data),
        }
        payloads.append((path, data, entry))

    registry = json.loads((root / "artifacts/registry.json").read_text())
    for artifact, artifact_entry in sorted(registry["artifacts"].items()):
        if not artifact_entry.get("release", False):
            continue
        result = subprocess.run(
            [
                sys.executable,
                str(root / "scripts/artifacts.py"),
                "materialize",
                artifact,
                "--no-link",
            ],
            cwd=root,
            text=True,
            stdout=subprocess.PIPE,
            check=True,
        )
        materialized = json.loads(result.stdout)
        files_directory = Path(materialized["directory"]) / "files"
        snapshot = artifact_entry["snapshot"]
        for source, target in sorted(artifact_entry.get("links", {}).items()):
            if target in paths or any(name == target for name, _, _ in payloads):
                raise ValueError(f"external artifact target collides in archive: {target}")
            data = (files_directory / source).read_bytes()
            payloads.append(
                (
                    target,
                    data,
                    {
                        "artifact": artifact,
                        "artifact_snapshot": snapshot,
                        "bytes": len(data),
                        "path": target,
                        "sha256": digest(data),
                    },
                )
            )

    subprocess.run(["latexmk", "-g"], cwd=root / "thesis", check=True)
    subprocess.run(["./check-build.sh"], cwd=root / "thesis", check=True)
    pdf = (root / "thesis/build/main.pdf").read_bytes()
    if not pdf.startswith(b"%PDF-"):
        raise ValueError("checked thesis output is not a PDF")
    pdf_name = "Stoehler-Probing-Viterbos-Conjecture.pdf"
    payloads.append(
        (
            pdf_name,
            pdf,
            {
                "path": pdf_name,
                "bytes": len(pdf),
                "sha256": digest(pdf),
                "role": "thesis_pdf",
            },
        )
    )

    manifest = {
        "schema_version": 1,
        "source_commit": head,
        "files": [entry for _, _, entry in payloads],
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        dir=args.output.parent, prefix=args.output.name + ".", delete=False
    ) as temporary:
        temporary_path = Path(temporary.name)
    try:
        with zipfile.ZipFile(temporary_path, "w", zipfile.ZIP_DEFLATED) as archive:
            for name, data, _ in payloads:
                archive.writestr(name, data)
            archive.writestr(
                "ARCHIVE-FILE-MANIFEST.json",
                json.dumps(manifest, indent=2, sort_keys=True) + "\n",
            )
        verify_zip(temporary_path)
        temporary_path.replace(args.output)
    except BaseException:
        temporary_path.unlink(missing_ok=True)
        raise
    print(args.output)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, subprocess.CalledProcessError, ValueError) as error:
        raise SystemExit(f"release build failed: {error}")
