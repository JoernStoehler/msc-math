#!/usr/bin/env python3
"""Inventory visible Codex and Claude session logs for the AI-use packet."""

from __future__ import annotations

import argparse
import json
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


CODEX_ROOTS = [
    Path("/home/vscode/.codex/sessions"),
    Path("/home/vscode/.codex/archived_sessions"),
    Path("/home/vscode/.codex/imported_session_logs"),
]

CLAUDE_ROOTS = [
    Path("/home/vscode/.claude/projects/-workspaces-msc-math"),
    Path("/home/vscode/.claude/imported_session_logs"),
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


def iter_logs() -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for root in CODEX_ROOTS:
        if not root.exists():
            continue
        for path in sorted(root.rglob("rollout-*.jsonl")):
            row = codex_metadata(path)
            row["size_bytes"] = path.stat().st_size
            row["modified_iso"] = iso_mtime(path)
            rows.append(row)
    for root in CLAUDE_ROOTS:
        if not root.exists():
            continue
        for path in sorted(root.rglob("*.jsonl")):
            if not re.search(r"(msc-math|msc-viterbo|workspaces-msc)", str(path)):
                continue
            row = claude_metadata(path)
            row["size_bytes"] = path.stat().st_size
            row["modified_iso"] = iso_mtime(path)
            rows.append(row)
    return rows


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, help="Output JSON path")
    args = parser.parse_args()

    rows = iter_logs()
    by_kind: dict[str, int] = {}
    by_cwd: dict[str, int] = {}
    for row in rows:
        kind = row.get("kind", "unknown")
        by_kind[kind] = by_kind.get(kind, 0) + 1
        cwd = row.get("cwd") or row.get("project") or ""
        if cwd:
            by_cwd[cwd] = by_cwd.get(cwd, 0) + 1

    output = {
        "generated_at": datetime.now(tz=timezone.utc).isoformat(),
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
