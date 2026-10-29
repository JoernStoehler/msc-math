# /// script
# dependencies = ["matplotlib>=3.8", "numpy>=1.26"]
# ///
"""Compact, caption-free paper figures for the affine outer body and tent caps."""
from pathlib import Path
import math
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon, Circle
from matplotlib.colors import to_rgba

OUT=Path(__file__).resolve().parent
INK='#24292b'; ACC='#b45130'; MUTED='#a6aeb0'
plt.rcParams.update({'font.family':'DejaVu Serif','font.size':9,
                     'mathtext.fontset':'dejavuserif','axes.linewidth':.7,
                     'pdf.fonttype':42,'savefig.facecolor':'white'})

def finish(fig,name):
    for ext in ('pdf','png'):
        fig.savefig(OUT/f'{name}.{ext}', dpi=240)
    plt.close(fig)

def simple_axes(ax):
    ax.spines[['top','right']].set_visible(False)
    ax.tick_params(length=2.5, color=MUTED, labelsize=8)

# The actual T: E is the even vertices of a unit regular 20-gon;
# rD occurs at heights +-a.
a=(5-math.sqrt(5))/4
r=143/200
k=np.arange(20)
D=np.column_stack((np.cos(k*math.pi/10),np.sin(k*math.pi/10)))
E=D[::2]
rD=r*D
assert len(D)==20 and len(E)==10
# Check all scaled D vertices lie strictly inside every oriented edge of E.
for v in rD:
    cross=[np.linalg.det(np.array([E[(i+1)%10]-E[i],v-E[i]])) for i in range(10)]
    assert min(cross)>0
assert r*r+a*a<1
assert np.allclose(np.linalg.norm(E,axis=1),1)
print(f'Outer body: r={r}, a={a:.12f}, r^2+a^2={r*r+a*a:.12f}<1; rD strictly inside E.')

fig,(ax1,ax2)=plt.subplots(1,2,figsize=(7.0,3.35))
fig.subplots_adjust(left=.07,right=.985,bottom=.15,top=.94,wspace=.24)
# Plan view: rD is a genuine 20-gon, E the even-vertex decagon.
ax1.add_patch(Circle((0,0),1,fill=False,edgecolor=MUTED,linewidth=.9,linestyle=(0,(3,2)),zorder=0))
ax1.add_patch(Polygon(E,closed=True,fill=False,edgecolor=INK,linewidth=1.3,zorder=2))
ax1.add_patch(Polygon(rD,closed=True,facecolor=to_rgba(ACC,.10),edgecolor=ACC,linewidth=1.1,zorder=1))
ax1.scatter(E[:,0],E[:,1],s=17,facecolor=ACC,edgecolor=ACC,zorder=4)
ax1.set(xlim=(-1.13,1.13),ylim=(-1.13,1.13),xticks=[-1,0,1],yticks=[-1,0,1])
ax1.set_aspect('equal');simple_axes(ax1)
ax1.text(.02,.99,r'$(a)\quad E\ \mathrm{and}\ rD$',transform=ax1.transAxes,va='top',fontsize=9)
ax1.annotate(r'$E\times\{0\}$',xy=(math.cos(2*math.pi/5),math.sin(2*math.pi/5)),
             xytext=(-1.05,.82),fontsize=8.5,color=INK,
             arrowprops=dict(arrowstyle='-',color=INK,lw=.65))
