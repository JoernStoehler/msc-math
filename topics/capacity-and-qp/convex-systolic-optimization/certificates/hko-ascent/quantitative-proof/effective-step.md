# An explicit, deliberately tiny improving parameter

29 October 2026. Independently audited proof; see `independent-audit.md`.
Root approved integration after that audit. This strengthens effective existence,
not the size of the counterexample. The proposed certified parameter is
`t*=10^-30`, with relative improvement greater than `t*/200=5·10^-33`.

## Fixed inputs and conventions

Use the explicit twenty-row family in `../hko-ascent-main.tex`. Its rows are
`a_r+tD_r`, where each of the ten original normals appears twice at `t=0`;
one copy has `D_r=0`, the other has the stated cut velocity. A simple word in
this twenty-row presentation has at most two occurrences of each base normal.

Write `q(t)=1/(2 c_EHZ(H_t))`, `q0=tau/5`, and `V0=Vol(H)`. Monotonicity gives
`q(t)>=q0`, since `H_t subset H`. The HK formula supplies a maximizing simple
word on the twenty-row bank. Zero weights and unused labels are permitted.
We use the literal quadratic action `Q=1/2 beta^T H beta`, with off-diagonal
entries of `H` the ordered symplectic pairings and zero diagonal.

The already audited finite base-response calculation gives, for every
maximizing base word and every feasible smooth continuation using these row
velocities,

```
g*=2 q0/125                                      (1)
```

as an upper bound on its action derivative. Indeed the midpoint identity
gives `Q'=2q0 sum beta_r D_r.x_mid`; each assigned `D_r.x` is at most the
radial hinge on that labeled facet, whose all-repeated-word response supremum
is `1/125`. This uses the proved repeated-word reduction, not an incomplete
bank. Only this upper bound is needed below.

Exact six-word arithmetic also gives the minimum strictly positive loss
between `q0` and a nonwinning triangle-pair action:

```
Delta = (-tau+3 tau^3)/20 > 1/50.                 (2)
```

This is checked over all 3000 primitive cyclic six-orders, the same primitive
space used in the complete catalogue. Duplicate occurrence choices do not
change these primitive action values. Also `q0>1/10`.

## 1. Uniform positive pivot for any moving maximizer

The companion `minor-bound.md` proves the following bounds for a feasible
moving-row weight vector with `Q_t>=q0/2` and `0<=t<=10^-3`:

* five augmented base columns `B_I(0)=(a_r,1)` have selected occurrence
  weights at least `1/4500`;
* `||B_I(0)^-1||_2<66`, hence
  `||B_I(0)^-1||_infinity<200=:M`.

For clarity, the residual bound must use the mixed old/new bank correctly:
`sum beta a = -t sum beta D`, so its Euclidean norm is at most `(5/4)t`.
Factoring a common `(1-3t/5)` out of all q- or p-weights is invalid because
the retained rows have zero velocity. The direct residual estimate suffices.

The proof uses `|a_r|<5/4`, `|D_r|<5/4` (allowing zero velocity), a lower
bound on both factor masses from the positive action, and the top three
aggregated pentagon-direction weights. Two distinct q normals and three
distinct p normals give an augmented determinant greater than `1/2`.
Hadamard's cofactor bound then gives the stated inverse bound. This argument
is independent of the order of the twenty rows.

Take a maximizer `beta_t` at any `0<t<=10^-30`, and choose such a pivot.
Keep all other weights `z` fixed, and solve the five pivot weights through

```
beta_I(s)=B_I(s)^-1 [e-B_J(s)z],  0<=s<=t,
e=(0,0,0,0,1)^T.                                (3)
```

Put `beta0=beta(0)`. Since moving closure gives
`B(0) beta_t=-t D beta_t+e`,

```
beta0_I-beta_t,I = t B_I(0)^-1 D beta_t.
```

Here the last row of `D` is zero. The right side has infinity norm at most
`2Mt`, using the looser bound `|D_r|<2`. Thus every pivot weight of `beta0`
is greater than `1/4500-400·10^-30>10^-4=:c`. The other coordinates are
unchanged and nonnegative. Therefore `beta0` is a base-feasible probability
vector on the same word.

## 2. Explicit Taylor and Lipschitz bounds on that chart

