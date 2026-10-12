# Flow graphs and exact tube search

The flow-graph route computes capacity through finite simple facet words and
exact affine tube fixed points. Its mathematical correctness theorem concerns
an idealized real-number algorithm on a specified nondegenerate, finite-orbit
regular input class. The implemented search uses rational arithmetic and an
explicit rejection boundary. Keeping those two layers visible prevents a
finite test suite or a paper-inspired geometric model from becoming an
unqualified capacity claim.

## Result and proof structure

A state records an oriented two-face, and a primitive triple `[i,j,k]` follows
the Reeb direction of facet `j` from the entry two-face to the exit two-face.
Gluing compatible tubes composes affine maps while intersecting their endpoint
domains. A closed facet word is realized by a fixed point; accepted strict
orbits require every reconstructed segment time to be positive.

[The idealized algorithm proof](flow-graph-real-algorithm.tex) owns the concrete
proof chain: primitive tubes, gluing, empty-subtube pruning, fixed points,
action normalization and exact action cutoffs. Its
`thm:fg-real-capacity-correctness` assumes a full-dimensional normalized facet
presentation, nonzero symplectic pairing for each nonempty distinct facet
intersection, independence of every subset of at most four dual rows, and
finite-orbit regularity of the exhaustive run. It uses the HK2017 simple
capacity minimizer to place a capacity-achieving word inside the finite search.
Short words cannot have a positive-time orbit under the independence
hypothesis; longer represented words have nonsingular fixed equations under
finite-orbit regularity.

The genericity corollary is relative to a common primitive-denominator domain.
The presentation-chamber proposition works inside fixed ordered normalized
irredundant H-presentation chambers open in dual-row coordinates and imposes
all-pair nonzero symplectic pairing, a stronger restriction than the runtime
nonempty-pair condition. It is not an unrestricted genericity assertion for
every polytope representation. Existing review markers and `unverified`
wrappers are preserved; no theorem is promoted by this relocation.

The proof packages have distinct purposes:

| Canonical source | Role and boundary |
| --- | --- |
| [Idealized real algorithm](flow-graph-real-algorithm.tex) | concrete conditional correctness and chamber-relative genericity |
| [Abstract semantic search](flow-graph-capacity.tex) | sound/complete resolver and lower-bound search axioms; not certification that Rust satisfies them |
| [CH2021 comparison](flow-graph-ch2021-comparison.tex) | geometric inspiration and precise limits on paper-to-project transfer |
| [Proof-risk ledger](flow-graph-proof-risk.tex) | local obligations, falsifiers and the length-three zero-time lemma |

CH2021 supplies flow-graph language and motivation. Its smoothing theorem,
rotation pruning and Type 1/2/3 classification do not prove this project's
implementation returns capacity. The project's capacity bridge uses HK2017
simple minimizers plus its local tube and search proofs. Degenerate transitions,
including zero-pairing products, remain outside this primitive-tube model.

## Implementation contract and evidence

The maintained [implementation contract](implementation-contract.md) owns the
full input/outcome, dependency, proposition and evidence ledgers.
[The Rust module](../../crates/symplectic/src/algorithms/flow_graph/mod.rs) owns
public exports; [exact_tube.rs](../../crates/symplectic/src/algorithms/flow_graph/exact_tube.rs)
resolves a selected rational closed word, and
[exact_search.rs](../../crates/symplectic/src/algorithms/flow_graph/exact_search.rs)
enumerates simple words, applies exact cutoffs and collects retained orbits.

Exhaustive search checks shapes, nonempty-pair zero symplectic pairing, and
independence up to four rows. Direct word resolution has a weaker boundary:
its common validator checks shapes, followed by local transition construction.
Neither entry point fully checks bounded irredundancy or consistency of supplied
incidence/sign matrices with the normals; callers supply those facts through
trusted fixtures/data generation. Finite-orbit regularity is not a globally
certified runtime input predicate.

The search skips words of length at most four under checked independence and
classifies singular fixed sets locally. Positive-action singular candidates
and other unsupported intersecting singular cases cause typed rejection;
exactly empty singular sets and the proved length-three zero-time case may
return no-orbit. Exhaustive no-positive-orbit completion is a typed
non-success. The idealized regularity theorem avoids relevant long-word
singular equations and does not certify the entire Rust singular classifier.

Accepted output reports exact action and cyclic words within the requested
nonnegative action gap. Exact cutoff pruning starts only after a positive
exact action is known and is tested against a disabled baseline. On selected
deterministic F5/F6 fixtures the suite compares scalar capacity with certified
QP and separately checks retained-word completeness. QP is a scalar oracle in
these comparisons, not a general word-level oracle. Ignored F7 checks and
historical timings remain recorded evidence, not newly rerun results.

The retained [theorem/runtime closure audit](runtime-closure/README.md) records
the 18 September source correspondence and an independent exact witness
checker. Runtime success is not a test of finite-orbit regularity: an empty
singular domain, or one pruned above the action window, can be harmless without
meeting the idealized global regularity hypothesis. The audit is dated evidence,
not a replacement for the live contract.

The earlier binary64 prototype is retired. No current floating FG capacity API
or soundness claim follows from it. A future floating frontend would need sound
true/false/indeterminate predicates or exact resolution of every decision that
can affect output.

## Consumers and bounded checks

[The selected thesis section](../../thesis/chapters/05-flow-graph.tex) is a
separate reader-facing consumer. [Development experiments](../../experiments/dev-flow-graph/README.md)
own frontier counts, spikes and profiling; experimental word counts are not
capacity certificates. The two legacy `tube-algorithm*.md` notes remain with the
Rust module as imported source evidence and are subordinate to the maintained
contract and formal proof.

From the repository root, a small boundary check is:

```bash
cargo test -p symplectic --lib algorithms::flow_graph::exact_search::tests
```

This exercises rejection, short-word and no-orbit policy. It does not establish
the capacity theorem or reproduce all retained comparisons. The contract maps
broader default and ignored exact checks; full producers and ignored suites
need their own cost decision.

## Source disposition

The four `formal/flow-graph-*.tex` paths are compatibility inputs forwarding to
this topic's canonical proof sources, with labels defined once. The previous
Rust-local long README has moved here as `implementation-contract.md`; its old
path is a short router. Rust module paths and public APIs stay intact.
Remaining implementation/theorem correspondence and broader singular support
are explicitly scoped proof work, reopened when selected for thesis review or
API expansion rather than silently resolved by a directory move.
