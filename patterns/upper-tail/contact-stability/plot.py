#!/usr/bin/env python3
"""Scientific figure for frozen local contact predictions; no model fitting."""
import argparse,json,math,statistics
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;common=json.loads((d/'metrics.json').read_text());canon=json.loads((d/'canonical-metrics.json').read_text());ind=[r for r in canon['rows'] if r['transformed_from'] is None];copies=[r for r in canon['rows'] if r['transformed_from'] is not None];colors={'full':'#0072B2','support':'#009E73','angular':'#D55E00'}
 plt.rcParams.update({'font.size':10,'axes.titlesize':11,'axes.spines.top':False,'axes.spines.right':False,'figure.dpi':140,'savefig.dpi':180})
 fig,axs=plt.subplots(2,2,figsize=(10.5,8),layout='constrained')
 for ax,rows,key,title in [(axs[0,0],common['rows'],'contact_prediction','Fixed HKO chart: 60 fresh cases'),(axs[0,1],ind,'exp_prediction','Canonical product invariant: 45 fresh cases')]:
  for fam in colors:
   rr=[r for r in rows if r['family']==fam];ax.scatter([r['truth'] for r in rr],[r[key] for r in rr],s=24,c=colors[fam],label=fam,alpha=.85,edgecolors='white',linewidths=.35)
  lo=min(min(r['truth'],r[key]) for r in rows)-.008;hi=1.055;ax.plot([lo,hi],[lo,hi],color='#333333',lw=1);ax.axvline(1,color='#BBBBBB',lw=.7);ax.axhline(1,color='#BBBBBB',lw=.7);ax.set(xlim=(lo,hi),ylim=(lo,hi),xlabel='Exact systolic ratio',ylabel='Frozen prediction',title=title);ax.set_aspect('equal',adjustable='box')
 axs[0,0].legend(loc='upper left',frameon=False,fontsize=9);axs[0,0].text(.96,.04,'Above one: 41 cases\nmedian error 0.00467%',ha='right',va='bottom',transform=axs[0,0].transAxes,fontsize=9);axs[0,1].text(.96,.04,'Above one: 23 cases\nmedian error 0.00656%',ha='right',va='bottom',transform=axs[0,1].transAxes,fontsize=9)
 ax=axs[1,0]
 for rows,key,label,color in [(common['rows'],'contact_prediction','raw contact defect','#0072B2'),(ind,'exp_prediction','canonical invariant','#CC79A7')]:
  from fractions import Fraction as F
  eps=sorted({float(F(r['epsilon'])) for r in rows});vals=[statistics.median(abs(r[key]/r['truth']-1)*100 for r in rows if float(F(r['epsilon']))==e) for e in eps];ax.loglog(eps,vals,'o-',c=color,label=label)
 eps=[.003,.1];ax.loglog(eps,[.6*e*e*100 for e in eps],'--',c='#777777',lw=1,label=r'proportional to $\epsilon^2$');ax.set(xlabel='Perturbation scale',ylabel='Median relative ratio error (%)',title='Quadratic error scaling');ax.legend(frameon=False,fontsize=9);ax.grid(alpha=.15,which='both')
 ax=axs[1,1];copies.sort(key=lambda r:r['name']);xx=list(range(1,10));ax.plot(xx,[r['truth'] for r in copies],'o-',c='#333333',label='exact ratio',ms=5);ax.scatter(xx,[r['raw_prediction'] for r in copies],c='#999999',marker='x',s=35,label='raw chart prediction');ax.scatter(xx,[r['exp_prediction'] for r in copies],c='#CC79A7',s=30,label='canonical prediction',zorder=5);ax.set(xlabel='Matched finite coordinate transform',ylabel='Systolic ratio / prediction',title='Canonicalization removes coordinate distortion',xticks=xx);ax.legend(frameon=False,fontsize=9,loc='lower right');ax.grid(axis='y',alpha=.15)
 fig.suptitle('HKO contact geometry predicts local nonregular upper tails',fontsize=14)
 fig.savefig(d/'contact-stability.png');fig.savefig(d/'contact-stability.pdf');print(d/'contact-stability.png')
if __name__=='__main__':main()
