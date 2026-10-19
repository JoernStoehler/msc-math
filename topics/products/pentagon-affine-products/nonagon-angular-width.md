# Two regular nonagons: an elementary angular width formula

Research note, 19 October 2026. This is a new proof-development note within the
fresh authorized research swarm. It depends on the maintained Haim–Kislev/sparse
product reduction in [research.tex](research.tex), Sections 2–3, and its regular
four-block hull computation in Section 7. It does not establish a formula for all
odd regular factors or for arbitrary independent affine nonagon images. Human
acceptance and novelty are unestablished. The proof received a separate independent agent audit on 19 October 2026;
human acceptance and a literature-wide novelty assessment remain separate.

## Result and normalization

Let `P_n` have circumradius one, facet normals at angles `2 pi k/n`, and facet
height `h_n=cos(pi/n)`, as in the maintained note. Put

\[
 h=\cos(\pi/9),\qquad s=(1+h)^{-1},\qquad \rho=s^2/2.
\]

Then for every real relative angle `theta`,

\[
 c_{\rm EHZ}(P_9\times_L R_\theta P_9)
   =(1+h)^2\sec d_9(\theta),
 \qquad d_9(\theta)=\min_{k\in\mathbb Z}|\theta-k\pi/9|.
\]

The notation `d_9` denotes distance to the nearest multiple, so its value lies
in `[0,pi/18]`. More precisely, every six-block coefficient lies strictly inside
the incircle of the regular eighteen-gon four-block coefficient hull. Therefore
the strict-containment stability proposition in the maintained note also gives
an open neighborhood of this labelled nonagon pair on which the width-body
formula holds at every relative rotation. The displayed regular secant profile
need not persist after a nonregular perturbation. No explicit neighborhood
radius is claimed.

## A reusable zero-sum triple estimate

For arbitrary complex triples with `x_1+x_2+x_3=y_1+y_2+y_3=0`, define

\[
 z=\frac12(x_1\overline{y_1}-x_3\overline{y_2}).
\]

Then

\[
 |z|^2\le\frac1{12}
       \left(\sum_{i=1}^3|x_i|^2\right)
       \left(\sum_{j=1}^3|y_j|^2\right).                 \tag{1}
\]

This estimate is independent of any polygon and unchanged when the entries of
either triple are permuted. Thus it bounds all twelve cyclic six-block orders
at once; it does not assert that their coefficients agree.

To prove (1), take orthonormal coordinates on the complex sum-zero plane:

\[
 u=\frac{2x_1-x_2-x_3}{\sqrt6},\quad
 v=\frac{x_2-x_3}{\sqrt2},
 \qquad U,V\text{ similarly for }y.
\]

The squared norm is `|u|^2+|v|^2=sum |x_i|^2`, and direct substitution gives

\[
 z=\frac14(u\overline U+v\overline V)
      +\frac1{4\sqrt3}(u\overline V-v\overline U).
\]

The corresponding real two-by-two coefficient matrix is

\[
 C=\frac14\begin{pmatrix}1&1/\sqrt3\\-1/\sqrt3&1\end{pmatrix},
 \qquad C^TC=\frac1{12}I.
\]

Complex Cauchy–Schwarz proves (1). No choice of relative spatial rotation is
needed: rotating either whole triple multiplies `z` by a unit complex number.

## Exhaustive closure shapes for the nonagon

For a positive three-facet normalized closure, put its normal indices in cyclic
order, with positive integer gaps `a,b,c`. They sum to nine. Positivity of the
origin barycentric coordinates is equivalent to every angular gap being less
than pi, so each index gap is at most four. Conversely every such gap triple
gives a positive closure. The only unordered possibilities are

\[
 (1,4,4),\qquad (2,3,4),\qquad (3,3,3).               \tag{2}
\]

The scalene type has both cyclic orientations; rotations and reflection do not
change its squared step norm. No opposite pair exists for an odd regular
polygon, so there are no two-facet closure vertices. Every closure vertex is
therefore represented by (2), by the support-at-most-three reduction.

For a closure distribution `gamma`, its normalized steps are
`x_i=gamma_i n_i/h`, where `n_i` are the corresponding unit normals. Hence

\[
 \sum_i x_i=0,\qquad \sum_i|x_i|=1/h.
\]

In cyclic direction order these are the edges of a triangle. Its interior
angles are `pi-2 pi a/9`, `pi-2 pi b/9`, `pi-2 pi c/9`, up to cyclic relabelling.
Thus they are positive odd multiples of `pi/9` summing to pi. If these angles
are `A,B,C`, the sine rule gives

\[
 \frac{\sum_i|x_i|^2}{s^2}
   =\frac{(1+h)^2}{h^2}
      \frac{\sin^2 A+\sin^2 B+\sin^2 C}
           {(\sin A+\sin B+\sin C)^2}.              \tag{3}
\]

We show that every quantity in (3) is strictly less than `5/3`. First,

\[
 h>\cos(\pi/8)>11/12.                               \tag{4}
\]

For the second inequality, square the positive quantities and use
`cos^2(pi/8)=(2+sqrt(2))/4` and `sqrt(2)>49/36`, the latter following from
`2592>2401`.

