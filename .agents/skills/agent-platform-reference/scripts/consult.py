#!/usr/bin/env python3
"""Read one relevant corrective reference without loading the whole corpus."""
import argparse,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
ROUTES={'handoff':'repository-handoff.md','goal':'goal-loops.md','questions':'user-input-lifecycle.md','repair':'corrective-maintenance.md','chatgpt':'chatgpt.md','claude':'claude-chat.md','claude-code':'claude-code.md','codex':'codex.md','api':'api-agents.md','deploy':'skill-deployment.md','research':'research-execution.md','traces':'session-traces.md','models':'model-evidence.md'}
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('topic',choices=ROUTES);p.add_argument('--measure',action='store_true');a=p.parse_args();f=ROOT/'references'/ROUTES[a.topic];body=f.read_text()
 if a.measure:print(json.dumps({'path':str(f.relative_to(ROOT)),'bytes':len(body.encode()),'words':len(body.split())}));return
 print('Reference: '+str(f.relative_to(ROOT))+'\n');print(body)
if __name__=='__main__':main()
