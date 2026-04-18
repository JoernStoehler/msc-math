<!--
Purpose: proposed API boundary for arithmetic and linear algebra over real
algebraic extensions of Q.
Context: this note turns the algebraic-exactness spike into an implementable
API plan that agents can refine while usage feedback arrives.
-->

# Real Algebraic Scalar API

## Goal

Provide a small reusable scalar layer for arithmetic and sign-sensitive linear
algebra over ordered real field extensions of `Q`, without trying to become a
symbolic computer algebra system.

The intended use cases are:

- proof-grade geometry and KKT certification for selected polytopes;
- high-trust reference values for floating-point error analysis;
- exact handling of symmetry-driven degeneracies whose intended coordinates are
  algebraic rather than rational.

The intended non-use cases are:

- symbolic simplification beyond field arithmetic;
- transcendental functions such as `sin`, `cos`, `exp`, `log`;
- generic polynomial factorization;
- replacing the current library geometry and KKT code immediately.

## Design Bias

Follow `KISS`, `YAGNI`, and predictable Rust conventions.

That means:

- use `BigRational` directly instead of wrapping it;
- use operator overloading for arithmetic because that is standard Rust;
- keep the first reusable API small enough that one reader can understand it in
  one pass;
- still write more convenience code than a minimal public library would, because
  our consumers mostly want to write formulas and small elimination routines as
  if the scalars were `f64`;
- add generic runtime-defined number fields only after a second real caller
  exists.

## Naming

Avoid the word `exact` in the long-term public API.

Recommended vocabulary:

- crate: `real-algebraic` or `algebraic-scalars`
- trait: `OrderedField`
- static field spec trait: `StaticFieldSpec`
- generic algebraic element type: `Algebraic<S>`
- runtime field context: `RealNumberField`
- runtime field element: `RealNumberFieldElem`
- concrete field-spec marker types: `TanPiFifth`, later other named specs if
  useful

Reason: the important mathematical property is not "exactness" in the abstract
but "ordered arithmetic in a chosen real embedding".

## Boundary

The reusable boundary is the scalar layer, not the whole current experiment.

Keep inside the scalar crate:

- rational scalar support via `BigRational`;
- ordered field trait(s);
- generic algebraic element implementation for `Q[t]/(p(t))`;
- field-spec marker traits and helper macros if useful;
- sign and comparison logic;
- canonical coefficient serialization metadata.

Keep outside the scalar crate:

- polytope construction;
- vertex enumeration;
- KKT assembly and solve code;
- experiment JSONL row schemas;
- HKO fixtures and polytope constructors.

## Core Model

Every non-rational field must come with a chosen real embedding.

That means:

- total order is valid;
- `sign()` and `cmp()` are mathematically meaningful;
- two elements from different runtime field contexts are not comparable.

The rational field is not wrapped in a repo-specific type in v1.

Use `num_rational::BigRational` directly because:

- the library already uses it;
- interop stays trivial;
- serde and trait plumbing stay simple;
- a custom wrapper would mostly forward methods without adding value.

If naming consistency helps, a type alias is enough:

```rust
pub type Rational = num_rational::BigRational;
```

For algebraic fields, the primary Rust model should be:

- one generic element container supplied by the crate;
- one consumer-declared field-spec marker type per named field.

That is the predictable Rust substitute for dependent types.

## API Shape

Arithmetic should use ordinary operators.

Recommended:

- implement `Add`, `Sub`, `Mul`, `Div`, `Neg`;
- support borrowed operands where practical so arithmetic-heavy code does not
  drown in clones;
- provide explicit methods for order-sensitive and embedding-sensitive
  operations.

Do not require callers to write method-based arithmetic such as
`field.add(a, b)`. It makes elimination code and determinant code much harder to
read.

The intended user experience is:

- arithmetic-heavy code should read like ordinary numeric code;
- the caller should not need to build utility layers before writing a 4x4
  determinant, Gaussian elimination, or KKT assembly;
- common embeddings and sign queries should be one method call away.

So this crate should be comfortable to use, not aggressively minimal.

### Scalar Trait

Proposed minimal trait for algorithm code:

