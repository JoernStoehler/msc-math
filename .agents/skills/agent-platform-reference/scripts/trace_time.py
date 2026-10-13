#!/usr/bin/env python3
"""Attribute normalized measured spans, preserving overlap and unobserved time."""
import argparse,json
from collections import defaultdict
from datetime import datetime

def seconds(v):
 if isinstance(v,(int,float)):return float(v)
 return datetime.fromisoformat(v.replace('Z','+00:00')).timestamp()
def union(intervals):
 total=0.;end=None
 for a,b in sorted(intervals):
  if end is None or a>=end:total+=b-a
  elif b>end:total+=b-end
  end=b if end is None else max(end,b)
 return total
def analyze(data):
 if data.get('schema')!='measured-spans/v1':raise ValueError('explicit normalized measured-spans/v1 schema required')
 start,end=seconds(data['start']),seconds(data['end'])
 if end<start:raise ValueError('reversed run boundary')
 groups=defaultdict(list);intervals=[];work=0
 for span in data['spans']:
  a,b=seconds(span['start']),seconds(span['end'])
  if not start<=a<=b<=end:raise ValueError('span outside run bounds or reversed: '+span['id'])
  if not span.get('timing_basis'):raise ValueError('timing basis required: '+span['id'])
  groups[span['category']].append((a,b));intervals.append((a,b));work+=b-a
 coverage=union(intervals);category={k:union(v) for k,v in sorted(groups.items())}
 # Category excess is overlap across category unions, distinct from nested same-category sum.
 return {'wall_seconds':end-start,'summed_span_seconds':work,'observed_union_seconds':coverage,'unobserved_seconds':end-start-coverage,'category_union_seconds':category,'cross_category_overlap_seconds':sum(category.values())-coverage,'scope':data.get('provenance','unspecified'),'warning':'Category times overlap; sum is not a wall-time partition. Transcripts are not presumed full execution traces.'}
if __name__=='__main__':
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('trace');a=p.parse_args()
 with open(a.trace) as f:data=json.load(f)
 print(json.dumps(analyze(data),indent=2))
