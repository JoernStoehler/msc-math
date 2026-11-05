# A uniform systolic bound without a uniform capacity formula

Research proof candidate, 28 October 2026. Independent mathematical review
pending. The argument uses the established feasible four-block construction,
not the open full tensor-containment conjecture. Novelty is unassessed; this
note does not change the selected manuscript.

The exact capacity of two independently affinely regular odd polygons is
known only for some pairs. An upper bound on capacity is much easier: a
feasible trajectory suffices. Combining that observation with planar area
inclusion proves a uniform systolic inequality for the entire class.

## Statement

Let $P_j$ be the circumradius-one regular $j$-gon with unit outward normals
$n_i^{(j)}=(\cos(2\pi i/j),\sin(2\pi i/j))$. For odd $n,m\geq5$, nonsingular
real $2\times2$ matrices $G,H$, and arbitrary translations $a,b$, put
$A=a+GP_n$, $B=b+HP_m$. Define

$$
\kappa_j=\frac{4(1+\cos(\pi/j))^2}
 {j^2\sin(\pi/j)\sin(2\pi/j)},\qquad
r_j=1+\sec(\pi/j).
$$

Then

$$
\boxed{\operatorname{sys}(A\times_L B)
\leq\frac12\min\{r_n\kappa_m,r_m\kappa_n\}
\leq\frac{3+\sqrt5}{5}.} \tag{1}
$$

The last constant is sharp. Equality there holds exactly when $n=m=5$ and
$G^{-1}H^{-T}=tR_{\pi/10}U$, where $t>0$ and $U$ is an orthogonal symmetry
of the regular decagon $\operatorname{conv}\{\pm n_i^{(5)}\}$.

If $(n,m)\ne(5,5)$, the stronger uniform conclusion is

$$
\operatorname{sys}(A\times_L B)
\leq \frac{\sqrt5}{2}\kappa_7<1. \tag{2}
$$

The bound in (2) is approximately $0.972260$. It is a convenient uniform
ceiling, not a claimed sharp value for any mixed pair. Thus every pair in
this class other than two affine pentagons satisfies the Viterbo bound,
regardless of whether its full tensor-containment conjecture is true.

## 1. Use four blocks only as an upper bound

Write $h_j=\cos(\pi/j)$, $w_j=1+h_j$ and
$D_j=\operatorname{conv}\{\pm n_i^{(j)}\}$. For odd $j$, the polar of the
difference body is

$$
(P_j-P_j)^\circ=\frac{D_j}{w_j}.
$$

The [four-block construction](research.tex), Lemma `lem:width` and Theorem
`thm:decomp`, gives feasible HK words whose maximal quadratic action is

$$
Q_4=\frac{1}{2w_nw_m}\max_{i,j}
 |(n_i^{(n)})^T M n_j^{(m)}|,
\qquad M=G^{-1}H^{-T}.
$$

Set the maximum to $\mu(M)>0$. The full HK maximum is at least $Q_4$;
therefore the [HK formula and conventions](source-contract.md) give

$$
c_{\mathrm{EHZ}}(A\times_L B)=\frac{1}{2Q_{\max}}
\leq\frac{w_nw_m}{\mu(M)}. \tag{3}
$$

Additional six-block words can only lower capacity relative to this ceiling.
Equality in (3) is not needed anywhere below. Translations do not affect
capacity, area, or difference bodies.

## 2. Planar inclusion bounds the normalized determinant

The definition of $\mu$ gives

$$
\frac{M}{\mu(M)}D_m\subseteq D_n^\circ.
$$

Area monotonicity, followed by the same argument for $M^T$, yields

$$
\frac{|\det M|}{\mu(M)^2}
\leq\min\left\{
\frac{|D_n^\circ|}{|D_m|},
\frac{|D_m^\circ|}{|D_n|}\right\}. \tag{4}
$$

Here $|\cdot|$ denotes planar area. The regular $2j$-gon $D_j$ has
circumradius one, so