```rust
pub trait OrderedField:
    Clone
    + std::fmt::Debug
    + PartialEq
    + Eq
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;

    fn from_rational(value: BigRational) -> Self;

    fn sign(&self) -> Sign;
    fn cmp_real(&self, other: &Self) -> std::cmp::Ordering;

    fn to_f64(&self) -> f64;

    fn canonical_coeffs(&self) -> Vec<BigRational>;
    fn field_id() -> FieldId;

    fn is_zero(&self) -> bool { self.sign() == Sign::Zero }
    fn is_positive(&self) -> bool { self.sign() == Sign::Positive }
    fn is_negative(&self) -> bool { self.sign() == Sign::Negative }
}
```

Notes:

- `cmp_real()` is explicit even if some implementations also support `Ord`.
- `to_f64()` is for reporting, debugging, and floating-point cross-checks, not
  for correctness decisions.
- `canonical_coeffs()` exists for persistence and debugging, not for ordinary
  algorithm code.

### Convenience Methods

The crate should provide standard convenience constructors and predicates even
if they are derivable from the core trait.

Recommended:

```rust
fn from_i64(value: i64) -> Self;
fn from_frac(numer: i64, denom: i64) -> Self;

fn is_zero(&self) -> bool;
fn is_positive(&self) -> bool;
fn is_negative(&self) -> bool;
```

Also provide small generic helpers that keep arithmetic code clean:

```rust
pub fn min_field<F: OrderedField>(a: F, b: F) -> F;
pub fn max_field<F: OrderedField>(a: F, b: F) -> F;
pub fn dot<const N: usize, F: OrderedField>(a: &[F; N], b: &[F; N]) -> F;
```

These are not mathematically deep. They exist so downstream code stays short and
predictable.

### Borrowed Arithmetic

The implementation should support borrowed arithmetic where practical.

Reason: once formulas stop fitting on one line, `clone()` noise becomes the main
readability cost.

Recommended support level:

- start with owned arithmetic so the design is easy to read;
- add `&a + &b`, `&a * &b`, and similar impls for hot or noisy paths;
- do not try to exhaust every operand combination on day one if that creates
  busywork.

### Sign Type

```rust
pub enum Sign {
    Negative,
    Zero,
    Positive,
}
```

Keep it tiny and stable. Avoid mixing it with floating-point epsilon logic.

### Metadata

Use a small field-identity surface for serialization and diagnostics:

```rust
pub enum FieldId {
    Rational,
    PentagonTanPiFifth,
}
```

## Type-Level Field Specs

The primary API for named fields should use a field-spec marker type.

Recommended shape:

```rust
pub trait StaticFieldSpec: 'static + Send + Sync {
    fn field_id() -> FieldId;
    fn name() -> &'static str;

    fn minimal_polynomial() -> Vec<BigRational>;
    fn isolating_interval() -> (BigRational, BigRational);
}

pub struct Algebraic<S: StaticFieldSpec> {
    coeffs: Vec<BigRational>,
    _marker: std::marker::PhantomData<S>,
}
```

Then a consumer defines a named field as:

```rust
pub struct TanPiFifth;

impl StaticFieldSpec for TanPiFifth {
    fn field_id() -> FieldId { FieldId::PentagonTanPiFifth }
    fn name() -> &'static str { "Q[tan(pi/5)]" }
    fn minimal_polynomial() -> Vec<BigRational> { ... }
    fn isolating_interval() -> (BigRational, BigRational) { ... }
}

pub type TanPiFifthField = Algebraic<TanPiFifth>;
```

Why this is the preferred shape:

- it is a standard Rust pattern;
- named fields become distinct Rust types;
- arithmetic stays generic over one implementation;
- consumers do not need to hand-write a bespoke element type per field.

This should replace the current design instinct of one custom struct such as
`PentagonField` for each new field.

## First Implementation Tier

The first implementation tier should use only static field types.

Examples:

- `BigRational`
- `Algebraic<TanPiFifth>`

Properties:

- type alone determines the field;
- ordering is straightforward;
- ergonomics and performance are best;
- useful for common named examples and experiments.

This is enough for the current spike and is the default recommendation until
there is pressure for something more general.

## Small Linear Algebra Helpers

Consumers mainly want arithmetic plus a few recurring linear-algebra routines.

