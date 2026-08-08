#!/usr/bin/env node

/**
 * Wake an idle Codex parent after a thread-spawned child stops.
 *
 * SubagentStop runs before Codex delivers the child's existing completion
 * message to the parent mailbox. The synchronous hook therefore records a
 * metadata-only event, starts this file again as a detached relay, and returns
 * `{}` immediately. The relay gives delivery and sibling completions a quiet
 * grace period, checks that the parent is idle, then starts one empty-input
 * turn. Codex drains its mailbox before sampling.
 *
 * The first probe used a fixed 300 ms sleep and one relay per hook. That proved
 * the app-server route but was not production-safe: duplicate hook sources
 * started duplicate turns, and the fixed delay did not coalesce siblings.
 * This version uses an exact child-turn event key, a per-parent leader lock,
 * a quiet-period grace, and one wake for all events captured in the batch.
 *
 * Accepted policy and sharp edges:
 * - An idle parent is considered wakeable even if its previous message looked
 *   terminal. A redundant wake is acceptable because the parent can inspect
 *   the completion, decide it is irrelevant, and end the turn again.
 * - The grace is burst coalescing, not an "all descendants finished" barrier.
 *   Completions farther apart can intentionally produce separate paid turns.
 * - Every completion starts a short-lived detached relay candidate. The
 *   per-parent lock permits only one to act; losing candidates exit immediately.
 * - Delivery is intentionally best-effort. If app-server is unavailable, the
 *   parent is active, or a race/error occurs, that settled batch is logged and
 *   discarded instead of keeping background retries alive.
 * - Empty-input `turn/start` is verified Codex 0.146 behavior, not assumed to
 *   be a permanent protocol guarantee. Re-run the integration probe after a
 *   Codex/app-server upgrade.
 *
 * Configuration:
 *   CODEX_APP_SERVER_URL              default ws://127.0.0.1:4500
 *   CODEX_SUBAGENT_WAKE_GRACE_MS      default 5000
 *   CODEX_SUBAGENT_WAKE_STATE_DIR     default OS temp directory
 *   app-server capability token       /workspaces/msc-math/.app-server-token
 */

