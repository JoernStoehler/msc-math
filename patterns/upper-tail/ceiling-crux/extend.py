#!/usr/bin/env python3
"""One root-selected frozen 60-geometry extension; no capacity labels."""
import hashlib,importlib.util,json,time
from fractions import Fraction as F
from pathlib import Path
D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('crux',D/'probe.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
def main():
 start=time.perf_counter();path=m.R/'patterns/upper-tail/contact-stability/sample.json'
 source=json.loads(path.read_text())['rows'];frozen=[x[2:] for x in (D/'PLAN-amendment.md').read_text().splitlines() if x.startswith('- contact_')]
 assert [r['name'] for r in source]==frozen and len(source)==60
 geometry=[dict(name=r['name'],A=[[F(x) for x in u] for u in r['A']],B=[[F(x) for x in u] for u in r['B']]) for r in source]
 (D/'extension-geometry.json').write_text(json.dumps(m.p.serial(geometry),indent=2)+'\n')
 rows=[];stop='all_60_frozen_extension_rows_survive'
 for g in geometry:
  z=m.select(g['A'],g['B']);width=1/max(abs(m.p.dot(u,v)) for u in m.cut.steps(g['A']) for v in m.cut.steps(g['B']))
  ceiling=min(width,z['capacity']);ratio=ceiling*ceiling/(2*z['volume']);failed=m.p.E(ratio)>m.H
  rows.append(dict(name=g['name'],selection=z,width=width,width_ratio=width*width/(2*z['volume']),ceiling=ceiling,ratio=ratio,surrogate_counterexample=failed))
  print(g['name'],'min',float(ratio),'star',float(z['ratio']),'FAIL' if failed else 'PASS',flush=True)
  out=dict(rows=rows,status='counterexample' if failed else 'running',frozen_names=frozen,source_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),plan_sha256=hashlib.sha256((D/'PLAN-amendment.md').read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),elapsed_seconds=time.perf_counter()-start,cpu_seconds=time.process_time(),limits='Geometry-only falsification; no eta, full paired capacity oracle, historical targets, fitting or global theorem claim')
  (D/'extension-results.json').write_text(json.dumps(m.p.serial(out),indent=2)+'\n')
  if failed:stop='first_strict_surrogate_counterexample';break
 out.update(status=stop,elapsed_seconds=time.perf_counter()-start,cpu_seconds=time.process_time())
 (D/'extension-results.json').write_text(json.dumps(m.p.serial(out),indent=2)+'\n');print(stop,'wall',out['elapsed_seconds'],'cpu',out['cpu_seconds'])
if __name__=='__main__':main()
