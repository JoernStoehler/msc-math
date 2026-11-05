# /// script
# dependencies = ["matplotlib==3.10.8", "numpy==2.4.2", "scipy==1.17.0"]
# ///
"""Plot verified historical observations, with no capacity evaluations.
Run from repo root: uv run <this script>. Optional --table and --provenance paths.
"""
import argparse, hashlib, json
from pathlib import Path
from collections import defaultdict
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from scipy.stats import spearmanr, pearsonr
from matplotlib.ticker import ScalarFormatter

HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--table', type=Path, default=Path.home()/'.cache/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl')
parser.add_argument('--provenance', type=Path, default=Path('experiments/polytope-invariant-table/polytope-provenance-table.jsonl'))
a = parser.parse_args()
expected = ['607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59', '6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2']
hashes=[]
for p,e in zip([a.table,a.provenance],expected):
    h=hashlib.sha256(p.read_bytes()).hexdigest()
    assert h==e,(str(p),h,e)
    hashes.append(h)
rows=[json.loads(s) for s in a.table.read_text().splitlines()]
prov=defaultdict(list)
for s in a.provenance.read_text().splitlines():
    r=json.loads(s); prov[r['poly_id']].append(r)
groups=defaultdict(list)
for r in rows:
    if r['capacity_source']=='random_sample':
        k=f"Generic: {r['facet_count']} facets"
    else:
        ps=[p for p in prov[r['poly_id']] if p['dataset']=='random_product_sample']
        labels={(p['product_k'],p['product_m']) for p in ps}
        assert len(labels)==1,(r['poly_id'],labels)
        k,m=next(iter(labels));k=f'Product: {k} × {m}'
    groups[k].append((r['ridge_symp_area_sum_over_volume_sqrt'],r['sys']))
groups={k:np.asarray(v) for k,v in groups.items()}
keys=[f'Generic: {f} facets' for f in range(5,13)]+[f'Product: {k} × {m}' for k in range(3,7) for m in range(k,7)]
assert sum(map(len,groups.values()))==14336
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'axes.spines.top':False,'axes.spines.right':False,'svg.fonttype':'none'})
colors=['#226b9b','#ba572b']
def draw(ax,z,color,title):
    ax.scatter(z[:,0],z[:,1],s=5,alpha=.32,color=color,edgecolors='none',rasterized=True)
    ax.set_xscale('log');ax.set_xlim(8,10000);ax.set_ylim(0,.9)
    ax.set_xticks([10,100,1000,10000]);ax.xaxis.set_major_formatter(ScalarFormatter())
    ax.tick_params(axis='x',labelsize=8)
    ax.grid(alpha=.15);ax.set_title(title,fontsize=10,loc='left')
fig,axs=plt.subplots(5,4,figsize=(12,12),sharex=True,sharey=True)
for i,k in enumerate(keys):
    draw(axs.flat[i],groups[k],colors[i>=8],f'{k}  (n={len(groups[k])})')
    axs.flat[i].tick_params(labelbottom=True,labelleft=True)
for ax in list(axs.flat)[18:]: ax.set_visible(False)
fig.supxlabel('Normalized unsigned symplectic ridge-area sum R (logarithmic axis)',y=.025)
fig.supylabel('Historical numerical systolic ratio',x=.015)
fig.tight_layout(rect=(.025,.04,1,1))
for suffix in ['svg','png']:fig.savefig(HERE/f'association-stratified.{suffix}',dpi=160)
plt.close(fig)
fig,axs=plt.subplots(1,2,figsize=(10,4.3),sharex=True,sharey=True)
for i,label in enumerate(['Generic','Product']):
    z=np.concatenate([groups[k] for k in keys if k.startswith(label)])
    draw(axs[i],z,colors[i],f'{label} polytopes (n={len(z):,})')
    axs[i].set_xlabel('R (logarithmic axis)')
axs[0].set_ylabel('Historical numerical systolic ratio')
fig.tight_layout()
for suffix in ['svg','png']:fig.savefig(HERE/f'association-overview.{suffix}',dpi=180)
allz=np.concatenate(list(groups.values()))
def stats(z): return dict(n=len(z),spearman=float(spearmanr(z[:,0],z[:,1]).statistic),pearson=float(pearsonr(z[:,0],z[:,1]).statistic),R_min=float(z[:,0].min()),R_max=float(z[:,0].max()),sys_max=float(z[:,1].max()))
receipt={'table_sha256':hashes[0],'provenance_sha256':hashes[1],'input_rows':len(rows),'plotted_rows':sum(map(len,groups.values())),'omitted_rows':0,'data_kind':'Historical numerical targets, no current capacity reevaluation','axis':'R displayed on logarithmic axis with raw R tick labels; sys unchanged','pooled':stats(allz),'groups':{k:stats(groups[k]) for k in keys}}
(HERE/'association-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
