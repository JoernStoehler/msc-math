#!/usr/bin/env python3
"""Capture an explicitly scoped Git source handoff; verify it without Git.

Selection is the caller's responsibility: tracked/nonignored is not a source-only
or secret-free guarantee. Inspect .cloud-handoff/manifest.json before transfer.
"""

import argparse
from datetime import datetime, timezone
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import posixpath
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile

META = ".cloud-handoff"
SCHEMA = "msc-math-cloud-handoff-v1"


def git(repo, *args, check=True, input=None):
    result = subprocess.run(["git", "-C", str(repo), *args], input=input,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if check and result.returncode:
        raise ValueError(result.stderr.decode(errors="replace").strip())
    return result


def safe_path(name):
    path = PurePosixPath(name)
    if (not name or path.is_absolute() or ".." in path.parts
            or ".git" in path.parts or "\x00" in name
            or path.as_posix() != name or name == "."):
        raise ValueError(f"unsafe repository path: {name!r}")
    return path


def selection_path(name):
    if name == ".":
        return name
    safe_path(name)
    if name == META or name.startswith(META + "/"):
        raise ValueError(f"reserved bundle path: {name}")
    return name


def selected(name, scopes):
    return any(scope == "." or name == scope or name.startswith(scope + "/")
               for scope in scopes)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def file_digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def link_destination(name, target, links, dependencies=None):
    """Resolve relative links lexically, including links in omitted scopes."""
    if posixpath.isabs(target):
        raise ValueError(f"nonrelocatable absolute symlink: {name} -> {target}")
    parts = name.split("/")[:-1] + target.split("/")
    resolved = []
    expansions = 0
    while parts:
        part = parts.pop(0)
        if part in ("", "."):
            continue
        if part == "..":
            if not resolved:
                raise ValueError(f"symlink escapes repository: {name} -> {target}")
            resolved.pop()
            continue
        resolved.append(part)
        current = "/".join(resolved)
        if current in links:
            if dependencies is not None:
                dependencies.append(current)
            expansions += 1
            if expansions > 40:
                raise ValueError(f"symlink cycle: {name}")
            next_target = links[current]
            if posixpath.isabs(next_target):
                raise ValueError(f"symlink escapes relocation: {name} via {current}")
            resolved.pop()
            parts = next_target.split("/") + parts
    destination = "/".join(resolved) or "."
    if ".git" in PurePosixPath(destination).parts:
        raise ValueError(f"symlink targets Git metadata: {name}")
    return destination


def working_inventory(repo):
    raw = git(repo, "ls-files", "--cached", "--others", "--exclude-standard", "-z").stdout
    names = sorted(set(os.fsdecode(p) for p in raw.split(b"\0") if p))
    # --no-index also excludes tracked paths matching ignore rules. No force-add
    # or ignored credential/cache exceptions are implied by selection.
    ignored_result = git(repo, "check-ignore", "--no-index", "-z", "--stdin",
                         input=raw, check=False)
    if ignored_result.returncode not in (0, 1):
        raise ValueError(ignored_result.stderr.decode(errors="replace").strip())
    ignored = {os.fsdecode(p) for p in ignored_result.stdout.split(b"\0") if p}
    return [p for p in names if p not in ignored], sorted(ignored)


def record_file(name, path):
    mode = stat.S_IMODE(path.lstat().st_mode)
    if path.is_symlink():
        target = os.readlink(path)
        data = os.fsencode(target)
        return {"path": name, "type": "symlink", "mode": mode,
                "target": target, "sha256": digest(data), "size": len(data)}
    return {"path": name, "type": "file", "mode": mode,
            "sha256": file_digest(path), "size": path.stat().st_size}


def capture_working(repo, stage, scopes):
    names, ignored = working_inventory(repo)
    links = {}
    for name in names:
        if (repo / name).is_symlink():
            links[name] = os.readlink(repo / name)
    missing = []
    for name in names:
        if not selected(name, scopes):
            continue
        safe_path(name)
        if name == META or name.startswith(META + "/"):
            raise ValueError(f"selection includes reserved bundle path: {name}")
        source = repo / name
        try:
            before = source.lstat()
        except FileNotFoundError:
            missing.append(name)
            continue
        if not (stat.S_ISREG(before.st_mode) or stat.S_ISLNK(before.st_mode)):
            raise ValueError(f"selected path is not a file/link (submodule?): {name}")
        if not source.resolve().is_relative_to(repo):
            raise ValueError(f"selected path resolves outside repository: {name}")
        destination = stage / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        if stat.S_ISLNK(before.st_mode):
            link_destination(name, links[name], links)
            destination.symlink_to(links[name])
        else:
            shutil.copyfile(source, destination)
            destination.chmod(stat.S_IMODE(before.st_mode))
        after = source.lstat()
        if (before.st_mode, before.st_size, before.st_mtime_ns, before.st_ctime_ns,
                before.st_ino) != (after.st_mode, after.st_size, after.st_mtime_ns,
                                  after.st_ctime_ns, after.st_ino):
            raise ValueError(f"source changed during capture: {name}; retry when edits stop")
    return links, {"ignored": [p for p in ignored if selected(p, scopes)],
                   "missing_tracked": missing, "untracked_ignored_not_enumerated": True}


def capture_revision(repo, stage, scopes, commit):
    # Git archive retains Git's executable and link modes and export attributes.
    archive = git(repo, "-c", "tar.umask=0022", "--literal-pathspecs", "archive", "--format=tar", commit,
                  "--", *scopes).stdout
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:") as bundle:
        for member in bundle:
            name = member.name.rstrip("/")
            safe_path(name)
            if name == META or name.startswith(META + "/"):
                raise ValueError(f"selection includes reserved bundle path: {name}")
            if member.isdir():
                continue  # Git emits structural parents of individually selected files.
            if not selected(name, scopes):
                raise ValueError(f"git archive emitted unselected path: {name}")
            destination = stage / name
            if member.issym():
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.symlink_to(member.linkname)
            elif member.isfile():
                destination.parent.mkdir(parents=True, exist_ok=True)
                with destination.open("wb") as stream:
                    shutil.copyfileobj(bundle.extractfile(member), stream)
                destination.chmod(member.mode)
            else:
                raise ValueError(f"unsupported archive entry: {name}")
    links = {}
    tree = git(repo, "ls-tree", "-rz", "--full-tree", commit).stdout
    for row in tree.split(b"\0"):
        if not row:
            continue
        metadata, name = row.split(b"\t", 1)
        mode, kind, oid = metadata.split()
        if mode == b"120000":
            links[os.fsdecode(name)] = os.fsdecode(git(repo, "cat-file", "blob", oid.decode()).stdout)
    return links, {"git_archive_export_attributes_apply": True}


def walk_files(root):
    result = []
    for directory, dirs, files in os.walk(root, followlinks=False):
        for name in list(dirs):
            path = Path(directory) / name
            if path.is_symlink():
                dirs.remove(name)
                files.append(name)
        result.extend(Path(directory) / name for name in files)
    return sorted(result)


def first_steps(manifest):
    return f"""SCOPED SOURCE HANDOFF — FIRST STEPS

1. Inspect .cloud-handoff/manifest.json and the tar member list before transfer.
   Selection does not guarantee source-only contents or absence of credentials.
2. Extract into an EMPTY directory, preserving POSIX modes and symlinks, e.g.:
     mkdir handoff && tar -xzf handoff.tar.gz -C handoff
3. Before running any included project code, verify with Python stdlib only:
     python3 handoff/.cloud-handoff/verify.py verify handoff
4. Read the selected project's entry points and the assigned task contract.
   This scoped bundle is not a clone; omitted dependencies/data stay omitted.

Identity: {manifest['snapshot_kind']}
Base commit (provenance only for working snapshots): {manifest['base_commit']}
Working snapshot bytes are identified by the manifest hashes, not that commit.
No atomic whole-repository snapshot is promised; stop edits during creation.
The manifest detects accidental changes, not malicious edits to the manifest.
Verification does not establish mathematical correctness or reproduction.
No upload, environment install, artifact access, or producer execution occurred.
"""


def create(args):
    repo = Path(git(args.repo, "rev-parse", "--show-toplevel").stdout.decode().strip()).resolve()
    output = Path(args.output).absolute()
    if os.path.lexists(output):
        raise ValueError(f"refusing to overwrite: {output}")
    scopes = sorted(set(selection_path(p) for p in args.path))
    commit = git(repo, "rev-parse", "--verify", "--end-of-options", (args.revision or "HEAD") + "^{commit}").stdout.decode().strip()
    dirty_before = bool(git(repo, "status", "--porcelain", "-z").stdout) if args.working_tree else None
    with tempfile.TemporaryDirectory(prefix="cloud-handoff-") as scratch:
        stage = Path(scratch) / "source"
        stage.mkdir()
        if args.working_tree:
            links, omissions = capture_working(repo, stage, scopes)
        else:
            links, omissions = capture_revision(repo, stage, scopes, commit)
        source_files = walk_files(stage)
        for scope in scopes:
            if not any(selected(p.relative_to(stage).as_posix(), [scope]) for p in source_files):
                raise ValueError(f"selection has no included files/links: {scope}")
        inventory = [record_file(p.relative_to(stage).as_posix(), p) for p in source_files]
        omitted_links = []
        included_names = {entry["path"] for entry in inventory}
        for entry in inventory:
            if entry["type"] == "symlink":
                dependencies = []
                target = link_destination(entry["path"], entry["target"], links, dependencies)
                missing_links = sorted(set(dependencies) - included_names)
                entry["required_symlinks"] = sorted(set(dependencies))
                entry["resolved_target"] = target
                entry["target_scope_selected"] = selected(target, scopes)
                present = bool(inventory) if target == "." else any(
                    p["path"] == target or p["path"].startswith(target + "/") for p in inventory)
                entry["target_has_included_entry"] = present
                if not entry["target_scope_selected"] or not present or missing_links:
                    omitted = {"link": entry["path"], "resolved_target": target}
                    if missing_links:
                        omitted["omitted_intermediate_links"] = missing_links
                    omitted_links.append(omitted)
        manifest = {"schema": SCHEMA, "created_utc": datetime.now(timezone.utc).isoformat(),
                    "snapshot_kind": "working-tree" if args.working_tree else "committed-revision",
                    "base_commit": commit, "selection": scopes,
                    "working_tree_dirty": dirty_before, "omissions": omissions,
                    "omitted_symlink_targets": omitted_links, "entries": inventory}
        if args.working_tree:
            manifest["working_tree_dirty_after_capture"] = bool(git(repo, "status", "--porcelain", "-z").stdout)
        metadata = stage / META
        metadata.mkdir()
        (metadata / "FIRST-STEPS.txt").write_text(first_steps(manifest))
        shutil.copyfile(Path(__file__), metadata / "verify.py")
        (metadata / "verify.py").chmod(0o644)
        inventory.extend(record_file(p.relative_to(stage).as_posix(), p)
                         for p in sorted(metadata.iterdir()))
        (metadata / "manifest.json").write_text(json.dumps(manifest, indent=2, ensure_ascii=True) + "\n")
        archive = Path(scratch) / "handoff.tar.gz"
        with tarfile.open(archive, "w:gz", dereference=False) as bundle:
            for path in walk_files(stage):
                bundle.add(path, arcname=path.relative_to(stage).as_posix(), recursive=False)
        # Exclusive publication also refuses an output created during capture.
        output.parent.mkdir(parents=True, exist_ok=True)
        with output.open("xb") as stream:
            try:
                with archive.open("rb") as source:
                    shutil.copyfileobj(source, stream)
            except BaseException:
                output.unlink()
                raise
    print(f"Created {output}: {manifest['snapshot_kind']}, {len(inventory)} files/links")
    print(f"Base commit: {commit}; inspect {META}/manifest.json before transfer")
    for item in omitted_links:
        print(f"OMITTED/PARTIAL symlink target: {item['link']} -> {item['resolved_target']}")
        if item.get("omitted_intermediate_links"):
            print(f"  missing intermediate links: {', '.join(item['omitted_intermediate_links'])}")


def verify(args):
    root = Path(args.directory).resolve()
    manifest_path = root / META / "manifest.json"
    if manifest_path.is_symlink() or not manifest_path.is_file():
        raise ValueError("missing regular .cloud-handoff/manifest.json")
    manifest = json.loads(manifest_path.read_text())
    if manifest.get("schema") != SCHEMA:
        raise ValueError("unsupported manifest schema")
    entries = manifest["entries"]
    names = [entry["path"] for entry in entries]
    if len(set(names)) != len(names):
        raise ValueError("duplicate manifest paths")
    for entry in entries:
        name = entry["path"]
        safe_path(name)
        path = root / name
        # No filesystem ancestor may redirect verification outside this bundle.
        if any(parent.is_symlink() for parent in path.parents if parent != root and root in parent.parents):
            raise ValueError(f"symlink ancestor: {name}")
        if not os.path.lexists(path):
            raise ValueError(f"missing listed path: {name}")
        mode = stat.S_IMODE(path.lstat().st_mode)
        if mode != entry["mode"]:
            raise ValueError(f"mode mismatch: {name} ({mode:o} != {entry['mode']:o})")
        if entry["type"] == "symlink":
            if not path.is_symlink() or os.readlink(path) != entry["target"]:
                raise ValueError(f"symlink mismatch: {name}")
            actual = digest(os.fsencode(os.readlink(path)))
        elif entry["type"] == "file":
            if not stat.S_ISREG(path.lstat().st_mode):
                raise ValueError(f"not a regular file: {name}")
            actual = file_digest(path)
            if path.stat().st_size != entry["size"]:
                raise ValueError(f"size mismatch: {name}")
        else:
            raise ValueError(f"unsupported entry type: {name}")
        if actual != entry["sha256"]:
            raise ValueError(f"hash mismatch: {name}")
    actual_names = {p.relative_to(root).as_posix() for p in walk_files(root)}
    if actual_names != set(names) | {f"{META}/manifest.json"}:
        raise ValueError(f"unlisted bundle paths: {sorted(actual_names - set(names) - {f'{META}/manifest.json'})}")
    links = {e["path"]: e["target"] for e in entries if e["type"] == "symlink"}
    for name, target in links.items():
        link_destination(name, target, links)
        # Existing targets must remain inside the relocated root; dangling,
        # explicitly omitted internal targets are allowed and reported.
        if not (root / name).resolve().is_relative_to(root):
            raise ValueError(f"symlink escapes extracted bundle: {name}")
    print(f"Verified {len(entries)} files/links: {manifest['snapshot_kind']}")
    if manifest["omitted_symlink_targets"]:
        print("Omitted/partial symlink targets remain; inspect manifest before work.")
        for item in manifest["omitted_symlink_targets"]:
            print(f"  {item['link']} -> {item['resolved_target']}")
            if item.get("omitted_intermediate_links"):
                print(f"  missing intermediate links: {', '.join(item['omitted_intermediate_links'])}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    build = commands.add_parser("create", help="create a scoped .tar.gz; inspect before transfer")
    build.add_argument("--repo", default=".")
    build.add_argument("--output", required=True)
    build.add_argument("--path", action="append", required=True, help="literal repo-relative scope (repeatable)")
    identity = build.add_mutually_exclusive_group(required=True)
    identity.add_argument("--revision", help="explicit committed revision; uses git archive")
    identity.add_argument("--working-tree", action="store_true", help="capture working bytes; HEAD is provenance only")
    check = commands.add_parser("verify", help="verify an extracted directory using Python stdlib, without Git")
    check.add_argument("directory")
    args = parser.parse_args()
    try:
        (create if args.command == "create" else verify)(args)
    except (ValueError, OSError, KeyError, json.JSONDecodeError, tarfile.TarError, RuntimeError) as error:
        parser.exit(1, f"error: {error}\n")


if __name__ == "__main__":
    main()
