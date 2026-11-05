#!/usr/bin/env python3
"""Independent all-vertex factor LP checks via generic rational elimination.
Does not import analyze.py or contact scripts. No capacity labels used.
"""
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path
import argparse,hashlib,json,time

def solve(matrix,rhs):
    aug=[list(r)+[v] for r,v in zip(matrix,rhs)];n=len(matrix[0]);rank=0;piv=[]
    for c in range(n):
        p=next((i for i in range(rank,len(aug)) if aug[i][c]),None)
        if p is None:continue
        aug[rank],aug[p]=aug[p],aug[rank];z=aug[rank][c];aug[rank]=[v/z for v in aug[rank]]
        for i in range(len(aug)):
            if i!=rank:
                z=aug[i][c];aug[i]=[v-z*w for v,w in zip(aug[i],aug[rank])]
        piv.append(c);rank+=1
    if rank<n or any(not any(r[:-1]) and r[-1] for r in aug):return None
    w=[F(0)]*n
    for i,c in enumerate(piv):w[c]=aug[i][-1]
    return w

def vertices(normals):
    out=[]
    for n in (1,2,3):
        for ids in combinations(range(len(normals)),n):
            w=solve([[normals[i][c] for i in ids] for c in (0,1)]+[[F(1)]*n],[F(0),F(0),F(1)])
            if w is None or min(w)<=0:continue
            full=[F(0)]*len(normals)
            for i,v in zip(ids,w):full[i]=v
            out.append(full)
    assert out
    return out

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def main():
    p=argparse.ArgumentParser();p.add_argument('--source',type=Path,required=True);p.add_argument('--run',type=Path,required=True);args=p.parse_args();start=time.perf_counter();r=json.loads((args.run/'results.json').read_text());names={z['name'] for z in r['rows']};source={}
    assert hashlib.sha256(args.source.read_bytes()).hexdigest()==r['source_sha256']
    for line in args.source.open():
        x=json.loads(line)
        if x['name'] in names:source[x['name']]=x
    checks=[]
    for row in r['rows']:
        x=source[row['name']];k=x['k'];alln=[list(map(F,u)) for u in x['dual_vertices_rational']];a=[u[:2] for u in alln[:k]];b=[u[2:] for u in alln[k:]];va=vertices(a);vb=vertices(b);s=row['selection'];assert [len(va),len(vb)]==s['LP_vertex_counts'];seed_a=list(map(F,s['seed_alpha']));seed_b=list(map(F,s['seed_gamma']));parts=[]
        for v in s['variants']:
            pos={z:i for i,z in enumerate(v['word'])};af=list(map(F,v['alpha']));bf=list(map(F,v['gamma']))
            # Signed prefix vectors are independent direct coefficient derivation.
            pa=[[sum(seed_b[j]*b[j][d]*(1 if pos[i]<pos[k+j] else -1) for j in range(len(b)))/4 for d in (0,1)] for i in range(k)]
            ac=[dot(a[i],pa[i]) for i in range(k)];assert ac==list(map(F,v['A_coefficients']))
            before=dot(seed_a,ac);maximum=max(dot(w,ac) for w in va);after=dot(af,ac)
            assert before==F(v['seed_Q']) and after==F(v['after_A_Q']) and maximum==after and after>=before
            pb=[[sum(af[i]*a[i][d]*(1 if pos[i]<pos[k+j] else -1) for i in range(k))/4 for d in (0,1)] for j in range(len(b))]
            bc=[dot(b[j],pb[j]) for j in range(len(b))];assert bc==list(map(F,v['B_coefficients']))
            maximum=max(dot(w,bc) for w in vb);assert maximum==dot(bf,bc)==F(v['Q']);assert dot(seed_b,bc)==after and F(v['Q'])>=after
            if maximum==after:assert bf==seed_b
            if after==before:assert af==seed_a
            assert af in va and bf in vb
            parts.append(dict(shadow_sign=v['shadow_sign'],all_A_vertices_optimality='PASS',all_B_vertices_optimality='PASS',strict_tie_retention='PASS',signed_prefix_coefficients='PASS'))
        checks.append(dict(name=row['name'],factor_vertex_counts=[len(va),len(vb)],variants=parts))
    result=dict(status='PASS',scope='Generic Gaussian elimination checks every factor closure LP vertex, exact signed-prefix coefficients and one-sweep stage optimality; no analyze/contact import and no capacity labels',source_sha256=r['source_sha256'],results_sha256=hashlib.sha256((args.run/'results.json').read_bytes()).hexdigest(),controls=checks,elapsed_seconds=time.perf_counter()-start)
    (args.run/'lp-verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:result[k] for k in ['status','scope','elapsed_seconds']},indent=2))
if __name__=='__main__':main()
