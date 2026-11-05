#!/usr/bin/env python3
"""Independent rational check of the facet-birth limit hypotheses."""
import argparse,hashlib,importlib.util,itertools,json,math,sys
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
def dot(x,y):return sum(a*b for a,b in zip(x,y))
def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=Path(__file__).resolve().parent);p.add_argument('--repo',type=Path,default=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17'));p.add_argument('--out',type=Path);o=p.parse_args();d=o.directory
 spec=importlib.util.spec_from_file_location('chord_independent',d/'verify.py');v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
 data=json.loads((d/'facet-limit.json').read_text());rows=[];source=o.repo/'patterns/upper-tail/facet-transfer/sample.json';sample=json.loads(source.read_text())['rows'][0];assert sample['name']==data['name']
 for key,row in data['limits'].items():
  a=[[F(t) for t in x] for x in row['limiting_centered_normals']];c=list(map(F,row['limiting_center']));n=len(a);assert n==6 and min(c[:5])>0 and c[5]==0 and sum(c)==1 and all(sum(ci*ai[k] for ci,ai in zip(c,a))==0 for k in [0,1])
  raw=[[F(t) for t in x] for x in sample[key][:5]];vertices=[]
  for x,y in itertools.combinations(raw,2):
   point=v.solve([x,y],[F(1),F(1)])
   if point is not None and all(dot(z,point)<=1 for z in raw) and point not in vertices:vertices.append(point)
  mean=[sum(x[k] for x in vertices)/len(vertices) for k in [0,1]];vertices.sort(key=lambda x:math.atan2(float(x[1]-mean[1]),float(x[0]-mean[0])));twice=sum(v.cross(x,vertices[(i+1)%len(vertices)]) for i,x in enumerate(vertices));centroid=[sum((x[k]+vertices[(i+1)%len(vertices)][k])*v.cross(x,vertices[(i+1)%len(vertices)]) for i,x in enumerate(vertices))/(3*twice) for k in [0,1]];assert centroid==list(map(F,row['old_center']))
  i,j=sample['cut_pairs'][key];raw.append([(raw[i][k]+raw[j][k])/2 for k in [0,1]]);assert a==[[t/(1-dot(x,centroid)) for t in x] for x in raw]
  weights=[F(0)]*5
  for i,x in enumerate(vertices):
   y=vertices[(i+1)%len(vertices)];active=[j for j,z in enumerate(raw[:5]) if dot(z,x)==dot(z,y)==1];assert len(active)==1;weights[active[0]]=v.cross([x[k]-centroid[k] for k in [0,1]],[y[k]-centroid[k] for k in [0,1]])/twice
  assert weights==c[:5]
  h=[[F(1)]*n]+list(map(list,zip(*a)));gram=[[dot(x,y) for y in h] for x in h];p=[]
  for j in range(n):
   z=v.solve(gram,[h[k][j] for k in range(3)]);p.append([F(i==j)-sum(h[k][i]*z[k] for k in range(3)) for i in range(n)])
  newrow=[p[j][5] for j in range(n)];assert newrow==list(map(F,row['projector_new_coordinate_row'])) and all(newrow)
  long=[]
  for ray in p:
   positive=ray[5]>0
   t=min(-x/u for x,u in zip(c,ray) if u<0) if positive else max(-x/u for x,u in zip(c,ray) if u>0)
   endpoint=[x+t*u for x,u in zip(c,ray)];assert min(endpoint)>=0 and endpoint[5]>0 and sum(endpoint)==1
   long.append(endpoint)
  # The only limiting star points on coordinate5=0 are the center itself.
  assert all(point[5]>0 for point in long)
  old=[list(map(F,x)) for x in row['old_closure_vertices']];assert len(old)>1 and all(point!=c[:5] for point in old)
  for point in old:assert sum(point)==1 and min(point)>=0 and all(sum(point[i]*a[i][k] for i in range(5))==0 for k in [0,1])
  rows.append(dict(factor=key,status='PASS',all_six_projector_new_coordinates_nonzero=True,all_six_noncentral_limit_endpoints_strictly_positive_new_coordinate=True,old_closure_face_vertices=len(old)))
 result_source=o.repo/'patterns/upper-tail/facet-transfer/results.json';truth=json.loads(result_source.read_text())['rows'][0];assert truth['name']==sample['name'];winner=truth['winners'][0];alpha=list(map(F,winner['alpha']));gamma=list(map(F,winner['gamma']));assert alpha[5]==gamma[5]==0
  # Strip zero-weight new labels and reindex B labels from6..10 to5..9.
 word=[label if label<5 else label-1 for label in winner['word'] if label not in [5,11]];a=[[F(t) for t in x] for x in sample['A'][:5]];b=[[F(t) for t in x] for x in sample['B'][:5]];q=v.direct(a,b,word,alpha[:5],gamma[:5]);assert q==F(truth['Q_true']) and 1/(2*q)==F(truth['capacity_true'])
 out=dict(status='PASS',rows=rows,capacity_inactive_cut=dict(status='PASS',added_weights_both_zero=True,outer_witness_Q=str(q),imported_exact_capacity=truth['capacity_true'],deduction='outer feasible witness gives c_outer<=c_cut; inclusion gives reverse, hence equality'),scope='Exact source-linked hypotheses for eta(delta)->0 and capacity-inactive finite cut; finite prediction checked by unchanged verify.py',source_sha256=hashlib.sha256((d/'facet-limit.json').read_bytes()).hexdigest(),sample_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),imported_truth_sha256=hashlib.sha256(result_source.read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
 (o.out or d/'facet-limit-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