import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import {
  appendFileSync,
  mkdirSync,
  openSync,
  closeSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const DEFAULT_APP_SERVER_URL = "ws://127.0.0.1:4500";
const DEFAULT_APP_SERVER_TOKEN_FILE = "/workspaces/msc-math/.app-server-token";
const DEFAULT_GRACE_MS = 5_000;
const RPC_TIMEOUT_MS = 10_000;
const STALE_LOCK_MS = 60_000;
const MAX_LOG_BYTES = 512 * 1024;
const UUID_AT_END_RE =
  /([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\.jsonl$/i;

function positiveIntegerEnv(name, fallback) {
  const raw = process.env[name];
  if (raw === undefined || raw === "") return fallback;
  const value = Number(raw);
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }
  return value;
}

function stateRoot() {
  const configured = process.env.CODEX_SUBAGENT_WAKE_STATE_DIR;
  const base = configured ? resolve(configured) : tmpdir();
  const uid = typeof process.getuid === "function" ? process.getuid() : "unknown";
  return join(base, `codex-subagent-wake-${uid}`);
}

function parentThreadId(transcriptPath) {
  if (typeof transcriptPath !== "string") {
    throw new Error("SubagentStop transcript_path is unavailable");
  }
  const match = UUID_AT_END_RE.exec(transcriptPath);
  if (!match) {
    throw new Error(`cannot recover parent thread id from ${transcriptPath}`);
  }
  return match[1].toLowerCase();
}

function eventId(input, parentId) {
  return createHash("sha256")
    .update(`${parentId}\0${input.agent_id}\0${input.turn_id}`)
    .digest("hex");
}

function eventFiles(parentDir) {
  try {
    return readdirSync(parentDir)
      .filter((name) => name.startsWith("event-") && name.endsWith(".json"))
      .sort();
  } catch (error) {
    if (error?.code === "ENOENT") return [];
    throw error;
  }
}

function safeAppendLog(parentDir, entry) {
  try {
    mkdirSync(parentDir, { recursive: true, mode: 0o700 });
    const path = join(parentDir, "relay.log.jsonl");
    try {
      if (statSync(path).size >= MAX_LOG_BYTES) {
        writeFileSync(path, "", { mode: 0o600 });
      }
    } catch (error) {
      if (error?.code !== "ENOENT") throw error;
    }
    appendFileSync(
      path,
      `${JSON.stringify({ at: new Date().toISOString(), ...entry })}\n`,
      { mode: 0o600 },
    );
  } catch {
    // The hook is fail-open. Logging must never block child completion.
  }
}

async function readStdinJson() {
  process.stdin.setEncoding("utf8");
  let body = "";
  for await (const chunk of process.stdin) body += chunk;
  return JSON.parse(body);
}

function validateHookInput(input) {
  if (!input || input.hook_event_name !== "SubagentStop") {
    throw new Error("expected a SubagentStop hook payload");
  }
  for (const field of ["agent_id", "turn_id"]) {
    if (typeof input[field] !== "string" || input[field] === "") {
      throw new Error(`SubagentStop ${field} is unavailable`);
    }
  }
}

function recordEvent(input) {
  validateHookInput(input);
  const parentId = parentThreadId(input.transcript_path);
  const parentDir = join(stateRoot(), parentId);
  mkdirSync(parentDir, { recursive: true, mode: 0o700 });
  const id = eventId(input, parentId);
  const path = join(parentDir, `event-${id}.json`);
  let created = false;
  try {
    const descriptor = openSync(path, "wx", 0o600);
    try {
      writeFileSync(
        descriptor,
        JSON.stringify({
          schemaVersion: 1,
          eventId: id,
          parentThreadId: parentId,
          childThreadId: input.session_id,
          agentId: input.agent_id,
          agentType: input.agent_type,
          childTurnId: input.turn_id,
          receivedAt: new Date().toISOString(),
        }),
      );
      created = true;
    } finally {
      closeSync(descriptor);
    }
  } catch (error) {
    if (error?.code !== "EEXIST") throw error;
  }
  return { parentId, parentDir, eventId: id, created };
}

function spawnRelay(parentId) {
  // Spawning before lock acquisition keeps the synchronous hook tiny and
  // fail-open. Losing candidates exit immediately after the one lock attempt.
  const child = spawn(process.execPath, [fileURLToPath(import.meta.url), "relay", parentId], {
    detached: true,
    stdio: "ignore",
    env: process.env,
  });
  child.unref();
}

async function hookMain() {
  try {
    const input = await readStdinJson();
    const recorded = recordEvent(input);
    if (recorded.created) {
      spawnRelay(recorded.parentId);
      safeAppendLog(recorded.parentDir, {
        event: "queued",
        eventId: recorded.eventId,
        agentId: input.agent_id,
        childTurnId: input.turn_id,
      });
    }
  } catch (error) {
    safeAppendLog(join(stateRoot(), "invalid"), {
      event: "hook-error",
      error: error instanceof Error ? error.message : String(error),
    });
  }
  // SubagentStop requires JSON. Do not add a system message to child context.
  process.stdout.write("{}\n");
}

function sleep(ms) {
  return new Promise((resolvePromise) => setTimeout(resolvePromise, ms));
}

function acquireLeader(parentDir) {
  const lockDir = join(parentDir, "leader.lock");
  try {
    mkdirSync(lockDir, { mode: 0o700 });
  } catch (error) {
    if (error?.code !== "EEXIST") throw error;
    if (!staleLeader(lockDir)) return null;
    rmSync(lockDir, { recursive: true, force: true });
    try {
      mkdirSync(lockDir, { mode: 0o700 });
    } catch (retryError) {
      if (retryError?.code === "EEXIST") return null;
      throw retryError;
    }
  }
  writeFileSync(
    join(lockDir, "owner.json"),
    JSON.stringify({ pid: process.pid, acquiredAt: new Date().toISOString() }),
    { mode: 0o600 },
  );
  return lockDir;
}

function staleLeader(lockDir) {
  try {
    if (Date.now() - statSync(lockDir).mtimeMs > STALE_LOCK_MS) return true;
    const owner = JSON.parse(readFileSync(join(lockDir, "owner.json"), "utf8"));
    if (!Number.isSafeInteger(owner.pid) || owner.pid <= 0) return false;
    try {
      process.kill(owner.pid, 0);
      return false;
    } catch (error) {
      return error?.code === "ESRCH";
    }
  } catch {
    // A leader can briefly exist before owner.json is written. Age, rather
    // than a missing/partial owner file, is the safe crash-recovery signal.
    return false;
  }
}

function releaseLeader(lockDir) {
  rmSync(lockDir, { recursive: true, force: true });
}

function newestEventMtime(parentDir) {
  return Math.max(
    0,
    ...eventFiles(parentDir).map((name) => statSync(join(parentDir, name)).mtimeMs),
  );
}

function settledEventFiles(parentDir, graceMs) {
  const cutoff = Date.now() - graceMs;
  return eventFiles(parentDir).filter(
    (name) => statSync(join(parentDir, name)).mtimeMs <= cutoff,
  );
}

async function waitForQuiet(parentDir, graceMs) {
  while (true) {
    const newest = newestEventMtime(parentDir);
    if (newest === 0) return;
    const remaining = graceMs - (Date.now() - newest);
    if (remaining <= 0) return;
    await sleep(remaining);
  }
}

function appServerWebSocket(url, capabilityToken) {
  return new WebSocket(url, {
    headers: { Authorization: `Bearer ${capabilityToken}` },
  });
}

class AppServerClient {
  constructor(url) {
    this.url = url;
    this.socket = null;
    this.nextId = 1;
    this.pending = new Map();
  }

  async connect() {
    const capabilityToken = readFileSync(DEFAULT_APP_SERVER_TOKEN_FILE, "utf8").trim();
    const socket = appServerWebSocket(this.url, capabilityToken);
    this.socket = socket;
    socket.addEventListener("message", (event) => this.#onMessage(event.data));
    await new Promise((resolvePromise, reject) => {
      const timeout = setTimeout(
        () => reject(new Error("app-server websocket open timed out")),
        RPC_TIMEOUT_MS,
      );
      socket.addEventListener(
        "open",
        () => {
          clearTimeout(timeout);
          resolvePromise();
        },
        { once: true },
      );
      socket.addEventListener(
        "error",
        () => {
          clearTimeout(timeout);
          reject(new Error("app-server websocket failed"));
        },
        { once: true },
      );
    });
    await this.request("initialize", {
      clientInfo: {
        name: "joern_subagent_wake_relay",
        title: "Jorn Subagent Wake Relay",
        version: "1.0.0",
      },
      capabilities: {
        experimentalApi: false,
        requestAttestation: false,
      },
    });
    socket.send(JSON.stringify({ method: "initialized" }));
  }

  #onMessage(raw) {
    let message;
    try {
      message = JSON.parse(typeof raw === "string" ? raw : Buffer.from(raw).toString());
    } catch {
      return;
    }
    if (typeof message.id !== "number") return;
    const waiter = this.pending.get(message.id);
    if (!waiter) return;
    this.pending.delete(message.id);
    clearTimeout(waiter.timeout);
    if (message.error) {
      waiter.reject(new Error(JSON.stringify(message.error)));
    } else {
      waiter.resolve(message.result);
    }
  }

  request(method, params) {
    if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
      return Promise.reject(new Error("app-server websocket is not open"));
    }
    const id = this.nextId++;
    return new Promise((resolvePromise, reject) => {
      const timeout = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error(`${method} timed out`));
      }, RPC_TIMEOUT_MS);
      this.pending.set(id, { resolve: resolvePromise, reject, timeout });
      this.socket.send(JSON.stringify({ method, id, params }));
    });
  }

  close() {
    this.socket?.close();
    for (const waiter of this.pending.values()) {
      clearTimeout(waiter.timeout);
      waiter.reject(new Error("app-server websocket closed"));
    }
    this.pending.clear();
  }
}

