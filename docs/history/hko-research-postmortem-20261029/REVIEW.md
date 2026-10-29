# Postmortem review and validation

The postmortem is an analysis of the 28–29 October research run. These checks
are checks of this historical report and its reading copy; they are not a
renewed scientific proof audit or human acceptance.

## Open-ended independent source/reader critique

Native reviewer `/root/postmortem_critique` read draft r1 against the verbatim
brief, causal and cost notes, and selected actual moderate-body and multicut
sources. Its final return found no consequential mathematical or arithmetic
overclaim and requested two changes: separate the 01:07 research-stop choice
from the 01:20–01:22 delivery, and make the 23:32-to-00:14 scheduling
counterfactual concrete while preserving uncertainty. Both were incorporated.
This was a freely scoped critique commissioned before final delivery, not
a request for Jörn to reconstruct sources or approve the analysis.

Coordinator `01a184a6-b383-7673-8fdd-fff5c4b49ad2` then read r2/r3, requested
bounded writing/delivery costs and explicit per-failed-probe attribution
limits, and supplied a depth-grid source for inspection. Those revisions
were incorporated without rerunning the scientific producers. The source
code, absent grid output and downstream note are distinguished in the report.

## Exact prepared-report/message review

Native `/root/postmortem_review` approved candidate `hko-postmortem-r2`.
The later substantive cost/discovery additions and rendered reading copy
supersede that approval's input scope. Native `/root/postmortem_review` then approved `hko-postmortem-r3`.
Its final return, retained verbatim, was:

> APPROVE hko-postmortem-r3
>
> The new cost envelopes are clearly scoped, and the report distinguishes recovered search code and child attribution from a completed scan or direct root instruction. It also makes the limits on per-probe costs and the corrected diagnostic counts explicit. The prepared message accurately points to the readable report. No consequential issues found.

[reviewed-inputs.json](reviewed-inputs.json) pins the approved report, reading
copy and support files; this review receipt is excluded from its own input
hash. The exact candidate, hashes and native return are also retained in
`/tmp/msc-math/message-reviews/hko-postmortem-r3/`.

## Validation performed

- Bounded original-run accounting reproduces 39 threads, 5,755 response
  records, 762,432,991 input tokens (749,892,352 cached), 3,579,678 output,
  and no accounting warnings. The coordinator independently reproduced
  these sums, cache bounds, helper hash and Astra shadow subtotal.
- Local Markdown targets resolve; retained JSON parses; Python source
  parses. Re-extraction preserves the original-window thread totals and
  earlier episode rows while adding the writing/delivery windows.
- `report.html` was actually viewed in isolated Chrome through the installed
  Playwright CLI at 1440 and 390 CSS pixels. The final main displays fit the
  356-pixel narrow content column; wide tables and the static diagram scroll
  within their own containers. The page has no whole-document horizontal
  overflow at either width. MathML renders and the embedded SVG loads.
- Opening the cost appendix from navigation expands it; source references
  navigate to their checkout-path catalogue; there are no sibling-file hrefs
  or external image dependencies. The usage projection and extraction-script
  downloads are data URLs. Screenshots and DOM observations are working
  evidence under `/tmp/msc-math/postmortem-20261029/render-qa/`. The final
  receipt is `playwright-final-metrics-r3b.json`, mirrored with the final HTML
  hash in `checks.json`; `headless-preliminary-metrics-r3.json` is the earlier
  500-pixel run and is not the final 390-pixel PASS receipt.
- The initial MCP browser request did not return and was terminated. The
  actual successful QA used an isolated CLI session and temporary loopback
  file server, not that unconfirmed request. Initial headless fragment
  screenshots were blank; they were not accepted as visual evidence. The
  Playwright screenshots were inspected instead.

The final commit's file hashes and Git diff checks identify the retained
report. Valid syntax, links, rendering and a source-backed agent review do
not establish that every historical decision or counterfactual is known.

## Analysis budget observation

The working clock began at 09:56:27 UTC on 29 October, with a 10:56:27
one-hour deadline and a separate 25-quota-point ceiling. The fresh 09:56:46
account reading was 70% used / 30% remaining. At 10:35:55 it was 72% used /
28% remaining; this shared-account change is not a two-point attribution to
the analysis. No reset, external spending, cloud execution or new scientific
producer was used.

The analysis thread `01a18716-e09c-7b71-93bc-c34b596b10eb` and four native
children had 133,720 recorded output tokens at 10:35:53, with 809,103
uncached and 26,755,840 cached input. This is an intermediate pre-handoff count,
not a final billing total. The helper's dated table does not
price its gpt-6.1-sol and gpt-6-luna models. The known QA browser session and
temporary loopback server were closed after inspection. Root retains main
integration and user-delivery ownership; this session edits only this folder.

## Final stopping-scope clarification (r3b)

The coordinator's last precision request narrowed the 01:07 wording to
“ending new research selection” and “the boundary for new discovery
allocation.” Subsequent proof repair, review, integration and delivery are
therefore not described as having stopped then. No mathematical result,
accounting total or counterfactual conclusion changed. The HTML was rebuilt
and the actual 1440/390 browser checks repeated on that final copy. Its SHA
is `687264c49d622d0f33aa2daa8a2aa4d5315540052fe140dadb5848d27bb1844f`;
`checks.json` and `reviewed-inputs.json` now identify that copy. Both known
QA sessions and their temporary servers were closed after their checks.

Native `/root/postmortem_review` approved the exact r3b inputs and prepared
message. Its final return is retained verbatim:

> APPROVE hko-postmortem-r3b
>
> The wording now clearly distinguishes ending new research selection at 01:07 from the subsequent review, integration, and delivery work. The prepared message remains accurate, and the optimality of that allocation decision is appropriately left unresolved.

The exact candidate, frozen input hashes and native return are in
`/tmp/msc-math/message-reviews/hko-postmortem-r3b/`. This final approval
supersedes r3 for delivery; r3 remains evidence of the preceding review.
