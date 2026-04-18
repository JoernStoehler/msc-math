<!--
Purpose: scope a temporary numerics-methods experiment that measures how far
SageMath can carry the repo's end-to-end capacity workflow.
Context: the repo now has three usable lanes:
1. Rust `f64` search/orchestration,
2. Rust exact algebraic kernels for selected certification work,
3. Sage as an independent exact oracle on selected one-sigma records.
This note defines the next experiment that probes whether Sage can also serve
as an end-to-end capacity baseline for small and medium facet counts.
-->

# Sage Feasibility

## Research question

How far can SageMath carry an end-to-end EHZ-capacity computation on the repo's
polytope search shape before performance or code complexity becomes a bad fit?

Subquestions:

1. Can Sage perform the full HK2017-style unpruned sigma search loop on small
   rational polytopes without heroic engineering?
2. Can the same Sage code shape handle both
   - exact rational / algebraic arithmetic, and
   - machine-double style approximate arithmetic (`RDF`)?
3. For the same formulas, how does Sage compare to the existing Rust paths on:
   - exact trusted baseline work,
   - fast `f64`-style search?
4. Is the natural long-term split
   - Rust for `f64` numerics and orchestration,
   - Sage for exact baseline / verification,
   supported by actual timings and code size?

## Hypothesis

Expected outcome:

- Sage exact is viable as a trusted baseline for selected full-search runs on
  modest facet counts and for selected algebraic cases.
- Sage `RDF` can reuse almost the same formulas, but will not beat the Rust
  `f64` path on raw end-to-end throughput.
- The clean split is:
  - Rust owns production search, batching, and `f64` numerics;
  - Sage owns exact baseline and selected independent verification.

## Scope

This packet is a temporary methods experiment under `experiments/numerics/`.

In scope:

- input: dual vertices of already-chosen polytopes;
- no boundedness / irredundancy validation;
- no pruning;
- HK2017-style sigma enumeration;
- one-sigma KKT solve;
- aggregate all minimum-action orbits and return capacity;
- exact modes:
  - `QQ` for rational controls,
  - `Q[t]/(t^4 - 10 t^2 + 5)` with the chosen real root for HKO;
- approximate mode:
  - `RDF` with the same control flow and formulas when possible;
- timing and code-complexity assessment.

Out of scope:

- broad library integration;
- symbolic formula derivation;
- automatic rigorous error propagation in Sage;
- pruning or unknown-predicate semantics;
- thesis-facing claims beyond feasibility and order-of-magnitude timings.

## Benchmark bank

First-pass bank:

- rational integer-coordinate controls with facet counts `F = 5, 6, 7, 8, 9, 10`;
- HKO2024 exact as the algebraic `F = 10` case;
- optionally the rational/dyadic HKO approximation if useful as a bridge row.

Prefer existing named controls from the repo over freshly generated random
polytopes for the first pass.

Reason:

- existing controls already have known structure and expected capacities;
- failures are easier to interpret than on ad-hoc random inputs.

## Search-size reality check

Unpruned cyclic-permutation counts:

- `F = 5`: `84`
- `F = 6`: `409`
- `F = 7`: `2365`
- `F = 8`: `16064`
- `F = 9`: `125664`
- `F = 10`: `1112073`

Interpretation:

- `F <= 7` should be cheap in all modes;
- `F = 8` is already a meaningful feasibility point;
- `F = 9` is real work;
- `F = 10` unpruned exact is the stress test.

## Approach options

### Option A: Sage-only script

One `sage -python analyze.py` script under
`experiments/numerics/sage-feasibility/` that

- defines the benchmark bank directly in Python, or
- reconstructs it from a tiny embedded table.

Pros:

- smallest code surface;
- easiest to iterate;
- no Rust helper needed.

Cons:

- duplicates benchmark-bank definitions if the same polytopes are already
  defined more naturally in Rust;
- harder to reuse existing repo fixture constructors.

### Option B: Rust exporter + Sage driver

