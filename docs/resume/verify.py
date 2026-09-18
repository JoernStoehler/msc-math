#!/usr/bin/env python3
"""Check preserved history at its checkpoint and current frozen-build evidence."""
import hashlib
import json
import re
import subprocess
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / 'docs/resume'


def current_sha(path):
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()


checkpoint_record = json.loads((HERE / 'preservation-checkpoint.json').read_text())
assert checkpoint_record['schema_version'] == 1
checkpoint = checkpoint_record['checkpoint_commit']
assert re.fullmatch(r'[0-9a-f]{40}', checkpoint), checkpoint
ancestry = subprocess.run(
    ['git', 'merge-base', '--is-ancestor', checkpoint, 'HEAD'],
    cwd=ROOT,
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
)
assert ancestry.returncode == 0, f'preservation checkpoint is not an ancestor of HEAD: {checkpoint}'

branch_expectations = defaultdict(list)
branch_count = 0
for branch in json.loads((HERE / 'branch-inventory.json').read_text()):
    for item in branch['files']:
        if 'retained_path' not in item:
            continue
        branch_expectations[item['retained_path']].append(
            (item['blob'], branch['branch'], item['path'])
        )
        branch_count += 1

dirty_expectations = defaultdict(list)
dirty_count = 0
for item in json.loads((HERE / 'dirty-source-inventory.json').read_text()):
    if 'retained_path' not in item:
        continue
    dirty_expectations[item['retained_path']].append((item['sha256'], item['path']))
    dirty_count += 1

# One long-lived batch process avoids thousands of Git invocations. Historical
# bytes are read only from retained_path entries in the immutable checkpoint.
paths = sorted(set(branch_expectations) | set(dirty_expectations))
process = subprocess.Popen(
    ['git', 'cat-file', '--batch'],
    cwd=ROOT,
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
)
assert process.stdin is not None and process.stdout is not None
try:
    for retained_path in paths:
        assert '\n' not in retained_path and '\r' not in retained_path, retained_path
        spec = f'{checkpoint}:{retained_path}'
        process.stdin.write(spec.encode() + b'\n')
        process.stdin.flush()
        header = process.stdout.readline().rstrip(b'\n')
        assert not header.endswith(b' missing'), f'missing checkpoint path: {retained_path}'
        fields = header.split()
        assert len(fields) == 3, (retained_path, header)
        object_id, object_type, size_text = fields
        assert object_type == b'blob', (retained_path, object_type.decode())
        data = process.stdout.read(int(size_text))
        assert len(data) == int(size_text), retained_path
        assert process.stdout.read(1) == b'\n', retained_path

        actual_blob = object_id.decode()
        for expected_blob, branch_name, original_path in branch_expectations[retained_path]:
            assert actual_blob == expected_blob, (branch_name, original_path, retained_path)
        if retained_path in dirty_expectations:
            actual_sha256 = hashlib.sha256(data).hexdigest()
            for expected_sha256, original_path in dirty_expectations[retained_path]:
                assert actual_sha256 == expected_sha256, (original_path, retained_path)
finally:
    process.stdin.close()
    returncode = process.wait()
    assert returncode == 0, returncode

# The frozen handoff PDF and its selected inputs describe the current checkout,
# so these checks intentionally remain separate from historical preservation.
build = json.loads((HERE / 'build-verification.json').read_text())
assert current_sha('docs/resume/thesis-resume.pdf') == build['pdf_sha256']
for item in build['source_inputs']:
    assert current_sha(item['path']) == item['sha256'], item['path']
assert build['build_exit'] == 0 and not build['unexpected_external_inputs']
print(
    f'PASS: {branch_count} branch records, {dirty_count} dirty snapshots at '
    f'checkpoint {checkpoint[:12]}, PDF and {len(build["source_inputs"])} selected build inputs'
)
