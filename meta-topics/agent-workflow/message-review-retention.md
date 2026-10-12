# Message review: durable findings and packet retention

This account retains the consequential findings extracted from the former
`docs/coordination/message-reviews/` tree on 12 October 2026. Exact outgoing
candidates, supplied context, revision copies and delivery verdicts are temporary
workflow records. New packets belong under `/tmp/msc-math/message-reviews/`.
The [project skill](../../.agents/skills/message-review/SKILL.md) owns the active
review procedure; this account records evidence and limits rather than a second
review protocol.

## Enforcement evidence, 8 October 2026

The initial inspection checked installed Codex 0.161.0 against release source
commit `979011409de0a60b52f179721948e65531d26144`. Its
[hook enum](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/protocol/src/protocol.rs#L1579)
had no arbitrary assistant-message pre-delivery event. Text deltas were
[emitted during sampling](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/core/src/session/turn.rs#L2284);
[Stop hooks ran after sampling](https://github.com/openai/codex/blob/979011409de0a60b52f179721948e65531d26144/codex-rs/core/src/session/turn.rs#L653).
A Stop hook could request continuation but could not withhold already streamed
text. This was release-specific source inspection, not a runtime intercept test
or a claim about later Codex releases.

The selected review mechanism was an instruction requiring separate review of
an exact candidate. A delivery-owning client that buffers text, or a runtime
change, would be needed for a hard delivery block; the inspected package did not
install either. The initial schema/hash checker did not intercept delivery.
Its focused tests checked receipt mechanics, not reviewer judgment quality, and
its cost/quality benefit was not established. Later review no longer required
JSON receipts, hashes or repeated candidate text.

The 8 October prompt trial supplied three synthetic cases in one no-tool
response: the reviewer rejected an unsupported inference from build/dependency
checks to repository-alone viability, approved a message keeping the migration
choice open, and rejected a request for a settled objective when that objective
was itself under discussion. This was a bounded synthetic application, not a
blind comparison or demonstrated future reliability. Per-review token and wall
time costs remained unmeasured.

## Findings retained by their substantive owners

| Finding from the removed delivery packets | Durable owner and boundary |
| --- | --- |
| App-server polling RPCs could exceed the requested wait window; a maintenance reference was initially missing. The subsequent review confirmed deadline clamping and the restored reference, with uncertainty wording corrected. | [App-server design and validation](../../.agents/skills/codex-app-server/references/design-notes.md) retains the defect, repair, validation and remaining cloud uncertainty. |
| The 10 October inventory checkpoint reversed public API and production-anchor test counts. The sender corrected them to 15 and 4. Browser operation remained unverified. | The inventory verification account and software survey own the corrected counts and check limits. Message approval did not validate all scientific content. |
| The 10 October graph received a fresh Astra planning review. Intrinsic SVG sizing and the reminder to freeze fixtures before geometry/data changes were checked; implementation correctness and overnight launch were outside approval. | The retained cleanup graph README and proposed graph own this planning disposition. Browser layout was unverified because no connection was available. |
| The 12 October pattern catalogue and revised priorities received separate message approval. The reviews distinguished observed discovery/stale-account problems from proposed folder/explanation remedies. | The repository-organization design account owns the layout, retrieval, lifecycle and resource tradeoffs. They are reasoned alternatives, not an experimentally ranked architecture. |
| The 11 October pilot summary reported completed geometry, builder and proof changes, alongside an unpromoted writing draft and missed human objections. | Pilot source reports, assembled review and writing/calibration evidence own these results. Exact delivery drafts are not mathematical or human thesis acceptance. |

No unique scientific proof, certificate or producer output was identified in the
removed delivery-packet tree by the targeted citation and finding searches.
Scientific and code/prose review evidence retained with inventory/pilot owners,
and the separate 9 October incident archive, were left intact.

## Recovery and migration boundary

All 107 files present in the former tree were copied, preserving relative paths,
to `/tmp/msc-math/message-reviews/migrated-repository-packets/` and byte-checked
before removal. Of these, 32 were tracked and 75 untracked. Tracked versions can
also be recovered from Git baseline
`844fc5be3884c12a93dabd795d0a0d12a92b2815`; the scratch copy covers the previously
untracked delivery packets. That local recovery copy is not the lasting owner of
any project conclusion. The substantive findings above and their source owners
are the retained project knowledge. Cleanup used targeted searches and Git
recovery, rather than a forensic audit of every candidate revision.
