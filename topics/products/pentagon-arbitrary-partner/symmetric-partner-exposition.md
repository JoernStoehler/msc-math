# A pentagon sees the largest inscribed decagon

Fix a regular pentagon in the momentum plane. How much freedom remains if its
partner in the position plane may be any centrally symmetric convex body?
For this class, including smooth bodies, capacity has a geometric answer:
measure the largest copy of one particular regular decagon that fits inside
the partner. This also identifies the unique partner shape with largest
systolic ratio.

The point is that the same decagon passes every triangle test required by the
capacity characterization. Once this is checked, monotonicity transfers its
capacity to the entire class. No enumeration of Reeb orbits or numerical
capacity evaluation enters the proof.

Let
\[
 \theta=\pi/5,\qquad h=\cos\theta,\qquad
 n_i=(\cos(2\pi i/5),\sin(2\pi i/5)),
 \qquad Q=\{p:n_i\cdot p\le h,\ 0\le i<5\}.
\]
Thus \(Q\) has circumradius one. Define the regular decagon
\[
 R=\frac{1}{2(1+h)},\qquad
 B=\operatorname{conv}\{\pm R n_i:0\le i<5\}.
\]
Its orientation is fixed relative to \(Q\): its vertices lie on the normal
lines of the pentagon. If \(K\) is centrally symmetric with centre \(z\), set
\[
 \lambda_B(K)=\max\{r\ge0:z+rB\subseteq K\}.
\]
This is also the largest scale of a translated copy of \(B\) contained in
\(K\). Indeed, symmetry and convexity move any such copy to \(z\) without
changing its scale.

**Theorem.** For every centrally symmetric planar convex body \(K\) with
nonempty interior,
\[
 c_{\mathrm{EHZ}}(K\times_L Q)=\lambda_B(K),\qquad
 \frac{c_{\mathrm{EHZ}}(K\times_L Q)^2}
      {2\operatorname{area}(K)\operatorname{area}(Q)}
 \le\frac{10+6\sqrt5}{25}<1.
\]
Equality in the ratio holds exactly for translates and positive dilates of
\(B\), with the specified orientation.

Here \(K\times_L Q\) is the Lagrangian product: \(K\) lies in the position
plane and \(Q\) in the momentum plane. Write \(c\) for its Ekeland–Hofer–Zehnder
capacity.

## The geometric input

