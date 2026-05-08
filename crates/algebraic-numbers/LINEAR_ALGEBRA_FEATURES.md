# Exact Linear Algebra Feature Gaps

This note records what is missing after the current scalar-only
`algebraic-numbers` branch, why it is missing, and what implementation
approaches look predictable for future agents. It is planning material, not the
current crate contract.

The goal is a small exact arithmetic and exact linear-algebra crate for
compile-time-known real algebraic fields. "Small" means low friction for future
Rust agents working in crates such as `symplectic/`: standard concepts,
standard Rust/nalgebra shapes where they fit, explicit mathematical outcomes,
and no hidden floating-point tolerances.

## Current Baseline

Already present:

- `BigRational` and `Algebraic<F>` as exact scalar types.
- `ExactScalar` as explicit opt-in for exact scalar code.
- `RealAlgebraicField` for static field markers.
- Exact arithmetic, equality, and ordering for `Algebraic<F>`.
- Ordinary nalgebra container syntax such as `Vector4<Algebraic<Sqrt5>>`.

Not present:

- Matrix algorithms.
- Row reduction, rank, solve, kernel, determinant, inverse.
- Symmetric matrix inertia or definiteness.
- Eigenvalue decomposition.
- Runtime construction of new fields.

## Predictability Rules

Prefer standard mathematical names and standard Rust/nalgebra data shapes.
Future agents should recognize the feature without reading an essay first.

Keep exact algorithms exact. No f64 pivots, rank thresholds, epsilons, or
caller-provided tolerances in this crate.

Return mathematical outcomes explicitly when callers must branch. A linear solve
can have no solution, one solution, or affine-family solutions; collapsing that
into `Option` would hide useful information.

Build features in dependency order. Row reduction comes before rank, solve,
kernel, inverse, and determinant. Symmetric inertia comes after row reduction and
basic solve are settled.

Use tests to pin down the first useful signature before implementation. A good
feature start is: choose the public or private helper signature, write tests that
exercise that signature on `BigRational` and one static algebraic field, then
code until the tests pass or the test exposes a bad API assumption. Property
tests are useful when small examples risk hiding accidental special cases.

Do not shape this crate around KKT before the generic exact linear algebra
exists. KKT-specific code belongs in `symplectic/` unless a generic primitive is
obviously reusable.

Do not implement eigenvalue decomposition by pretending eigenvalues stay in the
same field. A symmetric matrix over `Q[alpha]` can have eigenvalues outside
`Q[alpha]`.

## Missing Feature Stack

### Row Reduction

Status: missing.

Why it matters: row reduction is the reusable primitive for exact rank, solve,
kernel, determinant/invertibility checks, and many certificate-style tests.

Predictable approach: Gaussian elimination over `ExactScalar`, using exact
nonzero checks for pivots. A simple row-echelon form is enough for rank and
forward elimination; reduced row-echelon form is convenient for kernel and
solve output. Start with dense matrices.

Required scalar facts: exact zero and one, negation, addition, subtraction,
multiplication, and division by a known nonzero pivot.

Important output facts:

- reduced rows or echelon rows;
- pivot columns;
- rank;
- row operations only as internal/provenance data if later certificates need
  them.

Rejected first guesses:

- f64-assisted pivoting: breaks exactness.
- Bare `usize` rank only: too little evidence for solve/kernel.
- Full generic linear-algebra framework: too much before the first callers.
- Bareiss or other coefficient-growth controls as the first implementation:
  plausible later, but start simple and measure real coefficient swell first.

### Rank

Status: missing.

Why it matters: rank is used in exact geometry, symmetry checks, degeneracy
checks, and many "kernel has expected dimension" arguments.

Predictable approach: thin wrapper over row reduction. For dense matrices, row
rank is enough; column-rank naming should not create a second algorithm.

### Linear Solve

Status: missing.

Why it matters: exact KKT, affine constraints, vertex reconstruction, and
certificate code need exact solutions to `A x = b`.

Predictable approach: augment `A` with `b`, row-reduce, and return an explicit
outcome:

