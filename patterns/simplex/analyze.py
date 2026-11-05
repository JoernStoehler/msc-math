# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy", "matplotlib"]
# ///
"""Reconstruct the simplex capacity from all 24 cyclic HK orders; never runs a producer."""
import argparse, gzip, hashlib, itertools, json
from pathlib import Path
import numpy as np

SOURCE_SHA='a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f'
def main():
 p=argparse.ArgumentParser();p.add_argument('--source',type=Path,required=True);p.add_argument('--output',type=Path,required=True);args=p.parse_args()
 assert hashlib.sha256(args.source.read_bytes()).hexdigest()==SOURCE_SHA
 root=Path(__file__).resolve().parents[2]
 if not (root/'topics').exists():root=Path.cwd()
 geom=root/'topics/empirical-studies/retrospective-revalidation/geometry-full'
 prov={r['source_name']:r['poly_id'] for r in map(json.loads,gzip.open(geom/'polytope-provenance-table.jsonl.gz','rt'))}
 table={r['poly_id']:r for r in map(json.loads,gzip.open(geom/'polytope-table.jsonl.gz','rt'))}
 J=np.block([[np.zeros((2,2)),np.eye(2)],[-np.eye(2),np.zeros((2,2))]])
 perms=np.array([(0,)+z for z in itertools.permutations(range(1,5))]);ii,jj=np.triu_indices(5,1);rows=[]
 for line in args.source.open():
  x=json.loads(line)
  if x['facet_count']!=5:continue
  a=np.array(x['dual_vertices']);beta=np.linalg.solve(np.vstack([a.T,np.ones(5)]),[0,0,0,0,1]);assert beta.min()>0
  g=beta[:,None]*a;B=g@J@g.T;Q=max(B[perms[:,ii],perms[:,jj]].sum(axis=1));c=1/(2*Q);SB=abs(B[ii,jj]).sum();chi=Q/SB;V=x['volume'];pf=B[0,1]*B[2,3]-B[0,2]*B[1,3]+B[0,3]*B[1,2]
  row=table[prov[x['name']]];assert row['ridge_symp_area_ordering_failure_count']==0
  R=12*np.sqrt(V)*SB
  rows.append(dict(name=x['name'],poly_id=prov[x['name']],sys=x['sys'],capacity=x['capacity'],reconstructed_c=c,volume=V,R=R,stored_R=row['ridge_symp_area_sum_over_volume_sqrt'],chi=chi,pf_abs_volume=24*V*abs(pf),pf_sys=3*abs(pf)/Q**2))
 assert len(rows)==512
 result=dict(n=len(rows),source_sha256=SOURCE_SHA,source_contract='Historical numerical capacity/volume; independent floating reconstruction of exact simplex formulas, not certified intervals',max_capacity_relative_error=max(abs(x['reconstructed_c']/x['capacity']-1) for x in rows),max_ridge_relative_error=max(abs(x['R']/x['stored_R']-1) for x in rows),max_pfvolume_error=max(abs(x['pf_abs_volume']-1) for x in rows),max_sys_absolute_error=max(abs(x['pf_sys']-x['sys']) for x in rows),chi_quantiles=np.quantile([x['chi'] for x in rows],[0,.1,.5,.9,1]).tolist(),sys_quantiles=np.quantile([x['sys'] for x in rows],[0,.1,.5,.9,1]).tolist(),max_row=max(rows,key=lambda r:r['sys']))
 assert result['max_capacity_relative_error']<1e-11 and result['max_ridge_relative_error']<1e-10 and result['max_pfvolume_error']<1e-10
 args.output.mkdir(parents=True,exist_ok=True)
 (args.output/'scan.json').write_text(json.dumps(result,indent=2)+'\n');(args.output/'rows.json').write_text(json.dumps(rows,indent=2)+'\n')
 import matplotlib;matplotlib.use('Agg')
 import matplotlib.pyplot as plt
 plt.rcParams.update({'font.size':11,'axes.spines.top':False,'axes.spines.right':False})
 fig,ax=plt.subplots(1,2,figsize=(11,4.3),layout='constrained')
 R=np.array([x['R'] for x in rows]);s=np.array([x['sys'] for x in rows]);chi=np.array([x['chi'] for x in rows]);z=ax[0].scatter(R,s,c=chi,cmap='viridis',s=16,alpha=.8)
 xx=np.geomspace(R.min()*.95,R.max()*1.05,300)
 ax[0].plot(xx,162/xx**2,'--',color='#333333',label=r'$162/R^2$');ax[0].plot(xx,50/xx**2,':',color='#333333',label=r'$50/R^2$');ax[0].axhline(.75,color='#bd4c31',lw=1,label='Sharp simplex maximum 3/4');ax[0].set(xscale='log',yscale='log',xlabel='Normalized unsigned ridge sum R',ylabel='Historical numerical systolic ratio',title='512 retained simplices');ax[0].legend(fontsize=9);from matplotlib.ticker import FixedLocator,ScalarFormatter,NullFormatter
 ax[0].xaxis.set_major_locator(FixedLocator([20,50,100,200,500]));ax[0].xaxis.set_major_formatter(ScalarFormatter());ax[0].xaxis.set_minor_formatter(NullFormatter());fig.colorbar(z,ax=ax[0],label=r'Signed-order coherence $\chi$')
 ax[1].scatter(chi,s*R**2,s=16,color='#186b8b',alpha=.7);xx=np.linspace(1/3,.6,300);ax[1].plot(xx,18/xx**2,color='#bd4c31',label=r'Exact identity $\mathrm{sys} R^2=18/\chi^2$');ax[1].set(xlabel=r'Signed-order coherence $\chi$',ylabel=r'$\widehat{\mathrm{sys}} R^2$',title='The missing factor in ridge-only ordering');ax[1].legend(fontsize=9)
 fig.savefig(args.output/'simplex-ridge.png',dpi=160);fig.savefig(args.output/'simplex-ridge.pdf');plt.close(fig)
 print(json.dumps({k:v for k,v in result.items() if k!='max_row'},indent=2))
if __name__=='__main__':main()
