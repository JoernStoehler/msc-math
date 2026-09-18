---
name: chatgpt-pro
description: Prepare focused research ZIPs for Jörn to run in ChatGPT Pro (including Astra Pro), exchange them with his Chromebook through Taildrop, and receive and independently check the returned work. Use for this human-mediated deep-research handoff, not ordinary Codex subagent delegation or API model selection.
---

# ChatGPT Pro research handoff

## Choose and prepare the task

Use this route for concentrated reasoning where a good mathematical or conceptual insight could matter more than many parallel implementation steps. It can complement ordinary agents that recover sources, test examples and identify the bottleneck. Do not manufacture a hard problem just to use Pro, or require an elaborate preparation phase when a clear task and a few sources suffice.

Jörn operates ChatGPT and selects the requested Pro mode. This workflow does not spawn Pro through the Codex collaboration tools or imply an equivalent API call. Do not silently substitute a different model. For current product availability, mode names or limitations, use the OpenAI Docs skill; user observations do not establish product internals.

Create a compact dossier with `GOAL.md` at ZIP root:

- State the actual goal, useful outcomes and freedom to choose another productive approach. Supply definitions, conventions and necessary constraints.
- Include the few sources needed for independent reasoning. Prefer one clear proof plus optional supporting material over multiple near-duplicate expositions.
- Distinguish established results, imported assumptions, proposed conjectures and unresolved gaps. Keep provenance separate from the main argument.
- Ask for self-contained results and proofs or precise unresolved steps. A counterexample or useful obstruction can be a good outcome. For research, qualify novelty unless checked.
- Request a single return ZIP with an entry-point summary, substantive sources and any useful optional verification artifacts. Ask that proof and computational evidence remain distinct.

A repository snapshot or dataset is not the default attachment. Include data/code only when it materially supports this task; Pro can be selective, but irrelevant volume distracts. Do not include credentials, private logs or unrelated material. Keep useful working material in its owning repository, and disposable packaging work in `/tmp`.

Check archive contents and paths, open the entry point, and verify included source bytes/hashes. A small manifest linking input versions is useful, not mandatory infrastructure. Commit coherent project-owned handoff material with the repository's required provenance trailer. Never sweep unrelated dirty work into that commit.

## Send to Jörn

For an invoked handoff through this skill, send the prepared research ZIP to Jörn's Chromebook with Taildrop; no repeated confirmation is needed. Verify the destination from current state rather than copying an old IP:

```sh
tailscale file cp --targets
# If needed, inspect tailscale status --json; show only pertinent peer fields.
tailscale file cp /absolute/path/research.zip <verified-chromebook-name-or-ip>:
```

On the successful 2026-09-18 exchange, the peer was named `chromebook` (host name `brya`). Treat that as a discovery hint, not a permanent identifier. A successful command establishes transfer success, not that Jörn opened or uploaded it. Report the filename and result concisely.

Jörn uploads the ZIP to ChatGPT Pro. His exact successful chat prompt was:

> Prompt: <GOAL.md inside the zip>. Please work through this and report back with a single zip file that i will send back to the project agent. Thanks!

The angle-bracket text is part of the prompt referring to the attached file; there is no need to paste the whole goal again. Preserve a requested model/mode; ask only if actual selection is unclear or unavailable. Do not automate his browser or account unless separately requested.

Jörn reported typical waits of 40–90 minutes for his Astra Pro tasks. This is an observation, not a guaranteed latency, internal timer or evidence about hidden compute allocation. Keep the external task pending and continue independent work. Do not spend the interval polling Jörn.

## Receive and evaluate

When Jörn says he sent the result, receive into a new scratch directory:

```sh
mkdir -p /tmp/pro-return-unique
tailscale file get --verbose --conflict=rename /tmp/pro-return-unique
```

This drains available inbox files; identify the expected archive and preserve any unrelated arrivals rather than deleting them. Inspect the ZIP listing before extraction, reject path traversal/absolute paths, and check size and integrity. Inspect scripts before executing them. Verify any input-identity and payload hashes supplied. Missing manifests do not invalidate mathematics; record what identity can actually be established.

Preserve the original returned bytes in the project's handoff/provenance area. Read the substantive claims, then independently check the consequential arguments; delegate separable checks when authorized. Tests and algebra scripts can support a proof but do not replace missing implications or exhaustive-case arguments. State what was checked and what remains uncertain. Do not call a result accepted solely because Pro or several agents endorse it.

Move useful mathematics or other knowledge into the ordinary owning source area with provenance, corrections and review status. Keep an immutable original when editing returned material. A ZIP alone is not the maintained project result. Clean up task scratch after durable preservation; leave unrelated work untouched.

## Evidence for this workflow

The 2026-09-18 pentagon-generalization exchange used a seven-file, roughly15KB mathematical dossier, Taildrop in both directions, the exact chat prompt above, and a returned proof/source/verification ZIP. Input identity and all returned payload hashes matched. Separate audits found no internal gap in the main affine-pentagon argument; imported-source and integration work remained distinct. This demonstrates that the exchange worked once, not a general model-success rate or a guaranteed quality advantage. Project provenance: `docs/pro-handoffs/` on `research/pro-result-audit` and its descendant integration work.

Official documentation checked 2026-09-18: the [Astra guide](https://developers.openai.com/api/docs/guides/latest-model) and [reasoning-mode guide](https://developers.openai.com/api/docs/guides/reasoning#reasoning-mode) describe API Pro mode; this does not establish equivalence to this ChatGPT exchange. [Model controls](https://learn.chatgpt.com/docs/models) depend on product/account availability. The checked sources did not establish this chat UI's exact picker sequence, ZIP limits or a guaranteed runtime. These details are not prerequisites for repeating the observed exchange.
