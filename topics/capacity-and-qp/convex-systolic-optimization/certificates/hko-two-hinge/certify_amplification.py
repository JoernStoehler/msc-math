#!/usr/bin/env python3
"""Exact four-family predicates and constants for invariant-hinge amplification."""
from pathlib import Path
from fractions import Fraction as F
import sys,json,itertools,argparse,importlib.util
ROOT=Path(__file__).resolve().parent;SOURCE=ROOT.parent/'complete-simple'
E=None
def dec(c):return E.K([F(x) for x in c])
def enc(c):return c.coefficients()
def main():
 global E
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
 p.add_argument('--source',type=Path,default=SOURCE,help='existing exact replay directory: exact_core.py and raw or expected/ catalogue JSON')
 args=p.parse_args()
 if args.out.exists():p.error('output exists')
 source=args.source.resolve();data=source if (source/'primitives.json').exists() else source/'expected'
 sys.dont_write_bytecode=True
 spec=importlib.util.spec_from_file_location('amplification_exact_core',source/'exact_core.py')
 E=importlib.util.module_from_spec(spec);spec.loader.exec_module(E)
 prim=json.loads((data/'primitives.json').read_text());charts=json.loads((data/'catalogue.json').read_text())['charts']
 V=[]
 for key in ['triangles_q','triangles_p']:
  rows=[]
  for ids,ws in prim[key]:
   v=[E.ZERO]*10
   for i,w in zip(ids,ws):v[i]=dec(w)/2
   rows.append(v)
  V.append(rows)
 r=E.SQRT5;k=r-2;aa=(5-r)/40;bb=(5+r)/40;pL=r/10;pU=(5+2*r)/10;mu=-1+2*r/5;rstar=1+4*r/5
 N=r-1;smax=E.TAU/N;ymid=(1-1/N)/2;yhalf=(1+1/N)/2;e=[-E.A[0][1]/N,E.A[0][0]/N]
 T=1/(2*E.Q0);J=lambda a:[-a[2],-a[3],a[0],a[1]]
 def grad(sig,b):return [sum((b[j]*(E.omega(E.A[sig[j]],E.A[sig[i]]) if j<i else E.omega(E.A[sig[i]],E.A[sig[j]])) for j in range(len(sig)) if j!=i),E.ZERO) for i in range(len(sig))]
 records=[]
 for ci,edge,kind in [(115,1,'lower'),(148,-1,'lower'),(72,1,'upper'),(19,-1,'upper')]:
  ch=charts[ci];sig=ch['sigma'];qs=[i for i in range(5) if ch['q_vertices_mask']&(1<<i)];ps=[i for i in range(5) if ch['p_vertices_mask']&(1<<i)]
  ends=[[V[0][q][i]+V[1][p][i] for i in sig] for q,p in itertools.product(qs,ps)];assert len(ends)==2
  midpoint=E.scale(E.K(F(1,2)),E.add(*ends));assert all(E.sign(x)>0 for x in midpoint)
  assert all(E.qeval(E.A,sig,b)==E.Q0 for b in [ends[0],midpoint,ends[1]])
  M=[E.A[i]+[E.ONE] for i in sig]
  for I in itertools.combinations(range(len(sig)),5):
   try:inv=E.inv([M[i] for i in I]);break
   except ArithmeticError:pass
  coordinates=[]
  for b in ends:
   assert sum(b,E.ZERO)==E.ONE and all(E.sign(z)>=0 for z in b)
   assert all(sum((bw*E.A[lab][j] for lab,bw in zip(sig,b)),E.ZERO)==0 for j in range(4))
   g=grad(sig,b);lam=E.mv(inv,[g[i] for i in I]);assert all(E.dot(row,lam)==gi for row,gi in zip(M,g))
   x=E.scale(-T,lam[:4]);coord=None
   for lab,bw in zip(sig,b):
    end=E.add(x,E.scale(2*T*bw,J(E.A[lab])))
    assert all(E.sign(E.dot(a,x)-1)<=0 and E.sign(E.dot(a,end)-1)<=0 for a in E.A)
    assert E.dot(E.A[lab],x)==1 and E.dot(E.A[lab],end)==1
    if lab==0:
     xx=E.dot(e,x[:2])/smax;yy=(E.dot(e,x[2:])-ymid)/yhalf
     assert xx==edge and E.dot(e,end[2:])==E.dot(e,x[2:])
     expected=aa*(yy+3) if kind=='lower' else bb*(1-yy)
     assert bw==expected;coord=(xx,yy,bw)
    x=end
   assert coord is not None;coordinates.append(coord)
  yy=[z[1] for z in coordinates]
  assert set(yy)==set([E.K(-1),k] if kind=='lower' else [k,E.ONE])
  records.append({'chart':ci,'word':sig,'edge_coordinate':edge,'family':kind,
    'endpoint_weights':[[enc(x) for x in b] for b in ends],
    'endpoint_x_y_beta':[[enc(x) for x in xyz] for xyz in coordinates]})
 # Independent exact projection of the partner pentagon vertices.
 proj={}
 for w in E.W:
  y=(E.dot(e,w)-ymid)/yhalf;trans=E.dot([E.A[0][0]/N,E.A[0][1]/N],w)
  proj.setdefault(y,[]).append(trans)
 assert set(proj)=={E.K(-1),k,E.ONE}
 def width(vals):
  lo=hi=vals[0]
  for x in vals:
   if E.sign(x-lo)<0:lo=x
   if E.sign(x-hi)>0:hi=x
  return hi-lo
 area=E.area(E.W)
 assert width(proj[E.K(-1)])*yhalf/area==2*pL
 assert width(proj[k])*yhalf/area==pL*(k+3)==pU*(1-k)
 assert width(proj[E.ONE])==0
 # All constants in the elementary one-dimensional proof.
 checks={
  'mu_formula_and_density_integral':True,
  'downward_middle_perfect_square_linear':6*(5*aa/4)-1==-2*(5*aa/4)*rstar,
  'downward_middle_perfect_square_constant':9*(5*aa/4)+mu==(5*aa/4)*rstar*rstar,
  'downward_secant_bound':E.sign(20*aa-(1-mu))>0,
  'downward_large_threshold':E.sign(2*k+3-rstar)>0,
  'large_threshold_slope':E.sign(5*pL-1)>0,
  'upward_lower_endpoint':E.sign((25-9*r)/10)>0,
  'upward_zero_lower_bound':E.sign((210-91*r)/60)>0,
  'upward_zero_upper_bound':E.sign((555-233*r)/480)>0,
  'upward_upper_concavity':E.sign((25-19*r)/80)<0,
  'upper_density_ratio':(pU/bb)*(1-k)==4,
  'constant_hinge_bound':E.sign(pL-E.K(F(1,5)))>0,
  'sharp_witness_weight':aa*(mu+3)==E.K(F(1,5)),
 }
 # Integrate density and its first moment directly, exactly.
 def density_integrals(c,d,lo,hi):
  mass=c*(hi*hi-lo*lo)/2+d*(hi-lo)
  moment=c*(hi**3-lo**3)/3+d*(hi*hi-lo*lo)/2
  return mass,moment
 m1,y1=density_integrals(pL,3*pL,E.K(-1),k);m2,y2=density_integrals(-pU,pU,k,E.ONE)
 assert m1+m2==1 and y1+y2==mu
 assert all(checks.values()),checks
 out={'all_checks_passed':True,'scope':'Exact supporting orbit families, projected density and scalar proof predicates. The one-dimensional inequality and two-hinge inference are written analytic arguments.',
      'families':records,'constants':{name:enc(value) for name,value in [('k',k),('A',aa),('B',bb),('density_lower',pL),('density_upper',pU),('mean',mu),('sharp_threshold',rstar)]},'checks':checks}
 args.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'all_checks_passed':True,'charts':[x['chart'] for x in records],'checks':checks},indent=2))
if __name__=='__main__':main()
