# Appendix A Rewrite Notes

## Origin of this task

During the cleanup-loose-ends session (2026-02-24), Jörn reviewed Section A.1 in the PDF and gave four specific notes (all fixed, commit `14968e8`). While discussing A.1's "Adjacency (search-space reduction)" paragraph, Jörn raised a deeper question:

> "Is there contradictory content in the thesis? I recall a discussion with some agent about how we should make the combinatorial structure of the polytope exactly-known in advance, i.e. by saying that the 'exact' polytope realizes the combinatorial structure, and that the coordinates (n,h), (v) are all numerical approximations of the exact values. This requires a theorem that states that for any combinatorial structure with almost-exact floating point coordinates there are also exact coordinates."

An exploration agent searched all thesis and code files for this claim. **Finding: no such theorem or assumption exists in the current thesis or code.** The combinatorial structure is inferred numerically from floating-point coordinates (tolerance-based incidence checks), not assumed known in advance. There is no contradiction because the claim was never made — but Jörn wants it to be made, which requires new mathematical content.

This led to a broader discussion about restructuring the entire appendix. Jörn sketched a new outline and asked for my thoughts. The discussion was interrupted (Jörn had to context-switch to other sessions), and we agreed to write this note.

## Why the current structure is inadequate

The current appendix is structured bottom-up:
- A.1 starts with the most technical detail (near-singular KKT systems) before the reader knows the general framework
- A.2 mixes the general three-valued logic framework with specific predicate implementations
- A.3 (error tracking) makes sense only after understanding both A.1 and A.2
- A.4-A.5 are empirical experiments, not numerical theory — they belong in the experiments chapter

Jörn wants a top-down structure: general approach first, then concrete realizations.

## Current Structure (what exists)

File: `thesis/appendix-numerical.tex` with three `\input`'d files + two experiment `\input`s.

| Section | File | Content | Status |
|---------|------|---------|--------|
| A.1 Near-Singular KKT Systems | `appendix-numerical-dismissal.tex` | Walk-to-boundary dismissal, SVD-based detection, error bound | Jörn-approved (no `% Jörn:` markers — B6 issue) |
| A.2 Three-Valued Predicate Relaxation | `appendix-numerical-three-valued.tex` | Soundness requirement, adjacency/positivity/dismissal predicates | Jörn-approved (no `% Jörn:` markers — B6 issue) |
| A.3 Error Tracking and Final Answer | `appendix-numerical-error-tracking.tex` | Certified vs uncertain candidates, final capacity report | Jörn-approved (no `% Jörn:` markers — B6 issue) |
| A.4 Unknown Predicates | `experiments/unknown-predicates/unknown-predicates.tex` | Empirical validation of three-valued predicates | Experiment — should move to experiments chapter |
| A.5 Ablation | `experiments/ablation/ablation.tex` | Pruning effectiveness analysis | Experiment — should move to experiments chapter |

**Note on "Jörn-approved (no markers)"**: These files say `% Status: Jörn-approved.` in their header comments, but lack the formal `% Jörn:` inline markers with commit hashes that the convention requires. This is tracked as B6 in the session's inventory. The content was approved by Jörn reading the PDF, but the markers were never added. A rewrite would invalidate the approval anyway.

## Proposed Structure (Jörn's design, 2026-02-24)

Jörn sketched this in chat. His exact words are quoted where available.

### A.1 General Approach

Jörn's description:
> "general approach (A) including three-predicate logic to let downstream applications decide (e.g. later demonstrated by the flow of information from svd low-rank => accumulating the Q maximum => end consumer of minimum action which resolves some INDETERMINATE signals), errors when numerical edge cases occur which we cannot handle yet, assertions and test suites on claims about proven/unproven bounds / edge case frequency in the code, more expensive second-pass exact rationals / >64 bit floating point numerics on only those statements the downstream consumer needs."

This maps to:
- **Soundness requirement**: TRUE/FALSE are reliable, UNKNOWN/INDETERMINATE = inconclusive. Currently in `appendix-numerical-three-valued.tex:9-27`.
- **Information flow architecture**: upstream producers → downstream consumers → late resolution. The concrete example Jörn gave: SVD detects low rank → produces INDETERMINATE → Q maximum accumulation tracks certified vs uncertain → the minimum-action consumer (final answer) resolves some INDETERMINATE signals by checking whether uncertain candidates are above or below the certified best.
- **Error handling for unresolvable edge cases**: What happens when f64 precision is insufficient. Currently the code panics: `assert!` at `hk2017/mod.rs:145-151` fires if an uncertain orbit has lower action than the best certified orbit. The error message: "Numerical gap: certified capacity > uncertain capacity. An UNKNOWN orbit achieves lower action than the best certified orbit. Cannot resolve at f64 precision."
- **Assertions and test suites**: Empirical coverage of the gap between "we proved this bound holds" and "we proved it's tight enough in practice." The test suites check edge case frequency, not correctness proofs.
- **Second-pass escape hatch**: Re-evaluate with exact rationals or >64-bit floats, but only for the specific statements the downstream consumer needs. Currently NOT IMPLEMENTED — the code panics instead. The thesis should describe what the second pass *would* do, honestly marked as unimplemented.

