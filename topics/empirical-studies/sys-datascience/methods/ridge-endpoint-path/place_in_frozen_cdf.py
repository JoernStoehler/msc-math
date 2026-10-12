#!/usr/bin/env python3
"""Place designed R values in the surviving frozen 1M candidate cache."""

import hashlib
import json
import re
import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parent / "artifacts"
R_PAT = re.compile(br'"ridge_symp_area_sum_over_volume_sqrt":([^,}]+)')


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--cache",
        type=Path,
        required=True,
        help="frozen 1M candidate-feature-table.jsonl; its path is not recorded in output",
    )
    return parser.parse_args()


def main():
    cache = parse_args().cache
    rows = [json.loads(line) for line in (ROOT / "candidates.jsonl").open()]
    values = {bucket: [] for bucket in ("3x6", "4x4")}
    digest = hashlib.sha256()
    with cache.open("rb", buffering=1024 * 1024) as f:
        for line in f:
            digest.update(line)
            if b'"product_k":3,"product_m":6' in line:
                bucket = "3x6"
            elif b'"product_k":4,"product_m":4' in line:
                bucket = "4x4"
            else:
                continue
            values[bucket].append(float(R_PAT.search(line).group(1)))
    out = []
    for row in rows:
        sample = values[row["bucket"]]
        count_le = sum(x <= row["edge_formula_r"] for x in sample)
        out.append({
            "candidate_id": row["candidate_id"],
            "bucket": row["bucket"],
            "edge_formula_r": row["edge_formula_r"],
            "frozen_bucket_rows": len(sample),
            "frozen_count_le": count_le,
            "frozen_empirical_cdf_le": count_le / len(sample),
            "rarity_bits_censored_lower_bound": (
                None if count_le else 16.609640474436812
            ),
        })
    report = {
        "cache_role": "frozen 1M ridge-sum candidate-feature table supplied by --cache",
        "cache_sha256": digest.hexdigest(),
        "comparison": "count frozen rows with R <= candidate R within exact product bucket",
        "rows": out,
    }
    (ROOT / "cdf-placement.json").write_text(json.dumps(report, indent=2) + "\n")
    with (ROOT / "cdf-placement.tsv").open("w") as f:
        f.write("candidate_id\tbucket\tR\tfrozen_rows\tcount_le\tempirical_cdf_le\n")
        for x in out:
            f.write(f"{x['candidate_id']}\t{x['bucket']}\t{x['edge_formula_r']:.17g}\t{x['frozen_bucket_rows']}\t{x['frozen_count_le']}\t{x['frozen_empirical_cdf_le']:.9g}\n")


if __name__ == "__main__":
    main()
