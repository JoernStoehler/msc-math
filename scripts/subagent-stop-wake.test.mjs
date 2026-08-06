import assert from "node:assert/strict";
import { mkdtempSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

import {
  eventId,
  parentThreadId,
  recordEvent,
  stateRoot,
  waitForQuiet,
} from "./subagent-stop-wake.mjs";

const PARENT_ID = "019fd664-6b92-7f41-9834-4d6eaefff155";

function payload() {
  return {
    hook_event_name: "SubagentStop",
    agent_id: "/root/reader",
    agent_type: "explorer",
    agent_transcript_path: null,
    cwd: "/workspaces/msc-math",
    last_assistant_message: "not retained",
    model: "gpt-5.6-luna",
    permission_mode: "bypassPermissions",
    session_id: "019fd664-7b2a-7c21-9d44-3f1112032daa",
    stop_hook_active: false,
    transcript_path: `/tmp/rollout-${PARENT_ID}.jsonl`,
    turn_id: "019fd664-7b93-7c01-b4d8-8b7b603b3991",
  };
}

test("extracts the parent thread id from the parent transcript", () => {
  assert.equal(parentThreadId(payload().transcript_path), PARENT_ID);
  assert.throws(() => parentThreadId("/tmp/not-a-rollout.jsonl"));
});

test("deduplicates one child turn and stores no transcript text", () => {
  const base = mkdtempSync(join(tmpdir(), "subagent-wake-test-"));
  process.env.CODEX_SUBAGENT_WAKE_STATE_DIR = base;
  try {
    const input = payload();
    const first = recordEvent(input);
    const second = recordEvent(input);
    assert.equal(first.created, true);
    assert.equal(second.created, false);
    assert.equal(first.eventId, eventId(input, PARENT_ID));
    const names = readdirSync(join(stateRoot(), PARENT_ID));
    assert.deepEqual(names, [`event-${first.eventId}.json`]);
  } finally {
    delete process.env.CODEX_SUBAGENT_WAKE_STATE_DIR;
    rmSync(base, { recursive: true, force: true });
  }
});

test("quiet-period wait follows the newest sibling event", async () => {
  const base = mkdtempSync(join(tmpdir(), "subagent-wake-test-"));
  process.env.CODEX_SUBAGENT_WAKE_STATE_DIR = base;
  try {
    const first = recordEvent(payload());
    const startedAt = Date.now();
    const waiting = waitForQuiet(first.parentDir, 80);
    await new Promise((resolvePromise) => setTimeout(resolvePromise, 45));
    recordEvent({ ...payload(), agent_id: "/root/second-reader" });
    await waiting;
    assert.ok(Date.now() - startedAt >= 100);
  } finally {
    delete process.env.CODEX_SUBAGENT_WAKE_STATE_DIR;
    rmSync(base, { recursive: true, force: true });
  }
});
