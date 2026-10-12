#!/usr/bin/env python3
"""Compare full migrated features with immutable historical table, preserving all discrepancies."""
import hashlib,json,pathlib,sys
here=pathlib.Path(__file__).resolve().parent
oldpath=pathlib.Path(sys.argv[1]); oldbytes=oldpath.read_bytes()
assert hashlib.sha256(oldbytes).hexdigest()=='607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59'
old={r['poly_id']:r for r in map(json.loads,oldbytes.splitlines())}
new=[json.loads(x) for x in (here/'output/polytope-table.jsonl').read_text().splitlines()]
assert len(new)==14336 and len({r['poly_id'] for r in new})==14336
assert {r['poly_id'] for r in new}==set(old)
oldkeys=set.union(*(set(old[r['poly_id']]) for r in new)); newkeys=set.union(*(set(r) for r in new)); common=oldkeys&newkeys
mismatches=[]; numerical=[]
for r in new:
 for k in sorted(common):
  a=old[r['poly_id']].get(k); b=r.get(k)
  if a!=b:
   item={'poly_id':r['poly_id'],'field':k,'historical':a,'pilot':b}
   if isinstance(a,(int,float)) and not isinstance(a,bool) and isinstance(b,(int,float)):
    item.update(absolute_difference=abs(a-b),relative_difference=abs(a-b)/max(abs(a),abs(b),1e-300)); numerical.append(item)
   mismatches.append(item)
receipt={'rows':len(new),'old_keys':sorted(oldkeys),'new_keys':sorted(newkeys),'added_fields':sorted(newkeys-oldkeys),'removed_fields':sorted(oldkeys-newkeys),'compared_cells':len(common)*len(new),'mismatches':mismatches,'numerical_mismatch_count':len(numerical),'max_absolute_difference':max((r['absolute_difference'] for r in numerical),default=0),'max_relative_difference':max((r['relative_difference'] for r in numerical),default=0),'source_table_sha256':hashlib.sha256(oldbytes).hexdigest()}
(here/'comparison.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:receipt[k] for k in ['rows','added_fields','removed_fields','compared_cells','numerical_mismatch_count','max_absolute_difference','max_relative_difference']}))
