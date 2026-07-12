# Regular triangle--hexagon interval theorem

## Result

Let `e(theta) = (cos(theta), sin(theta))` and normalize the regular factors by

```text
T = conv{(2,0), (-1,sqrt(3)), (-1,-sqrt(3))},
Q_delta = conv{(2/sqrt(3)) e(delta + k*pi/3) : k=0,...,5}.
```

Thus both factors have inradius one. The relative-rotation fundamental interval
is `0 <= delta <= pi/6`. On this whole interval,

```text
c_EHZ(T x_L Q_delta) <= 3 sqrt(3) sec(delta),
sys(T x_L Q_delta) <= (3/4) sec(delta)^2 <= 1.                 (1)
```

At the right endpoint the inequalities are equalities:

```text
c_EHZ(T x_L Q_{pi/6}) = 6,
sys(T x_L Q_{pi/6}) = 1.                                     (2)
```

Equation (1) is an upper envelope supplied by one admissible billiard. It does
not assert that this billiard minimizes at every interior angle. Any competing
branch below it only decreases capacity and `sys`. Consequently, no all-branch
minimality argument is needed for Viterbo's inequality on this path. Such an
argument would be needed only for the stronger exact-profile statement

```text
sys(T x_L Q_delta) = (3/4) sec(delta)^2
```

at every interior angle.

## Explicit three-bounce billiard

The outward unit normals of the three triangle facets are

```text
a0 = e(pi/3),   a1 = e(pi),   a2 = e(5*pi/3),
```

and each facet has support height one. Put

```text
t = tan(delta),   lambda = sqrt(3) sec(delta),
```

and define the bounce points

```text
x0 = ((1-sqrt(3)t)/2, (sqrt(3)+t)/2),
x1 = ((1+sqrt(3)t)/2, (t-sqrt(3))/2),
x2 = (-1,-t).
```

Direct substitution gives

```text
x0 in F_T(a0),   x1 in F_T(a2),   x2 in F_T(a1).
```

The remaining triangle inequalities show that all three points stay on the
indicated closed edges for `0 <= delta <= pi/6`. Define

```text
u0 = e(3*pi/2 + delta),
u1 = e(5*pi/6 + delta),
u2 = e(pi/6 + delta).
```

Then

```text
x1-x0 = lambda u0,
x2-x1 = lambda u1,
x0-x2 = lambda u2.                                            (3)
```

Choose the momentum points

```text
p0 = sec(delta) e(3*pi/2),
p1 = sec(delta) e(5*pi/6),
p2 = sec(delta) e(pi/6).
```

The `Q_delta` facet with normal `uj` has support height one. Since
`uj dot pj = 1` and the angular displacement of `pj` from `uj` is `delta`, each
`pj` lies on that closed facet throughout the interval. This includes
`delta=pi/6`, where `pj` is an endpoint and its normal cone is larger. Hence
equation (3) gives

```text
x_{j+1}-x_j in N_{Q_delta}(p_j).
```

The momentum jumps are

```text
p1-p0 = -sqrt(3) sec(delta) a2,
p2-p1 = -sqrt(3) sec(delta) a1,
p0-p2 = -sqrt(3) sec(delta) a0.
```

Therefore

```text
p_{j+1}-p_j in -N_T(x_{j+1}),
```

with cyclic indices. These are exactly the generalized Minkowski-billiard
conditions. The construction is thus an admissible three-bounce billiard, not
only a numerically observed orbit signature.

Because `h_{Q_delta}(uj)=1`, every increment in (3) has `Q_delta`-length
`lambda`. Its total action is

```text
A_H(delta) = 3 lambda = 3 sqrt(3) sec(delta).                  (4)
```

The factor areas are

```text
area(T) = 3 sqrt(3),   area(Q_delta) = 2 sqrt(3),
vol_4(T x_L Q_delta) = 18.
```

The billiard characterization of EHZ capacity and (4) give

```text
sys(T x_L Q_delta)
  = c_EHZ(T x_L Q_delta)^2 / 36
  <= A_H(delta)^2 / 36
  = (3/4) sec(delta)^2.
```

