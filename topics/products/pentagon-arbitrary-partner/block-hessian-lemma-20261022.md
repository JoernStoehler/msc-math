# Block-Hessian inertia for the 10-triangle translation hull

A bounded strengthening of the existing smooth-cell reduction, derived 22 October 2026. This is an exact lemma for a fixed hull-order cell and affine slices of that cell. It does **not** classify the nonlinear contact strata, so it is not a reduction of the global problem to dimension eight.

## Proposition

Fix one labelled counterclockwise hull cycle of the thirty placed triangle vertices. Anchor `t_0=0`; write the other nine translations as `t_1,...,t_9 in R^2`. On this hull-order cell the hull area is a quadratic polynomial `F(t)` whose Hessian has equally many positive and negative eigenvalues. In particular, its positive index is at most 8.

Let `L` be any affine subspace of translation space of dimension `d`, and consider a point in the relative interior of a region where this same hull-cycle formula holds. If that point is a local minimum of `F` restricted to `L` and `d>8`, then it is not isolated: there is a nonzero direction in `L` along which `F` is exactly constant. Consequently, in a compact feasible region within the affine slice, such a minimum can be moved to the boundary of that piece or to the boundary of the hull-order cell.

## Proof

Write each labelled hull vertex as `p_i=t_{j_i}+v_i`, where `v_i` is the corresponding fixed vertex of its template, and set `t_0=0`. The shoelace formula is

`F(t) = (1/2) sum_i det(p_i,p_{i+1})`.

Its quadratic homogeneous part only comes from terms `det(t_{j_i},t_{j_(i+1)})`. Terms with the same translation label vanish because `det(t_j,t_j)=0`. Grouping terms by unordered label pairs gives

`Q(t) = sum_{j<k} c_{jk} det(t_j,t_k)`,

where `c_{jk}=-c_{kj}`; equivalently `C=(c_{jk})` is a real skew-symmetric `9 x 9` matrix. Thus the full Hessian is, up to an irrelevant overall sign/convention, `C tensor J`, with `J=[[0,-1],[1,0]]`. Orthogonally put `C` into its standard skew blocks `[[0,lambda],[-lambda,0]]` plus zeros. Each nonzero block tensored with `J` has two positive and two negative eigenvalues of equal magnitude. Therefore the Hessian has equal positive and negative indices, each equal to `rank(C)`, and `rank(C)<=8` because a skew matrix of odd order has rank at most eight.

Restrict to `L`, parameterized by `t=t_*+Uz` with `U` full column rank and `z in R^d`. The restricted Hessian is `U^T H U`; its positive index cannot exceed that of `H`, hence is at most 8. At a relative-interior local minimum this restricted Hessian must be positive semidefinite. If `d>8`, it therefore has a nonzero kernel vector `w`. The restricted objective is quadratic, and its first derivative in `w` vanishes at the local minimum. Its exact change along `sw` is consequently zero for every `s` for which the same formula remains valid. This gives the stated flat direction and boundary move.

## Scope and use

This strengthens the established open-cell fact that a full-dimensional interior local minimum forces the Hessian to vanish: it shows that any *affine* contact slice of dimension above eight still cannot contain an isolated local minimum in its relative interior. Thus a contact proof can discard high-dimensional affine pieces by moving their minima to more contacts or to cell boundaries.

The hull-order cells themselves are generally described by determinant inequalities involving three different translation blocks. Their contact boundaries can be nonlinear bilinear varieties. The argument above does not apply to a curved contact stratum merely by replacing its tangent space for `L`: the second derivative of the restriction includes the curvature of that stratum. Therefore it does not prove that every global minimizer lies on a contact stratum of dimension at most eight, and it does not improve the unconditional 30-vertex/18-variable global reduction. A next proof must either show the relevant contact equations are affine after a suitable stratification, or control their curvature terms.

## Smooth contact-stratum version (second-order statement only)

A related necessary condition applies to regular contact strata. A contact equation for three labelled hull points is

`g(t)=det(p_b-p_a,p_c-p_a)=0`.

After substituting `p_i=t_{j_i}+v_i`, its quadratic part is again a sum of terms `d_{jk} det(t_j,t_k)` with skew coefficient matrix `D=(d_{jk})`; terms with repeated labels cancel to at most linear terms. Coincidence contacts are linear. Hence the Hessian of every smooth contact equation belongs to the same block-skew space `C tensor J`.

Suppose a local minimum lies on a smooth active-contact stratum of dimension `d`. Restrict to one incident hull-cycle formula; it agrees with the area on that stratum. If the active equations satisfy LICQ, equality-constrained first- and second-order necessary conditions apply to this smooth restriction. Its Lagrangian Hessian is a linear combination of that hull-area formula's Hessian and the contact-equation Hessians, so it too has balanced positive/negative inertia and positive index at most 8. Its restriction to the `d`-dimensional tangent space must be positive semidefinite. If `d>8`, that restricted Hessian has a nontrivial kernel. Therefore every regular local minimum on a stratum of dimension above eight is second-order degenerate; any nondegenerate constrained minimum must have stratum dimension at most eight (at least ten independent active constraints in the 18-dimensional anchored space).

This is only a Morse-type necessary condition. Degenerate strict minima can occur for quadratic objectives on curved constraint sets, so the kernel does not yield a constant-area feasible curve without further analysis. It does not prove that all global minimizers have contact-stratum dimension at most eight or that a low-dimensional finite skeleton suffices.

Independent read-only review by `/root/partner_hessian_review` found no mathematical error in the full proof, including the contact-stratum limitation. External novelty and human acceptance are unassessed.