function consumeEvents(parentDir, names) {
  for (const name of names) {
    try {
      unlinkSync(join(parentDir, name));
    } catch (error) {
      if (error?.code !== "ENOENT") throw error;
    }
  }
}

async function processOneBatch(parentId, parentDir) {
  const graceMs = positiveIntegerEnv(
    "CODEX_SUBAGENT_WAKE_GRACE_MS",
    DEFAULT_GRACE_MS,
  );
  const url = process.env.CODEX_APP_SERVER_URL ?? DEFAULT_APP_SERVER_URL;

  await waitForQuiet(parentDir, graceMs);
  // Require at least one event to receive the full mailbox-delivery grace
  // before starting a wake. A sibling arriving after this quiet check may be
  // folded into the same wake below under the accepted best-effort policy.
  const captured = settledEventFiles(parentDir, graceMs);
  if (captured.length === 0) return;

  const client = new AppServerClient(url);
  try {
    await client.connect();
    const read = await client.request("thread/read", {
      threadId: parentId,
      includeTurns: false,
    });
    const status = read?.thread?.status;
    if (status?.type !== "idle") {
      safeAppendLog(parentDir, {
        event: "wake-skipped",
        reason: "parent-not-idle",
        parentStatus: status ?? null,
        eventCount: captured.length,
      });
      return;
    }
    const result = await client.request("turn/start", {
      threadId: parentId,
      input: [],
    });
    // The empty turn is deliberate: Codex drains completion mail before the
    // next sample. Consume every event already queued, including a very recent
    // sibling that missed the settled snapshot: that sibling's mail can join
    // this turn, and a second paid wake would be worse than the accepted small
    // risk that the hook preceded mailbox delivery by a few milliseconds.
    const coalesced = eventFiles(parentDir);
    consumeEvents(parentDir, coalesced);
    safeAppendLog(parentDir, {
      event: "wake-started",
      eventCount: coalesced.length,
      turnId: result?.turn?.id ?? null,
    });
  } catch (error) {
    safeAppendLog(parentDir, {
      event: "wake-skipped",
      reason: "relay-error",
      eventCount: captured.length,
      error: error instanceof Error ? error.message : String(error),
    });
  } finally {
    client.close();
    // Best-effort policy: do not turn a missed wake into a retained retry
    // queue. New completions remain eligible for their own independent batch.
    consumeEvents(parentDir, captured);
  }
}

