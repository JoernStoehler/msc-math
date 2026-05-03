# Planning And Verification

Use this file for repo-local planning and verification behavior that is broader
than one skill or task bundle.

## Plans

- For tasks with more than one concrete change or one verification step, keep a
  plan with objective, dependency, owner, and verification command or review
  check.
- Include a quality gate in the plan. Use subagent review when Jörn asks for
  delegation or the active session instructions allow it; otherwise run a local
  review against the same checklist.
- Update the plan after meaningful results. Do not leave stale statuses.

## Planning Surfaces

Route planning surfaces explicitly:

- `research/INDEX.md` and `research/*.md`: thesis story interpretation,
  proof-route state, and research caches
- `tasks/verify-thesis-done.md`: once-run final thesis-done gate
- `ROADMAP.md`: overview and routing surface
- `tasks/*.md`: topic mini-roadmaps and cached task knowledge

Do not put repeated quality workflows, intermediate milestones, or
`writer-ready` / `submission-ready` / `freeze-ready` acceptance detail into
`tasks/verify-thesis-done.md`. Put reusable checks in the `verification` skill
and topic-specific obligations in `tasks/*.md`.

If an intermediate milestone needs durable multi-session acceptance criteria but
is still not part of thesis-done, create a separate planning or milestone file
instead of extending `tasks/verify-thesis-done.md` by default.

## Before Asking For Human Review

Before asking Jörn to review a draft, proof sketch, experiment write-up, or
conclusion, first run the checks that agents can run:

- buildability
- internal consistency
- source attribution
- figure/text alignment
- claim/data alignment
- label/cross-reference resolution
- missing tests
- scope drift
