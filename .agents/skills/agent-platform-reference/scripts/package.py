#!/usr/bin/env python3
"""Package the validated project and verify its distributable file checksums."""
import argparse,hashlib,json,zipfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
SKIP={'.git','.pytest_cache','__pycache__'}
INDEX='evidence/package-integrity.json'
def files(root):
 return sorted(p for p in root.rglob('*') if p.is_file() and not any(x in SKIP for x in p.relative_to(root).parts) and p.name!='.refresh.lock' and p.relative_to(root).as_posix()!=INDEX)
def verify(root):
 index=json.loads((root/INDEX).read_text()); errors=[]
 for relative,meta in index.items():
  p=root/relative
  if not p.is_file():errors.append('missing '+relative)
  elif p.stat().st_size!=meta['bytes'] or hashlib.sha256(p.read_bytes()).hexdigest()!=meta['sha256']:errors.append('mismatch '+relative)
 return errors
def main():
 parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--verify',action='store_true');parser.add_argument('--output',type=Path,default=ROOT.parent/'agent-platform-reference.zip');a=parser.parse_args()
 if a.verify:
  errors=verify(ROOT)
  if errors:print('\n'.join(errors));raise SystemExit(1)
  print('PASS package checksums');return
 # Hosted clients receive curation, not an oversized bulk corpus.
 thin=ROOT/'dist/agent-platform-reference-thin.zip';thin.parent.mkdir(exist_ok=True)
 with zipfile.ZipFile(thin,'w',zipfile.ZIP_DEFLATED) as z:
  for p in [ROOT/'SKILL.md',*sorted((ROOT/'references').glob('*.md'))]:
   body=p.read_text()
   if p.name=='INDEX.md':body='Thin distribution: curated references and upstream URLs only. All corpus paths refer to the complete offline package.\n\n'+body
   if p.name=='SKILL.md':body=body.replace('Read [references/INDEX.md]', 'This thin distribution contains curation only; use the full package for offline search or maintenance, otherwise use upstream URLs. Read [references/INDEX.md]')
   z.writestr((Path(ROOT.name)/p.relative_to(ROOT)).as_posix(),body)
 selected=files(ROOT);index={p.relative_to(ROOT).as_posix():{'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in selected};(ROOT/INDEX).write_text(json.dumps(index,indent=2)+'\n')
 with zipfile.ZipFile(a.output,'w',zipfile.ZIP_DEFLATED) as z:
  for p in [*selected,ROOT/INDEX]:z.write(p,Path(ROOT.name)/p.relative_to(ROOT))
 with zipfile.ZipFile(a.output) as z:
  if z.testzip():raise SystemExit('archive CRC failed')
 print(json.dumps({'output':str(a.output),'files':len(selected)+1,'bytes':a.output.stat().st_size,'sha256':hashlib.sha256(a.output.read_bytes()).hexdigest()}))
if __name__=='__main__':main()
