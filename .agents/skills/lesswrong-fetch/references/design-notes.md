# Design notes

## Fetching versus answering, 6 October 2026

The preserved skill mixed an absolute HTML ban with fallback instructions and mandated full text output. GraphQL-first with labeled fallback now has one meaning, and fetched text is input rather than an instruction to reproduce the body. The trigger now depends on needing site content, rather than every passing mention. A display name cannot substitute for a user slug; feed-looking slugs are clues, not sufficient evidence that a body is absent.

Queries retain the old schema examples with explicit uncertainty. At the initial pre-sweep, no live post/comment/feed query was made; EA compatibility, chronological feed ordering and root filtering remain unverified. `postCommentsTop` supplies a bounded sample rather than a promise of the five newest entries. Verify those cases before strengthening the contract. See the [pre-sweep assessment](../../../../docs/history/skill-presweep-20261006/README.md).

## Bounded live query checks, 6 October evening

Public single-record post/user queries returned the supplied fields at both endpoints, including slug and a null shortformFeedId for the sampled users. The body now requests slug/feed ID directly, requests parentCommentId for comment classification, and handles absent feeds. Results and limitations are retained in [the query checks](../../../../docs/history/skill-presweep-20261006/query-check/recipe-results.json). One initially unscoped LessWrong comments query timed out; that failure is retained rather than generalized into schema failure. No all-schema or chronological-feed compatibility claim follows.

Scoped follow-up queries returned a nonempty LessWrong comment and accepted the bounded post-comments recipe at both endpoints; the sampled EA post returned no comments. These results are retained in `query-check/scoped-recipe-followup.json` alongside the earlier checks. A real populated shortform-feed sequence, chronological ordering and live HTML fallback remain untested.
