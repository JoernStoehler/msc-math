from fractions import Fraction as F
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon
A,B=F(11,20),F(4,5);a=1-A;g=A+B-1;d=B-a*a;z=A*B-g*g
T=[(0,0),(A,0),(A,B)]
Q=[(0,0),(1,0),(1,F(9,16)),(F(4,11),1),(0,1)]
D=(A*A*a/d,-A*A*B/d);L=tuple(-a/A*x for x in D);R=(L[0]+1,L[1]);C=(A,B)
t=(A+a*A*g/d,a*B*(A+g)/d)
slope=-B*(1-B)/(A*(1-A));zz=-g*(1-B)/z;dd=g*(1+zz)
x=(A-1)*(1+zz);y=A*(1+zz)
p1=(A,B);p4=(x,slope*x);p7=(y,slope*y);p10=(A+dd,B+slope*dd)
K0=[p4,p7,p10,p1];K=[L,D,R,C]
fig,axes=plt.subplots(1,3,figsize=(10,3.6),gridspec_kw={'width_ratios':[.9,1,1]})
colors=['#2864ad','#d47b00','#168275','#9256a1']
def drawpoly(ax,p,**kw):
 p=np.array(p,dtype=float);ax.add_patch(Polygon(p,closed=True,**kw));return p
for ax,P,title in zip(axes,[Q,K0,K],[r'$Q$: corner-truncated square',r'Proposed cover $K_0$',r'Competing cover $K$']):
 p=drawpoly(ax,P,facecolor='#f1f3f5',edgecolor='#202b38',linewidth=1.6,zorder=1)
 ax.set_title(title,fontsize=11,pad=10);ax.set_aspect('equal');ax.axis('off')
 xmin,xmax=p[:,0].min(),p[:,0].max();ymin,ymax=p[:,1].min(),p[:,1].max()
 ax.set_xlim((xmin+xmax)/2-.72,(xmin+xmax)/2+.72);ax.set_ylim((ymin+ymax)/2-.72,(ymin+ymax)/2+.72)
for ax,minus,h,v in [(axes[1],p10,p4,p7),(axes[2],t,L,D)]:
 drawpoly(ax,T,facecolor=colors[0],edgecolor=colors[0],alpha=.18,linewidth=1.3,zorder=2)
 nt=[(minus[0]-q[0],minus[1]-q[1])for q in T]
 drawpoly(ax,nt,facecolor=colors[1],edgecolor=colors[1],alpha=.18,linewidth=1.3,zorder=2)
 ax.plot([float(h[0]),float(h[0]+1)],[float(h[1])]*2,c=colors[2],lw=2.2,zorder=3)
 ax.plot([float(v[0])]*2,[float(v[1]),float(v[1]+1)],c=colors[3],lw=2.2,zorder=3)
axes[0].text(.5,-.03,r'$|Q|=303/352$',transform=axes[0].transAxes,ha='center',fontsize=11)
axes[1].text(.5,-.03,r'$c=1,\quad |K_0|=3861/6350$',transform=axes[1].transAxes,ha='center',fontsize=11)
axes[2].text(.5,-.03,r'$c=1,\quad |K|=144/239$',transform=axes[2].transAxes,ha='center',fontsize=11)
fig.text(.5,.02,'Blue and orange: opposite normal triangles. Green and purple: unit horizontal and vertical segments.',ha='center',fontsize=9,color='#343a40')
fig.subplots_adjust(left=.02,right=.99,top=.88,bottom=.2,wspace=.12)
p=Path(__file__).resolve().parent
fig.savefig(p/'truncated-covers.pdf',bbox_inches='tight');fig.savefig(p/'truncated-covers.png',dpi=180,bbox_inches='tight')
