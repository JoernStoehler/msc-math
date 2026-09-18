---
name: chatgpt-pro
description: Hand off research to ChatGPT Pro (including Astra Pro) through Jörn using a focused ZIP and Chromebook Taildrop, then receive the returned ZIP. Use for this human-mediated workflow, not Codex subagent delegation.
---

# ChatGPT Pro handoff

Jörn operates ChatGPT Pro. Prepare the task and files; he uploads them and returns the result. This worked for the pentagon-generalization task on 2026-09-18.

## Outbound

Put `GOAL.md` at ZIP root with the task and essential context. Include focused supporting material; omit the repository, datasets and duplicate explanations unless actually useful. Jörn preferred a small mathematical dossier for the proof task.

Send the ZIP to his Chromebook through Taildrop. An invoked handoff authorizes this transfer without another confirmation. Discover the current target rather than hardcoding its IP; the successful peer was `chromebook`, host name `brya`.

```sh
tailscale file cp --targets
tailscale file cp /absolute/path/task.zip <chromebook-target>:
```

Give Jörn the filename and this exact prompt, which he used successfully:

> Prompt: <GOAL.md inside the zip>. Please work through this and report back with a single zip file that i will send back to the project agent. Thanks!

He reported 40–90-minute waits for Astra Pro. Treat that as observed latency, not an internal timer or guarantee; continue independent work while waiting for him to return the result.

## Inbound

When Jörn says he sent the ZIP, receive it into a fresh scratch directory:

```sh
mkdir -p /tmp/pro-return-unique
tailscale file get --verbose --conflict=rename /tmp/pro-return-unique
```

This drains the inbox, so preserve any unrelated arrivals. Retain the original return, independently review consequential results, and integrate useful material into its ordinary repository home rather than leaving the ZIP as its only owner.
