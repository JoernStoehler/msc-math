# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
from parallel_envelope import *
parser=argparse.ArgumentParser();parser.add_argument('--root',type=Path,default=Path.cwd());parser.add_argument('--steiner',type=Path,required=True);parser.add_argument('--out',type=Path,required=True);args=parser.parse_args()
if args.out.exists():raise FileExistsError(args.out)
f=load_fixture(args.root);F,Z=f.F,f.Z;fix=f.make_fixture()
def parse(cs):
 x=Z
 for q in cs:x=x*f.tau+F(f.sp.Rational(q))
 return x
T=[[[parse(c) for c in p] for p in s['vertices']] for s in fix['normal_shapes']]
source=args.steiner
r=json.loads(source.read_text())['results'][0];m=r['height_levels'];zs=[parse(c) for c in r['primal_variables']]
base=[[Z,Z]]+[[Z,z] for z in zs[2*m:]]
groups=[[[a,b+t[1]] for a,b in tri] for tri,t in zip(T,base)]
lp=build_lp(groups,F,f.sign);result,z=solve_and_certify(lp,f)
points=[]
for j,tri in enumerate(groups):
 shift=Z if j==0 else z[2*lp['m']+j-1]
 points.extend([[a+shift,b] for a,b in tri])
area=f.area(f.hull(points));H=parse(fix['candidate_K_area']);bound=f.tau/F(5)
if f.serialize(area)!=result['objective']:raise ArithmeticError('hull mismatch')
result.update(name='restore_centers_at_Steiner_optimum',base_translations=[[f.serialize(c) for c in t] for t in base],shear_slope=[],
 area_over_HKO=float(F.to_sympy(area/H).evalf()),area_over_steiner=float(F.to_sympy(area/bound).evalf()),
 center_restoration_gap=f.serialize(area-bound),center_restoration_gap_float=float(F.to_sympy(area-bound).evalf()),hull_area_check='PASS exact independent hull area')
out={'scope':'Exact original horizontal-shift slice optimum at one Steiner-optimal vertical placement; no global original-cover bound beyond relaxation',
 'HKO_area':fix['candidate_K_area'],'results':[result]}
p=args.out;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ['name','objective','objective_float','area_over_HKO','area_over_steiner','center_restoration_gap','center_restoration_gap_float']}))
