#!/usr/bin/env python3
"""Reject active Git LFS attributes and pointer files in the tracked tree."""

from __future__ import annotations

import re
import subprocess
from pathlib import Path


LFS_POINTER_HEADER = b"version https://git-lfs.github.com/spec/v1"
LFS_ATTRIBUTE = re.compile(r"(?:^|\s)(?:filter|diff|merge)=lfs(?:\s|$)")


def tracked_paths(root: Path) -> list[Path]:
    output = subprocess.check_output(
        ["git", "-C", str(root), "ls-files", "-z"]
    )
    return [root / raw.decode() for raw in output.split(b"\0") if raw]


def find_problems(paths: list[Path]) -> list[str]:
    problems: list[str] = []
    for path in paths:
        if not path.exists() or path.is_symlink():
            continue
        if path.name == ".gitattributes":
            for number, line in enumerate(
                path.read_text(encoding="utf-8").splitlines(), start=1
            ):
                if LFS_ATTRIBUTE.search(line):
                    problems.append(f"{path}:{number}: active Git LFS attribute")
        with path.open("rb") as stream:
            first_line = stream.readline(len(LFS_POINTER_HEADER) + 3).rstrip(b"\r\n")
        if first_line == LFS_POINTER_HEADER:
            problems.append(f"{path}: Git LFS pointer")
    return problems


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    problems = find_problems(tracked_paths(root))
    if problems:
        print("Git LFS is retired; use the registered R2 artifact workflow:")
        for problem in problems:
            print(f"  {problem}")
        return 1
    print("No active Git LFS attributes or pointers in tracked files.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
