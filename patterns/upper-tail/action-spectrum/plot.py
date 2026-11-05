#!/usr/bin/env python3
"""Render the exact path and the frozen blind-panel failure."""
import argparse,json,math
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=Path(__file__).resolve().parent);o=p.parse_args();d=o.directory
rt=math.sqrt(5);rho=(3+rt)/5;beta=(1+rt)/4;phi=(1+rt)/2;a=(3-rt)/4;loads=np.array([a,-.5,1/phi,-.5,a]);ts=np.linspace(-.06,.06,501)
S1=np.array([np.mean((1+t*loads)**-1) for t in ts]);S2=np.array([np.mean((1+t*loads)**-2) for t in ts]);actual=rho*np.array([(1-.5*t)**2 if t>=0 else(1+t/phi)**2 for t in ts])/(1-beta*ts**2);bound=rho*S1**2/((1-beta*ts**2)*S2**2);rms=rho/((1-beta*ts**2)*S2)
plt.rcParams.update({'font.size':10,'axes.spines.top':False,'axes.spines.right':False})
fig,ax=plt.subplots(1,2,figsize=(11,4.3),layout='constrained')
ax[0].plot(ts,actual,label='Exact ratio',color='#174a7e',lw=2.5);ax[0].plot(ts,bound,label='Calibrated moment ratio',color='#b6651a',lw=2);ax[0].plot(ts,rms,label='Calibrated RMS',color='#298c67',ls='--',lw=1.8);ax[0].axhline(1,color='.5',ls=':',lw=1);ax[0].set(xlabel='Signed support amplitude t',ylabel='Systolic ratio / point prediction',title='The true upper tail has a linear cusp');ax[0].legend(loc='lower right',fontsize=8.5)
rows=json.loads((d/'scores.json').read_text())['rows'];colors={'support':'#174a7e','angular':'#b6651a','full':'#298c67'}
for f in colors:
 rr=[r for r in rows if r['family']==f];ax[1].scatter([r['ratio'] for r in rr],[r['predicted_bound_scaled'] for r in rr],label=f,s=25,alpha=.8,color=colors[f])
lo=min(r['ratio'] for r in rows)-.005;hi=max(r['predicted_bound_scaled'] for r in rows)+.005;ax[1].plot([lo,hi],[lo,hi],color='.4',lw=1,ls='--');ax[1].axvline(1,color='.5',ls=':',lw=1);ax[1].axhline(1,color='.5',ls=':',lw=1);ax[1].set(xlabel='Exact ratio, blind 60-product panel',ylabel='Frozen calibrated moment-ratio prediction',title='All 60 predicted above one; truth: 41');ax[1].legend(title='Perturbation family',loc='lower right',fontsize=8.5)
fig.savefig(d/'moment-cusp.png',dpi=180)
