#!/usr/bin/env python3
"""Draw exact analytic controls; no fit or target dataset."""
import argparse, importlib.util, json, math, sys
sys.dont_write_bytecode = True
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import numpy as np

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,required=True);args=ap.parse_args();folder=args.directory
    spec=importlib.util.spec_from_file_location('controls',folder/'verify.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
    eps=1/25;p=c.fpolygon(1,eps);m=c.fpolygon(-1,eps);base=c.fpolygon(1,0)
    q=c.rotate(base,math.pi/2);q=[[math.sin(2*math.pi/5)*x,math.sin(2*math.pi/5)*y] for x,y in q]
    fig,axes=plt.subplots(1,3,figsize=(13,3.7),layout='constrained')
    colorp,colorm='#166b96','#ce5939'
    for shape,label,color,ls in [(base,'regular baseline','#999999',':'),(p,r'$P_+$',colorp,'-'),(m,r'$P_-$',colorm,'--')]:
        x,y=zip(*(shape+shape[:1]));axes[0].plot(x,y,color=color,ls=ls,lw=1.8,label=label)
    axes[0].set_aspect('equal');axes[0].set_title(r'Nonregular controls, $\epsilon=1/25$');axes[0].legend(fontsize=9);axes[0].set_xlabel('physical x');axes[0].set_ylabel('physical y')
    t=np.linspace(0,2*np.pi,721)
    pp=[c.mixed(p,c.rotate(p,x)) for x in t];mm=[c.mixed(m,c.rotate(m,x)) for x in t]
    axes[1].plot(t*180/np.pi,pp,color=colorp,lw=2,label=r'$M(P_+,R_\theta P_+)$')
    axes[1].plot(t*180/np.pi,mm,color=colorm,ls='--',lw=1.4,label=r'$M(P_-,R_\theta P_-)$')
    axes[1].set_title('Every rotated mixed area agrees');axes[1].set_xlabel(r'$\theta$ (degrees)');axes[1].set_ylabel('self mixed area');axes[1].legend(fontsize=9);axes[1].set_xlim(0,360)
    b=(1+math.sqrt(5))/4;gold=(1+math.sqrt(5))/2;rho=(3+math.sqrt(5))/5;e=np.linspace(0,.055,250)
    sp=rho*(1-e/2)**2/(1-b*e*e);sm=rho*(1-e/gold)**2/(1-b*e*e)
    axes[2].axhline(1,color='#777777',lw=1);axes[2].plot(e,sp,color=colorp,lw=2,label=r'$\mathrm{sys}(P_+\times_LQ)$');axes[2].plot(e,sm,color=colorm,ls='--',lw=2,label=r'$\mathrm{sys}(P_-\times_LQ)$')
    for val in [rho*(1-eps/2)**2/(1-b*eps*eps),rho*(1-eps/gold)**2/(1-b*eps*eps)]:axes[2].scatter([eps],[val],s=25,color=colorp if val>1 else colorm,zorder=5)
    axes[2].axvline(eps,color='#bbbbbb',ls=':',lw=1);axes[2].set_title('Same descriptors, opposite threshold');axes[2].set_xlabel(r'support amplitude $\epsilon$');axes[2].set_ylabel('systolic ratio');axes[2].legend(fontsize=8,loc='lower left');axes[2].set_ylim(.978,1.05);axes[2].set_xlim(0,.055)
    for ax in axes:ax.spines[['top','right']].set_visible(False)
    fig.savefig(folder/'mixed-area-phase.png',dpi=180)
    fig.savefig(folder/'mixed-area-phase.svg')
if __name__=='__main__':main()