That argues for shipping a small helper layer together with the scalars instead
of forcing every caller to rewrite it.

Recommended helpers:

- dot products in fixed small dimension;
- tiny Gaussian elimination over `OrderedField`;
- solve for `2x2`, `3x3`, and `4x4` systems;
- rank for small dense matrices represented as slices or `Vec<Vec<F>>`;
- maybe a tiny `MatrixExt`-style helper module if usage becomes repetitive.

Not recommended in v1:

- a full matrix type;
- a generic decomposition zoo;
- chasing parity with `nalgebra`.

The design target is "enough to write geometry and KKT code comfortably", not
"invent a new linear algebra ecosystem".

## General Constructor

The mathematical core should still be the standard presentation `Q[t]/(p(t))`.

That means the reusable implementation should have an internal constructor that
works from:

- the minimal polynomial `p(t)`;
- an isolating interval for the chosen real root;
- coefficients in the power basis.

For the named-field path above, those inputs come from `StaticFieldSpec`.

If later needed, the same core can be exposed as a runtime constructor such as:

```rust
RealNumberField::new(minimal_polynomial, isolating_interval)
```

with elements of type `RealNumberFieldElem`.

That runtime surface is secondary. The primary consumer-facing path is the
static field-spec pattern.

## Comparison And Ordering

Because we work only with chosen real embeddings, a total order is legitimate.

Recommended policy:

- require `sign()` on all field elements;
- require `cmp_real(&self, &Self)`;
- implement `PartialOrd` and `Ord` for static field types whose embedding is
  fixed by the type.

## Interop With Linear Algebra

Be compatible with `nalgebra`, but do not design around it.

Reason:

- `nalgebra` is useful as a container type;
- the important algorithms here are small, sign-sensitive, and custom;
- bespoke elimination code is easier to audit for proof-driven uses.

So the intended posture is:

- scalar types should be usable inside `nalgebra` matrices when convenient;
- core repo algorithms should continue using repo-owned elimination and rank
  routines;
- `nalgebra` should not drive the trait design.

## Serialization

Canonical coefficient serialization is part of the scalar boundary.

For static fields:

- `BigRational` serializes as one coefficient;
- `Algebraic<TanPiFifth>` serializes as four coefficients in basis
  `1, t, t^2, t^3`.

This keeps persisted data deterministic and readable.

## Recommended Crate Layout

If and when this moves out of the experiment crate, it should live as a
companion workspace crate beside `library/`, not nested inside `library/`.

Recommended path:

```text
real-algebraic/
  src/
    lib.rs
    sign.rs
    field.rs
    algebraic.rs
    spec.rs
    rational.rs
    serialize.rs
```

Where:

- `field.rs` defines `OrderedField` and shared helpers;
- `spec.rs` defines `StaticFieldSpec`;
- `algebraic.rs` defines `Algebraic<S>` and the shared `Q[t]/(p)` machinery;
- `rational.rs` reexports or aliases `BigRational` support;
- named fields such as `TanPiFifth` can live either in `lib.rs` or a small
  `named_fields.rs` module if they multiply;
- `serialize.rs` holds field metadata and canonical coefficient helpers.

## What To Implement First

Implementation order should be:

1. rename the current experiment trait surface toward `OrderedField` and
   `Sign`;
2. keep `BigRational` as the rational backend;
3. introduce `StaticFieldSpec` plus `Algebraic<S>`;
4. port the pentagon example to a `TanPiFifth` field spec and
   `type TanPiFifthField = Algebraic<TanPiFifth>`;
5. add the convenience constructors, predicates, and helpers that ordinary
   callers reach for immediately;
6. add borrowed-operand arithmetic where profiling or ergonomics show pressure;
7. if duplication appears, move the tiny linear-algebra helpers into the same
   crate beside the scalars.

Do not start by exposing a large runtime API unless a second nontrivial field
really needs it.

## Observable Design Rules

Agents implementing this API should preserve these rules:

- arithmetic results stay inside the same field;
- sign and comparison never use floating-point approximations for correctness;
- `to_f64()` is never used for branching in field logic;
- serialization uses canonical basis coefficients only;
- convenience helpers should make common code shorter, not introduce a parallel
  abstraction hierarchy;
