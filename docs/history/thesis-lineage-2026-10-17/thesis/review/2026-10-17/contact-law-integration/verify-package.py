#!/usr/bin/env python3
"""Verify transported identities, receipt bindings and printed finite claims; no capacity producer."""
import argparse, datetime, hashlib, importlib.util, json, subprocess, sys
from fractions import Fraction
from pathlib import Path

p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);args=p.parse_args()
root=Path(__file__).resolve().parent
manifest=json.loads((root/'source-manifest.json').read_text())
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
for row in manifest['files']:
    target=root/row['retained_path']
    assert target.stat().st_size==row['bytes'] and digest(target)==row['sha256'],row['retained_path']
c=root/'witnesses/patterns/upper-tail/contact-stability'
# Source receipts retain original bytes. Check the local input identities they declare.
receipt_bindings=0
for name in ['fullword.json','verification.json','canonical-verification.json','canonical-metrics.json','canonical-predictions.json','global-counterexample.json','global-verification.json']:
    data=json.loads((c/name).read_text())
    refs=data.get('input_sha256',{})
    if isinstance(refs,dict):
        for rel,h in refs.items():
            assert (c/rel).is_file(),(name,rel)
            assert digest(c/rel)==h,(name,rel)
            receipt_bindings+=1
    for key,rel in [('prediction_sha256','predictions.json'),('source_sha256','fullword.py')]:
        if key in data:assert digest(c/rel)==data[key];receipt_bindings+=1
truth=json.loads((c/'canonical-truth.json').read_text())
assert truth['sample_sha256']==digest(c/'canonical-sample.json')
assert truth['contact_predictions_sha256']==digest(c/'canonical-predictions.json')
assert truth['oracle_sha256']==digest(c/'oracle-canonical.py')
truths={row['name']:row for row in truth['rows']}
metrics=json.loads((c/'canonical-metrics.json').read_text())
predictions={row['name']:row for row in json.loads((c/'canonical-predictions.json').read_text())['rows']}
assert len(truths)==len(predictions)==len(metrics['rows'])==54
for row in metrics['rows']:
    assert row['truth']==float(Fraction(truths[row['name']]['ratio']))
    assert all(row[key]==value for key,value in predictions[row['name']].items())
# Verify the exact scalar separation inside the retained obstruction certificate;
# this is not a reconstruction of its geometry or complete capacity enumeration.
spec=importlib.util.spec_from_file_location('retained_q5',root/'witnesses/patterns/upper-tail/conditional-coupling/quadratic.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);Q=module.Q5
cert=json.loads((c/'global-counterexample.json').read_text())
if hasattr(sys,'set_int_max_str_digits'):sys.set_int_max_str_digits(0)
assert Q.parse(cert['exact_invariant'])>Q(4)/5
assert Q.parse(cert['exact_ratio'])>Q(4)/5
assert Q.parse(cert['exact_reciprocal_prediction'])<Q(3)/5
args.out.parent.mkdir(parents=True,exist_ok=True)
metrics_out=args.out.with_name(args.out.stem+'-metrics.json')
subprocess.run([sys.executable,'-B',str(root/'check-metrics.py'),'--patterns',str(root/'witnesses'),'--out',str(metrics_out)],check=True)
args.out.write_text(json.dumps({'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'PASS','manifest_sha256':digest(root/'source-manifest.json'),'original_source_commit':manifest['source_commit'],'files':len(manifest['files']),'bytes':sum(r['bytes'] for r in manifest['files']),'receipt_bindings':receipt_bindings,'canonical_truth_prediction_rows':54,'exact_obstruction_scalar_separation':'PASS','metrics_receipt':str(metrics_out),'scope':'Transported file identities, local receipt bindings, exact scalar separations, exact-cusp bank evaluation and displayed finite metrics; no full capacity, canonical-gauge or derivative replay.'},indent=2)+'\n')
print('PASS: transported witnesses and displayed claims')
