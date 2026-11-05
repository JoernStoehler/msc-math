#!/usr/bin/env python3
"""Render retained diagnostic outputs only; does not rerun the feature."""
import argparse,gzip,json,math
from pathlib import Path
import numpy as np
import analyze as a

def main():
 p=argparse.ArgumentParser();p.add_argument('--run',type=Path,required=True);p.add_argument('--source',type=Path,required=True);args=p.parse_args()
 rows=[json.loads(l) for l in gzip.open(args.run/'rows.jsonl.gz','rt')];summary=json.loads((args.run/'summary.json').read_text());a.plot(rows,summary,args.run)
 import matplotlib;matplotlib.use('Agg')
 import matplotlib.pyplot as plt
 src={r['name']:r for r in map(json.loads,args.source.open())}
 eligible=[r for r in rows if r['bucket'] in ['5x5','5x6','6x6']]
 leader=max(eligible,key=lambda r:r['sys']);worst=max([r for r in eligible if r['name'] in summary['buckets'][r['bucket']]['top2_names']],key=lambda r:abs(r['prediction']/r['capacity']-1))
 theta=np.pi/10;R=np.array([[math.cos(theta),-math.sin(theta)],[math.sin(theta),math.cos(theta)]])
 hko=dict(name='Known HKO control',sys=a.HKO,prediction_sys=summary['HKO_control']['hko']['predicted_sys'],p=a.core(a.N5/a.H),q=a.core((a.N5/a.H)@R.T))
 entries=[hko,leader,worst]
 fig,ax=plt.subplots(2,3,figsize=(11,6.6),layout='constrained')
 for j,r in enumerate(entries):
  for i,key in enumerate(['p','q']):
   if j==0:v=a.P5 if i==0 else a.P5@R.T
   else:
    raw=src[r['name']];aa=np.array(raw['dual_vertices']);k=raw['k'];v=a.polygon(aa[:k,:2] if i==0 else aa[k:,2:])
   cr=r[key];vv=np.array(cr['center'])+a.P5@np.array(cr['G']).T
   scale=math.sqrt(a.area(v));m=np.mean(v,axis=0);v=(v-m)/scale;vv=(vv-m)/scale
   v=np.vstack([v,v[0]]);vv=np.vstack([vv,vv[0]]);aa=ax[i,j];aa.fill(*v.T,color='#c9dbe3',alpha=.65,label='original factor');aa.plot(*v.T,color='#266b88',lw=1.6);aa.plot(*vv.T,color='#cf6d26',lw=2,ls='--',label='frozen pentagon core')
   aa.set_aspect('equal');aa.set_xticks([]);aa.set_yticks([]);aa.margins(.12)
   if i==0:aa.set_title(f"{r['name']}\nratio {r['sys']:.4f}; prediction {r['prediction_sys']:.4f}",fontsize=10)
   aa.set_xlabel(f"Factor{'P' if i==0 else 'Q'}: core area / body area = {cr['coverage']:.3f}",fontsize=9)
 ax[0,0].legend(fontsize=8,loc='lower left');fig.suptitle('The HKO core fits exactly; sampled high-ratio bodies lose most of the area\nEach displayed original factor has area one; relative orientation retained',fontsize=13)
 fig.savefig(args.run/'core-geometry.png',dpi=160);fig.savefig(args.run/'core-geometry.pdf');plt.close(fig)
if __name__=='__main__':main()
