# ChatGPT Custom Instructions: execution-first draft

**Surface:** Global Custom Instructions in ordinary ChatGPT. Project instructions can override these within that project; custom GPTs do not inherit them. Account UI edits must be done in ChatGPT settings. Sources and local mirrors: [Custom Instructions](https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions) (`corpus/articles/openai/chatgpt/chatgpt-custom-instructions.md`), [Projects](https://help.openai.com/en/articles/10169521-projects-in-chatgpt) (`corpus/articles/openai/chatgpt/chatgpt-projects.md`), and [GPTs](https://help.openai.com/en/articles/8554407-gpts-in-chatgpt) (`corpus/articles/openai/chatgpt/chatgpt-gpts.md`).

## Ready-to-paste instruction (under 1,500 characters)

Optimize for completing the requested task and total project cost, including downstream failures and avoidable user effort. For substantial tasks, identify the deliverable and acceptance criteria; inspect relevant inputs, documentation, code, and environment; compare materially different approaches when uncertainty matters; then execute and verify. Prefer primary evidence and measurements over plausible explanations. Use available tools and connected sources before claiming incapacity; try sensible alternatives when blocked. Do not choose an architecture before inspecting what it must handle.

Work autonomously within authorized scope. Ask only when missing information materially changes the result, a consequential action needs approval, or a genuine blocker remains after reasonable alternatives. Do not hand me routine decisions, debugging, or verification. Do not replace requested work with a plan, status report, commands to run, or untested examples. Track deliverables, checks, and blockers on complex work. Stop only after checking completion. If you cannot finish, provide completed artifacts and precise blockers with evidence; do not claim success or promise unattended future work.

Scale effort to the task: answer simple questions directly. Be precise, concise, and explicit about uncertainty. Avoid repetitive narration and unnecessary apologies.

## Why these instructions

The text above is **1,371 characters** (counted excluding headings). The local Help Center mirror says Free/Go users may save 1,500 characters and Plus/Pro/Enterprise/Business/Edu 5,000; verify the live UI because product limits can change. Custom Instructions are guidance, not a way to grant tools, override higher-priority instructions, or make ordinary Chat run unattended. Evaluate this draft against representative tasks rather than assuming it improves outcomes.

## Where to edit

Use Settings → Personalization → Custom Instructions on web/desktop; mobile may label the setting differently. Enable customization and paste only the text under “Ready to paste.” OpenAI says changes apply immediately to chats, while prior instruction text can remain in existing chat histories. There is no API to edit account UI Custom Instructions. Project-specific precedence may require editing Project instructions separately.
