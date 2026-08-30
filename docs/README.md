# Project documentation

This directory contains project-wide information. Topic-local proofs, evidence,
implementation details, and thesis prose remain with their topic.

| File | Purpose | Authority |
| --- | --- | --- |
| `project-facts.md` | Jörn-confirmed project facts and accepted external constraints | current unless newer Jörn/Kai/source truth contradicts it |
| `project-status.md` | milestones, current integration state, and unresolved gates | current project-state view, not mathematical evidence |
| `capabilities.md` | compact cross-domain view of supported capabilities and important boundaries | navigation only; confirm every claim at its named source |
| `reproducibility.md` | thesis-facing code/data/archive route | policy and entry point; exact commands and artifacts remain producer-local |
| `artifacts.md` | shared R2 materialization and publication contract | current artifact workflow and per-environment XDG cache contract |
| `development-environments.md` | host, Docker Sandbox, and Codex Cloud execution model; clients and shared toolchain contracts | current operational model; sibling host runbooks own sandbox and Paseo administration |

Add a project-wide file only when the project itself owns the fact or policy.
Do not choose a home merely because it is the narrowest directory containing
one classification of the material. Prefer updating or deleting an existing
file over adding another overlapping summary.
