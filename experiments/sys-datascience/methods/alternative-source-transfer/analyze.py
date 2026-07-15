#!/usr/bin/env python3
"""Post-target analyzer; never generates targets or calls capacity."""
from __future__ import annotations
import argparse, json, math
from pathlib import Path
from validate_packet import validate, rows

def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("out", type=Path)
    p.add_argument("--targets", type=Path, required=True)
    args = p.parse_args()
    gate = validate(args.out)
    selected = {r["candidate_id"] for r in rows(args.out / "selection.jsonl")}
    target_rows = list(rows(args.targets))
    if not target_rows:
        raise SystemExit("failed/partial post-target artifact: empty target file")
    if {r.get("candidate_id") for r in target_rows} != selected:
        raise SystemExit("failed/partial post-target artifact: target union mismatch")
    if any(not isinstance(r.get("sys"), (int, float)) or not math.isfinite(float(r["sys"])) for r in target_rows):
        raise SystemExit("post-target rows require finite sys values")
    arms = {"rho": [], "ridge": [], "control": []}
    memberships = {r["candidate_id"]: r["memberships"] for r in rows(args.out / "selection.jsonl")}
    for row in target_rows:
        for arm in memberships[row["candidate_id"]]: arms[arm].append(float(row["sys"]))
    result = {"schema": "alternative-source-transfer-post-target-v1", "target_rows": len(target_rows), "means": {k: sum(v)/len(v) for k,v in arms.items()}, "rho_delta": sum(arms["rho"])/len(arms["rho"]) - sum(arms["control"])/len(arms["control"]), "ridge_delta": sum(arms["ridge"])/len(arms["ridge"]) - sum(arms["control"])/len(arms["control"]), "gate": gate}
    print(json.dumps(result, sort_keys=True))

if __name__ == "__main__": main()
