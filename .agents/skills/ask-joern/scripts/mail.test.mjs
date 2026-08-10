import assert from "node:assert/strict";
import test from "node:test";
import { inboxApiUrl, runMail } from "./mail.mjs";

const env = {
  CODEX_THREAD_ID: "thread-123",
  CODEX_GUI_URL: "http://gui.test/",
};

test("send requires an explicit title", async () => {
  await assert.rejects(
    runMail(["send", "--body", "message.md"], {
      env,
      stdout: () => {},
    }),
    /--title/,
  );
});

test("send submits the body and prints the generated ID", async () => {
  const calls = [];
  let output = "";
  await runMail(
    [
      "send",
      "--body",
      "message.md",
      "--title",
      "Review the proof",
    ],
    {
      env,
      readBody: async () => "Please review the proof.",
      fetchImpl: async (url, init) => {
        calls.push({ url, init });
        return Response.json({ id: "generated.md" });
      },
      stdout: (text) => {
        output += text;
      },
    },
  );

  assert.equal(output, "generated.md\n");
  assert.equal(calls[0].url, "http://gui.test/api/inbox");
  assert.deepEqual(JSON.parse(calls[0].init.body), {
    title: "Review the proof",
    codexThreadId: "thread-123",
    body: "Please review the proof.",
  });
});

test("status returns only this thread's handling state", async () => {
  let output = "";
  await runMail(["status", "generated.md"], {
    env,
    fetchImpl: async () =>
      Response.json({
        id: "generated.md",
        codexThreadId: "thread-123",
        status: "in_progress",
      }),
    stdout: (text) => {
      output += text;
    },
  });
  assert.equal(output, "in_progress\n");
});

test("HTTP failures are reported once and return to the caller", async () => {
  let calls = 0;
  await assert.rejects(
    runMail(["list"], {
      env,
      fetchImpl: async () => {
        calls += 1;
        return Response.json(
          { message: "Inbox is unavailable" },
          { status: 503 },
        );
      },
      stdout: () => {},
    }),
    /Inbox delivery failed: Inbox is unavailable/,
  );
  assert.equal(calls, 1);
});

test("the project endpoint is explicit and overridable", () => {
  assert.equal(
    inboxApiUrl({}, "/workspaces/codex-gui"),
    "http://127.0.0.1:5183/api/inbox",
  );
  assert.equal(
    inboxApiUrl({}, "/workspaces/msc-math"),
    "http://codex-gui:5173/api/inbox",
  );
  assert.equal(
    inboxApiUrl({ CODEX_GUI_URL: "http://elsewhere:9999/" }, "/tmp/x"),
    "http://elsewhere:9999/api/inbox",
  );
});
