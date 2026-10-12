---
name: lesswrong-fetch
description: Fetch LessWrong or EA Forum posts, comments and shortform content when reading that site content is needed for the task. Prefer GraphQL to avoid unrelated page and comment noise; use a labeled HTML fallback when needed. A passing mention or unused search-result URL does not require fetching.
---

# LessWrong Fetch Skill

Use the LessWrong GraphQL API to fetch post and comment content cleanly. **Use GraphQL first when site content is needed. If it fails, use an available HTML retrieval client and label the fallback and its limitations.**

## Endpoints

| Site | GraphQL endpoint |
|------|-----------------|
| LessWrong | `https://www.lesswrong.com/graphql` |
| EA Forum | `https://forum.effectivealtruism.org/graphql` |

All queries: HTTP POST, `Content-Type: application/json`.

---

## Step 1: Parse the URL → pick your workflow

| URL pattern | Workflow |
|---|---|
| `/posts/POST_ID/slug` | → **[A] Fetch post** |
| `/posts/POST_ID/slug?commentId=COMMENT_ID` | → **[B] Fetch comment + post title** (parallel) |
| `/posts/POST_ID/slug#COMMENT_ID` | → **[B] Fetch comment + post title** (parallel) |
| `/users/USERNAME/shortform` | → **[C] Shortform feed** (2 sequential steps) |

**Extracting IDs:**
- Post ID: path segment immediately after `/posts/`, e.g. `/posts/TxcRbCYHaeL59aY7E/meditations-on-moloch` → `TxcRbCYHaeL59aY7E`
- Comment ID: value of `?commentId=` query param, or `#fragment` (whichever is present)
- Username for shortform: path segment after `/users/`, e.g. `/users/habryka4/shortform` → `habryka4`

**Shortform detection** (affects how you present results in workflow A):
A slug ending in `shortform` or `shortform-feed` is a clue to check the returned fields. When `shortform: true` and `contents: null`, treat the post as a feed stub; its entries live in comments. Do not discard a returned body merely because the slug resembles a feed.

---

## Workflow A — Fetch a post

Single API call:

```json
{
  "query": "{ post(input: { selector: { _id: \"POST_ID\" } }) { result { _id title shortform postedAt baseScore commentCount user { displayName slug shortformFeedId } tags { name } contents { markdown } } } }"
}
```

**Present:** source context and the requested summary, analysis or permitted excerpt. Full text is research input; do not reproduce an entire body merely because it was fetched. Respect applicable quotation and copyright limits.

If `shortform: true` and `contents: null`, treat the post as a feed stub. Tell the user this is a shortform feed and offer to load feed entries — use the returned `user.shortformFeedId` directly for Workflow C step 2, or `user.slug` for step 1 (a display name is not a slug), or ask the user if they wanted a specific entry.

**Do not auto-fetch comments** on regular posts. The comment count may be in the hundreds.

---

## Workflow B — Fetch a comment (with post context)

Fire **two parallel calls** — they are fully independent:

**Call 1 — the comment:**
```json
{
  "query": "{ comment(input: { selector: { _id: \"COMMENT_ID\" } }) { result { _id postId parentCommentId postedAt baseScore user { displayName } contents { markdown } } } }"
}
```

**Call 2 — the post title (for context):**
```json
{
  "query": "{ post(input: { selector: { _id: \"POST_ID\" } }) { result { _id title shortform user { displayName slug shortformFeedId } } } }"
}
```

Call 2 intentionally requests only post context. A root comment on a confirmed shortform feed is an entry; a comment with a parent may be a reply. Preserve that distinction rather than calling every comment a feed entry.

**Present:** author and post context, then answer at the requested scope using summaries or permitted excerpts.

---

## Workflow C — Shortform feed (no specific comment)

URL: `/users/USERNAME/shortform` with no commentId.

This requires two **sequential** calls (the second depends on the first):

**Step 1 — resolve username to shortformFeedId:**
```json
{
  "query": "{ user(input: { selector: { slug: \"USERNAME\" } }) { result { displayName shortformFeedId } } }"
}
```

If the user is missing or `shortformFeedId` is null, report that no feed was returned; do not query with an invented or null post ID.

**Step 2 — fetch a bounded sample of feed comments** (use `shortformFeedId` from step 1):
```json
{
  "query": "{ comments(input: { terms: { postId: \"SHORTFORM_FEED_ID\", view: \"postCommentsTop\", limit: 5 } }) { results { _id parentCommentId postedAt baseScore user { displayName } contents { markdown } } } }"
}
```

Filter to `parentCommentId: null` to show only root shortform entries (not replies). Retain entry IDs and source context; summarize or quote within the requested scope and applicable limits. This query selects top comments, not a verified chronological feed; filtering after the limit may return fewer than five root entries.

---

## Error handling

- If `result` is `null`: the content is missing or restricted. Tell the user.
- Network failure, non-200, or HTTP-200 GraphQL `errors`: report the failed query; fall back through an available HTML retrieval client on the original URL and warn the user the result may be noisy.
- These query examples assume compatible fields at EA Forum; inspect returned data/errors rather than claiming identical schemas. A missing field needs a site-specific query adaptation.

When maintaining this skill, read [design notes](references/design-notes.md) for evidence, scope and unresolved choices. These are maintenance context, not another execution-policy owner.
