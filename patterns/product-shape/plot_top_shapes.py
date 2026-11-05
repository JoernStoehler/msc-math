# /// script
# dependencies = ["numpy==2.5.4", "scipy==1.18.1", "matplotlib==3.10.8"]
# ///
"""Plot four largest retained 5x5/5x6/6x6 products; no capacity calculations."""
import argparse,csv,gzip,hashlib,json
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
def primal(a):
    a=a[np.argsort(np.arctan2(a[:,1],a[:,0]))]
    return np.array([np.linalg.solve(np.stack([a[i],a[(i+1)%len(a)]]),np.ones(2)) for i in range(len(a))])
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--features',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);arg=ap.parse_args();arg.out.mkdir(parents=True,exist_ok=True)
    assert hashlib.sha256(arg.source.read_bytes()).hexdigest()==SOURCE_SHA
    with gzip.open(arg.features,'rt') as f: rows=list(csv.DictReader(f))
    selected=sorted([r for r in rows if int(r['k'])>=5],key=lambda r:-float(r['sys']))[:4]; names={r['name'] for r in selected};geometries={}
    for line in arg.source.open():
        r=json.loads(line)
        if r['name'] in names:geometries[r['name']]=r
    fig,axes=plt.subplots(2,4,figsize=(12,6.5),layout='constrained');details=[]
    for col,row in enumerate(selected):
        raw=geometries[row['name']];a=np.array(raw['dual_vertices']);p=primal(a[:raw['k'],:2]);q=primal(a[raw['k']:,2:]);
        details.append(dict(name=row['name'],historical_sys=float(row['sys']),capacity_over_two_bounce=float(row['c_over_c2']),alpha_p=float(row['asym_p']),alpha_q=float(row['asym_q']),p_vertices=p.tolist(),q_vertices=q.tolist()))
        for index,(v,color,label) in enumerate([(p,'#2877ad','P'),(q,'#c96b19','Q')]):
            v=v[ConvexHull(v).vertices];v=(v-v.mean(0))/np.sqrt(ConvexHull(v).volume);v=np.vstack([v,v[0]]);ax=axes[index,col];ax.fill(*v.T,color=color,alpha=.16);ax.plot(*v.T,color=color,lw=2);ax.set_aspect('equal');ax.set(xlim=(-1.65,1.65),ylim=(-1.65,1.65));ax.grid(alpha=.12);ax.set_xticks([]);ax.set_yticks([])
            if col==0:ax.set_ylabel(f'Factor {label}',fontsize=12)
            if index==0:ax.set_title(f"{row['name']}\nratio = {float(row['sys']):.4f}",fontsize=10)
            else:ax.set_xlabel(f"α(P)={float(row['asym_p']):.3f}, α(Q)={float(row['asym_q']):.3f}\nc / A₂ = {float(row['c_over_c2']):.3f}",fontsize=10)
    fig.suptitle('The four largest ratios in the retained 5×5, 5×6 and 6×6 products\nFactors translated and individually scaled to area one; original relative directions retained',fontsize=13)
    fig.savefig(arg.out/'top-shapes.png',dpi=160);plt.close(fig);(arg.out/'top-shapes.json').write_text(json.dumps(details,indent=2)+'\n');print('PASS: four retained shape pairs plotted; no capacities recomputed')
if __name__=='__main__':main()
