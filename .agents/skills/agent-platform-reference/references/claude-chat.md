# Claude chat, personalization, Cowork (not Claude Code)

**Reviewed:** 2026-10-13 against local Help Center mirrors. This is product documentation, not an observation of a particular account. Plans, permissions, and staged rollouts can change what a user sees.

## Global and project instructions

- Claude personalization guidance places persistent account-level instructions under Settings → Instructions for Claude. It describes their use across conversations where the feature is available. Source: [Personalization features](https://support.claude.com/en/articles/10185728-understanding-claude-s-personalization-features); local mirror: `corpus/articles/anthropic/claude-chat/claude-personalization.md`.
- Claude Projects have project instructions and a project knowledge base. The Projects help article says Free users can create up to five projects, while the personalization guide also contains a “paid plans only” statement about project instructions. This is a source inconsistency: verify the actual account UI; do not infer that Free definitively lacks project instructions. Sources: [Create and manage Projects](https://support.claude.com/en/articles/9519177-how-can-i-create-and-manage-projects), [Personalization features](https://support.claude.com/en/articles/10185728-understanding-claude-s-personalization-features); local mirrors in `corpus/articles/anthropic/claude-chat/`.
- Anthropic documents Chat Skills for eligible plans with code execution enabled. They are not the same deployment mechanism as filesystem Skills used by Claude Code. Source: [Create custom Skills](https://support.claude.com/en/articles/12512198-how-to-create-custom-skills); local mirror: `corpus/articles/anthropic/claude-chat/claude-chat-skills.md`.
- Memory is distinct from instructions and is not a shared complete transcript across project chats. Account-specific behavior can vary. Source: [Import and export memory](https://support.claude.com/en/articles/12123587-import-and-export-your-memory-from-claude); local mirror: `corpus/articles/anthropic/claude-chat/claude-chat-memory.md`.

## Chat and Cowork

- Anthropic documents a staged move toward one Claude experience combining chat and Cowork. Avoid asserting that every user has the merged UI or the same routing behavior. Source: [Cowork and Chat are one Claude](https://support.claude.com/en/articles/16761823-claude-cowork-and-chat-are-one-claude); local mirror: `corpus/articles/anthropic/claude-chat/claude-chat-cowork-merge.md`.
- Cowork documentation describes web, desktop, and mobile task access and execution context; it does not establish that a cloud task can automatically access arbitrary local files. Check the current surface and granted connections/permissions. Source: [Use Cowork on web, desktop, and mobile](https://support.claude.com/en/articles/15520349-use-claude-cowork-on-web-desktop-and-mobile); local mirror: `corpus/articles/anthropic/claude-chat/claude-chat-cloud-cowork.md`.
- Computer use is documented as beta and permission-dependent. Prefer the connector/browser/computer tool guidance only after checking which options the user actually has. Source: [Computer use in Cowork](https://support.claude.com/en/articles/14128542-let-claude-use-your-computer-in-cowork); local mirror: `corpus/articles/anthropic/claude-chat/claude-chat-cowork-computer-use.md`.
- Anthropic also documents a beta Projects experience that can divide work into cloud threads. Do not transfer those semantics to legacy Projects without identifying the current experience. Source: [What are Projects?](https://support.claude.com/en/articles/9517075-what-are-projects); local mirror: `corpus/articles/anthropic/claude-chat/claude-chat-projects-beta.md`.

## Deployment relevance

An offline reference must be made reachable through the actual chat surface, such as Project knowledge or an eligible Chat Skill. Installing a Claude Code filesystem Skill does not establish that ordinary Claude chat can read it or the surrounding source corpus.

## Evidence limits

Local mirrors cover personalization, Projects (including beta), Cowork, computer use, Skills, memory, and release notes. No account UI or Cowork runtime was tested; verify plan eligibility, rollout, execution location, and effective permissions live.
