# Rank-four Pfaffian restart packet — 29 October 2026

Retained research, separate from the capacity report and selected thesis.
The component arguments below and the corrected assembled
[LaTeX note](rank-four-pfaffian.tex) have independent agent audits. The
[whole-text audit](audit/assembled-audit.md) found no mathematical gap,
conditional on the cited FMMZ projection theorem. The unrestricted conjecture remains open. No consequence
for EHZ capacity or Viterbo's inequality is claimed, and no novelty or priority
claim has been established.

For a real skew matrix of rank at most four, write
`Phi=sum_{i<j}|W_ij|` and `Psi=sum_{|I|=4}|Pf(W_I)|`. The target is
`4 Psi <= Phi^2`, equivalently `vol_4(Z)<=4 Phi(Z)^2` for a zonotope with
half-generators `[-v_i,v_i]` and `W_ij=omega(v_i,v_j)`.

## Retained results and evidence

| Result | Proof owner | Component review |
|---|---|---|
| Every matrix with at most six indices satisfies the inequality; the new planar strip lemma controls the first nontrivial extension beyond the inherited five-index case. | [Six-generator proof](six-vertex/theorem.md) | [Independent audit](audit/six-theorem-audit.md) |
| In any matrix size, a row with exactly one or two nonzero entries suffices. The degree-two proof uses the established three-dimensional zonoid projection theorem. | [Sparse-row proof](sparse-degree-two.md) | [Sparse-row and seven-generator audit](audit/seven-sparse-audit.md) |
| Every matrix with at most seven indices satisfies the inequality. A minimal-support maximizer has degrees at most three; parity supplies a sparse row. A zero row uses the six-index theorem. | [Seven-generator proof](six-vertex/theorem7.md) | [Independent audit](audit/seven-sparse-audit.md) |
| Eight-generator equality examples include `C+JC`, where `C=[-1,1]^4` and `J` is an orthogonal complex structure. Generic examples are not linearly products of planar bodies. | [Equality construction](nonproduct-equality.md) | [Independent audit of the self-dual family](audit/nonproduct-equality-audit.md) |
| A proposed all-direction projection-body strengthening is exactly false. Replacing the directional ratio by its maximum gives a universally equivalent formulation of the still-open conjecture. | [Mechanism and exact witness](all-size-mechanism/projection-radius.md) | [Independent audit](audit/all-size-audit.md) |

The final section of the equality note records a broader paired-inverse
construction derived after that component review. It is explicitly **not
covered by the retained independent audit** and is not used by the assembled
note. The [source pinpoints](sources.md) identify the external theorem and
bounded prior-art checks. The [Pfaffian-polynomial source note](priorart/pfaffian-polynomial.md)
records why existing squared-Pfaffian sector-stability results do not settle
the absolute-value conjecture.

## Review status and provenance

The work ran on 28–29 October 2026 in the research-restart checkout. Research
owner: `/root/rank_four_zonoids`; six/seven-index proof and writeup collaborator:
`/root/rank_four_zonoids/six_vertex_pfaffian`; independent mathematical reviewer:
`/root/rank_four_zonoids/six_theorem_audit`; source investigator:
`/root/rank_four_zonoids/zonoid_priorart`. The audit files retain their original
findings. The later whole-text audit covers the assembled LaTeX; the separate
all-size audit covers the projection-radius equivalence and exact obstruction.
These are independent agent reviews, not formal verification or human acceptance.

A subsequent manual read of the assembled source corrected three transcription
issues: a sparse-row estimate needed `<=` rather than `=`, a degree-one square
needed `(r-s+t)^2`, and old-row index ranges needed to exclude the new index.
The corrected scratch source rebuilt successfully to seven pages and subsequently
passed the fresh whole-text audit. Its build is checked separately from the
mathematics. No PDF or build intermediates are retained in this packet.

The all-size mechanism notes, integer witness and two fresh audits were copied
from `/tmp/msc-math/restart/rank-four-zonoids/`, preserving their relative paths
and original authorship above. The witness uses only Python's standard library.

## Next action and exact remaining discriminator

The selected all-size iteration stopped at a concrete obstruction: the inequality
`b(u)^2-Phi*a(u)*b(u)+Psi*a(u)^2<=0` for every direction is false even at an
equality zonotope. The retained witness has `Phi=32`, `Psi=256`, `a=22`, `b=324`
and defect `784`. It does **not** refute `4Psi<=Phi^2`.

For `rho=max_u b(u)/a(u)`, the repaired bound
`Psi<=rho*(Phi-rho)` is universally equivalent to the original conjecture.
The next all-size discriminator is an actual proof of that maximum-radius
bound, an exact counterexample, or a different rank-preserving reduction.
The [targeted source check](all-size-mechanism/source-note.md) found a
three-dimensional Bonnesen precedent, but no theorem supplying this
four-dimensional symplectic step. The equivalence is not itself a new proof.

The unrestricted mathematical problem starts at eight generators. An earlier,
narrower resumption point is [the complement-of-cube weight reduction](frontier/complementcube-weight-reduction.md):
prove `A u^4+C u^3+E u^2+D u+B>=0` for all `u>0` with the linked coefficients
specified there, or give an exact positive parameter choice making it negative.
The reduction itself still needs independent checking. A proof would cover only
that support graph and positive Pfaffian-matching chamber, not the unrestricted
conjecture. Other support graphs remain separate obligations.

The coordinator stopped further eight-index enumeration and optimization on
29 October 2026. The subsequent bounded all-size mechanism iteration is now
complete at the blocker above.
This packet preserves a resumption point; it does not authorize renewed work.
Exploratory graph catalogues, numerical searches, rendered pages and transient
scripts remain outside this deliberately small proof packet and are not used
as support for its theorem statements.

## Replay and rebuild

Replay the retained exact obstruction from the repository root:

```bash
python topics/empirical-studies/discovery-design/desk/zonotope-research/rank-four-restart-20261029/all-size-mechanism/projection-witness.py
```

This verifies the specified integer witness, not the unrestricted conjecture.

From the repository root, direct outputs to scratch:

```bash
mkdir -p /tmp/msc-math/rank-four-pfaffian-build
latexmk -pdf -interaction=nonstopmode -halt-on-error \
  -outdir=/tmp/msc-math/rank-four-pfaffian-build \
  topics/empirical-studies/discovery-design/desk/zonotope-research/rank-four-restart-20261029/rank-four-pfaffian.tex
```

A successful build checks the LaTeX interface, not the proof or novelty.
