#!/usr/bin/env python3
"""Inspect, validate and export the local writing-evidence corpus (stdlib + Poppler)."""
import argparse
import hashlib
import json
import re
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent
LOCAL = ROOT / 'local'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(path):
    return [json.loads(s) for s in path.read_text().splitlines() if s.strip()]


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')


def extraction(pdf):
    result = subprocess.run(['pdftotext', '-layout', '-enc', 'UTF-8', str(pdf), '-'], check=True, capture_output=True)
    text = result.stdout.decode('utf-8')
    pages = text.split('\f')
    if pages[-1] == '':
        pages.pop()
    info = subprocess.run(['pdfinfo', str(pdf)], check=True, capture_output=True, text=True).stdout
    count = int(re.search(r'^Pages:\s+(\d+)', info, re.M).group(1))
    if len(pages) != count:
        raise ValueError(f'{pdf}: extraction pages {len(pages)} != PDF pages {count}')
    return pages


def extract():
    for case in rows(LOCAL / 'cases.jsonl'):
        if case['input_kind'] != 'pdf':
            continue
        pages = extraction(LOCAL / case['pdf'])
        output = []
        offset = 0
        for number, page in enumerate(pages, 1):
            output.append({'case_id': case['case_id'], 'pdf_page': number, 'start_char': offset,
                           'end_char': offset + len(page), 'text': page})
            offset += len(page) + 1  # form feed separating pages in .txt
        (LOCAL / case['text']).write_text('\f'.join(pages), encoding='utf-8')
        (LOCAL / case['pages']).write_text(''.join(json.dumps(r, ensure_ascii=False) + '\n' for r in output))


def seal():
    files = []
    for path in sorted(LOCAL.rglob('*')):
        if path.is_file() and path.name != 'manifest.json':
            files.append({'path': str(path.relative_to(LOCAL)), 'sha256': sha(path), 'bytes': path.stat().st_size})
    version = subprocess.run(['pdftotext', '-v'], capture_output=True, text=True).stderr.splitlines()[0]
    write_json(LOCAL / 'manifest.json', {'schema_version': 1, 'files': files, 'extraction': {
        'tool': version, 'argv': ['pdftotext', '-layout', '-enc', 'UTF-8', 'INPUT.pdf', '-'],
        'offset_unit': 'Unicode code points, zero-based half-open; page numbers physical PDF 1-based',
        'caveat': 'PDF is authoritative for mathematics and layout; extraction loses glyph positioning and formulas.'}})


def validate(reextract=False):
    manifest = json.loads((LOCAL / 'manifest.json').read_text())
    for item in manifest['files']:
        path = LOCAL / item['path']
        assert path.is_file(), path
        assert sha(path) == item['sha256'], f'hash mismatch: {path}'
        assert path.stat().st_size == item['bytes'], path
    listed = {x['path'] for x in manifest['files']}
    actual = {str(p.relative_to(LOCAL)) for p in LOCAL.rglob('*') if p.is_file() and p.name != 'manifest.json'}
    assert listed == actual, ('unmanifested/missing files', actual ^ listed)
    for original in json.loads((LOCAL / 'source-file-map.json').read_text()):
        assert sha(LOCAL / original['stored_path']) == original['sha256']
    cases = rows(LOCAL / 'cases.jsonl')
    by_id = {c['case_id']: c for c in cases}
    assert len(by_id) == len(cases)
    for case in cases:
        assert case['lineage_clusters'] and case['provenance']['unknowns']
        for ref in case['evidence'] + case['provenance_files']:
            assert (LOCAL / ref).is_file(), ref
        text = (LOCAL / case['text']).read_text()
        if case['input_kind'] == 'pdf':
            assert sha(LOCAL / case['pdf']) == case['source_sha256']
            pages = rows(LOCAL / case['pages'])
            assert len(pages) == case['physical_pages']
            assert '\f'.join(p['text'] for p in pages) == text
            for n, page in enumerate(pages, 1):
                assert page['case_id'] == case['case_id'] and page['pdf_page'] == n
                assert text[page['start_char']:page['end_char']] == page['text']
            if reextract:
                assert extraction(LOCAL / case['pdf']) == [p['text'] for p in pages], case['case_id']
    labels = rows(LOCAL / 'labels.jsonl')
    assert len({x['label_id'] for x in labels}) == len(labels)
    for label in labels:
        assert label['case_id'] in by_id
        assert label['reviewer_kind'] == 'human'
        assert label['status'] in ['active', 'withdrawn']
        source = LOCAL / 'evidence' / label['source_path']
        assert label['evidence_quote'] in source.read_text(), ('quote mismatch', label['label_id'])
        assert label['scope'] and label['dimension'] and label['attribution']
    for anchor in rows(LOCAL / 'anchors.jsonl'):
        assert anchor['label_id'] in {l['label_id'] for l in labels}
        case = by_id[anchor['case_id']]
        text = (LOCAL / case['text']).read_text()
        for location in anchor['anchors']:
            assert text[location['start_char']:location['end_char']] == location['matched_text']
    for review in rows(LOCAL / 'reviews.jsonl'):
        assert review['case_id'] in by_id and (LOCAL / review['path']).is_file()
    print(f'OK: {len(cases)} cases, {len(labels)} bounded human observations, {len(manifest["files"])} hashed files')


