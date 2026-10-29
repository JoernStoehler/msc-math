"""Exact finite predicates; the mathematical implication is in PROOF.md.
Run with Sage Python. No optimizer or external repository file is used.
"""
from sage.all import AA, QQ, RIF, NumberField, matrix, polygen, vector, diagonal_matrix, identity_matrix
from pathlib import Path
import itertools,json,time
started=time.monotonic();directory=Path(__file__).resolve().parent
T=polygen(QQ,'t');F=NumberField(T**4-10*T**2+5,'t',embedding=AA.polynomial_root(T**4-10*T**2+5,RIF(0,1)));t=F.gen();s=(5-t*t)/2

def require(condition,message):
    if not condition:raise ValueError(message)

def decode(cs):return sum(F(QQ(c))*t**i for i,c in enumerate(cs))

data=json.loads((directory/'certificate.json').read_text())
require(data['field_polynomial']=='t^4-10*t^2+5' and data['root_interval']==[0,1],'field metadata')
a0=(3-s)/2;b0=t*(1+s)/2;sec=s-1
normals=[vector(F,v) for v in [[1,t,0,0],[-a0,b0,0,0],[-sec,0,0,0],[-a0,-b0,0,0],[1,-t,0,0],[0,0,t,-1],[0,0,b0,a0],[0,0,0,sec],[0,0,-b0,a0],[0,0,-t,-1]]]
q0=1/(2*(5*t-t**3/2));require(decode(data['q0'])==q0,'capacity normalization')
B=matrix(F,[[decode(cs) for cs in row] for row in data['rows']]);pairs=list(itertools.combinations(range(4),2))
require(B.ncols()==6 and B.nrows()==len(data['witnesses']),'witness count')
for k,w in enumerate(data['witnesses']):
    sigma=w['sigma'];beta=[decode(cs) for cs in w['beta']]
    require(len(sigma)==len(beta) and len(set(sigma))==len(sigma),'distinct word')
    require(all(0<=i<10 for i in sigma) and min(beta)>=0 and sum(beta)==1,'simplex feasibility')
    require(sum((beta[i]*normals[j] for i,j in enumerate(sigma)),vector(F,[0]*4))==0,'closure')
    coeff=vector(F,[sum(beta[i]*beta[j]*(normals[sigma[i]][u]*normals[sigma[j]][v]-normals[sigma[i]][v]*normals[sigma[j]][u]) for i in range(len(sigma)) for j in range(i+1,len(sigma))) for u,v in pairs])/q0
    require(coeff==B.row(k),'action bivector reconstruction')
print('Feasible HK action rows:',B.nrows(),flush=True)
co=(11*t-t**3)/8;si=(s-1)/4
require(co*co+si*si==1 and co>0 and si>0,'rotation')
R=matrix(F,[[co,-si],[si,co]]);v=vector(F,[1,0]);D=[]
for k in range(20):D.append(v);v=R*v
require(v==vector(F,[1,0]) and len(set(map(tuple,D)))==20,'regular 20-gon')
E=D[::2];a=(5-s)/4;r=F(143)/200
require(a>0 and r>0 and a*a+r*r<1,'strict top sphere bound')
vertices=[vector(F,list(u)+[0]) for u in E]+[vector(F,[r*u[0],r*u[1],sign*a]) for sign in [-1,1] for u in D]
require(all(v.dot_product(v)<=1 for v in vertices),'sphere bound')
require(sum(v.dot_product(v)==1 for v in vertices)==10,'sphere contacts')
facet_normals=D[1::2]+[D[k]+D[(k+1)%20] for k in range(20)]
facets=[vector(F,[a,0,0,1]),vector(F,[a,0,0,-1])]
for n in facet_normals:
    d0=max(n.dot_product(v) for v in E);d1=r*max(n.dot_product(v) for v in D)
    require(d0>d1,'nested polygons')
    for sign in [-1,1]:facets.append(vector(F,[d0,-n[0],-n[1],sign*(d1-d0)/a]))
require(all(h[0]+vector(F,h[1:]).dot_product(v)>=0 for h in facets for v in vertices),'outer supporting planes')
L=matrix(F,[[0,QQ(1)/2,0,0,QQ(1)/2,0],[0,0,QQ(1)/2,-QQ(1)/2,0,0],[QQ(1)/2,0,0,0,0,-QQ(1)/2]])
require(len(facets)==62 and len(data['facet_duals'])==len(data['facet_indices'])==4,'facet dual count')
for k,cert in zip(data['facet_indices'],data['facet_duals']):
    h=facets[k]
    ids=cert['ids'];lam=vector(F,[decode(cs) for cs in cert['lambda']])
    require(len(ids)==len(lam) and all(0<=i<B.nrows() for i in ids),'dual support')
    require(min(lam)>=0 and sum(lam)==1,'convex dual coefficients')
    require(lam*B.matrix_from_rows(ids)==-(vector(F,h[1:])*L)/h[0],f'projection dual identity {k}')
# Exact symmetry interface: Ω ↦ sign T Ω Tᵀ, with T a body automorphism.
rot=matrix(F,[[(s-1)/4,-co,0],[co,(s-1)/4,0],[0,0,1]])
ref=diagonal_matrix(F,[1,-1,1]);inv=-identity_matrix(F,3)
Trot=identity_matrix(F,4);Trot[0,0]=Trot[1,1]=(s-1)/4;Trot[0,1]=co;Trot[1,0]=-co
Tref=diagonal_matrix(F,[1,-1,-1,1])
for body_map,sign,projection_map in [(Trot,1,rot),(Tref,-1,ref),(identity_matrix(F,4),-1,inv)]:
    require(set(tuple(body_map.transpose()*v) for v in normals)==set(map(tuple,normals)),'body symmetry')
    columns=[]
    for u,v in pairs:
        skew=matrix(F,4,4);skew[u,v]=1;skew[v,u]=-1
        transformed=sign*body_map*skew*body_map.transpose()
        columns.append(vector(F,[transformed[i,j] for i,j in pairs]))
    G=matrix(F,columns).transpose()
    require(L*G==projection_map*L,'selfdual symmetry intertwining')
group=[rot**k*ref**i*inv**j for k in range(5) for i in range(2) for j in range(2)]
normalized=[h/h[0] for h in facets]
for h in normalized:
    require(any(vector(F,h[1:])==vector(F,normalized[rep][1:])*g for rep in data['facet_indices'] for g in group),'facet symmetry coverage')
summary={'verified':True,'field':'Q(t), t=tan(pi/5)','feasible_action_rows':B.nrows(),'projection_facet_identities':len(data['facet_duals']),'facets_covered_by_symmetry':len(facets),'outer_vertices':len(vertices),'unit_sphere_contacts':10,'seconds':time.monotonic()-started}
print(json.dumps(summary,indent=2))
(directory/'verification-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