async function relayMain(parentId) {
  if (!UUID_AT_END_RE.test(`${parentId}.jsonl`)) {
    throw new Error(`invalid parent thread id: ${parentId}`);
  }
  const parentDir = join(stateRoot(), parentId.toLowerCase());
  const lockDir = acquireLeader(parentDir);
  if (!lockDir) return;
  try {
    // Process events that arrived before or while this leader held the lock.
    // A completion in the tiny final-check/release race may be missed; this is
    // accepted by the explicitly best-effort wake policy.
    while (eventFiles(parentDir).length > 0) {
      await processOneBatch(parentId, parentDir);
    }
  } catch (error) {
    safeAppendLog(parentDir, {
      event: "relay-error",
      error: error instanceof Error ? error.message : String(error),
    });
  } finally {
    releaseLeader(lockDir);
  }
}

export {
  appServerWebSocket,
  eventId,
  parentThreadId,
  recordEvent,
  stateRoot,
  waitForQuiet,
};

const isMain =
  process.argv[1] !== undefined &&
  resolve(process.argv[1]) === resolve(fileURLToPath(import.meta.url));

if (isMain) {
  const [mode, parentId] = process.argv.slice(2);
  if (mode === "hook") {
    await hookMain();
  } else if (mode === "relay" && parentId) {
    await relayMain(parentId);
  } else {
    process.stderr.write(
      `usage: node ${fileURLToPath(import.meta.url)} hook|relay PARENT_THREAD_ID\n`,
    );
    process.exitCode = 2;
  }
}
