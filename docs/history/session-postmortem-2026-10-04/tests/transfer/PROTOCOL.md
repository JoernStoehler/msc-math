# Preregistered transfer comparison

Frozen 2026-10-04 00:11:35 UTC, before participant sampling. Scope approved by the parent: 12 fresh native participants plus two blinded scorers, included native quota only, no API/paid calls or live project actions. Each participant/scorer may read exactly its assigned self-contained input file once and use no further tools, reads or delegation. Finish within 25 minutes after this freeze (00:36:35 UTC).

## Question and limits

Does the retained October knowledge improve a fresh agent's decisions on nearby, materially different project cases beyond current project guidance and the same case facts? The previous consumer review tested retrieval of conclusions already present in the text. This comparison tests action selection, rationale, uncertainty and stopping boundaries on new facts.

This is a six-case paper comparison, authored with knowledge of the report. It is not an independent representative benchmark, a replay of the historical incidents, sustained coordination, or a comparison of models. Current native participants inherit the available model/configuration with no override; historical roots requested Sol. Do not infer historical cause, model superiority, significance, general reliability, or live improvement.

## Frozen inputs and allocation

`transfer-test/build_inputs.py` generated the exact case facts, rubric, packets and assignment order. `transfer-test/manifest.json` records timestamps, seed, condition mapping, words, bytes and SHA-256 hashes. The baseline receives 916 words of current `AGENTS.md`, identical task instructions and its case. Treatment receives the same input plus the entire frozen 2,133-word knowledge note (2,137 additional words including wrapper). Historical facts in that note are explicitly not facts about the hypothetical case.

Cases are:

- A: a complete four-lemma audit beside an unfinished, separately owned chapter obligation; necessary closure is the correct negative case.
- B: a route-comparison assignment with a real full-run approval gate, a remaining aggregate toy allowance and unresolved target coverage.
- C: two mathematical reviewers sharing an incomplete extracted source, alongside an independently resolvable sign dispute.
- D: a claimed backend failure contradicted by two different error paths and preserved certificate receipts.
- E: four minutes of human review with three feasible candidates differing in thesis value, evidence and later human effort.
- F: assembly of three existing excerpts with a root unavailable for the first ten minutes, a serial merge and required final reading.

Case labels and mechanisms are not supplied as headings to participants. Facts include consequential tradeoffs and permit alternatives. Cases B–F do not prescribe success simply by invoking more review, more workers, persistence or approval. Case A checks harmful overapplication. Each answer is limited to 300 words and must give actions/outputs, factual rationale, uncertainty and a stopping/waiting condition. Each pair receives byte-identical case facts and guidance. There is one independent participant per condition/case; no retries chosen for answer quality.

## Criteria locked before sampling

`transfer-test/criteria.json` contains separate case-specific acceptable actions, boundaries, uncertainties, critical errors and permissible alternatives. `transfer-test/scoring-instructions.txt` fixes the shared dimensions: decision/action 0–3, evidence 0–2, authority/completion boundary 0–2, uncertainty 0–2, and usable concise output 0–1. Half-points are permitted. Report raw dimension scores, critical flags, spurious gates, concrete reasons and confidence. An 8/10 answer without critical errors is a descriptive adequate-answer category, not a validated cutoff.

No criteria will be changed after sampling. Apparent defects in the rubric will be reported, not silently repaired or used to cherry-pick cases. Failed input delivery or extra file access is a protocol deviation, distinct from decision quality. No answer will be dropped for disagreement with the report or for an inconvenient result.

## Scoring and analysis

Two fresh scorers get all twelve complete responses, their case facts and the frozen criteria, in separate seeded random orders. They do not receive treatment labels, the note, the hypothesis or participant traces. Participants are asked to cite case facts, avoiding overt condition labels. Blinding is incomplete if wording reveals background exposure; report that limitation. Do not let scores depend on discovering the expected condition.

Store full matched inputs and exact outputs, scorer inputs/outputs and manifests. Lock both scorer returns before examining condition-level scores. Report each pair's scores, absolute adequacy and disagreements; do not make significance claims from six pairs. A baseline ceiling is a valid finding: if both conditions already make the same useful choices, the test demonstrates no incremental decision benefit despite successful answers. Any treatment-specific critical error or new gate is important counterevidence even when its prose is polished.

The main purpose is to find omissions or harmful overgeneralizations the report should repair now. A difference must be tied to a concrete action or rationale, not note vocabulary or a longer answer. Current project guidance already contains several relevant maxims; treatment must earn any benefit beyond those. The note includes historical facts about Jörn and agent behavior, so this is a knowledge-package comparison, not an isolated pure-reasoning intervention. Case facts override those histories; any imported historical preference is flagged rather than credited.

## Costs and stopping rule

Record words/bytes read, response length, dispatch/return times, native token telemetry where available, failed calls, extra reads, scorer burden and any exclusions. Unknown telemetry stays unknown. Treatment adds 12,822 words across six participants before considering native tokenization. The two scorer reads are evaluation overhead, not a deployment cost of the knowledge note. Time includes the permitted file read. Latency is noisy and this sample cannot isolate a causal latency effect.

Stop after all responses and two scorer returns or the 00:36:35 deadline, whichever comes first. At the deadline analyze returned data and mark missing outcomes rather than extending the experiment or claiming completion. No production guidance or configuration is changed by this test. Parent owns any report integration or correction.
