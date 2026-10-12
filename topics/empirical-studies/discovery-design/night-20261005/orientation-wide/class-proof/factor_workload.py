# /// script
# dependencies = ["sympy"]
# ///
"""Exact bounded factor census. Writes only sibling factor-workload.json."""
from pathlib import Path
import json,time,hashlib,sys
import sympy as s

def main():
    start=time.perf_counter();root=Path(__file__).resolve().parent;relevant='--clipped' in sys.argv
    source=root/('clipped.json' if relevant else 'formulas.json')
    packet=json.loads(source.read_text());u=s.Symbol('u');z=s.sqrt(5);F=s.QQ.algebraic_field(z)
    def decode(v):return s.Rational(v[0])+s.Rational(v[1])*z
    def encode(v):
        q=F.convert(v).to_list()
        return [str(q[-1]),str(q[0]) if len(q)==2 else '0'] if q else ['0','0']
    def coeffs(poly):return [encode(v) for v in reversed(poly.rep.to_list())]
    factors={};records=[]
    for i,rec in enumerate(packet['polynomials']):
        p=s.Poly(sum(decode(v)*u**j for j,v in enumerate(rec['coefficients'])),u,domain=F)
        unit,fac=p.factor_list();fs=[]
        for q,multiplicity in fac:
            q=q.monic();cs=coeffs(q);key=json.dumps(cs,separators=(',',':'))
            if key not in factors:factors[key]=len(records);records.append(dict(coefficients=cs,degree=q.degree(),occurrences=[]))
            idx=factors[key];records[idx]['occurrences'].append(dict(polynomial=i,multiplicity=multiplicity,
                kinds=sorted({v['kind'] for v in rec['occurrences']})))
            fs.append(dict(factor=idx,multiplicity=multiplicity))
        rec['factorization']=dict(unit=encode(unit),factors=fs)
    out=dict(status='Exact factor census; relevance/positivity unproved',
        input=str(source),input_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
        polynomial_count=len(packet['polynomials']),factors=records,
        polynomial_factorizations=[p['factorization'] for p in packet['polynomials']],
        factor_degree_counts={str(d):sum(r['degree']==d for r in records) for d in range(1,5)},
        cubic_stationary_membership_factors=[i for i,r in enumerate(records) if r['degree']==3 and any('stationary_membership' in o['kinds'] for o in r['occurrences'])],
        quartic_factors=[i for i,r in enumerate(records) if r['degree']==4],
        elapsed_seconds=round(time.perf_counter()-start,3))
    (root/('factor-relevant.json' if relevant else 'factor-workload.json')).write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['polynomial_count','factor_degree_counts','cubic_stationary_membership_factors','quartic_factors','elapsed_seconds']}))

if __name__=='__main__':main()
