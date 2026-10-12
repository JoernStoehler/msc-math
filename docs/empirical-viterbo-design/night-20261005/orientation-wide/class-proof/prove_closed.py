#!/usr/bin/env python3
"""Exact closed original and internal feasibility-fiber proof.

Writes only closed-proof.json. All arithmetic remains in the actual F(alpha)
embedding of each u; no averaging or ordering roots from distinct fields.
"""
from derive import *

def main():
    start=time.perf_counter();form=json.loads((ROOT/'formulas.json').read_text());joint=json.loads((ROOT.parent/'joint-denominators'/'joint-denominators.json').read_text())
    events=json.loads((ROOT/'feasibility-events.json').read_text());matrices=[tuple(v['rows']) for v in form['fit_matrices']]
    lookup={(f['vector'],tuple(f['fit_triple'])):i for i,f in enumerate(form['formulas'])}
    checks=[];mapping=[];failures=[];equality=[];excluded=[];empty=0
    def verify(fi,u,left,right,owner):
        nonlocal empty
        f=form['formulas'][fi];residuals=[list(map(decode,r)) for r in f['residuals']]
        lo=left;hi=right
        for i,(r,s) in enumerate(residuals):
            rv,sv=evaluate(r,u),evaluate(s,u)
            if sv==0:
                if rv<0:empty+=1;return
            elif sv>0:
                bound=-rv/sv
                if bound>lo:lo=bound
            else:
                bound=-rv/sv
                if hi is None or bound<hi:hi=bound
            if hi is not None and hi<lo:empty+=1;return
        if hi is not None and hi<=0:empty+=1;return
        q=[evaluate(decode(p),u) for p in f['gap']]
        samples=[('lower',lo)]+([] if hi is None else [('upper',hi)])
        if q[2]>0:
            stationary=-q[1]/(2*q[2])
            if stationary>=lo and (hi is None or stationary<=hi):samples.append(('stationary',stationary))
        if hi is None and (q[2]<0 or(q[2]==0 and q[1]<0)):
            failures.append(dict(owner=owner,formula=fi,kind='negative unbounded gap tail'));return
        values=[]
        for label,x in samples:
            if x==0:continue # excluded endpoint; handled by limit sign separately
            hh=[evaluate(decode(coord[0]),u)+x*evaluate(decode(coord[1]),u) for coord in f['supports']]
            edgevalues=[evaluate(r,u)+x*evaluate(s,u) for r,s in residuals[5:]]
            value=q[0]+q[1]*x+q[2]*x*x
            values.append(dict(label=label,x=x.data(),gap=value.data(),sign=value.sign(),
                edge_signs=[v.sign() for v in edgevalues]))
            if value<0:failures.append(dict(owner=owner,formula=fi,x=x.data(),gap=value.data()))
            if value==0:
                assert all(v>0 for v in edgevalues),'Zero-edge equality contradicts strict HKO primary bound'
                equality.append(dict(owner=owner,formula=fi,u=u.data(),x=x.data(),supports=[h.data() for h in hh]))
        if lo==0 and q[0]<0:failures.append(dict(owner=owner,formula=fi,kind='negative gap near excluded x0'))
        checks.append(dict(owner=owner,formula=fi,u=u.data(),lower=lo.data(),upper=None if hi is None else hi.data(),
            q=[v.data() for v in q],tests=values,identically_zero_gap=all(v==0 for v in q)))
    for domain in form['closed_domains']:
        fiber,cell=domain['incidence'];f=joint['joint']['closed_fibers'][fiber];c=f['cells'][cell]
        u=engine.decode_u(f['u']);lo=engine.decode_value(c['left']);hi=None if c['right'] is None else engine.decode_value(c['right'])
        vi=domain['vectors'][0];ds=form['vectors'][vi]
        # Exact intercept/slope equality against the raw active D representatives
        # in the actual event embedding, rather than an ID-only equivalence.
        differences=[]
        for l,vector in enumerate(c['vector']):
            ref=joint['circuits'][l]['candidates'][vector['candidate_ids'][0]]['coefficients']
            p=list(map(lambda v:K(*v),ds[l]));q=list(map(lambda v:K(*v),ref))
            delta=[p[0]-q[0]+(p[2]-q[2])*u,p[1]-q[1]+(p[3]-q[3])*u]
            assert all(v==0 for v in delta)
            differences.append([v.data() for v in delta])
        mapping.append(dict(fiber=fiber,cell=cell,vector=vi,u=u.data(),coefficient_differences=differences))
        for triple in matrices:verify(lookup[(vi,triple)],u,lo,hi,['original',fiber,cell])
        if cell==0:print(json.dumps(dict(original_fiber=fiber,checks=len(checks),failures=len(failures))),flush=True)
    for entry in events['event_domains']:
        di,fi=entry['domain'],entry['formula'];domain=form['open_domains'][di]
        si,ti,ci=domain['incidence'];s=joint['joint']['topology_strips'][si];cell=s['substrips'][ti]['cells'][ci]
        for data in entry['internal_cuts']:
            u=engine.decode_u(data)
            lo=K(0) if cell['left_boundary'] is None else engine.curve_value({key:decode(s['curves'][cell['left_boundary'][0]][key]) for key in ['numerator','denominator']},u)
            hi=None if cell['right_boundary'] is None else engine.curve_value({key:decode(s['curves'][cell['right_boundary'][0]][key]) for key in ['numerator','denominator']},u)
            verify(fi,u,lo,hi,['internal',di,fi,data])
    out=dict(status='PASS closed fibers' if not failures else 'Exact failed closed predicates',
        formula_sha256=hashlib.sha256((ROOT/'formulas.json').read_bytes()).hexdigest(),
        feasibility_events_sha256=hashlib.sha256((ROOT/'feasibility-events.json').read_bytes()).hexdigest(),
        exact_mapping=mapping,checks=checks,empty_fibers=empty,failures=failures,equality=equality,
        elapsed_seconds=round(time.perf_counter()-start,3))
    (ROOT/'closed-proof.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['status','empty_fibers','elapsed_seconds']}));print('checks',len(checks),'equality',len(equality))

if __name__=='__main__':main()
