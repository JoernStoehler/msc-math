WIP: Jörn is currently reviewing and correcting this fact sheet.

# Jörn-Confirmed Project Facts

This is a central location of facts that Jörn has confirmed. It exists so
agents have a source of truth they can depend on without second-guessing.
Other locations for facts exist, sometimes even marked as Jörn-confirmed, but
most of the repository is written by Codex GPT-5.5 agents. Those claims are
often inaccurate or uncalibrated because of oversimplification during write-up,
misunderstandings from limited context, and limited time-to-think for Codex
agents.

The fact sheet was created by questioning Jörn in session:
- thread id: `019e8451-792d-7c60-b2e7-dd3c9524bfea`
- local transcript:
  `/home/vscode/.codex/sessions/2026/06/01/rollout-2026-06-01T17-53-12-019e8451-792d-7c60-b2e7-dd3c9524bfea.jsonl`

The file is grouped by interaction/topic between items to make it easier for
Jörn to check for gaps and contradictions. Epistemic confidence, source,
related reasoning, and use limits are provided in prose when useful, to keep
maintenance low. The numbers are stable and can be out of order.

## Thesis Submission

1. 9.6.2026 is a good deadline for sending the finished PDF to Kai.

2. Official submission facts need to be refreshed from current MNTF sources
   before relying on 2026-04-24 downloaded forms or old deadline text.

3. Elizabeth approved the registration form. The pending action is to hand in
   the registration note to the `Prüfungsamt`.

4. The `Einsichtnahme Dritter` choice is a Jörn decision unless a current
   official rule makes it mandatory. Jörn said this decision costs him about
   30 seconds.

5. Agents may prepare archive options and an artifact checklist, but the final
   archive target and artifact set need Jörn acceptance near finalization.

6. Zenodo is the leading non-GitHub preservation candidate because Kai named
   it. arXiv/outreach are post-Kai-review unless promoted.

7. Stale deadline/admin notes should be removed or corrected when found. The
   repo should not keep old deadline notes as live planning context.

## Thesis Scope And Results

8. The thesis has to cover the following content areas. Treat this as a scope
   fact; exact theorem and prose wording still comes from the thesis and source
   files.

   8.1. HKO local result.

   8.2. Pentagon product result.

   8.3. Search/data-science result.

   8.4. Generalized Reeb orbit and HK2019 finite-computation foundation.

   8.5. First-order perturbation method.

   8.6. Numerics/exactness story.

   8.7. Code/data/reproducibility story.

   8.8. Use-of-AI disclosure.

   8.9. Visualization as exploration.

   8.10. CH2021/flow-graph/tube algorithm story.

   8.11. Preliminaries needed for readability.

9. The thesis has multiple results going in different directions and answering
   different research questions, so it makes no sense to force them into one
   uniform story beyond roughly "we tried to find something interesting and we
   did".

10. The pentagon/rotated regular polygon story is a thesis result, but it is a
    side result and not among the first things Jörn would immediately name or
    focus on in the conclusion.

11. The 3d visualization is a side result that generates nice pictures and
    helps a bit with imagining what a 4d polytope looks like.

12. Tube/CH2021 is a retained content area. Its thesis role still has to be
    matched to the support available in the relevant thesis/research/source
    files.

## HKO Proof Support

20. The needed HKO form must close. The theorem is known true in project
    context; the work is to satisfy Kai's wish for a proper proof instead of a
    quick handwavy computation.

21. For HKO proof support, SageMath is used for verification, Rust is used for
    generation, LaTeX proves the correctness of the simple algebra algorithms
    that SageMath runs, and LaTeX proves that the algorithm implies the
    theorem.

22. HKO SageMath verification is not a typical experiment. Normally we are
    satisfied if we trust our proofs, algorithms, tests, and datasets, but here
    Kai has to be on board and wants something standard and human-readable.
    Thus the verifier is in SageMath, while the generator can remain a
    black-box/custom Rust component to outsiders.

23. Agents should not default to weakening or cutting central expensive routes
    just because they are expensive. Jörn noted that this is a recurring agent
    failure mode: agents may stumble into premature weakening/cutting unless
    someone watches for it.

24. We do not have raw local maximality. Agents should inspect the HKO theorem
    and source files for exact local-maximality wording instead of asking
    broad external-context questions about it.

## Data Science And Search

30. The random/gradient search story remains a major novel method needed for
    the data-science part, not merely supporting background.

31. For data-science method coverage, do not weaken the standard-method
    expectation to merely representative families. Exhaust everything known
    from the standard repertoire where feasible. If a known applicable method
    is not executed, explicitly record that it was skipped because of time,
    cost, and low promise at a glance, or record the actual reason.

