#!/usr/bin/env python3
"""Check retained branch/dirty versions and the frozen PDF's exact build inputs."""
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = ROOT / 'docs/resume'

def sha(path):
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()

count = 0
for branch in json.loads((HERE / 'branch-inventory.json').read_text()):
    for item in branch['files']:
        if 'retained_path' not in item:
            continue
        blob = item['blob']
        assert subprocess.run(['git', 'cat-file', '-e', blob], cwd=ROOT).returncode == 0, (branch['branch'], item['path'], blob)
        retained = item['retained_path']
        if retained.startswith('docs/history/'):
            data = (ROOT / retained).read_bytes()
            actual = hashlib.sha1(b'blob ' + str(len(data)).encode() + b'\0' + data).hexdigest()
            assert actual == blob, (branch['branch'], item['path'])
        count += 1
for item in json.loads((HERE / 'dirty-source-inventory.json').read_text()):
    if 'retained_path' in item:
        assert sha(item['retained_path']) == item['sha256'], item['path']
build = json.loads((HERE / 'build-verification.json').read_text())
assert sha('docs/resume/thesis-resume.pdf') == build['pdf_sha256']
for item in build['source_inputs']:
    assert sha(item['path']) == item['sha256'], item['path']
assert build['build_exit'] == 0 and not build['unexpected_external_inputs']
print(f'PASS: {count} branch records, dirty snapshots, PDF and {len(build["source_inputs"])} selected build inputs')
