# /// script
# dependencies = ["sympy"]
# ///
"""Exact endpoint premises; no cut grid, optimizer, or evidence writes."""
import hashlib
import itertools
import json
from pathlib import Path
import runpy
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM

root = Path(__file__).resolve().parent
source = root.parent / "facet-split/moving-old"
packet = json.loads((root / "certificate.json").read_text())
assert hashlib.sha256((source / "certificate.json").read_bytes()).hexdigest() == packet["source_certificate_sha256"]
# Rebuild the maintained stationary continuum, full annihilators and W jet.
ctx = runpy.run_path(str(source / "verify.py"))
for key in ["contact", "W", "q_W_jet"]:
    assert packet[key] == ctx["packet"][key]
A, b, mu, C = [ctx[k] for k in ["A", "b", "mu", "C"]]
v, s, r = [ctx[k] for k in ["v", "s", "r"]]
base = {v: S.Rational(1, 2), s: S.Rational(1, 12)}
rmax = S.Rational(packet["balance"]["r_interval"][1])
sig = packet["contact"]["word"]
piv = packet["closure_pivots"]
for expr in list(b) + list(mu) + [ctx["parse"](z) for z in packet["W"] + packet["q_W_jet"]]:
    den = S.cancel(expr).as_numer_denom()[1]
    assert r not in den.free_symbols and den.subs(base) != 0
det = S.cancel(C[:, piv].det())
assert r not in det.free_symbols and det.subs(base) == -S.Rational(4608, 49)
# The eighteen common annihilators span the entire old rank22 kernel.
Wcol = S.Matrix([ctx["parse"](z) for z in packet["W"]])
kernel = ctx["symmetry"].row_join(A.diff(v).reshape(40, 1)).row_join(A.diff(s).reshape(40, 1)).row_join(Wcol)
assert kernel.subs(base).to_DM().rank() == 18
G = ctx["G"]
old_lambda = S.Matrix([S.Rational(z) for z in ctx["cert"]["weights"]])
assert min(old_lambda) > 0 and sum(old_lambda) == 1
assert G.T * old_lambda == S.zeros(40, 1)
GR = S.Matrix([[ctx["F"].to_sympy(z).subs(base) for z in ctx["GR"]]])
bal = packet["balance"]
gamma = S.Rational(bal["gamma"])
assert gamma == S.Rational(1, 100000)
lam = S.Matrix([S.sympify(z, locals={"r": r}) for z in bal["lambda_old"]])
assert S.cancel(sum(lam) + gamma - 1) == 0
assert all(S.cancel(z) == 0 for z in G.T * lam + gamma * GR.T)
lower = []
for z in lam:
    p = S.Poly(z, r)
    n = p.degree()
    bernstein = [sum(p.nth(k) * rmax**k * S.binomial(j, k) / S.binomial(n, k) for k in range(j + 1)) for j in range(n + 1)]
    assert min(bernstein) > 0
    lower.extend(bernstein)
assert min(lower) == S.Rational(bal["minimum_bernstein_coefficient"])
E = G.T.col_join(S.ones(1, 24))
rows, cols = bal["pivot_rows"], bal["pivot_columns"]
assert len(rows) == len(cols) == 23
assert E[rows, cols].det(method="domain-ge") == S.Rational(bal["pivot_determinant"]) != 0
# Exact endpoint weight orders, including the merged old0 after splitting.
bb = b.subs(base)
assert bb[0] == r and bb[4] == 7*r/4
constant_lower = min(bb[i].subs(r, endpoint) for i in range(len(bb)) if i not in [0, 4] for endpoint in [0, rmax])
assert constant_lower == S.Rational(1, 8)
eta, delta = S.symbols("eta delta")
u = 1-v+v*v
D = S.Matrix([[0, -2*(u-eta), 1, 0]])
F = S.QQ.frac_field(v, s, r, eta, delta)
zero = F.zero
k = (DM.from_Matrix(C[:, piv]).convert_to(F).inv() * DM.from_Matrix(-D.T.col_join(S.zeros(1, 1))).convert_to(F)).to_Matrix()
word = [10] + sig
normals = A.col_join(A[0, :])
weights = S.Matrix([delta] + list(b))
weights[1] -= delta
db = S.zeros(len(word), 1)
for i, z in zip(piv, k):
    db[i+1] = delta*z
edges = [[F.from_sympy(z) for z in weights[i]*normals[lab, :]] for i, lab in enumerate(word)]
jets = [[F.from_sympy(z) for z in db[i]*normals[lab, :] + (delta*D if lab == 10 else S.zeros(1, 4))] for i, lab in enumerate(word)]
def om(x, y):
    return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
pairs = list(itertools.combinations(range(len(word)), 2))
q0 = sum((om(edges[i], edges[j]) for i, j in pairs), zero)
q1 = sum((om(jets[i], edges[j])+om(edges[i], jets[j]) for i, j in pairs), zero)
q2 = sum((om(jets[i], jets[j]) for i, j in pairs), zero)
assert q0 == F.from_sympy(S.Rational(1, 2))
assert q1 == F.from_sympy(delta*(eta-4*u*r))
H = S.sympify(packet["adaptation"]["family_quadratic_coefficient"], locals={"v": v, "s": s, "eta": eta})
assert q2 == F.from_sympy(delta**2*H)
assert not (H.free_symbols & {r, delta})
for z in list(k) + [H]:
    assert S.cancel(z).as_numer_denom()[1].subs(base).subs(eta, 0) != 0
ell = S.cancel((2*F.to_sympy(q1)).subs({r: eta/6, delta: eta/12}))
assert S.cancel(ell-eta**2*(1-2*u/3)/6) == 0
assert (1-2*S.Rational(4, 5)/3)/6 == S.Rational(7, 90)
# Generic bilinear expansion, independent of product normals or a closure
# optimizer: all arbitrary-A edge jets carry the common tau*delta factor.
x, y, p, q = [S.symbols(f"{name}0:4") for name in ["x", "y", "p", "q"]]
t = S.symbols("t")
assert S.expand(om([x[j]+t*p[j] for j in range(4)], [y[j]+t*q[j] for j in range(4)]) - om(x, y) - t*(om(p, y)+om(x, q)) - t*t*om(p, q)) == 0
print("PASS: endpoint regularity, full kernel18/rank22, uniform positive balance gamma=1/100000 on r in [0,1/1200].")
print("PASS: small old/new weights scale as eta; pivot coefficients are regular through eta=0.")
print("PASS: Qnew-Qold=tau*delta*L+tau^2*delta^2*H structurally; L_family=eta-4ur, ell >= 7eta^2/90 for u<=4/5.")
print("Analytic uniform feasibility and scaled remainder follow by the retained compact-neighborhood argument in README.md.")
