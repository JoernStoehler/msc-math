#!/usr/bin/env python3
"""Offline regression checks: no credentials, daemon, network or inference."""
import argparse
import importlib.util
import io
import json
from pathlib import Path
import unittest
from unittest.mock import Mock, patch

spec = importlib.util.spec_from_file_location('client', Path(__file__).with_name('codex-session.py'))
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


class ProtocolTests(unittest.TestCase):
    def client(self, messages):
        client = m.Client.__new__(m.Client)
        client.timeout = 1
        client.serial = 0
        client.sock = Mock()
        client.sock.recv.side_effect = [json.dumps(x) for x in messages]
        return client

    def test_notifications_and_unrelated_ids_do_not_complete_rpc(self):
        c = self.client([{'method': 'item/agentMessage/delta', 'params': {'delta': 'x'}},
                         {'id': 999, 'result': {}}, {'id': 1, 'result': {'ok': True}}])
        self.assertEqual(c.call('thread/read', {}), {'ok': True})
        self.assertEqual(c.sock.send.call_count, 1)

    def test_server_request_is_reported_and_never_approved(self):
        c = self.client([{'id': 'approval-7', 'method': 'item/commandExecution/requestApproval'},
                         {'id': 1, 'result': {}}])
        with patch.object(m, 'emit') as output:
            c.call('thread/read', {})
        self.assertFalse(output.call_args.args[0]['handled'])
        response = json.loads(c.sock.send.call_args.args[0])
        self.assertEqual(response['id'], 'approval-7')
        self.assertIn('error', response)
        self.assertNotIn('result', response)

    def test_poll_deadline_caps_rpc_timeout(self):
        c = self.client([{'id': 1, 'result': {}}])
        c.deadline = m.time.monotonic() + 0.05
        c.call('thread/read', {})
        self.assertLessEqual(c.sock.settimeout.call_args.args[0], 0.05)

    def test_error_and_disconnect_are_not_retried(self):
        c = self.client([{'id': 1, 'error': {'code': -1, 'message': 'failure'}}])
        with self.assertRaises(m.RpcError):
            c.call('turn/start', {})
        self.assertEqual(c.sock.send.call_count, 1)
        c = self.client([])
        c.sock.recv.side_effect = None
        c.sock.recv.return_value = ''
        with self.assertRaisesRegex(m.RpcError, 'unknown'):
            c.call('thread/start', {})
        self.assertEqual(c.sock.send.call_count, 1)

    def test_timeout_is_unknown_outcome_and_no_retry(self):
        c = self.client([])
        c.sock.recv.side_effect = m.websocket.WebSocketTimeoutException('timed out')
        with self.assertRaisesRegex(TimeoutError, 'already have been accepted'):
            c.call('turn/start', {})
        self.assertEqual(c.sock.send.call_count, 1)


class WorkflowTests(unittest.TestCase):
    def test_create_preserves_id_if_turn_start_fails(self):
        c = Mock(timeout=20, deadline=None)
        c.call.side_effect = [{'thread': {'id': 'saved-id'}}, m.RpcError('turn failed')]
        args = m.parser().parse_args(['create', '-C', '/tmp', '--prompt', 'bounded task'])
        with patch.object(m, 'emit') as output, self.assertRaises(m.RpcError):
            m.execute(c, args)
        self.assertEqual(output.call_args.args[0]['threadId'], 'saved-id')
        self.assertEqual(c.call.call_args_list[0].args[1]['approvalPolicy'], 'never')
        self.assertEqual(c.call.call_args_list[0].args[1]['sandbox'], 'workspace-write')

    def test_empty_prompt_does_not_mutate(self):
        c = Mock(timeout=20, deadline=None)
        args = m.parser().parse_args(['create', '-C', '/tmp', '--prompt', '  '])
        with self.assertRaises(m.RpcError):
            m.execute(c, args)
        c.call.assert_not_called()

    def test_model_is_not_silently_overridden_on_followup(self):
        c = Mock(timeout=20, deadline=None)
        c.call.return_value = {}
        args = m.parser().parse_args(['send', 'id', '--prompt', 'next task'])
        with patch.object(m, 'emit'):
            m.execute(c, args)
        self.assertEqual([x.args[0] for x in c.call.call_args_list], ['thread/resume', 'turn/start'])
        for call in c.call.call_args_list:
            self.assertNotIn('model', call.args[1])
            self.assertNotIn('approvalPolicy', call.args[1])

    def test_steer_uses_active_turn_precondition(self):
        c = Mock(timeout=20, deadline=None)
        c.call.side_effect = [{'data': [{'id': 'active-id', 'status': 'inProgress'}]}, {'turnId': 'active-id'}]
        args = m.parser().parse_args(['steer', 'thread', '--prompt', 'correction'])
        with patch.object(m, 'emit'):
            m.execute(c, args)
        self.assertEqual(c.call.call_args.args[1]['expectedTurnId'], 'active-id')

    def test_wait_timeout_never_interrupts(self):
        c = Mock(timeout=20, deadline=None)
        c.call.side_effect = [{'thread': {'status': {'type': 'active', 'activeFlags': []}}},
                              {'data': [{'id': 'turn', 'status': 'inProgress'}]}]
        args = argparse.Namespace(seconds=0, thread_ids=['thread'], poll=2)
        with patch.object(m, 'emit') as output:
            self.assertEqual(m.wait(c, args), 124)
        self.assertTrue(output.call_args.args[0]['timedOut'])
        self.assertNotIn('turn/interrupt', [x.args[0] for x in c.call.call_args_list])

    def test_incomplete_snapshot_is_reported_and_deadline_restored(self):
        c = Mock(timeout=20, deadline=None)
        c.call.side_effect = TimeoutError('read timed out')
        with patch.object(m, 'emit') as output:
            self.assertEqual(m.wait(c, argparse.Namespace(seconds=1, thread_ids=['t'], poll=2)), 124)
        self.assertTrue(output.call_args.args[0]['snapshotIncomplete'])
        self.assertIsNone(c.deadline)

    def test_failed_turn_and_pending_input_remain_visible(self):
        for status, turn in [({'type': 'idle'}, {'status': 'failed', 'error': 'failure'}),
                             ({'type': 'active', 'activeFlags': ['waitingOnUserInput']}, {'status': 'inProgress'})]:
            c = Mock(timeout=20, deadline=None)
            c.call.side_effect = [{'thread': {'status': status}}, {'data': [turn]}]
            with patch.object(m, 'emit') as output:
                self.assertEqual(m.wait(c, argparse.Namespace(seconds=0, thread_ids=['t'], poll=2)), 0)
            row = output.call_args.args[0]['threads'][0]
            self.assertEqual(row['latestTurn'], turn)
            self.assertEqual(row['status'], status)

    def test_timeout_parser_rejects_nan_zero_and_infinity(self):
        for value in ['0', '-1', 'nan', 'inf']:
            with self.assertRaises(argparse.ArgumentTypeError):
                m.positive(value)
        for value in ['-1', 'nan', 'inf', '61']:
            with self.assertRaises(argparse.ArgumentTypeError):
                m.window(value)


if __name__ == '__main__':
    unittest.main()
