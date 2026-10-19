# Rotation profiles for every pair of odd regular polygons

Research proof, 19 October 2026, from the fresh authorized research swarm.
Independently checked by two agents; see the accompanying audit records.
Human proof acceptance and a literature-wide novelty assessment are open.

Let `P_j` be the circumradius-one regular j-gon whose outward facet normals
have angles `2 pi k/j` and facet height `h_j=cos(pi/j)`, as in the
[maintained research note](research.tex). For every odd `n,m>=5`, set
`L=lcm(n,m)` and let `d_L(theta)` be the distance to the nearest multiple of
`pi/L`, taking values in `[0,pi/(2L)]`. Then

\[
 \boxed{c_{\rm EHZ}(P_n\times_L R_\theta P_m)
   =(1+h_n)(1+h_m)\sec d_L(\theta)}.
\]

Except for `5 x 5`, every six-block coefficient lies strictly inside the
incircle of the four-block angular hull. Consequently the width formula
persists throughout an open neighborhood of each of these labelled polygon
pairs, uniformly over every relative rotation. The regular secant expression
need not persist after a nonregular perturbation. No explicit neighborhood
radius is claimed. The `5 x 5` case has known six-block edge contacts.

This is an angular result. It does not establish independent-affine tensor
containment or a formula for arbitrary independently affinely regular factors.
It makes no decision about manuscript inclusion.

The imported interface is the [HK source contract](source-contract.md) and the
sparse-product, four-/six-block, and regular four-block-hull reductions in
Sections 2, 3 and 7 of [research.tex](research.tex). The proof below controls
six-block coefficients by translated circles containing step triangles. A
vertex inside a diameter circle forces a deficit because the partner closure
steps have a positive minimum length. A continuous minimum-angle inequality
makes that argument uniform for factors with at least seven sides. Pentagons
use their single isosceles closure type; `5 x 5` and `5 x 7` retain the existing
proofs, and `5 x 9` uses the separate [nonagon norm lemma](nonagon-angular-width.md).

## 1. Shape-sensitive circle deficit (all orders)

For closed complex triples x,y, set

\[
 2z=x_1\overline y_1-x_3\overline y_2.
\]

The three points `(x_1,-x_3,0)` form a triangle with side lengths
`|x_1|,|x_2|,|x_3|`. Translate them by any `c`. Since `sum y_i=0`, the pairing
is unchanged, and the triangle inequality gives

\[
 2|z|\le\sum_i|p_i-c|\,|y_i|.                    \tag{1}
\]

If the point triangle is obtuse or right, let ell be its longest side, let c
be that side's midpoint, and let d be the remaining vertex's distance from c.
Then `r=ell/2` contains the triangle, `d<=r`, and

\[
 2|z|\le rS-m(r-d)\le\frac S2[\ell-p(\ell-2d)]       \tag{2}
\]

whenever `S=sum |y_i|`, `m=min |y_i|`, and `0<=p<=m/S`. Permuting either
triple changes only the labelling of the same side-length triangle, so this
holds for all twelve orders. If the point triangle is acute, its circumradius
Rc instead gives `2|z|<=S Rc`. These bounds are radial: rotating either whole
triple leaves them unchanged.

## 2. Extremal obtuse triangle reduction

Fix `0<b<pi/4`, `0<=p<=1/2`, and perimeter R. Among triangles whose three
angles are at least b and whose largest angle C is at least pi/2, the maximum
of

\[
 F=\ell(1-p)+2pd
\]

is attained with one of the two endpoint angles of the longest side equal to
b. Here ell is that side and d is distance of its midpoint from the remaining
vertex.

Proof: fix ell and R. Write the other side lengths as `r>=s` and `v=r-s`.
Increasing v increases

\[
 d^2=(r^2+s^2)/2-\ell^2/4.
\]

The smaller endpoint angle B has

\[
 \cos B=\frac{\ell^2+(R-\ell)v}{\ell(R-\ell+v)},
\]

