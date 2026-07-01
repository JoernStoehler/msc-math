#!/usr/bin/env python3
"""Check that evidence paths cited by an AI-use provenance report exist."""

from __future__ import annotations

import argparse
import json
import re
from datetime import datetime, timezone
from pathlib import Path


PATH_RE = re.compile(
    r"(?P<path>/(?:home|tmp|workspaces)/[^\s`|)>\]]+?(?:\.jsonl|\.md|\.txt))"
)


def extract_paths(text: str) -> list[str]:
    paths: list[str] = []
    seen: set[str] = set()
    for match in PATH_RE.finditer(text):
        path = match.group("path").rstrip(".,;:")
        if path not in seen:
            seen.add(path)
            paths.append(path)
    return paths


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", help="Markdown report to check")
    parser.add_argument("--out", required=True, help="Output JSON path")
    args = parser.parse_args()

    report = Path(args.report)
    text = report.read_text(encoding="utf-8")
    rows = []
    for raw in extract_paths(text):
        path = Path(raw)
        exists = path.exists()
        rows.append(
            {
                "path": raw,
                "exists": exists,
                "size_bytes": path.stat().st_size if exists else None,
            }
        )

    missing = [row["path"] for row in rows if not row["exists"]]
    output = {
        "generated_at": datetime.now(tz=timezone.utc).isoformat(),
        "report": str(report),
        "total_paths": len(rows),
        "missing_paths": missing,
        "paths": rows,
    }

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, indent=2), encoding="utf-8")
    print(f"checked {len(rows)} cited paths; missing={len(missing)}")
    if missing:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
