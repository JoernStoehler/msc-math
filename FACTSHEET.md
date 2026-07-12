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

1. The previous June 2026 dates for sending the finished PDF to Kai are stale
   and are not active planning constraints. Jörn confirmed this on 2026-07-10;
   no replacement hard deadline is currently recorded here.

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

7.1. Current planned final review flow: once Jörn is satisfied with the thesis,
     he sends the final version to Kai, receives feedback, incorporates that
     feedback, and then hands in the thesis without showing Kai another full
     version.

## Thesis Scope And Results

8. The thesis has to cover the following content areas. Treat this as a scope
   fact; exact theorem and prose wording still comes from the thesis and source
   files.

   8.1. HKO local result: prove or honestly state the local-maximality result
   at the support strength actually achieved.

   8.2. Pentagon product result: present the rotated-regular-polygon/pentagon
   product result as a side result.

   8.3. Search/data-science result: present the standard-method search story
   and its final negative, positive, or caveated outcome.

   8.4. Generalized Reeb orbit and HK2019 finite-computation foundation: give
   the mathematical setup that makes the computations meaningful.

   8.5. First-order perturbation method: explain the local-variation method
   used by the HKO and search stories.

   8.6. Numerics/exactness story: explain where exact arithmetic, f64
   numerics, indeterminate decisions, and SageMath verification enter.

   8.7. Code/data/reproducibility story: explain what code/data support the
   thesis and how the claimed results can be reproduced.

   8.8. Use-of-AI disclosure: disclose AI use at the level Jörn decides.

   8.9. Visualization as exploration: include 3d visualization as exploratory
   support, not as a central result.

   8.10. CH2021/flow-graph/tube algorithm story: present the retained
   algorithmic story at the support level available.

   8.11. Preliminaries needed for readability: include only the background
   needed to make the retained results readable.

9. The thesis contains multiple results that answer different research
   questions. They should not be forced into a single tight narrative; the
   shared frame is that the project probed Viterbo's conjecture from several
   computational, experimental, and proof-by-computation directions.

10. The pentagon/rotated regular polygon story is a thesis result, but it is a
    side result and not among the first things Jörn would immediately name or
    focus on in the conclusion.

11. The 3d visualization is a side result that generates nice pictures and
    helps a bit with imagining what a 4d polytope looks like.

12. Tube/CH2021 is a retained content area, but it is unfinished right now and
    will change before thesis completion. The thesis needs the final
    Tube/CH2021 role to match the support available in the relevant thesis,
    research, and source files.

13. Thesis success does not require completing every possible side effort, such
    as interesting side routes, broad cleanup programs, publication-grade
    extensions, or post-thesis dissemination ideas, unless retained thesis
    claims or promises depend on them.

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

25. If retained thesis or HKO wording relies on arbitrary-polytope first-order
    behavior, the accepted route needs support for the non-generic
    arbitrary-polytope case, or the thesis must weaken or caveat the claim. A
    generic smooth-branch or Danskin-style statement is not a substitute for
    that non-generic support.

## Data Science And Search

30. The random/gradient search story is a major novel method for the
    data-science part, not merely supporting background. Current retained
    random-start gradient evidence supports systematic finite-step improvement
    and a measured cost profile on its named fixed-`F` panel; it does not
    support endpoint or local-maximality claims.

31. For data-science method coverage, do not weaken the standard-method
    expectation to merely representative families. Exhaust everything known
    from the standard repertoire where feasible. If a known applicable method
    is not executed, explicitly record that it was skipped because of time,
    cost, and low promise at a glance, or record the actual reason.

31.1. The data-science "standard repertoire" means the known data-science
      method/tool repertoire, on the order of 100 methods/tools. It does not
      mean proving exhaustion over every possible data-science method.

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

34.1. More data will be needed before the data-science story can be considered
      closed at thesis level. The exact datasets, sizes, and producer commands
      belong in the relevant experiment/research files, not in this fact sheet.

34.2. Jörn accepted the following data-science result wording on 2026-06-03 as
      the working replacement for vague phrases such as "transferable regime"
      or "useful general search rule":
      "The closed method table records no new source of `sys > 1` examples and
      no candidate-proposer for finding one, beyond examples that are already
      explained by the HKO2024 construction and its symplectic images or
      controlled perturbations."

35. The reproducibility target is that somebody can execute a flow around two
    years from now and reproduce the thesis results, ideally down to generating
    the final PDF.

36. The reproducibility flow should prefer regenerating claimed-reproducible
    artifacts from source over preserving them as opaque outputs. A plausible
    verification pattern is to delete claimed-reproducible artifacts, such as
    the thesis PDF, dataset JSONL files, or figure PNGs, then rerun the
    documented producer pipeline. Exact artifact classes still need final
    checking.

37. Byte-identical reproduction is the ideal regression signal. Ideally, after
    deleting and rerunning reproducible artifacts, `git diff` is empty because
    the pipeline reproduced outputs byte-for-byte. Some artifacts may need an
    accepted non-byte-identical comparison because of architecture, toolchain,
    or execution-environment differences, such as LICCA versus a local machine.

