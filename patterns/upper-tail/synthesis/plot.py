"""Assemble frozen-predictor comparison; no fitting or new capacity work."""
import argparse,json
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
b=a.repo/'patterns/upper-tail'
c=json.loads((b/'contact-stability/metrics.json').read_text())['rows']
polar={r['name']:r for r in json.loads((b/'polar-geometry/blind-results.json').read_text())['rows']}
symbolic={r['name']:r for r in json.loads((b/'symbolic-search/blind-results.json').read_text())['rows']}
y=np.array([r['truth'] for r in c]); pred=np.array([r['contact_prediction'] for r in c]); mask=y>1
fig,ax=plt.subplots(1,3,figsize=(13.1,4.1),layout='constrained')
for family,marker,color in [('full','o','#286796'),('support','s','#cb6e20'),('angular','^','#568645')]:
 s=np.array([r['family']==family for r in c]);ax[0].scatter(y[s],pred[s],marker=marker,s=26,c=color,label=family)
ax[0].plot([.87,1.055],[.87,1.055],color='#555555',lw=1,ls='--');ax[0].set(xlabel='Exact ratio',ylabel='Frozen local contact prediction',title='60 nonregular near-HKO products');ax[0].axvline(1,color='#999999',lw=.8);ax[0].axhline(1,color='#999999',lw=.8);ax[0].legend(fontsize=8)
for scale in [.003,.01,.03,.08]:
 errs=np.abs(pred/y-1)[np.array([float(__import__('fractions').Fraction(r['epsilon']))==scale for r in c])];ax[1].scatter([scale]*len(errs),np.maximum(errs,1e-9),s=13,alpha=.5,color='#286796')
med=np.array([np.median(np.abs(pred/y-1)[np.array([float(__import__('fractions').Fraction(r['epsilon']))==s for r in c])]) for s in [.003,.01,.03,.08]])
ax[1].plot([.003,.01,.03,.08],med,color='#cb6e20',marker='o',label='Median');ax[1].plot([.003,.08],med[0]*(np.array([.003,.08])/.003)**2,color='#555555',ls='--',label='Quadratic reference');ax[1].set(xscale='log',yscale='log',xlabel='Frozen perturbation scale',ylabel='Absolute relative ratio error',title='Error follows the local remainder');ax[1].set_xticks([.003,.01,.03,.08], labels=['0.003','0.01','0.03','0.08']);ax[1].xaxis.set_minor_locator(matplotlib.ticker.NullLocator());ax[1].legend(fontsize=8)
methods=[('Contact',pred),('5 / polar pairing',np.array([polar[r['name']]['predictions']['HKO_mechanistic'] for r in c])),('Symbolic product',np.array([symbolic[r['name']]['predictions']['products'] for r in c])),('Constant HKO',np.full(len(c),(3+np.sqrt(5))/5))]
for i,(name,z) in enumerate(methods):
 e=np.abs(z[mask]/y[mask]-1)*100;ax[2].scatter(np.full(len(e),i),e,s=11,alpha=.38,color=['#286796','#568645','#cb6e20','#777777'][i]);ax[2].plot([i-.2,i+.2],[np.median(e)]*2,color='black',lw=2)
ax[2].set(yscale='log',ylabel='Absolute relative ratio error (%)',title='41 exact above-one cases',xticks=range(len(methods)),xticklabels=[m[0].replace(' ','\n',1) for m in methods]);ax[2].tick_params(axis='x',labelsize=8)
fig.suptitle('Local contact geometry captures loss that bulk calibration misses',fontsize=13)
fig.savefig(a.out/'upper-tail-comparison.png',dpi=180);fig.savefig(a.out/'upper-tail-comparison.pdf');plt.close(fig)