For `(1,4,4)`, the isosceles step triple is, up to rotation and labelling,

\[
 s\left(1,\frac{-1+i\tan(\pi/9)}2,
          \frac{-1-i\tan(\pi/9)}2\right).
\]

Its normalized squared norm is `(3+tan^2(pi/9))/2<5/3`, since `pi/9<pi/6`.

For `(3,3,3)`, the steps have equal length `1/(3h)`, so the normalized squared
norm is `(1+h)^2/(3h^2)<5/3`. After positive denominators this is
`4h^2-2h-1>0`, which follows from (4): this polynomial increases for `h>1/4`
and its value at `11/12` is `19/36>0`.

For `(2,3,4)`, the triangle angles are `pi/9,pi/3,5pi/9`. Equation (3) becomes

\[
 N(h)=\frac{(1+h)^2}{h^2}
       \frac{16h^4-16h^2+9}{48h^4-24h^2+3}.
\]

Indeed the sine sum is `sqrt(3)(1/2+cos(2pi/9))`, and the sum of squared sines
is `7/4+(1/2)cos(4pi/9)`. Its denominator is positive, being
`48h^4-24h^2+3=3(4h^2-1)^2`. The triple-angle identity gives
`8h^3-6h-1=0`; reducing powers by this identity yields

\[
 5h^2(48h^4-24h^2+3)
   -3(1+h)^2(16h^4-16h^2+9)
   =3(14h^2-5h-7).
\]

The quadratic on the right increases for `h>5/28`; at `11/12` it equals
`13/72>0`. This proves `N(h)<5/3`, including the reflected scalene type.
Consequently

\[
 \sum_i|x_i|^2<\frac53s^2                         \tag{5}
\]

for every nonagon closure vertex.

## Containment and capacity

Apply (1) and (5) to each pair of closure triangles. In the normalization of the
maintained note, every six-block coefficient therefore satisfies

\[
 \frac{|z|^2}{\rho^2}<\frac{25}{27}.
\]

The four-block hull is the regular eighteen-gon of circumradius `rho`, whose
squared incircle radius is

\[
 \rho^2\cos^2(\pi/18)
   =\rho^2\frac{1+h}{2}
   >\rho^2\frac{23}{24}
   >\rho^2\frac{25}{27}.
\]

Thus every six-block coefficient lies strictly inside that incircle. The sparse
reduction and four-/six-block decomposition now give

\[
 Q_{\max}(\theta)=\rho\cos d_9(\theta),
 \qquad c_{\rm EHZ}=1/(2Q_{\max}),
\]

which proves the stated formula. Because the strict gap also holds at switching
angles, no six-block sparse word is a maximizer here. This is a statement about
this class of sparse words, not uniqueness or classification of all capacity
minimizers.

## Transfer value and limits

For any polygon pair, let `s_A,s_B>0` be radii for which the four-block angular
hull contains the closed disk of radius `s_A s_B r/2`. If every factor closure
triple satisfies

\[
 \sum|x_i|^2\le M_A s_A^2,\qquad
 \sum|y_i|^2\le M_B s_B^2,
 \qquad M_A M_B<3r^2,
\]

then (1) gives strict angular containment. For odd regular factors one can take
`s_A=(1+h_n)^-1`, `s_B=(1+h_m)^-1`, `r=cos(pi/(2 lcm(n,m)))`.
The scalar factor norms can be checked by cyclic gap types without enumerating
all interleavings. This is a sufficient condition; failure of the condition is
not a counterexample to the width formula.

The aggregate-norm criterion cannot settle all odd regular pairs. Here is an
exact obstruction to that criterion, rather than a counterexample to the width
formula. Let `n=4k+3>=23`, set `a=pi/n`, `u=sin(a/2)`, and take the closure gaps
`(k+1,k+1,2k+1)`. Its step triangle has angles
`a,(pi-a)/2,(pi-a)/2`. Its normalized squared step norm is

\[
 N=\frac{(1+h_n)^2}{h_n^2}
       \frac{u^2+1/2}{(1+u)^2}.
\]

Since `h_n<1`, the first factor exceeds four. Also `0<u<1/13` follows from
`sin t<t` and `pi<22/7`. The function `(u^2+1/2)/(1+u)^2` decreases for
`0<u<1/2`. Therefore

\[
 N>4\frac{1/169+1/2}{(1+1/13)^2}
    =\frac{171}{98}>\sqrt3,
\]

where the last comparison squares to `29241>28812`. Taking this closure shape
in each factor gives `N^2/3>1`, whereas the four-block incircle squared radius
normalized by `rho^2` is strictly less than one. Thus the sufficient criterion
fails for every such equal-factor pair. A uniform proof must use more
information than the two aggregate squared step norms, or use a different
sufficient condition for these remaining shapes. This failure does not say
that the actual six-block coefficients leave the four-block polygon.

An optional exact rational-interval checker was developed in the scratch run
before the analytic simplification. The proof above does not consume its
output. No retained numerical campaign, manuscript promotion, human proof
acceptance, or novelty claim is implied by this note.
