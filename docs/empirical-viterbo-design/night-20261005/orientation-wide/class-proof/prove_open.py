#!/usr/bin/env python3
"""Exact open-component gap proof. Writes only sibling open-proof.json.

Uses odd-multiplicity Sturm counts, exact probes/endpoints and signed bound
denominators. A failed certificate remains explicit and never means PASS.
"""
from sturm import *

def main():
    started=time.perf_counter();clip=json.loads((ROOT/'clipped.json').read_text());form=json.loads((ROOT/'formulas.json').read_text())
    polynomials=clip['polynomials'];queries=[];cache={};results=[];failed=[]
    def query(p,left,right,sample,target):
        key=(coeff_key(p),left,right,target)
        if key not in cache:
            cache[key]=len(queries);cert=sign_certificate(p,left,right,sample,target)
            queries.append(dict(polynomial=encode(p),left=left.data(),right=right.data(),certificate=cert))
        return cache[key],queries[cache[key]]['certificate']['passed']
    def referenced(ref):
        if ref.get('zero'):return [K(0)]
        return scale(decode(polynomials[int(ref['key'])]['coefficients']),K(*ref['multiplier']))
    for i,t in enumerate(clip['open_segments']):
        left,right=engine.decode_u(t['left']),engine.decode_u(t['right']);sample=K(*t['sample_u'])
        endpoint_checks=[];okay=True
        for check in t['checks']:
            label=check['bound']['label']
            if isinstance(label,list) and label[0]=='residual' and label[1]>=5:
                endpoint_checks.append(dict(label=check['label'],mode='zero-edge strict primary bound <=1'));continue
            qi,passed=query(referenced(check['gap']),left,right,sample,1)
            endpoint_checks.append(dict(label=check['label'],query=qi));okay=okay and passed
        mode=None;minquery=None
        if t['singleton']:mode='singleton endpoint'
        else:
            qi,passed=query(referenced(t['q2']),left,right,sample,-1)
            if passed:mode='concave/linear';minquery=qi
            else:
                # If convex, a signed endpoint derivative excludes an interior
                # minimum. If concave, endpoint bounds suffice regardless.
                for check in t['checks']:
                    denominator=decode(check['bound']['denominator']);dsign=evaluate(denominator,sample).sign();assert dsign!=0
                    target=dsign if check['label']=='lower' else -dsign
                    qi,passed=query(referenced(check['station_membership']),left,right,sample,target)
                    if passed:mode='endpoint derivative '+check['label'];minquery=qi;break
                if mode is None:
                    qi,passed=query(referenced(t['stationary_gap']),left,right,sample,1)
                    if passed:mode='stationary numerator nonnegative';minquery=qi
        passed=okay and mode is not None
        results.append(dict(segment=i,passed=passed,endpoint_checks=endpoint_checks,
            minimum_mode=mode,minimum_query=minquery))
        if not passed:failed.append(i)
        if i%200==0:print(json.dumps(dict(segment=i,queries=len(queries),failed=len(failed))),flush=True)
    out=dict(status='PASS open component gap certificates' if not failed else 'Exact unresolved open predicates',
        clipped_sha256=hashlib.sha256((ROOT/'clipped.json').read_bytes()).hexdigest(),
        sturm_tests=tests(),queries=queries,segments=results,failed_segments=failed,
        minimum_mode_counts={mode:sum(r['minimum_mode']==mode for r in results) for mode in {r['minimum_mode'] for r in results}},
        elapsed_seconds=round(time.perf_counter()-started,3))
    (ROOT/'open-proof.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['status','failed_segments','minimum_mode_counts','elapsed_seconds']}));print('queries',len(queries))

if __name__=='__main__':main()
