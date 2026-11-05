#!/usr/bin/env python3
import argparse,json,math
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
ap=argparse.ArgumentParser();ap.add_argument('--data',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();d=json.loads((z.data/'blind-results.json').read_text());rows=d['rows'];fig,ax=plt.subplots(figsize=(7.4,4.9));colors={'full':'#547a98','support':'#c78f41','angular':'#7a9870'}
for f in sorted(colors):
 rr=[r for r in rows if r['family']==f];ax.scatter([r['alpha_root_volume']**2 for r in rr],[r['truth'] for r in rr],s=25,c=colors[f],alpha=.6,label=f+' perturbations',edgecolor='none')
x=np.linspace(11.93,14.75,200);ax.plot(x,25/(2*x),color='#242424',lw=1.7,label='regular pentagon products: sys=25/(2x²)');ax.axhline(1,color='#888',ls='--',lw=1)
for fn,label,offset in [('exact-threshold-counterexample.json','same x², opposite sides of 1',(43,-28)),('exact-counterexample.json','same x², both above 1',(48,54))]:
 c=json.loads((z.data/fn).read_text());q=c['decimals'];xx=q['x_squared'];y1=q['nonregular_ratio'];y0=q['regular_ratio'];ax.plot([xx,xx],[y0,y1],color='#a83b3b',lw=2.2);ax.scatter([xx,xx],[y0,y1],s=44,c=['white','#a83b3b'],edgecolors='#a83b3b',lw=1.6,zorder=10);ax.annotate(label,(xx,(y0+y1)/2),xytext=offset,textcoords='offset points',fontsize=9,arrowprops=dict(arrowstyle='-',color='#a83b3b'),color='#7d2525')
S=(3+math.sqrt(5))/5;ax.scatter([25/(2*S)],[S],s=60,c='#242424',marker='*',zorder=10);ax.annotate('HKO',(25/(2*S),S),xytext=(5,5),textcoords='offset points',fontsize=9);ax.set(xlim=(11.91,14.78),ylim=(.87,1.061),xlabel='x² = alpha(K)² vol₄(K)',ylabel='Systolic ratio',title='Maximal polar pairing is close near HKO, but loses the threshold');ax.spines[['top','right']].set_visible(False);ax.legend(loc='lower left',fontsize=8,frameon=False);fig.tight_layout();z.out.mkdir(parents=True,exist_ok=True)
for suffix in ['png','pdf']:fig.savefig(z.out/('polar-threshold-collision.'+suffix),dpi=180)
