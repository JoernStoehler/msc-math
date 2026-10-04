#!/usr/bin/env python3
"""Read-only checks of retained bytes, frozen inputs and descriptive aggregation."""
from pathlib import Path
import gzip
import hashlib
import json
import math

ROOT = Path(__file__).resolve().parent

def read(name):
    return json.loads((ROOT / name).read_text())

def data(name):
    p = ROOT / name
    return p.read_bytes() if p.exists() else gzip.decompress(p.with_suffix(p.suffix + '.gz').read_bytes())

def digest(name):
    return hashlib.sha256(data(name)).hexdigest()

for name, record in read('PUBLICATION-MANIFEST.json').items():
    assert digest(name) == record['sha256'], ('published bytes', name)
    assert len(data(name)) == record['bytes'], ('published size', name)
frozen = read('freeze-manifest.json')
verified_frozen = 0
for name, record in frozen['frozen_files'].items():
    p = ROOT / name
    if p.exists() or p.with_suffix(p.suffix + '.gz').exists():
        assert digest(name) == record['sha256'], ('frozen input', name)
        assert len(data(name)) == record['bytes'], ('frozen size', name)
        verified_frozen += 1
participants = read('participant-provenance.json')
assignments = {r['reader']: r for r in frozen['assignments_launch_order']}
receipts = read('reader-receipts.json')
for row in participants:
    assert digest('answer-' + row['answer_id'] + '.md') == row['answer_sha256']
    assert digest('readers/' + row['reader'] + '.json') == row['input_sha256']
    assert row['condition'] == assignments[row['reader']]['condition']
    calls = receipts[row['reader']]
    assert len(calls) == row['reader_calls']
    assert calls[-1]['source_words_total'] == row['source_words']
    assert sum(c['source_words_this_call'] for c in calls) == row['source_words']
    assert all(c['source_words_total'] <= 25000 for c in calls)
locked = read('locked-scoring-results.json')
ratings = {}
for scorer, record in locked['scores'].items():
    assert digest(record['file']) == record['sha256']
    ratings[scorer] = {row['answer_id']: row for row in record['parsed']}
analysis = read('analysis.json')
for pair in analysis['pairs']:
    for scorer, results in pair['ratings'].items():
        for condition in ('archive', 'packet'):
            assert results[condition] == ratings[scorer][pair[condition + '_answer']]
    assert pair['source_words_packet'] - pair['source_words_archive'] == pair['source_words_delta_packet_minus_archive']
    assert math.isclose(pair['wall_seconds_packet'] - pair['wall_seconds_archive'], pair['wall_seconds_delta'], abs_tol=1e-9)
for condition, summary in analysis['bycondition'].items():
    rows = [r for r in participants if r['condition'] == condition]
    assert sum(r['source_words'] for r in rows) == summary['source_words']
    assert sum(r['reader_calls'] for r in rows) == summary['reader_calls']
    assert math.isclose(sum(r['seconds_to_final'] for r in rows), summary['summed_individual_seconds'], abs_tol=1e-9)
    for key, value in summary['usage'].items():
        actual = sum(r['tokens']['input_tokens'] - r['tokens']['cached_input_tokens'] if key == 'uncached_input_tokens' else r['tokens'][key] for r in rows)
        assert actual == value, ('usage', condition, key)
print(f'PASS: publication hashes, {verified_frozen} retained frozen inputs, eight answers, read accounting, locked scores and paired aggregation.')
print('Private raw-source contents and effective model input are not independently proved by these checks.')
