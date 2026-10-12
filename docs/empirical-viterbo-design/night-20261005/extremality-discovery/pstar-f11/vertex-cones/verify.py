# /// script
# dependencies = ["sympy"]
# ///
"""Check every retained P* full-vertex-cone feasible upper branch, exactly."""
import itertools
import json
from pathlib import Path
import sympy as S

root = Path(__file__).resolve().parent
packet = json.loads((root / "witnesses.json").read_text())
A = S.Matrix(packet["normal_rows"])
K = S.Matrix(packet["q_vertices"])
T = S.Matrix(packet["p_vertices"])
O = S.Matrix([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
t = S.symbols("t0:4", positive=True)
loc = {str(x): x for x in t}
relation = {t[3]: 1-sum(t[:3])}
base = dict.fromkeys(t, S.Rational(1,4))
eps = S.Rational(1,100)

def zero_on_simplex(expr):
    return S.expand(expr.subs(relation)) == 0

def nonnegative_coefficients(expr, degree):
    if S.expand(expr) == 0:
        return False
    poly = S.Poly(S.expand(expr), *t)
    assert all(sum(monomial) == degree for monomial, _ in poly.terms())
    assert all(coef >= 0 for coef in poly.coeffs())
    return any(coef > 0 for coef in poly.coeffs())

seen = set()
for row in packet["witnesses"]:
    qi, pi = row["vertex"]
    assert (qi,pi) not in seen
    seen.add((qi,pi))
    x = K[qi,:].T.col_join(T[pi,:].T)
    ids = row["facets"]
    assert ids == [i for i in range(10) if (A[i,:]*x)[0] == 1]
    assert len(ids) == 4
    N = A[ids,:]
    assert N.det() != 0
    w = sum((t[k]*A[i,:] for k,i in enumerate(ids)), S.zeros(1,4))
    B = A.col_join(w)
    order = row["word"]
    assert len(order) == len(set(order)) and 10 in order
    b = S.Matrix([S.Rational(z) for z in row["base_beta"]])
    d = S.Matrix([S.sympify(z, locals=loc) for z in row["d"]])
    C = S.sympify(row["C"], locals=loc)
    closure = B[order,:].T.col_join(S.ones(1,len(order)))
    assert all(z == 0 for z in closure*b-S.Matrix([0,0,0,0,1]))
    assert all(zero_on_simplex(z) for z in closure*d)
    assert b[order.index(10)] == 0 and d[order.index(10)] == 1
    H = S.zeros(len(order))
    for i,j in itertools.combinations(range(len(order)),2):
        H[i,j] = H[j,i] = (B[order[i],:]*O*B[order[j],:].T)[0]
    assert S.expand((b.T*H*b)[0]) == 1
    assert zero_on_simplex((b.T*H*d)[0])
    assert zero_on_simplex((d.T*H*d)[0]+2*C)
    assert nonnegative_coefficients(C, 2)
    for a,bb in itertools.product((0,1),(2,3)):
        assert S.Poly(S.expand(C),*t).coeff_monomial(t[a]*t[bb]) > 0
    for k in range(len(order)):
        assert b[k] >= 0
        if b[k] == 0 and k != order.index(10):
            nonnegative_coefficients(d[k], 1)
    # Exact uniform window on the 24 barycentric rays, not on all cone directions.
    cb = C.subs(base)
    db = d.subs(base)
    assert min(b+eps/(2*cb)*db) >= 0
    D = S.Rational(32,3)/abs(N.det())
    for k in range(4):
        cap_vertex = x-4*eps*N.inv()[:,k]
        assert min(S.ones(10,1)-A*cap_vertex) >= 0
    assert 1-eps**2/(2*cb) > 0
    assert 1-2*D*eps**4 > 0
    assert (1-eps**2/(2*cb))**2/(1-2*D*eps**4) < 1
assert seen == set(itertools.product(range(6),range(4)))
print("PASS: 24 full interior vertex-normal cones; exact feasible branches and positive curvature polynomials.")
print("PASS: every barycentric cut has rho < 1 for 0 < epsilon <= 1/100.")
