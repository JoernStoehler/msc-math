# Bounded direct translation-cover rehearsal · 19 October 2026

This retained small evidence packet tests a concrete unrestricted candidate-generation primitive, not a global optimizer. It uses the ten exact-fixture-derived oriented triangles, anchors one translation, and minimizes convex-hull area over the other eighteen translation coordinates by alternating common-direction scalar slides. See the [algorithm/source account](../translation-cover-algorithms-20261019.md) and [exact run handoff](direct-cover-handoff.md).

## Result and limits

Twelve deterministic starts gave no evaluated area below the known HKO control. The initial 144 parallel updates all hit their eighteen-cut cap. Refinements of three existing starts closed numerical LP gaps inside the artificial shift box, in 23–80 additional evaluations. Every start stayed at or below 297 cumulative area evaluations. These results show a cheap useful inner primitive and expose an inadequate initial cut budget; they establish no local/full-space convergence, exact LP lower bound or global minimum.

Both owned processes exited successfully, using about 10.1 seconds observed process CPU in total. This is CPU time, not model/account quota. Numerical versions, fixture/script hashes, configurations, all initial/best placements, termination reasons and refinement query traces are retained. Primary query traces are not individually retained; source/configuration identifies their producer. No claim of complete floating-point reliability is made.

## Review and reproduction scope

The root coordinator read the producer, checked the shoelace-gradient sign, template-label accumulation, fixed anchor, supporting-plane LP sign, incumbent update and gap scope against the primary parallel-slide lemma. The theory makes each exact supporting plane globally valid on the scalar-slide space, including contact ties; floating-point cuts and LP solves remain uncertified. Recorded maximum subsequent-query cut violation was zero, a finite check only.

The original scripts and matched run bytes are preserved. They target the assigned `/workspaces/msc-math` fixture path and are exploratory Linux/Python producers, not a portable production API. Use a fresh output directory: the primary script refuses an existing result path but also writes its named input companion. Full reconstruction requires the recorded fixture/code identities and dependencies. Any rerun is a new run; no expensive continuation is selected.

```sh
uv run topics/products/pentagon-arbitrary-partner/translation-cover-probe-20261019/direct-cover-probe.py \
  --out /tmp/msc-math/direct-cover/NEW-RUN/results.json
```

The refinement source records its original input paths/settings and is retained as provenance rather than advertised as a clean command-line interface. A future selected implementation should expose portable paths and full query logging before claiming a reusable certified optimizer.
