# Existing upper-tail data: seed coverage has a word-specific gap

The separate patterns branch owns [the scientific report](../../../../../patterns-2026-10-17/patterns/upper-tail/seed-coverage/REPORT.md). On the existing above-one control `lpseed_near_hko_s0_e100` (ratio about 1.025703), a strict coordinate maximum for the negative angular word, A{0,2,4}, B{0,1,3}, is absent from every one of that word's ten seeded trajectories. The same supports are reached under the positive word, with strictly smaller objective. The missing negative-word pair is itself nonglobal; the all-seed selector still recovers all twelve retained capacities.

This exact counterexample rules out explaining the successes by complete coverage of every strict basin for each word. It identifies a specific possible counterexample construction: continue the missed basin and the seed/LP chambers until either the basin becomes globally best or a seed enters it. No such continuation or crossing is claimed. The existing nonempty open region of exact completed-response recovery remains valid; an entire-HKO-neighborhood guarantee is unresolved.

A label-free sufficient certificate gives a positive statement: for each word, if first responses to row seeds cover all A closure vertices, completing the B response yields the fixed-word optimum; the symmetric column/B condition suffices too. If either condition holds separately for both words, the sparse-product and two-shadow reductions certify true capacity. It applies to eight of the 24 existing words and both words on only two of the twelve bodies. These count certificate applicability, not prediction accuracy. Thus this simple certificate cannot explain ten known successes; failure of the certificate is not prediction failure.

The backward plan sought a structural implication or obstruction rather than more numerical examples. Recounting known nonglobal endpoints adds no insight; a whole-HKO-neighborhood proof has unresolved chambers; exact response coverage is falsifiable using only retained inputs. Word-specific reconstruction is necessary because the union of support pairs hides different objective values. The scientific report compares these alternatives and the ways a score based only on numerical agreement would mislead selection.

Root independently reconstructs Gaussian closure vertices, 24 exact LP certificates, 600 direct four-dimensional pair-word objectives, 240 complete deterministic seed trajectories, all fixed-word strict coordinate maxima, and the full-factor coverage counts. It also reconstructs all twelve inherited global objectives and ratios using exact polygon areas, and checks that the positive-word copy of the missing pair has smaller value. [verification.json](verification.json) records PASS and exact rational margins; [owner-check.py](owner-check.py) imports only the earlier independent geometry checker, not the new scientific producer.

```sh
python3 -B thesis/review/2026-10-17/seed-coverage-integration/owner-check.py \
  --thesis "$PWD" --patterns /workspaces/msc-math-worktrees/patterns-2026-10-17 \
  --out /tmp/msc-math/seed-coverage/root-replay.json
```

The scientific checker independently uses the earlier patterns-side rational geometry/HK routines; a separate read-only reviewer checked the sufficient-condition proof and a single 25-entry negative-word matrix with the ten retained trajectories. Its verdict is retained in [scientific-review.md](scientific-review.md). These checks do not replace human proof acceptance. The findings use exposed local controls, with no new geometries, LP solves, capacities, fitted predictor, cohort expansion or global bound. The selected 105-page thesis and Kai PDF remain unchanged. Root owns local integration, separate branch commits and cleanup; the worker owns only the new patterns packet.

[review: /root/intake_review]
