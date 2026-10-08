#!/usr/bin/env python3
"""Shell client for a persistent local Codex app-server (Linux/macOS, Python 3.10+, websocket-client)."""
import argparse
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time

try:
    import websocket
except ImportError:
    sys.exit("Missing websocket-client; run setup.sh deps or use its venv Python")


class RpcError(RuntimeError):
    pass


def emit(value, stream=sys.stdout):
    print(json.dumps(value, ensure_ascii=False), file=stream, flush=True)


def positive(value):
    number = float(value)
    if not 0 < number <= 3600:
        raise argparse.ArgumentTypeError('must be > 0 and <= 3600 seconds')
    return number


def window(value):
    number = float(value)
    if not 0 <= number <= 60:
        raise argparse.ArgumentTypeError('must be between 0 and 60 seconds')
    return number


def codex_json(*args):
    result = subprocess.run(['codex', 'app-server', 'daemon', *args],
                            capture_output=True, text=True, timeout=70)
    if result.returncode:
        raise RpcError(result.stderr.strip() or result.stdout.strip() or 'daemon command failed')
    return json.loads(result.stdout)


class Client:
    def __init__(self, path, timeout=20):
        self.timeout = timeout
        self.serial = 0
        transport = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.sock = websocket.WebSocket()
        try:
            transport.settimeout(timeout)
            transport.connect(str(path))
            self.sock.connect('ws://localhost/', socket=transport, timeout=timeout)
            self.call('initialize', {
                'clientInfo': {'name': 'msc_math_shell', 'title': 'Codex shell sessions', 'version': '1.0.0'},
                'capabilities': {'experimentalApi': True},
            })
            self.send({'method': 'initialized'})
        except BaseException:
            transport.close()
            self.close()
            raise

    def close(self):
        self.sock.close()

    def send(self, message):
        self.sock.send(json.dumps(message))

    def receive(self, deadline):
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise TimeoutError('RPC timed out; action may already have been accepted. Inspect before retrying.')
        self.sock.settimeout(remaining)
        try:
            data = self.sock.recv()
        except websocket.WebSocketTimeoutException as error:
            raise TimeoutError('RPC timed out; action may already have been accepted. Inspect before retrying.') from error
        if not data:
            raise RpcError('daemon disconnected; action outcome may be unknown. Inspect before retrying.')
        return json.loads(data)

    def call(self, method, params):
        self.serial += 1
        request_id = self.serial
        self.send({'id': request_id, 'method': method, 'params': params})
        deadline = time.monotonic() + self.timeout
        if getattr(self, "deadline", None) is not None:
            deadline = min(deadline, self.deadline)
        while True:
            message = self.receive(deadline)
            if 'method' in message:
                # This short-lived client cannot own a durable human interaction.
                # Never silently approve commands or invent answers to user questions.
                if 'id' in message:
                    emit({'pendingRequest': message, 'handled': False}, sys.stderr)
                    self.send({'id': message['id'], 'error': {
                        'code': -32601, 'message': 'Use an interactive Codex client for this request'}})
                continue
            if message.get('id') == request_id:
                if 'error' in message:
                    raise RpcError(json.dumps(message['error']))
                return message['result']


def text_input(args):
    if args.prompt_file:
        text = Path(args.prompt_file).read_text()
    elif args.prompt is not None:
        text = args.prompt
    else:
        if sys.stdin.isatty():
            raise RpcError('supply --prompt, --prompt-file, or text on stdin')
        text = sys.stdin.read()
    if not text.strip():
        raise RpcError('prompt must not be empty')
    return [{'type': 'text', 'text': text}]


def active_turn(client, thread_id):
    page = client.call('thread/turns/list', {
        'threadId': thread_id, 'limit': 1, 'sortDirection': 'desc', 'itemsView': 'notLoaded'})
    turns = page['data']
    if not turns or turns[0]['status'] != 'inProgress':
        raise RpcError('no active turn; use send for a new turn')
    return turns[0]['id']


