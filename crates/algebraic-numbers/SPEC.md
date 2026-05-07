# algebraic-numbers Spec

Small Rust 2021 crate for exact arithmetic in ordered real algebraic extensions
of `Q`, with only the tiny linear algebra and canonical serialization needed by
exact validation paths. This is not a CAS: no runtime fields, towers,
factorization API, symbolic simplifier, or public polynomial API.

Dependencies: `serde`, `num-bigint`, `num-rational`, `num-traits`. Dev:
`criterion`, `proptest`, `serde_json`. Modules: `algebraic`, `field`,
`linear`, `named_fields`, `serialize`, `sign`, `spec`.

Root API:

```rust
pub type Rational = BigRational;
pub use algebraic::Algebraic;
pub use field::{cmp_field, dot, max_field, min_field, OrderedField};
pub use linear::{rank_rows, solve_square, SolveResult};
pub use named_fields::TanPiFifth;
pub use serialize::{canonical_element, CanonicalElement};
pub use sign::Sign;
pub use spec::{validate_field_spec, FieldSpecError, StaticFieldSpec};
```

## Field Specs

Fields are compile-time marker types, not runtime values:

```rust
pub trait StaticFieldSpec: 'static {
    fn name() -> &'static str;
    fn generator_name() -> &'static str;
    fn minimal_polynomial() -> Vec<BigRational>; // ascending coefficients
    fn isolating_interval() -> (BigRational, BigRational); // chosen real root
}
```

Semantics: `Q[t]/(p(t))`, where `p` is the caller-supplied irreducible minimal
polynomial and the interval isolates the chosen real root. `name()` is used for
diagnostics/serialization; `generator_name()` is used for basis labels.
`validate_field_spec` trims trailing zeros, rejects zero/constant polynomials,
rejects `lo >= hi`, rejects root endpoints, and uses a rational Sturm sequence
to require exactly one real root in `(lo, hi)`. It does not prove
irreducibility.

```rust
pub fn validate_field_spec<S: StaticFieldSpec>() -> Result<(), FieldSpecError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldSpecError {
    ZeroPolynomial,
    ConstantPolynomial,
    InvalidIntervalOrder,
    LowerEndpointIsRoot,
    UpperEndpointIsRoot,
    IntervalDoesNotIsolateUniqueRoot { root_count: usize },
}
```

Element constructors may cache validation by `TypeId`; they must panic on an
invalid static spec or poisoned cache because those are invariant failures.

Ship `TanPiFifth`: `name = "Q(tan(pi/5))"`, `generator_name = "t"`,
`minimal_polynomial = [5, 0, -10, 0, 1]`, `isolating_interval = (1/2, 1)`,
and derives `Clone`, `Copy`, `Debug`.

## Scalars

`Algebraic<S>` stores canonical power-basis coefficients
`1, t, ..., t^(d-1)` as `Vec<BigRational>` plus `PhantomData<S>`.

```rust
pub struct Algebraic<S: StaticFieldSpec> { coeffs: Vec<BigRational>, ... }
impl<S: StaticFieldSpec> Algebraic<S> {
    pub fn from_coeffs(coeffs: Vec<BigRational>) -> Self;
    pub fn generator() -> Self;
    pub fn coeffs(&self) -> &[BigRational];
}
```

Canonicalization: normalize the modulus to monic form; trim trailing zeros but
store exactly `degree(p)` coefficients; reduce overlong inputs modulo `p`;
`generator()` is `t`; basis labels are `["1", symbol, "symbol^2", ...]`;
equality is coefficient equality after reduction.

Arithmetic: coefficientwise add/sub/neg, convolution plus reduction for mul,
division via polynomial extended gcd inverse, panic on zero divisor. Implement
`Clone`, `Debug` with field name and coeffs, `PartialEq`, `Eq`, `PartialOrd`,
`Ord`, and owned plus borrowed `Add/Sub/Mul/Div/Neg` combinations for
formula-style code.

