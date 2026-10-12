# Local persistence of the symmetric-partner capacity formula

3 October 2026 UTC. Agent-derived deduction, independently checked by
`research_choices` for closure stability, the support identity, template
placements and traversal orders. No human acceptance, novelty assessment or
thesis selection is established.

## Statement and topology

Fix an odd `n>=5` and a regular n-gon `Q_0`. Within convex polygons with `n`
genuine, cyclically labeled facets, there is an open neighborhood `U` of `Q_0`
such that every `Q` in `U` satisfies

```text
c_EHZ(K ×_L Q)=max{r>=0:r(Q-Q)^polar⊆K-K}
```

for every centrally symmetric planar convex body `K` with nonempty interior.

The neighborhood is defined in the finite-dimensional facet-normal/support
parameter space. It may depend on `n`; no explicit radius or uniform-in-n
neighborhood is claimed. This is not openness in the space of all convex bodies.
Translations and nonsingular linear images of these neighborhoods are also
covered by the symplectic coordinate change in the preceding theorem.

For `B_Q=(Q-Q)^polar/2`, the sharp ratio at fixed `Q` is

```text
max_K sys(K ×_L Q)=1/[2 area(B_Q)area(Q)],
```

with equality precisely for translates and positive dilates of `B_Q`.
After shrinking `U`, this value is strictly below one, because it varies
continuously and is below one at `Q_0`.

## Stable templates and strict placements

Choose continuously varying unit outward normals `nu_i` and support heights.
At `Q_0`, no two normals are parallel. This remains true locally, and the list
of positive three-normal closures stays fixed: a closure coefficient can change
sign only by passing through zero, which would leave two opposite normals.
For each positive triple, its normalized normal triangle varies continuously.
Facet genuineness and the cyclic labeling are preserved by the chosen topology.

The unified regular-polygon proof in `odd-regular-symmetric-partner.md` places
every non-extreme template strictly inside the incircle of `B_Q0`.
The placements of these finitely many templates persist under sufficiently
small changes of `Q`.

The remaining extreme templates have cyclic normal gaps
`(1,(n-1)/2,(n-1)/2)` at `Q_0`. Each uses adjacent normals `nu_j,nu_k` whose
shared vertex cone contains `-nu_i` strictly. This adjacency and strict cone
containment persist. These boundary-contact templates require an exact support
identity; mere continuity of an inclusion would be insufficient.

## Exact antipodal contact for the extreme templates

Write

```text
-nu_i=a_j nu_j+a_k nu_k,    a_j,a_k>0.
```

Because `nu_j,nu_k` expose the same vertex of `Q`,

```text
s_Q(-nu_i)=a_j s_Q(nu_j)+a_k s_Q(nu_k).
```

The closing triangle with directed steps proportional to
`nu_i,a_j nu_j,a_k nu_k` has total support-function length
`s_Q(nu_i)+s_Q(-nu_i)=s_(Q-Q)(nu_i)` before rescaling.
Its unit-length normalization therefore gives the designated side vector

```text
u=nu_i/s_(Q-Q)(nu_i).
```

Center that side at zero. Its endpoints are exactly `±u/2`, the corresponding
antipodal vertices of `B_Q`; the difference body retains the genuine facet
normals `±nu_i`. The third vertex was strictly interior at `Q_0` and varies
continuously, so it stays interior after shrinking the neighborhood.
Convexity places the entire triangle in `B_Q`.

For closing vectors `u+v+w=0`, reversing the cyclic edge order replaces
`{0,u,u+v}` by `u-{0,u,u+v}`. Thus central inversion and translation cover
the other order as well, since `B_Q=-B_Q`.

## Capacity transfer

Every normalized normal triangle now translates into `B_Q`. The imported
[BMP translation-cover criterion](https://arxiv.org/html/2603.12495v2),
Theorems 2.5/3.3, gives `c(B_Q ×_L Q)>=1`. The universal segment bound gives
`c(B_Q ×_L Q)<=W_Q(B_Q)=1`.

For centered symmetric `K`, `W_Q(K)B_Q⊆K`; monotonicity and one-factor
homogeneity give the exact formula. Area containment gives the fixed-Q sharp
ratio and its equality class. This uses the same calibration transfer as the
pentagon proof, with the exact endpoint identity replacing its rigid symmetry.

## Consequence and boundaries

The calibration method therefore covers nonregular polygon factors in open
families around every odd regular n-gon of at least five sides. It provides an
infinite symmetric-partner control class for each such factor.
It neither establishes the formula for arbitrary polygon factors nor settles
Viterbo's inequality for arbitrary asymmetric partners.
