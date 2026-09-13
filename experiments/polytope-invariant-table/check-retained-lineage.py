#!/usr/bin/env python3
"""Cheap guard for the tracked part of the retained random/product lineage."""

from __future__ import annotations

from collections import Counter
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PROVENANCE = Path(__file__).with_name("polytope-provenance-table.jsonl")
REGISTRY = ROOT / "artifacts" / "registry.json"

EXPECTED_DATASETS = {"random_sample": 4096, "random_product_sample": 10240}
EXPECTED_BACKENDS = {"ehz_capacity": 4096, "ehz_capacity_billiard": 10240}
EXPECTED_PROVENANCE_SHA256 = (
    "6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2"
)
EXPECTED_SNAPSHOTS = {
    "polytope-datasets": "f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96",
    "polytope-invariant-table": "c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a",
}
UNRECORDED_FIELDS = {
    "capacity",
    "volume",
    "sys",
    "capacity_method",
    "capacity_certificate",
    "producer_commit",
    "producer_revision",
    "run_manifest",
}


def main() -> None:
    digest = hashlib.sha256(PROVENANCE.read_bytes()).hexdigest()
    assert digest == EXPECTED_PROVENANCE_SHA256, (
        "provenance bytes changed; re-audit retained-lineage.md before updating the guard"
    )

    datasets: Counter[str] = Counter()
    backends: Counter[str] = Counter()
    poly_ids: set[str] = set()
    seen_unrecorded: set[str] = set()
    with PROVENANCE.open() as rows:
        for line in rows:
            row = json.loads(line)
            datasets[row["dataset"]] += 1
            backends[row["backend"]] += 1
            poly_ids.add(row["poly_id"])
            seen_unrecorded.update(UNRECORDED_FIELDS.intersection(row))

    assert dict(datasets) == EXPECTED_DATASETS
    assert dict(backends) == EXPECTED_BACKENDS
    assert len(poly_ids) == sum(EXPECTED_DATASETS.values())
    assert not seen_unrecorded, (
        "new numerical/run provenance exists; strengthen retained-lineage.md: "
        f"{sorted(seen_unrecorded)}"
    )

    registry = json.loads(REGISTRY.read_text())["artifacts"]
    actual_snapshots = {
        name: registry[name]["snapshot"] for name in EXPECTED_SNAPSHOTS
    }
    assert actual_snapshots == EXPECTED_SNAPSHOTS, (
        "registered retained snapshot changed; re-audit lineage before updating the guard"
    )
    print("retained lineage guard: ok (14,336 tracked provenance rows)")


if __name__ == "__main__":
    main()
