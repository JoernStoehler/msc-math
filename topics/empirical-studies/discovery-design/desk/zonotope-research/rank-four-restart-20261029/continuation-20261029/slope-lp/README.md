# Fixed-C operator-extension discriminator — paused at user wrap-up

29 October 2026. Owner `/root/alternative_mechanism`; root owns integration,
commits and cleanup. The user required immediate wrap-up at 18:52 UTC. No
producer for this discriminator was launched, and no numerical or exact result
from this proposed LP is claimed. This note preserves the concrete next test.

## Target and inherited evidence

For a nonzero decomposable skew matrix `C=g wedge h`, let

```
Phi(W)=sum_(i<j)|W_ij|,
c=sum_(i<j)|C_ij|,
B(W,C)=sum_l sum_(i<j<k)|W_ij C_kl-W_ik C_jl+W_jk C_il|.
```

The proposed slope inequality is `B(W,C)<=c Phi(W)` whenever `W wedge C=0`.
The [projection-slope note](../projection-slope.md) owns its geometric
interpretation and unresolved status. The
[Lagrangian-addition note](../lagrangian-addition.md) supplies a structured
known-equality input and conventions; its equality does not prove this bound
for arbitrary W or C.

Before this assignment, the rank-four worker ran twelve fixed-C numerical
sign-ascent probes, with eight starts each, and reported a largest ratio about
0.99219243. Those finite probes are not a universal fixed-C certificate. Their
scratch producer and data were read only:

```
/tmp/msc-math/viterbo-continuation-20261029/rank-four/reduction/slope_lp.py
/tmp/msc-math/viterbo-continuation-20261029/rank-four/reduction/slope-lp-results.json
```

The present worker did not repeat that sample or independently replay its
values. Its intended next step differs: one LP sufficient to certify **every**
W for one fixed C, without enumerating output sign patterns.

## A sufficient operator-extension LP

Index the N=`binomial(n,2)` independent entries of W as a vector w. Let M have
full column rank and image

```
S_C={w:W wedge C=0}={g wedge r+h wedge s:r,s in R^n}.
```

For C nonzero, this space has dimension `2n-3`. Indeed, choose an algebraic
basis whose first two vectors are g,h. The equation `W wedge g wedge h=0`
sets every coefficient of W with both indices outside those two vectors to
zero. A practical rational M can be selected from the independent columns of
the explicit map `(r,s)->g wedge r+h wedge s`.

Let T be the matrix taking w to the n*binomial(n,3) entries in the sum defining
B. Thus `B(W,C)=||T M u||_1` when `w=M u`.

Duplicate and proportional rows of T M can be compressed exactly. If rows
`a_i r` share a common normalized rational row r, replace them by the single
row `(sum_i |a_i|)r`; discard zero rows. Call the resulting matrix H. Then
`||H u||_1=||T M u||_1` for every u.

Seek a matrix A, nonnegative entrywise bounds U, and scalar lambda satisfying

```
A M = H,
-U_ke <= A_ke <= U_ke,
sum_k U_ke <= lambda       for every input edge e.
```

Minimize lambda. This is a finite linear program with rational coefficients
when g,h are rational. A feasible solution with `lambda<=c` proves the desired
inequality for **all** W in S_C, since

```
||T M u||_1=||A M u||_1 <= lambda ||M u||_1 <= c Phi(W).
```

A retained rational certificate would need only exact multiplication `AM=H`
and the column sums `sum_k |A_ke|<=c`; solver tolerances would not be proof.
No implementation or certificate for this LP was completed before wrap-up.

This is a sufficient extension condition, potentially stronger than the
original restricted-domain operator bound. A computed optimum above c would
only reject this particular ambient-extension certificate route; it would not
refute the slope inequality. Infeasible or inaccurate numerical output would
need separate diagnosis.

## Prepared structured input and next discriminator

The first intended input was the eight-index equality example from the
Lagrangian-addition note. Its rotated projected generators are

```
q_i=(0,0),(4,1),(-1,2),(-1,2),(0,3),(0,0),(-3,0),(3,0).
```

Thus one may take

```
g=(0,1,2,2,3,0,0,0),
h=(0,-4,1,1,0,0,3,-3),
C=g wedge h.
```

These follow the convention `q_i=(-h_i,g_i)` in that note. Repeated and zero
projected directions should make row compression effective, but no row count
or runtime was measured. A subsequent rational perturbation of those repeated
and zero directions was contemplated, not run.

The next concrete decision is whether this structured input admits an exact
extension with lambda<=c. If it does, inspect A for a reusable formula before
sampling more C. If it does not, preserve an exact obstruction to the extension
LP, keep the original slope inequality open, and compare with the geometric
worker's approach. No future execution is authorized by this note alone.

## Stop receipt and limits

No new process or PID was created for this discriminator, and no child agent
was spawned. All earlier pentagon producers and certificate replays had already
completed; that completed packet is retained separately under
`topics/capacity-and-qp/research-continuation-20261029/alternative/`.
The present LP formulation has not received an independent mathematical or
implementation review. It makes no global rank-four or EHZ claim.
