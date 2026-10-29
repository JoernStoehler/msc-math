#!/usr/bin/env python3
"""Sanitized native Codex token projection for the dated HKO research run.

Uses agent-fleet-tools' installed subtree-usage ownership/dedup logic. Emits
response counters and thread metadata only, never messages or tool content.
"""
from __future__ import annotations
import argparse
import hashlib
import importlib.util
import json
from collections import defaultdict
from datetime import datetime
from pathlib import Path

HELPER = Path('/workspaces/agent-fleet-tools/skills/codex-resources/scripts/subtree-usage.py')
ROOT = '01a184a6-b383-7673-8fdd-fff5c4b49ad2'
START = '2026-10-28T22:35:36.893+00:00'
END = '2026-10-29T01:25:00+00:00'
MULTICUT_START = '2026-10-28T22:38:19.104+00:00'


def parse_time(s: str) -> datetime:
    return datetime.fromisoformat(s.replace('Z', '+00:00'))


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--codex-home', type=Path, default=Path.home() / '.codex')
    ap.add_argument('--output', type=Path, required=True)
    args = ap.parse_args()
    spec = importlib.util.spec_from_file_location('subtree_usage', HELPER)
    if spec is None or spec.loader is None:
        raise RuntimeError(f'cannot load accounting helper: {HELPER}')
    usage = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(usage)
    start, end = parse_time(START), parse_time(END)
    sessions = usage.inventory(args.codex_home)
    selected = usage.descendants(sessions, ROOT)
    warnings: set[str] = set()
    threads = []
    raw_events = {}
    for ident in sorted(selected):
        path, meta = sessions[ident]
        source = meta.get('source')
        spawn = (source.get('subagent') or {}).get('thread_spawn') if isinstance(source, dict) else {}
        agent_path = (spawn or {}).get('agent_path') or ('root' if ident == ROOT else 'unknown')
        parent = (spawn or {}).get('parent_thread_id') or meta.get('parent_thread_id')
        events = [(t, model, counters) for t, model, counters in usage.usage_events(path, meta, end, warnings)
                  if start <= t < end]
        raw_events[ident] = (agent_path, events)
        if not events:
            continue
        by_model = defaultdict(lambda: {
            'responses': 0, 'input_tokens': 0, 'cached_input_tokens': 0,
            'output_tokens': 0, 'cache_write_input_tokens': 0,
            'api_shadow_usd': 0.0,
        })
        unpriced = set()
        for _, model, counters in events:
            row = by_model[model]
            row['responses'] += 1
            for field, value in zip(('input_tokens', 'cached_input_tokens', 'output_tokens', 'cache_write_input_tokens'), counters):
                if value is None:
                    row[field] = None
                elif row[field] is not None:
                    row[field] += value
            if model in usage.RATES:
                row['api_shadow_usd'] += usage.price(model, counters)
            else:
                unpriced.add(model)
        for model in unpriced:
            by_model[model]['api_shadow_usd'] = None
        threads.append({
            'thread_id': ident, 'parent_thread_id': parent, 'agent_path': agent_path,
            'first_usage_at': events[0][0].isoformat(), 'last_usage_at': events[-1][0].isoformat(),
            'response_records': len(events), 'models': dict(by_model),
        })

    episodes = [
        ('geometry_fixed_body_development', '2026-10-29T00:14:26.973+00:00', '2026-10-29T00:51:10.249+00:00', ('/root/geometry_frontiers',)),
        ('fixed_body_mathematical_audit', '2026-10-29T00:29:00+00:00', '2026-10-29T00:56:00+00:00', ('/root/hko_ascent_math_audit',)),
        ('fixed_body_fresh_exact_replay', '2026-10-29T00:38:53.079+00:00', '2026-10-29T00:48:50.385+00:00', ('/root/affine_hko_exact_replay',)),
        ('multicut_discovery_to_approx_0004', MULTICUT_START, '2026-10-29T00:04:10.888+00:00', ('/root/multicut_variation',)),
        ('multicut_extensions_after_approx_0004', '2026-10-29T00:04:10.888+00:00', '2026-10-29T01:09:02.224+00:00', ('/root/multicut_variation',)),
        ('whole_paper_writing_integration', '2026-10-28T23:57:58+00:00', '2026-10-29T01:18:23+00:00', ('/root/paper_integration',)),
        ('whole_paper_assembled_mathematical_review', '2026-10-29T00:10:53+00:00', '2026-10-29T01:14:36+00:00', ('/root/assembled_math_review',)),
        ('whole_paper_reader_review', '2026-10-29T00:11:04+00:00', '2026-10-29T01:14:44+00:00', ('/root/assembled_reader_review',)),
        ('whole_paper_final_archive_build_delivery_replay', '2026-10-29T01:15:05+00:00', '2026-10-29T01:19:14+00:00', ('/root/affine_hko_exact_replay',)),
    ]
    episode_rows = []
    for name, since_s, until_s, prefixes in episodes:
        since, until = parse_time(since_s), parse_time(until_s)
        aggregate = defaultdict(lambda: {
            'responses': 0, 'input_tokens': 0, 'cached_input_tokens': 0,
            'output_tokens': 0, 'cache_write_input_tokens': 0,
            'api_shadow_usd': 0.0,
        })
        owners = set()
        for ident, (agent_path, events) in raw_events.items():
            if not any(agent_path == prefix or agent_path.startswith(prefix + '/') for prefix in prefixes):
                continue
            for when, model, counters in events:
                if not (since <= when < until):
                    continue
                owners.add(ident)
                row = aggregate[model]
                row['responses'] += 1
                for field, value in zip(('input_tokens', 'cached_input_tokens', 'output_tokens', 'cache_write_input_tokens'), counters):
                    if value is None:
                        row[field] = None
                    elif row[field] is not None:
                        row[field] += value
                if model in usage.RATES:
                    row['api_shadow_usd'] += usage.price(model, counters)
                else:
                    row['api_shadow_usd'] = None
        episode_rows.append({
            'name': name, 'since_inclusive': since.isoformat(), 'until_exclusive': until.isoformat(),
            'agent_path_prefixes': list(prefixes), 'thread_ids_with_usage': sorted(owners),
            'models': dict(aggregate),
            'interpretation': 'overlapping worker-window accounting; not marginal causal cost or root coordination allocation',
        })

    result = {
        'schema': 'msc-math-hko-postmortem-usage-projection-v1',
        'source': 'locally selected native Codex rollouts; installed subtree-usage.py ownership/dedup logic',
        'accounting_helper_path': str(HELPER),
        'accounting_helper_sha256': hashlib.sha256(HELPER.read_bytes()).hexdigest(),
        'historical_shadow_price_table_date': '2026-09-16',
        'shadow_rates_usd_per_million_used': {
            model: list(usage.RATES[model])
            for model in sorted({model for _, events in raw_events.values() for _, model, _ in events})
            if model in usage.RATES
        },
        'shadow_price_formula': 'uncached=(input-cached-cache_write); long=input>272000; dollars=(uncached*in_rate*(2 if long else 1)+cached*cache_rate*(2 if long else 1)+cache_write*in_rate*1.25*(2 if long else 1)+output*out_rate*(1.5 if long else 1))/1000000',
        'root_thread_id': ROOT, 'window_start_inclusive': start.isoformat(), 'window_end_exclusive': end.isoformat(),
        'thread_count_with_usage': len(threads), 'accounting_warnings': sorted(warnings),
        'threads': threads, 'episodes': episode_rows,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(f"wrote {args.output}: {len(threads)} threads, {len(warnings)} accounting warnings")


if __name__ == '__main__':
    main()
