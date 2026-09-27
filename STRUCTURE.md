# msc-math structure map (agent view; orchestration-level agents only)

Owner: Claude session https://claude.ai/code/session_015Yt4FvgviGuyBHMGS7ketm (started 2026-09-26).
Human view is generated on demand; Jörn is not expected to read this file.

## Nodes

- G0 Submit-ready thesis PDF (Jörn PASS). Proxies: MSc degree + a thesis Jörn is not embarrassed by. uphill. conf low. 2026-09-26
- G1 Wrap-up research (low-hanging theorems/counterexamples). Proxies: scientific value, thesis content. uphill. 2026-09-26
- G2 Learnings for future large agent projects. Proxies: Jörn's future productivity. uphill. 2026-09-26
- K1 Constraint: Jörn's reading time is the scarce resource; his whole-PDF read is the only trusted gate.
- K2 Constraint: Budget — Claude Pro plan (memory, Sept 2026) vs ChatGPT Pro $200; Codex quota was near-exhausted 2026-09-18.
- K3 Constraint: deadline — former 2026-09-18 22:00 UTC passed; current deadline UNKNOWN (U1).
- U1 Unknown: current hard deadline / Kai expectations.
- U2 Unknown: what Jörn attends to first when judging writing (partial evidence: DS Hypothesis annotations 2026-09-18).
- U3 Unknown: root cause of past trouble in Jörn's view vs process-audit REPORT (docs/history/process-audit/REPORT.md, 2026-09-15).
- U4 Unknown: whether thesis scope is settled (pentagon symmetric-partner bound selection unresolved).
- D1 Decision pending: execution harness (Codex vs Claude) for authoring.
- D2 Decision pending: repo cleanup / retire process-doc machinery (docs/ ~255k words vs thesis ~6k tex lines).
- P1 Candidate packet: mine ~/.codex user messages -> Jörn feedback corpus (3.1 GB, 1885 rollouts, 80% in Aug).
- P2 Candidate packet: style guide from Jörn annotations.
- P3 Candidate packet: chapter-level authoring.

