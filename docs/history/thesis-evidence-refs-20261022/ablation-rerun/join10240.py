"""Recompute the 10,240 random-product / shared-cache join (product_closure_route/RESULTS.md)."""
import json, sys, collections
d = sys.argv[1]
key = lambda r: json.dumps(r["dual_vertices_rational"], separators=(",", ":"))
cache = collections.defaultdict(list)
for line in open(d + "/shared-cache.jsonl"):
    r = json.loads(line)
    if r.get("source", {}).get("family") == "lagrangian_product":
        cache[key(r)].append(r)
prods = [json.loads(l) for l in open(d + "/random-product.jsonl")]
buckets = collections.Counter((p["k"], p["m"]) for p in prods)
match_counts = collections.Counter()
winner_len = collections.Counter()
long_winners = 0
cap_eq = 0
matched_keys = set()
for p in prods:
    k = key(p)
    rows = cache.get(k, [])
    match_counts[len(rows)] += 1
    if len(rows) != 1:
        continue
    matched_keys.add(k)
    c = rows[0]
    best = min(c["sigmas"], key=lambda s: s["action"])
    winner_len[len(best["perm"])] += 1
    long_winners += len(best["perm"]) >= 7
    cap_eq += (p["capacity"] == best["action"] == c["capacity"])
other = [rs for k, rs in cache.items() if k not in matched_keys]
other_rows = [r for rs in other for r in rs]
other_len = collections.Counter(len(min(r["sigmas"], key=lambda s: s["action"])["perm"]) for r in other_rows)
print(json.dumps({
    "product_rows": len(prods),
    "buckets": {f"{a}x{b}": n for (a, b), n in sorted(buckets.items())},
    "cache_product_rows": sum(len(v) for v in cache.values()),
    "product_rows_by_cache_match_count": dict(match_counts),
    "winner_length_counts": dict(winner_len),
    "winners_length_ge_7": long_winners,
    "capacity_equals_winner_action": cap_eq,
    "other_cache_product_rows": len(other_rows),
    "other_cache_product_winner_lengths": dict(other_len),
}, indent=1))
