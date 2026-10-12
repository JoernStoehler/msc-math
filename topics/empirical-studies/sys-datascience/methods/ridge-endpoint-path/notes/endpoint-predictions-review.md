# Independent review: frozen endpoint `sys` predictions

## Verdict

**Pass after one proof repair and two convention clarifications.** The forced
endpoint values are valid preflight checks:

```text
4x4 endpoint: c_EHZ = 4 for the canonical cube, sys = 1/2.
3x6 endpoint: c_EHZ = 3/2 in the stated normalization, sys = 3/4.
```

I found no sign, support-function, factor-of-two, affine-scaling, or volume
error. The `3x6` memo currently compresses its essential lower bound into one
sentence. Replace that sentence with the finite total-variation argument below
before treating the derivation as review-grade. This is an exposition/proof
gap, not a wrong prediction.

## 1. Conventions and billiard interface

With `J(x,y)=(-y,x)`, the support function transforms as

```text
h_{J K}(v) = h_K(J^T v) = h_K(-Jv).
```

For `D=P-P`, `h_D(u)=width_P(u)`. Therefore for the standard triangle and
`Q=JD`,

```text
h_Q(v)=width_P(-Jv).
```

This is the correct sign. Replacing `J` by `-J` gives the reflected norm; the
endpoint value is unchanged, but a forced coordinate-level check must use the
same sign in the constructed `Q` and in the support formula.

The capacity convention used by the memo is also correct: the
`Q^circ`-Minkowski length of an increment is `h_Q(v)`, so the polygonal length
is `sum h_Q(delta_i)`. State `J` explicitly in the memo because the earlier
ridge equality permits either verbal “quarter-turn,” while the displayed norm
depends on the literal choice.

## 2. The `3x6` endpoint

Take

```text
P = conv{(0,0),(1,0),(0,1)},
D = P-P,
Q = JD.
```

Then `area(P)=1/2` and `area(Q)=area(D)=3`. For `v=(x,y)`,

```text
-Jv = (y,-x),
width_P(y,-x)
  = max{0,y,-x} - min{0,y,-x}
  = max(|x|,|y|,|x+y|).
```

Thus the stated norm

```text
N(x,y)=max(|x|,|y|,|x+y|)
```

is correct.

### Upper bound

The medial triangle

```text
(1/2,0), (1/2,1/2), (0,1/2)
```

has increments `(0,1/2)`, `(-1/2,0)`, `(1/2,-1/2)`, each of norm `1/2`.
Its length is `3/2`.

It cannot be translated into `int(P)`: a translating vector `(a,b)` would
have to satisfy `a>0`, `b>0`, and `a+b<0`, from the two coordinate facets and
the facet `x+y<1`. Hence the non-translatable-polygon characterization gives
`c_EHZ <= 3/2`.

### Matching lower bound

This is the missing review-grade calculation. Let the vertices of any closed
polygonal curve be `(x_i,y_i)` and set

```text
x_min = min_i x_i,
y_min = min_i y_i,
s_min = min_i (x_i+y_i),
s_max = max_i (x_i+y_i).
```

The curve can be translated into `int(P)` exactly when there are `a,b` with

```text
a > -x_min,
b > -y_min,
a+b < 1-s_max.
```

Such `a,b` exist exactly when

```text
F := s_max - x_min - y_min < 1.
```

Consequently every non-translatable curve has `F>=1`. Translate the curve so
that `x_min=y_min=0`; then `s_max=F`. Put

```text
X=max_i x_i,  Y=max_i y_i,  m=min_i(x_i+y_i),  M=max_i(x_i+y_i)=F.
```

For every increment `(dx,dy)`,

```text
N(dx,dy)
 = (|dx|+|dy|+|dx+dy|)/2.
```

Therefore, writing `TV` for cyclic total variation,

```text
L_N = [TV(x)+TV(y)+TV(x+y)]/2
    >= X + Y + (M-m).
```

Here `X+Y>=M`. A vertex with `x=0` shows `m<=Y`, and a vertex with `y=0`
shows `m<=X`; hence `m<=min(X,Y)<=(X+Y)/2`. It follows that

```text
L_N >= X+Y+M-m
    >= M + (X+Y)/2
    >= 3M/2
    >= 3/2.
```

This proves the lower bound for every closed polygonal curve, not only curves
with at most three vertices. Combined with the medial triangle,

```text
c_EHZ(P x_L J(P-P)) = 3/2.
```

