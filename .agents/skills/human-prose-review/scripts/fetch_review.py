#!/usr/bin/env python3
"""Fetch one reviewed document without printing or serving its API credential."""
import argparse, datetime, json, sys, urllib.request, urllib.parse, urllib.error
from pathlib import Path
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--review', type=Path, help='review.json from open_review.py')
p.add_argument('--uri', help='Explicit document URI for an existing review')
p.add_argument('--output', type=Path, required=True, help='New JSON snapshot; must not exist')
p.add_argument('--compare', type=Path, help='Compare IDs and feedback fields with a prior JSON export')
p.add_argument('--user', default='acct:JoernStoehler@hypothes.is')
a = p.parse_args()
if bool(a.review) == bool(a.uri):
    p.error('Supply exactly one of --review or --uri')
if a.review:
    a.uri = json.loads(a.review.read_text())['document_uri']
if a.output.exists():
    raise SystemExit('Output exists; choose a new snapshot filename.')
credential = Path.home() / '.config/hypothesis/api-token'
if not credential.exists():
    raise SystemExit('Hypothesis token has not been saved yet.')
print('Reading Hypothesis API credential from ~/.config/hypothesis/api-token; value not printed.', flush=True)
token = credential.read_text().strip()
class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None
opener = urllib.request.build_opener(NoRedirect())
params = {'uri': a.uri, 'user': a.user, 'limit': 200, 'sort': 'id', 'order': 'asc'}
rows, seen, pages = [], set(), []
while True:
    req = urllib.request.Request('https://hypothes.is/api/search?' + urllib.parse.urlencode(params),
        headers={'Authorization': 'Bearer ' + token, 'Accept': 'application/json'})
    try:
        with opener.open(req, timeout=30) as response:
            page = json.load(response)
    except urllib.error.HTTPError as e:
        raise SystemExit(f'Hypothesis returned HTTP {e.code}; no response body or credential printed.')
    except urllib.error.URLError:
        raise SystemExit('Hypothesis connection failed; credential not printed.')
    pages.append({k: v for k, v in page.items() if k != 'rows'})
    batch = page['rows']
    for row in batch:
        if row['id'] in seen:
            raise SystemExit('Repeated annotation in pagination; not saving an incomplete snapshot.')
        seen.add(row['id']); rows.append(row)
    if len(batch) < 200:
        break
    params['search_after'] = batch[-1]['id']
result = {'fetched_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'query': {'uri': a.uri, 'user': a.user}, 'pages': pages, 'annotations': rows}
with a.output.open('x') as f:
    json.dump(result, f, ensure_ascii=False, indent=2)
    f.write('\n')
print(f'Saved {len(rows)} annotations to {a.output}')
if a.compare:
    prior = {r['id']: r for r in json.loads(a.compare.read_text())['annotations']}
    current = {r['id']: r for r in rows}
    fields = ['text', 'target', 'uri', 'created', 'updated', 'tags', 'permissions', 'user']
    changed = {k: [f for f in fields if current[k].get(f) != prior[k].get(f)] for k in prior.keys() & current.keys()}
    changed = {k: v for k, v in changed.items() if v}
    print(json.dumps({'prior_count': len(prior), 'fetched_count': len(current),
        'missing_ids': sorted(prior.keys()-current.keys()), 'additional_ids': sorted(current.keys()-prior.keys()),
        'changed_fields': changed}, indent=2))
