#!/usr/bin/env python3
"""Render the historical report and selected evidence; requires Python and Pandoc.

No network, browser, or private corpus is required for rendering. --check verifies
that report.html is the exact current rendering. --verify-private additionally
checks retained source hashes and selected visible-message excerpts when the
private frozen corpus is available; it never copies those raw logs.
"""
import argparse
import hashlib
import html
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent
CSS = """
:root{color-scheme:light;--ink:#19272b;--muted:#536268;--line:#cfd9dc;--paper:#fff;--wash:#f4f7f7;--accent:#14616a}
*{box-sizing:border-box}html{scroll-padding-top:1.5rem}body{margin:0;color:var(--ink);background:var(--paper);font:18px/1.63 system-ui,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}
a{color:var(--accent);text-underline-offset:.16em;text-decoration-thickness:.065em}a:hover{text-decoration-thickness:.12em}a:focus-visible,summary:focus-visible{outline:3px solid #e6a329;outline-offset:4px}
header{border-bottom:1px solid var(--line);padding:1.25rem 2rem;background:var(--wash)}header p{margin:.15rem 0}.eyebrow{color:var(--muted);font-size:.82rem;letter-spacing:.07em;text-transform:uppercase}.files{font-size:.92rem}.layout{display:grid;grid-template-columns:255px minmax(0,1fr);max-width:1450px;margin:auto;gap:2.5rem;padding:2.1rem 2rem 5rem}nav{align-self:start;position:sticky;top:1rem;max-height:94vh;overflow:auto;font-size:.85rem;line-height:1.45;padding-right:.6rem}nav ul{list-style:none;padding-left:0}nav ul ul{padding-left:.8rem;border-left:1px solid var(--line);margin:.45rem 0}nav li{margin:.6rem 0}nav a{text-decoration:none}nav .nav-title{font-weight:650;color:var(--muted)}main{min-width:0;max-width:91ch}h1{font-size:2rem;line-height:1.18;letter-spacing:-.025em;margin:0 0 1.3rem}h2{font-size:1.4rem;line-height:1.3;margin-top:2.6rem;padding-top:.6rem;border-top:1px solid var(--line)}h3{font-size:1.13rem;line-height:1.4;margin-top:2rem}h4{font-size:1rem;margin:1.2rem 0 .5rem}p{margin:.9rem 0}li{margin:.35rem 0}strong{font-weight:650}.table-wrap{overflow-x:auto;margin:1.3rem 0;border:1px solid var(--line);border-radius:5px}table{border-collapse:collapse;width:100%;font-size:.87rem;line-height:1.48}th,td{padding:.7rem .75rem;text-align:left;vertical-align:top;border-bottom:1px solid var(--line)}th{background:var(--wash);font-weight:650}tr:last-child td{border-bottom:0}th{min-width:9rem}td{overflow-wrap:anywhere}code{font: .86em/1.5 ui-monospace,SFMono-Regular,Consolas,monospace;background:#f1f4f5;padding:.08em .22em;border-radius:3px;overflow-wrap:anywhere}pre{white-space:pre-wrap;overflow-wrap:anywhere}blockquote{margin:1rem 0;padding:.15rem 1.1rem;border-left:3px solid #b6cdd1;color:#344a51}.note{color:var(--muted);font-size:.88rem}.evidence-group{margin:1rem 0;border:1px solid var(--line);border-radius:5px;padding:.25rem 1rem .75rem}.evidence-group>summary{cursor:pointer;padding:.55rem 0;font-size:1rem;font-weight:650}.provenance{font-size:.78rem;color:var(--muted);overflow-wrap:anywhere}.excerpt{padding:1rem 0;border-top:1px solid var(--line)}.excerpt-head{font-size:.85rem;font-weight:650;margin:0 0 .45rem}.excerpt blockquote{white-space:pre-wrap;overflow-wrap:anywhere;font-size:.9rem;margin:.5rem 0;line-height:1.6}.excerpt-note{font-size:.75rem;color:var(--muted);margin:.35rem 0}footer{margin-top:2rem;border-top:1px solid var(--line);padding-top:1rem;font-size:.85rem;color:var(--muted)}
@media(max-width:1000px){body{font-size:17px}.layout{grid-template-columns:210px minmax(0,1fr);gap:1.5rem;padding:1.5rem}header{padding:1.1rem 1.5rem}}
@media(max-width:760px){.layout{display:block;padding:1.2rem}nav{position:static;max-height:none;border-bottom:1px solid var(--line);padding:0 0 1rem;margin:0 0 1.6rem}nav ul ul{display:none}nav li{margin:.35rem 0}h1{font-size:1.7rem}h2{font-size:1.3rem}header{padding:1rem 1.2rem}th,td{padding:.6rem}.evidence-group{padding:.25rem .7rem}}
@media print{body{font-size:10.5pt}.layout{display:block;padding:0}nav,header .files{display:none}header{padding:0;background:none}main{max-width:none}h1{font-size:22pt}h2{break-after:avoid;font-size:16pt}h3{break-after:avoid}a{color:inherit}.table-wrap{overflow:visible}table{font-size:8pt}details{break-inside:avoid}.excerpt{break-inside:avoid}}
"""