### A.2 Polytope Combinatorics

Jörn's description:
> "Polytope combinatorics (skeleton combinatorics, floating point coordinates, floating point vertices); theorem: combinatorics + fp coords + TRUE&INDETERMINATE consistency => exact coords exist with exact consistency"

And when I asked about ω₀:
> "omega>=0 would be handled as part of skeleton combinatorics i.e. treat as part of the 'combinatorics' data where we just want approximate realization via fp64 and then automatically have the implication that the exact coords fulfill it."

And about input validation:
> "again see combinatorics of polytopes?"

**New theorem needed**: For any combinatorial type C (including skeleton structure AND directed adjacency via ω₀ signs), if fp64 coordinates (n,h) produce three-valued skeleton verdicts that are TRUE-and-INDETERMINATE-consistent with C, then exact coordinates (n',h') exist that realize C exactly, with ||(n,h) - (n',h')|| small.

**Open questions for Jörn** (raised by me in chat, not yet answered):

1. **Genericity**: Is this true for all combinatorial types, or only for generic/simple polytopes? For simple polytopes (each vertex on exactly 4 facets in R^4), transversality gives stability — the face lattice is stable under small perturbations. For non-simple polytopes, a small perturbation can split a non-simple vertex, changing the combinatorial type. Jörn's response was interrupted — he was uncertain: "I am not sure that is true in all cases, or just for e.g. generic cases where 0-faces have 4 adjacent 3-facets etc."

2. **ω₀ = 0 boundary**: ω₀ = 0 exactly corresponds to Lagrangian 2-faces, a non-generic condition. If ω₀ is part of the combinatorial data, the theorem needs to handle the boundary case where floating-point ω₀ is near zero but the exact sign matters. Jörn's answer treats it as combinatorial data, but the mathematical details are unresolved.

3. **Precise meaning of consistency**: What does "TRUE-and-INDETERMINATE-consistent with C" mean precisely? My proposal: every TRUE verdict in the computed skeleton is correct for the exact polytope; every INDETERMINATE verdict may go either way. Not discussed further.

**What currently exists in the codebase:**
- Adjacency three-valued predicate: `appendix-numerical-three-valued.tex:76-109`
- Skeleton computation: `library/src/geom/skeleton.rs` (uses `EPS_FACET_INCIDENCE = 1e-8`)
- Directed adjacency (ω₀): `library/src/algorithms/hk2017/mod.rs:218-241` — uses hard `>= 0.0` cutoff, NO three-valued handling

**Known gap**: The ω₀ ≥ 0 test in the code is a hard boolean cutoff, not three-valued. Near-Lagrangian 2-faces could be misclassified. This must either get three-valued treatment or the theorem must explain why it's safe.

**Related prior work**: historical Jörn note in deleted `experiments/ablation/ideas-future.md:45-54` (dated 2026-02-22): "Replace three-valued predicates (TRUE/FALSE/UNKNOWN) in skeleton computation with deterministic rounding: if ω₀(n_i,n_j) ≈ 0, round to TRUE or FALSE and argue via small perturbation that the capacity changes by at most the perturbation. This separates exact combinatorial decisions (skeleton) from approximate numerical decisions (KKT solver). Requires careful analysis of when the perturbation direction matters." This is a related but different approach (rounding + perturbation argument vs. the consistency theorem Jörn is now proposing).

### A.3 SVD Step (given S, σ)

Jörn didn't elaborate much. When I asked about β > 0:
> "part of SVD step (?)"

This section covers solving the KKT system numerically for a specific (S, σ) pair. Content mostly comes from the current A.1:
- SVD decomposition of the KKT system matrix
- Near-singular detection and dismissal (walk-to-boundary argument, Proposition + Algorithm)
- Error bound for dismissed systems (Remark with the bound formula)
- The β > 0 positivity predicate (currently in A.2, moves here per Jörn)

*[Deleted: the dismissal-error experiment was removed (commit 72cf05c) — it validated the old dismiss logic (Lemmas B.5-B.9) which was replaced by the Q error bound framework (lem:q-error-bound). The q-error experiment validates the current error bound.]*

### A.4 Accumulated Q Maximum and Final A_min

Jörn's description:
> "Accumulated Q max, final A_min; includes post-search second-pass verification (iirc rn: a error is thrown bc we didn't decide much less implement a second pass!)"

Jörn is correct — the code throws an error (assert! panic) because the second pass is unimplemented. Content mostly comes from the current A.3:
- Certified vs uncertain candidate tracking
- Final capacity report with certainty qualifiers
- Post-search second-pass verification

**The second pass**: Currently at `hk2017/mod.rs:142-151`, the code asserts `uncertain_cap >= certified.0` and panics if violated. A proper second pass would re-solve only the uncertain candidates with higher precision. Options mentioned by Jörn: exact rationals, >64-bit floats. Not decided which approach to use. The thesis should honestly describe this as unimplemented and explain what it *would* do.

