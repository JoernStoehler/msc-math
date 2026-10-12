I think this is worth trying. **The useful intermediate deliverable is a complete scientific account whose consumer is another agent.** That lets us postpone pedagogical pacing, background explanations and polished prose while still requiring the arguments, evidence and connections to be complete.

This builds on earlier discussions: the [3 October plan](https://joern-pc.tailc5e761.ts.net/files/workspaces/msc-math/docs/coordination/structure.md) proposed a coverage spine and whole-argument checks; the [September writing proposals](https://joern-pc.tailc5e761.ts.net/files/workspaces/msc-math/docs/history/scientific-writing-solutions/README.md) proposed testing what readers can reconstruct. Your proposal gives those checks a substantive object to work on before human exposition.

I agree that agent readability plausibly makes *communication* much easier: the consumer can search the repo, follow links, run checks and ask follow-ups. **It doesn't automatically make mathematical correctness easier.** We should test the claimed advantage rather than infer it from agents agreeing with each other.

I'd make the intermediate account flexible Markdown plus equations, figures and precise source links. Compact technical language is fine. Its essential contents would be:

- The exact results, hypotheses, conventions and limitations.
- The actual arguments connecting premises to conclusions, including the bridges between papers, proofs, certificates and implementations.
- For empirical results: the question, population, method, observed outcome and what that outcome supports.
- Figures with their intended scientific meaning and producer/data references; visual polish can wait.
- Explicit unresolved obligations and choices about thesis inclusion.

This should contain enough substance that the next author doesn't have to rediscover the science. A list of claims and links alone wouldn't achieve your goal. Existing proof/evidence packets should remain their source owners; the writeup assembles their relationships and fills missing connections.

Your bidirectional checking idea is particularly useful, with two different jobs:

| Direction | Question |
|---|---|
| Writeup → repo | Does each claim have support, and does that support establish this exact statement? Are hypotheses, normalizations and computational trust boundaries preserved? |
| Repo → writeup | Have we omitted a selected result, necessary dependency, caveat or contradiction? Have newer findings changed the account? |

The second direction needs an independent search through relevant sources. Following only the writeup's own links can't detect an omitted dependency. And “completion” should mean covering the selected thesis scope and necessary support, not documenting everything in the repository.

A concrete example: the retained [HKO eleven-facet review](https://joern-pc.tailc5e761.ts.net/files/workspaces/msc-math/docs/empirical-viterbo-design/night-20261005/f11-acceptance/report.md) distinguishes a finite replay, inherited ten-facet sharpness, and analytic arguments covering moving normals and limiting sequences. An agent-facing account should make those implications recoverable. “The verifier passed, therefore local maximality” would fail even with perfect links.

For the feedback loop, I'd ask a fresh consumer to **use** the account: reconstruct a proof dependency, determine whether a boundary case is covered, or locate the precise data supporting a conclusion. Check its answers against original sources, allowing it to return a concrete gap. Repair those gaps and rerun affected tasks. Where an executable verifier exists, use it for its actual predicates. Agreement and successful reconstruction provide useful evidence; they don't certify the mathematics.

Cheap reruns can combine link/source-change checks with targeted semantic review. Unchanged material can reuse a dated check; changed statements or dependencies require renewed inspection. We'd still need occasional broader searches for additions outside the declared links. I wouldn't build a general checking framework first.

**The cheapest first comparison is one result, not a second whole thesis.** Use the selected fixed-pentagon/arbitrary-symmetric-partner result: it has a retained proof packet and a concrete inclusion need. Compare:

1. A short supplement to the existing packet that supplies missing connections, followed by direct human-section authoring.
2. A fuller agent-facing account, tested by a fresh consumer, followed by human-section authoring.

Keep the scientific sources and total effort comparable. Look at unsupported claims, missed caveats, source rediscovery, downstream authoring effort, and your response to the resulting section. The fuller route wins only if it saves enough downstream work or catches enough consequential gaps to justify its preparation and maintenance.

My preference is to try that comparison before committing to the entire intermediate writeup. The main uncertainty is whether it removes repeated scientific reconstruction or merely adds another document. Even if it works, human authoring will still need decisions about emphasis, explanations and omissions—and checks that those edits preserve the supported claims.

[review: /root/writeup_review]