CSS += """
.mobile-toc{display:none}
@media(max-width:760px){.desktop-toc{display:none}.mobile-toc{display:block;border:1px solid var(--line);border-radius:5px;margin:0 0 1.4rem;padding:.55rem .8rem;background:var(--wash)}.mobile-toc>summary{cursor:pointer;font-weight:650}.mobile-toc nav{position:static;max-height:none;border:0;margin:.6rem 0 0;padding:0;background:transparent}.mobile-toc nav ul ul{display:block}.mobile-toc nav li{margin:.55rem 0}.mobile-toc nav a{display:inline-block;padding:.15rem 0}}
@media print{.mobile-toc,.desktop-toc{display:none}}
"""


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def escaped_excerpt(value):
    # Preserve exact quoted whitespace without trailing whitespace in this
    # generated source file. Character references decode to the same text.
    return re.sub(r'[ \t]+(?=\n|$)', lambda m: ''.join('&#32;' if c==' ' else '&#9;' for c in m[0]), html.escape(value))


def evidence_html(data):
    out = ['<section id="evidence-appendix"><h2>Selected evidence appendix</h2>',
           '<p class="note">The main report is the analysis. These expandable records retain exact selected excerpts for checking it. They are not a complete log or an incident count. Quotations retain the original spelling; an agent’s claim is evidence of what it said, not independent proof that the claim is true.</p>']
    metrics=data.get('historical_resource_metrics')
    if metrics:
        out.append('<details class="evidence-group" id="resource-metrics"><summary>Historical resource aggregates and method</summary>')
        out.append('<p class="note">Eight roots and 29 descendants, using the same frozen observation window. These counts describe allocation; they do not establish overload, wasted spend or human effort.</p>')
        out.append('<div class="table-wrap"><table><thead><tr><th>Owner class</th><th>Recorded responses</th><th>Input processed</th><th>Cached subset</th><th>Output</th></tr></thead><tbody>')
        for key,label in [('aggregate_roots','Roots'),('aggregate_descendants','Descendants'),('aggregate_selected','Selected total')]:
            values=metrics['aggregates'][key]
            out.append('<tr><td>'+label+'</td>'+''.join(f'<td>{values[field]:,}</td>' for field in ['recorded_responses','input_tokens','cached_input_tokens','output_tokens'])+'</tr>')
        out.append('</tbody></table></div><p>'+html.escape(metrics['method'])+'</p><ul>')
        for limitation in metrics['limitations']:out.append('<li class="note">'+html.escape(limitation)+'</li>')
        out.append('</ul><p class="note">Validation found no duplicate response IDs, owner mismatches or missing numeric usage fields in the captured records. Copied child-history records were excluded from behavioral counts. Live quota observations are kept separate.</p>')
        for provenance in metrics['private_provenance']:
            out.append('<p class="provenance">Private method/source: <code>'+html.escape(provenance['path'])+'</code><br>SHA-256 <code>'+provenance['sha256']+'</code></p>')
        out.append('</details>')
    runtime=data.get('runtime_source_evidence')
    if runtime:
        out.append('<details class="evidence-group" id="runtime-source"><summary>Matching runtime source and version provenance</summary>')
        out.append('<p>'+html.escape(runtime['provenance_note'])+'</p>')
        if runtime.get('delivery_audit_note'):
            out.append('<p>'+html.escape(runtime['delivery_audit_note'])+'</p>')
        out.append('<p class="provenance">Release <code>'+html.escape(runtime['observed_release'])+'</code>; source commit <code>'+runtime['source_commit']+'</code>; observed executable SHA-256 <code>'+runtime['binary_sha256']+'</code>.</p>')
        for entry in runtime['entries']:
            link=runtime['source_repository']+'/blob/'+runtime['source_commit']+'/'+entry['path']+'#L'+str(entry['start_line'])
            out.append('<article class="excerpt" id="'+entry['id']+'"><p class="excerpt-head">'+html.escape(entry['label'])+'</p><p class="provenance"><a href="'+html.escape(link,quote=True)+'">'+html.escape(entry['path'])+'</a> · lines '+str(entry['start_line'])+'–'+str(entry['end_line'])+'<br>File SHA-256 <code>'+entry['file_sha256']+'</code></p><pre>'+html.escape(entry['excerpt'])+'</pre></article>')
        out.append('</details>')
    delivery=data.get('historical_input_delivery_audit')
    if delivery:
        out.append('<details class="evidence-group" id="input-delivery-audit"><summary>Historical receipt checks against the live-history budget</summary><p>'+html.escape(delivery['method'])+'</p><p>'+html.escape(delivery['limitation'])+'</p><div class="table-wrap"><table><thead><tr><th>Receipt</th><th>Text-block bytes</th><th>Approximate tokens / cap</th><th>Established boundary</th></tr></thead><tbody>')
        for row in delivery['records']:
            label=row['source_key']+' L'+str(row['source_line'])
            out.append('<tr><td><a href="#evidence-'+row['source_key'].lower()+'">'+label+'</a></td><td>'+html.escape(', '.join(map(str,row['text_item_utf8_bytes'])))+'</td><td>'+str(row['approximate_total_tokens'])+' / '+str(row['history_budget_tokens'])+'</td><td>'+html.escape(row['scope'])+'</td></tr>')
        out.append('</tbody></table></div><p class="note">Exact source-line hashes and per-item charges are retained in evidence.json. Matching runtime source is in the preceding appendix section.</p></details>')
    for src in data['sources']:
        key=src['key']; entries=[x for x in data['entries'] if x['source_key']==key]
        kind=src.get('source_kind') or ('historical root' if src['root'] else 'supporting child')
        out.append(f'<details class="evidence-group" id="evidence-{key.lower()}"><summary>{html.escape(key)} · {html.escape(src["title"])} · {kind} · {len(entries)} excerpts</summary>')
        if src.get('scope_note'):out.append('<p class="note">'+html.escape(src['scope_note'])+'</p>')
        out.append(f'<p class="provenance">Thread <code>{html.escape(src["thread_id"])}</code><br>Frozen source: {src["source_lines"]:,} lines, {src["bytes"]:,} bytes.<br>SHA-256 <code>{src["sha256"]}</code></p>')
        out.append('<details class="provenance"><summary>Private archival locations (paths only)</summary>')
        out.append(f'<p>Frozen snapshot: <code>{html.escape(src["snapshot_path"])}</code><br>Native rollout: <code>{html.escape(src["native_rollout_path"])}</code></p></details>')
        for e in entries:
            label=e.get('label') or (e['role'] + (f' / {e["phase"]}' if e.get('phase') else ''))
            out.append(f'<article class="excerpt" id="{e["id"]}"><p class="excerpt-head"><a href="#{e["id"]}">{key} L{e["line"]}</a> · {html.escape(e["timestamp"])} · {html.escape(label)}</p>')
            omitted=[]
            if e.get('omitted_before'):omitted.append('preceding content omitted')
            if e.get('omitted_after'):omitted.append('following content omitted')
            if e['kind']=='source-event-fields':
                out.append('<pre>'+escaped_excerpt(e['excerpt'])+'</pre><p class="excerpt-note">Selected recorded fields, reserialized for display; field values are unchanged.</p>')
            else:
                out.append('<blockquote>'+escaped_excerpt(e['excerpt'])+'</blockquote>')
            if omitted:out.append('<p class="excerpt-note">Excerpt: '+', '.join(omitted)+'.</p>')
            out.append('</article>')
        out.append('</details>')
    out.append('<p class="note">No raw conversation logs, private reasoning, credentials or complete tool bodies are served by this package. The selected evidence JSON retains source-line hashes and excerpt offsets. The private corpus is required only to independently verify those archival references.</p></section>')
    return ''.join(out)


