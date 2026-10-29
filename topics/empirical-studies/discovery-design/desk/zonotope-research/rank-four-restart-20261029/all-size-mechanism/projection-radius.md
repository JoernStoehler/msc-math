# Projection-body mechanism: exact obstruction and equivalent remaining claim

29 October 2026. This bounded all-size iteration followed the retained seven-generator/equality packet. It did not resume eight-generator graph enumeration or broad optimization. The unrestricted rank-four inequality remains open. The useful outcome is an exact falsifier of a natural projection-body strengthening, together with a corrected equivalent formulation that identifies the missing geometric step.

## Definitions and the proposed mechanism

For a zonotope Z with half-generators v_i in standard symplectic R4, put

    Phi = sum_{i<j} |omega(v_i,v_j)|,
    Psi = sum_{i<j<k<l} |det(v_i,v_j,v_k,v_l)|,
    a(u) = sum_i |omega(v_i,u)|,
    b(u) = sum_{i<j<k} |det(v_i,v_j,v_k,u)|.

Then vol_4(Z)=16 Psi. The support functions satisfy a(u)=h_{JZ}(u) and 8b(u)=h_{Pi Z}(u), where Pi Z is the usual projection body and J is the standard orthogonal symplectic matrix. Adjoining the segment t[-u,u] changes (Phi,Psi) exactly to (Phi+ta,Psi+tb).

A tempting all-direction inequality is

    b(u)^2 - Phi a(u)b(u) + Psi a(u)^2 <= 0.                (1)

It would force Phi^2>=4Psi whenever a(u)>0. At equality in the target, it would require b(u)=(Phi/2)a(u) in every direction, equivalently Pi Z=4Phi JZ. The new nonproduct equality family shows that this proportionality is too strong.

## Exact falsifier at an equality zonotope

Take the skew matrix A with upper entries (1,2,2,2,-2,1), in increasing pair order, and let

    W = [[A,3I],[-3I,A]].

Here A^2=-9I, so the Schur complement vanishes and rank(W)=4. The equality-family formula, or the exact 70-term Pfaffian sum, gives Phi=32 and Psi=256. For u=v_1+v_2+v_3+v_4, the appended pairing vector is the sum of the first four columns of W:

    y = (5,-1,-3,-1,-3,-3,-3,-3).

Thus a=22. The 56-term triple determinant sum is b=324. Consequently the left side of (1) is

    324^2 - 32*22*324 + 256*22^2 = 784 > 0.

This does not refute the target, nor even threaten it along this segment. Its exact extension deficit is

    (32+22t)^2 - 4(256+324t) = 112t+484t^2 >= 0,  t>=0.

The failure is the lower bound implicit in (1): the directional ratio r(u)=162/11 is below Phi/2=16. Another direction, u=v_1, has a=8 and b=128, so r(v_1)=16. These directional ratios must not be confused with their maximum.

`projection-witness.py` reproduces the integer identities without third-party packages. The separate audit `../audit/all-size-audit.md` independently recomputed the witness. No finite test is used as proof of the unrestricted inequality.

## Corrected maximum-radius equivalence

Define

    rho(Z) = max_{u:a(u)>0} b(u)/a(u),

with rho=0 if a is identically zero. This is a genuine attained finite maximum after quotienting by ker(a): ker(a) is contained in ker(b). If the generators span at most two dimensions, b=0; if they span a hyperplane H, its symplectic perpendicular is the characteristic line in H and every a-null u gives zero triple determinants; if they span R4, a is a norm.

For a full-dimensional Z, 8rho is the least R with Pi Z contained in R JZ. The following two universal statements are equivalent:

    (T) 4Psi(Z) <= Phi(Z)^2 for every zonotope Z in R4;
    (R) rho(Z)^2 - Phi(Z)rho(Z) + Psi(Z) <= 0 for every such Z.

(R) implies (T) immediately because a quadratic taking a nonpositive value has nonnegative discriminant.

Conversely assume (T) universally, and fix Z. If Phi=0 then Psi=b=rho=0. Otherwise let delta=Phi^2-4Psi and lambda_+/-=(Phi+/-sqrt(delta))/2. Applying (T) to Z+t[-u,u] for every t>=0 gives

    delta + (2Phi a-4b)t + a^2 t^2 >= 0.

For b/a<=Phi/2, the bound b/a<=lambda_+ is automatic. For b/a>Phi/2, evaluate at its positive minimum t=(2b-Phi a)/a^2 to obtain b/a<=lambda_+. Hence rho<=lambda_+.

The exact averaging identities

    sum_i a(v_i)=2Phi,    sum_i b(v_i)=4Psi

give rho>=2Psi/Phi. Moreover

    2Psi/Phi = 2lambda_-lambda_+/(lambda_-+lambda_+) >= lambda_-.

Therefore rho lies between the roots and (R) follows. The equivalence is global: (T) is needed also for every added-segment body. This does not establish a new proof of (T) or an independent weaker sufficient condition.

## Concrete blocker and stopping point

The remaining statement is precisely the Bonnesen-shaped bound

    Psi <= rho(Phi-rho).

The existing three-dimensional zonoid projection theorem used in the sparse-row proof does not state this symplectic four-dimensional maximum-radius bound. The targeted primary-source comparison in `source-note.md` found a structural precedent, not an implication. Absence in that bounded search is not a novelty claim.

The completed mechanism test rejects (1) exactly. Replacing every directional ratio by its maximum repairs that objection but returns an equivalent open claim. A productive continuation would need an actual geometric argument for this maximum-radius inequality, or a different rank-preserving reduction; more generic numerical samples or eight-index graph cases do not resolve this blocker. This bounded iteration stops here.
