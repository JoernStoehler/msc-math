"""Check the frozen covariance-balancing experiment and summarize all paired outcomes."""
import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path

import numpy as np

from analyze import rank, spearman, write_tsv
from balance import SOURCE

HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def read(path):
    opener = gzip.open if path.suffix == '.gz' else open
    with opener(path, 'rt') as stream:
        return [json.loads(line) for line in stream]


def summarize(values):
    values = np.asarray(values)
    return dict(n=len(values), mean=float(np.mean(values)), median=float(np.median(values)),
        minimum=float(np.min(values)), maximum=float(np.max(values)),
        positive=int(np.sum(values > 0)), negative=int(np.sum(values < 0)), zero=int(np.sum(values == 0)))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--packet', type=Path, default=HERE / 'balance-artifacts')
    parser.add_argument('--source', type=Path, default=SOURCE)
    args = parser.parse_args()
    packet = args.packet
    freeze = json.loads((packet / 'freeze.json').read_text())
    assert sha(HERE / 'balance.py') == freeze['script_sha256']
    for file, key in [('inputs.jsonl', 'input_sha256'), ('pilot-inputs.jsonl', 'pilot_input_sha256'),
                      ('remaining-inputs.jsonl', 'remaining_input_sha256'), ('geometry.jsonl', 'geometry_sha256')]:
        assert sha(packet / file) == freeze[key]
    inputs = read(packet / 'inputs.jsonl')
    geometry = {row['id']: row for row in read(packet / 'geometry.jsonl')}
    assert len(inputs) == len(geometry) == 640
    all_results = []
    for prefix, expected_hash in [('pilot', freeze['pilot_input_sha256']), ('remaining', freeze['remaining_input_sha256'])]:
        result_path = packet / f'{prefix}-results.jsonl'
        if not result_path.exists():
            result_path = result_path.with_suffix('.jsonl.gz')
        results = read(result_path)
        expected = read(packet / f'{prefix}-inputs.jsonl')
        assert len(results) == len(expected)
        for index, (actual, wanted) in enumerate(zip(results, expected), 1):
            assert actual['line_number'] == index and actual['input'] == wanted and actual['id'] == wanted['id']
            assert actual['input_line_sha256'] == hashlib.sha256(actual['input_raw'].encode()).hexdigest()
            assert actual['evaluator']['input_file_sha256'] == expected_hash
        all_results.extend(results)
    assert len({row['id'] for row in all_results}) == 640
    operational = dict(requests=len(all_results), statuses=dict(Counter(row['status'] for row in all_results)),
        summed_request_wall_seconds=sum(row['wall_ms'] for row in all_results) / 1000,
        largest_request_wall_seconds=max(row['wall_ms'] for row in all_results) / 1000,
        analysis_sha256=sha(Path(__file__)))
    (packet / 'operational-summary.json').write_text(json.dumps(operational, indent=2) + '\n')
    if any(row['status'] != 'ok' for row in all_results):
        raise SystemExit('Errors retained: no complete-panel scientific summary produced.')
    receipts = {json.dumps({key: row['evaluator'][key] for key in ['binary_sha256', 'build_source_sha256', 'supervisor_sha256']}, sort_keys=True) for row in all_results}
    assert len(receipts) == 1
    result_by_id = {row['id']: row for row in all_results}
    assert sha(args.source) == freeze['source_sha256']
    baseline_by_name = {row['input']['source_name']: row for row in all_results if row['input']['arm'] == 'baseline'}
    lineage_pairs = []
    for source in read(args.source):
        if source['name'] in baseline_by_name:
            current = baseline_by_name[source['name']]
            assert source['dual_vertices'] == current['input']['dual_vertices']
            lineage_pairs.append(dict(source_name=source['name'], historical_sys=source['sys'], current_sys=current['sys'],
                absolute_sys_difference=abs(current['sys'] - source['sys'])))
    assert len(lineage_pairs) == 320
    lineage = dict(n=320, maximum_absolute_sys_difference=max(row['absolute_sys_difference'] for row in lineage_pairs), pairs=lineage_pairs)
    (packet / 'baseline-lineage-check.json').write_text(json.dumps(lineage, indent=2) + '\n')
    groups = defaultdict(list)
    armrows = defaultdict(list)
    maximum_volume_error = 0
    for item in inputs:
        result = result_by_id[item['id']]
        g = geometry[item['id']]
        assert result['capacity_route'] == 'product' and result['capacity_method'] == 'production-capacity-scalar-v1'
        assert result['primal_vertex_count'] == int(g['group'][0]) * int(g['group'][2])
        assert result['capacity_lower'] <= result['capacity'] <= result['capacity_upper']
        assert abs(result['sys'] - result['capacity'] ** 2 / (2 * result['volume'])) < 1e-13
        error = abs(result['volume'] / g['volume'] - 1)
        maximum_volume_error = max(maximum_volume_error, error)
        assert error < 1e-9
        row = {**g, 'sys': result['sys'], 'capacity': result['capacity']}
        groups[(g['group'], g['arm'])].append(row)
        armrows[g['arm']].append(row)
    group_stats = []
    arm_stats = {}
    for arm in ['baseline', 'balanced']:
        pooled = []
        for (group, actual_arm), rows in sorted(groups.items()):
            if actual_arm != arm:
                continue
            assert len(rows) == 32
            R = [row['R'] for row in rows]
            sys = [row['sys'] for row in rows]
            pooled.extend(zip(rank(R) / len(R), rank(sys) / len(sys)))
            group_stats.append(dict(group=group, arm=arm, n=len(rows), R_sys_spearman=spearman(R, sys),
                mean_R=float(np.mean(R)), mean_sys=float(np.mean(sys)), max_sys=max(sys),
                min_R=min(R), max_R=max(R)))
        pooled = np.asarray(pooled)
        rows = armrows[arm]
        arm_stats[arm] = dict(n=len(rows), pooled_within_group_spearman=spearman(pooled[:, 0], pooled[:, 1]),
            R=summarize([row['R'] for row in rows]), sys=summarize([row['sys'] for row in rows]),
            covariance_rho=summarize([row['rho'] for row in rows]))
    pairs = []
    for baseline in armrows['baseline']:
        name = baseline['source_name']
        balanced = geometry[name + ':balanced']
        result = result_by_id[name + ':balanced']
        pairs.append(dict(source_name=name, group=baseline['group'], baseline_R=baseline['R'], balanced_R=balanced['R'],
            delta_R=balanced['R'] - baseline['R'], baseline_sys=baseline['sys'], balanced_sys=result['sys'],
            delta_sys=result['sys'] - baseline['sys'], baseline_covariance_rho=baseline['rho'], balanced_covariance_rho=balanced['rho']))
    paired_stats = dict(delta_R=summarize([row['delta_R'] for row in pairs]), delta_sys=summarize([row['delta_sys'] for row in pairs]),
        opposite_signs=sum(row['delta_R'] * row['delta_sys'] < 0 for row in pairs),
        both_positive=sum(row['delta_R'] > 0 and row['delta_sys'] > 0 for row in pairs),
        both_negative=sum(row['delta_R'] < 0 and row['delta_sys'] < 0 for row in pairs))
    summary = dict(**operational, arms=arm_stats, paired=paired_stats, checks=dict(
        maximum_relative_volume_error_vs_exact_factor_areas=maximum_volume_error,
        max_balanced_covariance_rho_error=max(abs(row['rho'] - 1) for row in armrows['balanced']),
        max_relative_factor_area_change=max(row['max_relative_area_change'] for row in armrows['balanced']),
        one_evaluator_identity=True, all_product_route=True, complete_paired_grid=True))
    write_tsv(packet / 'paired-effects.tsv', pairs)
    write_tsv(packet / 'group-summary.tsv', group_stats)
    (packet / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps(summary, indent=2))


if __name__ == '__main__':
    main()