32. A sudden positive or conjectured-positive data-science lead does not
    automatically force full follow-up before submission. It may be escalated
    or put in future work depending on importance and timing.

33. Numerics work can be postponed or rerun where it is internal to the
    library and not needed by retained thesis text. This item is about
    numerics, not a license to postpone the data-science story, which remains a
    major thesis method.

34. Agents should not prewrite the data-science section as purely negative
    before row closure. If a positive/conjectured-positive pattern appears, the
    thesis should honestly reflect that.

35. The reproducibility target is that somebody can execute a flow around two
    years from now and reproduce the thesis results, ideally down to generating
    the final PDF.

## Thesis Writing And Prose

40. All models Jörn tried write very badly by default for thesis prose:
    overconfident or nonsensical claims, unhelpful analogies, ultra-dense
    sentences, and about two style violations per sentence on average.

41. Fixing LaTeX build errors and converting markdown to LaTeX are trivial
    compared with phrasing and figuring out what to say where. The build should
    still be maintained so Jörn can read and review the PDF, especially for
    math.

42. Scaffold-only active thesis files are acceptable as a stage. A file with
    headings, labels, and TODO/context comments is not automatically a failure.

43. Legacy thesis prose is source material only. Even Jörn-approved legacy
    passages need revalidation if rewritten into the active thesis under
    changed structure or claims.

44. Formal `unverified` blocks are proof-route material, not accepted thesis
    proof.

45. AI-use disclosure is required in substance. The final disclosure length,
    tone, and level of detail remain Jörn's decision.

## Agent Work And Planning

50. Agents should minimize Jörn's total time. Do not replace this with proxies
    such as minimizing reading time, typing time, thinking time, context
    switching time, waiting time, review time, number of interruptions, or
    message length.

51. Agents can lessen the workload for Jörn/Kai by finding, flagging, and
    sometimes fixing errors, but they cannot replace Jörn/Kai acceptance where
    acceptance is required.

52. There is no stable one-size process for thesis work. Prose, notes, code,
    review, experiments, and parallel agents should be chosen case by case.

53. Source transfer and new reasoning should usually be separated when mixing
    them would make review harder. Jörn said mixing them often ends in
    confusion and errors that are hard to recover from except rollbacks.

54. Long tasks often get broken down into multiple sessions during the work:
    they are re-scoped and ended prematurely once size becomes clear.

55. Reusable review/task formats have been worthwhile in the past, but not
    always.

56. Standard tasks such as fixing LaTeX errors, exploring the repo, gathering
    relevant files, or producing a quick source report are reliable from very
    straightforward requests and do not need setup such as a reviewer subagent.

57. It is a live risk that important project knowledge is hidden or scattered
    enough that bounded source gathering and navigation aids can help. Jörn was
    unsure about the stronger version of this claim, so this should not be
    treated as a settled universal fact.

58. Do not introduce rigid labels, YAML, tags, or companion tables before they
    solve a real problem. Start with prose and grep unless structure clearly
    reduces total Jörn time or prevents likely errors.

59. Useful agent work should reduce total Jörn time. Task-definition cost is
    part of task cost; if available agent work would consume more total Jörn
    time than it saves, it is better to have no active agent for that work.

60. Unless there is newer evidence, the main advisor-facing problem is still
    that the thesis is incomplete, not a specific subtle objection from Kai or
    Elizabeth.

## External Memory And Advisor Context

70. Agents will see Kai/Elizabeth feedback through Jörn's commented
    interpretation and notes, not raw advisor feedback.

70.1. Jörn does not know whether there is Kai/Elizabeth feedback missing from
      the repo, because he does not know what feedback is currently recorded in
      the repo. Agents should inspect the repo-recorded feedback context before
      asking a narrower question.

71. Jörn has long-term memories of the project, chats with other agents, and
    external discussions with Kai that agents cannot inspect from the repo.

72. Even if the repo is insufficient to answer a question, agents should first
    know what the repo says so the question is easier to pose to Jörn and
    easier for Jörn to answer.

73. If Kai or Elizabeth give blocker feedback on clarity, proof support, scope,
    or submission readiness, agents should treat it as a real blocker unless
    Jörn explicitly records a non-blocking/cut decision. Agents will see such
    feedback through Jörn's interpretation/notes.

74. If a final submission requirement conflicts with a current thesis/repo
    promise, agents should surface the conflict immediately rather than
    silently choosing a workaround.

## Planning Artifacts And Source Truth

80. The two large dated route/control reports were not read by Jörn. Do not
    treat Jörn's silence as endorsement of their contents.

81. Old thesis prose, formal notes, task files, maps, route/control packets,
    and Codex-written planning surfaces should not be promoted beyond their
    actual support. Source files and verbatim Jörn responses overrule agent
    summaries.
