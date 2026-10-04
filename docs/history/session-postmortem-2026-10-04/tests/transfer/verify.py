#!/usr/bin/env python3
"""Check retained study bytes and aggregation without new calls or writes."""
from pathlib import Path
import hashlib
import json
import statistics

ROOT = Path(__file__).resolve().parent

def read(name):
    return json.loads((ROOT / name).read_text())

def digest(name):
    return hashlib.sha256((ROOT / name).read_bytes()).hexdigest()

manifest = read('manifest.json')
for name, expected in manifest['files'].items():
    assert digest(name) == expected, ('frozen input', name)
for record in read('publication-manifest.json')['records']:
    assert digest(record['published_name']) == record['published_sha256'], ('published bytes', record['published_name'])
participants = read('participant-results.json')
for row in participants:
    assert digest(row['response_file']) == row['response_sha256'], ('answer', row['answer_id'])
locked = read('locked-scoring-results.json')
for row in locked['source_hashes']:
    assert digest(row['file']) == row['sha256'], ('scoring input', row['file'])
assignments = {row['answer_id']: row for row in manifest['assignments']}
analysis = read('analysis.json')
for case in analysis['paired_results']:
    for condition in ('baseline', 'knowledge'):
        row = case[condition]
        aid = row['answer_id']
        assert assignments[aid]['condition'] == condition
        scores = {who: results[aid]['scores']['total'] for who, results in locked['results'].items()}
        assert scores == row['scores']
        assert statistics.mean(scores.values()) == row['mean']
    assert case['knowledge']['mean'] - case['baseline']['mean'] == case['mean_difference']
for condition in ('baseline', 'knowledge'):
    assert statistics.mean(case[condition]['mean'] for case in analysis['paired_results']) == analysis['overall_mean'][condition]
    selected = [r for r in participants if assignments[r['answer_id']]['condition'] == condition]
    for key in selected[0]['token_usage']:
        assert sum(r['token_usage'][key] for r in selected) == analysis['participant_costs'][condition]['tokens'][key]
assert all(row['complete_expected_text_in_model_facing_output'] for row in read('model-facing-read-checks.json'))
print('PASS: frozen inputs, published bytes, answers, score lock, paired aggregation and participant usage.')
