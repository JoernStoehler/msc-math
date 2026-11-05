#!/usr/bin/env python3
"""Check retained old/new panel joins and arithmetic; run no scientific producer."""
import argparse
import hashlib
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--patterns', type=Path, required=True)
    ap.add_argument('--out', type=Path, required=True)
    args = ap.parse_args()
    repo = args.patterns.resolve()
    base = repo / 'patterns/upper-tail'
    inputs = {}

    def read(relative):
        path = base / relative
        inputs[relative] = sha(path)
        return json.loads(path.read_text())

    old_manifest = read('full-partner-validation/manifest.json')
    new_manifest = read('conditional-validation/manifest.json')
    for dirname, manifest in [('full-partner-validation', old_manifest),
                               ('conditional-validation', new_manifest)]:
        expected = manifest.get('sha256') or {k: v['sha256'] for k, v in manifest['files'].items()}
        for filename, digest in expected.items():
            assert sha(base / dirname / filename) == digest, (dirname, filename)
    oldsample = read('full-partner-validation/sample.json')
    newsample = read('conditional-validation/sample.json')
    old = read('full-partner-validation/results.json')
    new = read('conditional-validation/results.json')
    comp = read('full-partner-validation/complementarity.json')
    prior_verification = read('full-partner-validation/geometry-verification.json')
    new_verification = read('conditional-validation/verification.json')
    owner_verification = read('conditional-validation/owner-verification.json')
    assert new_verification['status'] == owner_verification['status'] == 'PASS'
    for dirname, sample, result in [('full-partner-validation', oldsample, old),
                                      ('conditional-validation', newsample, new)]:
        assert result['sample_sha256'] == inputs[dirname + '/sample.json']
        assert result['plan_sha256'] == sha(base / dirname / 'PLAN.md')
        for rel, digest in sample['sources'].items():
            assert sha(repo / rel) == digest, rel
    assert oldsample['source_sha256'] == newsample['source_sha256'] == old['source_sha256']
    om = {r['name']: r for r in oldsample['sample']}
    nm = {r['name']: r for r in newsample['sample']}
    orows = {r['name']: r for r in old['rows']}
    nrows = {r['name']: r for r in new['rows']}
    assert len(om) == len(old['rows']) == len(orows) == 36
    assert len(nm) == len(new['rows']) == len(nrows) == 35
    assert set(om) == set(orows) and set(nm) == set(nrows)
    assert set(om) - set(nm) == {'random_5x6_s1_198'}
    assert not set(nm) - set(om)
    assert set(nm) == set(om) - set(newsample['conditional_design_excluded'])
    assert oldsample['excluded20'] == newsample['excluded20']
    assert not set(om) & set(oldsample['excluded20'])
    assert all(om[name] == nm[name] for name in nm)
    columns = ('name', 'bucket', 'role', 'family_rank')
    records = []
    for name, nr in nrows.items():
        oo = orows[name]
        assert all(nr[k] == oo[k] == nm[name][k] for k in columns)
        assert nr['A'] == [r[:2] for r in oo['geometry'][:oo['k']]]
        assert nr['B'] == [r[2:] for r in oo['geometry'][oo['k']:]]
        assert nr['historical_capacity'] == oo['historical_capacity']
        assert nr['historical_ratio'] == oo['historical_ratio']
        oldbest = min(F(oo['capacity']), F(oo['incumbent_capacity']))
        c, lower = F(nr['capacity']), F(nr['capacity_lower'])
        assert oldbest == F(nr['incumbent_capacity'])
        assert F(nr['combined_capacity']) == min(c, oldbest) == c
        assert c <= oldbest and c >= lower
        assert nr['exact_capacity'] == (c == lower)
        assert F(nr['rounding']['capacity']) == c
        assert F(nr['rounding']['best']['capacity']) == c
        assert F(1, 2) / F(nr['rounding']['best']['Q']) == c
        assert lower == F(1, 2) / max(F(v['Q']) for v in nr['variants'])
        records.append({'name': name, 'role': nr['role'],
                        'old_combined_capacity': str(oldbest),
                        'new_upper': str(c), 'new_lower': str(lower),
                        'exact_capacity': c == lower,
                        'exact_reduction': str(1-c/oldbest),
                        'reduction_percent': float(100*(1-c/oldbest)),
                        'old_gap_to_numerical_label': float(oldbest)/nr['historical_capacity']-1,
                        'new_gap_to_numerical_label': float(c)/nr['historical_capacity']-1})
    tails = [r for r in orows.values() if r['role'] == 'principal_top2']
    old_gaps = {r['name']: float(min(F(r['capacity']), F(r['incumbent_capacity'])))/r['historical_capacity']-1 for r in tails}
    worst = max(old_gaps, key=old_gaps.get)
    assert worst == 'random_5x5_s3_227'
    assert len(tails) == comp['tail_n'] == 27
    assert sum(g <= .01 for g in old_gaps.values()) == comp['within_1pct'] == 26
    assert sum(abs(g) < 1e-10 for g in old_gaps.values()) == comp['numerical_agreement'] == 25
    assert max(old_gaps.values()) == comp['max_gap']
    assert {r['name']: r['gap'] for r in comp['rows']} == old_gaps
    assert all(F(r['exact_minimum_capacity']) == min(F(orows[r['name']]['capacity']), F(orows[r['name']]['incumbent_capacity'])) for r in comp['rows'])
    exact = [r for r in records if r['exact_capacity']]
    ntails = [r for r in records if r['role'] == 'principal_top2']
    bulk = [r for r in records if r['role'] != 'principal_top2']
    assert len(ntails) == 26 and len(bulk) == 9 and len(exact) == 26
    assert sum(r['exact_capacity'] for r in ntails) == 17
    assert all(abs(r['new_gap_to_numerical_label']) < 1e-10 for r in records)
    strict = [r['name'] for r in records if F(r['exact_reduction']) > 0]
    material = [r['name'] for r in records if F(r['exact_reduction']) >= F(1,100)]
    assert strict == ['random_5x5_s3_227', 'random_6x6_s2_63']
    assert material == [worst]
    target = next(r for r in records if r['name'] == worst)
    assert target['exact_capacity']
    output = {'status': 'PASS', 'scope': 'Source-hash bindings, exact membership/geometry/label joins, retained-bound arithmetic; no optimizer, original corpus reconstruction, or fresh certificate proof replay.',
              'checker_sha256': sha(Path(__file__)), 'input_sha256': inputs,
              'source_geometry_sha256_declared': oldsample['source_sha256'],
              'old_panel': {'total': 36, 'tail':27, 'bulk':9},
              'new_panel': {'total':35, 'tail':26, 'bulk':9},
              'intersection': sorted(nm), 'old_only': sorted(set(om)-set(nm)), 'new_only':[],
              'unshared_conditional_design_controls_already_absent': sorted(set(newsample['conditional_design_excluded'])-set(om)),
              'new_tail_family_counts': dict(Counter(nrows[name]['bucket'] for name in nm if nm[name]['role']=='principal_top2')),
              'old_combined': {'within_1pct':26, 'agreement':25, 'worst':worst, 'max_gap':old_gaps[worst]},
              'new_result': {'numerical_agreements':35, 'exact_capacities':26, 'exact_tail_capacities':17, 'exact_bulk_capacities':9, 'uncertified_tails':9, 'strict_improvements':strict, 'material_improvements':material},
              'material_row': target, 'rows':records,
              'limitations': ['Capacity-certificate validity retains the scope of the hash-bound independent scientific verification packets.', 'All inherited numerical labels were previously exposed; this is retrospective validation.', 'Old exact width obstruction is excluded as a conditional-LP design case; no new held-out result is asserted for it.']}
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(output, indent=2)+'\n')
    print(json.dumps({'status':output['status'], 'intersection':len(nm), 'removed':output['old_only'], 'material_row':worst, 'exact_new_capacity':target['new_upper'], 'exact_cut_percent':target['reduction_percent']}))


if __name__ == '__main__':
    main()
