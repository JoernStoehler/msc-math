---
name: human-prose-review
description: Present PDF, HTML or Markdown for Jörn to annotate in Hypothesis, then retrieve his feedback through the API when he says done.
---

# Prepare the artifact

Choose the prose and surrounding context for the review. Retain the frozen artifact and feedback with their manuscript, experiment or other evidence owner; choose a new review directory there before running the helper. Keep the intended format; PDF need not become Markdown. If comparing with an agent review, freeze that review and withhold it until after Jörn's reading.

# Prepare the page

From the repository root on the host:

```bash
review_dir="path/to/evidence-owner/human-review/example"  # choose the retained owner
python3 .agents/skills/human-prose-review/scripts/open_review.py draft.pdf --output "$review_dir"
```

Accepts PDF, standalone HTML or Markdown. The new output directory retains the source, served artifact and `review.json`. The script prints an eight-hour Tailscale link with Hypothesis included: PDF.js for PDF, rendered HTML otherwise. No extension or download is needed. Check readability and annotation controls; leave the reviewed files unchanged. Use a new directory for a revision.

Requires Python 3, Tailscale, a systemd user session, and Pandoc for Markdown. The first PDF run downloads a pinned viewer into the host cache. From a sandbox, use an available host handoff route from `external-agent-communication` and confirm that the host can serve it; do not give Jörn an unreachable localhost link.

# Ask Jörn to review

Give the link, needed context and the judgment sought. He signs in to Hypothesis, annotates, then says **done**. First-use shortcuts: select text → **H** to highlight or **A** to comment; **Ctrl+Enter** saves. Comments are optional; overall feedback can go in chat.

# Receive and use feedback

```bash
python3 .agents/skills/human-prose-review/scripts/fetch_review.py --review "$review_dir/review.json" --output "$review_dir/feedback.json"
```

The fetcher reads `~/.config/hypothesis/api-token`, queries only this document and Jörn's account, and saves the returned annotations. Tell Jörn when using the credential; never print it or put it in the served page. If unavailable, he can use **Share → Export → JSON → Copy to clipboard** instead.

Match selections to the retained artifact, preserving the raw response. PDF extraction and HTML mathematics can produce unusual quoted text. Keep his words and corrections separate from your interpretation, then use the feedback for the requested revision or comparison. Unmarked text is not an approval.

When maintaining this skill, read [design notes](references/design-notes.md) for evidence, scope and unresolved choices. These are maintenance context, not another execution-policy owner.
