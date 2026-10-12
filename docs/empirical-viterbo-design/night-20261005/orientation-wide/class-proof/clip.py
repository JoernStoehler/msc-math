#!/usr/bin/env python3
"""Exact candidate feasibility clipping; writes only sibling clipped.json.

Uses accepted degree-two event engine. Open-domain sign predicates are derived,
not declared passed. Closed original fibers are tested exactly in F(alpha).
"""
from derive import *
import sys

def main():
    started=time.perf_counter();form=json.loads((ROOT/'formulas.json').read_text())
    joint=json.loads((ROOT.parent/'joint-denominators'/'joint-denominators.json').read_text());U=K(0,'1/5')
    formulas=form['formulas'];lookup={(r['vector'],tuple(r['fit_triple'])):i for i,r in enumerate(formulas)}
    matrices=[tuple(v['rows']) for v in form['fit_matrices']]
    cache={};polyrecords={};segments=[];excluded=[];closed=[];zeros=[];eventdomains=[]
    def roots(p):
        p=trim(p)
        if all(v==0 for v in p):return []
        key=coeff_key(scale(p,K(1)/p[-1]))
        if key not in cache:cache[key]=engine.roots_in_strip(p+[K(0)]*(3-len(p)),U)
        return cache[key]
    def bound(n,d,label):return dict(n=trim(n),d=trim(d),label=label)
    def bv(b,u):return evaluate(b['n'],u)/evaluate(b['d'],u)
    def eb(b):return dict(numerator=encode(b['n']),denominator=encode(b['d']),label=b['label'])
    def register(p,kind,owner):
        p=trim(p)
        if all(v==0 for v in p):return dict(zero=True)
        key=coeff_key(scale(p,K(1)/p[-1]))
        if key not in polyrecords:polyrecords[key]=dict(coefficients=encode(scale(p,K(1)/p[-1])),degree=len(p)-1,occurrences=[])
        polyrecords[key]['occurrences'].append(dict(kind=kind,owner=owner,multiplier=p[-1].data()))
        return dict(key=str(list(polyrecords).index(key)),multiplier=p[-1].data())
    for di,domain in enumerate(form['open_domains']):
        si,ti,ci=domain['incidence'];s=joint['joint']['topology_strips'][si];t=s['substrips'][ti];cell=t['cells'][ci]
        ulo,uhi=engine.decode_u(t['left']),engine.decode_u(t['right'])
        initiallo=bound([K(0)],[K(1)],'x0') if cell['left_boundary'] is None else bound(decode(s['curves'][cell['left_boundary'][0]]['numerator']),decode(s['curves'][cell['left_boundary'][0]]['denominator']),['Dleft',cell['left_boundary']])
        initialhi=None if cell['right_boundary'] is None else bound(decode(s['curves'][cell['right_boundary'][0]]['numerator']),decode(s['curves'][cell['right_boundary'][0]]['denominator']),['Dright',cell['right_boundary']])
        for triple in matrices:
            fi=lookup[(domain['vector'],triple)];f=formulas[fi];residuals=[list(map(decode,r)) for r in f['residuals']]
            zeroedges=[i for i,(r,slope) in enumerate(residuals[5:],5) if all(v==0 for v in r+slope)]
            if zeroedges:excluded.append(dict(domain=di,formula=fi,zero_edge_rows=zeroedges));continue
            bounds=[initiallo]+([] if initialhi is None else [initialhi])+[bound(neg(r),sl,['residual',i]) for i,(r,sl) in enumerate(residuals) if any(v!=0 for v in sl)]
            cuts={ulo,uhi}
            for r,sl in residuals:
                for p in [r,sl]:cuts.update(root for root in roots(p) if ulo<root<uhi)
            for p,q in combinations(bounds,2):
                numerator=sub(mul(p['n'],q['d']),mul(q['n'],p['d']))
                cuts.update(root for root in roots(numerator) if ulo<root<uhi)
            ordered=sorted(cuts)
            eventdomains.append(dict(domain=di,formula=fi,internal_cuts=[u.data() for u in ordered if ulo<u<uhi]))
            for left,right in zip(ordered,ordered[1:]):
                u=engine.field_sample(left,right);lo=initiallo;hi=initialhi;okay=True
                for i,(r,sl) in enumerate(residuals):
                    rv,sv=evaluate(r,u),evaluate(sl,u)
                    if sv==0:
                        if rv<0:okay=False;break
                    else:
                        b=bound(neg(r),sl,['residual',i])
                        if sv>0 and bv(b,u)>bv(lo,u):lo=b
                        if sv<0 and (hi is None or bv(b,u)<bv(hi,u)):hi=b
                if not okay or (hi is not None and bv(hi,u)<bv(lo,u)):continue
                if hi is not None and bv(hi,u)<=0:continue
                q=list(map(decode,f['gap']));owner=len(segments)
                checks=[]
                for label,b in [('lower',lo),('upper',hi)]:
                    if b is None:continue
                    n,d=b['n'],b['d'];p=add(add(mul(q[0],mul(d,d)),mul(q[1],mul(n,d))),mul(q[2],mul(n,n)))
                    checks.append(dict(label=label,bound=eb(b),gap=register(p,'endpoint_gap',owner),
                        station_membership=register(add(mul(q[1],d),scale(mul(q[2],n),K(2))),'stationary_membership',owner)))
                stat=sub(scale(mul(q[0],q[2]),K(4)),mul(q[1],q[1]))
                segments.append(dict(domain=di,formula=fi,left=left.data(),right=right.data(),sample_u=u.data(),
                    lower=eb(lo),upper=None if hi is None else eb(hi),checks=checks,
                    stationary_gap=register(stat,'stationary_gap',owner),
                    q2=register(q[2],'q2',owner),q1=register(q[1],'q1',owner),
                    singleton=hi is not None and bv(hi,u)==bv(lo,u)))
        if di%25==0:print(json.dumps(dict(open_domain=di,segments=len(segments),polynomials=len(polyrecords))),flush=True)
    out=dict(status='Open feasibility partition and relevance predicates; sign proof pending',
        formula_input_sha256=hashlib.sha256((ROOT/'formulas.json').read_bytes()).hexdigest(),
        excluded_zero_edges=excluded,open_segments=segments,
        polynomials=list(polyrecords.values()),degree_counts={str(d):sum(p['degree']==d for p in polyrecords.values()) for d in range(5)},
        root_polynomial_count=len(cache),elapsed_seconds=round(time.perf_counter()-started,3))
    if '--events-only' in sys.argv:
        events=dict(status='All internal feasibility event inputs; closed checks pending',
            formula_sha256=hashlib.sha256((ROOT/'formulas.json').read_bytes()).hexdigest(),
            event_domains=eventdomains,elapsed_seconds=out['elapsed_seconds'])
        (ROOT/'feasibility-events.json').write_text(json.dumps(events,indent=2)+'\n')
    else:(ROOT/'clipped.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['degree_counts','root_polynomial_count','elapsed_seconds']}));print('segments',len(segments))

if __name__=='__main__':main()