whose derivative in v is
`R(R-2ell)/[ell(R-ell+v)^2]>0`. Thus B decreases. Also

\[
 \cos C=\frac{R(R-2\ell)}{2rs}-1
\]

increases as `rs` decreases, so C decreases. The other endpoint angle stays
at least B. Increase v until either `B=b` or `C=pi/2`. The objective does not
decrease. Nondegeneracy cannot be lost before such a boundary, since all angles
remain at least b.

On the right-angle boundary `2d=ell`, so `F=ell`. If its smaller endpoint
angle is B, then

\[
 \ell/R=\frac1{1+\sin B+\cos B}
       \le\frac1{1+\sin b+\cos b},
\]

because `b<=B<=pi/4`. This upper bound is attained at the right triangle with
`B=b`, also included in the desired endpoint-angle family. Hence the maximum
over the entire domain is bounded by that family.

Set `q=tan(b/2)`, `t=tan(A/2)` for the other endpoint angle A. The family has

\[
 q\le t\le\frac{1-q}{1+q},\quad
 \frac\ell R=\frac{1-tq}{2},\quad
 \left(\frac{2d}R\right)^2
   =tq+\frac{(t-q)^2(1-tq)^2}{4(t+q)^2}.             \tag{3}
\]

The maximum possible longest side for any triangle of perimeter R and minimum
angle b is `ell_max=R cos(b)/(1+cos(b))=R(1-q^2)/2`, attained at endpoint
angles both b.

## 3. A scalar circle-deficit inequality

The following elementary lemma holds: for `0<q<=23/100`, triangles of minimum angle
`b=2 arctan(q)` and perimeter R, and `0<=v<=q`, one has

\[
 \ell-v(1-v)(\ell-2d)
    \le\ell_{\max}\left(1-\frac32v^2\right)        \tag{4}
\]

in the obtuse/right case. The proof below uses the boundary reduction above.
For each fixed triangle the difference between the right and left sides is

\[
 D+v(1-v)I-\tfrac32v^2\ell_{\max},\quad
 D=\ell_{\max}-\ell\ge0,\quad I=\ell-2d\ge0.
\]

This is concave in v. Thus it suffices to prove the endpoint `v=q`; the
endpoint `v=0` is automatic.

For the family (3), divide by R and set `p=q(1-q)` and
`B=(1-q^2)(1-3q^2/2)/2`. To prove

\[
 (1-p)(1-tq)/2
 +p\sqrt{tq+(t-q)^2(1-tq)^2/[4(t+q)^2]}\le B,
\]

put `K=B-(1-p)(1-tq)/2`. Its minimum over `t>=q` is at t=q and is
`q(1-q^2)(1-5q/2)/2>0`. Squaring therefore preserves the inequality.
Define

\[
 E=1-\tfrac72q+\tfrac32q^3+t(1-q+q^2).
\]

After multiplication by the positive factor `4(t+q)^2/q^2`, the squared
inequality is equivalent to

\[
 P=(t+q)^2E^2-4qt(t+q)^2(1-q)^2
       -(1-q)^2(t-q)^2(1-tq)^2\ge0.               \tag{5}
\]

Writing `w=t-q>=0`, expansion gives `P=sum_{j=0}^4 c_j(q)w^j`, where

\[
\begin{aligned}
 c_0&=q^2(1-q)^2(2-7q-5q^2)(2+q-5q^2),\\
 c_1&=q(4-12q-43q^2+124q^3-70q^4-48q^5+45q^6),\\
 c_2&=5q-\tfrac{155}4q^2+58q^3-\tfrac{21}2q^4-39q^5+\tfrac{117}4q^6,\\
 c_3&=2-5q+q^2+10q^3-11q^4+7q^5,\\
 c_4&=1-2q+2q^2.
\end{aligned}                                                       \tag{6}
\]

All five coefficients are nonnegative for `0<=q<=3/20`. For the larger
interval `3/20<=q<=23/100`, the following rational bounds suffice:

