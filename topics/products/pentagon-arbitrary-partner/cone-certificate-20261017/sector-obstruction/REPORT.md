# The fixed sector has no PSD-plus-linear-products quadratic certificate

17 October 2026. **The remaining certificate class for the existing 36-degree
sector is impossible.** An explicit positive-definite moment matrix strictly
separates the target from every positive-semidefinite quadratic and every
nonnegative product of sector-valid homogeneous linear forms. This is an exact
algebraic obstruction to that proof method; the sharp target inequality itself,
other sectors, and higher-degree or other proof methods remain open.

The sole input is the parent packet's immutable `cone.json`, SHA256
`9f02548e305ed586b83b2709719818281847ce2d2e5b66981b35973acc07e9b6`.
The [checker](check.py) is standard-library-only, reads that input, and emits
[the receipt](result.json), including the full ambient matrix over
$\mathbb Q(\sqrt5)$. It calls no optimizer and writes no files.

## End state and method choice

The intended end state was an exact certificate or an exact obstruction for the
still-open PSD-plus-products class, rather than another numerical negative.
Checking first-order equality constraints was necessary because they can force
whole product families to vanish; checking their tangent kernel was necessary
because an indefinite target there would immediately exclude any PSD remainder.
Those tests did not settle the class. A single reduced conic solve supplied a
candidate separating functional, and exact rational reconstruction settled it.

| Approach | Potential material outcome | Rubric pitfall | Outcome |
| --- | --- | --- | --- |
| Exact equality, gradient and tangent conditions | Cheap elimination or impossibility | Treating necessary-condition feasibility as positivity evidence | Active rank is six, tangent is exactly the equality ray, and the gradient has a strictly positive active-row representation; no obstruction here |
| One reduced PSD conic solve followed by exact checking | Actual certificate or exact dual obstruction | Treating a solver failure/status or floating eigenvalues as proof | Rational strict separator below; only exact checks support the theorem |
| Another no-PSD LP, sector tuning, or full face enumeration | Could change the search problem | Repeating a failed class or enlarging scope without a new mathematical crux | Not performed |

The comparison values eliminating a proof class or establishing the sharp bound;
it does not value tiny numerical improvements, solver status, or dataset fit.
This bounded work used one Clarabel solve without installation, network, cloud,
capacity evaluation or population generation. Its `InsufficientProgress` status
is not evidence of infeasibility; the returned dual numbers merely suggested the
rational matrix. [Discovery metadata](discovery.json) records this distinction.

## Definitions and proposition

Use the parent packet's seven ambient variables
$z=(h_0,h_1,h_2,b_{00},b_{01},b_{10},b_{11})$, target
$q(z)=z^TQz=P(h)-\det B/(3+\sqrt5)$, and 610 normalized rows.
Append exactly the previously frozen sector rows

$$\ell_{610}=u,\qquad \ell_{611}=(5-2\sqrt5)t-u,$$

where $t=b_{00}+b_{11}$ and
$u=b_{10}-(5-2\sqrt5)b_{01}$. Indices here and in the receipt start at zero.
Write $C_{\rm sec}=\{z:\ell_i(z)\ge0,\ 0\le i<612\}$.

**Proposition.** There are no real coefficients $a_{ij}\ge0$ and real symmetric
$S\succeq0$ such that the polynomial identity

$$q(z)=z^TSz+\sum_{0\le i\le j<612}a_{ij}\ell_i(z)\ell_j(z)$$

holds. The same exclusion holds if the products use arbitrary homogeneous
linear forms nonnegative on this same fixed sector cone.

## Explicit strict separator

Let $z_0$ be the input's exact HKO control, for which $z_{0,6}=1$ and $q(z_0)=0$.
Use coordinates

$$\lambda=z_6,\qquad y_i=z_i-\lambda z_{0,i}\quad(0\le i<6).$$

The following rational moment matrix is positive definite:

$$M=\begin{pmatrix}2048&v^T\\v&D\end{pmatrix},\qquad
v=\frac1{1000}(-133,195,-25,-419,177,-63)^T,$$

$$D=\frac1{1000}\begin{pmatrix}
1000&418&-151&529&231&269\\
418&561&118&-138&100&58\\
-151&118&747&199&-516&-220\\
529&-138&199&965&-224&83\\
231&100&-516&-224&738&56\\
269&58&-220&83&56&219
\end{pmatrix}.$$

Let $E$ be the first six columns of $I_7$ and set

$$X=[z_0\ E]M[z_0\ E]^T.$$

