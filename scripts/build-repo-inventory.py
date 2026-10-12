#!/usr/bin/env python3
"""Validate topic inventories and render a searchable, standalone review page.

The physical census is a separate snapshot. Source links establish locations,
not proof validity or complete semantic coverage.
"""
from __future__ import annotations
import argparse
from collections import Counter
import html
import json
from pathlib import Path
import subprocess
from urllib.parse import quote

ROOT = Path(__file__).resolve().parents[1]
INVENTORY_DIR = ROOT / 'meta-topics/repository-organization/inventory'
REQUIRED = ('id', 'kind', 'title', 'summary', 'state', 'agent_readability',
            'human_readability', 'runtime', 'code_quality', 'sources',
            'relationships', 'action', 'rationale', 'uncertainties')


def read_parts():
    items, surveys, errors = [], [], []
    seen = set()
    for path in sorted((INVENTORY_DIR / 'parts').glob('*.json')):
        data = json.loads(path.read_text())
        surveys.append({'name': path.stem, 'scope': data.get('scope'),
                        'coverage': data.get('coverage'),
                        'changes': data.get('changes', [])})
        for row in data.get('items', []):
            missing = [key for key in REQUIRED if key not in row]
            if missing:
                errors.append(f"{path.name}/{row.get('id')}: missing {missing}")
            if row.get('id') in seen:
                errors.append(f"duplicate ID {row.get('id')}")
            seen.add(row.get('id'))
            for field in ('sources', 'relationships'):
                for ref in row.get(field, []):
                    source = ref.get('path', '') if isinstance(ref, dict) else ref
                    if not source or source.startswith('/') or '..' in Path(source).parts:
                        errors.append(f"{row.get('id')}: invalid source path {source!r}")
                    elif not (ROOT / source.split('#', 1)[0]).exists():
                        errors.append(f"{row.get('id')}: missing source {source}")
            row = dict(row, survey=path.stem)
            items.append(row)
    return items, surveys, errors


