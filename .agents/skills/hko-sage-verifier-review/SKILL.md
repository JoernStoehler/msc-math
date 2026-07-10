---
name: hko-sage-verifier-review
description: Use when reviewing, editing, or delegating review of the HKO Sage verifier and its thesis-facing explanation, especially `experiments/hko-local-maximum/theorem/verify.sage.py`, `thesis/07-hko-local-maximum-sage-verifier.tex`, and the exact-certificate text they support. Also use when prompt-engineering a fresh subagent review for that coupled code/thesis surface.
---

<!-- Live-test skill: not yet Jörn-approved as settled long-term harness policy.
Keep, revise, or remove after observing it on future HKO verifier/thesis reviews. -->

# HKO Sage Verifier Review

Use this skill to review the executable verifier and thesis explanation as a
coupled audit surface. The live question is not only "is the code correct?" but
"can a mathematically strong reader with little/no coding background cheaply see
what Sage checks, what the thesis claims, and why the exact certificate is
accepted?"

Fresh subagent review is useful evidence, not authority. Treat findings as
signals to check. A summary like "delete X" is too lossy; require file/line
evidence, why it matters, and a concrete fix.

## Review Surface

For a narrow verifier/thesis-explanation review, required artifacts are:
- `experiments/hko-local-maximum/theorem/verify.sage.py`
- `thesis/07-hko-local-maximum-sage-verifier.tex`

For a broader HKO layer audit about thesis/documentation layers, navigation,
claim-control, proof status, or trust boundaries, also require:
- `thesis/07-hko-local-maximum.tex`
- `thesis/07-hko-local-maximum-chart-reduction.tex`
- `thesis/07-hko-local-maximum-exact-certificate.tex`
- `formal/hko-feasible-section-upper-branches.tex`
- `experiments/hko-local-maximum/README.md`
- `experiments/hko-local-maximum/theorem/README.md`
- `thesis/hko-local-maximum-content.md`
- `thesis/MAP.md`
- `thesis/central-claim-control.md`

Usually needed for either review mode: inspect these unless you have a concrete
reason they are not needed for the review question; if skipped, say why.
- `experiments/hko-local-maximum/theorem/verify.sage.py.explained.md`
- `experiments/hko-local-maximum/theorem/verification-summary.json`

Optional supporting artifacts: use when a concrete uncertainty requires them.
- `experiments/hko-local-maximum/theorem/witness.json`
- directly referenced source files, generated outputs, or build logs

These categories set a minimum review surface, not a maximum.

## Review Standard

Review code and thesis text bidirectionally:
- Code readability problems create thesis-explanation debt.
- Thesis prose or snippets can reveal that code names, comments, assertions, or
  formula displays are not carrying enough mathematical meaning.
- Prefer small local fixes that name proof-facing invariants over broad
  refactors.

Use this practical cost model: for each code line or snippet, ask how much prose
the thesis would need to explain it. If a small code edit, comment, renaming,
assertion, formula display, or snippet change would significantly reduce that
prose burden, flag it.

Check especially:
- every proof-facing computation has nearby mathematical meaning, for example
  the closure matrix, block solve for beta-polynomials, differentiated
  feasible-section equation, HKO quadratic form, and quotient-slice certificate;
- local assertions state mathematical invariants where they matter;
- names separate verification gates from data extraction or candidate
  construction;
- thesis snippets are the right amount of code and explain why routine
  boilerplate is omitted;
- the Sage/Rust/floating-point trust boundary is clear: Rust and f64 search may
  propose candidates, but Sage exact arithmetic accepts certificate claims;
- comments, docs, and thesis prose agree on formulas, symbols, dimensions,
  signs, file paths, and source-truth roles;
- hidden process-status prose does not leak into thesis-facing text or proof
  artifacts.

## Calibration

This skill exists because earlier review checked correctness/source backing too
much and mathematician-audit cost too late.

Issues this review should catch:
- A function named like a verifier should not return a certificate row as a side
  effect of verification. Prefer a pure verification gate followed by separate
  row extraction.
- A partition condition such as `sorted(minor_columns + fixed_indices) ==
  expected_indices` may be correct but expensive to explain. Prefer disjoint
  and cover checks, or a named helper, when that exposes the invariant.
- `beta_polynomials` is much easier to audit when text and code expose
  `C = [C_I C_J]`, `beta_I(u) = C_I^{-1}(e - C_J u)`, `beta_J(u) = u`.
- If a local formula depends on a dimension, square minor, partition, or sign
  convention, a local assertion or comment may be better than relying on a
  distant construction.
- If the thesis shows snippets, it should be clear why these snippets are shown,
  why more routine code is omitted, and why less code would not demonstrate the
  trust boundary.
- Raw reviewer summaries are weak signals. A finding needs file/line evidence
  and reasoning before it should drive edits.
- Do not ask one reviewer to evaluate too many aspects over too much text. If
  the artifact surface or aspect list is large, split the task or narrow the
  outcome.

## Output

Findings first, severity ordered. For each finding include:
- `file:line`
- the issue
- why it matters for mathematical review or thesis submission
- a concrete recommended fix

Then include:
- areas explicitly judged good enough and why
- tests or checks run, or intentionally not run
- questions for Jörn only if a specific expert decision is genuinely needed

If there are no findings, say so directly and still report the review surface
and residual risk. Do not produce a plan unless needed to explain a blocker.
