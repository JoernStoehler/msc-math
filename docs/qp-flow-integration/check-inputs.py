#!/usr/bin/env python3
"""Check the exact interrupted inputs before applying the integration patch."""
import hashlib
import json
from pathlib import Path
import sys

packet = Path(__file__).resolve().parent
manifest = json.loads((packet / 'manifest.json').read_text())
root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(manifest['source_root'])
failed = []
for relative, expected in manifest['inputs'].items():
    path = root / relative
    if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
        failed.append(str(path))
for relative, expected in manifest['proposed'].items():
    path = packet / 'proposed' / relative
    if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
        failed.append(str(path))
if failed:
    raise SystemExit('Changed or missing inputs; reconcile before applying:\n' + '\n'.join(failed))
print('All prerequisite and proposed file hashes match.')