def generate():
    items, surveys, errors = read_parts()
    if errors:
        raise SystemExit('\n'.join(errors))
    reconciliation = json.loads((INVENTORY_DIR / 'reconciliation.json').read_text())
    by_id = {item['id']: item for item in items}
    hidden = set()
    for match in reconciliation['aliases']:
        alias, canonical = match['alias'], match['canonical']
        if alias not in by_id or canonical not in by_id or alias == canonical:
            raise SystemExit(f'Invalid identity reconciliation: {match}')
        other, retained = by_id[alias], by_id[canonical]
        retained.setdefault('aliases', []).append(alias)
        retained.setdefault('additional_assessments', []).append({
            'survey': other['survey'], 'state': other['state'],
            'agent_readability': other['agent_readability'],
            'human_readability': other['human_readability'],
            'summary': other['summary'], 'runtime': other['runtime'],
            'code_quality': other['code_quality'], 'action': other['action'],
            'rationale': other['rationale'],
            'reason': match['reason']})
        for field in ('sources', 'relationships', 'uncertainties'):
            for value in other[field]:
                if value not in retained[field]:
                    retained[field].append(value)
        hidden.add(alias)
    items = [item for item in items if item['id'] not in hidden]
    census = json.loads((INVENTORY_DIR / 'files.json').read_text())
    refs = {ref['path'].split('#', 1)[0] for item in items
            for ref in item['sources'] if isinstance(ref, dict)}
    tracked = {row['path'] for row in census['files']}
    exact_refs = sorted(refs & tracked)
    dir_refs = sorted(ref for ref in refs if (ROOT / ref).is_dir())
    counts = dict(Counter(item['kind'] for item in items))
    data = {'baseline': census['baseline'], 'physical': census['summary'],
            'kinds': counts, 'surveys': surveys, 'items': items, 'reconciliation': reconciliation,
            'coverage': {'explicit_tracked_source_paths': len(exact_refs),
                         'directory_references': len(dir_refs),
                         'unreferenced_tracked_paths': len(tracked - refs),
                         'meaning': 'Direct source links are discovery evidence, not proof that every linked file was fully reviewed. Directory links do not count all their descendants as reviewed.'}}
    raw = json.dumps(data, indent=2, ensure_ascii=False) + '\n'
    payload = json.dumps(data, ensure_ascii=False).replace('<', '\\u003c')
    source_base = 'https://joern-pc.tailc5e761.ts.net/files' + quote(str(ROOT), safe='/') + '/'
    questions_path = INVENTORY_DIR / 'review-notes.html'
    notes = questions_path.read_text() if questions_path.exists() else '<p>The survey is being integrated. The review questions and next-step comparison will appear here.</p>'
    guide = source_base + 'docs/research-guide.md'
    page = '''<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>What is in this repository?</title>
<style>
:root{--ink:#182a34;--sub:#526675;--line:#ccd8df;--accent:#126578;--wash:#f1f7f8}*{box-sizing:border-box}body{margin:0;background:#edf2f5;color:var(--ink);font:17px/1.55 system-ui,sans-serif}main{max-width:1200px;margin:auto;padding:32px;background:white}h1{font-size:2.1rem;line-height:1.2}h2{margin-top:2.7rem;border-top:2px solid var(--line);padding-top:1rem}h3{font-size:1.1rem}a{color:var(--accent)}small,.muted{color:var(--sub)}nav{display:flex;gap:18px;flex-wrap:wrap;padding:15px 0}.box{background:var(--wash);padding:16px;border-left:4px solid var(--accent)}.warning{background:#fff8e8;border-color:#a57b2a}.stats{display:flex;flex-wrap:wrap;gap:12px;margin:20px 0}.stat{background:var(--wash);padding:12px 18px;flex:1;min-width:145px}.stat b{font-size:1.5rem;display:block}input,select{font:inherit;padding:8px;border:1px solid var(--line);border-radius:3px;max-width:100%}.filters{display:flex;flex-wrap:wrap;gap:12px;margin:15px 0}input{flex:1;min-width:210px}table{border-collapse:collapse;width:100%;font-size:.92rem}th,td{text-align:left;vertical-align:top;border-bottom:1px solid var(--line);padding:10px}th{background:var(--wash)}.tablewrap{overflow-x:auto}.item{border:1px solid var(--line);margin:12px 0;border-radius:4px}.item>summary{padding:13px 16px;cursor:pointer}.item>summary strong{display:inline-block}.item>summary small{display:block;margin-left:17px}.detail{padding:0 18px 18px}.badges{font-size:.82rem;color:var(--sub);margin-right:10px}.facts{display:grid;grid-template-columns:170px 1fr;gap:7px 15px}.facts dt{font-weight:650}.facts dd{margin:0}li{margin:6px 0}.coverage{border:1px solid var(--line);padding:12px;margin:10px 0}.coverage summary{cursor:pointer;font-weight:650}code{font-size:.88em;overflow-wrap:anywhere}.q{padding:14px;background:#f7f8ee;border-left:4px solid #7c8238;margin:14px 0}.anchor{scroll-margin-top:10px}#nojs{color:#ad3018}@media(max-width:650px){main{padding:20px 15px}.facts{grid-template-columns:1fr;gap:3px}.facts dd{margin-bottom:10px}h1{font-size:1.7rem}th,td{padding:7px}}@media print{body{background:white}main{max-width:none}.filters,nav{display:none}details{break-inside:avoid}}
</style></head><body><main><p class="muted">Semantic sources updated after 12 October migration · Physical file census frozen at 10 October baseline · Not a full proof or code audit</p>
<h1>What is in this repository?</h1><p>This is the small layer we can discuss and rearrange before changing thousands of underlying files. Entries describe what exists, what supports it, and what remains unclear.</p>
<nav><a href="#decisions">10 Oct discussion</a><a href="#catalogue">Browse the inventory</a><a href="#coverage">What was covered?</a><a href="#next">10 Oct next steps</a><a href="GUIDE">Read the agent guide</a></nav>
<section id="decisions" class="anchor">NOTES</section>
<div id="stats" class="stats"></div>
<section id="catalogue" class="anchor"><h2>Browse results, code, data and guidance</h2><p>Search by topic or path. Open an entry for proof/readability, runtime, code-quality and source details. <strong>Entries include individual results and grouped families: their total is not a theorem count.</strong></p>
<div class="filters"><input id="search" type="search" placeholder="Search: HKO, flow, certification, stale…" aria-label="Search inventory"><select id="kind" aria-label="Filter by kind"><option value="">All kinds</option></select><select id="survey" aria-label="Filter by survey"><option value="">All areas</option></select></div><p id="match" class="muted"></p><div id="items"></div><noscript><p id="nojs">Enable JavaScript to browse this standalone inventory, or read the source JSON files under meta-topics/repository-organization/inventory/parts.</p></noscript></section>
<section id="coverage" class="anchor"><h2>What was covered—and what was not?</h2><p>The physical census describes tracked paths in the 10 October pre-migration baseline, so its area counts do not describe this worktree. The semantic surveys inspect selected source material and follow relationships, with source locations updated after the move. Neither means every line of code, proof or historical note has been reviewed. External cached artifacts and later untracked working additions are outside the baseline census.</p><div id="coverageStats" class="box"></div><div id="surveys"></div><h3>Tracked files by area in the 10 October baseline</h3><div class="tablewrap"><table><thead><tr><th>Area</th><th>Paths</th></tr></thead><tbody id="domains"></tbody></table></div><p class="muted">Source references can point to files or collections. The view does not silently treat a directory link as proof that all descendants were understood.</p></section>
<p class="muted">Generated from independent topic records. Stable entry IDs support corrections without renaming the underlying files. This page does not certify mathematical or human-writing acceptance.</p>
<script id="inventory-data" type="application/json">PAYLOAD</script><script>
const data=JSON.parse(document.getElementById('inventory-data').textContent),base=BASE;
const E=s=>String(s??'Unknown').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const text=v=>typeof v==='string'?v:JSON.stringify(v,null,2);
const source=(r)=>{if(typeof r==='string')r={path:r};return '<a href="'+base+r.path.split('/').map(encodeURIComponent).join('/')+'">'+E(r.path)+'</a>'+((r.supports||r.relation)?' — '+E(r.supports||r.relation):'')};
const kindLabels={'result':'Result','result-family':'Result family','algorithm':'Algorithm / code','dataset':'Dataset','figure-set':'Figures','workflow':'Workflow','reference':'Reference','collection':'Collection'};
for(const k of Object.keys(data.kinds).sort()){const o=document.createElement('option');o.value=k;o.textContent=(kindLabels[k]||k)+' ('+data.kinds[k]+')';document.getElementById('kind').append(o)}
for(const s of data.surveys){const o=document.createElement('option');o.value=s.name;o.textContent=s.name;document.getElementById('survey').append(o)}
document.getElementById('stats').innerHTML=[['Tracked paths (10 Oct)',data.physical.tracked_paths],['Inventory entries',data.items.length],['Baseline source files linked',data.coverage.explicit_tracked_source_paths],['Cargo manifests (10 Oct)',data.physical.cargo_manifests.length]].map(([k,v])=>'<div class="stat"><b>'+E(v)+'</b>'+E(k)+'</div>').join('');
function row(i){const fields=[['Support / status',i.state],['For agent readers',i.agent_readability],['For human readers',i.human_readability],['Execution cost',i.runtime],['Code quality',i.code_quality],['Proposed treatment',i.action+' — '+i.rationale]];return '<details class="item" id="'+E(i.id)+'"><summary><span class="badges">'+E(i.id)+' · '+E(kindLabels[i.kind]||i.kind)+'</span><strong>'+E(i.title)+'</strong><small>'+E(i.summary)+'</small></summary><div class="detail"><dl class="facts">'+fields.map(([k,v])=>'<dt>'+E(k)+'</dt><dd>'+E(text(v))+'</dd>').join('')+'</dl>'+(i.additional_assessments?'<details><summary>Reconciled duplicate and other survey observations</summary><pre style="white-space:pre-wrap">'+E(text(i.additional_assessments))+'</pre></details>':'')+'<h3>Sources and what they support</h3><ul>'+i.sources.map(r=>'<li>'+source(r)+'</li>').join('')+'</ul>'+(i.relationships.length?'<h3>Relationships</h3><ul>'+i.relationships.map(r=>'<li>'+source(r)+'</li>').join('')+'</ul>':'')+(i.uncertainties.length?'<h3>Still unclear</h3><ul>'+i.uncertainties.map(x=>'<li>'+E(text(x))+'</li>').join('')+'</ul>':'')+'</div></details>'}
function render(){const q=document.getElementById('search').value.toLowerCase(),k=document.getElementById('kind').value,s=document.getElementById('survey').value;const filtered=data.items.filter(i=>(!k||i.kind===k)&&(!s||i.survey===s||(i.additional_assessments||[]).some(a=>a.survey===s))&&(!q||JSON.stringify(i).toLowerCase().includes(q)));document.getElementById('match').textContent=filtered.length+' of '+data.items.length+' entries';document.getElementById('items').innerHTML=filtered.map(row).join('');openHash()}
function openHash(){let id=decodeURIComponent(location.hash.slice(1));const alias=data.reconciliation.aliases.find(x=>x.alias===id);if(alias)id=alias.canonical;const el=document.getElementById(id);if(el&&el.tagName==='DETAILS')el.open=true}
for(const id of ['search','kind','survey'])document.getElementById(id).addEventListener(id==='search'?'input':'change',render);
window.addEventListener('hashchange',openHash);
document.getElementById('coverageStats').textContent=data.coverage.explicit_tracked_source_paths+' distinct tracked source files are explicitly linked by entries; '+data.coverage.directory_references+' additional references point to directories. '+data.coverage.unreferenced_tracked_paths+' tracked paths are not individually cited. These are coverage signals, not reviewed/unreviewed verdicts.';
document.getElementById('surveys').innerHTML=data.surveys.map(s=>'<details class="coverage"><summary>'+E(s.name)+' — '+E(text(s.scope))+'</summary><p>'+E(text(s.coverage))+'</p><h3>Changes reported by this survey</h3><pre style="white-space:pre-wrap">'+E(text(s.changes))+'</pre></details>').join('');
document.getElementById('domains').innerHTML=Object.entries(data.physical.by_top_level).map(([k,v])=>'<tr><td>'+E(k)+'</td><td>'+E(v)+'</td></tr>').join('');render();
</script></main></body></html>'''.replace('GUIDE', guide).replace('NOTES', notes).replace('PAYLOAD', payload).replace('BASE;', json.dumps(source_base) + ';')
    return {INVENTORY_DIR / 'index.json': raw, INVENTORY_DIR / 'index.html': page}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='Validate source paths and check generated projections without writing.')
    args = parser.parse_args()
    outputs = generate()
    if args.check:
        mismatches = [str(p.relative_to(ROOT)) for p, text in outputs.items() if not p.exists() or p.read_text() != text]
        if mismatches:
            raise SystemExit('Stale inventory projections: ' + ', '.join(mismatches))
        print('PASS: inventory schema, unique IDs, source paths and generated projections. Not semantic completeness or proof validity.')
    else:
        for path, text in outputs.items():
            path.write_text(text)
        print('Rendered inventory; reconciled counts are recorded in meta-topics/repository-organization/inventory/index.json.')

if __name__ == '__main__':
    main()
