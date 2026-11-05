#!/usr/bin/env python3
"""Fetch hash-locked official releases and preserve task-specific annotation semantics."""
import argparse
import ast
import csv
import hashlib
import json
from pathlib import Path
import tarfile
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parent

def digest(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def dump(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + '\n')

def lines(path):
    return [json.loads(x) for x in path.read_text().splitlines() if x.strip()]

def write_records(name, records):
    out = ROOT / 'normalized' / (name + '.jsonl')
    with out.open('w') as f:
        for record in records:
            f.write(json.dumps(record, ensure_ascii=False, allow_nan=False) + '\n')
    return {'records': len(records), 'sha256': digest(out), 'bytes': out.stat().st_size}

def run(offline=False):
    lock = json.loads((ROOT / 'sources.lock.json').read_text())
    cache = ROOT / 'cache'
    cache.mkdir(exist_ok=True)
    for item in lock['files']:
        dest = cache / item['file']
        if not dest.exists():
            if offline:
                raise RuntimeError(f'Missing cache file: {dest}')
            temp = dest.with_suffix(dest.suffix + '.part')
            try:
                with urllib.request.urlopen(item['url'], timeout=120) as response, temp.open('wb') as f:
                    while block := response.read(1024 * 1024):
                        f.write(block)
                if digest(temp) != item['sha256']:
                    raise RuntimeError(f'Upstream content changed: {item["url"]}')
                temp.replace(dest)
            finally:
                temp.unlink(missing_ok=True)
        if digest(dest) != item['sha256']:
            raise RuntimeError(f'Hash mismatch: {dest}')
    (ROOT / 'normalized').mkdir(exist_ok=True)
    report = {}
    # Do not pool raters or infer majority labels. Offsets remain source-native.
    with (cache / 'scarecrow.csv').open(newline='') as f:
        rows = list(csv.DictReader(f))
    records = []
    annotations = 0
    raters = 0
    for row in rows:
        responses = ast.literal_eval(row['responses'])
        for rater in responses:
            for mark in rater:
                assert len(mark) == 6 and 0 <= mark[3] <= mark[4] <= len(row['generation'])
                annotations += 1
        raters += len(responses)
        records.append({'id': 'scarecrow:' + row['gid'], 'group_id': 'scarecrow-prompt:' + hashlib.sha256(row['prompt'].encode()).hexdigest(), 'text': row['generation'], 'context': row['prompt'], 'responses': responses, 'raw': row})
    assert len({r['id'] for r in records}) == len(records)
    report['scarecrow'] = write_records('scarecrow', records)
    report['scarecrow'].update(rater_responses=raters, error_marks=annotations)
    # Preserve source chunk boundaries, not invented sentence offsets.
    summaries = json.loads((cache / 'snac.download').read_text())
    records = []
    for summary_id, chunks in summaries.items():
        context = []
        for chunk_id in sorted(chunks, key=int):
            chunk = chunks[chunk_id]
            records.append({'id': f'snac:{summary_id}:{chunk_id}', 'group_id': f'snac:{summary_id}', 'text': chunk['text'], 'context_chunks': context.copy(), 'raw': chunk})
            context.append(chunk['text'])
    report['snac'] = write_records('snac', records)
    report['snac'].update(summaries=len(summaries), error_marks=sum(len(r['raw']['errors']) for r in records), votes=sum(e['votes'] for r in records for e in r['raw']['errors']))
    with zipfile.ZipFile(cache / 'storyfeedback.zip') as z:
        # Python's input parser accepts upstream NaN. Normalize it explicitly to null;
        # untouched bytes remain in the archive and no missing score becomes zero.
        records = []
        with z.open('data/human_ratings.jsonl') as f:
            for line in f:
                raw = json.loads(line, parse_constant=lambda _: None)
                records.append({'id': 'storyfeedback:' + raw['example_id'], 'group_id': 'storyfeedback:' + str(raw['story_id']), 'text': raw['story'], 'raw': raw})
        report['storyfeedback'] = write_records('storyfeedback-human-ratings', records)
        report['storyfeedback']['generated_feedback_records'] = sum(1 for line in z.open('data/generated_feedback.jsonl') if line.strip())
    # Only manual test alignment task here; synthetic training labels are not fetched.
    comments = {(r['doc_id'], r['comment_id']): r for r in lines(cache / 'aries-review_comments.jsonl')}
    edits = {r['doc_id']: r for r in lines(cache / 'aries-paper_edits.jsonl')}
    labels = lines(cache / 'aries-edit_labels_test.jsonl')
    needed = {edits[r['doc_id']][k] for r in labels for k in ['source_pdf_id', 'target_pdf_id']}
    papers = {}
    with tarfile.open(cache / 'aries-s2orc.tar.gz', 'r:gz') as tar:
        for member in tar:
            key = Path(member.name).stem
            if member.isfile() and key in needed:
                with tar.extractfile(member) as f:
                    papers[key] = json.load(f)
    assert needed == papers.keys(), 'Missing ARIES full-text records'
    # Store full records once rather than duplicate paper context per comment.
    report['aries_papers'] = write_records('aries-papers', [{'id': k, 'raw': papers[k]} for k in sorted(papers)])
    records = []
    for label in labels:
        doc = edits[label['doc_id']]
        aligned = {x['edit_id']: x for x in doc['edits']}
        resolved = []
        for edit_id in label['positive_edits']:
            edit = aligned[edit_id]
            texts = {}
            for side in ['source', 'target']:
                pdf = papers[doc[side + '_pdf_id']]['pdf_parse']
                # Required by upstream aries.util.s2orc.load_s2orc semantics.
                paragraphs = pdf['body_text'] + pdf['back_matter']
                texts[side + '_paragraphs'] = [paragraphs[i] for i in edit[side + '_idxs']]
            resolved.append({'raw_edit': edit, **texts})
        records.append({'id': f"aries:{label['doc_id']}:{label['comment_id']}", 'group_id': 'aries:' + label['doc_id'], 'raw_label': label, 'raw_comment': comments[(label['doc_id'], label['comment_id'])], 'source_pdf_id': doc['source_pdf_id'], 'target_pdf_id': doc['target_pdf_id'], 'positive_edits': resolved, 'all_edit_candidates': doc['edits']})
    report['aries_test'] = write_records('aries-test', records)
    report['aries_test']['documents'] = len({r['group_id'] for r in records})
    report['aries_generated'] = write_records('aries-generated', [{'id': f"aries-generated:{r['doc_id']}:{r['comment_id']}", 'group_id': 'aries:' + r['doc_id'], 'raw': r} for r in lines(cache / 'aries-generated_edits.jsonl')])
    dump(ROOT / 'normalized' / 'manifest.json', report)
    print(json.dumps(report, indent=2))

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--offline', action='store_true', help='Verify cached release hashes and rebuild without network')
    run(parser.parse_args().offline)
