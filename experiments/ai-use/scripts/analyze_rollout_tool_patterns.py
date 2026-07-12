#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# ///

"""Count coarse tool/command patterns in selected Codex rollout logs.

This is a diagnostic proxy for repeated repository reads. It does not measure
file bytes, cache keys, or tool-output token volume.
"""

from __future__ import annotations

import argparse
import csv
import json
import re
from collections import Counter, defaultdict
from pathlib import Path


COMMANDS = (
    "rg", "sed", "cat", "find", "git", "ls", "pwd", "cargo", "python",
    "uv", "latexmk", "apply_patch",
)
PATH_PATTERN = re.compile(
    r"(?:(?:/workspaces/msc-math/)?(?:thesis|crates|experiments|formal|\.agents)/"
    r"[A-Za-z0-9_./-]+|(?:AGENTS|README|FACTSHEET|PROJECT_COMPLETION)\.md)"
)


def args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rollout-csv", type=Path, required=True)
    parser.add_argument("--start", required=True, help="Inclusive UTC date")
    parser.add_argument("--end", required=True, help="Inclusive UTC date")
    parser.add_argument("--out-dir", type=Path, required=True)
    return parser.parse_args()


def read_paths(path: Path, start: str, end: str) -> dict[str, set[Path]]:
    paths: dict[str, set[Path]] = defaultdict(set)
    with path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            if start <= row["date"] <= end:
                paths[row["date"][:7]].add(Path(row["path"]))
    return paths


def main() -> None:
    parsed = args()
    by_month = read_paths(parsed.rollout_csv, parsed.start, parsed.end)
    rows: list[dict[str, object]] = []
    for month, paths in sorted(by_month.items()):
        tools: Counter[str] = Counter()
        commands: Counter[str] = Counter()
        refs: Counter[str] = Counter()
        files_read = 0
        for path in paths:
            if not path.exists():
                continue
            files_read += 1
            with path.open(encoding="utf-8") as handle:
                for line in handle:
                    try:
                        event = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    if not event.get("timestamp", "").startswith(month):
                        continue
                    payload = event.get("payload") or {}
                    if payload.get("type") not in {"function_call", "custom_tool_call"}:
                        continue
                    raw = f"{payload.get('arguments', '')} {payload.get('input', '')}"
                    tools[str(payload.get("name", ""))] += 1
                    for command in COMMANDS:
                        if re.search(rf"(?<![A-Za-z0-9_]){re.escape(command)}(?![A-Za-z0-9_])", raw):
                            commands[command] += 1
                    for reference in PATH_PATTERN.findall(raw):
                        refs[reference.rstrip("`);,")] += 1
        for kind, counts in (("tool", tools), ("command", commands), ("path_reference", refs)):
            for item, count in counts.most_common():
                rows.append({
                    "month": month,
                    "kind": kind,
                    "item": item,
                    "count": count,
                    "rollout_files_read": files_read,
                })
    parsed.out_dir.mkdir(parents=True, exist_ok=True)
    with (parsed.out_dir / "tool-patterns.csv").open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0])) if rows else None
        if writer:
            writer.writeheader()
            writer.writerows(rows)
    (parsed.out_dir / "summary.json").write_text(
        json.dumps({
            "start": parsed.start,
            "end": parsed.end,
            "months": sorted(by_month),
            "interpretation_boundary": (
                "Command and path-reference counts are proxies; they do not "
                "measure exact file reads, cache keys, or tool-output tokens."
            ),
        }, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"wrote {len(rows)} tool-pattern rows to {parsed.out_dir}")


if __name__ == "__main__":
    main()
