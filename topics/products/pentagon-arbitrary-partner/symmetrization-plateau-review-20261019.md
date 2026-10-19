# Independent plateau and symmetric-exposition check

Checker: `/root/plateau_check`, 2026-10-19, initial bounded check.
Receiving owner: `/root`; researcher: `/root/partner_geometry`.
Read-only project check; only this scratch packet is checker-owned. No running
subprocesses or child agents were created. Parent owns integration and cleanup.

## Plateau verdict

PASS on the stated mathematical implication, conditional on the imported BMP
covering theorem. No material gap found. This is an independent agent check,
not human proof acceptance, a sharp-threshold proof, or a novelty judgment.

For every full-dimensional compact planar convex body K, fix the regular
circumradius-one pentagon Q with outward normals n_i and h=cos(pi/5). Put

```
B = conv{±n_i/[2(1+h)]}, S=(K-K)/2,
W=max{r>=0:r(Q-Q)^polar ⊆ K-K}.
```

Then the proposed conclusion is

```
c_EHZ(((1-t)K+t(-K)) ×_L Q)=W
for (3-sqrt(5))/2 <= t <= (sqrt(5)-1)/2.
```

### Imported scope actually checked

Primary source read directly on the web: Balitskiy–Mitrofanov–Polyanskii,
*Triangle covering problems and the Viterbo inequality in the plane*,
[v2](https://arxiv.org/html/2603.12495v2), Definition 3.2, Theorems 2.5 and
3.3, and Section 6.1. Also cross-checked those passages in v1. The criterion
uses unit support-function perimeter and translations of oriented normal
triangles. For the regular pentagon the complete list has ten translation
classes, all with angles pi/5, pi/5, 3pi/5, related by pi/5 rotations.
There are no degenerate normal templates, because no two pentagon facet
normals are opposite. Unit out-and-back segments in other directions remain
available through Theorem 2.5 for the upper bound.

### Independent implication chain

1. The difference body Q-Q has outward facet normals ±n_i and height 1+h.
   Thus B=(Q-Q)^polar/2 and W B ⊆ S. Full dimension and compactness make
   W positive, finite and attained.
2. Every normalized normal triangle has longest edge length
   L=1/(1+h)=2R, where R=1/[2(1+h)]: its other edges have lengths L/(2h),
   and every oriented edge has support length h times its Euclidean length.
3. Center the longest edge at zero. Its endpoints are antipodal vertices
   ±d_0/2 of B. Its apex v is perpendicular to that edge, with
   |v|=R tan(pi/5). Since the inradius of B is R cos(pi/10),

   ```
   v ∈ eta B, eta=tan(pi/5)/cos(pi/10)=3-sqrt(5).
   ```

   The equality for eta follows from
   tan(theta)/cos(theta/2)=2 sin(theta/2)/cos(theta), with
   sin(pi/10)=(sqrt(5)-1)/4 and cos(pi/5)=(sqrt(5)+1)/4.
   This is valid for every template orientation; no rotation of K is used.
4. For 0<=t<=1/2 let s=1-2t. Convexity gives the exact set identity

   ```
   K_t=(1-t)K+t(-K)=sK+(1-s)S.
   ```

   In a W-scaled template, its longest-edge vector d=W d_0 lies in
   2W B ⊆ K-K. Select a,b in K with d=a-b and put m=(a+b)/2 in K.
5. Translating this entire scaled template by s m places its edge endpoints at

   ```
   s m+d/2 = (1-t)a-t b ∈ K_t,
   s m-d/2 = (1-t)b-t a ∈ K_t.
   ```

   These expressions use a point of K and a point of -K with coefficients
   1-t and t. They should not be described merely as convex combinations of
   a and b lying in K_t.
6. Its apex is s m+W v. Because Wv∈eta W B⊆eta S and 0∈S,
   eta<=1-s=2t implies Wv∈(1-s)S. Thus the apex lies in K_t.
   Convexity covers each full triangle. The covering criterion on K_t/W
   gives capacity at least W, including the endpoint t=eta/2.
7. Minkowski subtraction gives K_t-K_t=K-K for every t∈[0,1]. An
   out-and-back segment with displacement u has length h_{Q-Q}(u), and
   translates into K_t precisely when u∈K_t-K_t. Theorem 2.5, after
   scaling, therefore bounds capacity by W. This step requires no disputed
   identification with a repository implementation's billiard class.
8. For t>=1/2, repeat steps 4–6 with K'=-K and t'=1-t. Its interpolated
   body is exactly K_t and W(K')=W(K). This proves the upper half of the
   interval without needing a separate anti-symplectic-invariance citation.

Translations of the original K cause K_t and the fitting translations to
translate by s times the original shift, so no centering assumption was hidden.
The proof establishes a universal guaranteed interval; some partners may
reach the segment bound earlier. Its endpoints are not proved optimal here.

## Exposition verdict (separate)

PASS for the main displayed pentagon theorem and its assembled proof in
`/tmp/msc-math/research-swarm-20261019/symmetric-writeup/exposition.md`, as read
during this check. No mathematical correction required.

Independently checked the five-segment upper bound, recentering of translated
symmetric segments by convexity, normal-template count and normalization,
incircle placement, first-factor homogeneity, strict-area equality condition,
and the sharp constant (10+6sqrt(5))/25. The general calibration equivalence
for B_Q=(Q-Q)^polar/2 is valid: all centered unit out-and-back segments have
that body as convex hull, and equality for all symmetric K is equivalent to
the single calibration c(B_Q×_L Q)=1. General asymmetric K retains only the
segment upper bound and the symmetric-comparison interpretation.

The referenced odd-regular extension was inspected to determine the stated
scope. Its full proof and the linked triangle countercalibration were not
re-audited in this bounded exposition check. The exposition appropriately
treats novelty, thesis inclusion, agent checking, and human acceptance as
separate status questions.

## Access limits

Local sources inspected include root README/AGENTS/commentary, collaboration
rationale, research guide and product entry material, the symmetric-partner
lemma/source check, the odd-regular note, and the exact scratch exposition.
This is neither exhaustive literature search nor repository-wide validation.