\[
 c_0\ge0,\quad c_1>3/80,\quad c_2>-1/4,
 \quad c_3>4/5,\quad c_4>0.                       \tag{7}
\]

Here are elementary checks, to avoid treating a successful code execution as
proof. The factored c0 is positive since `2-7Q-5Q^2>0` for `Q=23/100`, as is
the other factor. For `f=c_1/q`,

\[
 f'=-12-86q+372q^2-280q^3-240q^4+270q^5<0,
\]

because `372Q+270Q^4<87` implies `f'<-12+q<0`. Direct rational substitution
gives `f(Q)=50777293721/200000000000>1/4`. Hence `c_1>q/4`, proving its bound
in (7) and positivity everywhere.

For c2, the function `g(q)=5-(155/4)q+58q^2` decreases on `[0,Q]`. Dropping
the final positive term shows

\[
 c_2\ge Q[g(Q)-(21/2)Q^3-39Q^4]
     =-2486740427/10^{10}>-1/4.
\]

This follows since the bracket is bounded below by the negative displayed
endpoint bracket and `q<=Q`. On `[0,3/20]`, the same decreasing-g argument
bounds the bracket below by
`g(3/20)-(21/2)(3/20)^3-39(3/20)^4>0`, so c2 is nonnegative there.
Finally `c_3>=2-5Q-11Q^4>4/5`, and `c_4=(1-q)^2+q^2>0`.

For `q>=3/20`, the middle terms satisfy

\[
 c_1w+c_2w^2+c_3w^3
 \ge w\left(3/80-w/4+4w^2/5\right)\ge0,
\]

because the quadratic has positive leading coefficient and discriminant
`1/16-3/25=-23/400<0`. This proves (5), hence (4).

## 4. Acute triangles and elementary parameter bounds

For an acute triangle of perimeter R with each angle at least b<pi/4, its
circumradius satisfies

\[
 R_c\le\frac R{2(1+\sin b+\cos b)}.                \tag{8}
\]

Indeed `R=2 R_c(sum sin A)`. The angle domain is the compact convex polygon
`b<=A,B,C<=pi/2`, `A+B+C=pi`; concavity of sine makes its sum attain a minimum
at an extreme point, a permutation of `(b,pi/2-b,pi/2)`. The bound therefore
also holds on the acute-domain closure.

With `q=tan(b/2)<=Q=23/100`, define

\[
 g(q)=\frac{1+q^2}{(1-q^2)(1+q)}.
\]

Then `g(q)<1-3q^2/2` for q>0: after positive denominators, the difference is

\[
 q[1-(7/2)q-(5/2)q^2+(3/2)q^3+(3/2)q^4]>0,
\]

since even `1-(7/2)Q-(5/2)Q^2>0`.

The bound `tan(pi/14)<23/100` used above is elementary. For
`0<x<=9/40`, Taylor bounds give

\[
 \tan x\le\frac{x-x^3/7}{1-x^2/2}.
\]

For sine this follows from `sin x<=x-x^3/6+x^5/120<=x-x^3/7`;
cosine satisfies `cos x>=1-x^2/2>0`. The displayed rational function
increases on this interval. Since `pi<22/7` gives `pi/14<11/49<9/40`,

\[
 \tan(\pi/14)<\frac{100071}{436660}<23/100.          \tag{9}
\]

For any triangle of perimeter R whose minimum angle is at least a, its
smallest side is at least

\[
 R\,\delta(a),\qquad
 \delta(a)=\frac{\sin(a/2)}{1+\sin(a/2)}.
\]

For a side opposite A>=a, divide `sin A` by the sine sum to obtain

\[
 \frac{\sin A}{\sin A+\sin B+\sin C}
 =\frac{\sin(A/2)}{\sin(A/2)+\cos((B-C)/2)}
 \ge\delta(a).
\]

If `v=tan(a/2)`, then

