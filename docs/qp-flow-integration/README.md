# Bounded QP and flow integration proposal

18 September 2026. **Ready-to-apply patch against the interrupted production
sources, not applied to production.** This packet makes the QP derivation
explicit and selects the existing repaired flow chapter. A disposable copy of
the actual interrupted assembly, including its uncommitted overlays, compiled
to 76 pages with no final LaTeX warnings, undefined references/citations, or
overfull boxes. This is not a mathematical acceptance or visual review result.

## Artifacts and input identity

- `integration.patch`: changes QP proof, flow input selection, and one overflowing
  display in the selected flow replacement.
- `proposed/`: full proposed QP, flow and main source files. Flow is the existing
  interrupted overlay with its return-map/action display split across two displays;
  no flow theorem or algorithm was changed.
- `manifest.json`: hashes of exact production prerequisites and proposed files.
- `check-inputs.py`: verifies those hashes before applying.
- `build-check.json`: actual local inputs consumed by the isolated build, their
  SHA256 hashes, and build outcome.

The production root is `/tmp/msc-math-thesis-review-20260917`; its candidate
sources are not the clean candidate sources in this research worktree. Applying
this patch to the clean research checkout fails as expected: that checkout does
not contain the interrupted QP overlay or changed main. The production PDF and
its old build receipts have not been touched.

## Mathematical and source checks performed

The QP proof now obtains both inequalities before using the formula:

1. Positive feasible value q gives T=1/(2q), a closed pure-velocity loop with
   A=I=T. Clarke duality gives c≤T, hence q≤1/(2c). Nonpositive q cannot violate
   that upper bound.
2. The already-assembled simple capacity minimizer has period c and normalized
   dwell weights giving Q=1/(2c)>0 by the same shoelace formula.
3. Finite words and compact feasible sets supply attainment. HK attribution,
   weight conversion, reversed-word convention and the subsequent translation
   of a globally optimal dual curve onto the boundary remain explicit.

Checked against the actual assembled `02-duality.tex`, recovered symplectic
notation and new `03-generalized-reeb-orbits-polytopes.tex`:
J0=[[0,-I],[I,0]], omega(u,v)=<J0u,v>, Ri=2J0ai,
2A=Στjτk omega(Rj,Rk), and H* = h²/4. Thus omega(Rj,Rk)=4omega(aj,ak),
-J0Ri=2ai and h(ai)=1. No use of the headline QP formula enters either inequality.
The existing later realization proposition can now use the proved formula
without obscuring the derivation. Simple-minimizer and nonsmooth duality
foundations are dependencies, not newly established in this bounded task.

Flow dependency closure:

- The selected orbit chapter provides the named simple-minimizer theorem and
  velocity/action convention. Flow does not require QP's later realization.
- The CH background comparison resolves to
  `legacy/05-flow-graph-ch2021-background-comparison.tex`.
- The figure resolves to
  `legacy/figures/flow-graph/flow-graph-f6-tube-sequence.pdf`.
- Existing labels remain resolvable after switching from recovered flow to the
  top-level overlay; the isolated full build has no unresolved references.
- The replacement already rejects empty closed tubes before fixed-point solving,
  states its regularity hypotheses and separates rational runtime input contracts.
  These are retained, not additional repairs claimed by this packet.

The first isolated build exposed a 72.7pt overflow in the prepared flow example.
Separating the return map and action formula removed it. The final build's log
has no warning/overfull/error matches. No visual inspection was performed.

## Application by the eventual assembly owner

Preserve and commit the interrupted overlays coherently before integration. Do
not cherry-pick this documentation commit expecting it to activate chapters.
From the production repository root, after reconciling any newer changes:

```sh
python3 /workspaces/msc-math/.worktrees/qp-flow-integration-patch/docs/qp-flow-integration/check-inputs.py /tmp/msc-math-thesis-review-20260917/thesis/candidate
git apply --check /workspaces/msc-math/.worktrees/qp-flow-integration-patch/docs/qp-flow-integration/integration.patch
git apply /workspaces/msc-math/.worktrees/qp-flow-integration-patch/docs/qp-flow-integration/integration.patch
```

Both the hash check and `git apply --check` passed against the interrupted source
at handoff. Hash mismatch means compare and adapt; do not overwrite newer work
with the full proposal copies. The patch assumes the interrupted untracked flow
and QP files are present. If integrating elsewhere, first promote the chosen
interrupted sources and their dependencies deliberately.

## Remaining checks

- Independent mathematical reading of the selected foundations/QP chain,
  especially if preliminaries are later switched to their separate overlay.
- Full bounded flow theorem/runtime comparison from the technical closure plan
  (singular/empty/short-word cases, cutoff semantics and exact input predicates).
  No Rust tests or expensive evidence producers were run here.
- Verify the new flow example's displayed rounded values against its producer
  before treating their addition as reviewed numerical evidence. Input/figure
  availability and successful TeX are weaker than that check.
- Final visual/prose review of these pages and whole-manuscript coherence.
  The 76-page diagnostic PDF still includes unrelated stale companion claims;
  it is not a delivery candidate and was discarded after recording the build.
- Preserve the existing six-facet theorem/algorithm/proof ordering; this packet
  does not change it or attempt any general-germ theory.

Planning references:
`/workspaces/msc-math/.worktrees/technical-closure-plan/docs/technical-closure-plan/README.md`
and
`/workspaces/msc-math/.worktrees/assembly-source-map/docs/assembly-source-map/README.md`.
The retained worktree is an active proposal/reference; disposable build scratch
was removed. No secrets were read.
