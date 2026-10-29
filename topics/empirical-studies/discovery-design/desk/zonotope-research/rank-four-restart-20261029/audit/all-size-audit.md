# Audit: projection-radius reformulation and exact obstruction

The universal equivalence is correct, with one definition detail: define
`rho_Z = sup_{u:a_Z(u)>0} b_Z(u)/a_Z(u)` (and `rho_Z=0` if `a_Z` is
identically zero). This supremum is attained after quotienting by `ker(a_Z)`.
Indeed `a_Z` and `b_Z` are seminorms, and `ker(a_Z) subset ker(b_Z)`: if the
vectors span at most two dimensions, all triple determinants vanish; if they
span a hyperplane in R4, its symplectic perpendicular is the characteristic
line contained in that hyperplane, so an `a_Z`-null `u` makes every determinant
with three generators vanish; if they span R4, `a_Z(u)=0` forces `u=0`.
Thus the quotient ratio is continuous on a compact unit sphere.

Assume the universal inequality. For any fixed zonotope `Z` and any `u` with
`a=a_Z(u)>0`, adjoining `t[-u,u]` changes `(Phi,Psi)` to
`(Phi+ta,Psi+tb)`, where `b=b_Z(u)`. Since the inequality holds for every
`t>=0`, the quadratic

`delta + (2 Phi a-4b)t + a^2 t^2`, `delta=Phi^2-4Psi>=0`,

is nonnegative on the half-line. Its minimum implies
`b/a <= (Phi+sqrt(delta))/2`; when `b/a<=Phi/2` this is automatic, and
otherwise it follows by evaluating the interior minimum. Hence
`rho<=lambda_+`, where `lambda_+=(Phi+sqrt(delta))/2`.

For the lower root, evaluate on the original generators. Counting each
four-set four times and each edge twice gives
`sum_i b_Z(v_i)=4Psi` and `sum_i a_Z(v_i)=2Phi`. Terms with zero denominator
also have zero numerator by the kernel observation. Therefore the weighted
average of the ratios is `2Psi/Phi`, so `rho>=2Psi/Phi>=lambda_-`, where
`lambda_-=(Phi-sqrt(delta))/2`. Thus `rho` lies between the roots of
`x^2-Phi*x+Psi`, and the proposed strong inequality follows.

Conversely, if the universal strong condition
`rho_Z^2-Phi(Z)rho_Z+Psi(Z)<=0` holds, the quadratic has a real point where it
is nonpositive. Its discriminant is therefore nonnegative, giving
`Phi(Z)^2-4Psi(Z)>=0`. This is exactly the original conjectured inequality.
The equivalence is valid.

I independently recomputed the proposed counterexample. Let `A` have upper
entries `(1,2,2,2,-2,1)` and
`W=[[A,3I],[-3I,A]]`. Since `A^2=-9I`, the Schur complement vanishes and
`rank(W)=4`. Direct exact Pfaffian sums give `Phi=32`, `Psi=256`. For `u=v_1+v_2+v_3+v_4`, its pairing vector is the sum of the first four
columns, `y=(W_{i1}+W_{i2}+W_{i3}+W_{i4})_{i=1}^8=(5,-1,-3,-1,-3,-3,-3,-3)`, so
`a=22`. Summing the
56 absolute determinants of each triple of generators with `u` gives `b=324`.
For this particular direction, `r_u=b/a=324/22=162/11`, and

`r_u^2-32r_u+256=196/121>0`.

So the all-direction quadratic condition is false. This calculation does not
falsify the maximum-radius condition, because the directional ratio
`r(u)=162/11` is not `rho`. It also does not falsify the original inequality
(the base gap is zero). My exact recomputation used the Pfaffian formula on all
70 four-subsets and the extended Pfaffian formula for the 56 triples.
