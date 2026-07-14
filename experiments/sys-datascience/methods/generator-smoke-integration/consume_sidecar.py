#!/usr/bin/env python3
"""Tiny method-consumer example for an opt-in prepared smoke sidecar."""

import argparse
import json


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prepared", required=True)
    parser.add_argument("--law", required=True)
    parser.add_argument("--require-target", action="store_true")
    args = parser.parse_args()
    rows = []
    with open(args.prepared, encoding="utf-8") as handle:
        for line in handle:
            if line.strip():
                row = json.loads(line)
                if row["law"] == args.law and (not args.require_target or row["target_status"] == "evaluated"):
                    rows.append(row)
    print(json.dumps({"law": args.law, "rows": len(rows), "target_statuses": sorted({r["target_status"] for r in rows})}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
