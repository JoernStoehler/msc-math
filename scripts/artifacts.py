#!/usr/bin/env python3
"""Publish and materialize immutable directory snapshots with rclone."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import sys
import tempfile
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
REGISTRY_PATH = REPO_ROOT / "artifacts" / "registry.json"


class ArtifactError(RuntimeError):
    pass


def canonical_json(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def relative_path(raw: str) -> PurePosixPath:
    path = PurePosixPath(raw)
    raw_parts = raw.split("/")
    if not raw or path.is_absolute() or any(part in {"", ".", ".."} for part in raw_parts):
        raise ArtifactError(f"unsafe relative path: {raw!r}")
    return path


def hash_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def file_records(directory: Path) -> list[dict[str, Any]]:
    if not directory.is_dir():
        raise ArtifactError(f"snapshot source is not a directory: {directory}")
    records: list[dict[str, Any]] = []
    for path in sorted(directory.rglob("*")):
        if path.is_symlink():
            raise ArtifactError(f"snapshot sources may not contain symlinks: {path}")
        if not path.is_file():
            continue
        relative = path.relative_to(directory).as_posix()
        relative_path(relative)
        records.append({"path": relative, "sha256": hash_file(path), "size": path.stat().st_size})
    if not records:
        raise ArtifactError(f"snapshot source contains no files: {directory}")
    return records


def identity(artifact: str, files: list[dict[str, Any]]) -> dict[str, Any]:
    allowed = "abcdefghijklmnopqrstuvwxyz0123456789-"
    if not artifact or any(character not in allowed for character in artifact):
        raise ArtifactError("artifact names use lowercase letters, digits, and hyphens")
    normalized = sorted(files, key=lambda record: record["path"])
    for record in normalized:
        relative_path(str(record["path"]))
        if not isinstance(record.get("size"), int) or record["size"] < 0:
            raise ArtifactError(f"invalid size for {record.get('path')}")
        digest = record.get("sha256")
        if (
            not isinstance(digest, str)
            or len(digest) != 64
            or any(c not in "0123456789abcdef" for c in digest)
        ):
            raise ArtifactError(f"invalid sha256 for {record.get('path')}")
    return {"artifact": artifact, "files": normalized, "schema_version": 1}


def snapshot_id(artifact: str, files: list[dict[str, Any]]) -> str:
    return hashlib.sha256(canonical_json(identity(artifact, files))).hexdigest()


def load_registry() -> dict[str, Any]:
    try:
        registry = json.loads(REGISTRY_PATH.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise ArtifactError(f"cannot read {REGISTRY_PATH}: {error}") from error
    if registry.get("schema_version") != 1 or not isinstance(registry.get("artifacts"), dict):
        raise ArtifactError(f"unsupported artifact registry: {REGISTRY_PATH}")
    linked_targets: set[str] = set()
    for artifact, entry in registry["artifacts"].items():
        identity(artifact, [])
        if not isinstance(entry, dict):
            raise ArtifactError(f"registry entry must be an object: {artifact}")
        snapshot = entry.get("snapshot")
        if not isinstance(snapshot, str) or len(snapshot) != 64 or any(
            character not in "0123456789abcdef" for character in snapshot
        ):
            raise ArtifactError(f"registry has no valid snapshot for {artifact}")
        links = entry.get("links", {})
        if not isinstance(links, dict):
            raise ArtifactError(f"registry links must be an object: {artifact}")
        for source, target in links.items():
            relative_path(source)
            normalized_target = relative_path(target).as_posix()
            if normalized_target in linked_targets:
                raise ArtifactError(
                    f"registry target is linked more than once: {normalized_target}"
                )
            linked_targets.add(normalized_target)
    return registry


def require_rclone() -> None:
    if shutil.which("rclone") is None:
        raise ArtifactError("rclone is required; see docs/artifacts.md")


def rclone_remote(registry: dict[str, Any]) -> tuple[str, str]:
    remote = os.environ.get(
        "MSC_MATH_RCLONE_REMOTE", registry.get("default_remote", "mscmath")
    ).rstrip(":")
    bucket = os.environ.get("MSC_MATH_R2_BUCKET", registry.get("bucket", ""))
    if not remote or not bucket or "/" in bucket:
        raise ArtifactError("invalid rclone remote or R2 bucket")
    return remote, bucket


def remote_snapshot(registry: dict[str, Any], artifact: str, snapshot: str) -> str:
    remote, bucket = rclone_remote(registry)
    prefix = registry.get("prefix", "snapshots").strip("/")
    return f"{remote}:{bucket}/{prefix}/{artifact}/{snapshot}"


def run_rclone(*arguments: str, capture: bool = False) -> subprocess.CompletedProcess[str]:
    require_rclone()
    process = subprocess.run(
        ["rclone", *arguments, "--s3-acl", "", "--s3-no-check-bucket"],
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.PIPE if capture else None,
        check=False,
    )
    if process.returncode:
        detail = (process.stderr or process.stdout or "").strip()
        raise ArtifactError(f"rclone {' '.join(arguments)} failed: {detail}")
    return process


def remote_manifest(base: str) -> bytes | None:
    listing = run_rclone(
        "lsf", base, "--files-only", "--include", "manifest.json", capture=True
    ).stdout
    if "manifest.json" not in listing.splitlines():
        return None
    return run_rclone("cat", f"{base}/manifest.json", capture=True).stdout.encode()


def parse_manifest(raw: bytes, expected_artifact: str, expected_snapshot: str) -> dict[str, Any]:
    try:
        manifest = json.loads(raw)
    except json.JSONDecodeError as error:
        raise ArtifactError(f"invalid remote manifest: {error}") from error
    files = manifest.get("files")
    if not isinstance(files, list):
        raise ArtifactError("manifest files must be a list")
    computed = snapshot_id(expected_artifact, files)
    if manifest.get("schema_version") != 1 or manifest.get("artifact") != expected_artifact:
        raise ArtifactError("remote manifest identity does not match the requested artifact")
    if manifest.get("snapshot") != expected_snapshot or computed != expected_snapshot:
        raise ArtifactError("remote manifest snapshot digest does not match its file inventory")
    return manifest


def default_cache_root() -> Path:
    configured = os.environ.get("MSC_MATH_CACHE_ROOT")
    if configured:
        return Path(configured).expanduser()
    data_cache = Path("/data/cache")
    if data_cache.is_dir() and os.access(data_cache, os.W_OK):
        return data_cache / "msc-math"
    xdg_cache = Path(os.environ.get("XDG_CACHE_HOME", Path.home() / ".cache"))
    return xdg_cache / "msc-math" / "artifacts"


def verify_directory(directory: Path, files: list[dict[str, Any]]) -> None:
    expected = {relative_path(str(record["path"])).as_posix(): record for record in files}
    actual: set[str] = set()
    for path in directory.rglob("*"):
        if path.is_symlink():
            raise ArtifactError(f"materialized snapshot contains a symlink: {path}")
        if path.is_file():
            actual.add(path.relative_to(directory).as_posix())
    if actual != set(expected):
        missing = sorted(set(expected) - actual)
        extra = sorted(actual - set(expected))
        raise ArtifactError(
            f"materialized file inventory mismatch; missing={missing}, extra={extra}"
        )
    for relative, record in expected.items():
        path = directory / relative
        if path.stat().st_size != record["size"] or hash_file(path) != record["sha256"]:
            raise ArtifactError(f"materialized file failed size/hash verification: {relative}")


def install_links(entry: dict[str, Any], files_directory: Path) -> None:
    links = entry.get("links", {})
    if not isinstance(links, dict):
        raise ArtifactError("registry links must be an object")
    for source_raw, target_raw in sorted(links.items()):
        source_rel = relative_path(source_raw)
        target_rel = relative_path(target_raw)
        source = files_directory.joinpath(*source_rel.parts)
        target = REPO_ROOT.joinpath(*target_rel.parts)
        if not source.is_file():
            raise ArtifactError(f"registry link source is absent from snapshot: {source_raw}")
        target.parent.mkdir(parents=True, exist_ok=True)
        if target.is_symlink():
            if target.resolve() == source.resolve():
                continue
            raise ArtifactError(f"refusing to replace a different symlink: {target}")
        if target.exists():
            raise ArtifactError(f"refusing to replace an existing path: {target}")
        target.symlink_to(source)


def publish(args: argparse.Namespace) -> None:
    registry = load_registry()
    source = Path(args.directory).resolve()
    files = file_records(source)
    snapshot = snapshot_id(args.artifact, files)
    base = remote_snapshot(registry, args.artifact, snapshot)
    manifest = {
        **identity(args.artifact, files),
        "snapshot": snapshot,
    }
    existing = remote_manifest(base)
    if existing is not None:
        parse_manifest(existing, args.artifact, snapshot)
        run_rclone("check", str(source), f"{base}/files", "--download")
        print(
            json.dumps(
                {
                    "artifact": args.artifact,
                    "snapshot": snapshot,
                    "status": "already-published",
                }
            )
        )
        return
    run_rclone("copy", str(source), f"{base}/files", "--immutable")
    run_rclone("check", str(source), f"{base}/files", "--download")
    with tempfile.NamedTemporaryFile("wb", delete=False) as handle:
        manifest_path = Path(handle.name)
        handle.write(canonical_json(manifest))
    try:
        run_rclone("copyto", str(manifest_path), f"{base}/manifest.json", "--immutable")
    finally:
        manifest_path.unlink(missing_ok=True)
    published = remote_manifest(base)
    if published is None:
        raise ArtifactError("published manifest is not readable")
    parse_manifest(published, args.artifact, snapshot)
    print(json.dumps({"artifact": args.artifact, "snapshot": snapshot, "status": "published"}))


def materialize(args: argparse.Namespace) -> None:
    registry = load_registry()
    try:
        entry = registry["artifacts"][args.artifact]
    except KeyError as error:
        raise ArtifactError(f"unknown artifact: {args.artifact}") from error
    snapshot = entry["snapshot"]
    base = remote_snapshot(registry, args.artifact, snapshot)
    raw_manifest = remote_manifest(base)
    if raw_manifest is None:
        raise ArtifactError(
            f"remote snapshot has no completion manifest: {args.artifact}/{snapshot}"
        )
    manifest = parse_manifest(raw_manifest, args.artifact, snapshot)
    cache_root = Path(args.cache_root).expanduser() if args.cache_root else default_cache_root()
    artifact_cache = cache_root.resolve() / args.artifact
    snapshot_directory = artifact_cache / snapshot
    files_directory = snapshot_directory / "files"
    if snapshot_directory.exists():
        verify_directory(files_directory, manifest["files"])
        local_manifest = snapshot_directory / "manifest.json"
        if not local_manifest.is_file() or local_manifest.read_bytes() != raw_manifest:
            raise ArtifactError(
                f"cached manifest differs from remote snapshot: {snapshot_directory}"
            )
    else:
        artifact_cache.mkdir(parents=True, exist_ok=True)
        temporary_directory = Path(tempfile.mkdtemp(prefix=f".{snapshot}.", dir=artifact_cache))
        try:
            temporary_files = temporary_directory / "files"
            temporary_files.mkdir()
            run_rclone("copy", f"{base}/files", str(temporary_files), "--immutable")
            verify_directory(temporary_files, manifest["files"])
            (temporary_directory / "manifest.json").write_bytes(raw_manifest)
            temporary_directory.rename(snapshot_directory)
        except BaseException:
            shutil.rmtree(temporary_directory, ignore_errors=True)
            raise
    if not args.no_link:
        install_links(entry, files_directory)
    print(
        json.dumps(
            {
                "artifact": args.artifact,
                "directory": str(snapshot_directory),
                "snapshot": snapshot,
            }
        )
    )


def list_artifacts(_: argparse.Namespace) -> None:
    registry = load_registry()
    for name, entry in sorted(registry["artifacts"].items()):
        print(f"{name}\t{entry.get('snapshot', 'unpublished')}\t{entry.get('description', '')}")


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    subparsers = result.add_subparsers(dest="command", required=True)
    publish_parser = subparsers.add_parser(
        "publish", help="publish a content-addressed directory snapshot"
    )
    publish_parser.add_argument("artifact")
    publish_parser.add_argument("directory")
    publish_parser.set_defaults(function=publish)
    materialize_parser = subparsers.add_parser(
        "materialize", help="download, verify, and link a registered snapshot"
    )
    materialize_parser.add_argument("artifact")
    materialize_parser.add_argument("--cache-root")
    materialize_parser.add_argument("--no-link", action="store_true")
    materialize_parser.set_defaults(function=materialize)
    list_parser = subparsers.add_parser("list", help="list registered shared artifacts")
    list_parser.set_defaults(function=list_artifacts)
    return result


def main() -> int:
    try:
        args = parser().parse_args()
        args.function(args)
    except ArtifactError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
