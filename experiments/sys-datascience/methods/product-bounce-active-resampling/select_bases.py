#!/usr/bin/env python3
"""Select four target-blind matched 5x5 bases for the active-facet smoke."""

import argparse
import hashlib
import json
import math
from pathlib import Path
from statistics import fmean


EXPECTED = {
    "raw": "66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736",
    "classes": "187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4",
}

FEATURES = (
    "q_mean_log_support",
    "p_mean_log_support",
    "q_sd_log_support",
    "p_sd_log_support",
    "q_min_angle_gap",
    "p_min_angle_gap",
    "ridge_symp_area_normalized_entropy",
    "ridge_symp_area_max_share",
)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def read_jsonl(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.strip()]


def factor_controls(raw):
    def summarize(duals, offset):
        heights = []
        angles = []
        for a in duals:
            norm = math.hypot(a[offset], a[offset + 1])
            heights.append(1.0 / norm)
            angles.append(math.atan2(a[offset + 1], a[offset]) % math.tau)
        angles.sort()
        gaps = [
            (angles[(i + 1) % len(angles)] - angles[i]) % math.tau
            for i in range(len(angles))
        ]
        logs = [math.log(h) for h in heights]
        mean = fmean(logs)
        sd = math.sqrt(fmean((x - mean) ** 2 for x in logs))
        return mean, sd, min(gaps)

    qmean, qsd, qgap = summarize(raw["dual_vertices"][:5], 0)
    pmean, psd, pgap = summarize(raw["dual_vertices"][5:], 2)
    return {
        "q_mean_log_support": qmean,
        "p_mean_log_support": pmean,
        "q_sd_log_support": qsd,
        "p_sd_log_support": psd,
        "q_min_angle_gap": qgap,
        "p_min_angle_gap": pgap,
    }


def canonical_rotation(sigma):
    return min(tuple(sigma[i:] + sigma[:i]) for i in range(len(sigma)))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw", required=True)
    ap.add_argument("--classes", required=True)
    ap.add_argument("--ridge-features", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    paths = {
        "raw": args.raw,
        "classes": args.classes,
        "ridge_features": args.ridge_features,
    }
    identities = {key: sha256(path) for key, path in paths.items()}
    for key, expected in EXPECTED.items():
        if identities[key] != expected:
            raise ValueError(f"{key} SHA-256 mismatch: {identities[key]}")

    raws_list = read_jsonl(args.raw)
    classes_list = read_jsonl(args.classes)
    ridge_features_list = read_jsonl(args.ridge_features)
    raws = {r["name"]: r for r in raws_list}
    classes = {r["name"]: r for r in classes_list}
    ridge_features = {r["name"]: r for r in ridge_features_list}
    if len(raws) != len(raws_list) or len(classes) != len(classes_list):
        raise ValueError("raw/class names must be unique")
    if set(raws) != set(classes):
        raise ValueError("raw/class name sets differ")
    expected_feature_names = {
        name for name, raw in raws.items() if raw["k"] == 5 and raw["m"] == 5
    }
    if set(ridge_features) != expected_feature_names:
        raise ValueError("ridge-feature names differ from retained 5x5 names")

    candidates = []
    for name in sorted(raws):
        raw = raws[name]
        cls = classes[name]
        if raw["k"] != 5 or raw["m"] != 5 or raw["bounces"] not in (2, 3):
            continue
        winner = cls["class_minima"][str(raw["bounces"])]
        if winner is None or cls["class_minima"]["3"] is None:
            continue
        supports = {tuple(sorted(set(sigma))) for sigma in winner["minimizer_sigmas"]}
        if len(supports) != 1 or len(next(iter(supports))) != 6:
            continue
        support = next(iter(supports))
        if sum(i < 5 for i in support) != 3:
            continue
        canonical_sigmas = sorted(canonical_rotation(s) for s in winner["minimizer_sigmas"])
        controls = factor_controls(raw)
        controls.update(
            {
                "ridge_symp_area_normalized_entropy": ridge_features[name][
                    "ridge_symp_area_normalized_entropy"
                ],
                "ridge_symp_area_max_share": ridge_features[name][
                    "ridge_symp_area_max_share"
                ],
            }
        )
        if not all(math.isfinite(controls[f]) for f in FEATURES):
            continue
        candidates.append(
            {
                "name": name,
                "producer_bounces": raw["bounces"],
                "features": controls,
                "sigma": list(canonical_sigmas[0]),
                "active_support": list(support),
                "winner_action_exact": winner["action_exact"],
                "winner_action": winner["action"],
                "winner_minimizer_count": winner["minimizer_count"],
            }
        )

    by_label = {b: [r for r in candidates if r["producer_bounces"] == b] for b in (2, 3)}
    if min(map(len, by_label.values())) < 2:
        raise ValueError("fewer than two eligible bases in a label")

    means = {f: fmean(r["features"][f] for r in candidates) for f in FEATURES}
    sds = {
        f: math.sqrt(fmean((r["features"][f] - means[f]) ** 2 for r in candidates))
        for f in FEATURES
    }

    def distance(a, b):
        return math.sqrt(
            sum(
                ((a["features"][f] - b["features"][f]) / sds[f]) ** 2
                for f in FEATURES
            )
        )

    edges = sorted(
        (distance(a, b), a["name"], b["name"], a, b)
        for a in by_label[2]
        for b in by_label[3]
    )
    selected = []
    used = set()
    for dist, _, _, two, three in edges:
        if two["name"] in used or three["name"] in used:
            continue
        selected.append((dist, two, three))
        used.update((two["name"], three["name"]))
        if len(selected) == 2:
            break

    bases = []
    pairs = []
    for pair_index, (dist, two, three) in enumerate(selected):
        pair_id = f"pair-{pair_index}"
        pairs.append(
            {
                "pair_id": pair_id,
                "two_bounce_name": two["name"],
                "three_bounce_name": three["name"],
                "standardized_euclidean_distance": dist,
            }
        )
        for base in (two, three):
            base = dict(base)
            base["pair_id"] = pair_id
            base["match_distance"] = dist
            bases.append(base)

    result = {
        "schema": "product-bounce-active-resampling/base-selection/v1",
        "inputs": {key: {"path": str(Path(path)), "sha256": identities[key]} for key, path in paths.items()},
        "eligibility": {
            "bucket": "5x5",
            "labels": [2, 3],
            "a3_required": True,
            "winner_support": "one unique unordered six-facet 3q+3p support",
            "eligible_counts": {str(b): len(by_label[b]) for b in (2, 3)},
        },
        "matching": {
            "target_blind": True,
            "target_fields_excluded_from_features_and_ordering": [
                "sys",
                "volume",
                "capacity",
            ],
            "features": list(FEATURES),
            "standardization_population": "all eligible 5x5 rows pooled across labels",
            "algorithm": "two smallest deterministic disjoint cross-label standardized-Euclidean edges; ties by source names",
        },
        "pairs": pairs,
        "bases": bases,
    }
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"out": str(out), "bases": [b["name"] for b in bases], "pairs": pairs}, indent=2))


if __name__ == "__main__":
    main()