def snapshot(client, ids):
    result = []
    for thread_id in ids:
        thread = client.call('thread/read', {'threadId': thread_id, 'includeTurns': False})['thread']
        turns = client.call('thread/turns/list', {
            'threadId': thread_id, 'limit': 1, 'sortDirection': 'desc', 'itemsView': 'summary'})['data']
        result.append({'threadId': thread_id, 'status': thread['status'],
                       'latestTurn': turns[0] if turns else None})
    return result


def wait(client, args):
    # Zero means one snapshot, not a zero-budget RPC. Bound that snapshot too.
    deadline = time.monotonic() + (args.seconds or min(client.timeout, 60))
    previous_deadline = getattr(client, 'deadline', None)
    client.deadline = deadline
    state = []
    try:
        while True:
            try:
                state = snapshot(client, args.thread_ids)
            except TimeoutError:
                emit({'ready': False, 'timedOut': True, 'snapshotIncomplete': True,
                      'threads': state})
                return 124
            # A notLoaded/idle thread is not necessarily a successful task.
            ready = all(row['status']['type'] != 'active' or row['status'].get('activeFlags')
                        for row in state)
            if ready or args.seconds == 0 or time.monotonic() >= deadline:
                emit({'ready': bool(ready), 'timedOut': not ready, 'threads': state})
                return 0 if ready else 124
            time.sleep(min(args.poll, max(0, deadline - time.monotonic())))
    finally:
        client.deadline = previous_deadline


def parser():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--socket', help='explicit local daemon control socket; otherwise use daemon version')
    p.add_argument('--rpc-timeout', type=positive, default=20)
    sub = p.add_subparsers(dest='command', required=True)
    sub.add_parser('status', help='CLI/daemon versions and login status; never starts a daemon')
    sub.add_parser('start', help='explicit idempotent daemon start; no login or remote-control setup')
    ls = sub.add_parser('list')
    ls.add_argument('--archived', action='store_true')
    ls.add_argument('--limit', type=int, default=20)
    ls.add_argument('--cursor')
    ls.add_argument('--cwd')
    rd = sub.add_parser('read', help='metadata plus a page of full turns, newest first')
    rd.add_argument('thread_id')
    rd.add_argument('--limit', type=int, default=5)
    rd.add_argument('--cursor')
    rd.add_argument('--summary', action='store_true')
    for command in ('create', 'send', 'steer'):
        cmd = sub.add_parser(command)
        if command != 'create':
            cmd.add_argument('thread_id')
        group = cmd.add_mutually_exclusive_group()
        group.add_argument('--prompt')
        group.add_argument('--prompt-file')
        if command != 'steer':
            cmd.add_argument('--model', help='omit to inherit Codex configuration/thread model')
            cmd.add_argument('--effort', choices=['minimal', 'low', 'medium', 'high', 'xhigh', 'max'])
        if command == 'create':
            cmd.add_argument('-C', '--cwd', required=True)
            cmd.add_argument('--title')
            cmd.add_argument('--sandbox', default='workspace-write',
                             choices=['read-only', 'workspace-write', 'danger-full-access'])
    fork = sub.add_parser('fork', help='copy history; does not start a turn')
    fork.add_argument('thread_id')
    fork.add_argument('--last-turn-id')
    title = sub.add_parser('title')
    title.add_argument('thread_id')
    title.add_argument('title')
    for command in ('archive', 'restore', 'interrupt'):
        cmd = sub.add_parser(command)
        cmd.add_argument('thread_id')
    wt = sub.add_parser('wait', help='bounded status polling; timeout does not interrupt work')
    wt.add_argument('thread_ids', nargs='+')
    wt.add_argument('--seconds', type=window, default=30)
    wt.add_argument('--poll', type=positive, default=2)
    return p


