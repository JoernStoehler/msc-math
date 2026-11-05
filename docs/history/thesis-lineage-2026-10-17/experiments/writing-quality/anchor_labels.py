#!/usr/bin/env python3
"""Locate literal quoted targets in extracted inputs; never infer human span boundaries."""
import json
import re
import unicodedata
from dataset import LOCAL, rows


def normalize(text):
    output, mapping = [], []
    for index, char in enumerate(text):
        for expanded in unicodedata.normalize('NFKC', char):
            if expanded.isspace():
                if output and output[-1] == ' ':
                    continue
                expanded = ' '
            output.append(expanded)
            mapping.append(index)
    return ''.join(output), mapping


def main():
    cases = {c['case_id']: c for c in rows(LOCAL / 'cases.jsonl')}
    output = []
    for label in rows(LOCAL / 'labels.jsonl'):
        c = cases[label['case_id']]
        text = (LOCAL / c['text']).read_text()
        normalized, mapping = normalize(text)
        quotes = [label['target_quote']] if label.get('target_quote') else re.findall('“([^”]+)”', label['evidence_quote'])
        anchors = []
        for quote in quotes:
            needle = normalize(quote)[0].strip()
            if not needle:
                continue
            for match in re.finditer(re.escape(needle), normalized):
                start, end = mapping[match.start()], mapping[match.end()-1] + 1
                pages = rows(LOCAL / c['pages']) if c.get('pages') else []
                page = next((p for p in pages if p['start_char'] <= start < p['end_char']), None)
                anchors.append({'quoted_candidate': quote, 'start_char': start, 'end_char': end,
                                'matched_text': text[start:end], 'pdf_page': page['pdf_page'] if page else None,
                                'page_start_char': start-page['start_char'] if page else None})
        output.append({'label_id': label['label_id'], 'case_id': c['case_id'],
                       'method': 'literal quoted candidate, Unicode NFKC and whitespace normalization only',
                       'status': 'candidate_locations' if anchors else 'not_located', 'anchors': anchors,
                       'caution': 'Quoted words may be suggested repairs or general judgments. Locations are not human-highlight boundaries; no unmatched span is a clean label.'})
    (LOCAL / 'anchors.jsonl').write_text(''.join(json.dumps(row,ensure_ascii=False)+'\n' for row in output))
    print(f'{sum(bool(r["anchors"]) for r in output)}/{len(output)} observations have literal candidate locations')

if __name__ == '__main__': main()