def render():
    report=(ROOT/'REPORT.md').read_text()
    data=json.loads((ROOT/'evidence.json').read_text())
    # Markdown file links deliberately open the durable HTML appendix; inside
    # this generated view they can resolve directly to the same-page anchor.
    report=report.replace('(report.html#','(#')
    result=subprocess.run(['pandoc','--from=markdown+fenced_divs+pipe_tables','--to=html5','--standalone','--toc','--toc-depth=2','--metadata=title:MSc/Codex session postmortem — 3–4 October 2026'],input=report,text=True,capture_output=True,check=True)
    body=re.search(r'<body>\s*(.*?)\s*</body>',result.stdout,re.S).group(1)
    body=re.sub(r'<header id="title-block-header">.*?</header>','',body,flags=re.S)
    toc=re.search(r'<nav id="TOC".*?</nav>',body,re.S).group(0)
    body=body.replace(toc,'')
    toc=toc.replace('<ul>','<p class="nav-title">In this report</p><ul>',1)
    toc=toc.replace('</nav>','<p><a href="#evidence-appendix">Selected evidence appendix</a></p></nav>')
    desktop_toc=toc.replace('<nav', '<nav class="desktop-toc"', 1)
    mobile_inner=re.sub(r'^<nav[^>]*>|</nav>$', '', toc.strip())
    mobile_inner=re.sub(r'\s+id="[^"]*"', '', mobile_inner)
    mobile_inner=mobile_inner.replace('<p class="nav-title">In this report</p>', '')
    mobile_toc='<details class="mobile-toc"><summary>In this report</summary><nav aria-label="Report sections">'+mobile_inner+'</nav></details>'
    body=body.replace('<table>','<div class="table-wrap"><table>').replace('</table>','</table></div>')
    report_hash=digest(ROOT/'REPORT.md'); evidence_hash=digest(ROOT/'evidence.json')
    return '<!doctype html>\n<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="color-scheme" content="light"><title>MSc/Codex session postmortem — 3–4 October 2026</title><style>'+CSS+'</style></head><body><header><p class="eyebrow">Historical cases frozen 3 October 2026, 23:24:26 UTC · later process and studies labeled separately</p><p class="files"><a href="REPORT.md">Report source</a> · <a href="evidence.json?view=1">Selected evidence JSON</a> · <a href="README.md">Scope, provenance and rebuild</a></p></header><div class="layout">'+desktop_toc+mobile_toc+'<main>'+body+evidence_html(data)+f'<footer>Generated from REPORT.md and evidence.json. Report SHA-256 <code>{report_hash}</code>; evidence SHA-256 <code>{evidence_hash}</code>. Rendering does not establish the analysis or proposed interventions are correct.</footer></main></div></body></html>\n'


