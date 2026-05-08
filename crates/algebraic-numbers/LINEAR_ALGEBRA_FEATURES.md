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

Return mathematical outcomes explicitly when callers must branch. For linear
solve, the predictable first shape is either inconsistent or consistent with one
particular solution and a kernel basis. An empty kernel basis means the solution
is unique.

Build features in dependency order. Row reduction comes before rank, solve,
kernel, inverse, and determinant. Negative-definite checks come after row
reduction and basic solve are settled.

Use tests to pin down the first useful signature before implementation. A good
feature start is: choose the public or private helper signature, write tests that
exercise that signature on `BigRational` and one static algebraic field, then
code until the tests pass or the test exposes a bad API assumption. Property
tests are useful when small examples risk hiding accidental special cases.

Do not shape this crate around KKT before the generic exact linear algebra
exists. KKT-specific code belongs in `symplectic/` unless a generic primitive is
obviously reusable.

Avoid eigendecomposition for now. Exact eigendecomposition needs a field
extension story, because a symmetric matrix over `Q[alpha]` can have eigenvalues
outside `Q[alpha]`.

## Decision States

Use these labels when refining this note. "Not fixed yet" is too ambiguous.

- Preferred, needs witness: the current best choice is clear enough to test, but
  should not become API contract before a compile-time or runtime witness proves
  it is ergonomic.
- Defer until observed: a possible problem or optimization is real in principle,
  but not worth designing for before a caller, large example, or failing test
  shows it matters.
- Decide with first implementation: several simple choices are plausible, and
  the cheapest reliable tie-breaker is the first TDD pass rather than more
  planning.

## Missing Feature Stack

### Row Reduction

Status: missing.

Why it matters: row reduction is the reusable primitive for exact rank, solve,
kernel, determinant/invertibility checks, and many certificate-style tests.

Predictable approach: Gaussian elimination over `ExactScalar`, using exact
nonzero checks for pivots. A simple row-echelon form is enough for rank and
forward elimination; reduced row-echelon form is convenient for kernel and
solve output. Start with dense matrices.

Decision state: decide with first implementation. The algorithm family is fixed;
the exact helper names and result structs should come from the first tests.

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
- Bareiss or other coefficient-growth controls now: premature until real
  examples produce coefficients large enough to matter.

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

Decision state: preferred, needs witness.

Predictable approach: augment `A` with `b`, row-reduce, and return an explicit
outcome:

- inconsistent;
- consistent with one particular solution and a kernel basis.

An empty kernel basis means the solution is unique. A non-empty kernel basis
means the solution set is affine. Avoid returning a single arbitrary vector when
the system is underdetermined.

Multiple right-hand sides are not required for the first solve API. Leave room
for `A X = B` later, because inverse and batched constraint solves can share
that shape.

Rejected first guesses:

- `Option<Vec<T>>`: cannot represent underdetermined solution sets.
- separate `Unique` and `Affine` variants: plausible, but likely gives callers
  one more case than they need; use an empty kernel basis to represent unique
  solutions unless a real caller wants to branch on uniqueness.
- panic on singular/non-square matrices: those are ordinary mathematical
  outcomes, not programmer bugs.
- KKT-only solve first: too specialized and would obscure the linear algebra
  primitive.

Matrix shape: prefer nalgebra's dynamic `DMatrix<T>` and `DVector<T>` for the
first public algorithm surface. `symplectic/` already uses those for dynamic
linear systems, while `Vector4<T>` remains useful for fixed-dimensional geometry.
Do not add parallel `Vec<Vec<T>>` APIs unless nalgebra ownership or conversion
friction is observed in tests.

Decision state for matrix shape: preferred, needs witness.

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

Deferred performance note: exact elimination can make rational/algebraic
coefficients large. Do not optimize for this before observing a real problem;
keep the elimination code localized so a later measured fix does not rewrite
every caller.

Decision state for coefficient growth: defer until observed.

Priority: lower than rank/solve/kernel.

### Symmetric Definiteness And Inertia

Status: missing.

Why it matters: exact substitutes for numerical eigenvalue classification often
need to know whether a symmetric matrix is positive/negative definite,
semidefinite, or indefinite. KKT-style code may need inertia rather than actual
eigenvectors. The projection approach and related experiments need exact
negative-definite checks, so `is_negative_definite` does not violate YAGNI.

Predictable approach: exact symmetric elimination / LDL^T-style reasoning or
Sylvester-style criteria where applicable. The output can be inertia counts
`(positive, zero, negative)` or a definiteness enum derived from those counts.

Important boundary: this should decide signs exactly using `Ord`, not estimate
eigenvalues.

Open design question: the algorithm choice is nontrivial. LDL^T-style reasoning
needs care around zero or indefinite pivots, while principal-minor criteria can
be expensive for semidefinite classification. Do not add a broad public API
until the needed outcome shape and algorithm witness are clear.

API pressure: a single `is_negative_definite` predicate may be enough for the
first caller. If computing full inertia is the simpler internal route, expose
only the part callers need unless the extra result removes complexity.

Decision state: preferred public need, unresolved internal algorithm. The public
need is exact `is_negative_definite`; the algorithm should be chosen by the first
implementation proof/tests, not by chat preference.

### Eigenvalue Decomposition

Status: intentionally deferred.

Decision state: defer until a concrete caller proves rank, kernel, solve, and
negative-definite checks are not enough.

Why it matters: numerical KKT solvers use symmetric eigen decomposition, so
agents may naturally look for an exact replacement.

Why it is not a simple feature: eigenvalues of a matrix over `Q[alpha]` need not
lie in `Q[alpha]`. Exact eigendecomposition can require new algebraic field
extensions, characteristic-polynomial factorization, and root selection. That
conflicts with this crate's current fixed-field contract.

Predictable substitute for current callers: rank, kernel, solve, and negative
definite / inertia checks.

When to revisit: only if a concrete proof or algorithm really needs exact
eigenvectors/eigenvalues rather than inertia or kernel/rank data. At that point
the design likely needs a separate field-extension story.

## Near-Term Feature Order

1. Dense row reduction over `ExactScalar`.
2. Rank from row reduction.
3. Solve `A x = b` with explicit outcomes.
4. Kernel basis from reduced row-echelon form.
5. Negative-definite checks, possibly via inertia, after the basic
   row-reduction layer is stable.
6. Determinant/inverse as thin wrappers if a caller asks.
7. Keep eigendecomposition deferred.

## Evidence To Add With Features

Each feature should include at least:

- `BigRational` tests;
- one `Algebraic<Sqrt5>` test;
- a singular/underdetermined case if the feature has such outcomes;
- property tests for row reduction, rank, solve, and kernel when the invariant
  is easy to state;
- a test that would fail under f64 thresholding when practical;
- one nalgebra-container use if the public surface uses nalgebra types.

Tests should witness mathematical outcomes, not implementation internals.
