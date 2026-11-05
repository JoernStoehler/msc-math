#!/usr/bin/env python3
"""Independent scalar verification; no imports from producer, no capacity solve."""
import argparse, csv, gzip, hashlib, json, math
from pathlib import Path
import numpy as np
ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,default=Path('patterns/product-shape/features.csv.gz'));ap.add_argument('--run',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args()
source={r['name']:r for r in map(json.loads,args.source.open())}
with gzip.open(args.run/'rows.jsonl.gz','rt') as f:rows=list(map(json.loads,f))
summary=json.loads((args.run/'summary.json').read_text())
assert len(source)==len(rows)==summary['rows']==10240
assert {r['name'] for r in rows}==set(source)
assert hashlib.sha256(args.source.read_bytes()).hexdigest()==summary['source_sha256']
assert hashlib.sha256(args.width.read_bytes()).hexdigest()==summary['width_sha256']
with gzip.open(args.width,'rt') as f:baseline={r['name']:r for r in csv.DictReader(f)}
assert set(baseline)==set(source)
max_violation=max_capacity_error=max_coverage_error=max_volume_error=0.
normal=[(math.cos(math.pi/5+2*math.pi*j/5),math.sin(math.pi/5+2*math.pi*j/5)) for j in range(5)]
vertex=[(math.cos(2*math.pi*j/5),math.sin(2*math.pi*j/5)) for j in range(5)]
area5=2.5*math.sin(2*math.pi/5)
def inverse(A):
 a,b=A[0];c,d=A[1];D=a*d-b*c
 assert D!=0
 return [[d/D,-b/D],[-c/D,a/D]]
def mul(A,B):return [[sum(A[i][k]*B[k][j] for k in range(2)) for j in range(2)] for i in range(2)]
def transpose(A):return [[A[j][i] for j in range(2)] for i in range(2)]
def signed_area(v):return sum(v[i][0]*v[(i+1)%len(v)][1]-v[i][1]*v[(i+1)%len(v)][0] for i in range(len(v)))/2
for r in rows:
 s=source[r['name']];assert s['capacity']==r['capacity'] and s['sys']==r['sys']
 assert s['k']==r['k'] and s['m']==r['m']
 assert float(baseline[r['name']]['c2'])==r['width']
 a=s['dual_vertices'];halves=[[v[:2] for v in a[:s['k']]], [v[2:] for v in a[s['k']:]]]
 original_areas=[]
 for key,hs in zip(['p','q'],halves):
  p=r[key];G=p['G'];cx,cy=p['center'];corners=[(cx+G[0][0]*x+G[0][1]*y,cy+G[1][0]*x+G[1][1]*y) for x,y in vertex]
  violation=max(nx*x+ny*y-1 for nx,ny in hs for x,y in corners)
  max_violation=max(max_violation,violation);assert violation<1e-8
  hs=sorted(hs,key=lambda v:math.atan2(v[1],v[0]));orig=[]
  for i,(ax,ay) in enumerate(hs):
   bx,by=hs[(i+1)%len(hs)];D=ax*by-ay*bx
   orig.append(((by-ay)/D,(ax-bx)/D))
  A=abs(signed_area(orig));Ac=abs(signed_area(corners));original_areas.append(A)
  max_coverage_error=max(max_coverage_error,abs(Ac/A-p['coverage']))
  assert abs(Ac/A-p['coverage'])<1e-9
  assert abs(A/p['area_original']-1)<1e-9
  assert abs(Ac/p['area_core']-1)<1e-9
 M=mul(inverse(r['p']['G']),transpose(inverse(r['q']['G'])))
 denom=max(abs(sum(x[i]*M[i][j]*y[j] for i in range(2) for j in range(2))) for x in normal for y in normal)
 L=(1+math.cos(math.pi/5))**2/denom
 max_capacity_error=max(max_capacity_error,abs(L/r['L']-1));assert abs(L/r['L']-1)<1e-10
 V=original_areas[0]*original_areas[1];max_volume_error=max(max_volume_error,abs(V/s['volume']-1));assert abs(V/s['volume']-1)<1e-8
 assert L<=r['capacity']*(1+1e-8) and r['capacity']<=r['width']*(1+1e-8)
 assert abs(r['prediction']**2/(L*r['width'])-1)<1e-10
 assert abs(r['prediction_sys']/(L*r['width']/(2*V))-1)<1e-8
 role='calibration' if int(hashlib.sha256(r['name'].encode()).hexdigest()[:8],16)%5==0 else 'diagnostic'
 assert r['role']==role

def score(rr,ss):
 assert len(rr)==ss['n']
 for tag,field in [('core','L'),('center','prediction'),('width','width')]:
  capacity=[abs(r[field]/r['capacity']-1) for r in rr]
  sysfield={'L':'core_sys','prediction':'prediction_sys','width':'width_sys'}[field]
  ratio=[abs(r[sysfield]/r['sys']-1) for r in rr]
  mae=sum(abs(r[field]-r['capacity']) for r in rr)/len(rr)
  assert abs(mae-ss[tag]['capacity_MAE'])<1e-9
  for name,q in [('min',0),('q10',.1),('median',.5),('q90',.9),('max',1)]:
   assert abs(float(np.quantile(capacity,q))-ss[tag]['capacity_absolute_relative'][name])<1e-9
   assert abs(float(np.quantile(ratio,q))-ss[tag]['sys_absolute_relative'][name])<1e-9
 improved=sum(abs(r['prediction']-r['capacity'])<abs(r['width']-r['capacity'])-1e-12*max(1,r['capacity']) for r in rr)
 assert improved==ss['center_beats_width_capacity_count']
 if ss['width']['capacity_MAE']:
  assert abs(ss['center_over_width_capacity_MAE']-ss['center']['capacity_MAE']/ss['width']['capacity_MAE'])<1e-10
score(rows,summary['all'])
score([r for r in rows if r['role']=='diagnostic'],summary['diagnostic'])
score([r for r in rows if r['role']=='calibration'],summary['calibration'])
for bucket,ss in summary['buckets'].items():
 rr=[r for r in rows if r['bucket']==bucket]
 assert len(rr)==1024
 # Membership determined independently from original source labels only.
 names=sorted([r['name'] for r in rr],key=lambda name:(-source[name]['sys'],name))[:21]
 assert names==ss['top2_names']
 lookup={r['name']:r for r in rr};top=[lookup[n] for n in names]
 score(rr,ss['all']);score([r for r in rr if r['role']=='diagnostic'],ss['diagnostic']);score(top,ss['top2']);score([r for r in top if r['role']=='diagnostic'],ss['diagnostic_top2'])
ctrl=summary['HKO_control'];hko=(3+math.sqrt(5))/5
assert abs(ctrl['hko']['predicted_sys']-hko)<1e-12
for c in ctrl['regular_rotation_controls']:
 true=(1+math.cos(math.pi/5))**2/math.cos(c['theta'])
 assert abs(c['L']/true-1)<1e-12
 assert abs(c['coverage_product']-1)<1e-12
result=dict(status='PASS',rows=len(rows),factors=2*len(rows),checked='original halfplane containment, scalar25-pair capacity formula, shoelace areas, historical source labels and pinned width baseline, bracket arithmetic, frozen partition, all10 top21 memberships, all/calibration/diagnostic/per-family/tail scores, separate regular/HKO scalar control',max_halfplane_violation=max_violation,max_relative_scalar_capacity_error=max_capacity_error,max_coverage_error=max_coverage_error,max_relative_volume_error=max_volume_error,limits='No new capacity solving or historical label certification; binary64 verification, not exact containment certification.')
args.out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