class LinkCheck(HTMLParser):
    def __init__(self):
        super().__init__(); self.ids=set(); self.fragments=[]; self.duplicates=[]; self.scripts=[]
    def handle_starttag(self,tag,attrs):
        a=dict(attrs)
        if 'id' in a:
            if a['id'] in self.ids:self.duplicates.append(a['id'])
            self.ids.add(a['id'])
        if tag=='a' and a.get('href','').startswith('#'):self.fragments.append(a['href'][1:])
        if tag=='script':self.scripts.append(a)


def decoded_strings(value):
    """Inspect a selected receipt internally; never export the surrounding body."""
    if isinstance(value,str):
        yield value
        try: parsed=json.loads(value)
        except (ValueError,TypeError): parsed=None
        if parsed is not None and not isinstance(parsed,str):yield from decoded_strings(parsed)
        for literal in re.findall(r'"(?:\\.|[^"\\])*"',value):
            try: text=json.loads(literal)
            except ValueError:continue
            if isinstance(text,str):yield text
    elif isinstance(value,dict):
        for item in value.values():yield from decoded_strings(item)
    elif isinstance(value,list):
        for item in value:yield from decoded_strings(item)


def verify_private(data):
    paths={s['key']:Path(s['snapshot_path']) for s in data['sources']}
    rows={}
    for src in data['sources']:
        p=paths[src['key']]
        if digest(p)!=src['sha256']:raise ValueError(f'Frozen source mismatch: {src["key"]}')
        rows[src['key']]=p.read_bytes().splitlines(keepends=True)
    for e in data['entries']:
        raw=rows[e['source_key']][e['line']-1]
        if hashlib.sha256(raw).hexdigest()!=e['source_line_sha256']:raise ValueError(f'Source line mismatch: {e["id"]}')
        if e['kind'] in ('visible-message','agent-message-excerpt'):
            msg=json.loads(raw)['payload']
            if e['kind']=='visible-message':
                assert msg['type']=='message' and msg['role'] in ('assistant','user') and msg.get('channel')!='analysis'
            else:
                assert msg['type']=='agent_message'
            text='\n'.join(x.get('text','') for x in msg.get('content',[]) if isinstance(x,dict))
            if text[e['excerpt_character_start']:e['excerpt_character_end']]!=e['excerpt']:raise ValueError(f'Excerpt mismatch: {e["id"]}')
        elif e['kind'] in ('tool-input-excerpt','tool-output-excerpt'):
            payload=json.loads(raw)['payload']
            assert payload['type'] in ('custom_tool_call','function_call','custom_tool_call_output','function_call_output')
            if not any(e['excerpt'] in text for text in decoded_strings(payload)):raise ValueError(f'Tool excerpt mismatch: {e["id"]}')
        elif e['kind']=='source-event-fields':
            obj=json.loads(raw)
            for field in e['fields']:
                value=obj
                for part in field['path']:value=value[part]
                if value!=field['value']:raise ValueError(f'Event field mismatch: {e["id"]}')
            expected=json.dumps({'.'.join(str(x) for x in field['path']):field['value'] for field in e['fields']},ensure_ascii=False,indent=2)
            if expected!=e['excerpt']:raise ValueError(f'Event display mismatch: {e["id"]}')
        else:
            raise ValueError(f'Unsupported evidence kind: {e["kind"]}')
    for record in data.get('historical_resource_metrics',{}).get('private_provenance',[]):
        if digest(Path(record['path']))!=record['sha256']:raise ValueError('Historical metric provenance mismatch')
    for entry in data.get('runtime_source_evidence',{}).get('entries',[]):
        path=Path(entry['private_source_path'])
        if digest(path)!=entry['file_sha256']:raise ValueError(f'Runtime file mismatch: {entry["id"]}')
        excerpt=''.join(path.read_text().splitlines(keepends=True)[entry['start_line']-1:entry['end_line']])
        if excerpt!=entry['excerpt'] or hashlib.sha256(excerpt.encode()).hexdigest()!=entry['excerpt_sha256']:raise ValueError(f'Runtime excerpt mismatch: {entry["id"]}')
    sources={s['key']:s for s in data['sources']}
    for row in data.get('historical_input_delivery_audit',{}).get('records',[]):
        raw=Path(sources[row['source_key']]['snapshot_path']).read_bytes().splitlines(keepends=True)[row['source_line']-1]
        source=json.loads(raw)
        sizes=[len(item['text'].encode()) for item in source['payload']['output']]
        charges=[(size+3)//4 for size in sizes]
        if hashlib.sha256(raw).hexdigest()!=row['source_line_sha256'] or sizes!=row['text_item_utf8_bytes'] or charges!=row['approximate_item_tokens'] or sum(charges)!=row['approximate_total_tokens'] or source['metadata']['fallback_token_limit_override']!=row['history_budget_tokens'] or sum(charges)>row['history_budget_tokens']:
            raise ValueError('Historical input-delivery audit mismatch')


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--check',action='store_true')
    ap.add_argument('--verify-private',action='store_true')
    args=ap.parse_args()
    data=json.loads((ROOT/'evidence.json').read_text())
    assert len([s for s in data['sources'] if s['root']])==8
    assert len({e['id'] for e in data['entries']})==len(data['entries'])
    if args.verify_private:verify_private(data)
    output=render(); check=LinkCheck();check.feed(output)
    missing=sorted(set(check.fragments)-check.ids)
    if missing or check.duplicates or check.scripts:raise ValueError({'missing_anchors':missing,'duplicate_ids':check.duplicates,'scripts':check.scripts})
    dest=ROOT/'report.html'
    if args.check:
        if not dest.exists() or dest.read_text()!=output:raise SystemExit('report.html is stale; run build_report.py')
    else:dest.write_text(output)
    print(f'PASS: rendering, {len(check.ids)} anchors, {len(data["entries"])} excerpts; eight root sources.' + (' Private hashes and all selected excerpts verified.' if args.verify_private else ''))

if __name__=='__main__':main()