These estimates hold more generally for `0<=s<=10^-8`. The infinity norm
of the five-column perturbation matrix is at most ten. The Neumann bound
therefore gives `||B_I(s)^-1||_infinity<=2M`. The exact difference formula

```
beta_I(s)-beta0_I = -s B_I(s)^-1 D beta0
```

gives `||beta(s)||_1<=1+20Ms<2`. Differentiating the constraint equations
gives

```
beta'_I=-B_I(s)^-1 D beta(s),
beta''_I=-2 B_I(s)^-1 D_I beta'_I.
```

Consequently

```
||beta'||_1 <= 40M,
||beta''||_1 <= 1600M^2.                         (4)
```

For instance, `||beta'_I||_infinity<=8M`; multiplying this by
`2·(2M)·10` bounds `||beta''_I||_infinity` by `320M^2`, and there are five
coordinates. No positivity of intermediate chart weights is needed for
these norm estimates.

Set `F(s)=Q_s(beta(s))`. On this interval each moving normal has norm below
two, so the entries of the symmetric objective matrix satisfy

```
|H(s)_ij|<=4,   |H'(s)_ij|<=8,   |H''(s)_ij|<=8.
```

Differentiating `F=beta^T H beta/2` twice and using (4) gives

```
|F''| <= 4[(40M)^2+2·1600M^2]
          +2·8·(40M)·2 + (1/2)·8·2^2
       =19200M^2+1280M+16
       < 2·10^5 M^2.
```

Thus, with `R=10^5 M^2=4·10^9`,

```
q(t)=F(t) <= Q0(beta0)+t g(beta0)+R t^2.         (5)
```

Here `g` denotes the derivative at zero of the same algebraic chart,
considered as a quadratic function of any base-feasible probability vector
on this word and using the fixed selected pivot. Explicitly,

```
g(beta)= (1/2) beta^T H'(0) beta
          + (H(0) beta).u(beta),
u_I(beta)=-B_I(0)^-1 D beta,  u_J=0.
```

For any two probability vectors `beta,beta*`,

```
|g(beta)| <= 4+40M <= 44M,
|g(beta)-g(beta*)| <= (8+80M)||beta-beta*||_1
                  <= 88M ||beta-beta*||_1.      (6)
```

The second inequality uses `||u(beta)||_1<=10M`,
`||u(beta)-u(beta*)||_1<=10M||beta-beta*||_1`, and the bound four on every
entry of `H(0)`. The direct quadratic term contributes at most
`8||beta-beta*||_1`.

Put `d=q0-Q0(beta0)>=0`. Since `q(t)>=q0`, equations (5)--(6) imply

```
d <= 44Mt+Rt^2 < 10^-20.                        (7)
```

This uses `t<=10^-30`, `M=200`, and `R=4·10^9`. In particular
`Q0(beta0)>q0/2`.

## 3. Quantitative rounding to a maximizing word in the same chart

Let `s` be the total q-factor mass of `beta0`, and let `u,v` be its
normalized factor weights. The occurrence-wise closure polytopes have at
most forty vertices in each factor: every vertex uses one of five positive
base triangles and at most two choices for each of its three occurrences.
This count remains valid for subwords by taking a face of the full closure
polytope.

Choose convex decompositions `u=sum_i lambda_i u_i`,
`v=sum_j gamma_j v_j`, using at most forty vertices per side. Let `q_ij` be
the action at total factor mass one half for that triangle pair in the given
order. Then

```
Q0(beta0)=4s(1-s) S,       S=sum_ij lambda_i gamma_j q_ij,
d=4q0(s-1/2)^2+4s(1-s)(q0-S).                   (8)
```

All `q_ij<=q0`; each strict loss is at least `Delta`. Since
`Q0(beta0)>q0/2`, one has `4s(1-s)>1/2`. Let `epsilon` be the total
product mass of nonwinning pairs. Equation (8) gives

```
epsilon <= (q0-S)/Delta <= 2d/Delta,
2|s-1/2| <= sqrt(d/q0).                         (9)
```

