#!/usr/bin/env python3
"""Independent boundedness/support/inclusion check for the pruning sandwich."""
import argparse,gzip,hashlib,importlib.util,itertools,json,sys
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('prune_independent',D/'verify.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
def dot(x,y):return sum(a*b for a,b in zip(x,y))
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,default=D/'pruning-inclusion-verification.json');args=parser.parse_args()
 path=D/'pruning-prediction.json'
 if not path.exists():path=D/'pruning-certificates.json.gz'
 p=json.loads(gzip.decompress(path.read_bytes()) if path.suffix=='.gz' else path.read_bytes());op=D/'facet-prediction.json';original=json.loads(op.read_text() if op.exists() else gzip.decompress((D/'facet-certificates.json.gz').read_bytes()))['rows'][0]['selection'];rows=[]
 for i,r in enumerate(p['pruning']):
  a=[[F(t) for t in x] for x in r['original_centered_normals']];outer=[[F(t) for t in x] for x in r['outer_normals_at_original_centroid']];weight=list(map(F,original['canonical_closures'][i]));chosen=r['deleted_index'];assert weight[chosen]==min(weight) and weight.count(min(weight))==1 and outer==a[:chosen]+a[chosen+1:]
  closure=v.closures(outer);positive=[sum(x[j] for x in closure)/len(closure) for j in range(len(outer))];assert min(positive)>0 and sum(positive)==1 and all(sum(positive[j]*outer[j][k] for j in range(len(outer)))==0 for k in [0,1]);assert any(v.cross(x,y) for x,y in itertools.combinations(outer,2))
  vertices=[]
  for x,y in itertools.combinations(outer,2):
   point=v.solve([x,y],[F(1),F(1)])
   if point is not None and all(dot(n,point)<=1 for n in outer) and point not in vertices:vertices.append(point)
  assert sorted(vertices)==sorted([[F(t) for t in x] for x in r['outer_vertices_at_original_centroid']]);supports=[max(dot(n,x) for x in vertices) for n in a];assert supports==list(map(F,r['all_original_supports_on_outer']));radius=F(r['R']);assert radius==max(supports)>=1 and all(dot(n,[t/radius for t in x])<=1 for n in a for x in vertices)
  rows.append(dict(factor=r['factor'],status='PASS',unique_minimum_deleted=chosen,outer_bounded_by_positive_closure=True,R=str(radius),all_original_halfspaces_verified_on_scaled_outer_vertices=True))
 outfeature=p['rows'][0]['selection'];product=F(p['pruning'][0]['R'])*F(p['pruning'][1]['R']);lower=F(outfeature['eta'])*F(outfeature['capacity'])/product;upper=F(outfeature['capacity']);assert lower==F(p['lower_capacity']) and upper==F(p['upper_capacity']);assert F(p['combined_lower'])==max(lower,F(original['lower_capacity'])) and F(p['combined_upper'])==min(upper,F(original['capacity']))
 result=dict(status='PASS',rows=rows,input_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),scope='Independent exact facet selection, outer boundedness, full original-centroid support dilations and sandwich endpoint arithmetic; unchanged verify.py checks outer chord-star')
 args.out.write_text(json.dumps(result,indent=2)+'\n');print('PASS pruning sandwich')
if __name__=='__main__':main()
