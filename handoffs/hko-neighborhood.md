# Task: Complete HKO neighborhood experiment writeup and analysis

## Context

The HKO2024 polytope (pentagon × pentagon at θ=18°) is the known Viterbo counterexample with sys ≈ 1.047. We're investigating whether it's a local maximum of sys in various ambient spaces. Phase A (sensitivity analysis, gradient ascent in h-space) and Phase B (facet splitting) have data generated but the experiment may need additional analysis and the writeup needs to tell a coherent story.

## Scope

1. **Review existing data and writeup** for completeness and accuracy:
   - Phase A: sensitivity analysis at HKO2024, near-optimal orbit tracking, normal gradient, gradient ascent
   - Phase B: facet splitting (2 representative facets × 100 directions + 50 mixed + 20 control)
   - Check that the writeup (`hko-neighborhood.tex`) accurately describes what the data shows

2. **Assess whether additional experiments are needed** to strengthen the "local maximum" claim:
   - Are the Phase B directions sufficient for the claimed conclusion?
   - Should more facets be split, or more directions tested?
   - Is the gradient ascent in Phase A convincing (Δsys ≈ 5e-9 suggests convergence at machine precision — is this real convergence or numerical noise)?

3. **Ensure the writeup tells a coherent story** connecting to the thesis narrative:
   - Local maximality in h-space (Phase A) vs in polytope space (Phase B) — different claims
   - How this relates to the broader "probing Viterbo's conjecture" goal

4. **Run review** before presenting to Jörn

## Out of scope

- Changing the KKT solver or derivative computation code
- Running new large-scale experiments (just assess whether they're needed and report)
- Other experiments (sys-optimization, gradient-descent) — those are separate concerns
- Changing the (n, h) parameterization

## Key files

Experiment code and data:
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko_neighborhood.rs` (2118 LOC)
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood-sensitivity.jsonl`
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood-ascent.jsonl`
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood-splitting.jsonl`

Writeup and figures:
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood.tex`
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood-gradient.png`
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood-orbits.png`
- `/workspaces/msc-math/crates/exp-hko-local-maximum/gradient-is-zero/hko-neighborhood-splitting.png`

Related experiments for context:
- `/workspaces/msc-math/crates/exp-sys-optimization/sensitivity-analysis/` — gradient analysis across random polytopes
- `/workspaces/msc-math/crates/exp-sys-optimization/large-scale-descent/` — gradient ascent on F=10 polytopes
- `/workspaces/msc-math/crates/exp-hko-local-maximum/perturbation-neighborhood/` — perturbation analysis of HKO pentagon

Memory with project context:
- `/home/vscode/.claude/projects/-workspaces-msc-math/memory/project_hko_neighborhood.md`

## Prior findings

- Phase A: all ∂sys/∂h_k < 0 (local max in h-space). Normal gradient |∇sys_n| ≈ 1.53 (nonzero — NOT a critical point in full (n,h) space). 44 near-optimal orbits at action gaps < 5e-14.
- Phase B: all 536 facet splits decrease sys (best Δsys = -4.43e-9, worst = -3.18e-4).
- The distinction between "local max in h-space" and "local max in polytope space" is important — these are different claims with different evidence.
- The experiment code contains a copy of the instrumented KKT solver (will be deduplicated separately).

## Success criteria

- Writeup accurately reflects the data (verified against JSONL files)
- Writeup clearly distinguishes h-space vs polytope-space claims
- Review subagents run and findings addressed
- Assessment of whether additional experiments are needed, with reasoning
- Ready for Jörn to review the PDF
