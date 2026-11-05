# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
"""Replay global Steiner-relaxation certificates without an LP solver."""
from steiner_bound import *

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path.cwd());p.add_argument('certificate',type=Path);args=p.parse_args()
 f=load_fixture(args.root);F,Z=f.F,f.Z;fix=f.make_fixture()
 def parse(cs):
  x=Z
  for q in cs:x=x*f.tau+F(f.sp.Rational(q))
  return x
 T=[[[parse(c) for c in p] for p in s['vertices']] for s in fix['normal_shapes']]
 obj=json.loads(args.certificate.read_text());H=parse(fix['candidate_K_area'])
 if H!=parse(obj['HKO_area']):raise ArithmeticError('HKO reference mismatch')
 for rec in obj['results']:
  slope=parse(rec['shear_slope']);S=[steiner([[x,y-slope*x] for x,y in tri],f) for tri in T]
  if [[[f.serialize(c) for c in p] for p in tri] for tri in S]!=rec['steiner_templates']:raise ArithmeticError('Steiner reconstruction mismatch')
  groups=[[[y,x] for x,y in tri] for tri in S];lp=build_lp(groups,F,f.sign)
  A,b,c=lp['A'],lp['b'],lp['c'];z=[parse(c) for c in rec['primal_variables']]
  support=rec['dual_support'];ys=[parse(c) for c in rec['dual_weights']]
  if len(z)!=lp['n'] or len(support)!=len(ys):raise ArithmeticError('Dimension mismatch')
  if len(set(support))!=len(support) or any(i<0 or i>=len(A) for i in support):raise ArithmeticError('Invalid support')
  if not all(f.sign(rhs-sum((a*x for a,x in zip(row,z)),Z))>=0 for row,rhs in zip(A,b)):raise ArithmeticError('Primal infeasibility')
  if not all(f.sign(y)>=0 for y in ys):raise ArithmeticError('Negative dual weight')
  if any(sum((A[i][j]*y for i,y in zip(support,ys)),Z)!=-cj for j,cj in enumerate(c)):raise ArithmeticError('Dual stationarity')
  primal=sum((ci*zi for ci,zi in zip(c,z)),Z);dual=-sum((b[i]*y for i,y in zip(support,ys)),Z)
  if primal!=dual or primal!=parse(rec['objective']):raise ArithmeticError('Objective mismatch')
  pts=[]
  for j,tri in enumerate(groups):
   shift=Z if j==0 else z[2*lp['m']+j-1]
   pts.extend([[x+shift,y] for x,y in tri])
  hull=f.hull(pts)
  if f.area(hull)!=primal:raise ArithmeticError('Hull area mismatch')
  if slope==Z:
   if primal!=f.tau/F(5):raise ArithmeticError('Axis bound mismatch')
   if primal/H!=(F(5)+f.s5)/F(10):raise ArithmeticError('Exact gap mismatch')
  print(f'PASS Steiner slope {F.to_sympy(slope)}: exact symmetrals, primal/dual zero gap, hull')
 print('PASS all global-relaxation certificates; no LP solver invoked; no original cover inferred')
if __name__=='__main__':main()
