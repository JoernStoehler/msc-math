# Planning Benchmark Method

Use this reference when updating or reviewing `$planning-strategy`, especially
after new planning failures or unexpectedly good planning sessions. Do not load
it for ordinary task use.

Goal: turn local Codex session history into a private behavior benchmark for
planning workflows. The benchmark should test whether the skill catches
planning failures before costly execution and preserves successful behavior.

Raw rollout JSONL is source truth. Benchmark files should contain paraphrased
rows, not transcript dumps.

## Inputs

- `~/.codex/session_index.jsonl` for thread ids, names, and update times.
- Raw rollout JSONL under `~/.codex/sessions/**` and
  `~/.codex/archived_sessions/**`.
- The current `SKILL.md` under review.
- Recent Jörn corrections when they identify concrete failure modes.
- External practice only as design background, not as source truth for local
  behavior.

Use `$codex-session-log-parsing` before inspecting logs.

## Sampling

Start from recent session-index rows and filter for likely planning surfaces:

- `plan`, `choose`, `workflow`, `harness`, `coordination`, `audit`, `inspect`,
  `execute`, `review`, `update`, `slice`, `status`, `packet`, `claim`,
  `thesis`.

Include:

- known planning failures and repair-heavy sessions;
- successful route-choice sessions;
- small direct tasks that should not trigger planning;
- objective/slice choice cases that should route to `$scoping`;
- prompt/harness/workflow changes where a wrong plan can consume Jörn time;
- successful bounded workflows whose speed should not regress.

Do not sample only failures. Success and non-trigger rows are what keep the
skill from becoming ritual planning.

## Extractor Role

Use cheap subagents, usually `gpt-5.4-mini`, as extractors. Their job is to
propose paraphrased benchmark rows, not to make final labels.

Give each extractor:

- a TSV batch of rollout paths with thread ids and names;
- the benchmark schema;
- the instruction to inspect only enough raw log content to classify
  planning-relevant episodes;
- the instruction to avoid transcript dumps and long quotes;
- a target of roughly 8-15 high-signal rows per batch;
- permission to mark `unclear` instead of guessing private intent.

Main agent must audit the extracted rows. Do not copy extractor labels into the
skill without review.

## Row Schema

Use this schema for extractor output:

- `id`
- `source_thread_id`
- `source_thread_name`
- `source_rollout_path`
- `episode_type`: `failure`, `success`, `near_miss`, or `unclear`
- `task_domain`: `harness`, `thesis`, `experiment`, `proof`, `review`,
  `coordination`, or `other`
- `prompt_shape`: `visible_plan_request`, `autonomous_work`,
  `objective_choice`, `route_choice`, `recovery`, `review`,
  `implementation`
- `primary_objective`
- `candidate_routes_present`: `yes`, `no`, or `unclear`
- `observed_agent_behavior`
- `planning_issue`: `objective_stack`, `route_comparison`,
  `synergy_assumption`, `stop_criteria`, `report_target`, `jörn_attention`,
  `delegation`, `chat_premature_stop`, `none`, or `other`
- `would_planning_strategy_trigger`: `yes`, `no`, or `unclear`
- `trigger_reason`
- `desired_pre_execution_detection`
- `success_regression_guard`
- `benchmark_assertion`
- `confidence`

## Audit Categories

After extraction, reclassify rows into benchmark categories:

- `trigger_failure`: the skill should trigger and prevent a known or likely
  planning failure.
- `trigger_success_guard`: the skill may trigger; expected behavior preserves a
  successful route choice.
- `nontrigger_success_guard`: the skill should not trigger; expected behavior
  is fast direct execution/review.
- `scoping_boundary`: use `$scoping`, not `$planning-strategy`.
- `unclear_needs_raw_audit`: read the raw rollout before using the row.

Common extractor mistakes:

- marking all successful workflows as `would_planning_strategy_trigger: yes`;
- misclassifying objective/slice choice as planning-strategy instead of
  `$scoping`;
- treating a no-regression guard as a trigger case;
- missing placement failures where the problem is chat-vs-scratch-vs-artifact;
- accepting "planning happened" without asking whether it happened before
  costly execution.

## Evaluation Questions

For each audited row, ask:

- Would the skill metadata trigger correctly without reading the body?
- Does the body force the missing pre-execution detection?
- Does it put planning work in the right place: scratch, chat, charter, file,
  subagent, or stop?
- Does it make the agent continue autonomous work after a scratch strategy
  check unless stop/report criteria or a Jörn-only crux fires?
- Does it preserve the success-regression guard?
- Does it avoid ritual planning for the non-trigger rows?

Patch the skill only for benchmark failures that generalize beyond one row.

## Durable Artifacts

Keep in the skill reference folder:

- a compact audited benchmark such as `references/benchmark-v0.md`;
- this method file.

Keep out of tracked files unless explicitly needed:

- raw rollout excerpts;
- extractor scratch outputs;
- broad transcript dumps;
- large session indexes.

If provenance matters, store thread ids and rollout paths in `/tmp` or another
private scratch artifact used for review, not in the public-facing benchmark.