\[
 \delta(a)=v(\sqrt{1+v^2}-v)\ge v(1-v).             \tag{10}
\]

## 5. Application to every odd pair n,m>=7

Use the normalization and sparse decomposition of the maintained research note.
Order the factors so `n>=m>=7`, and put

\[
 a=\pi/n\le b=\pi/m,\quad
 v=\tan(a/2)\le q=\tan(b/2)\le23/100,
 \quad s_j=(1+\cos(\pi/j))^{-1},\quad
 \rho=s_ns_m/2.
\]

A factor closure triple has step perimeter `R_j=sec(pi/j)` and minimum
interior angle at least `pi/j`: its angles are positive odd multiples of
`pi/j` summing to pi. The proof of this angle assertion is given in the
[reviewed nonagon note](nonagon-angular-width.md) and works without change for every odd j.

Apply the circle bound using a step triangle from the smaller m-gon as the
point triangle, and a closure triple from the larger n-gon as the partner.
The bound applies also with factors exchanged: conjugating z and suitably
relabeling the two closed triples restores its displayed form. Equation (10)
allows `p=v(1-v)`.

The maximum possible longest side of the m-step triangle is exactly `s_m`.
One direct proof is this. For a triangle of perimeter R and minimum angle b,
let C be the largest angle. If A+B=pi-C and A,B>=b, then

\[
 \ell/R=\frac{\sin(C/2)}{\sin(C/2)+\cos((A-B)/2)}
 \le\frac{\sin(C/2)}{\sin(C/2)+\sin(C/2+b)}
 \le\frac{\cos b}{1+\cos b}.
\]

The last ratio increases with C/2 up to `pi/2-b`. For `R=sec b`, the bound
is `s_m`, attained by the isosceles triangle with endpoint angles b,b.

If the m-step triangle is obtuse/right, (2) and (4) give

\[
 |z|\le\frac{R_n s_m}4(1-3v^2/2),\qquad
 \frac{|z|}{\rho}\le\frac{1-3v^2/2}{1-v^2}
       <\frac1{\sqrt{1+v^2}}=\cos(a/2).            \tag{11}
\]

The strict scalar comparison follows from
`(1-v^2)/sqrt(1+v^2)>1-3v^2/2`, for example using
`(1+x)^(-1/2)>1-x/2` for x>0 and `v<1`.

If the m-step triangle is acute, (1) and (8) give instead

\[
 \frac{|z|}{\rho}
 \le\frac{g(q)}{1-v^2}
 <\frac{1-3v^2/2}{1-v^2}<\cos(a/2).               \tag{12}
\]

Here `g(q)<1-3q^2/2<=1-3v^2/2`, and
`R_n/(2s_n)=1/(1-v^2)`. Thus every paired closure-triangle coefficient lies
strictly inside the disk of radius `rho cos(pi/(2n))`. Since
`lcm(n,m)>=n`, this disk is contained in the four-block incircle, whose radius
is `rho cos(pi/(2 lcm(n,m)))`.

## 6. Pairs with a pentagon factor

The maintained research note already proves 5x5 and 5x7. The 5x5 case can have
six-block edge contacts; the other cases here have strict radial containment.

For 5x9, the reusable norm estimate in the [reviewed nonagon note](nonagon-angular-width.md) gives

\[
 |z/\rho|^2\le M_5M_9/3,
 \quad M_5=4-\sqrt5<16/9,
 \quad M_9<5/3.
\]

Thus `|z/rho|^2<80/81`. The regular four-block incircle squared radius is
`cos^2(pi/90)>1-4/2025=2021/2025>80/81`, using `pi<4` and
`sin x<x`. The rational square comparison `5>400/81` proves
`sqrt(5)>20/9`, hence the stated M5 bound.

For 5xm with odd m>=11, every pentagon closure triangle is isosceles, with
longest side `ell=s_5`, diameter-circle radius `r=ell/2`, and remaining vertex
distance `d=r tan(pi/5)`. Put `a=pi/m`, `v=tan(a/2)`. The same partner-minimum
bound allows `p=v(1-v)`. The exact pentagon value satisfies