def execute(client, args):
    command = args.command
    if command == 'list':
        params = {'limit': args.limit, 'archived': args.archived, 'sortKey': 'updated_at',
                  'useStateDbOnly': True}
        if args.cursor:
            params['cursor'] = args.cursor
        if args.cwd:
            params['cwd'] = str(Path(args.cwd).resolve())
        emit(client.call('thread/list', params))
    elif command == 'read':
        result = client.call('thread/read', {'threadId': args.thread_id, 'includeTurns': False})
        if not args.summary:
            params = {'threadId': args.thread_id, 'limit': args.limit, 'itemsView': 'full'}
            if args.cursor:
                params['cursor'] = args.cursor
            result['turnsPage'] = client.call('thread/turns/list', params)
        emit(result)
    elif command in ('create', 'send', 'steer'):
        content = text_input(args)  # validate before any mutation
        if command == 'create':
            cwd = Path(args.cwd).resolve()
            if not cwd.is_dir():
                raise RpcError('cwd must be an existing directory')
            params = {'cwd': str(cwd), 'sandbox': args.sandbox, 'approvalPolicy': 'never'}
            if args.model:
                params['model'] = args.model
            result = client.call('thread/start', params)
            thread_id = result['thread']['id']
            emit({'threadId': thread_id, 'phase': 'created'})  # retain ID even if next RPC fails
            if args.title:
                client.call('thread/name/set', {'threadId': thread_id, 'name': args.title})
        else:
            thread_id = args.thread_id
            if command == 'send':
                client.call('thread/resume', {'threadId': thread_id, 'excludeTurns': True})
        params = {'threadId': thread_id, 'input': content}
        if command == 'steer':
            params['expectedTurnId'] = active_turn(client, thread_id)
            method = 'turn/steer'
        else:
            if args.model:
                params['model'] = args.model
            if args.effort:
                params['effort'] = args.effort
            method = 'turn/start'
        emit({'threadId': thread_id, 'accepted': client.call(method, params)})
    elif command == 'fork':
        params = {'threadId': args.thread_id, 'excludeTurns': True}
        if args.last_turn_id:
            params['lastTurnId'] = args.last_turn_id
        emit(client.call('thread/fork', params))
    elif command == 'title':
        emit(client.call('thread/name/set', {'threadId': args.thread_id, 'name': args.title}))
    elif command in ('archive', 'restore'):
        emit(client.call('thread/archive' if command == 'archive' else 'thread/unarchive',
                         {'threadId': args.thread_id}))
    elif command == 'interrupt':
        emit(client.call('turn/interrupt', {'threadId': args.thread_id,
                                           'turnId': active_turn(client, args.thread_id)}))
    elif command == 'wait':
        return wait(client, args)
    return 0


def main():
    args = parser().parse_args()
    if args.command == 'wait' and len(args.thread_ids) > 8:
        raise RpcError('wait accepts at most eight thread IDs')
    if getattr(args, 'limit', 1) < 1:
        raise RpcError('limit must be positive')
    if args.command == 'status':
        emit(codex_json('version'))
        login = subprocess.run(['codex', 'login', 'status'], capture_output=True, text=True, timeout=20)
        emit({'loggedIn': login.returncode == 0, 'message': (login.stdout + login.stderr).strip()})
        return 0
    if args.command == 'start':
        emit(codex_json('start'))
        return 0
    path = args.socket
    if not path:
        info = codex_json('version')
        if info.get('status') != 'running':
            raise RpcError('daemon is not running; use start explicitly')
        path = info['socketPath']
    client = Client(path, args.rpc_timeout)
    try:
        return execute(client, args)
    finally:
        client.close()  # closes only this connection; daemon owns ongoing turns


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (RpcError, OSError, ValueError, subprocess.TimeoutExpired, websocket.WebSocketException) as error:
        emit({'error': str(error)}, sys.stderr)
        sys.exit(1)
    except KeyboardInterrupt:
        sys.exit(130)
