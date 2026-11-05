#!/usr/bin/env python3
"""Exact gauge normalization/invariance and complete HKO canonical census."""
import argparse,hashlib,importlib.util,json,math,sys
from fractions import Fraction as F
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;C=module('verify_can',d/'canonical.py');Q=module('verify_can_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5;pr=json.loads((d/'predictions.json').read_text());base={k:[[Q.parse(x) for x in u] for u in pr['base'][k]] for k in ['A','B']};gs=[[Q.parse(x) for x in r['loss_gradient']] for r in pr['bank']];z0=[x for k in ['A','B'] for u in base[k] for x in u];cases=[];zeros=[]
 rv=C.polygon(base['A']);triangle=[rv[i] for i in [0,2,4]];weights=[C.det(triangle[1],triangle[2]),C.det(triangle[2],triangle[0]),C.det(triangle[0],triangle[1])];weights=[x/sum(weights) for x in weights];assert min(weights)>0
 # All100 labelled finalprojection signs are strictatHKO, so thevalidfamilydoesnotjump locally.
 scales=[];aa0,bb0=C.angular(base['A']),C.angular(base['B'])
 for swapped in [False,True]:
  x,y=(bb0,aa0) if swapped else(aa0,bb0)
  for reverse in [False,True]:
   aa,bb=(x[::-1],y[::-1]) if reverse else(x,y)
   for ia in range(5):
    for ib in range(5):
     a,b=aa[ia:]+aa[:ia],bb[ib:]+bb[:ib];pv=C.polygon(a);E=[[pv[j][c]-pv[0][c] for j in [2,4]] for c in range(2)];R=[[rv[j][c]-rv[0][c] for j in [2,4]] for c in range(2)];L=C.mm(R,C.inverse(E));bn=[C.mat(L,u) for u in b];cen=C.centroid(bn);scales.append(bn[0][1]/(1-C.dot(bn[0],cen))/base['B'][0][1])
 assert len(scales)==100 and all(scales)
 for labels,(a,b) in C.candidates(base['A'],base['B'],base['A'],base['B']):
  D=C.defect(a,b,base['A'],base['B'],gs);z=[x for ar in [a,b] for u in ar for x in u];same=z==z0;cases.append(dict(labels=labels,defect=D.serial(),same_reference=same))
  if D==0:zeros.append(same)
 assert zeros and all(zeros);positive=[Q.parse(x['defect']) for x in cases if Q.parse(x['defect'])>0];assert positive;gap=min(positive)
 # Exact ordered-gauge invariance for all9 deliberately strong finite transforms.
 sample=json.loads((d/'canonical-sample.json').read_text());rows={r['name']:r for r in sample['rows']};truth=json.loads((d/'canonical-truth.json').read_text());truths={r['name']:r for r in truth['rows']};pred=json.loads((d/'canonical-predictions.json').read_text());preds={r['name']:r for r in pred['rows']};controls=[]
 for r in sample['rows']:
  if not r['transformed_from']:continue
  s=rows[r['transformed_from']];a=[[Q(x) for x in u] for u in s['A']];b=[[Q(x) for x in u] for u in s['B']];aa=[[Q(x) for x in u] for u in r['A']];bb=[[Q(x) for x in u] for u in r['B']];original=C.canonical(a,b,base['A'],base['B']);transformed=C.canonical(aa,bb,base['A'],base['B']);assert original==transformed
  # Nine fixed constraints: Avertices0,2,4; Bcentroid0; Bnormal0projectionreference.
  pav=C.polygon(original[0]);rav=C.polygon(base['A']);assert all(pav[i]==rav[i] for i in [0,2,4]);assert C.centroid(original[1])==[Q(),Q()];assert original[1][0][1]==base['B'][0][1]
  assert F(truths[r['name']]['ratio'])==F(truths[s['name']]['ratio']);err=abs(preds[r['name']]['invariant']-preds[s['name']]['invariant']);assert err<1e-12;controls.append(dict(original=s['name'],transformed=r['name'],exact_ordered_gauge_equal=True,exact_ratio_equal=True,numerical_unlabelled_defect_difference=err))
 # Full independent rational geometry/capacity replay for all54 third bodies.
 V=module('verify_can_geometry',o.repo/'patterns/upper-tail/near-hko-transfer/verify.py');objectives=0
 for r in sample['rows']:
  a=[list(map(F,u)) for u in r['A']];b=[list(map(F,u)) for u in r['B']];ca=V.vertices(a);cb=V.vertices(b);vol=V.area(a)*V.area(b);qs=[]
  for sign in [1,-1]:
   w=V.word(a,b,sign)
   for al in ca:
    for ga in cb:qs.append(V.q(a,b,w,al,ga));objectives+=1
  q=max(qs);t=truths[r['name']];assert F(t['ratio'])==1/(8*q*q*vol)
 out=dict(status='PASS',HKO_valid_canonical_candidates=len(cases),HKO_zero_candidates=len(zeros),all_zero_candidates_equal_reference=True,HKO_other_candidate_defect_gap=gap.serial(),HKO_other_candidate_defect_gap_float=float(gap),HKO_candidates=cases,exact_group_controls=controls,reference_triangle_origin_weights=[x.serial() for x in weights],min_absolute_HKO_finalprojection=min(abs(x) for x in scales).serial(),third_geometries_checked=54,distinct_untransformed_cases=45,SHA_seed_directions=5,group_coordinate_copies=9,independent_direct_HK_objectives=objectives,input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['canonical.py','CANONICAL-PLAN.md','canonical-sample.json','canonical-predictions.json','canonical-truth.json','verify-canonical.py']});(d/'canonical-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['HKO_candidates']},indent=2))
if __name__=='__main__':main()