### Experiments (moved out of Appendix A)

Current A.4 (unknown-predicates) and A.5 (ablation) move to the experiments chapter. This was not explicitly discussed — it's my inference from Jörn's proposed structure only covering four theory sections.

## What Jörn asked me if I thought was missing

His exact words: "Anything else I am forgetting?"

My response identified three additional items:

1. **The ω₀ sign predicate** for directed adjacency — Jörn said to fold into combinatorics (A.2). Resolved.

2. **The β > 0 predicate** — Jörn said "part of SVD step (?)" with a question mark. Tentatively in A.3. Resolved.

3. **Input validation** — normals are unit vectors, heights are positive. Jörn said "again see combinatorics of polytopes?" — i.e., fold into A.2. Resolved.

I also raised but did not discuss with Jörn:

4. **Tolerance cascade**: The code uses multiple independent tolerances (EPS_FACET_INCIDENCE = 1e-8, SVD threshold τ, β > 0 threshold ε_β). Their relationship is undocumented. No formal analysis shows they're mutually consistent. The rewrite should at minimum acknowledge this gap.

5. **Pruning predicates besides adjacency**: The simple-orbit test (checking if a permutation visits each facet at most once) is exact — it's a combinatorial check, no floating-point involved. Probably doesn't need treatment in the appendix.

## What the agent searched and found (exploration report)

The exploration agent (`a5fe055ee1e528c3e`) searched all thesis .tex files, all Rust source, and all .md files. Full findings:

1. **No "exact combinatorial structure" assumption exists anywhere.** The combinatorial structure is always inferred numerically.
2. **No perturbation theorem exists.** The closest thing is the ideas-future.md note.
3. **Adjacency is computed numerically everywhere** — skeleton.rs, hk2017/mod.rs.
4. **"Generic" polytopes are discussed only in tube-algorithm.tex** (Type 1 vs Type 2 orbits, CH2021 genericity conjecture). Not related to the combinatorics theorem Jörn wants.
5. **The directed adjacency ω₀ ≥ 0 uses a hard cutoff** — no three-valued handling.
6. **The β > 0 predicate IS three-valued** in the thesis (appendix-numerical-three-valued.tex:111-151) with proper TRUE/FALSE/UNKNOWN handling.
7. **The near-singular dismissal is binary** (not three-valued) — dismiss or use β₀. This is by design and correctly described in appendix-numerical-three-valued.tex:153-169.

## Files involved

| File | Current role | Proposed action |
|------|-------------|-----------------|
| `thesis/appendix-numerical.tex` | Top-level structure | Rewrite: new section headings, new intro paragraph |
| `thesis/appendix-numerical-dismissal.tex` | A.1 (near-singular KKT) | Becomes part of new A.3 (SVD step) |
| `thesis/appendix-numerical-three-valued.tex` | A.2 (three-valued predicates) | Split: general framework → new A.1, specific predicates → new A.2/A.3 |
| `thesis/appendix-numerical-error-tracking.tex` | A.3 (error tracking) | Becomes part of new A.4 (accumulated Q max) |
| `thesis/experiments.tex` | Experiments chapter | Add `\input` for unknown-predicates and ablation (moved from appendix) |
| NEW file | — | A.1 general approach framework |
| NEW file | — | A.2 polytope combinatorics (new math: the consistency theorem) |
| `library/src/geom/skeleton.rs` | Skeleton computation | May need updates if the combinatorics theorem changes assumptions |
| `library/src/algorithms/hk2017/mod.rs:237` | ω₀ hard cutoff | May need three-valued treatment |
| ~~`experiments/dismissal-error/`~~ | ~~Deleted~~ | ~~Replaced by q-error experiment~~ |
| Deleted `experiments/ablation/ideas-future.md:45-54` | Jörn's perturbation idea | Historical prior thinking, should inform A.2 |

## Scope and effort estimate

This is a significant rewrite with these components:
- **New mathematical content** (the combinatorics theorem) — Jörn must design this. Agent cannot.
- **Restructured exposition** (top-down instead of bottom-up) — agent can do after Jörn provides the theorem and section outlines.
- **Filling the second-pass gap** (currently unimplemented, code panics) — at minimum, the thesis must honestly describe what the second pass would do. Actually implementing it in Rust is optional.
- **Moving experiment sections** out of the appendix — mechanical, agent can do.
- **Existing approved content** will be invalidated by the restructure — Jörn will need to re-approve after the rewrite.

Estimated: 1-2 full agent sessions, with Jörn designing the combinatorics theorem upfront before the first session starts.

## Verification (after the rewrite)

- `cd thesis/ && latexmk && ./check-build.sh` passes
- No new undefined references
- All `\ref{}` and `\label{}` updated for the new section structure
- Cross-references from main chapters still resolve: grep for `\ref{app:near-singular}`, `\ref{app:three-valued}`, `\ref{app:error-tracking}` and update
- Grep `library/src/` for `[alg:]` or `[lem:]` references to appendix labels — update if labels changed
- The experiment sections render correctly in their new location in the experiments chapter
