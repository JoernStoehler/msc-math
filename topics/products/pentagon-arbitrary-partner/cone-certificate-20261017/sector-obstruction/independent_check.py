#!/usr/bin/env python3
"""Independent full ambient dual check using integral field rows and Bareiss minors.
No optimizer, imports from producer, equality pruning or float signs.
"""
import argparse,json,math,hashlib,time
from fractions import Fraction as R
from pathlib import Path

def add(x,y):return x[0]+y[0],x[1]+y[1]
def mul(x,y):return x[0]*y[0]+5*x[1]*y[1],x[0]*y[1]+x[1]*y[0]
def scale(c,x):return c*x[0],c*x[1]
def sign(x):
 a,b=x
 if a==0:return (b>0)-(b<0)
 if b==0:return (a>0)-(a<0)
 if a*b>0:return (a>0)-(a<0)
 d=a*a-5*b*b
 return ((d>0)-(d<0))*((a>0)-(a<0))
def dot(a,b):
 ans=(0,0)
 for x,y in zip(a,b):ans=add(ans,mul(x,y))
 return ans

def det(a):
 a=[r[:] for r in a];previous=1
 for k in range(len(a)-1):
  assert a[k][k]!=0
  for i in range(k+1,len(a)):
   for j in range(k+1,len(a)):
    numerator=a[i][j]*a[k][k]-a[i][k]*a[k][j]
    assert numerator%previous==0
    a[i][j]=numerator//previous
  previous=a[k][k]
  for i in range(k+1,len(a)):a[i][k]=0
 return a[-1][-1]

def main():
 p=argparse.ArgumentParser();p.add_argument('--source',type=Path,required=True);p.add_argument('--dual',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();start=time.perf_counter()
 data=json.loads(args.source.read_text());dual=json.loads(args.dual.read_text());assert hashlib.sha256(args.source.read_bytes()).hexdigest()==dual['source_sha256']
 field=lambda x:tuple(R(c) for c in x)
 rows=[[field(c) for c in r['coefficients']] for r in data['rows']];q=[[field(c) for c in r] for r in data['positive_det_quadratic_matrix']];z0=[field(c) for c in data['hko_control']['point']];assert z0[-1]==(1,0)
 t=(R(5),R(-2));zero=(R(0),R(0));one=(R(1),R(0))
 rows.extend([[zero,zero,zero,zero,scale(-1,t),one,zero],[zero,zero,zero,t,t,scale(-1,one),t]])
 assert len(rows)==612
 c=dual['lambda_square_moment'];v=list(map(R,dual['dual_cross']));d=[list(map(R,r)) for r in dual['dual_y_matrix']];moment=[[R(c)]+v]+[[v[i]]+d[i] for i in range(6)]
 mscale=math.lcm(*(x.denominator for row in moment for x in row));mi=[[int(x*mscale) for x in r] for r in moment]
 minors=[det([r[:k] for r in mi[:k]]) for k in range(1,8)];assert all(x>0 for x in minors)
 transformed=[[dot(r,z0)]+r[:6] for r in rows]
 ints=[]
 for row in transformed:
  den=math.lcm(*(x.denominator for p in row for x in p));ints.append([(int(a*den),int(b*den)) for a,b in row])
 mr=[]
 for row in ints:
  mr.append([tuple(sum(mi[i][j]*row[j][k] for j in range(7)) for k in range(2)) for i in range(7)])
 count=0
 for i in range(612):
  for j in range(i,612):
   value=dot(ints[i],mr[j]);assert sign(value)>0,(i,j,value);count+=1
 assert count==187578
 # Full congruence of target. Column0=z0, remaining columns=e0..e5.
 basis=[z0]+[[one if i==j else zero for i in range(7)] for j in range(6)]
 qp=[[dot(a,[dot(r,b) for r in q]) for b in basis] for a in basis]
 pairing=(0,0)
 for i in range(7):
  for j in range(7):pairing=add(pairing,scale(moment[i][j],qp[i][j]))
 assert sign(pairing)<0 and list(map(str,pairing))==dual['target_pairing']
 receipt=dict(status='PASS',source_sha256=dual['source_sha256'],dual_sha256=hashlib.sha256(args.dual.read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),method='all612row-pair signs in integral Q(sqrt5); positive definite via7Bareiss leading minors; full congruence target trace',row_count=612,strict_positive_product_checks=count,moment_integer_scale=mscale,positive_leading_principal_minors=[str(x) for x in minors],target_pairing=list(map(str,pairing)),elapsed_seconds=time.perf_counter()-start)
 args.out.write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt))
if __name__=='__main__':main()