## Ordered Field

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign { Negative, Zero, Positive }

pub trait OrderedField:
    Clone + Debug + PartialEq + Eq + Add<Output=Self> + Sub<Output=Self>
    + Mul<Output=Self> + Div<Output=Self> + Neg<Output=Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn from_rational(value: BigRational) -> Self;
    fn field_name() -> &'static str;
    fn basis_labels() -> Vec<String>;
    fn sign(&self) -> Sign;
    fn cmp_real(&self, other: &Self) -> Ordering;
    fn to_f64(&self) -> f64;
    fn canonical_coeffs(&self) -> Vec<BigRational>;
    fn from_i64(value: i64) -> Self;
    fn from_frac(numer: i64, denom: i64) -> Self;
    fn is_zero(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
}
```

Implement for `BigRational` with field `"Q"`, basis `["1"]`, exact rational
sign, one canonical coefficient, and `to_f64 = numer.to_f64().unwrap_or(NaN) /
denom.to_f64().unwrap_or(1.0)`. Implement for `Algebraic<S>` in the chosen real
embedding. Sign asserts the isolating endpoints are nonroots and bracket the
selected root, then uses interval Horner evaluation, bisection refinement, and a
conservative bit-height budget computed from element coefficients, modulus
coefficients, and interval endpoints as `64 * max_bits.max(1) + 64`. Return
`Zero` if the root is hit exactly; panic if the budget is exhausted. `Ord` and
`cmp_real` compare by the sign of the difference. `to_f64()` is diagnostics
only: 256 bisections of the selected root, coefficient power-sum evaluation,
and `ToPrimitive`, yielding `NaN` on conversion failure.

Helpers: `cmp_field`, `min_field`, `max_field`, and fixed-size `dot`.

## Linear Algebra

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SolveResult<T> { Unique(T), Singular }
pub fn solve_square<const N: usize, F: OrderedField>(
    matrix: &[[F; N]; N],
    rhs: &[F; N],
) -> SolveResult<[F; N]>;
pub fn rank_rows<F: OrderedField>(rows: &[Vec<F>]) -> usize;
```

Use exact Gaussian elimination with row swaps. `solve_square` returns
`Singular` when no nonzero pivot exists, otherwise back-substitutes.
`rank_rows` copies rectangular rows, row-reduces across all columns, and returns
`0` for empty input.

## Serialization

Do not serialize `Algebraic<S>` directly. Persist this stable schema:

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalElement {
    pub field_name: String,
    pub basis_labels: Vec<String>,
    pub coeffs: Vec<BigRational>, // custom JSON
}
pub fn canonical_element<F: OrderedField>(value: &F) -> CanonicalElement;
```

Each rational coefficient is `{"numer":"...","denom":"..."}` with base-10
integer strings. For `3/7 in Q`:
`{"field_name":"Q","basis_labels":["1"],"coeffs":[{"numer":"3","denom":"7"}]}`.
Schema changes require migration for existing JSONL consumers.

## Parity Checks

Tests must cover consumer-defined `Q(sqrt(2))`, borrowed operators, field laws,
division by self for nonzero values, `TanPiFifth` defining-polynomial/HKO
expressions, non-monic trailing-zero moduli, invalid specs and constructor
panics, exact signs near `sqrt(2)` convergents including about 700 denominator
bits, canonical coefficients, real ordering, rational/algebraic `dot`,
`solve_square`, singular detection, row swaps, `rank_rows`, serialization JSON
shape and round trip, and rational 2x2 residual property tests.

Benches compare `f64`, `BigRational`, and `TanPiFifthField` for add, sub, mul,
div, sign, dot4, and 2x2 solve.

Formal-reference gaps remain for field-spec validation, Sturm root counting,
Bezout invertibility, sign determination, and root approximation. Do not weaken
exact semantics to match the current proof state.