The forced ratio is therefore

```text
sys = (3/2)^2 / [2*(1/2)*3] = 3/4.
```

Exact repair: replace the memo's sentence beginning “The standard
non-translatable-polygon characterization ... forces” with this calculation,
or preserve it as a referenced lemma. Merely naming a “finite-support
calculation” does not expose the only nontrivial inequality.

## 3. Affine/scaling extension of the `3x6` value

The claimed arbitrary-triangle formula is correct. If `P=A P_0+t`, then

```text
P-P=A(P_0-P_0),
A^T J A = det(A) J.
```

The block symplectic map `(q,p)->(A^{-1}q,A^T p)` sends the product to

```text
P_0 x_L det(A) J(P_0-P_0).
```

The billiard length, hence capacity, is multiplied by `|det(A)|`; reflection
handles a negative determinant. Since `area(P)=|det(A)|/2`,

```text
c_EHZ = (3/2)|det(A)| = 3 area(P).
```

Also `area(J(P-P))=6 area(P)`, giving `sys=3/4`. An additional independent
positive scale on `Q` multiplies capacity linearly and `area(Q)` quadratically,
so it cancels from `sys` as claimed.

Repair: write `|det(A)|` when discussing capacity/area scaling. The set-level
sign is harmless, but `det(A)` itself need not be positive.

## 4. The `4x4` equality family

Verified. Every centrally symmetric nondegenerate quadrilateral is a
parallelogram. After translation write `P=AS`, `S=[-1,1]^2`. For `Q=JP`, the
block symplectic map sends

```text
P x_L Q  ->  S x_L A^T J A S
           = S x_L det(A) J S.
```

Because `JS=S` and `S=-S`, this is `S x_L |det(A)|S`. A block symplectic
rescaling makes it a uniform dilation of `S x_L S`, and `sys` is unchanged.
The canonical product is exactly `[-1,1]^4`.

The repo's registered literature fixture has

```text
c_EHZ([-1,1]^4)=4,
volume=16,
sys=16/(2*16)=1/2.
```

Thus `sys=1/2` holds for the whole nondegenerate `4x4` ridge-equality family.
Translations, the choice `J` versus `-J`, and an independent positive scale
on `Q` do not alter the ratio.

Repair: as above, replace `det(A)` by `|det(A)|` after passing from the matrix
identity to scale. No other change is needed.

## 5. Are these endpoints already known?

- **`4x4`: yes, exactly.** The centered square pair is the registered
  hypercube fixture. Its capacity `4` is recorded with the HK2019 Example 4.6
  citation. A target evaluation is a regression/preflight check, not a new
  value or example.

- **The value `3/4`: yes.** The registered standard four-simplex has
  `capacity=1/4`, volume `1/24`, and hence `sys=3/4`. The registered
  crosspolytope also evaluates to `3/4`, although its source annotation says
  its capacity is computed and lacks a literature cross-check. Do not present
  either as new evidence for the value.

- **The specific `3x6` Lagrangian product:** not found as a registered known
  polytope or named repo example. Its 9 facets already rule out linear identity
  with the 5-facet simplex or 16-facet crosspolytope. The memo's cautious claim
  that the realization may be additional while the value is not new is
  accurate. A broader literature novelty claim would require a literature
  search and is not established by this repo audit.

## 6. Preflight recommendation

Use the endpoint predictions as hard smoke assertions after verifying the
constructed geometry:

```text
3x6: R=4sqrt(6), volume=3/2, capacity=3/2, sys=3/4.
4x4 canonical: R=8, volume=16, capacity=4, sys=1/2.
```

These volume/capacity numbers assume exactly the normalizations in the memo;
for rescaled representatives, assert the invariant `sys` values and the
appropriately scaled capacity rather than forcing `3/2` or `4` blindly.
Material endpoint disagreement should stop interpretation, as the memo says.
The likely first checks are the literal `J` sign, factor scales, reconstructed
volume, complete face ordering, and capacity pipeline regression.

The qualitative perturbation predictions are appropriately labeled as
expectations rather than consequences of continuity. No monotonicity claim is
justified before the paths are fixed.

## Review evidence and limits

Review passes performed: direct support-function/rotation derivation; complete
non-translatability and total-variation lower bound; affine and independent
scale audit; hypercube reduction and fixture check; normalization arithmetic;
and repo search for prior endpoint registrations. No subagent was used. No
repository files were changed, and no endpoint target capacity was evaluated.