Since `sec(delta)^2 <= 4/3` on the fundamental interval, (1) follows.
Continuity of capacity is not needed.

## Separate lower bound at `delta = pi/6`

The admissible orbit gives `c_EHZ <= 6` at the endpoint. Equality requires a
lower bound for every non-translatable polygonal curve; it does not follow from
the interior upper envelope.

The polar triangle is

```text
T^circ = conv{a0,a1,a2}.
```

Comparing the six vertices gives

```text
Q_{pi/6} = (2/3)(T^circ - T^circ).
```

Consequently, for `f_i(x)=a_i dot x`,

```text
h_{Q_{pi/6}}(v)
  = (2/3)(max_i f_i(v) - min_i f_i(v)).                        (5)
```

Let `gamma` be any closed polygonal curve and set

```text
M_i = max_{x in gamma} f_i(x),   S = M_0+M_1+M_2.
```

Because `a0+a1+a2=0`, the map `z -> (f_0(z),f_1(z),f_2(z))` is an isomorphism
from the plane onto the subspace whose coordinates sum to zero. Translating
`gamma` by `z` into `int(T)` is therefore equivalent to finding three numbers
`f_i(z)` with sum zero and

```text
f_i(z) < 1-M_i   for i=0,1,2.
```

Such numbers exist exactly when `sum_i(1-M_i)>0`. Thus

```text
gamma is not translatable into int(T)  iff  S >= 3.            (6)
```

For each `i`, choose `z_i` on `gamma` with `f_i(z_i)=M_i`, and list the three
chosen points in their cyclic order along `gamma`, say `z_i,z_j,z_k`. Let

```text
rho(v) = max_l f_l(v) - min_l f_l(v).
```

The `rho`-length of the arc from `z_i` to `z_j` is at least

```text
rho(z_j-z_i)
  >= (a_j-a_i) dot (z_j-z_i)
   = M_j-f_j(z_i) + M_i-f_i(z_j).
```

Apply the analogous inequality to the other two arcs. At `z_i`, the sum of
the other two `f`-coordinates equals `-M_i`, again because
`a0+a1+a2=0`. Summing the three arc inequalities therefore gives

```text
length_rho(gamma) >= 3S.                                      (7)
```

Combining (5)--(7), every non-translatable polygonal curve satisfies

```text
length_{Q_{pi/6}}(gamma)
  = (2/3) length_rho(gamma)
  >= 2S
  >= 6.
```

The non-translatable-polygon characterization of the shortest Minkowski
billiard now gives `c_EHZ >= 6`. Together with the explicit endpoint orbit,
this proves (2).

## Source and verification status

The theorem-level inputs are:

- the generalized billiard normal-cone definition and the action
  `sum h_Q(x_{i+1}-x_i)` in
  `papers/hko2024/counterexample.tex`, definition
  `def-T-billiard-in-K` and the discussion immediately following it;
- the shortest-billiard/non-translatable-polygon characterization and the
  at-most-three-bounce reduction cited there to Bezdek--Bezdek and Rudolf;
- the Lagrangian-product EHZ/billiard interface in
  `thesis/04-haim-kislev-quadratic-program.tex`, theorem
  `thm:lagrangian-product-finite-enumeration`, with citations to AAO2014
  Theorem 2.13, Rudolf2022 Theorem 1, and Bezdek--Bezdek Theorem 1.1 and
  Lemma 2.4;
- the regular-polygon fundamental-domain lemma in
  `formal/lagrangian-product-rotation-symmetry.tex`, label
  `lem:rotation-fundamental-domain`.

The explicit orbit, interval upper bound, and endpoint range-norm lower bound
in this file are agent-derived mathematics. They were independently accepted
in the mathematical audit that preceded this promotion. They have not been
reviewed or approved by Jörn, and this file is not thesis-facing exposition.

The two retained interior target evaluations and the earlier `q01` row agree
with equality in the upper envelope to floating-point precision. Those rows
are numerical validation that this branch is minimizing at the sampled
angles; they are not inputs to either inequality proof above and do not prove
the exact interior profile.
