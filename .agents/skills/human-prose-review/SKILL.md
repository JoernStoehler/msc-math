---
name: human-prose-review
description: Present PDF, HTML or Markdown for Jörn to annotate in Hypothesis, then retrieve his feedback through the API when he says done.
---

# Prepare the artifact

Choose the prose and surrounding context for the review. Keep the intended format; PDF need not become Markdown. If comparing with an agent review, freeze that review and withhold it until after Jörn's reading.

# Prepare the page

From the repository root on the host:

```bash
python3 .agents/skills/human-prose-review/scripts/open_review.py draft.pdf --output reviews/example
```

Accepts PDF, standalone HTML or Markdown. The new output directory retains the source, served artifact and `review.json`. The script prints an eight-hour Tailscale link with Hypothesis included: PDF.js for PDF, rendered HTML otherwise. No extension or download is needed. Check readability and annotation controls; leave the reviewed files unchanged. Use a new directory for a revision.

Requires Python 3, Tailscale, a systemd user session, and Pandoc for Markdown. The first PDF run downloads a pinned viewer into the host cache. From a sandbox, ask a host agent to serve it through Herdr; do not give Jörn an unreachable localhost link.

# Ask Jörn to review

Give the link, needed context and the judgment sought. He signs in to Hypothesis, annotates, then says **done**. First-use shortcuts: select text → **H** to highlight or **A** to comment; **Ctrl+Enter** saves. Comments are optional; overall feedback can go in chat.

# Receive and use feedback

```bash
python3 .agents/skills/human-prose-review/scripts/fetch_review.py --review reviews/example/review.json --output reviews/example/feedback.json
```

The fetcher reads `~/.config/hypothesis/api-token`, queries only this document and Jörn's account, and saves the returned annotations. Tell Jörn when using the credential; never print it or put it in the served page. If unavailable, he can use **Share → Export → JSON → Copy to clipboard** instead.

Match selections to the retained artifact, preserving the raw response. PDF extraction and HTML mathematics can produce unusual quoted text. Keep his words and corrections separate from your interpretation, then use the feedback for the requested revision or comparison. Unmarked text is not an approval.