For an oriented polygonal curve with edge vectors \(v_j\), its \(Q\)-length is
\(\sum_j s_Q(v_j)\), where \(s_Q(v)=\max_{p\in Q}v\cdot p\).
This can depend on the direction of traversal. The
[translation-cover characterization of Balitskiy–Mitrofanov–Polyanskii,
Theorems 2.5 and 3.3](https://arxiv.org/html/2603.12495v2#S3.SS2), says that
\(c(K\times_L Q)\ge1\) exactly when all closed curves of unit \(Q\)-length
translate into \(K\). For polygonal \(Q\), it suffices to check its *normal
triangles*: unit-length triangles whose oriented edges follow outward facet
normals. Degenerate triangles are included when opposite normals occur.

For our pentagon, the normal triangles have angles
\(\theta,\theta,3\theta\). To see their shape, the three edge directions must
surround the origin. Their cyclic gaps are therefore less than \(\pi\),
hence are \(2\theta,4\theta,4\theta\); the triangle's angles are their
supplements. There are five choices of the direction triple and two traversal
orders, giving ten translation classes. Their longest sides follow pentagon
normal lines. The triangles have fixed orientations: each must fit by
translation alone.

## Proof

Translate \(K\) so that \(K=-K\).
First consider a segment traversed out and back along \(n_i\). Because
\(s_Q(n_i)=h\) and \(s_Q(-n_i)=1\), the segment joining
\(-R n_i\) to \(R n_i\) has \(Q\)-length
\[
 2R(h+1)=1.
\]
If \(c(K\times_L Q)\ge r>0\), apply the curve-cover characterization to
\(K/r\). Each segment \([-rRn_i,rRn_i]\) translates into \(K\).
A segment symmetric about zero fits into the symmetric convex body \(K\)
only if its centred copy already lies in \(K\): average a fitting translation
with its negative. Taking the convex hull of these five segments gives
\(rB\subseteq K\). Consequently
\[
 c(K\times_L Q)\le\lambda_B(K).
\]

We now show that \(B\) itself has capacity at least one. A normal triangle
with longest side \(L\) has two other sides of length \(L/(2h)\). Its
oriented edges follow outward normals, so its \(Q\)-length is \(h\) times
its Euclidean perimeter. Its normalization therefore reads
\[
 1=h\bigl(L+L/h\bigr)=L(1+h),\qquad L=2R.
\]
Translate the midpoint of this side to zero. Its endpoints become two opposite
vertices of \(B\). The third vertex lies at distance \(R\tan\theta\) from
zero, strictly inside the incircle of \(B\), whose radius is
\(R\cos(\theta/2)\), because
\[
 \tan^2(\pi/5)=5-2\sqrt5
 <\frac{5+\sqrt5}{8}=\cos^2(\pi/10).
\]
Convexity now puts the whole triangle in \(B\). This placement works for
every normal triangle, so the covering theorem gives \(c(B\times_L Q)\ge1\).
The upper bound already proved gives equality.

Finally, \(\lambda_B(K)B\subseteq K\). Capacity monotonicity and homogeneity
in the first factor yield
\[
 c(K\times_L Q)\ge\lambda_B(K)c(B\times_L Q)=\lambda_B(K).
\]
This proves the capacity formula. The same containment gives
\(\operatorname{area}(K)\ge\lambda_B(K)^2\operatorname{area}(B)\).
Since
\[
 \operatorname{area}(B)=5R^2\sin\theta,\qquad
 \operatorname{area}(Q)=\tfrac52\sin(2\theta),
\]
the sharp ratio is
\[
 \frac{1}{2\operatorname{area}(B)\operatorname{area}(Q)}
 =\frac{10+6\sqrt5}{25}.
\]
A proper inclusion between full-dimensional convex bodies strictly increases
area. Thus equality forces \(K=\lambda_B(K)B\) after translation; these
partners also attain equality. \(\square\)

## What transfers beyond this example

The proof separates a reusable mechanism from the pentagon calculation.
For a planar convex body \(Q\), form
\[
 B_Q=\tfrac12(Q-Q)^\circ,
\]
where \(A^\circ=\{x:x\cdot a\le1\text{ for all }a\in A\}\) denotes the polar.
The out-and-back segments of unit \(Q\)-length have centred convex hull
\(B_Q\). For every symmetric \(K\), they give the upper bound
\(c(K\times_L Q)\le\lambda_{B_Q}(K)\).
If one proves the single calibration \(c(B_Q\times_L Q)=1\), monotonicity
makes this an equality for **every** symmetric partner. Conversely, equality
for every symmetric partner would include this calibration. The substantive
geometric task is therefore whether this particular body covers the required
curves; defining it alone does not settle that task.

For the pentagon, this also recovers the difference-body formula
\[
 c(K\times_L Q)=
 \max\{r\ge0:r(Q-Q)^\circ\subseteq K-K\}
 \quad\text{when }K\text{ is centrally symmetric}.
\]
For an arbitrary partner \(K\), a segment translates into \(K\) exactly when
its displacement lies in \(K-K\). Thus the same maximum remains an upper bound
for \(c(K\times_L Q)\). It equals the capacity of the central symmetrization
\(S=(K-K)/2\), since \(S-S=K-K\). Equality for asymmetric \(K\) does not follow.
Thus the theorem gives an exact symmetric comparison class and isolates the
additional triangle-covering obstruction that asymmetry can introduce.

The [odd-regular extension](odd-regular-symmetric-partner.md) proves the same
calibration for every odd regular polygon with at least five sides. It requires
additional triangle estimates; the pentagon proof above is the short selected
instance. The triangle case does not satisfy that calibration. None of these
statements proves global HKO optimality among arbitrary asymmetric partners.

---

**Source and status.** This exposition reorganizes the maintained
[symmetric-partner proof](symmetric-partner-lemma.md) and its
[3 October exposition candidate](../../../docs/history/symmetric-partner-candidate-2026-10-03/README.md).
The imported covering criterion was checked against the linked BMP v2 source
on 19 October 2026; the remaining displayed argument is included above.
Jörn selected the pentagon theorem for inclusion
([project facts, item 13.2](../../../docs/project-facts.md)); this file remains
outside the selected manuscript. Agent checking, human proof/prose acceptance,
and publication novelty are separate questions. The
[bounded literature check](source-check.md) does not establish novelty.
