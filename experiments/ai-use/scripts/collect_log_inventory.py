#!/usr/bin/env python3
"""Inventory visible Codex and Claude session logs for the AI-use packet."""

from __future__ import annotations

import argparse
import json
import os
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


CODEX_HOME = Path(os.environ.get("CODEX_HOME", Path.home() / ".codex"))
DEFAULT_CODEX_ROOTS = [
    CODEX_HOME / "sessions",
    CODEX_HOME / "archived_sessions",
    CODEX_HOME / "imported_session_logs",
]


def iso_mtime(path: Path) -> str:
    return datetime.fromtimestamp(path.stat().st_mtime, tz=timezone.utc).isoformat()


def read_jsonl_prefix(path: Path, limit: int = 200) -> list[dict[str, Any]]:
    events: list[dict[str, Any]] = []
    try:
        with path.open("r", encoding="utf-8", errors="replace") as handle:
            for i, line in enumerate(handle):
                if i >= limit:
                    break
                try:
                    event = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if isinstance(event, dict):
                    events.append(event)
    except OSError:
        pass
    return events


def codex_metadata(path: Path) -> dict[str, Any]:
    metadata: dict[str, Any] = {
        "kind": "codex",
        "path": str(path),
        "session_id": "",
        "cwd": "",
        "forked_from_id": "",
    }
    for event in read_jsonl_prefix(path):
        payload = event.get("payload")
        if not isinstance(payload, dict):
            continue
        if event.get("type") == "session_meta":
            metadata["session_id"] = payload.get("id", "") or metadata["session_id"]
            metadata["forked_from_id"] = (
                payload.get("forked_from_id", "") or metadata["forked_from_id"]
            )
            metadata["cwd"] = payload.get("cwd", "") or metadata["cwd"]
            git = payload.get("git")
            if isinstance(git, dict):
                metadata["cwd"] = git.get("cwd", "") or metadata["cwd"]
        if payload.get("type") == "turn_context":
            metadata["cwd"] = payload.get("cwd", "") or metadata["cwd"]
    return metadata


def claude_metadata(path: Path) -> dict[str, Any]:
    session_id = path.stem
    project = ""
    try:
        parts = path.parts
        idx = parts.index("projects")
        project = parts[idx + 1]
    except (ValueError, IndexError):
        project = ""
    return {
        "kind": "claude",
        "path": str(path),
        "session_id": session_id,
        "project": project,
    }


def iter_logs(
    codex_roots: list[Path], claude_roots: list[Path]
) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for root in codex_roots:
        if not root.exists():
            continue
        for path in sorted(root.rglob("rollout-*.jsonl")):
            row = codex_metadata(path)
            row["root"] = str(root)
            row["size_bytes"] = path.stat().st_size
            row["modified_iso"] = iso_mtime(path)
            rows.append(row)
    for root in claude_roots:
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.jsonl")):
            if not re.search(r"(msc-math|msc-viterbo|workspaces-msc)", str(path)):
                continue
            row = claude_metadata(path)
            row["root"] = str(root)
            row["size_bytes"] = path.stat().st_size
            row["modified_iso"] = iso_mtime(path)
            rows.append(row)
    return rows


def require_disjoint_roots(
    parser: argparse.ArgumentParser, label: str, roots: list[Path]
) -> None:
    resolved = [(root, root.resolve(strict=True)) for root in roots if root.is_dir()]
    for index, (left, left_resolved) in enumerate(resolved):
        for right, right_resolved in resolved[index + 1 :]:
            if (
                left_resolved == right_resolved
                or left_resolved in right_resolved.parents
                or right_resolved in left_resolved.parents
            ):
                parser.error(
                    f"{label} roots must not repeat or overlap: {left}, {right}"
                )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, help="Output JSON path")
    parser.add_argument(
        "--codex-root",
        action="append",
        type=Path,
        help="Codex rollout root; repeat to add roots (replaces CODEX_HOME defaults)",
    )
    parser.add_argument(
        "--claude-root",
        action="append",
        type=Path,
        default=[],
        help="Explicitly staged Claude log root; repeat to add roots",
    )
    args = parser.parse_args()

    explicit_codex_roots = args.codex_root is not None
    codex_roots = args.codex_root or DEFAULT_CODEX_ROOTS
    missing_codex = [root for root in codex_roots if not root.is_dir()]
    if explicit_codex_roots and missing_codex:
        parser.error(
            "explicit Codex roots do not exist: "
            + ", ".join(str(root) for root in missing_codex)
        )
    if len(missing_codex) == len(codex_roots):
        parser.error(
            "none of the Codex roots exists; set CODEX_HOME or pass --codex-root"
        )
    missing_claude = [root for root in args.claude_root if not root.is_dir()]
    if missing_claude:
        parser.error(
            "explicit Claude roots do not exist: "
            + ", ".join(str(root) for root in missing_claude)
        )
    require_disjoint_roots(parser, "Codex", codex_roots)
    require_disjoint_roots(parser, "Claude", args.claude_root)
    require_disjoint_roots(
        parser, "Codex and Claude", [*codex_roots, *args.claude_root]
    )

    rows = iter_logs(codex_roots, args.claude_root)
    by_kind: dict[str, int] = {}
    by_cwd: dict[str, int] = {}
    codex_root_counts = {str(root): 0 for root in codex_roots if root.is_dir()}
    claude_root_counts = {str(root): 0 for root in args.claude_root}
    for row in rows:
        kind = row.get("kind", "unknown")
        by_kind[kind] = by_kind.get(kind, 0) + 1
        root = str(row.get("root", ""))
        if kind == "codex" and root in codex_root_counts:
            codex_root_counts[root] += 1
        elif kind == "claude" and root in claude_root_counts:
            claude_root_counts[root] += 1
        cwd = row.get("cwd") or row.get("project") or ""
        if cwd:
            by_cwd[cwd] = by_cwd.get(cwd, 0) + 1
    empty_explicit_codex = [
        root
        for root in codex_roots
        if explicit_codex_roots and codex_root_counts.get(str(root), 0) == 0
    ]
    if empty_explicit_codex:
        parser.error(
            "explicit Codex roots contain no rollout files: "
            + ", ".join(str(root) for root in empty_explicit_codex)
        )
    empty_explicit_claude = [
        root
        for root in args.claude_root
        if claude_root_counts.get(str(root), 0) == 0
    ]
    if empty_explicit_claude:
        parser.error(
            "explicit Claude roots contain no matching project logs: "
            + ", ".join(str(root) for root in empty_explicit_claude)
        )
    if by_kind.get("codex", 0) == 0:
        parser.error("no Codex rollout files found below the available roots")

    output = {
        "generated_at": datetime.now(tz=timezone.utc).isoformat(),
        "codex_roots": [str(root) for root in codex_roots],
        "codex_roots_scanned": [str(root) for root in codex_roots if root.is_dir()],
        "missing_default_codex_roots": [
            str(root) for root in missing_codex if not explicit_codex_roots
        ],
        "codex_root_log_counts": codex_root_counts,
        "claude_roots": [str(root) for root in args.claude_root],
        "claude_root_log_counts": claude_root_counts,
        "total_logs": len(rows),
        "by_kind": by_kind,
        "by_cwd_or_project": dict(sorted(by_cwd.items())),
        "logs": rows,
    }

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, indent=2), encoding="utf-8")
    print(f"wrote {out_path} with {len(rows)} logs")


if __name__ == "__main__":
    main()
