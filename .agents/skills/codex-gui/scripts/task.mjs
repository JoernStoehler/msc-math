#!/usr/bin/env node

import { randomUUID } from "node:crypto";
import { readFile } from "node:fs/promises";
import { basename, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const HELP = `Read and publish durable Codex tasks.

Usage:
  task [--project PROJECT] list
  task [--project PROJECT] show TASK_REF
  task [--project PROJECT] comments TASK_REF [--after COMMENT_OR_REVISION_REF | --since RFC3339]
  task [--project PROJECT] create --file FILE
  task [--project PROJECT] revise TASK_REF --file FILE --base-revision REVISION_ID
  task [--project PROJECT] comment TASK_REF --file FILE
  task [--project PROJECT] revise-comment COMMENT_REF --file FILE --base-revision REVISION_ID
  task [--project PROJECT] close TASK_REF
  task [--project PROJECT] reopen TASK_REF

All text is read once from --file and sent as JSON. Collections are JSONL; all
other successful commands print one JSON object. Mutations take provenance from
CODEX_THREAD_ID. The project defaults to the repository containing this skill;
--project overrides it. CODEX_GUI_URL defaults to http://codex-gui:5173.`;

const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const INSTALLED_PROJECT = basename(resolve(dirname(fileURLToPath(import.meta.url)), "../../../.."));

export async function runTask(args, dependencies = {}) {
  const {
    cwd = () => process.cwd(), defaultProject = () => INSTALLED_PROJECT,
    env = process.env, fetchImpl = globalThis.fetch,
    idempotencyKey = randomUUID, readText = (path) => readFile(path, "utf8"),
    stdout = (text) => process.stdout.write(text),
  } = dependencies;
  if (args.length === 0 || ["help", "--help", "-h"].includes(args[0])) {
    stdout(`${HELP}\n`);
    return;
  }
  let project = nonempty(defaultProject());
  let commandIndex = 0;
  if (args[0] === "--project") {
    project = nonempty(args[1]);
    if (project === undefined) throw new Error("--project requires a project name");
    commandIndex = 2;
  }
  if (project === undefined) throw new Error("Could not determine the installed project; use --project PROJECT");
  const command = args[commandIndex];
  if (command === undefined) throw new Error("Missing task command");
  if (["help", "--help", "-h"].includes(command)) {
    stdout(`${HELP}\n`);
    return;
  }
  const rest = args.slice(commandIndex + 1);
  const workingDirectory = cwd();
  const tokenPath = nonempty(env.CODEX_GUI_TOKEN_FILE) ??
    nonempty(env.CODEX_APP_SERVER_TOKEN_FILE) ?? resolve(workingDirectory, ".app-server-token");
  const token = (await readText(tokenPath)).trim();
  if (token === "") throw new Error(`Task API token file is empty: ${tokenPath}`);
  const guiUrl = (nonempty(env.CODEX_GUI_URL) ?? "http://codex-gui:5173").replace(/\/$/, "");
  const apiUrl = `${guiUrl}/api/tasks`;
  const projectQuery = `project=${encodeURIComponent(project)}`;

  if (command === "list") {
    requireArgumentCount(rest, 0, "task [--project PROJECT] list");
    const response = await requestJson(fetchImpl, `${apiUrl}?${projectQuery}`, token);
    if (!plainObject(response) || !Array.isArray(response.data)) throw new Error("Task API response is missing data");
    for (const task of response.data) stdout(`${JSON.stringify(task)}\n`);
    return;
  }

  if (command === "show") {
    requireArgumentCount(rest, 1, "task [--project PROJECT] show TASK_REF");
    writeJson(stdout, await requestJson(fetchImpl, `${apiUrl}/${encodeURIComponent(taskIdFromReference(rest[0]))}?${projectQuery}`, token));
    return;
  }

  if (command === "comments") {
    if (rest[0] === undefined || rest[0].startsWith("--")) throw new Error("Usage: task [--project PROJECT] comments TASK_REF [--after REF | --since RFC3339]");
    const flags = parseFlags(rest.slice(1), ["after", "since"]);
    if (flags.after !== undefined && flags.since !== undefined) throw new Error("Use either --after or --since, not both");
    if (flags.since !== undefined && !Number.isFinite(Date.parse(flags.since))) throw new Error("--since must be an RFC3339 timestamp");
    const boundary = flags.after === undefined ? (flags.since === undefined ? "" : `&since=${encodeURIComponent(flags.since)}`) : `&after=${encodeURIComponent(idFromReference(flags.after, ["comment", "revision"], "--after"))}`;
    const response = await requestJson(fetchImpl, `${apiUrl}/${encodeURIComponent(taskIdFromReference(rest[0]))}/comments?${projectQuery}${boundary}`, token);
    if (!plainObject(response) || !Array.isArray(response.data)) throw new Error("Task API response is missing comment data");
    for (const comment of response.data) stdout(`${JSON.stringify(comment)}\n`);
    return;
  }

  const sourceThreadId = requiredEnvironment(env, "CODEX_THREAD_ID");
  const writeFlags = (values, names = []) => parseFlags(values, ["file", "base-revision", "idempotency-key", ...names]);
  const retry = (flags) => flags["idempotency-key"] ?? idempotencyKey();

  if (command === "create") {
    const flags = writeFlags(rest);
    const body = await readBody(readText, requiredFlag(flags, "file"), true);
    writeJson(stdout, await requestJson(fetchImpl, apiUrl, token, {
      method: "POST", body: JSON.stringify({ project, body, sourceThreadId, idempotencyKey: retry(flags) }),
    }));
    return;
  }

  if (["revise", "comment"].includes(command)) {
    if (rest[0] === undefined || rest[0].startsWith("--")) throw new Error(`Usage: task [--project PROJECT] ${command} TASK_REF --file FILE`);
    const flags = writeFlags(rest.slice(1));
    const body = await readBody(readText, requiredFlag(flags, "file"), command === "revise");
    const base = command === "revise" ? { baseRevisionId: requiredFlag(flags, "base-revision") } : {};
    const suffix = command === "revise" ? "revisions" : "comments";
    writeJson(stdout, await requestJson(fetchImpl, `${apiUrl}/${encodeURIComponent(taskIdFromReference(rest[0]))}/${suffix}`, token, {
      method: "POST", body: JSON.stringify({ project, body, sourceThreadId, ...base, idempotencyKey: retry(flags) }),
    }));
    return;
  }

  if (command === "revise-comment") {
    if (rest[0] === undefined || rest[0].startsWith("--")) throw new Error("Usage: task [--project PROJECT] revise-comment COMMENT_REF --file FILE --base-revision REVISION_ID");
    const commentId = idFromReference(rest[0], ["comment"], "COMMENT_REF");
    const flags = writeFlags(rest.slice(1));
    const body = await readBody(readText, requiredFlag(flags, "file"), false);
    writeJson(stdout, await requestJson(fetchImpl, `${guiUrl}/api/task-comments/${encodeURIComponent(commentId)}/revisions`, token, {
      method: "POST", body: JSON.stringify({ project, body, sourceThreadId, baseRevisionId: requiredFlag(flags, "base-revision"), idempotencyKey: retry(flags) }),
    }));
    return;
  }

  if (command === "close" || command === "reopen") {
    if (rest[0] === undefined || rest[0].startsWith("--")) throw new Error(`Usage: task [--project PROJECT] ${command} TASK_REF`);
    const flags = parseFlags(rest.slice(1), ["idempotency-key"]);
    writeJson(stdout, await requestJson(fetchImpl, `${apiUrl}/${encodeURIComponent(taskIdFromReference(rest[0]))}/${command}`, token, {
      method: "POST", body: JSON.stringify({ project, sourceThreadId, idempotencyKey: retry(flags) }),
    }));
    return;
  }

  throw new Error(`Unknown command: ${command}\n\n${HELP}`);
}

async function readBody(readText, path, brief) {
  const body = await readText(path);
  if (body.trim() === "") throw new Error(`Task file is empty: ${path}`);
  if (brief) {
    const first = body.split(/\r?\n/).find((line) => line.trim() !== "")?.trim();
    if (first === undefined || !/^#\s+\S/.test(first)) throw new Error("Task brief must start with a level-one Markdown heading (# Title)");
  }
  return body;
}

function parseFlags(args, allowedNames) {
  const allowed = new Set(allowedNames);
  const flags = {};
  for (let index = 0; index < args.length; index += 2) {
    const flag = args[index];
    const value = args[index + 1];
    if (!flag?.startsWith("--") || value === undefined) throw new Error("Options must be provided as --name value pairs");
    const name = flag.slice(2);
    if (!allowed.has(name)) throw new Error(`Unknown option: ${flag}`);
    if (name in flags) throw new Error(`Duplicate option: ${flag}`);
    if (value.trim() === "") throw new Error(`${flag} must not be empty`);
    flags[name] = value;
  }
  return flags;
}

function requiredFlag(flags, name) {
  const value = flags[name];
  if (value === undefined) throw new Error(`Missing required option: --${name}`);
  return value;
}
function requiredEnvironment(env, name) {
  const value = nonempty(env[name]);
  if (value === undefined) throw new Error(`Missing required environment variable: ${name}`);
  return value;
}
function nonempty(value) {
  const trimmed = value?.trim();
  return trimmed === undefined || trimmed === "" ? undefined : trimmed;
}
function requireArgumentCount(args, count, usage) {
  if (args.length !== count) throw new Error(`Usage: ${usage}`);
}
function writeJson(stdout, value) { stdout(`${JSON.stringify(value)}\n`); }

async function requestJson(fetchImpl, url, token, init = {}) {
  let response;
  try {
    response = await fetchImpl(url, {
      ...init,
      headers: { authorization: `Bearer ${token}`, ...(init.body === undefined ? {} : { "content-type": "application/json" }) },
    });
  } catch (error) {
    throw new Error(`Task request failed: ${error instanceof Error ? error.message : String(error)}`);
  }
  const body = await response.json().catch(() => null);
  if (!response.ok) {
    const detail = plainObject(body) && typeof body.message === "string" ? body.message : `HTTP ${response.status}`;
    throw new Error(`Task request failed: ${detail}`);
  }
  return body;
}

function taskIdFromReference(reference) { return idFromReference(reference, ["task"], "TASK_REF"); }
function idFromReference(reference, parameters, label) {
  if (UUID.test(reference)) return reference;
  let url;
  try { url = new URL(reference); } catch { throw new Error(`${label} must be a UUID or copied task link`); }
  for (const parameter of parameters) {
    const value = url.searchParams.get(parameter);
    if (value !== null && UUID.test(value)) return value;
  }
  throw new Error(`${label} must be a UUID or copied task link`);
}
function plainObject(value) { return typeof value === "object" && value !== null && !Array.isArray(value); }

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  runTask(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
    process.exitCode = 1;
  });
}
