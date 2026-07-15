# Historical local two-bounce difference-body derivation

Status: historical pre-merge derivation only; it is not current mathematical
authority.  The stronger merged proof is
`formal/product-two-bounce-class.tex`, at agent-reviewed, not Jörn-reviewed,
status.  This file remains as the local provenance for the frozen formula and
the exact implementation's finite reduction.  Its retained-table equality is
bounded implementation validation, not an exhaustive check or a substitute for
the formal proof.

Let `P` be the full-dimensional convex polygon in the `q`-plane and `Q` the
full-dimensional convex polygon in the `p`-plane.  Coordinates in the product
are `(q1,q2,p1,p2)`.  Put

```text
D_P = P-P,        D_Q = Q-Q.
```

The repository's billiard convention assigns an increment `d` the length
`h_Q(d)`.  A closed two-vertex polygon has increments `d,-d`, hence length

```text
h_Q(d)+h_Q(-d) = h_{D_Q}(d).
```

The two-point set `{x,x+d}` is translatable into `int(P)` exactly when
`d in int(P-P)`: one implication follows by subtracting the two translated
points, and the converse follows from an interior representation
`d=p_1-p_0` by translating `x` to `p_0`.  Positive homogeneity then puts the
shortest non-translatable two-point set on `boundary(D_P)`.  Therefore the
two-bounce class action is

```text
A2(P,Q) = min_{d in boundary(D_P)} h_{D_Q}(d).              (1)
```

Equivalently, since `h_{D_Q}` is the gauge of `(D_Q)^polar`,

```text
A2(P,Q) = max { r >= 0 : r (D_Q)^polar is contained in D_P }.  (2)
```

Thus `A2` is a geometry-only difference-body inradius.  It uses neither a
capacity target nor the billiard word/KKT search.  Translation invariance is
immediate.  Scaling `P` or `Q` scales the value in the corresponding factor,
as required by the product capacity convention.

For polygons, (1) is an exact finite optimization.  On each edge
`d(t)=(1-t)a+tb` of `D_P`,

```text
h_{D_Q}(d(t)) = max_{z in vertices(D_Q)} d(t) dot z
```

is the maximum of finitely many affine functions.  Its edge minimum occurs at
an endpoint or at an intersection of two of those affine functions.  The
method-local analyzer enumerates exactly these rational candidates.

Hypotheses and boundaries:

- `P,Q` are compact, full-dimensional convex polygons; the retained rows meet
  this condition.
- Equation (1) classifies the mathematical two-bounce billiard class under the
  cited non-translatable-polygon characterization.  It does not classify the
  three-bounce class or decide which class wins.
- Equality with retained `A2` checks the existing solved/certified stream on a
  finite generator.  It is validation of that stream against (1), not the
  proof of (1) and not an exhaustive theorem for arbitrary implementation
  inputs.
- `s2=A2^2/(2 area(P) area(Q))` is definitionally determined by this formula.
  Any association of `s2` with a stored target remains descriptive; it is not
  independent validation of `s2`.

Local source route:

- `formal/billiard-capacity-algorithm.tex`, theorem
  `thm:billiard-characterization` and `thm:bounce-bound`, records the
  Artstein--Avidan--Ostrover/Rudolf and Bezdek--Bezdek interfaces.
- `experiments/sys-datascience/methods/ridge-endpoint-path/notes/endpoint-predictions.md`
  and its independent review explicitly use the same support-function length
  and non-translatable-polygon characterization.
- `crates/symplectic/src/algorithms/billiard/` confirms that the existing
  implementation obtains the retained class minimum by enumerating and
  solving two-bounce words; it does not implement (1).
