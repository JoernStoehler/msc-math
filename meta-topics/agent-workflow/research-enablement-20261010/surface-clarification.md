# Surface requirements clarified on 10 October 2026

Owner: root in thread `01a1263d-568e-7100-aae5-eac9275fca3f`.
The user granted a further five minutes at 15:33:55 UTC, through 15:38:55 UTC.
This is bounded investigation/recording, not implementation or config activation.

## User requirements

See [environment requirements](../../development-environment/development-environments.md) and
[attributed project facts](../../../docs/project-facts.md). Supported targets include
Claude Chat/Cowork, ChatGPT Chat/Work, shell agents on chosen machines/microVMs
and the hosted coding surfaces. The main benefit is human interaction: input,
request visibility, rendered files and documented continuation. Chat's separate
quota and observed lower runaway tendency are further reasons Jörn gives.
No unresolved cloud-target choice or demonstrated access block remains from
this discussion. Actual account capabilities still need observation where used.

## Dated primary-source findings

Checked 10 October 2026. Product documentation is evidence of documented
behavior, not a live test of Jörn's account.

- [ChatGPT Work and Codex](https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex)
  distinguishes conversational Chat from longer agent work; Work/Codex draw
  from an included agent allowance. It also distinguishes cloud execution from
  remotely accessing a desktop task. This supports separate allocation and
  configuration, without establishing complete Chat/Work feature parity.
- [ChatGPT GitHub app](https://help.openai.com/en/articles/11145903-connecting-github-to-chatgpt)
  documents on-demand repository reads and a read-only app; coding writes use
  Codex. Another installed connector/plugin may supply other operations.
- [ChatGPT Library](https://help.openai.com/en/articles/20001052-using-library-to-manage-files-in-chatgpt)
  saves uploaded/generated files separately from daily attachment/chat limits.
  This does not establish persistence of sandbox processes or caches.
- [Claude Cowork architecture](https://support.claude.com/en/articles/14479288-claude-cowork-architecture-overview)
  describes account-saved sessions/files and temporary isolated execution,
  with mandatory egress proxy and separately brokered connector calls.
  It does not establish Jörn's proposed exact freeze/overlay implementation.
- [Claude GitHub integration](https://support.claude.com/en/articles/10167454-use-the-github-integration)
  documents selected branch files as context, without PR/history metadata.
- [Claude Code on the web](https://support.claude.com/en/articles/12618689-claude-code-on-the-web)
  documents repository setup, isolated tasks, branch push and a PR route.

Therefore distinguish sandbox shell/network, connector permissions, product
storage and configuration. Neither a proxy nor a short-lived sandbox implies
that durable project outputs cannot be saved; repository access alone does not
establish PR write authority. Actual interface/tools determine each route.

## Preparation changes

The then-prepared executor prompt and its original 90-minute proposal remained inactive
and unaccepted. Its launch requirement to choose one actual cloud runtime, and
its workstation/clean-Linux/Codex-only distribution framing, are superseded by
these clarified requirements. A revised implementation case must cover shared
scientific sources/setup and adapters for task input, retrieval, relevant
execution, human review, decisions, return and integration on the named targets.
Focused packets are a fallback, not a categorical limitation imposed on Chat.
No complete end-to-end adapter, new notification route or research readiness
was demonstrated in this five-minute check.

## Supplied failure evidence (historical paths)

Jörn supplied parent `01a125a6-d5b7-72a0-8064-6cee3eeb95fc` at this thread's launch.
Its 11:58:35 UTC user turn reports failed maintenance and dated collection work.
The parent diagnosis acknowledged it had not reconstructed collection creation.
Known retained maintenance evidence is in main's
`docs/history/skill-presweep-20261006/`, `docs/coordination/cloud-readiness-20261008/` and
`docs/coordination/handoffs/20261008-ab-split/`
and the repository-claims incident. Asking Jörn again for a general pointer was
premature. The original collection-creation trace is not yet established by
this record; source recovery can continue from the known evidence.
