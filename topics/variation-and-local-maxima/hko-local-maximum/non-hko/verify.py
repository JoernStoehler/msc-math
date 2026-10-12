#!/usr/bin/env python3
"""Recompute the complete exact certificate, without discovery or networking.

Run: python verify.py --output-dir fresh-evidence
Outputs are written only to a new or empty directory. Retained evidence is not
read for acceptance. The sole fixed numerical input is the rational witness.
"""
from __future__ import annotations
import argparse,contextlib,datetime,hashlib,json,platform,sys,time
from pathlib import Path

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT/'code'))
import sympy
from certificate_exact import verify_base,field_family_check
from product_exact import verify_product
from independent_exact import verify_independent

class Tee:
    def __init__(self,*streams):self.streams=streams
    def write(self,text):
        for stream in self.streams:stream.write(text)
        return len(text)
    def flush(self):
        for stream in self.streams:stream.flush()

def dump(path:Path,obj:object)->None:
    path.write_text(json.dumps(obj,indent=2)+'\n',encoding='utf-8')

def sha(path:Path)->str:return hashlib.sha256(path.read_bytes()).hexdigest()

def main()->None:
    if not __debug__:
        raise RuntimeError('Assertions must be enabled; do not use python -O or PYTHONOPTIMIZE.')
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--output-dir',type=Path,default=ROOT/'fresh-evidence')
    args=p.parse_args();out=args.output_dir.resolve()
    if out.exists() and any(out.iterdir()):
        raise FileExistsError(f'Refusing to overwrite a nonempty evidence directory: {out}')
    out.mkdir(parents=True,exist_ok=True)
    inputs=[ROOT/'verify.py',ROOT/'certificate/certificate_base.json',*sorted((ROOT/'code').glob('*.py'))]
    provenance={'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'python':sys.version,'sympy':sympy.__version__,'platform':platform.platform(),
        'command':sys.argv,'acceptance_arithmetic':['QQ','QQ(v,s)','QQ(z)'],
        'source_sha256':{str(x.relative_to(ROOT)):sha(x) for x in inputs},
        'network_used_by_verifier':False,'retained_reports_used_for_acceptance':False}
    start=time.perf_counter()
    with (out/'verify.log').open('w',encoding='utf-8') as log:
      with contextlib.redirect_stdout(Tee(sys.stdout,log)),contextlib.redirect_stderr(Tee(sys.stderr,log)):
        print('Exact non-HKO F=10 local-maximum certificate',flush=True)
        c=json.loads((ROOT/'certificate/certificate_base.json').read_text())
        product=verify_product(symbolic=True);dump(out/'product_exact.json',product)
        print('Complete product capacity PASS:',product['ordered_candidates'],'ordered cases; max Q =',product['max_Q'],flush=True)
        base=verify_base(c);dump(out/'certificate_base_exact.json',base)
        ff=field_family_check(c,base['flat_spec'])
        assert ff['flat_base_vector']==base['flat_vector']
        dump(out/'certificate_family_exact.json',ff)
        independent=verify_independent(c,base);dump(out/'independent_exact.json',independent)
        assert independent['all_Q_of_z_constant_half']
        assert independent['weighted_second_derivative']=='-13/432'
        assert product['capacity']=='1' and base['volume']=='1/2'
        summary={'status':'PASS','theorem_argument':'PROOF.md',
            'base_capacity':'1','base_volume':'1/2','base_ratio':'1',
            'genuine_facets':10,'vertices':24,'simple':True,
            'sections':24,'positive_gradient_rank':22,'symmetry_rank':15,
            'equality_parameters_mod_symmetry':2,'remaining_transverse_directions':1,
            'full_chart_rank':40,'min_beta':base['min_beta'],'min_lambda':base['min_weight'],
            'remaining_second_derivative':'-13/432',
            'capacity_candidate_count':product['ordered_candidates'],
            'family_capacity_identities':True,'rational_function_kernel_identity':True,
            'independent_direct_derivative_check':True,'quantitative_neighborhood_radius':None,
            'proof_assistant_formalization':False,'elapsed_seconds':time.perf_counter()-start}
        dump(out/'SUMMARY.json',summary)
        print('ALL EXACT CHECKS PASS',flush=True)
        print(json.dumps(summary,indent=2),flush=True)
    provenance['elapsed_seconds']=time.perf_counter()-start
    provenance['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
    dump(out/'RUN.json',provenance)

if __name__=='__main__':main()