ax1.text(-.36,-.08,r'$rD\times\{\pm a\}$',fontsize=8.5,color=ACC)
ax1.set_xlabel('$u_1$');ax1.set_ylabel('$u_2$',rotation=0,labelpad=8)
# Exact axial cross-section T intersect {u2=0}. At height +-a the horizontal
# endpoints are +-r; at z=0 they are +-1.
section=np.array([[-r,-a],[r,-a],[1,0],[r,a],[-r,a],[-1,0]])
ax2.add_patch(Circle((0,0),1,fill=False,edgecolor=MUTED,linewidth=1.0,linestyle=(0,(3,2)),zorder=0))
ax2.add_patch(Polygon(section,closed=True,facecolor=to_rgba(ACC,.10),edgecolor=INK,linewidth=1.25,zorder=1))
ax2.scatter([-1,1],[0,0],s=28,color=ACC,zorder=4)
ax2.scatter([-r,r,-r,r],[-a,-a,a,a],s=11,facecolor='white',edgecolor=INK,zorder=4)
ax2.set(xlim=(-1.13,1.13),ylim=(-1.13,1.13),xticks=[-1,0,1],yticks=[-1,0,1])
ax2.set_aspect('equal');simple_axes(ax2)
ax2.text(.02,.99,r'$(b)\quad T\cap\{u_2=0\}$',transform=ax2.transAxes,va='top',fontsize=9)
ax2.text(.03,.08,r'$r^2+a^2<1$',transform=ax2.transAxes,fontsize=9,color=INK,
         bbox=dict(facecolor='white',edgecolor='none',alpha=.85,pad=1.5))
ax2.annotate(r'$z=a$',xy=(r,a),xytext=(.2,.92),fontsize=8.5,color=INK,
             arrowprops=dict(arrowstyle='-',lw=.65,color=INK))
ax2.set_xlabel('$u_1$');ax2.set_ylabel('$z$',rotation=0,labelpad=8)
finish(fig,'affine-outer-body')

# Two single-tent examples, with source K_s and exact cap C_s shown per column.
def upper(u,s):
    rt=np.sqrt(u)
    if s==1: return (u+rt)/2
    return np.where(u<=s*s,(u+rt)/(1+s),(rt-u)/(1-s))
def lower(u,s): return (u-np.sqrt(u))/(1+s)
fig,axes=plt.subplots(2,2,figsize=(6.75,4.45),gridspec_kw={'height_ratios':[1,1]})
fig.subplots_adjust(left=.085,right=.985,bottom=.14,top=.975,wspace=.26,hspace=.29)
for col,s in enumerate((.5,1)):
    src,cap=axes[:,col]
    slabel='1/2' if s==.5 else '1'
    p=np.array([[1.,0.],[s,1.],[-1.,0.],[-s,-1.]])
    src.add_patch(Polygon(p,closed=True,facecolor=to_rgba(ACC,.10),edgecolor=ACC,lw=1.25))
    src.scatter(p[:,0],p[:,1],s=12,color=ACC,zorder=3)
    src.axhline(0,color=MUTED,lw=.55);src.axvline(0,color=MUTED,lw=.55)
    src.set(xlim=(-1.1,1.1),ylim=(-1.1,1.1),xticks=[-1,0,1],yticks=[-1,0,1])
    src.set_aspect('equal');simple_axes(src)
    src.set_xlabel('$x$');src.set_ylabel('$y$',rotation=0,labelpad=8)
    src.text(.04,.96,rf'$K_{{{slabel}}}$',transform=src.transAxes,va='top',fontsize=10)
    u=np.linspace(0,1,800); hi=upper(u,s);lo=lower(u,s)
    cap.fill_between(u,lo,hi,color=ACC,alpha=.10)
    cap.plot(u,hi,color=ACC,lw=1.45)
    cap.plot(u,lo,color=INK,lw=1.0)
    cap.axhline(0,color=MUTED,lw=.55)
    if s<1:
        cap.plot(s*s,upper(np.array([s*s]),s)[0],'o',color=ACC,ms=2.5)
        cap.axvline(s*s,color=MUTED,lw=.55,ls='--')
    cap.set(xlim=(-.04,1.04),ylim=(-.27,1.08),xticks=[0,.5,1],yticks=[0,.5,1])
    simple_axes(cap)
    cap.set_xlabel('$u=x^2$');cap.set_ylabel('$v=xy$',labelpad=6)
    cap.text(.04,.96,rf'$C_{{{slabel}}}$',transform=cap.transAxes,va='top',fontsize=10)
finish(fig,'tent-caps-compact')
print('Compact cap panels: s=1/2,1; exact source vertices and piecewise cap boundaries.')