The change of coordinates is invertible, so $X\succ0$. The checker verifies
positive definiteness using exact rational LDL pivots, all recorded in the
receipt. No floating eigenvalue is used in the proof. It verifies

$$\langle Q,X\rangle=
\frac{3719}{4000}-\frac{8857}{20000}\sqrt5<0.$$

For every row, put $s_i=\ell_i(z_0)$ and $r_i=\ell_i|_{z_6=0}$, so its
transformed coefficients are $(s_i,r_i)$. Exactly 20 of the $s_i$ vanish.
Every pair touching those active rows has

$$\ell_i^TX\ell_j=r_i^TDr_j+s_i r_j^Tv+s_j r_i^Tv>0.$$

There are 12,050 distinct such pairs. The exact minimum, attained by pair
$(435,610)$, is

$$\frac{4171}{400}-\frac{46489}{10000}\sqrt5>0.$$

The remaining pairs have $s_i,s_j>0$. For their normalized vectors
$a_i=r_i/s_i$, the checker finds the exact uniform coordinate bound

$$\|a_i\|_\infty\le A=\frac{25+11\sqrt5}{4}.$$

Consequently

$$\frac{\ell_i^TX\ell_j}{s_i s_j}
=2048+v^T(a_i+a_j)+a_i^TDa_j
\ge2048-2\|v\|_1A-\Big(\sum_{k,l}|D_{kl}|\Big)A^2>0,$$

because the subtracted bound is exactly

$$\frac{135479}{160}+\frac{1514139}{4000}\sqrt5<2048.$$

This covers all $612\cdot613/2=187578$ unordered row products without enumerating
the inactive/inactive pairs.

Apply the linear functional $H\mapsto\langle H,X\rangle$ to the proposed
identity. The PSD term has nonnegative pairing because $X\succ0$, and every
row-product term has nonnegative pairing by the inequalities above. Their sum
cannot equal the negative target pairing. This proves the proposition.

For its extension, the homogeneous Farkas alternative expresses every
homogeneous linear form nonnegative on $C_{\rm sec}$ as a nonnegative combination
of the 612 row normals. Expanding any product of two such forms gives
nonnegative row-product coefficients, so changing globally sector-valid linear
forms does not escape the obstruction.

## What the result closes

The parent packet excluded the analogous certificate on the unfurled cone by
using ten HKO equality rays. Its exact sector fold removed that particular
obstruction, and its single no-PSD LP was only a numerical negative. This new
strict separator closes the larger PSD-plus-products class on that fixed
sector. Equality/derivative tests alone would have missed it: the active rows
have rank six and the target gradient admits a strictly positive representation
using all 20; both facts are also checked exactly here.

The next mathematical crux is a different proof mechanism: a materially
different sector decomposition, nonquadratic multipliers, or a geometric
inequality. No such route is selected or run by this packet. In particular,
this does not show the target is negative at any feasible point, does not
exclude all piecewise quadratic certificates, and does not prove or disprove
the inherited general-matrix sharp bound or global HKO maximality.

## Reproduction and source limits

From the patterns checkout, write a fresh scratch receipt:

```sh
python3 -B topics/products/pentagon-arbitrary-partner/cone-certificate-20261017/sector-obstruction/check.py \
  topics/empirical-studies/discovery-design/night-20261005/orientation-wide/outer-matrix-preflight/cone.json \
  > /tmp/msc-math/sector-crux/replay.json
```

The retained receipt was produced from this exact checker and source. A separate
[independent checker](independent_check.py), with no imported field arithmetic,
checks all 187,578 row pairs directly using integral $\mathbb Q(\sqrt5)$ signs,
checks positive definiteness through seven positive Bareiss leading principal
minors, and computes the full congruence target trace. Its
[receipt](independent-result.json) passes and binds this retained witness.

```sh
python3 -B topics/products/pentagon-arbitrary-partner/cone-certificate-20261017/sector-obstruction/independent_check.py \
  --source topics/empirical-studies/discovery-design/night-20261005/orientation-wide/outer-matrix-preflight/cone.json \
  --dual topics/products/pentagon-arbitrary-partner/cone-certificate-20261017/sector-obstruction/result.json \
  --out /tmp/msc-math/sector-crux/independent-replay.json
```

Root owns integration, commits and cleanup; this worker changed only the new
packet and made no commit. Sources inspected were this checkout's
README, AGENTS and rationale, research/products guides, parent cone report and
checker, frozen sector probe report/source, and the bound cone input. The
scientific interpretation of that upstream cone reduction is imported rather
than re-proved. No exhaustive repository or literature novelty search is claimed.

[review: /root/queue_triage]
