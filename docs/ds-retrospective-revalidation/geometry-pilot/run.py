#!/usr/bin/env python3
"""Run frozen geometry-only pilot in sbx; retain binary/source hashes and elapsed time."""
import hashlib, json, pathlib, subprocess, time, datetime
root=pathlib.Path(__file__).resolve().parents[3]
out=pathlib.Path(__file__).resolve().parent
base=root/'docs/ds-retrospective-revalidation'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
assert not (out/'run-receipt.json').exists(), 'refuse overwrite'
files=list((root/'experiments/polytope-invariant-table').glob('*.rs'))+[root/'experiments/polytope-invariant-table/Cargo.toml', root/'Cargo.lock',base/'pilot-inputs.jsonl']+list((base/'pilot-source').glob('*.jsonl'))
meta={'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'git_head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'hashes':{str(p.relative_to(root)):sha(p) for p in files},'capacity_recomputed':False,'volume_recomputed':False,'rayon_threads':4}
# This environment uses the workspace-local target directory.
cmd=f'cd {root} && sha256sum target/release/sys-dataset && RAYON_NUM_THREADS=4 timeout 240 target/release/sys-dataset --random-only --random {base}/pilot-source/random.jsonl --random-product {base}/pilot-source/random-product.jsonl --out-dir {out}/output'
meta['command']=cmd
start=time.monotonic()
r=subprocess.run(['ssh','codex-msc-math.sbx',cmd],capture_output=True,text=True)
meta.update(elapsed_seconds=time.monotonic()-start,exit_code=r.returncode)
(out/'run.stdout').write_text(r.stdout); (out/'run.stderr').write_text(r.stderr)
meta['output_hashes']={str(p.relative_to(out)):sha(p) for p in (out/'output').glob('*')}
(out/'run-receipt.json').write_text(json.dumps(meta,indent=2)+'\n')
print(json.dumps({k:meta[k] for k in ['elapsed_seconds','exit_code']}))