$$
|D_j|=j\sin(\pi/j),\qquad
|D_j^\circ|=2j\tan(\pi/(2j)).
$$

Also $S_j=|P_j|=j\sin(2\pi/j)/2$, and
$|A||B|=S_nS_m/|\det M|$. Combining (3) and (4) gives

$$
\operatorname{sys}(A\times_L B)
\leq\frac{w_n^2w_m^2}{2S_nS_m}
 \min\left\{\frac{|D_n^\circ|}{|D_m|},
             \frac{|D_m^\circ|}{|D_n|}\right\}.
$$

For the first area ratio, substitution and
$\tan(x/2)=\sin x/(1+\cos x)$ simplify its value to

$$
\frac{(1+h_n)(1+h_m)^2}
 {m^2h_nh_m\sin^2(\pi/m)}
=\frac12 r_n\kappa_m.
$$

The other ratio gives $r_m\kappa_n/2$, proving the first inequality of (1).

## 3. Monotonicity, sharpness and the gap below one

Both $r_j$ and $\kappa_j$ strictly decrease for $j\geq5$.
For completeness, with $x=\pi/j\in(0,\pi/5]$,

$$
\kappa(x)=\frac{2}{\pi^2}
 \frac{x^2\cot^2(x/2)}{\cos x},\qquad
\frac{d}{dx}\log\kappa(x)
=\frac2x-\frac2{\sin x}+\tan x.
$$

Since $\sin x>x\cos x$,

$$
\frac1{\sin x}-\frac1x<\tan(x/2),
$$

and hence the logarithmic derivative is greater than
$\tan x-2\tan(x/2)>0$. The last inequality follows by putting
$t=\tan(x/2)>0$ in $\tan x=2t/(1-t^2)$.
Thus increasing $j$ decreases $\kappa_j$; the same is immediate for $r_j$.
This reproduces the elementary monotonicity argument in the
[odd symmetric-partner proof](../pentagon-arbitrary-partner/odd-regular-symmetric-partner.md).

Consequently the right side of (1) is at most $r_5\kappa_5/2$. Using
$r_5=\sqrt5$ and $\kappa_5=(10+6\sqrt5)/25$, this is
$(3+\sqrt5)/5$. If at least one side count is at least seven, choose the term
that places that index on $\kappa$ to obtain (2).

Here is a purely rational check that the constant in (2) is below one,
without relying on the displayed decimal. The classical bounds
$6283/2000<\pi<22/7$, the alternating sine and cosine expansions on $(0,1)$,
and rational arithmetic give, for $x=\pi/7$,

$$
\frac9{10}<\cos x<\frac{901}{1000},\qquad
\sin x>\frac{43}{100}.
$$

Explicitly use $l=6283/14000<x<u=22/49$ and bound the cosine from below by
$1-u^2/2+u^4/24-u^6/720$, from above by $1-l^2/2+l^4/24$, and the sine from
below by $l-l^3/6$. All three comparisons follow by multiplication of
positive integer denominators. Since $\sqrt5<9/4$,

$$
\frac{\sqrt5}{2}\kappa_7
=\frac{\sqrt5(1+\cos x)^2}{49\cos x\sin^2x}
<\frac{(9/4)(1901/1000)^2}
 {49(9/10)(43/100)^2}
=\frac{3613801}{3624040}<1.
$$

Equality in the final sharp HKO bound forces $n=m=5$, because the decrease
above is strict. For that pair, the established independent-affine
[pentagon theorem](research.tex), Theorems `thm:affine` and `thm:sys`, supplies
both attainment and exactly the stated equality class. This completes the
argument relative to those established HK and pentagon inputs.

## What this changes, and what remains open

The systolic upper-bound problem for all independently affine odd regular
pairs separates from the exact-capacity problem. A proof or counterexample
to general full tensor containment would still be mathematically useful,
but is unnecessary for (1)–(2). The argument does not cover arbitrary
pentagons, arbitrary planar factors, triangles, or unrestricted convex
four-dimensional bodies. It neither proves a universal Viterbo inequality
nor supplies a new counterexample.
