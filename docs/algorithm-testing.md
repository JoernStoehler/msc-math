# Algorithm testing and evidence

Algorithm checks in this repository have different jobs. A fast unit test is a
good development gate; it is not interchangeable with an independent known
value, a cross-implementation comparison, or a theorem-specific certificate.
Use the smallest layer that catches the defect while developing, then run the
affected confidence layers before relying on a scientific result.

## Evidence layers

| Layer | What it checks | What a pass does not establish |
| --- | --- | --- |
| **Compile smoke** | Formatting and compilation across the root Cargo workspace. | Runtime behavior; the root workspace also excludes standalone experiment method crates. |
| **Development regression** | Local contracts, errors, edge cases, and public API behavior. Focused filters should be quick enough to run while editing; complete crate gates can take longer. | Mathematical correctness of the whole algorithm or representative behavior outside the fixtures. |
| **Cross-route correspondence** | Production, readable/instrumented, legacy, numerical, or exact-control routes agree where their contracts overlap. | Correctness when the routes share logic or the same mistake; coverage beyond the compared fixtures. |
| **Properties and anchors** | Behavioral invariants such as scaling, symplectic invariance, monotonicity, and continuity, plus literature or hand-checkable values. | The universal mathematical property: these are finite checks. |
| **Rich-output reconstruction** | A scalar result remains consistent with minimizing words, KKT data, recovered geometry, closure, facet adherence, and action. | Completeness outside the retained target pool or proof-strength numerical margins. |
| **Theorem/risk certificate** | Exact or exhaustive predicates tied to a named mathematical implication, or targeted falsifiers for a known proof risk. | More than the packet's stated input class and implication. Exact arithmetic alone does not supply a missing theorem. |
| **Performance and numerics** | Runtime, memory, conditioning, residuals, fallback frequency, and error behavior. | Mathematical correctness. Keep performance claims separate from correctness claims. |

Independence matters. Agreement with a route that shares enumeration helpers is
useful regression evidence, but a literature value, symmetry, exact alternative,
or independently reconstructed orbit can catch different failures. Source review
and formal arguments establish why code is intended to implement the mathematics;
execution checks probe that correspondence from finite, complementary angles.
Neither replaces the other.

## Capacity and systolic-ratio stack

Capacity is unusually well covered because the repository can inspect related
capacity contracts at several levels: public scalar bounds, alternative
implementations, mathematical properties and known values, minimizing words,
and recovered orbits. Those layers do **not** all exercise the same evaluator.
In particular, the retained axioms and orbit-recovery packets use the legacy
pruned/unpruned HK2017 and billiard routes. There is currently no corresponding
end-to-end property or geometric-recovery suite for the production
`capacity_4d` API. The current entry points are:

### 1. Compile smoke and development gate

A warm compile smoke currently takes about ten seconds on the development
machine:

```bash
cargo fmt --all -- --check
cargo check --workspace
```

This is only the root Cargo workspace, not every standalone method crate under
`experiments/sys-datascience/`. It is a quick edit check, not a repository-wide
runtime test.

The reusable capacity regression gate is:

```bash
cargo test -p symplectic --release --lib
cargo test -p symplectic --release --test public_capacity_api
```

Use focused test filters during an edit. Run both commands for a reusable
capacity API change. They do not rewrite repository evidence, although Cargo
does update build/test files under `target/`. With a warm cache these two
commands currently take about 40 seconds together; timings are orientation, not
a contract.

### 2. Production-to-control correspondence

```bash
cargo test -p exp-dev-quadratic-program --release \
  --test selected_route_correspondence
```

This compares selected production behavior with readable or exact/control
routes for general bounds, an action window, window derivatives, and product
capacity/winners. It is a finite correspondence gate, not an independent proof:
some compared paths deliberately share exact kernels or enumeration helpers.

### 3. Properties and known-value anchors

```bash
cargo test -p dev-capacity-validation --release --bin axioms-correctness
```

This test checks the retained `correctness.jsonl` cases against the current
legacy/research orbit-route code without rewriting that evidence file. The six
families cover direct pruned/unpruned/billiard implementation agreement,
literature values, conformality, symplectic invariance, continuity, and
monotonicity. They do not currently test those properties end to end through
the production `capacity_4d` dispatcher. See
[`experiments/verification/correctness/`](../experiments/verification/correctness/).

Refreshing that tracked evidence is a different operation:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-correctness
```

Run the producer only when reviewing and committing an intentional artifact
refresh.

### 4. Minimum-set and geometric reconstruction

The default commands write disposable smoke outputs:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery
```

The full chain refreshes tracked evidence for 28 selected polytopes and the
retained 469 near-minimum orbit rows produced by the legacy/research orbit routes:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum -- --full
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery -- --full
```

Run the full chain after changes to candidate generation or aggregation,
minimum-row schemas, orbit reconstruction, or its tolerances. Read the
[`all-minimum`](../experiments/verification/all-minimum/) and
[`orbit-recovery`](../experiments/verification/orbit-recovery/) contracts first;
`--full` overwrites tracked artifacts.

This is rich-output confidence for the orbit-producing routes, not geometric
validation of the sparse exact winners returned by production `capacity_4d`.

### 5. Route-specific proof risks and certificates

Use only the packet affected by the claim. Examples include the exact
flow-graph falsifier packet and topic-local Sage certificates:

```bash
# Writes disposable smoke output; add `-- --full` only for an intended
# tracked-artifact refresh.
cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk
```

The flow-graph packet checks exact-flow-graph versus certified HK/QP output,
direct word resolution, cutoff behavior, and typed singular/zero-action cases.
Its README states the proof boundaries it does not close. Specialized Sage
packets under `experiments/` own their exact input class, reproduction command,
and mathematical implication.

## Choosing checks for a change

1. Run the focused unit test while editing, then the owning crate's development
   gate.
2. Run a correspondence gate when changing a production algorithm, its readable
   copy, dispatch, candidate coverage, arithmetic fallback, or shared kernel.
3. Run properties and anchors when a scalar result or accepted input class can
   change.
4. Run the reconstruction chain when minimizers, KKT payloads, orbit geometry,
   schemas, or tolerances can change.
5. Run a theorem/risk packet only when its route or stated implication is in the
   change's dependency cone.
6. Benchmark separately when the change can affect cost. A faster passing route
   and a scientifically stronger route are different claims.

When a required full run is too slow for the current work window, do not replace
it with a stronger-sounding description of smoke results. Record the fast checks
that passed, the full check still owed, and the scientific claims that remain
blocked on it.

The calculation contracts themselves—including arithmetic, candidate coverage,
and failure semantics—are mapped in
[`capacity-calculation-map.md`](capacity-calculation-map.md).
