#!/usr/bin/env python3
"""Build the final tracked repository state plus checked thesis PDF."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import tempfile
import zipfile
from pathlib import Path


LFS_PATTERN = re.compile(
    rb"\Aversion https://git-lfs.github.com/spec/v1\n"
    rb"oid sha256:([0-9a-f]{64})\nsize ([0-9]+)\n?\Z"
)
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


def lfs_identity(data: bytes) -> tuple[str, int] | None:
    match = LFS_PATTERN.fullmatch(data)
    if match is None:
        return None
    return match.group(1).decode(), int(match.group(2))


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
        committed = git(root, "show", f"HEAD:{path}")
        identity = lfs_identity(committed)
        if identity is None:
            data = committed
            lfs_oid = None
            lfs_size = None
        else:
            lfs_oid, lfs_size = identity
            data = (root / path).read_bytes()
            if lfs_identity(data) is not None:
                raise ValueError(f"Git LFS file is not hydrated: {path}")
            if len(data) != lfs_size or digest(data) != lfs_oid:
                raise ValueError(f"Git LFS payload does not match HEAD: {path}")
        entry = {
            "path": path,
            "bytes": len(data),
            "sha256": digest(data),
            "lfs_oid": lfs_oid,
            "lfs_size": lfs_size,
        }
        payloads.append((path, data, entry))

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
