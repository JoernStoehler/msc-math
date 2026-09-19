#!/usr/bin/env python3
"""Serial JSONL supervisor: immutable output, retained input and per-body deadline."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import subprocess
import time
from build import source_identity


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def finite_float(text):
    value = float(text)
    if not math.isfinite(value):
        raise ValueError("nonfinite coordinate/metadata number")
    return value


def main():
    parser = argparse.ArgumentParser(__doc__)
    parser.add_argument('--input', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--build-receipt', type=Path, required=True)
    parser.add_argument('--timeout-seconds', type=float, default=30.)
    args = parser.parse_args()
    if not math.isfinite(args.timeout_seconds) or args.timeout_seconds <= 0:
        parser.error('timeout must be finite and positive')
    packet = Path(__file__).resolve().parent
    root = packet.parents[3]
    receipt = json.loads(args.build_receipt.read_text())
    if receipt['binary_sha256'] != digest(args.binary) or receipt['build_source_sha256'] != source_identity(packet):
        parser.error('binary/build source mismatch; rebuild with build.py')
    identity = {**receipt, 'supervisor_sha256':digest(Path(__file__)),
                'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
                'git_status':subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True),
                'input_file_sha256':digest(args.input), 'timeout_seconds':args.timeout_seconds}
    env = dict(os.environ, RAYON_NUM_THREADS='2', OMP_NUM_THREADS='2', OPENBLAS_NUM_THREADS='2')
    seen = set()
    with args.input.open() as source, args.output.open('x') as out:
        for line_number, raw in enumerate(source,1):
            start = time.monotonic()
            row = {'schema':'current-body-evaluator-v1','line_number':line_number,'input_raw':raw,
                   'input_line_sha256':hashlib.sha256(raw.encode()).hexdigest(), 'evaluator':identity,
                   'capacity_method':'production-capacity-scalar-v1','capacity_relative_tolerance':1e-10,
                   'volume_method':'f64-from-exact-binary64-derived-incidence-v1'}
            try:
                body = json.loads(raw, parse_float=finite_float, parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))
                row['input'] = body
                if not isinstance(body,dict) or not isinstance(body.get('id'),str) or not body['id']:
                    raise ValueError('nonempty string id required')
                row['id'] = body['id']
                if body['id'] in seen:
                    raise ValueError('duplicate id')
                seen.add(body['id'])
                child = subprocess.run([str(args.binary.resolve())],input=raw,text=True,capture_output=True,
                                       timeout=args.timeout_seconds,env=env)
                if child.returncode:
                    row.update(status='error',error_stage='worker_process',error_type='WorkerExit',
                               returncode=child.returncode,stderr=child.stderr)
                else:
                    row.update(json.loads(child.stdout))
            except subprocess.TimeoutExpired:
                row.update(status='error',error_stage='worker_process',error_type='Timeout')
            except (ValueError,TypeError) as exc:
                row.update(status='error',error_stage='input_or_worker_schema',error_type=type(exc).__name__,error=str(exc))
            except OSError as exc:
                row.update(status='error',error_stage='worker_process',error_type=type(exc).__name__,error=str(exc))
            row['wall_ms'] = (time.monotonic()-start)*1000
            out.write(json.dumps(row,allow_nan=False)+'\n')
            out.flush()


if __name__ == '__main__':
    main()
