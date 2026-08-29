# AI-Use Provenance Investigation Prompt

Use this prompt in a fresh Codex session to refresh
`experiments/ai-use/reports/ai-provenance-log-backed-summary.md`.

```text
We need an evidence-backed AI-use / provenance investigation for the thesis
project at /workspaces/msc-math.

Context:
- This is Jörn Stöhler's master thesis repo.
- The final output is not prose for the thesis yet. It is a provenance model
  that later agents and Jörn can use to write an honest AI-use disclosure and
  PaperOrchestra input.
- Do not rely on git history as source truth for idea provenance.
- Treat raw Codex and Claude session logs as the main source of truth for
  AI/Jörn interaction provenance.
- Repo notes such as docs/project-facts.md and thesis/use-of-ai-content.md can guide the
  investigation but are downstream evidence, not source truth.
- Existing scratch hypothesis worksheet, if present:
  /tmp/joern/ai-provenance-interview.md. Treat it as hypotheses to test, not as
  evidence.

Task:
Build a log-backed summary of what AI was used for in this project, what Jörn
provided, and where provenance is uncertain.

Suggested pipeline:
1. Read /workspaces/msc-math/.agents/skills/codex-session-log-parsing/SKILL.md
   and follow it for Codex logs.
2. Inspect `$CODEX_HOME/session_index.jsonl` and rollout JSONL logs below
   `$CODEX_HOME/sessions`, `$CODEX_HOME/archived_sessions`, and any declared
   imported-session root to identify sessions relevant to
   /workspaces/msc-math, this thesis, PaperOrchestra, HKO, pentagon products,
   data science, experiments, proofs, code, writing, and orchestration.
3. Inspect Claude logs only when an explicit staged Claude root was provided,
   especially paths below it matching msc-math or msc-viterbo. Do not assume
   that the active execution environment exposes Claude state; a missing root
   is not evidence that no relevant Claude history exists. Follow the staging
   and `--claude-root` contract in `experiments/ai-use/README.md`.
4. For each relevant session, inspect the rollout/transcript enough to classify
   chat-level provenance:
   - user messages,
   - final-channel or user-visible assistant messages,
   - subagent completion summaries if present.
   Avoid dumping long tool outputs.
5. In scratch notes only, record:
   - session id and rollout/transcript path,
   - rough date,
   - topic/work area,
   - what Jörn contributed,
   - what AI contributed,
   - what was accepted/rejected/uncertain,
   - artifacts or repo areas affected if visible,
   - whether this is strong, medium, or weak provenance evidence.
6. Use fast subagents if helpful: one log or small log batch per subagent is
   ideal. Their task should be to read chat/final messages and classify
   Jörn-vs-AI contributions. Do not ask them to inspect the whole repo.
7. Merge the session classifications into a conceptual model by work area:
   - thesis framing and research questions,
   - HKO local maximum,
   - pentagon/rotated regular product formula,
   - Clarke dual action principle and foundations,
   - HK2017/QP algorithm,
   - CH2021/flow-graph algorithm,
   - Rust/Sage/Python coding,
   - verification and tests,
   - experiments and data science,
   - figures/plots/assets,
   - literature review and bibliography,
   - thesis prose,
   - project management / agent orchestration / prompt engineering,
   - advisor context from Kai/Elizabeth.
8. Compare the merged model against /tmp/joern/ai-provenance-interview.md if it
   exists and flag where that worksheet appears wrong, unsupported, or missing
   distinctions.
9. Write the final result to
   experiments/ai-use/reports/ai-provenance-log-backed-summary.md.

Output format:
- Start with a short "How to read this" note explaining evidence limits.
- Then a concise executive model: what AI was broadly used for, what Jörn
  broadly contributed, what remains uncertain.
- Then a per-area provenance table.
- Then an aggregate update from earlier Claude/pre-Codex evidence if available.
- Then a comparison against the interview worksheet, if available.
- Then "questions for Jörn" listing only high-value uncertainties.
- Do not write thesis disclosure prose yet.
- Do not include per-session evidence tables, transcript-path tables, or raw
  transcript excerpts in the committed report. Keep those in scratch space if
  needed for local review.

Important constraints:
- Logs may contain sensitive material. Summarize; do not paste long transcript
  excerpts.
- Do not claim something is Jörn-made or AI-made unless the logs support it or
  mark it as uncertain.
- User messages are strong evidence for what Jörn provided, but not complete
  evidence for all Jörn offline thinking.
- Assistant final messages are evidence of what AI produced, but not
  necessarily evidence that Jörn accepted it.
- Session logs are source truth for interaction history, not for mathematical
  correctness.
- Keep the final file useful for a later PaperOrchestra run and for an eventual
  AI-use disclosure.
```
