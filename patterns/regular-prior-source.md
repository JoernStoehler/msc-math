# Recovering the remembered sub-one regular-product ridge relation

Source search, 17 October 2026. Receiving owner: thesis owner; this note is a
bounded source recovery, with no new capacity experiment or literature search.
The trigger is Jörn's pm answer `c71417e1` to question `92fd833a`: he recalls
being told of a rotated regular-polygon product whose minimum total symplectic
two-face area has maximum systolic ratio below one, but knows no side counts or
source and offers to retrieve the Astra 6 Pro chat if needed.

## Finding

The repository contains enough mathematics to supply **two concrete examples
matching that description, 7×7 and 5×7**, but the focused search did not locate
an original passage attributing their ridge-sum relation to the remembered
conversation. This resolves the mathematical-control choice without resolving
which example or chat Jörn remembers. The existing 5×5 relation alone does not
match the “maximum below one” qualification.

| Family | What is already explicit in maintained sources | Match to recollection |
| --- | --- | --- |
| Rotated regular 5×5 | [Data chapter](../thesis/chapters/08-data-science.tex), equation `eq:ds-pentagon-ridge-capacity`: $\operatorname{sys}R^2=16(3+\sqrt5)$; minimum ridge sum gives maximum ratio $(3+\sqrt5)/5>1$. | Exact inverse relation, but its family maximum is above one. Restricting to a sub-one interval does not identify a new full-family maximum. |
| Independent affine regular 5×5 | [Maximum-ridge report](ridge-functionals/REPORT.md): $\operatorname{sys}R_\infty^2=8/25$. | Uses the maximum individual ridge, not the total ridge sum; the affine family again includes above-one HKO. |
| Rotated regular 7×7 | [Maintained product note](../topics/products/pentagon-affine-products/research.tex), `eq:77` and Section “Heptagons and unequal regular factors”: exact secant profile, maximum approximately $0.9174081964$. | Full-family maximum below one; the elementary ridge sum below supplies the matching inverse relation. |
| Rotated regular 5×7 | Same source, `eq:57`: exact secant profile, maximum approximately $0.9106527968$. | Another full-family maximum below one, with the same ridge-sum mechanism. |
| Rotated regular 3×3 | Data chapter, `eq:ds-triangle-profile`: ridge sum and ratio both decrease with the fundamental angle, with ratio $2/3$ to $1/2$. | Opposite ordering. |
| 4×4 ridge minimum / regular 3×6 endpoint | [Endpoint derivation](../topics/empirical-studies/sys-datascience/methods/ridge-endpoint-path/notes/endpoint-predictions.md): ridge minima have ratios $1/2$ and $3/4$; [3×6 interval result](../topics/empirical-studies/sys-datascience/methods/ridge-symmetry-completion/MATHEMATICAL-HANDOFF.md) reaches ratio one at the other endpoint. | These are not minimum-ridge / maximum-ratio examples. |

Here $R=\sum_F|\int_F\omega_0|/\sqrt{\operatorname{vol}_4K}$ and
$R_\infty=\max_F|\int_F\omega_0|/\sqrt{\operatorname{vol}_4K}$.
Within a rotation family volume is constant, so normalized and unnormalized
ridge sums select the same angles.

## Why the two sub-one controls fit

This paragraph is a source-local algebraic deduction, **not a recovered
quotation or a new capacity theorem**. Use the normal-oriented regular polygons
and angle convention of the maintained note. Put

$$
 A_n=\frac n2\sin(2\pi/n),\quad \ell_n=2\sin(\pi/n),\quad
 h_n=\cos(\pi/n),\quad L=\operatorname{lcm}(n,m),\quad
 d_L=\operatorname{dist}(\theta,(\pi/L)\mathbb Z).
$$

The edge formula in the data chapter gives, for odd $n,m$,

$$
 R(P_n\times_LR_\theta P_m)=C_{nm}\cos d_L,\qquad
 C_{nm}=\frac{\ell_n\ell_m}{\sqrt{A_nA_m}}
          \frac{nm}{L}\csc\frac{\pi}{2L}.
$$

