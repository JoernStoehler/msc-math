#!/usr/bin/env python3
"""Recompute the complete exact HKO ascent certificate in a fresh directory."""
from pathlib import Path
import argparse,json,os,shlex,shutil,subprocess,sys,time

ROOT=Path(__file__).resolve().parent
def normalized(value):
    if isinstance(value,dict):return {k:normalized(v) for k,v in value.items() if k not in ('seconds','elapsed_seconds')}
    if isinstance(value,list):return [normalized(v) for v in value]
    return value

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True,help='new output directory; must not exist')
    p.add_argument('--cxx',default=os.environ.get('CXX','c++'),help='C++17 compiler executable (default CXX or c++)')
    args=p.parse_args();out=args.out.resolve()
    if out.exists():p.error('output directory already exists; refusing to replace it')
    compiler=shutil.which(args.cxx)
    if compiler is None:p.error('C++17 compiler not found: '+args.cxx)
    out.mkdir(parents=True)
    inputs=['prepare.py','catalogue.cpp','optimize_exact.py','exact_core.py','check_radial_profile_witness.py','check_effective_constants.py']
    for name in inputs:shutil.copyfile(ROOT/name,out/name)
    commands=[[sys.executable,str(out/'prepare.py')],
      [compiler,'-O2','-std=c++17',str(out/'catalogue.cpp'),'-o',str(out/'catalogue')],
      [str(out/'catalogue'),str(out/'primitives.txt'),str(out/'catalogue.json')],
      [sys.executable,str(out/'optimize_exact.py')],
      [sys.executable,str(out/'check_radial_profile_witness.py'),'--core',str(out/'exact_core.py'),'--out',str(out/'radial-profile-witness-PASS.json')],
      [sys.executable,str(out/'check_effective_constants.py'),'--core',str(out/'exact_core.py'),'--out',str(out/'constants-PASS.json')]]
    start=time.time();records=[]
    for command in commands:
        print('+ '+shlex.join(command),flush=True)
        tic=time.time();r=subprocess.run(command,cwd=out,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
        print(r.stdout,end='',flush=True)
        records.append({'command':command,'returncode':r.returncode,'elapsed_seconds':time.time()-tic,'output':r.stdout})
        if r.returncode:
            (out/'replay-log.json').write_text(json.dumps(records,indent=2)+'\n')
            raise SystemExit(r.returncode)
    matched=[]
    for name in ['primitives.json','catalogue.json','exact-results.json','radial-profile-witness-PASS.json','constants-PASS.json']:
        expected=normalized(json.loads((ROOT/'expected'/name).read_text()))
        actual=normalized(json.loads((out/name).read_text()))
        if actual!=expected:raise AssertionError('recomputed certificate mismatch: '+name)
        matched.append(name)
    result=json.loads((out/'exact-results.json').read_text())
    report={'all_checks_passed':True,'matched_exact_outputs':matched,
      'comparison_excludes_only_runtime_fields_seconds_and_elapsed_seconds':True,
      'effective_gain_scope':'All ten unit cuts in the original twenty-row family only; not the seventeen-facet subfamily.',
      'explicit_parameter':'1/10^30','relative_gain_lower_bound':'1/(200*10^30)',
      'complete_charts':result['chart_count'],'max_response_coefficients_ascending':result['maximum'],
      'positive_log_derivative_coefficients_ascending':result['log_systolic_derivative'],
      'python':sys.version,'compiler_executable':compiler,'elapsed_seconds':time.time()-start,
      'write_effects':'Only this fresh output directory, including copied sources, compiled executable, Python bytecode and generated JSON/log files.'}
    (out/'replay-log.json').write_text(json.dumps(records,indent=2)+'\n')
    (out/'PASS.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2),flush=True)

if __name__=='__main__':main()
