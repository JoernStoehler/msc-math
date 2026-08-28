# Flow-Graph Algorithm / CH2021 — Section Companion

Status: source, claim-boundary, and review ledger for
`thesis/05-flow-graph-algorithm-ch2021.tex`. This file is not thesis prose or
mathematical evidence.

## Intended Thesis Role

The section presents a second finite algorithm for the four-dimensional EHZ
capacity. Its thesis-facing mathematical results are:

1. Under the explicit flow-graph regularity conditions, the idealized finite
   search with exact operations over real facet data returns `c_EHZ`.
2. For fixed facet count, those regularity conditions hold on an open dense
   subset of every nonempty Euclidean-open chamber of ordered normalized
   irredundant facet presentations.
3. If the facet rows are rational, the idealized construction closes over the
   rationals and gives a finite exact rational-arithmetic algorithm.

Jörn confirmed on 2026-07-11 that the correctness and genericity results should
be retained; the genericity result is what makes the conditional correctness
theorem broadly useful.

The correctness proof uses the simple-minimizer theorem from Section 3. It does
not use the CH2021 Type 1/2/3 capacity theorem as a black box.

## Integrated Audit, 2026-07-14

The chapter was audited from Main commit `3dcc1efd` against the revised and
published CH2021 paper, the active simple-minimizer theorem,
`formal/flow-graph-real-algorithm.tex`, the Rust module contract/source/tests,
the proof-risk verification packet, and both figure producers and thesis
copies. No conflict was found in the conditional idealized theorem or its
normalization. The chapter now distinguishes the two-dimensional ambient
facet-pair chart from a section polygon that may be empty or lower-dimensional,
and presents the idealized exhaustive algorithm explicitly.

The projection asset remains experiment-owned but was removed from the active
chapter after cold-reader review: at whole-page scale it did not make the word
or affine-tube mechanism legible enough to repay its attention cost. The
tube-sequence figure remains active, with tube-focused crops, full-section
insets, and identical start/return limits so the fixed-point return is visible.

The current reader-facing boundary is:

- CH2021's open-two-face flow graph and smoothing correspondence motivate the
  affine-passage model but do not prove the project algorithm;
- the project theorem uses closed facet-pair sections, simple distinct-facet
  words, and the active simple-minimizer theorem under its displayed regularity
  hypotheses;
- the exact rational Rust wrapper has an additional caller/input and typed
  rejection boundary; an inconsistent singular fixed equation, or one whose
  fixed set misses the searched tube, is an exact no-orbit outcome, while an
  intersecting unsupported singular fixed set is rejected; it is implementation
  correspondence evidence, not the proof;
- the earlier binary64 prototype lacked a sound true/false/indeterminate
  predicate contract and was retired when project time ended; it supplies no
  current FG capacity evidence;
- selected exact F5/F6/F7 comparisons with certified QP and the targeted
  resolver tests are implementation evidence for selected rational inputs,
  not proof of the idealized algorithm or the CH2021 capacity scope.

The published CH2021 numbering is Proposition 2.14, Theorems 1.11 and 1.12,
and Corollary 1.15. The bibliography now records the Journal of Computational
Dynamics publication and DOI. Reopen this chapter if Jörn/Kai changes the
conditional theorem role, if the Rust caller contract broadens, or if a
singular positive-action branch is admitted rather than rejected.

## Final Domain Repair, 2026-08-28

A final audit found that the active correctness theorem quantified over
arbitrary real facet rows while calling the procedure rational arithmetic.
The chapter now states the proved idealized real-arithmetic theorem separately,
derives exact rational arithmetic only when every facet row lies in
`\mathbb Q^4`, and makes clear that chamber genericity supplies the real
theorem rather than executable coverage of arbitrary real presentations. The
Rust boundary is unchanged: it targets the rational-input corollary under its
caller contract, with tested rather than formally verified correspondence.

## Proof Sources

- `formal/flow-graph-real-algorithm.tex`: transition signs, affine primitive
  tubes, gluing, fixed-point semantics, strict-time boundary, short-word
  exclusion, long-word determinant nonvanishing, chamber genericity, and the
  idealized exact-search correctness theorem.