If `epsilon>0`, delete from each marginal all vertices of mass less than
`2 sqrt(epsilon)` and renormalize what remains. At most forty vertices are
deleted per side, so the removed marginal mass is at most
`80 sqrt(epsilon)<1`, by (7). Any surviving bad pair would have product mass
at least `4 epsilon`, exceeding the total bad-pair mass `epsilon`.
Therefore all surviving pairs win. If `epsilon=0`, simply keep the positive
supports. The two marginal L1 changes are at most `160 sqrt(epsilon)`.

Let `u*,v*` be the resulting mixtures and set
`beta*=(u*/2,v*/2)` on the same ordered occurrences. Every surviving pair is
winning, so `Q0(beta*)=q0`. The triangle vertices have L1 norm one; hence

```
||beta0-beta*||_1
 <= 2|s-1/2|+(1/2)||u-u*||_1+(1/2)||v-v*||_1
 <= [1/sqrt(q0)+160 sqrt(2/Delta)] sqrt(d)
 < 2000 sqrt(d).                               (10)
```

By (7), this distance is less than `2·10^-7<c/2`. Thus every selected pivot
weight of `beta*` remains positive. The fixed-pivot algebraic chart through
`beta*` is consequently a feasible smooth continuation for sufficiently
small positive parameters. Its derivative is bounded by (1), so combining
(6) and (10) yields

```
g(beta0) <= g*+A sqrt(d),
A=88M·2000=35,200,000.                          (11)
```

This is the only use of response maximization. It applies at an exactly
maximizing base word after a quantified rounding, rather than assuming that
a nearly maximizing word already has the same response bound.

## 4. A global action upper bound and the actual improvement

Insert (11) into (5), then maximize `-d+At sqrt(d)` over `d>=0`. Completing
the square gives

```
q(t) <= q0+g*t+(A^2/4+R)t^2
     <= q0+(2q0/125)t+10^15 t^2,
                     0<t<=10^-30.              (12)
```

Indeed `A^2/4+R=309,764,000,000,000<10^15`. This is an upper bound for the
TRUE optimal action, hence a lower bound on the true capacity. No feasible
branch or sampled bank is substituted for that global bound.

For volume, write the radial volume formula using normalized cone-volume
on the old facets. At `x` on an old facet, the gauge of `H_t` is at least
`1+t f_i(x)` because both the old and its split row are present. The other
rows can only increase this gauge. Since `0<=f_i<=2/5`, Taylor's theorem for
`(1+u)^-4`, whose second derivative is at most twenty for `u>=0`, gives

```
V(t)/V0 <= integral (1+t f)^-4 dnu
        <= 1-4 nu[f]t+(8/5)t^2.                 (13)
```

This is an upper bound in the needed direction and requires no new normal-
fan or vertex enumeration. Set `a=2/125`, `v=4nu[f]`, and
`eta=v-2a=(20+64sqrt(5))/9375>1/100`. By (12), `q0>1/10` gives

```
(q(t)/q0)^2 <= (1+a t+10^16 t^2)^2
            <= 1+2a t+3·10^16 t^2.
```

Multiplying this nonnegative upper bound by the positive right side of
(13), and discarding negative cross terms, yields

```
q(t)^2 V(t)/(q0^2 V0)
 <= 1-eta t+4·10^16 t^2
 < 1-t/200,                 0<t<=10^-30.        (14)
```

The elementary coefficient estimates use `a<1`, `t<=10^-30`, and
`4·10^16 t<1/200`. Since `sys=1/(8Vq^2)`,

```
sys(H_t)/sys(H) > 1/(1-t/200) > 1+t/200.
```

In particular `t=10^-30` is explicitly improving, with relative gain greater
than `5·10^-33`. This bound is intentionally tiny. It is not a numerically
meaningful improvement of the ratio, and it does not establish where along
the family the ratio reaches a useful larger maximum.

## Reusable content and limitations

The useful method is the explicit quadratic upper remainder for the global
optimal action: a finite primitive action gap gives a square-root distance
to an optimizer via the winning-pair graph; a uniform positive constraint
minor makes the sensitivity chart quantitative; completing the square absorbs
the near-optimality loss. The constants here are deliberately conservative.
They depend on the two-copy finite bank and product base geometry, not on
enumerating all twenty-row permutations.

The independent audit approved all inequalities and the corrected mixed
old/new pivot lemma. The frozen v1 replay package remains unchanged; the
quantitative result and its exact checks are included in the v2 supplement.