Rust binary exports the benchmark-bank dual vertices to JSONL; Sage reads that
artifact and performs the full search.

Pros:

- reuses exact repo polytope fixtures without re-encoding them by hand;
- clean separation between fixture generation and Sage search logic.

Cons:

- larger packet;
- one more artifact and interface to maintain.

### Recommendation

Start with Option B unless the benchmark bank turns out to be tiny enough to
encode directly in Sage in under ~30 lines.

Reason:

- the repo already has stable polytope fixtures in Rust;
- the experiment question is about Sage search feasibility, not about manually
  re-entering vertex data;
- a Rust-exported bank also gives a clean path to later compare Sage and Rust
  on the same exact inputs.

## Planned layout

Prefer:

- `experiments/numerics/sage-feasibility/main.rs`
  - optional exporter for the benchmark bank if Option B is used;
- `experiments/numerics/sage-feasibility/analyze.py`
  - Sage search/benchmark driver;
- `experiments/numerics/sage-feasibility/sage-feasibility.jsonl`
  - canonical timing/results report;
- `experiments/numerics/sage-feasibility/smoke-sage-feasibility.jsonl`
  - smoke output.

If an exported bank is needed:

- `experiments/numerics/sage-feasibility/sage-feasibility-input.jsonl`
- `experiments/numerics/sage-feasibility/smoke-sage-feasibility-input.jsonl`

## Row schema

Each result row should record at least:

- `polytope`
- `facet_count`
- `scalar_mode` in `{rdf, rational_exact, algebraic_exact}`
- `sigma_count_total`
- `sigma_count_admissible`
- `minimizer_count`
- `capacity`
- `wall_time_ms`
- whether the run completed, timed out, or failed

Useful optional columns:

- `median_solve_ns` for the one-sigma KKT kernel
- `median_action_ns`
- `notes` for interpretation such as "algebraic F=10 hit timeout"

## Verification contract

For this experiment, the important outputs are:

1. completion / timeout behavior by scalar mode and facet count;
2. capacity agreement between Sage exact and known rational controls;
3. capacity agreement between Sage exact and Rust exact / Rust `f64` where
   comparison is meaningful;
4. wall-clock order of magnitude.

Do not oversell:

- this experiment is feasibility and methods evidence;
- it is not a proof that Sage should replace the Rust search stack.

## Acceptance checks

Minimal successful packet:

```bash
cargo build -p dev-numerical-analysis --release --bin num-sage-feasibility
cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --smoke
cd experiments/numerics/sage-feasibility && sage -python analyze.py --smoke
cd experiments/numerics/sage-feasibility && sage -python analyze.py --canonical
```

And the canonical report should answer:

- which `F=5..10` rational rows completed in exact mode;
- whether HKO exact `F=10` completed, timed out, or was too slow to be a useful
  baseline;
- whether `RDF` mode runs with the same code shape;
- how Sage `RDF` compares in order of magnitude to Rust `f64`;
- rough code complexity:
  - exact-only path LOC,
  - extra LOC needed for `RDF`.

## Expected interpretation paths

### If Sage exact is fine up to `F=8` or `F=9`, but bad at `F=10`

Then use Sage for:

- exact baseline on selected hard cases,
- exact full-search runs only on smaller controls,
- independent spot checks of thesis-critical exact records.

### If Sage `RDF` is still much slower than Rust `f64`

Then keep:

- Rust as the production search/orchestration path,
- Sage as the exact baseline and independent oracle.

### If Sage exact is unexpectedly competitive even at `F=10`

Then consider a later follow-up:

- a Sage-only baseline experiment for a broader bank,
- or a hybrid workflow where Rust exports fixtures and Sage owns the trusted
  exact end-to-end reference.

## Process notes

- This experiment is temporary and exploratory. Start in `experiments/`.
- Use smoke outputs by default.
- Only refresh canonical JSONL deliberately.
- Record dead ends and performance surprises in the note or experiment output so
  later sessions do not rerun the same failed setup.