- `formal/flow-graph-ch2021-comparison.tex`: exact boundary between the
  project theorem and CH2021.
- `thesis/05-flow-graph-ch2021-background-comparison.tex`: active
  reader-facing provenance and concise relation to the project algorithm.
- `thesis/03-generalized-reeb-orbits-simple-minimizers.tex`: the
  simple-minimizer theorem used for completeness.
- `papers/ch2021/`: source cache for the cited flow-graph terminology and
  comparison. Check published numbering before hardcoding any numbered
  reference.

## Claim Boundary

The theorem concerns an idealized exhaustive search with exact real operations
on a bounded full-dimensional four-polytope with an ordered normalized
irredundant facet presentation. Its regularity hypotheses are stated in the
thesis. The executable corollary additionally requires every facet row to be
rational; the open-dense real genericity proposition does not remove that
input-domain restriction.

The genericity proposition is chamber-relative genericity of presentations.
It is not the CH2021 conjecture that generic polytopes have no Type 2 orbit
below a prescribed action bound.

The source audit behind the compressed CH2021 comparison records three
qualifications for any later expansion: Theorem 1.12 uses a sequence
\(\varepsilon_i\to0\); Theorem 1.11 gives eventual equality of rotation and
Conley--Zehnder data rather than only convergence; and the rotation bound in
Corollary 1.15 applies to Type 1 orbits, while CH2021 does not define a
combinatorial rotation number for Type 2.

The Rust exact implementation uses rational inputs and caller-supplied
incidence and symplectic-sign matrices. Tests provide implementation
correspondence and falsification evidence; they are not the proof of the two
mathematical results. The earlier binary64 prototype was diagnostic only and
is retired.

## Explanatory Figure

Figure `fig:flow-graph-tube-sequence` uses the deterministic generated six-facet
case with master seed `20260605`, attempt `3`, and facet word `(1,2,4,5,3)`.
The exact resolver and retained regression tests identify this word as a
positive capacity word. The figure is regenerated from the exact rational tube
geometry; rationals are converted to finite floats only at the JSON/rendering
boundary. It is explanatory rather than proof evidence.

The producer artifacts and reproduction commands are in
`experiments/dev-flow-graph/visualize-tube/` and
`experiments/dev-flow-graph/README.md`. The active PDF is copied deliberately
to `thesis/figures/flow-graph/` so the thesis build is self-contained. The
experiment also retains the projection asset for future redesign or diagnostic
use, but the active thesis does not include it.

- The sequence panel shows only the visited facet-pair charts. Its plotted
  vertices use Euclidean-orthonormal coordinates in the corresponding affine
  two-plane, constructed by Gram--Schmidt from the producer's raw affine
  coordinate split. The JSON names these `vertices_plot_f64`; the exact
  vertices and inequalities are retained separately as `*_construction_*`
  fields in the raw construction chart. Affine origins and orientations are
  chosen independently for distinct sections. Intermediate panels are
  autoscaled separately, while start and return use the same frame and limits
  so the fixed-point coordinates are directly comparable.
- The panel explains the construction. It does not support correctness,
  genericity, or implementation-validation claims.

## Review Status

The proof chain is agent-developed. Independent mathematical review checked
the sign conventions, higher-codimension breakpoint boundary, tube semantics,
simple-minimizer bridge, short-word exclusion, contraction identity, explicit
determinant witnesses, and chamber-density argument. The determinant values
were recomputed independently.

Jörn's 2026-07-11 direction settles retention of both mathematical results. It
does not replace final line-by-line mathematical or advisor review of the
printed proof.

A focused human review should check:

1. the regularity definition and the completeness step using the
   simple-minimizer theorem;
2. the repeated-section contraction used to cover every word length;
3. the restriction of the nonzero rational determinant conditions to each
   presentation chamber;
4. whether the section gives enough geometric motivation relative to the rest
   of the thesis.

## Deliberately Excluded From This Section

- fixture-by-fixture verification output;
- experiment provenance and cutoff-counter details;
- the retired binary64 prototype and its deleted mixed API;
- CH2021 rotation pruning and Type 2/Type 3 implementation;
- claims of correctness for arbitrary raw halfspace input.