Indeed the edge-angle differences run through all $L$ phases $2\pi k/L$,
each $nm/L$ times. Their absolute-cosine sum is
$\sum_{k=0}^{L-1}|\cos(\theta+2\pi k/L)|
=\csc(\pi/(2L))\cos d_L$ for odd $L$; pair the positive and negative terms
on $|\theta|\le\pi/(2L)$ and sum the finite geometric series. This is the same
root-sum mechanism spelled out for five sides in the data chapter.

For the **proved** cases $(n,m)=(7,7),(5,7)$, the existing capacity formulas are
$c=(1+h_n)(1+h_m)\sec d_L$. Consequently

$$
 \operatorname{sys}R^2=
 \frac{(1+h_n)^2(1+h_m)^2C_{nm}^2}{2A_nA_m}.
$$

Minimum total ridge area and maximum ratio therefore occur exactly at
$d_L=\pi/(2L)$: relative angles $(k+1/2)\pi/7$ for 7×7 and
$(k+1/2)\pi/35$ for 5×7 in this normal convention. A vertex-oriented producer
can shift its reported angle origin. The below-one maxima are already stated
in the maintained note; their proofs and sparse-reduction dependencies are
imported here, not independently re-audited. This deduction does not extend
the capacity formula to arbitrary odd side counts or arbitrary polygons.

## Attribution and what a chat excerpt would resolve

The [affine-product README](../topics/products/pentagon-affine-products/README.md)
explicitly attributes the affine argument and angular extensions to a ChatGPT
Pro return delivered by Jörn on **18 September 2026**. The original bytes are
[received/math/research.tex](../topics/products/pro-handoffs/pentagon-return/received/math/research.tex),
with independent audits linked by the [handoff owner](../topics/products/pro-handoffs/README.md).
The maintained proof and original return give the 7×7 and 5×7 capacity profiles,
but no explicit total-symplectic-ridge-area discussion was found there. The
retained provenance does not identify that return as Astra 6 Pro. The 17 October
patterns packet separately derives the 5×5 maximum-ridge bridge and sum law.
The selected data chapter also already states the 5×5 sum law explicitly.
No attribution of either ridge deduction to Jörn's remembered chat follows.

A concrete optional request is warranted **if identifying that original result
matters**: ask for the short Astra 6 Pro passage containing the side counts and
the minimum-area / maximum-ratio claim, ideally with its formula and whether
“area” means sum or maximum. It is not a mathematical blocker: 7×7 and 5×7
already provide the requested sub-one controls. Do not ask Jörn to repeat the
side-count question he has just answered with “I do not remember.”

## Reproducible search scope and limits

Read entry routes: root README and research guide; products README;
regular-products README and rotated-sweep README; affine-product README and
source contract; Pro-handoff README and prepared-packet provenance; empirical
study and sys-datascience READMEs; patterns owner follow-up and ridge report.
Focused content reads: maintained and original-return affine proof sections on
triangles/heptagons, data chapter exact-geometry section, endpoint-path status
and prediction derivations, 3×6 mathematical handoff, ridge-tail anatomy README,
and the routed legacy pentagon proof's search matches/status header.

Reproduce the text search from the patterns worktree root (no producers):

```sh
rg -n -i '(regular|polygon|pentagon|heptagon|rotation|rotated).{0,100}(ridge|two.face|symplectic.area)|(ridge|two.face|symplectic.area).{0,100}(regular|polygon|pentagon|heptagon|rotation|rotated)|lowest.{0,60}(area|sum)|minimi.{0,50}(ridge|symplectic)' topics/empirical-studies/sys-datascience topics/products patterns thesis/chapters formal/FINDINGS.md --glob '*.md' --glob '*.tex' --glob '*.py' --glob '*.rs' --glob '!*/target/*'
rg -n -i 'ridge|symplectic.area|two.face' topics/products/pro-handoffs topics/products/pentagon-affine-products --glob '*.md' --glob '*.tex' --glob '*.txt'
```

These are searches of named maintained routes and their directly relevant
provenance, not an exhaustive search of Git history, every experiment output,
all literature or private chat history. No chat access was available or used;
no new numerical experiment, Sage run, capacity evaluation or web search ran.

[review: /root/batch_review]