37.1. The long-horizon reproducibility instructions should name which
      artifacts are expected to reproduce byte-for-byte and which only
      reproduce up to an accepted comparison.

38. The full reproduction pipeline may be slow. The intended user-facing shape
    is still simple: to verify that output `X` is the output of producer `F`,
    delete `X` and rerun `F` from a single instruction file. That instruction
    file may reference scripts such as Slurm jobs.

38.1. The final reproducibility flow should include the route from source code
      and experiment outputs to thesis figures and the final PDF. This includes
      copy steps such as commands that copy generated figures into `thesis/`.

38.2. The obvious long-horizon reproduction route is the devcontainer or a
      documented local environment. When practical, run even long executables
      there, because reproduction matters more than performance.

38.3. The reproducibility playbook should also document how the original runs
      were done on LICCA. LICCA access may no longer be possible two years
      later, and most researchers do not have LICCA access.

38.4. The reproducibility playbook should record approximate timings for each
      step on the original machine, together with machine specs.

39. Git and Git LFS are useful for worktrees, checkouts, and partial reruns
    without paying for full reruns of expensive computations. For full
    reproducibility, tracking outputs is not what matters; the producer flow
    and its verification matter.

39.1. Expensive experiment outputs should not automatically be treated as data
      that must be preserved instead of regenerated. Saved data can still be
      useful as a regression test, depending on the artifact.

39.2. Reruns can support retained experiment claims when they support exactly
      the claim made in the thesis. A public certified solver claim needs
      certification support. Broad solver formalization is not a default thesis
      requirement unless retained thesis wording depends on it.

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

## Agent Limits And Advisor Risk

51. Agents can lessen the workload for Jörn/Kai by finding, flagging, and
    sometimes fixing errors, but they cannot replace Jörn/Kai acceptance where
    acceptance is required.

60. Unless there is newer evidence, the main advisor-facing problem is still
    that the thesis is incomplete, not a specific subtle objection from Kai or
    Elizabeth.

## External Memory And Advisor Context

70. Agents will see Kai/Elizabeth feedback through Jörn's commented
    interpretation and notes, not raw advisor feedback.

70.1. Jörn does not know whether there is Kai/Elizabeth feedback missing from
      the repo, because he does not know what feedback is currently recorded in
      the repo. Agents should inspect the repo-recorded feedback context before
      asking a narrower question. A 2026-06-03 scan found no obvious current
      unprocessed Kai/Elizabeth blocker in `FACTSHEET.md`,
      `thesis/DEVELOPMENT.md`, direct Kai/Elizabeth/advisor/review hits under
      then-current `thesis/`, `research/`, and `tasks/`, and local hits in
      `thesis/numerics.tex`, `thesis/hko-local-maximum.tex`, and the tube
      algorithm source note now at
      `crates/symplectic/src/algorithms/flow_graph/tube-algorithm-legacy-source-note.md`.
      That scan did not inspect local Codex logs, external mail/chat/calendar
      sources, or current official university sources.

71. Jörn has long-term memories of the project, chats with other agents, and
    external discussions with Kai that agents cannot inspect from the repo.

72. Advisor/context decisions may make a story sufficient, optional, or future;
    they do not by themselves settle theorem wording, proof correctness, final
    prose readiness, or submission readiness.

73. If Kai or Elizabeth give blocker feedback on clarity, proof support, scope,
    or submission readiness, agents should treat it as a real blocker unless
    Jörn explicitly records a non-blocking/cut decision. Agents will see such
    feedback through Jörn's interpretation/notes.

74. If a final submission requirement conflicts with a current thesis/repo
    promise, agents should surface the conflict immediately rather than
    silently choosing a workaround.

75. There is no concrete fast-review arrangement with Kai. Current planning
    should not assume Kai can review large material immediately.

## Planning Artifacts And Source Truth

80. The two large dated route/control reports were not read by Jörn. Do not
    treat Jörn's silence as endorsement of their contents.

81. Old thesis prose, formal notes, task files, maps, route/control packets,
    and Codex-written planning surfaces should not be promoted beyond their
    actual support. Source files and verbatim Jörn responses overrule agent
    summaries.

82. Facts should stay in their canonical owner file when that location is easy
    to find, predictable, expected, and robust enough. Facts belong in
    `FACTSHEET.md` when agents keep overlooking the other location, when the
    other location is too fragile under maintenance, when no good owner exists,
    or when related information belongs together and has no single
    discoverable robust home.

83. Jörn-confirmed current-state facts may belong in `FACTSHEET.md` when they
    are important. If staleness is a concern, mark the item with
    `[potentially stale]` instead of excluding it merely because it may become
    stale.

84. GPT-5.5 agents sometimes make wrong inferences. `FACTSHEET.md` can
    therefore include Jörn-confirmed facts that help agents avoid recurring
    wrong inferences or repeated bad questions, even when those facts seem too
    basic to mention.