\[
 \tan(\pi/5)<73/100,
\]

because its square is `5-2sqrt(5)`, and `sqrt(5)>447/200` gives the stronger
square upper bound `53/100<(73/100)^2`. Also

\[
 v\le\tan(\pi/22)<14/97<3/20;
\]

use `pi/22<1/7` and `tan x<=x/(1-x^2/2)` on that interval. Consequently

\[
 p(1-\tan(\pi/5))
  >\frac{27}{100}v(1-v)
  >\frac32v^2,
\]

because at `v=3/20`, `(27/100)(1-v)=459/2000>9/40=(3/2)v`.
The circle deficit therefore gives exactly the upper bound in (11), with the
pentagon as point triangle. Since `lcm(5,m)>=m`, strict radial containment
follows again.

## 7. Capacity theorem and precise dependencies

The preceding lemmas, the maintained sparse reduction, four-/six-block
decomposition, and known four-block regular hull prove, for every pair of odd integers n,m>=5 and every relative angle theta,

\[
 c_{\rm EHZ}(P_n\times_L R_\theta P_m)
   =(1+\cos(\pi/n))(1+\cos(\pi/m))
       \sec d_{\operatorname{lcm}(n,m)}(\theta).
\]

The strict incircle result holds for every such pair except 5x5, whose angular
containment is the independently proved pentagon tensor theorem. Therefore the
maintained stability proposition gives open labelled nonregular width-formula
neighborhoods of all the remaining pairs. Its nondegeneracy hypotheses hold:
regular odd dual vertices lie on a circle, no three are collinear, and no pair
is opposite, so the origin belongs to no line through a pair. This is rotation-only:
none of these angular/radial bounds implies independent-affine tensor
containment. No novelty, human acceptance, or automatic manuscript inclusion
is asserted.

## 8. Sharp systolic bound throughout the rotated odd regular class

For all the pairs above,

\[
 \frac{c_{\rm EHZ}(P_n\times_LR_\theta P_m)^2}
      {2\operatorname{area}(P_n)\operatorname{area}(P_m)}
 \le\frac{3+\sqrt5}{5}.
\]

Equality holds exactly when `n=m=5` and
`theta=pi/10+k pi/5`, `k` an integer. This is a global statement within the
rotated odd regular class; it is not a maximum over arbitrary polygon pairs or
arbitrary convex bodies.

Indeed, set

\[
 f(j)=\frac{(1+\cos(\pi/j))^2}{j\sin(2\pi/j)}.
\]

The area formula `area(P_j)=j sin(2pi/j)/2` and the capacity theorem give the
systolic ratio `2 f(n) f(m) sec^2 d_L(theta)`. The function f strictly decreases
for real `j>=5`. To verify this, write `x=pi/j` and

\[
 F(x)=\frac{x(1+\cos x)^2}{\pi\sin(2x)},
 \qquad 0<x\le\pi/5.
\]

Direct differentiation gives

\[
 (\log F)'=\frac1x-\csc x-\tan(x/2)+\tan x
   >\tan x-2\tan(x/2)>0.
\]

For the first strict inequality use
`csc x-1/x<tan(x/2)`, equivalent to `cot x<1/x`, which follows from
`tan x>x`. For the second use the tangent double-angle formula. Thus f is
strictly decreasing. Since `L>=5` and `d_L(theta)<=pi/(2L)`,

\[
 2f(n)f(m)\sec^2d_L(\theta)
 \le2f(5)^2\sec^2(\pi/10)=\frac{3+\sqrt5}{5}.
\]

The last value is the regular-pentagon maximum already computed in the
maintained research note. Strict decrease of f forces both side counts to be
five in any equality case; the angular bound then forces the displayed
switching angles. Conversely these pentagon angles attain the value by the
known pentagon profile.
