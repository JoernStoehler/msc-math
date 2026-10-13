# ChatGPT chat: operational facts and traps

**Reviewed:** 2026-10-13 against local mirrors; vendor docs describe product behavior, not an observation of this account or runtime. Distinguish ordinary Chat, Projects, custom GPTs, Skills/plugins, Work, Codex, and API agents. Availability and limits can vary by plan, workspace, region, rollout, and client.

## Instructions and context

- **Global Custom Instructions:** The Help Center says they are available on web, desktop, iOS, and Android, apply immediately to chats, and can be edited or disabled in Settings. The current article says Free/Go can save 1,500 characters and Plus/Pro/Enterprise/Business/Edu 5,000; verify the account UI because limits may change. The same article also says updates are reflected only in future conversations in its history FAQ, conflicting with its explicit existing-conversation statement. This may concern historical instruction records, but no runtime resolution was tested; verify a live existing session before promising reload behavior. Source: [Custom Instructions](https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions); local mirror: `corpus/articles/openai/chatgpt/chatgpt-custom-instructions.md`.
- **No account-settings API:** The same article explicitly says there is no API for ChatGPT UI Custom Instructions; API system/developer messages are a separate mechanism, not a way to edit account personalization.
- **Projects:** Project instructions apply only within that project and override global Custom Instructions there. Project files and chat references provide available context, but do not prove every uploaded file is fully included on every turn. Existing personal projects can change memory mode in Project settings, with a possible delay of hours. The Projects article says project-only mode also makes ChatGPT Work unavailable in that project; do not recommend isolation to a Work-dependent workflow without checking this tradeoff. Shared projects are already project-only. Source: [Projects](https://help.openai.com/en/articles/10169521-projects-in-chatgpt); local mirror: `corpus/articles/openai/chatgpt/chatgpt-projects.md`.
- **Custom GPTs:** OpenAI says GPT conversations do not use saved memory, global Custom Instructions, or previous conversations. Source: [GPTs in ChatGPT](https://help.openai.com/en/articles/8554407-gpts-in-chatgpt); local mirror: `corpus/articles/openai/chatgpt/chatgpt-gpts.md`.
- **Memory:** Memory is distinct from direct instructions and is selective context, not an exhaustive transcript archive. Deleting a chat does not delete a separately saved memory; remove both and any other relevant sources when full removal is needed. Controls and behavior vary. Source: [Memory](https://help.openai.com/en/articles/8590148-memory-in-chatgpt); local mirror: `corpus/articles/openai/chatgpt/chatgpt-memory.md`.

## Execution surfaces and tools

- Ordinary Chat, ChatGPT Work, and Codex have separate sessions and permission semantics. Work capability depends on the specific web/desktop/mobile surface and granted access; do not assume an ordinary chat can use a local shell or arbitrary local files. Check effective tools and permissions in the active environment. Source: [ChatGPT Work and Codex](https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex); local mirror: `corpus/articles/openai/chatgpt/chatgpt-work-codex.md`.
- ChatGPT Skills and Plugins are distinct from account Custom Instructions and Project files. Eligibility and availability vary; a Skill does not itself grant network, file, or account permissions. Sources: [Skills in ChatGPT](https://help.openai.com/en/articles/20001066-skills-in-chatgpt), [Plugins in ChatGPT](https://help.openai.com/en/articles/20001256-plugins-in-chatgpt); local mirrors in `corpus/articles/openai/chatgpt/`.
- For difficult tasks, a model/effort control exposed by the UI is distinct from asking in a prompt to “think harder.” Availability depends on plan/workspace. Source: [GPT-6 and other models](https://help.openai.com/en/articles/20001354-gpt-6-and-other-models-in-chatgpt); local mirror: `corpus/articles/openai/chatgpt/chatgpt-model-selection.md`.

## Editing Custom Instructions

On web/desktop, use Settings → Personalization → Custom Instructions and enable customization. Mobile labels differ. This repository cannot edit account settings; Project instructions may need separate editing. See [custom-instructions.md](custom-instructions.md).

## Evidence limits

Local coverage includes Help Center mirrors for Custom Instructions, GPTs, Memory, Projects, model selection, Skills, Plugins, Work/Codex, personality, and release notes. These are documentation snapshots, not runtime tests. Confirm account-specific UI, plan limits, tool availability, and rollout state directly before relying on them.