- inconsistent;
- unique solution;
- affine solution family.

For affine families, the useful mathematical object is one particular solution
plus a kernel basis. Avoid returning a single arbitrary vector when the system
is underdetermined.

Multiple right-hand sides are not required for the first solve API. Leave room
for `A X = B` later, because inverse and batched constraint solves can share
that shape.

Rejected first guesses:

- `Option<Vec<T>>`: cannot distinguish inconsistent from non-unique.
- panic on singular/non-square matrices: those are ordinary mathematical
  outcomes, not programmer bugs.
- KKT-only solve first: too specialized and would obscure the linear algebra
  primitive.

### Kernel / Nullspace Basis

Status: missing.

Why it matters: kernel dimensions and bases appear in symmetry and tangent-space
checks. A kernel basis is also the natural representation of underdetermined
solve output.

Predictable approach: compute from reduced row-echelon form. Use free variables
as basis directions in the standard way. The result should be exact vectors.

Naming note: `kernel_basis` is more mathematical in this repo than
`nullspace`, but both terms are standard. Pick one public name and mention the
other in docs for grepability.

### Determinant, Invertibility, Matrix Inverse

Status: missing.

Why it matters: determinant and inverse are common exact-linear-algebra
queries, but most near-term callers can use rank/solve instead.

Predictable approach: derive from elimination. Determinant for square matrices
needs row-swap parity and pivot product. Matrix inverse is solving against the
identity matrix.

Implementation warning: exact elimination can make rational/algebraic
coefficients large. Keep the first version simple, but keep the elimination code
localized so a later measured coefficient-growth fix does not rewrite every
caller.

Priority: lower than rank/solve/kernel.

### Symmetric Definiteness And Inertia

Status: missing.

Why it matters: exact substitutes for numerical eigenvalue classification often
need to know whether a symmetric matrix is positive/negative definite,
semidefinite, or indefinite. KKT-style code may need inertia rather than actual
eigenvectors.

Predictable approach: exact symmetric elimination / LDL^T-style reasoning or
Sylvester-style criteria where applicable. The output can be inertia counts
`(positive, zero, negative)` or a definiteness enum derived from those counts.

Important boundary: this should decide signs exactly using `Ord`, not estimate
eigenvalues.

Open design question: the algorithm choice is nontrivial. LDL^T-style reasoning
needs care around zero or indefinite pivots, while principal-minor criteria can
be expensive for semidefinite classification. Do not add a broad public API
until the needed outcome shape and algorithm witness are clear.

### Eigenvalue Decomposition

Status: intentionally deferred.

Why it matters: numerical KKT solvers use symmetric eigen decomposition, and
agents may naturally look for an exact replacement.

Why it is not a simple feature: eigenvalues of a matrix over `Q[alpha]` need not
lie in `Q[alpha]`. Exact eigendecomposition can require new algebraic field
extensions, characteristic-polynomial factorization, and root selection. That
conflicts with this crate's current fixed-field contract.

Predictable substitute for many callers: rank, kernel, solve, and inertia.

When to revisit: only if a concrete proof or algorithm really needs exact
eigenvectors/eigenvalues rather than inertia or kernel/rank data. At that point
the design likely needs a separate field-extension story.

## Near-Term Feature Order

1. Dense row reduction over `ExactScalar`.
2. Rank from row reduction.
3. Solve `A x = b` with explicit outcomes.
4. Kernel basis from reduced row-echelon form.
5. Determinant/inverse as thin wrappers if a caller asks.
6. Symmetric inertia/definiteness after the basic row-reduction layer is stable.
7. Keep eigendecomposition deferred.

## Evidence To Add With Features

Each feature should include at least:

- `BigRational` tests;
- one `Algebraic<Sqrt5>` test;
- a singular/underdetermined case if the feature has such outcomes;
- a test that would fail under f64 thresholding when practical;
- one nalgebra-container use if the public surface uses nalgebra types.

Tests should witness mathematical outcomes, not implementation internals.
