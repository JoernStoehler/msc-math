# Jörn's Thesis Narrative Notes (2026-04-07)

Raw notes from Jörn, lightly formatted. Source of truth for thesis story arc.

## Thesis stale state

The thesis/ is rather stale. First major work type: consider how to structure it, move content around, then refactor since many definitions changed:
- We no longer use (n,h) but just dual vertices (a) — simpler math. "Unit length" just isn't a symplectic property, so no wonder it is never used anywhere.
- Sign convention for Lagrange multipliers changed to give symmetric matrices — worth propagating.
- Order in which we apply simplification theorems to HK2017 changed. Previously there were some bugs/misunderstandings around whether we are still finding a minimum action Reeb orbit, even if we restrict to a subset of sigma and reject Q,beta values with beta_k=0. Turns out: first drop the beta_k=0 redundant boundary values, THEN drop the impossible sigma — because now we can use that all dwell times are >0, and before that we couldn't.

## Conclusion / narrative (crystallized but can still change)

**Main conjecture:** HKO2024 is a local maximum of the systolic ratio.

**Empirical arguments:**
1. Random sampling in the neighborhood of F=10 polytopes (~R^40) and random sampling in the neighborhood of 5x5 Lagrangian products (~R^20) yielded no higher sys.
2. The first-order subdifferential in R^40 is <= 0 in all directions, with an (empirical) 15-dim space of directions where D_d sys = 0.
3. We randomly(?? Jörn doesn't recall exactly) sampled in this 15-dim subspace and the second-order change of sys was also <= 0.
4. The family P_5 x_L R(theta) P_5 takes a maximum at the HKO2024 angle.

## Stronger conjecture (believed less strongly)

HKO2024 may even be the global maximum, or may be (up to perturbations and symplectomorphisms) the only sys>1 case. Main evidence: we failed to find any other sys>1 polytope that isn't just close to HKO2024.

A BOTEC based on empirical sampling shows the sys>1 region around HKO2024 in the 5x5 Lagrangian product space is very small in volume relative to basically any non-targeted search measure. Random sampling is unlikely to even hit the known HKO2024. Gradient ascent converges to local optima with sys<1 usually.

**Conclusion on the landscape:** The sys landscape is mostly lots of local optima with sys<1 and small attractor volumes. The only (family of) sys>1 local maximum we know has a tiny attractor volume. This is the kind of dataset where most standard data science methods fail. We'd need structural insight into what makes a polytope have high sys (or sys>1 in particular), and then look systematically.

## Structural insights

1. **Regular polytope products** have a degenerate minimum-action Reeb orbit set, so subdifferentials can easily be <= 0 in all directions. Looking for max_theta sys(P_m x R(theta) P_n) yielded mostly sys<1 and a few sys=1 cases.

2. **[TODO for Kai]** Finding a general formula even for this simple family looked too difficult so we didn't try. We only did so for 5x5 [TODO! this is something Kai asked for but hasn't been done yet] — and even that involved by-hand discussion of what Reeb orbit has minimum action for any given theta, guided by empirical computations for isolated theta.

3. **Performance improvements:** We improved performance dramatically over previous systolic ratio computations. But this doesn't help when the density of sys>1 cases is exponentially small wrt the dimension of the polytope space (dim = 4F in general; dim = 2F for Lagrangian product spaces).

4. **Variable-F optimization:** We investigated how increasing the number of facets may change the attractor space due to increased flexibility. Found insufficient returns: while local maxima in F-space often aren't local maxima in (F+1)-space, they belong again to a relatively small attractor that is only marginally higher in sys.

**Conjecture:** The gradient ascent process we use, and the +1 facet increase method, actually converges most often to the local maxima of the systolic ratio in the space of smooth strictly convex bodies, which are per [HK-something, Zoll property iirc] symplectomorphic to a ball B^4(r), which has systolic ratio = 1. "Most often" is needed since we also conjecture HKO2024 is a local maximum in the space of not-strictly-convex not-necessarily-smooth bodies, so even for large F the gradient/subdifferential points towards HKO2024. Probably tiny volume again — we didn't find for moderate F and random start points any convergence to a sys>1 maximum (and not HKO2024 in particular).

## Evaluation criteria for experiments

With the finish line in sight, evaluate each experiment against:
1. Does it serve the finish line?
2. What to polish?
3. How to get even stronger evidence for an even better conclusion?
4. How to present our work better to the advisors?
5. What to paranoidly check so we don't say something wrong?
6. How to present all the proofs / how to show conjectures that so far were (luckily) used only as heuristics?
