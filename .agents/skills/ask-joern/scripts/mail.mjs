#!/usr/bin/env node

import { readFile } from "node:fs/promises";
import { basename, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const HELP = `Send a one-way message to Jörn's project inbox.

Usage:
  mail send --body FILE --title TITLE
  mail list
  mail show ID
  mail status ID

The command reads CODEX_THREAD_ID automatically. Replies arrive as ordinary
messages in the owning Codex root thread. Inbox handling status is not a reply.`;

export async function runMail(
  args,
  {
    env = process.env,
    fetchImpl = globalThis.fetch,
    readBody = (path) => readFile(path, "utf8"),
    stdout = (text) => process.stdout.write(text),
  } = {},
) {
  if (args.length === 0 || args[0] === "--help" || args[0] === "-h") {
    stdout(`${HELP}\n`);
    return;
  }

  const command = args[0];
  if (command === "help") {
    stdout(`${HELP}\n`);
    return;
  }

  const threadId = requiredEnvironment(env, "CODEX_THREAD_ID");
  const apiUrl = inboxApiUrl(env);

  if (command === "send") {
    const fields = parseRequiredFlags(args.slice(1), ["body", "title"]);
    const body = await readBody(fields.body);
    if (body.trim() === "") {
      throw new Error(`Body file is empty: ${fields.body}`);
    }
    const created = await requestJson(fetchImpl, apiUrl, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        title: fields.title,
        codexThreadId: threadId,
        body,
      }),
    });
    stdout(`${requiredResponseString(created, "id")}\n`);
    return;
  }

  if (command === "list") {
    requireArgumentCount(args, 1, "mail list");
    const messages = await requestJson(fetchImpl, apiUrl);
    if (!Array.isArray(messages)) {
      throw new Error("Inbox API returned an invalid list");
    }
    stdout(
      `${JSON.stringify(
        messages.filter(
          (message) =>
            plainObject(message) && message.codexThreadId === threadId,
        ),
        null,
        2,
      )}\n`,
    );
    return;
  }

  if (command === "show" || command === "status") {
    requireArgumentCount(args, 2, `mail ${command} ID`);
    const message = await requestJson(
      fetchImpl,
      `${apiUrl}/${encodeURIComponent(args[1])}`,
    );
    if (!plainObject(message) || message.codexThreadId !== threadId) {
      throw new Error("Inbox message does not belong to this Codex thread");
    }
    if (command === "show") {
      stdout(`${JSON.stringify(message, null, 2)}\n`);
    } else {
      stdout(`${requiredResponseString(message, "status")}\n`);
    }
    return;
  }

  throw new Error(`Unknown command: ${command}\n\n${HELP}`);
}

export function inboxApiUrl(env, projectRoot = bundledProjectRoot()) {
  const configured = env.CODEX_GUI_URL?.trim();
  if (configured !== undefined && configured !== "") {
    return `${configured.replace(/\/$/, "")}/api/inbox`;
  }
  switch (basename(projectRoot)) {
    case "codex-gui":
      return "http://127.0.0.1:5183/api/inbox";
    case "msc-math":
      return "http://codex-gui:5173/api/inbox";
    default:
      throw new Error(
        "Cannot locate this project's GUI; set CODEX_GUI_URL",
      );
  }
}

function bundledProjectRoot() {
  return resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
}

function parseRequiredFlags(args, names) {
  const allowed = new Set(names);
  const values = {};
  for (let index = 0; index < args.length; index += 2) {
    const flag = args[index];
    const value = args[index + 1];
    if (!flag?.startsWith("--") || value === undefined) {
      throw new Error("send options must be provided as --name value pairs");
    }
    const name = flag.slice(2);
    if (!allowed.has(name)) {
      throw new Error(`Unknown send option: ${flag}`);
    }
    if (name in values) {
      throw new Error(`Duplicate send option: ${flag}`);
    }
    if (value.trim() === "") {
      throw new Error(`${flag} must not be empty`);
    }
    values[name] = value;
  }
  const missing = names.filter((name) => !(name in values));
  if (missing.length > 0) {
    throw new Error(`Missing required option${missing.length === 1 ? "" : "s"}: ${missing.map((name) => `--${name}`).join(", ")}`);
  }
  return values;
}

function requiredEnvironment(env, name) {
  const value = env[name]?.trim();
  if (value === undefined || value === "") {
    throw new Error(`Missing required environment variable: ${name}`);
  }
  return value;
}

function requireArgumentCount(args, count, usage) {
  if (args.length !== count) {
    throw new Error(`Usage: ${usage}`);
  }
}

async function requestJson(fetchImpl, url, init) {
  let response;
  try {
    response = await fetchImpl(url, init);
  } catch (error) {
    throw new Error(
      `Inbox delivery failed: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
  const body = await response.json().catch(() => null);
  if (!response.ok) {
    const detail = plainObject(body) && typeof body.message === "string"
      ? body.message
      : `HTTP ${response.status}`;
    throw new Error(`Inbox delivery failed: ${detail}`);
  }
  return body;
}

function requiredResponseString(response, field) {
  if (!plainObject(response) || typeof response[field] !== "string") {
    throw new Error(`Inbox API response is missing ${field}`);
  }
  return response[field];
}

function plainObject(value) {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  runMail(process.argv.slice(2)).catch((error) => {
    process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`);
    process.exitCode = 1;
  });
}
