#!/usr/bin/env python3
"""Execute on sbx: frozen full geometry migration; no capacity/volume evaluations."""
import datetime,hashlib,json,os,pathlib,subprocess,time
root=pathlib.Path(__file__).resolve().parents[3]; here=pathlib.Path(__file__).resolve().parent
base=here.parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert not (here/'run-receipt.json').exists() and not (here/'output').exists(), 'refuse overwrite'
pilot=json.loads((base/'geometry-pilot/run-receipt.json').read_text())
for name,h in pilot['hashes'].items():
 if name.startswith('experiments/') or name=='Cargo.lock': assert sha(root/name)==h, name
binary=root/'target/release/sys-dataset'; bh=sha(binary)
assert bh==(base/'geometry-pilot/run.stdout').read_text().split()[0]
source=pathlib.Path.home()/'.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files'
expected=json.loads((base/'source-verification.json').read_text())['files']
inputs={str(source/n):sha(source/n) for n in ['random.jsonl','random-product.jsonl']}
for p,h in inputs.items(): assert h==expected[pathlib.Path(p).name]
cmd=[str(binary),'--random-only','--random',str(source/'random.jsonl'),'--random-product',str(source/'random-product.jsonl'),'--out-dir',str(here/'output')]
receipt={'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'git_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'binary_sha256':bh,'input_hashes':inputs,'pilot_source_identity_verified':True,'command':cmd,'execution_cap_seconds':1500,'rayon_threads':4,'capacity_recomputed':False,'volume_recomputed':False,'status':'running','resume':'Producer writes outputs after entire build; timeout requires full restart into a fresh directory.'}
(here/'run-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
start=time.monotonic()
with (here/'run.stdout').open('w') as out,(here/'run.stderr').open('w') as err:
 try:
  r=subprocess.run(cmd,cwd=root,env=dict(os.environ,RAYON_NUM_THREADS='4'),stdout=out,stderr=err,timeout=1500)
  receipt.update(exit_code=r.returncode,status='completed' if r.returncode==0 else 'failed')
 except subprocess.TimeoutExpired: receipt.update(exit_code=None,status='timed_out')
receipt.update(elapsed_seconds=time.monotonic()-start,finished_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),output_hashes={str(p.relative_to(here)):sha(p) for p in (here/'output').glob('*')})
(here/'run-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(receipt['status'],receipt['elapsed_seconds'])
