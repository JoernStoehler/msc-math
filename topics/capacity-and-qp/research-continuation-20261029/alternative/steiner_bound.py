# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
from parallel_envelope import *

def steiner(poly,f):
 F,Z=f.F,f.Z; pts=[]
 ys=sorted(set(p[1] for p in poly),key=cmp_to_key(lambda a,b:f.sign(a-b)))
 for y in ys:
  xx=[]
  for i,p in enumerate(poly):
   q=poly[(i+1)%len(poly)]
   if p[1]==y:xx.append(p[0])
   if f.sign((p[1]-y)*(q[1]-y))<0:xx.append(p[0]+(y-p[1])*(q[0]-p[0])/(q[1]-p[1]))
  xx.sort(key=cmp_to_key(lambda a,b:f.sign(a-b)))
  half=(xx[-1]-xx[0])/F(2)
  pts.extend([[-half,y],[half,y]])
 return f.hull(pts)

def main():
 p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path.cwd());p.add_argument('--out',type=Path,required=True);args=p.parse_args()
 if args.out.exists():raise FileExistsError(args.out)
 f=load_fixture(args.root);F,Z=f.F,f.Z;fix=f.make_fixture()
 def parse(cs):
  x=Z
  for q in cs:x=x*f.tau+F(f.sp.Rational(q))
  return x
 T=[[[parse(c) for c in p] for p in s['vertices']] for s in fix['normal_shapes']]
 H=parse(fix['candidate_K_area']);results=[]
 # Use rational shear coordinates of determinant one. This is parallel-chord
 # symmetrization in an affine frame; every such frame preserves physical area.
 for slope in [F(0),F(1)/F(7),F(1)/F(2),F(1)]:
  sheared=[[[x,y-slope*x] for x,y in tri] for tri in T]
  S=[steiner(tri,f) for tri in sheared]
  groups=[[[y,x] for x,y in tri] for tri in S]
  lp=build_lp(groups,F,f.sign);result,z=solve_and_certify(lp,f)
  pts=[]
  for j,tri in enumerate(groups):
   shift=Z if j==0 else z[2*lp['m']+j-1]
   pts.extend([[x+shift,y] for x,y in tri])
  hull=f.hull(pts);area=f.area(hull)
  if f.serialize(area)!=result['objective']:raise ArithmeticError('Hull comparison')
  result.update(shear_slope=f.serialize(slope),area_over_HKO=float(F.to_sympy(area/H).evalf()),
   relaxed_sys_upper_over_HKO=float(F.to_sympy(H/area).evalf()),
   steiner_templates=[[[f.serialize(c) for c in p] for p in tri] for tri in S],
   swapped_relaxed_hull=[[f.serialize(c) for c in p] for p in hull])
  results.append(result)
  print(json.dumps({'slope':str(F.to_sympy(slope)),'ratio':result['area_over_HKO'],'vars':lp['n'],'rows':len(lp['A']),'status':result['status']}),flush=True)
 args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps({'scope':'Global lower bound on arbitrary original translation covers via one parallel Steiner relaxation; not necessarily sharp','HKO_area':fix['candidate_K_area'],'results':results},indent=2)+'\n')
if __name__=='__main__':main()
