#!/usr/bin/env python3
"""Read-only size/hash inventory; stdout only, no file mutations.

Canonical math-section projection removes observational timing, absolute input
locations, and snapshot/source hash links. Those links remain in the original
certificates and the separate whole-file SHA-256 inventory.
"""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
GROUPS = {
    'essential certificate': [
        'formulas.json', 'clipped.json', 'feasibility-events.json',
        'open-proof.json', 'closed-proof.json', 'equality.json',
        'physical-control.json',
    ],
    'validation/coverage summary': ['closure-census.json', 'sturm-tests.json'],
    'workload cache, not needed by final reviewers': [
        'factor-workload.json', 'factor-relevant.json',
    ],
    'checksummed inherited input': [
        '../joint-denominators/joint-denominators.json',
        '../preflight/active-events.json',
    ],
    'source dependency': [
        '../joint-denominators/extract.py',
        '../joint-denominators/review_joint.py',
        '../preflight/active_events.py',
        '../../hko-angle-nonconformal/review_candidates.py',
    ],
}
OBSERVED = {
    'input', 'source', 'source_catalogue', 'field_engine',
    'elapsed_seconds', 'seconds', 'screen_seconds', 'total_seconds',
}

def math_projection(value):
    if isinstance(value, dict):
        return {
            key: math_projection(item) for key, item in value.items()
            if key not in OBSERVED and not key.endswith('_sha256')
        }
    if isinstance(value, list):
        return [math_projection(item) for item in value]
    return value


def main():
    rows = []
    for group, names in GROUPS.items():
        for name in names:
            path = (ROOT / name).resolve()
            data = path.read_bytes()
            row = dict(path=name, role=group, bytes=len(data),
                       snapshot_sha256=hashlib.sha256(data).hexdigest())
            if path.suffix == '.json':
                payload = math_projection(json.loads(data))
                canonical = json.dumps(payload, sort_keys=True,
                                       separators=(',', ':'), ensure_ascii=False).encode()
                row['math_section_sha256'] = hashlib.sha256(canonical).hexdigest()
            rows.append(row)
    for path in sorted(ROOT.glob('*.py')):
        if path.name == 'factor_workload.py':
            role = 'workload helper, optional'
        elif path.name == 'retention.py':
            role = 'read-only inventory helper'
        else:
            role = 'producer/reviewer source'
        data = path.read_bytes()
        rows.append(dict(path=path.name, role=role, bytes=len(data),
                         snapshot_sha256=hashlib.sha256(data).hexdigest()))
    print(json.dumps(dict(status='read-only retained snapshot inventory',
                          writes=False, rows=rows), indent=2))

if __name__ == '__main__':
    main()
