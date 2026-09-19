"""Prepare exact dyadic uniform rescalings; no capacity evaluations."""
import gzip, hashlib, json, math
from fractions import Fraction as F
from pathlib import Path
ROOT=Path(__file__).resolve().parent
FULL=ROOT.parents[1]/'target-full'
SRC=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):
    with gzip.open(p,'rt') if p.suffix=='.gz' else p.open() as f:
        yield from map(json.loads,f)
def power(k): return F(2)**k
def norms(vs): return [max(map(abs,v)) for v in vs]
def good(ns): return all(F(1,1000)<=n<=1000 for n in ns)
diag=json.loads((FULL/'norm-diagnostic.json').read_text())
manifest=json.loads((FULL/'input-manifest.json').read_text())
inputs=FULL/'inputs.jsonl.gz'
assert hashlib.sha256(gzip.decompress(inputs.read_bytes())).hexdigest()==manifest['sha256']
rows={r['id']:r for r in read(inputs) if r['id'] in {d['id'] for d in diag['rows']}}
assert len(rows)==54
wanted={(r['source_file'],r['source_name']):r for r in rows.values()}
source={}
for fn,expected in manifest['source_sha256'].items():
    assert sha(SRC/fn)==expected
    for r in read(SRC/fn):
        if (fn,r['name']) in wanted: source[wanted[fn,r['name']]['id']]=r
out=[];receipt=[];unresolved=[]
for d in diag['rows']:
    r=rows[d['id']];s=source[r['id']]
    dual=[[F(x) for x in v] for v in s['dual_vertices_rational']]
    primal=[[F(x) for x in v] for v in s['vertices_rational']]
    assert dual==[[F(x) for x in v] for v in r['dual_vertices']]
    feasible=[k for k in range(-30,31) if good([n*power(k) for n in norms(primal)]) and good([n/power(k) for n in norms(dual)])]
    assert feasible==d['feasible_primal_dilation_pow2_exponents']
    item=dict(id=r['id'],source_name=r['source_name'],source_file=r['source_file'],feasible_k=feasible)
    if not feasible:
        pn=norms(primal);dn=norms(dual)
        # t bounds combine exact primal and dual norm policies.
        lo=max(F(1,1000)/min(pn),max(dn)/1000)
        hi=min(F(1000)/max(pn),min(dn)*1000)
        item.update(min_primal_norm=str(min(pn)),max_primal_norm=str(max(pn)),min_dual_norm=str(min(dn)),max_dual_norm=str(max(dn)),uniform_scale_lower=str(lo),uniform_scale_upper=str(hi),uniform_scale_lower_float=float(lo),uniform_scale_upper_float=float(hi),no_positive_uniform_scale=lo>hi,primal_vertices=len(primal),dual_vertices=len(dual))
        unresolved.append(item);continue
    k=min(feasible,key=lambda k:(abs(k),k));t=power(k)
    newdual=[[math.ldexp(x,-k) for x in v] for v in r['dual_vertices']]
    assert [[F(x) for x in v] for v in newdual]==[[x/t for x in v] for v in dual]
    new=dict(r);new['dual_vertices']=newdual
    new['transform']={'kind':'uniform_primal_power_of_two','k':k,'original_id':r['id'],'convention':'K_scaled=2^k K; dual_scaled=dual_original/2^k','capacity_original_multiplier':str(power(-2*k)),'volume_original_multiplier':str(power(-4*k))}
    # Metadata (including historical targets) remains original, not scaled.
    out.append(new);item.update(k=k,scaled_primal_norm_min=float(min(norms(primal))*t),scaled_primal_norm_max=float(max(norms(primal))*t),scaled_dual_norm_min=float(min(norms(dual))/t),scaled_dual_norm_max=float(max(norms(dual))/t),exact_binary64_scaling=True)
    receipt.append(item)
out.sort(key=lambda r:r['id'])
payload=''.join(json.dumps(r,sort_keys=True)+'\n' for r in out).encode()
(ROOT/'inputs.jsonl').write_bytes(payload)
(ROOT/'selection.json').write_text(json.dumps(dict(rows=len(out),inputs_sha256=hashlib.sha256(payload).hexdigest(),full_inputs_sha256=manifest['sha256'],diagnostic_sha256=sha(FULL/'norm-diagnostic.json'),prepare_sha256=sha(Path(__file__)),source_sha256=manifest['source_sha256'],policy='Every retained rational primal and dual infinity norm in [1/1000,1000]; fresh geometry reconstruction remains required',rows_selected=receipt,unresolved=unresolved),indent=2)+'\n')
print(json.dumps({'prepared':len(out),'input':str(ROOT/'inputs.jsonl'),'unresolved':unresolved},indent=2))
