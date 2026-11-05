#!/usr/bin/env python3
"""Fresh third geometry panel; no capacity label inputs."""
import argparse,hashlib,json
from fractions import Fraction as F
from pathlib import Path

def inv(A):
 d=A[0][0]*A[1][1]-A[0][1]*A[1][0];return [[A[1][1]/d,-A[0][1]/d],[-A[1][0]/d,A[0][0]/d]]
def mat(A,x):return [sum(a*b for a,b in zip(u,x)) for u in A]
def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;h=json.loads((o.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());q=F(22360679775,10000000000);base={k:[[F(v['rational'])+q*F(v['sqrt5']) for v in u] for u in h[k]] for k in ['A','B']};rows=[]
 for seed in range(5):
  raw=hashlib.sha256(f'contact-canonical-third-v1:{seed}'.encode()).digest();dirs=[F(v-128,128) for v in raw[:20]]
  for fam in ['full','support','angular']:
   for eps in [F(1,100),F(1,25),F(1,10)]:
    row=dict(name=f'canon_{fam}_s{seed}_e{eps.numerator}of{eps.denominator}',family=fam,seed=seed,epsilon=str(eps),direction_digest=raw.hex(),transformed_from=None)
    for ki,k in enumerate(['A','B']):
     vals=[]
     for i,(x,y) in enumerate(base[k]):
      j=ki*10+2*i
      if fam=='full':u,v=dirs[j:j+2]
      elif fam=='support':u,v=dirs[j]*x,dirs[j]*y
      else:u,v=-dirs[j]*y,dirs[j]*x
      vals.append([str(x+eps*u),str(y+eps*v)])
     row[k]=vals
    rows.append(row)
 G=[[F(3,2),F(1,3)],[F(1,4),F(5,4)]];Gi=inv(G);Git=list(map(list,zip(*Gi)));lam=F(7,6);translations={'A':[F(1,7),-F(1,9)],'B':[-F(1,8),F(1,10)]};original=rows[:]
 for r in original:
  if r['seed']>=3 or F(r['epsilon'])!=F(1,25):continue
  row={k:v for k,v in r.items() if k not in ['A','B']};row['name']=r['name']+'_group';row['transformed_from']=r['name']
  for k,M in [('A',Git),('B',G)]:
   vals=[]
   for u in r[k]:
    v=mat(M,list(map(F,u)));den=lam+sum(x*y for x,y in zip(v,translations[k]));assert den>0;vals.append([str(x/den) for x in v])
   row[k]=vals
  rows.append(row)
 out=dict(protocol='45 independent fresh rational perturbations +9 fixed groupcoordinate transforms; no truth labels',rows=rows,group_transform=dict(G=[[str(x) for x in u] for u in G],lambda_value=str(lam),translations={k:list(map(str,v)) for k,v in translations.items()}));(d/'canonical-sample.json').write_text(json.dumps(out,indent=2)+'\n');print('CANONICAL_SAMPLE_SHA256',hashlib.sha256((d/'canonical-sample.json').read_bytes()).hexdigest())
if __name__=='__main__':main()
