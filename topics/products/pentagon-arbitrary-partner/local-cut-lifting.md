# Coupled one-cut stability when the exposed primal face has dimension <=1

Status: a deduction from the accepted F=10 sharpness input and the supplied
vertex-insertion construction, not a complete F=11 theorem. It reduces the
remaining task to exposed primal facets and two-faces.

## Inputs

P0 is HKO, c0=c_EHZ(P0), V0=Vol(P0), rho0=c0^2/(2V0).
The accepted ten-facet theorem, in a local symmetry slice, gives

    rho(P(A)) <= rho0-kappa r,   r=||A-A0||, kappa>0.

The exact six-segment orbit checker shows that every HKO vertex v lies on a
tight positive-weight HK word with rank-five constraints. Inserting b at v,
giving it weight z, and correcting five old weights gives

    Q(b,z)=Q0+2Q0(<v,b>-1)z+R_v(b)z^2.

For b in a fixed bounded shell and any vertex maximizing <v,b>, choose
z=mu delta, delta=(h_P0(b)-1)_+, with a common small mu>0. There are finitely
many vertices/sections, so R_v is uniformly bounded and old positive weights
stay positive. It follows that

    c(P0 intersect H_b) <= c0/(1+a delta^2)

uniformly for small delta, a>0. This is an upper capacity bound from feasible
HK weights. Subadditivity alone gives the opposite bound on capacity loss.

## Elementary uniform cap-volume estimate

Fix b0 in the relative interior of a polar face dual to a primal k-face F.
The vertices of P0 outside F have a strictly positive gap in <b0,x>. The gap
persists for nearby A,b. The moving face F(A) has the same labelled vertices.
Write delta=(h_P(A)(b)-1)_+.

For a removed point x, express x as a convex combination of vertices. Every
vertex outside F(A) is at least g>0 below h_P(A)(b), uniformly nearby. The total
weight of vertices outside F(A) is therefore at most delta/g. Hence the removed
cap lies in an O(delta)-tube of the bounded k-face F(A), and

    Vol(P(A) minus H_b) <= C delta^(4-k).

This argument works for all nearby b, not just radial cuts or a fixed direction.
It includes cuts that actually expose a subface of F. At delta=0 the cut is
redundant.

## Coupling with old-normal perturbations

Monotonicity and the ten-facet deficit give

    rho(P(A) intersect H_b)-rho0 <= -kappa r+C delta^(4-k).   (1)

The fixed-P0 insertion bound, the same volume estimate, and local Lipschitz
comparison with moving A give, for k<=1,

    rho(P(A) intersect H_b)-rho0 <= Lr-a' delta^2.            (2)

The Lipschitz estimate follows from multiplicative inclusions of uniformly
bounded bodies containing a common ball. The normalized normals vary by O(r);
capacity homogeneity/monotonicity and ordinary volume scaling give O(r) ratio
variation. The actual cut depths for A and A0 differ by O(r); on a bounded
small interval so do their squares. No equality of capacities is asserted.

A positive combination (L+kappa)*(1)+kappa*(2) eliminates the possible positive
linear r term and leaves

    const*(rho-rho0) <= -kappa^2 r-kappa a' delta^2
                        +(L+kappa)C delta^(4-k).

Since 4-k>2 for k=0,1, the last term is absorbed on a smaller neighborhood.
This proves coupled local maximality in every chart with b0 exposing a primal
vertex OR edge (Cases IV and III). It includes arbitrary higher-order paths.

## Still missing

For k=2 the volume term is quadratic and its coefficient must be controlled;
for k=3 it is linear. The preceding absorption does not decide those cases.
The duplicate-normal zero-slope set is generally a cone, not a linear space.
One must not reinterpret this note as a completed theorem on all eleven-facet
polytopes. Complete local neighborhoods at Cases I and II would, together with
these and the redundant interior case, cover the compact base polar and yield
the full Hausdorff statement by finite-subcover/sequence compactness.