## Edges
- G0 depends-on U2 (can't write to his bar without knowing it)
- G0 depends-on U1 (appetite)
- P3 depends-on P2, D1
- P1 feeds U2, U3
- D2 couples-with P3 (process docs crowd author context)

## Observations 2026-09-26 (from GitHub clone of main @3929a8c7)
- 3914 commits; activity Feb–Jul heavy, Aug 74, Sep 204. Since Aug: docs 121 commits, thesis 70.
- Recent commits are preservation/verification/navigation meta-work.
- Agent docs register: hedge per sentence ("does not establish…"), provenance-heavy.
- PDF 94 pages; whole-thesis FAIL 2026-09-15; completion attempt stopped 2026-09-18.
- RESUME.md says main at /workspaces/msc-math (devcontainer) with Jörn's uncommitted work.

## Log
- 2026-09-26 20:2x pre-registered predictions for claim batch B1 (mark, p(mark)):
  C1 deadline none/soft ✓ .45 | C2 binding constraint = Jörn time + no trusted writing workflow ✓ .75
  C3 scope ~settled, gap mostly writing ✓ .6 | C4 trouble = process overhead crowding authoring ✓ .7
  C5 agent-doc hedging leaked into thesis prose ✓ .6 | C6 PASS def per RESUME (default) ✓ .85
  C7 heavy execution on Codex, Claude coordinates ? .45 | C8 mine logs for Jörn's own messages ✓ .65
  C9 rewrite-per-chapter beats repair lists ? .45 | C10 retire process-doc machinery ✓ .6
  C11 GitHub main sufficient; uncommitted work irrelevant ? .4 | C12 one COORDINATION file naming live sessions ✓ .7
  C13 async-queue/Taildrop protocol still applies ✗ .6
- 2026-09-26 20:49 B1 marks: C1 ✓(pred ✓ .45 hit) | C7 ✗-ish: "most work in Codex; GPT-6 weak at user communication; fresh threads cheap" | C9 ? "cheap to try" | C11 ✗ "uncommitted work" label wrong; work on own branch/PRs | C13 ✓ | C3 ✓ (+ small low-hanging additions OK) | C5 ? missing context | C10 ✓ but question was wasted (don't ask him to estimate chores) | C4 ? one of many hypotheses | C8 ? | C2 ✓ "nothing to scale up yet" | C12 ✗ unclear question | C6 ✗ "massively incomplete operationalization"
  Hit rate on confident predictions ok; misses were on framing: 4 claims unclear/irrelevant to him. Lesson: don't present info-only facts as claims; define terms.
- Q3 (Jörn): agents wasted time instead of trying different workflows; collapsed idea space; explored workflows with low VOI; stopped asking for text review; scaled bad workflows to hit a fake deadline (~60% resources wasted).
- Q4: minimize time-to-PASS; scaling a PASSing workflow is expected cheap/fast. Budget not binding for now.
- Q2: "just give me a page" -> probe P0: send PDF p.5 (Intro start), ask for first-attention reactions.
- Reframe G0: bottleneck = discovering a writing workflow whose output Jörn PASSes on sampled pages. Cheapest probe = page-level reactions, then contrastive rewrites of the same page by different workflows.
- 2026-09-27 10:13 P0 result (p.5 Intro): FAIL at framing — intro says "questions -> methods"; Jörn: thesis idea is "methods we pick -> progress on questions we come up with". Other notes: "proposed" for conjecture sloppy; "inequality was", "dilation" odd; scaling invariance too trivial; EHZ undefined for MSc-student audience; introduce Lagrangian product + x_L early; "several geometric questions" falsely suggests short finite list; rhetorical opening question not his style. He stops at a framing error; agents should ask for more if they have bandwidth.
- Provenance: method-first idea present in Jörn's thesis/planned-toc.md @ea68f66c (2026-04-29: "natural idea to develop computational methods ... apply standard computational methods; recent interest in data-science for pure math"); Kai-meeting outline @dfc328d2 (2026-05-05) reduced it to "The thesis resumes the computational efforts"; file removed @f9d17b78 (2026-05-18). project-facts.md has only item 9 ("probed from several directions"). Aug intro companion frames questions->methods. => lost via summarization, never recorded as a fact.
- New node N-FRAME (uphill): thesis idea statement. Blocks intro, abstract, conclusion, AI chapter framing; couples with every chapter's opening.
- B2 predictions (T-lines, draft of Jörn's view): T1 ✓.7 T2 ✓.6 T3 ✓.8 T4 ?.45 T5 ✓.6 T6 ✓.55 T7(Kai knows/agrees) ?.5
- 2026-09-27 10:20 B2 marks: T1 ✓ T2 ✓ (origin: random perturbations didn't improve HKO) T3 "mu" (claim about text vs plan confusion) T4 ✓ T5 ✗-ish: not "same prominence"; answer the DS question early in intro, before theorems; never raw-false "no sys>1 found" T6 ✓ (+AI goes further: AI throws DS and does theory) T7 ✓. Recorded as docs/project-facts.md items 90–97.
- 2026-09-27 ~10:50 Trial T-INTRO-1 (experiments/writing-trials/20260927-intro-opening): 3 Claude subagents (general-purpose), same BRIEF. Blind labels sent to Jörn: A=W3 (write→simulated-Jörn critique→revise), B=W1 (fresh from facts), C=W2 (repair existing). Baseline W0 = current text.
  Predictions: Jörn prefers A .35, B .35, C .30; P(any variant judged PASS-level for the opening) .25; P(he stops at a framing error in all three) .3. Known risks: LICCA only in facts; B's GPT-5-series claim unchecked; C's "none with a larger ratio" possibly unsupported; A shows "Section ??" (standalone build).
