#!/usr/bin/env python3
import gzip,hashlib,json,math,sys
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
D=Path(__file__).parent;p=D/'broad-predictions.json';raw_sha=hashlib.sha256(p.read_bytes()).hexdigest();assert raw_sha=='81e6836510348e1870c850a0aa00e04d61d27248e7595e4de8e3c9d813834d3e';raw=json.loads(p.read_text());summary=[]
def encode(v):
 v=list(map(F,v));den=math.lcm(*(x.denominator for x in v));return dict(numerators=[str(x.numerator*(den//x.denominator)) for x in v],denominator=str(den))
with gzip.open(D/'broad-best-witnesses.jsonl.gz','wt',compresslevel=6) as g:
 for row in raw['rows']:
  z=row['selection'];best=z['best'];v=best['best'];a=[[F(x) for x in u] for u in z['centered_A']];b=[[F(x) for x in u] for u in z['centered_B']];ca,cb=[[F(x) for x in u] for u in z['centers']];alpha=[F(x)*(1+sum(q*c for q,c in zip(u,ca))) for x,u in zip(v['alpha'],a)];gamma=[F(x)*(1+sum(q*c for q,c in zip(u,cb))) for x,u in zip(v['gamma'],b)];assert sum(alpha)==sum(gamma)==1;g.write(json.dumps(dict(name=row['name'],word=best['word'],alpha=encode(alpha),gamma=encode(gamma),Q=v['Q']))+'\n');summary.append(dict(name=row['name'],capacity=float(F(z['capacity'])),ratio=float(F(z['ratio'])),volume=float(F(z['volume'])),selected_sign=best['sign'],selected_kind=v['kind'],selected_index=v.get('index'),singular_values=[v['singular_values'] for v in z['variants']],top_dimensions=[v['top_dimension'] for v in z['variants']]))
out=dict(rows=summary,raw_predictions_sha256=raw_sha,code_sha256=raw['code_sha256'],plan_sha256=raw['plan_sha256'],elapsed_seconds=raw['elapsed_seconds'],best_witness_sha256=hashlib.sha256((D/'broad-best-witnesses.jsonl.gz').read_bytes()).hexdigest());(D/'broad-frozen-summary.json').write_text(json.dumps(out,indent=2)+'\n');print('RAW_BYTES',p.stat().st_size,'WITNESS_GZIP_BYTES',(D/'broad-best-witnesses.jsonl.gz').stat().st_size,'SUMMARY_SHA256',hashlib.sha256((D/'broad-frozen-summary.json').read_bytes()).hexdigest())
