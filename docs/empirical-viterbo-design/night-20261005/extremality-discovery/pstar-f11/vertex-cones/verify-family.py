# /// script
# dependencies = ["sympy"]
# ///
"""Verify rational continuation and coefficient-sign stability, not sampled signs."""
import itertools
import json
import sys
from pathlib import Path
import sympy as S

root = Path(__file__).resolve().parent
repo = next(p for p in root.parents if (p / "formal").is_dir() and (p / "experiments").is_dir())
sys.path.insert(0, str(repo / "experiments/hko-local-maximum/non-hko/code"))
from family_exact import family

v, s = S.symbols("v s")
A, _, K, T = family(v,s)
F = S.QQ.frac_field(v,s)
zero = F.zero
base = {v:S.Rational(1,2), s:S.Rational(1,12)}
locals_ = {"v":v,"s":s}

def parse(expr):
    return F.from_sympy(S.sympify(expr,locals=locals_))

def base_value(expr):
    sym = F.to_sympy(expr)
    assert sym.as_numer_denom()[1].subs(base) != 0
    return sym.subs(base)

norms = [[F.from_sympy(A[i,j]) for j in range(4)] for i in range(10)]

def omega(a,b):
    return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]

base_packet = json.loads((root / "witnesses.json").read_text())
byvertex = {tuple(r["vertex"]):r for r in base_packet["witnesses"]}
packet = json.loads((root / "family-witnesses.json").read_text())
seen = set()
for r in packet["witnesses"]:
    key = tuple(r["vertex"])
    assert key not in seen
    seen.add(key)
    orig = byvertex[key]
    qi,pi = key
    x = K[qi,:].T.col_join(T[pi,:].T)
    ids = orig["facets"]
    assert all(S.cancel((A[i,:]*x)[0]-1) == 0 for i in ids)
    assert A[ids,:].det().subs(base) != 0
    sig = r["word"]
    assert sig == orig["word"] and len(sig) == len(set(sig))
    nn = sig.index(10)
    old = r["old_positions"]
    assert old == [k for k in range(len(sig)) if k != nn]
    b = [parse(z) for z in r["base_beta"]]
    assert b[nn] == zero and sum(b,zero) == F.one
    assert all(sum((b[k]*norms[sig[k]][j] for k in old),zero) == zero for j in range(4))
    assert all(base_value(z) >= 0 for z in b)
    for k,z in enumerate(b):
        assert base_value(z) == S.Rational(orig["base_beta"][k])
        if base_value(z) == 0:
            assert z == zero
    dc = [[parse(z) for z in row] for row in r["d_coefficient_rows"]]
    assert len(dc) == len(old) and all(len(row) == 4 for row in dc)
    for a in range(4):
        assert sum((dc[i][a] for i in range(len(old))),zero) == -F.one
        for j in range(4):
            assert sum((dc[i][a]*norms[sig[k]][j] for i,k in enumerate(old)),zero)+norms[ids[a]][j] == zero
    for i,k in enumerate(old):
        for coef in dc[i]:
            val = base_value(coef)
            if b[k] == zero:
                assert val >= 0
                if val == 0:
                    assert coef == zero
    Q = sum((b[old[i]]*b[old[j]]*omega(norms[sig[old[i]]],norms[sig[old[j]]]) for i,j in itertools.combinations(range(len(old)),2)),zero)
    assert Q == F(S.Rational(1,2))
    for a in range(4):
        L = sum(((b[old[i]]*dc[j][a]+dc[i][a]*b[old[j]])*omega(norms[sig[old[i]]],norms[sig[old[j]]]) for i,j in itertools.combinations(range(len(old)),2)),zero)
        L += sum((b[k]*omega(norms[sig[k]],norms[ids[a]])*(1 if k < nn else -1) for k in old),zero)
        assert L == zero
    # Derive the entire quadratic polynomial C(t) independently.
    co = [[zero]*4 for _ in range(4)]
    for i,j in itertools.combinations(range(len(old)),2):
        om = omega(norms[sig[old[i]]],norms[sig[old[j]]])
        for a,bb in itertools.product(range(4),repeat=2):
            co[a][bb] -= dc[i][a]*dc[j][bb]*om
    for i,k in enumerate(old):
        sign = 1 if k < nn else -1
        for a,bb in itertools.product(range(4),repeat=2):
            co[a][bb] -= sign*dc[i][a]*omega(norms[sig[k]],norms[ids[bb]])
    positive = False
    for a,bb in itertools.combinations_with_replacement(range(4),2):
        c = co[a][bb] if a == bb else co[a][bb]+co[bb][a]
        assert c == parse(r["C_coefficients"][f"{a},{bb}"])
        val = base_value(c)
        assert val >= 0
        if a in (0,1) and bb in (2,3):
            assert val > 0
        if val == 0:
            assert c == zero
        else:
            positive = True
    assert positive
assert seen == set(itertools.product(range(6),range(4)))
# Constant volume uses polygon identities, with the displayed orientations.
for polygon, area in [(K,-1),(T,S.Rational(1,2))]:
    double_area = sum((polygon[i,0]*polygon[(i+1)%polygon.rows,1]-polygon[i,1]*polygon[(i+1)%polygon.rows,0] for i in range(polygon.rows)),S.Integer(0))
    assert S.cancel(double_area-2*area) == 0
print("PASS: all 24 family branches over QQ(v,s), including closure, base Q=1/2 and vanishing linear term.")
print("PASS: old zero-weight and C coefficient signs persist on a common parameter neighborhood; zero coefficients are identities.")
print("Consequence: all fixed interior vertex-cut rays decrease rho near E(v,s); no uniform cut-depth radius at cone boundaries asserted.")
