#!/usr/bin/env python3
"""One-shot, root-scoped research cutoff; never starts turns or redeems quota.

Uses installed websocket-client and the release-0.160.0 local daemon protocol.
Preflight is read-only unless --enforce is passed. Independent sessions and
ordinary forks are excluded by the existing spawned-descendant inventory.
"""
import argparse
import datetime as dt
import importlib.util
import json
from pathlib import Path
import socket
import time
import websocket


def scope(home, root):
    spec = importlib.util.spec_from_file_location('subtree_usage', Path(__file__).with_name('subtree-usage.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    sessions = module.inventory(home)
    if root not in sessions:
        raise RuntimeError('Root missing from local inventory; refusing an unscoped stop')
    return sorted(module.descendants(sessions, root) - {root}) + [root]


class RPC:
    def __init__(self, home):
        sock = socket.socket(socket.AF_UNIX)
        sock.settimeout(5)
        try:
            sock.connect(str(home / 'app-server-control/app-server-control.sock'))
            self.ws = websocket.create_connection('ws://localhost/', socket=sock, timeout=5)
        except Exception:
            sock.close()
            raise
        self.ident = 0
        self.request('initialize', {'clientInfo': {'name': 'msc_math_cutoff', 'version': '1'},
                                    'capabilities': {'experimentalApi': True}})
        self.ws.send(json.dumps({'method': 'initialized'}))

    def request(self, method, params):
        self.ident += 1
        self.ws.send(json.dumps({'id': self.ident, 'method': method, 'params': params}))
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline:
            reply = json.loads(self.ws.recv())
            if reply.get('id') != self.ident:
                continue
            if 'error' in reply:
                # Do not copy daemon error text, which can contain private data.
                raise RuntimeError(f'{method} rejected (code {reply["error"].get("code")})')
            return reply['result']
        raise TimeoutError(method)

    def close(self):
        self.ws.close()


def sweep(rpc, ids, enforce):
    errors = []
    paused = interrupted = active = 0
    # Disable autonomous continuation across the entire selected tree first.
    for ident in ids:
        try:
            goal = rpc.request('thread/goal/get', {'threadId': ident}).get('goal')
            if enforce and goal and goal.get('status') == 'active':
                rpc.request('thread/goal/set', {'threadId': ident, 'status': 'paused'})
                paused += 1
        except Exception as exc:
            errors.append({'thread': ident, 'operation': 'goal', 'failure': type(exc).__name__})
    for ident in ids:
        try:
            turns = rpc.request('thread/turns/list', {'threadId': ident, 'limit': 5, 'itemsView': 'summary'})['data']
            for turn in turns:
                if turn['status'] == 'inProgress':
                    active += 1
                    if enforce:
                        rpc.request('turn/interrupt', {'threadId': ident, 'turnId': turn['id']})
                        interrupted += 1
        except Exception as exc:
            errors.append({'thread': ident, 'operation': 'interrupt', 'failure': type(exc).__name__})
    return {'scoped_threads': len(ids), 'active_turns_observed': active,
            'goals_paused': paused, 'interrupts_accepted': interrupted, 'errors': errors}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', required=True)
    parser.add_argument('--at', required=True, help='UTC timestamp; enforcement before this time is refused')
    parser.add_argument('--home', type=Path, default=Path.home() / '.codex')
    parser.add_argument('--enforce', action='store_true')
    parser.add_argument('--guard-seconds', type=int, default=120)
    args = parser.parse_args()
    cutoff = dt.datetime.fromisoformat(args.at.replace('Z', '+00:00'))
    if cutoff.tzinfo is None:
        parser.error('--at requires a timezone')
    if args.enforce and dt.datetime.now(dt.timezone.utc) < cutoff:
        parser.error('enforcement before the selected cutoff is refused')
    if not 0 <= args.guard_seconds <= 300:
        parser.error('--guard-seconds must be between 0 and 300')
    end = time.monotonic() + (args.guard_seconds if args.enforce else 0)
    failed = False
    while True:
        rpc = None
        try:
            ids = scope(args.home, args.root)
            rpc = RPC(args.home)
            report = sweep(rpc, ids, args.enforce)
            failed = bool(report['errors'])
        except Exception as exc:
            failed = True
            report = {'failure': type(exc).__name__}
        finally:
            if rpc:
                rpc.close()
        print(json.dumps({'at': dt.datetime.now(dt.timezone.utc).isoformat(),
                          'enforce': args.enforce, **report}), flush=True)
        if not args.enforce or time.monotonic() >= end:
            break
        time.sleep(min(2, max(0, end - time.monotonic())))
    return int(failed)


if __name__ == '__main__':
    raise SystemExit(main())
