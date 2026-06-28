# Tube Algorithm Legacy Source Note

## Status

Epistemic status: legacy/imported source material.

Live flow-graph algorithm control surface:
`crates/symplectic/src/algorithms/flow_graph/README.md`.

This file is retained because it records earlier Jörn-confirmed import material
and stale-surface cleanup. It is no longer the live source of truth for the
current flow-graph implementation, tests, experiments, or thesis status.

Current state, as of 2026-05-04: the algorithm mostly exists in Jörn's head and
on paper. Old repo material was deleted from the active tree because it was not
trusted as a specification. Use git history only if comparison with the stale
drafts becomes necessary.

Refresh or invalidate this note only when migrating its remaining useful content
into the live flow-graph README or a proof/thesis source.

## Accepted Clarifications

- [accepted 2026-05-04] The nondegeneracy condition is local to trajectory
  transitions. The algorithm does not need `omega_0(a_i,a_j) != 0` for every
  pair of facets. Facet pairs that never occur as a nonempty trajectory
  transition may have `omega_0(a_i,a_j) = 0`. The intended weaker condition is:
  whenever the point-intersection relevant to a trajectory transition is
  nonempty, the corresponding `omega_0(a_i,a_j)` is nonzero.
- [accepted 2026-05-04] Rotation pruning is not part of the first implementation
  milestone. The first milestone should write a TODO for rotation and keep the
  algorithm correct without it. Later implementation can add the
  Conley-Zehnder/rotation cutoff behind an easy-to-disable flag, because it is
  control-flow pruning by a scalar number rather than part of the affine tube
  data.
- [accepted 2026-05-04] The target output is `capacity` and all simple Reeb
  orbits below `capacity + threshold`. The action pruning rule is therefore
  based on `segment_action <= best_action_so_far + threshold`.
- [accepted 2026-05-04] The implementation should use a functional-programming
  style with modular primitives. Define what a tube is, how to intersect tubes,
  how to build primitive three-facet tubes `(a_1,a_2,a_3)` describing flow from
  `a_1 cap a_2` to `a_2 cap a_3` along `R_2 = 2J a_2`, how to detect empty
  tubes, and how to solve fixed points of closed tubes. The orchestrator is a
  separate layer that chooses the tube-build order, first to get a good action
  bound quickly and then to exhaust the full tube set. An empty sub-segment
  implies every containing tube is empty.
- [accepted 2026-05-04] A closed tube has combinatorics
  `(a_1,a_2,...,a_k,a_1,a_2)`. The start and end both live on
  `a_1 cap a_2`; fixed points are solved on that two-face.
- [accepted 2026-05-04] For thesis/numerics wording, it is acceptable to state
  a stronger input condition than exact mathematics needs.
- [accepted 2026-05-04] Thesis scope: implementation and empirical validation
  are still part of the desired complete tube story. Theory-only is not enough
  for the ideal outcome, but including clean theory without empirics is better
  than dropping the tube algorithm entirely.

## Deleted Repo Noise

These stale surfaces were removed from the active tree on 2026-05-04. They were
not trusted as specifications; use git history only if comparison material is
needed.

- `thesis/tube-algorithm.tex`: long agent-written thesis draft. It contained
  many Jörn TODOs and unreviewed claims about rotation, closing, and Type 2
  orbits.
- `formal/tube-algorithm.tex`: compressed formal copy sourced from the thesis
  draft, wrapped in `unverified` blocks. It was not independent proof-bearing
  source.
- `crates/symplectic/src/algorithms/tube/mod.rs`: blocked test-only Rust module.
  Its header said the rotation-increment formula was incorrect and
  `tube_capacity` was not re-exported.
- `thesis/tube-algorithm-notes.md`: stale migration task note. It pointed to
  deleted files and old `library/` paths.
- `thesis/legacy/migration-findings.md`: legacy-era mismatch inventory. Rows 1
  and 11-14 may still be useful as a checklist for conflicts between old thesis
  prose and old code, but need revalidation before driving current work.

The removed tail of this note contained active-looking templates, old success
criteria, implementation notes, and questions. They were intentionally deleted
from the live file because they now compete with the flow-graph README. Use git
history only if migration needs a specific old sentence.
