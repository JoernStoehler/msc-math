#!/usr/bin/env python3
"""Validate manifest, skill header, local source integrity, and curated references."""
import hashlib
from pathlib import Path
import re
from refresh import ROOT, load_manifest
import argparse


def check(root=ROOT):
    errors=[]
    s=(root/'SKILL.md').read_text(encoding='utf-8')
    if not s.startswith('---\n') or '\nname: agent-platform-reference\n' not in s or '\ndescription:' not in s:
        errors.append('invalid skill frontmatter')
    for path in sorted((root/'references').glob('*.md')):
        if len(path.read_text(encoding='utf-8'))<100: errors.append(f'empty reference {path}')
    for m in re.findall(r'\]\((references/[^)#]+\.md)\)',s):
        if not (root/m).exists(): errors.append(f'broken SKILL.md link {m}')
    try: rows=load_manifest(root/'corpus/manifest.jsonl')
    except Exception as e: return [f'manifest: {e}']
    for r in rows:
        path=root/r['path']
        if r['status']=='ok':
            if not path.exists(): errors.append(f'OK but missing: {r["id"]}')
            else:
                content=path.read_bytes()
                if hashlib.sha256(content).hexdigest()!=r['sha256']: errors.append(f'hash mismatch: {r["id"]}')
                if len(content)<100: errors.append(f'empty: {r["id"]}')
            if r.get('raw_path'):
                raw=root/r['raw_path']
                if not raw.is_file(): errors.append(f'missing raw snapshot: {r["id"]}')
                elif hashlib.sha256(raw.read_bytes()).hexdigest()!=r.get('raw_sha256'): errors.append(f'raw hash mismatch: {r["id"]}')
            else: errors.append(f'missing raw provenance: {r["id"]}')
        elif r['status'] not in {'pending','error'}: errors.append(f'bad status: {r["id"]}')
    return errors

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--require-mirrors',action='store_true',help='Fail unless every source is locally mirrored and validated')
    args=parser.parse_args()
    problems=check()
    if args.require_mirrors:
        rows=load_manifest()
        pending=[r['id'] for r in rows if r['status']!='ok']
        if pending: problems.append(f'{len(pending)} missing/unverified mirrors: '+', '.join(pending[:10]))
    if problems:
        print('\n'.join(problems)); raise SystemExit(1)
    else:
        from refresh import load_manifest
        r=load_manifest()
        print(f'PASS structural checks ONLY: {len(r)} sources; {sum(x["status"]=="ok" for x in r)} mirrored; {len(list((ROOT/"references").glob("*.md")))} references')