- linear algebra code is allowed to stay repo-owned and specialized.

## Tests

Unit tests and smoke tests are required from the start.

### Unit Tests

Every scalar backend should have unit tests for:

- additive and multiplicative identities;
- additive inverse and multiplicative inverse for nonzero elements;
- canonicalization after arithmetic;
- equality of different representations of the same value;
- sign classification for negative, zero, and positive elements;
- comparison ordering on representative examples;
- `to_f64()` sanity on a few known values;
- serialization roundtrips for canonical coefficient vectors.

For algebraic fields specifically:

- the generator satisfies its defining polynomial;
- multiplication reduces back to the canonical basis;
- one or two known closed-form identities are checked directly.
- at least one field-spec-defined type alias such as `TanPiFifthField` works
  through the same generic implementation as any future named field.

### Linear Algebra Unit Tests

Small linear-algebra helpers should have unit tests for:

- solving tiny systems with known answers;
- singular-system detection;
- rank on full-rank and rank-deficient examples;
- rational control examples where the expected answer is easy to inspect.

### Smoke Tests

Smoke tests should cover the intended user workflows, not just local algebra.

Recommended smoke tests:

- construct a rational control polytope and run one exact solve path;
- construct the pentagon field example and run one selected KKT solve;
- verify one generated algebraic value against a known `f64` reference;
- if a standalone experiment binary exists, run it in a smoke configuration and
  check that it finishes and writes the expected output files.

The point of the smoke tests is to catch "the pieces compile but do not work
together" failures early.

## Benchmarks And Profiling

Benchmarks are also part of the initial rollout.

The question is not "is this fast in the abstract?" but "what is the cost
relative to the scalar types we already know how to reason about?"

In particular, we want early order-of-magnitude answers such as:

- how much slower is `BigRational` than `f64` on the relevant kernels;
- how much slower is `Algebraic<TanPiFifth>` than `BigRational`;
- whether the algebraic layer looks more like `10^2`, `10^3`, or `10^4`
  overhead relative to `f64` for our actual workloads.

### Benchmark Scope

Start with a small benchmark set that matches how the code is actually used:

- scalar addition, subtraction, multiplication, division;
- sign and comparison;
- dot products in dimensions `2`, `4`, and maybe `8`;
- tiny Gaussian elimination or `4x4` solve;
- one selected KKT build/solve for a rational control;
- one selected KKT build/solve for the pentagon-field HKO example.

Do not start with synthetic benchmarks that have no obvious connection to the
geometry or KKT code.

### Baselines

Use these baselines:

- `f64`
- `BigRational`
- `Algebraic<TanPiFifth>`

The first useful result is a simple table of slowdown factors, not a deep
performance report.

### Benchmark Style

Recommended style:

- use stable, repeatable inputs;
- keep benchmarks small enough to run regularly during development;
- report both absolute timings and ratios against `f64`;
- separate scalar microbenchmarks from end-to-end workflow benchmarks.

If profiling is needed, use it to explain large slowdowns after the benchmark
ratios are known. Do not start by profiling blindly.

### Smoke Performance Checks

It is also useful to have one or two very cheap performance smoke checks.

Examples:

- a benchmark target that is expected to finish in well under a minute locally;
- a regression check that flags if the `TanPiFifthField` `4x4` solve gets
  dramatically slower after an API or arithmetic change.

The goal is not strict performance gating. The goal is to make cost visible
early, so agents and humans notice when a convenience change multiplies runtime.

## Open Questions

These are still open and can be revised during implementation:

- final crate name: `real-algebraic` vs `algebraic-scalars`;
- whether `cmp_real()` should live on the trait if `Ord` is enough for all
  static types;
- whether named field specs should be written by hand or generated by a small
  helper macro once the pattern stabilizes;
- whether borrowed arithmetic should be implemented manually for hot paths or
  deferred until needed;
- when the scalar layer has enough reuse to justify extracting from
  `experiments/numerics`;
- when a second field is important enough to justify generic runtime field
  support;
- which smoke test entrypoint should be treated as the default integration check
  once the scalar layer is extracted;
- which benchmark set should be treated as the standard local cost dashboard.