def export(args):
    cases = rows(LOCAL / 'cases.jsonl')
    wanted = set(args.cases.split(',')) if args.cases else {c['case_id'] for c in cases if c['input_kind'] == 'pdf'}
    unknown = wanted - {c['case_id'] for c in cases}
    if unknown:
        raise ValueError(f'unknown cases: {sorted(unknown)}')
    out = Path(args.output)
    out.mkdir(parents=True, exist_ok=False)
    try:
        index = []
        for c in cases:
            if c['case_id'] not in wanted:
                continue
            # Deliberate allowlist: no source paths, labels, provenance or AI predictions.
            item = {'case_id': c['case_id'], 'context': c['context_scope']}
            for key in ['pdf', 'text', 'pages']:
                if key in c:
                    src = LOCAL / c[key]
                    dst = out / (c['case_id'] + src.suffix)
                    shutil.copyfile(src, dst)
                    item[key] = dst.name
            index.append(item)
        write_json(out / 'index.json', index)
        (out / 'README.txt').write_text('Reading inputs only. No review labels or predictions are supplied.\n'
            'PDF is authoritative; extracted formulas may be damaged. Physical page numbers are 1-based.\n'
            'IDs hide label-bearing filenames, not the subject or all PDF metadata. This is not a claim of uncontaminated holdout.\n')
    except Exception:
        shutil.rmtree(out)
        raise
    print(f'Exported {len(index)} inputs to {out.resolve()}')


def inventory():
    labels = rows(LOCAL / 'labels.jsonl')
    for c in rows(LOCAL / 'cases.jsonl'):
        n = sum(l['case_id'] == c['case_id'] for l in labels)
        print(f'{c["case_id"]}: {c["input_kind"]}, {c.get("physical_pages", "?")} pages, {n} human observations; clusters={",".join(c["lineage_clusters"])}')


def partition(args):
    cases = rows(LOCAL / 'cases.jsonl')
    selected = set(args.holdout.split(','))
    assert selected and selected <= {c['case_id'] for c in cases}, 'unknown/empty holdout'
    clusters = {g for c in cases if c['case_id'] in selected for g in c['lineage_clusters']}
    development, excluded = [], []
    for c in cases:
        if c['case_id'] in selected:
            continue
        (excluded if clusters.intersection(c['lineage_clusters']) else development).append(c['case_id'])
    print(json.dumps({'candidate_holdout': sorted(selected), 'development': development,
                      'excluded_shared_lineage': excluded,
                      'warning': 'Administrative separation only. Historical root/research agents have seen labels. New writing and human judgments are required for genuinely prospective evidence.'}, indent=2))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    commands.add_parser('inventory')
    check = commands.add_parser('validate'); check.add_argument('--reextract', action='store_true')
    commands.add_parser('extract'); commands.add_parser('seal')
    view = commands.add_parser('export'); view.add_argument('--output', required=True); view.add_argument('--cases')
    part = commands.add_parser('partition'); part.add_argument('--holdout', required=True)
    args = parser.parse_args()
    if args.command == 'validate': validate(args.reextract)
    elif args.command == 'export': export(args)
    elif args.command == 'inventory': inventory()
    elif args.command == 'partition': partition(args)
    elif args.command == 'extract': extract()
    elif args.command == 'seal': seal()
