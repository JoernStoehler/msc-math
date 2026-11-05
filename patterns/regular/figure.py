"""Illustrations only; exact containment is checked by certificate.py."""
from pathlib import Path
import math
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from scan import closures
from itertools import permutations

n=7;h=math.cos(math.pi/n);w=1+h;area=.5*n*math.sin(2*math.pi/n);rho=.5/w**2
z=[]
for _,_,x in closures(n):
    for _,_,y in closures(n):
        for ox in [(0,1,2),(0,2,1)]:
            for oy in permutations(range(3)):
                z.append(.5*(x[ox[0]]*y[oy[0]].conjugate()-x[ox[2]]*y[oy[1]].conjugate())/rho)
z=np.array(z);z=np.unique(np.round(np.array([z.real,z.imag]).T,9),axis=0)
v=np.exp(1j*math.pi*np.arange(15)/7)
fig,ax=plt.subplots(1,2,figsize=(11,4.5),layout='constrained')
ax[0].fill(v.real,v.imag,color='#dfedf2');ax[0].plot(v.real,v.imag,color='#245b73',lw=2,label='four-block coefficient hull')
ax[0].scatter(z[:,0],z[:,1],s=9,c='#d47642',alpha=.65,label='all six-block coefficients')
ax[0].set_aspect('equal');ax[0].set_title('Angular shadow of the containment')
ax[0].set_xlabel('cosine coefficient / four-block radius');ax[0].set_ylabel('sine coefficient / four-block radius')
ax[0].legend(fontsize=8,loc='lower left')
angles=2*math.pi*np.arange(7)/7;normals=np.array([np.cos(angles),np.sin(angles)]).T
alpha=math.pi/14;rot=np.array([[math.cos(alpha),-math.sin(alpha)],[math.sin(alpha),math.cos(alpha)]])
tvals=np.linspace(-.8,.8,321);curves=[]
for family in ['stretch','shear']:
    ys=[]
    for t in tvals:
        b=np.diag([math.exp(t),math.exp(-t)]) if family=='stretch' else np.array([[1.,t],[0.,1.]])
        mat=rot@b;m=np.max(np.abs(normals@mat@normals.T));ys.append(w**4/(2*area**2*m*m))
    curves.append(ys)
ax[1].plot(tvals,curves[0],label='area-preserving stretch',color='#245b73',lw=2)
ax[1].plot(tvals,curves[1],label='area-preserving shear',color='#d47642',lw=2)
bound=w**4/(2*area**2*math.cos(alpha)**2)
ax[1].axhline(bound,color='#555',ls='--',lw=1,label=f'sharp affine-class bound {bound:.4f}')
ax[1].scatter([0],[bound],c='#222',s=25,zorder=4)
ax[1].set(title='Affine distortion cannot improve the maximum',xlabel='deformation parameter t',ylabel='systolic ratio',ylim=(0,1.02))
ax[1].legend(fontsize=8,loc='lower center')
for a in ax:a.spines[['top','right']].set_visible(False)
fig.savefig(Path(__file__).with_name('heptagon-structure.png'),dpi=180)
fig.savefig(Path(__file__).with_name('heptagon-structure.pdf'))
