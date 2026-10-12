# /// script
# dependencies = ["sympy"]
# ///
"""Verify a finite section-bank miss and its exact continuum repair at facet 0."""
import itertools
import json
import sys
from pathlib import Path
import sympy as S

root = Path(__file__).resolve().parent
repo = next(p for p in root.parents if (p / "formal").is_dir() and (p / "experiments").is_dir())
sys.path.insert(0, str(repo / "experiments/hko-local-maximum/non-hko/code"))
from family_exact import family, O, word_data

A, _, K, T = family(S.Rational(1,2),S.Rational(1,12))
cert = json.loads((repo / "experiments/hko-local-maximum/non-hko/certificate/certificate_base.json").read_text())
c = S.Rational(149,200)
eta = S.Rational(3,4)-c
D = S.Matrix([[0,-2*c,1,0]])
assert (D*O*A[0,:].T)[0] == 0
responses = []
for j,w in enumerate(cert["sections"]):
    sig = w["sigma"]
    if 0 not in sig:
        continue
    beta = S.Matrix(w["beta"])
    _, C, H, _, _ = word_data(A,sig)
    mu5 = C[:,w["pivot"]].T.inv(method="DM")*(H*beta)[w["pivot"],0]
    assert H*beta == C.T*mu5 and mu5[4] == 1
    i = sig.index(0)
    vec = sum((beta[k]*A[lab,:].T*(1 if k>i else -1 if k<i else 0) for k,lab in enumerate(sig)),S.zeros(4,1))
    kk = O*vec-mu5[:4,0]
    assert kk[1] == S.Rational(1,2)
    response_per_two_delta = (D*kk)[0]
    assert response_per_two_delta < 0
    responses.append((j,beta[i],kk[2]))
assert len(responses) == 20
assert max(kp1 for _,_,kp1 in responses) == S.Rational(48,65) < c

# An unsampled positive stationary continuum repairs the finite bank.
r = S.symbols("r",positive=True)
sig = [0,6,4,8,2,1,9]
beta = S.Matrix([r,S.Rational(1,4),S.Rational(1,4)-r/2,S.Rational(1,8),7*r/4,S.Rational(1,4)-9*r/4,S.Rational(1,8)])
mu = S.Matrix([-S.Rational(1,4),-S.Rational(1,2),3*r-S.Rational(3,4),2*r])
_, C, H, _, _ = word_data(A,sig)
assert C*beta == S.Matrix([0,0,0,0,1])
assert H*beta == C.T*mu.col_join(S.Matrix([1]))
assert S.expand((beta.T*H*beta)[0]/2) == S.Rational(1,2)
vec = sum((beta[k]*A[lab,:].T for k,lab in enumerate(sig) if k>0),S.zeros(4,1))
assert O*vec-mu == S.Matrix([S.Rational(1,4),S.Rational(1,2),S.Rational(3,4)-3*r,0])

r0 = eta/6
new_weight = r0/2
beta0 = beta.subs(r,r0)
assert min(beta0) > 0
old_pivots = [0,2,3,4,6]
assert C[:,old_pivots].det() == -S.Rational(4608,49)
jet = -new_weight*C[:,old_pivots].inv(method="DM")*D.T.col_join(S.zeros(1,1))
expected = S.Matrix([7*c*new_weight/9,-7*c*new_weight/18,3*new_weight/16,-7*c*new_weight/18,-3*new_weight/16])
assert jet == expected

# Keep all nonpivot and new weights fixed; old pivot weights vary linearly.
tau = S.symbols("tau",nonnegative=True)
db = S.zeros(7,1)
for k,i in enumerate(old_pivots):
    db[i] = jet[k]
old_weights = beta0+tau*db
old_weights[0] -= new_weight
weights = S.Matrix([new_weight]).col_join(old_weights)
order = [10]+sig
B = A.col_join(A[0,:]+tau*D)
closure = B[order,:].T.col_join(S.ones(1,len(order)))
assert all(S.expand(z) == 0 for z in closure*weights-S.Matrix([0,0,0,0,1]))
Q = S.expand(sum((weights[i]*weights[j]*(B[order[i],:]*O*B[order[j],:].T)[0] for i,j in itertools.combinations(range(len(order)),2)),S.Integer(0)))
assert Q == S.Rational(1,2)+new_weight*(eta-3*r0)*tau-S.Rational(5,6)*c*new_weight**2*tau**2
capacity_loss_response = 2*S.diff(Q,tau).subs(tau,0)
assert capacity_loss_response == S.Rational(1,480000)

# Exact facet integral and exact body volume for the displayed cut.
x = S.symbols("x")
I1 = S.integrate((x-c)*(1-4*x/3),(x,c,S.Rational(3,4)))
I2 = S.integrate((x-c)**2*(1-4*x/3),(x,c,S.Rational(3,4)))
assert I1 == 2*eta**3/9 and I2 == eta**4/9
V = S.Rational(1,2)-5*tau*I1/(12*(1-c*tau))-tau**2*I2/(12*(1-c*tau)**2)
volume_loss_response = -S.diff(V,tau).subs(tau,0)
assert volume_loss_response == S.Rational(1,86400000)
assert capacity_loss_response-volume_loss_response == S.Rational(179,86400000)

window = S.Rational(1,100)
assert 1-c*window > 0
assert window*eta/(2*(1-c*window)) < S.Rational(1,2)
assert min(weights.subs(tau,window)) > 0
assert min(weights.subs(tau,0)) > 0

def bernstein_positive(poly):
    p = S.Poly(S.expand(poly),tau)
    n = p.degree()
    coefficients = [sum((p.nth(k)*window**k*S.binomial(j,k)/S.binomial(n,k) for k in range(j+1)),S.Integer(0)) for j in range(n+1)]
    assert min(coefficients) > 0
    return min(coefficients)

# Denominators and coefficient positivity prove the whole window, not one point.
bernstein_positive(Q)
v_num,v_den = S.cancel(V).as_numer_denom()
bernstein_positive(v_num)
bernstein_positive(v_den)
ratio_gap = S.cancel(8*V*Q**2-1)
gap_num,gap_den = ratio_gap.as_numer_denom()
assert gap_num.subs(tau,0) == 0
bernstein_positive(gap_num/tau)
bernstein_positive(gap_den)

# All ten old facets and the new facet are genuine in the same window.
X = [K[i,:].T.col_join(T[j,:].T) for i in range(6) for j in range(4)]
for f in range(10):
    vertices = [xx for xx in X if (A[f,:]*xx)[0] == 1]
    midpoint = sum(vertices,S.zeros(4,1))/len(vertices)
    assert (A[f,:]*midpoint)[0] == 1
    assert all((A[g,:]*midpoint)[0] < 1 for g in range(10) if g != f)
    assert (B[10,:].subs(tau,window)*midpoint)[0] < 1
p1 = (c+S.Rational(3,4))/2
q2 = (1-window*p1)/(2-2*c*window)
new_facet_point = S.Matrix([0,q2,p1,0])
assert (B[10,:].subs(tau,window)*new_facet_point)[0] == 1
assert max(A*new_facet_point) < 1

print("PASS: all 20 eligible sections from the fixed 24-section bank have nonpositive splitting response; bank best is zero.")
print("PASS: explicit stationary continuum repairs the miss; capacity response 1/480000 exceeds volume response 1/86400000.")
print("PASS: every displayed eleven-facet body has rho < 1 for 0 < tau <= 1/100, by exact Bernstein coefficient bounds.")
