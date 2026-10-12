"""Bounded retained-vertex diagnostic, not a repaired capacity request."""
import itertools, json, math
from fractions import Fraction as F
from pathlib import Path
ROOT=Path(__file__).resolve().parent
src=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random.jsonl')
r=next(r for r in map(json.loads,src.open()) if r['name']=='random_F8_s3_45')
p=[[F(x) for x in v] for v in r['vertices_rational']];d=[[F(x) for x in v] for v in r['dual_vertices_rational']]
candidates=[]
for a,b in itertools.product(range(-12,13),repeat=2):
    es=(a,b,-a,-b)
    pn=[max(abs(float(x))*2.**e for x,e in zip(v,es)) for v in p]
    dn=[max(abs(float(x))*2.**(-e) for x,e in zip(v,es)) for v in d]
    lo=max(.001/min(pn),max(dn)/1000);hi=min(1000/max(pn),min(dn)*1000)
    for k in range(-24,25):
        if lo<=2.**k<=hi: candidates.append((max(abs(a),abs(b)),abs(k),a,b,k))
for _,_,a,b,k in sorted(candidates):
    es=[k+a,k+b,k-a,k-b]
    pp=[[x*F(2)**e for x,e in zip(v,es)] for v in p]
    dd=[[x/F(2)**e for x,e in zip(v,es)] for v in d]
    if all(F(1,1000)<=max(map(abs,v))<=1000 for v in pp+dd):
        assert all(F(math.ldexp(float(x),-e))==x/F(2)**e for v in d for x,e in zip(v,es))
        result={'status':'exact_retained_vertex_candidate','a':a,'b':b,'k':k,'primal_coordinate_exponents':es,'convention':'D=diag(2^a,2^b,2^-a,2^-b), K_new=2^k D K; D symplectic in (q1,q2,p1,p2)','capacity_original_multiplier':str(F(2)**(-2*k)),'volume_original_multiplier':str(F(2)**(-4*k)),'fresh_geometry_and_policy_checks_required':True}
        break
else:result={'status':'no_diagonal_candidate','a_b_search':[-12,12],'k_search':[-24,24]}
(ROOT/'diagonal-probe.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
