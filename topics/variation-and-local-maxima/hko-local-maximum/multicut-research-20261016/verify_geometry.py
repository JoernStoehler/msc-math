#!/usr/bin/env python3
"""Exact chart/normal identities using the accepted F11 arithmetic helper.
Standard library only; reuses, rather than independently validates, that helper.
"""
import argparse, hashlib, importlib.util, itertools, json, tempfile, time
from pathlib import Path
from zipfile import ZipFile


def main():
    p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();start=time.perf_counter()
    archive=args.repo/'topics/variation-and-local-maxima/hko-local-maximum/f11-return-20261004/hko_f11_complete_proof.zip'
    with tempfile.TemporaryDirectory(prefix='hko-multicut-exact-') as temporary:
        source=Path(temporary)/'exact_core.py'
        with ZipFile(archive) as z:source.write_bytes(z.read('inherited/code/exact_core.py'))
        spec=importlib.util.spec_from_file_location('accepted_f11_exact_core',source);core=importlib.util.module_from_spec(spec);spec.loader.exec_module(core)
        def require(ok,msg):
            if not ok:raise ValueError(msg)
        A,V=core.A,core.V;zero=core.ZERO;one=core.ONE
        incidence=[{i for i,a in enumerate(A) if core.dot(v,a)==one} for v in V]
        require(all(len(ids)==4 for ids in incidence),'simple HKO vertices')
        def mean(ids):return [sum((A[i][k] for i in ids),zero)/len(ids) for k in range(4)]
        edge_records=[]
        for v,w in itertools.combinations(range(25),2):
            ids=incidence[v]&incidence[w]
            if len(ids)==3:edge_records.append((v,w,mean(ids)))
        require(len(edge_records)==50,'50 HKO primal edges')
        gaps=[]
        for vertex,ids in enumerate(incidence):
            n=mean(ids);active=[i for i,v in enumerate(V) if core.dot(n,v)==one]
            require(active==[vertex],'vertex mean exposes exactly vertex')
            gaps.extend(one-core.dot(n,v) for i,v in enumerate(V) if i!=vertex)
            attached=[e for v,w,e in edge_records if vertex in (v,w)]
            require(len(attached)==4,'four incident edges')
            require([sum((e[k] for e in attached),zero)/4 for k in range(4)]==n,'vertex-cut normal is mean of incident edge-cut normals')
        for v,w,n in edge_records:
            require([i for i,x in enumerate(V) if core.dot(n,x)==one]==[v,w],'edge mean exposes exactly edge')
            gaps.extend(one-core.dot(n,x) for i,x in enumerate(V) if i not in (v,w))
        require(all(core.sign(g)>0 for g in gaps),'strict support gaps')
        minimum=gaps[0]
        for g in gaps[1:]:
            if core.sign(g-minimum)<0:minimum=g
        result={'all_checks_passed':True,'arithmetic':'Exact Q[tau], tau^4-10 tau^2+5=0; ordered-root sign checks from accepted F11 helper','vertices':25,'edges':50,'vertex_exposure_checks':25,'edge_exposure_checks':50,'exact_redundancy_relations':25,'strict_gap_comparisons':len(gaps),'minimum_support_gap_coefficients_ascending':minimum.coefficients(),'minimum_support_gap_nominal':core.nominal(minimum),'accepted_archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'helper_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'seconds':time.perf_counter()-start,'scope':'Exact finite geometry checks only; the continuum multicut theorem is proved separately.'}
    args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))

if __name__=='__main__':main()
