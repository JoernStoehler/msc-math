# Project documentation

This directory contains project-wide information. Topic-local proofs, evidence,
implementation details, and thesis prose remain with their topic.

[Current work and ownership](../memories/todos.md) records the live assignment
and distinguishes proposals from accepted work. [Migration review](migration-review.md)
holds the supporting review notes, not a separate queue.

| File | Purpose | Authority |
| --- | --- | --- |
| `project-facts.md` | Jörn-confirmed project facts and accepted external constraints | current unless newer Jörn/Kai/source truth contradicts it |
| `reproducibility.md` | thesis-facing code/data/archive route | policy and entry point; exact commands and artifacts remain producer-local |
| `artifacts.md` | shared R2 materialization and publication contract | current artifact workflow and per-environment XDG cache contract |
| `development-environments.md` | host, Docker Sandbox, and Codex Cloud execution model; clients and shared toolchain contracts | dated verification and remaining gaps; host `~/.dotfiles/memories/host-estate.md` owns sandbox operations |

## Historical agent guidance

Former project skills and steering were removed from the working tree because
they were already quarantined, had no identified active consumer, and added
obsolete guidance to searches. Their complete tracked contents are available
at commit `892fc9ab3b3c242cd961cfa469f6d81cef0d626d` under
`.agents/legacy-skills/` and `.codex/legacy-steering/`.

```bash
git ls-tree -r --name-only 892fc9ab -- .agents/legacy-skills .codex/legacy-steering
git show 892fc9ab:.agents/legacy-skills/README.md
```

Use `git show <commit>:<path>` to read an individual historical file. These
files are fallible recovery material, not current instructions.

The former `.worktrees/migration-scratch` proposals are preserved separately on
branch `archive/migration-scratch-2026-09-05`, commit `c14e33c8`. Useful repairs
were transferred individually; the archive's AGENTS.md and model policies were
not adopted. Its old PDF/log and remaining generated files are preserved at
`/home/joern/.local/share/msc-math-recovery/migration-scratch-2026-09-05.qeW9jF/`.

Add a project-wide file only when the project itself owns the fact or policy.
Do not choose a home merely because it is the narrowest directory containing
one classification of the material. Prefer updating or deleting an existing
file over adding another overlapping summary.
