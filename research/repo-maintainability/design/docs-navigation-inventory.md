<!--
Purpose: question-oriented inventory for repo navigation and orientation docs.
Context: written from concrete repo files on 2026-04-16 so later sessions can
see which common "where does this live?" questions are already answered and
which ones still need a repo-level guide or a Jörn decision.
-->

# Docs And Navigation Inventory

## Status

- Packet: D4, docs/navigation inventory.
- Date: 2026-04-16.
- Scope checked: `AGENTS.md`, `TASKS.md`, `library/src/lib.rs`,
  `library/src/database.rs`, `experiments/<topic>/src/lib.rs`, and the
  maintainability note at `research/repo-maintainability/design/main.md`.
- Output rule: `question -> current answer source -> gap -> likely future home`.

## Method / Evidence Commands

- `cd /workspaces/msc-math && pwd`
- `nl -ba AGENTS.md | sed -n '1,220p'`
- `nl -ba TASKS.md | sed -n '440,500p'`
- `nl -ba research/repo-maintainability/design/main.md | sed -n '1,260p'`
- `nl -ba library/src/lib.rs | sed -n '1,120p'`
- `nl -ba library/src/database.rs | sed -n '1,140p'`
- `nl -ba experiments/combinatorial-cells/src/lib.rs | sed -n '1,80p'`
- `nl -ba experiments/hko-local-maximum/src/lib.rs | sed -n '1,80p'`
- `nl -ba experiments/numerics/src/lib.rs | sed -n '1,80p'`
- `nl -ba experiments/numerics/gradient/src/lib.rs | sed -n '1,80p'`
- `nl -ba experiments/sys-landscape/src/lib.rs | sed -n '1,120p'`
- `nl -ba experiments/sys-landscape/gradient-ascent-dev/src/lib.rs | sed -n '1,80p'`
- `find experiments -path '*/src/lib.rs' | sort`
- `rg -n "ARCHITECTURE\\.md|docs/navigation|navigation inventory|maintainability" -S research .agents .codex AGENTS.md TASKS.md`

## Question Inventory

| Common question | Current answer source | Gap | Likely future home |
| --- | --- | --- | --- |
| What is the repo, what are the deliverables, and where do I start? | `AGENTS.md:3-37, 43-52, 92-109` | The repo map is split across several sections; there is no one-page navigation guide that ties deliverables, workspace areas, and first commands together. | `ARCHITECTURE.md` |
| Where is active maintainability work tracked, and what is the current phase? | `TASKS.md:445-495` and `research/repo-maintainability/design/main.md:12-20, 95-107, 230-237` | No gap for status. This is already the home for the program plan, packet queue, and safe resume point. | Tracker packet / no new doc needed |
| What is the public library surface for routine use? | `library/src/lib.rs:1-66` | The simple re-exports are listed, but the file does not separate stable simple surfaces from expert-only deep paths or accidental internals. | `ARCHITECTURE.md` |
| How does the JSONL cache/data layer work? | `library/src/database.rs:1-54, 114-140` and `TASKS.md:468-482` | The storage contract is clear, but the canonical shared catalog versus mirror versus transient-search distinction is still open. That is an architecture decision, not a wording gap. | Tracker packet first; `ARCHITECTURE.md` after the policy lands |
| What do topic experiment helper crates own? | `experiments/combinatorial-cells/src/lib.rs:1-4`, `experiments/hko-local-maximum/src/lib.rs:1-3`, `experiments/numerics/src/lib.rs:1-3`, `experiments/numerics/gradient/src/lib.rs:1-4`, `experiments/sys-landscape/src/lib.rs:1-6`, `experiments/sys-landscape/gradient-ascent-dev/src/lib.rs:1-9` | The headers explain topic intent, but not the extraction rule for shared logic, nor when helper code should stay per-binary. | `ARCHITECTURE.md` for the policy; file header for package-local intent |
| Which deep imports are okay for experiments to use? | `research/repo-maintainability/design/main.md:44-78` and `library/src/lib.rs:23-36` | The note records examples of deep imports, but the classification into stable/public, expert/public, accidental internal, or unclear is not finished. This is an architecture decision. | D1 import-surface packet, then `ARCHITECTURE.md` |
| Where does the repo explain library-internal architecture and the math/code boundary? | `library/src/lib.rs:6-36` and `AGENTS.md:45-52` | Mostly answered already. The current docs explain submodules, dependency direction, and the formal-file linkage. | No new doc needed |

## Observed Strengths

- `AGENTS.md` already gives a repo-wide orientation map, operating rules, and quick commands.
- `TASKS.md` already separates program status from execution packets and names the maintainability work explicitly.
- `library/src/lib.rs` already compresses the library into a readable submodule map plus dependency graph.
- `library/src/database.rs` already states ownership: experiment-owned JSONL storage with no canonical shared mutable cache path.
- The experiment `src/lib.rs` headers already tell a reader what each topic package is about, even when the helper crate is still empty.
- `research/repo-maintainability/design/main.md` already separates observed facts, open decisions, discovery packets, and the Jörn decision surface.

## Unresolved Doc-Policy Questions

- Should `ARCHITECTURE.md` be only a navigation map, or also a policy document for stable versus expert-only surfaces?
- Should data-policy language stay in tracker packets until the canonical catalog decision is settled, or should `ARCHITECTURE.md` already name the policy shape?
- Should topic helper extraction rules live centrally in `ARCHITECTURE.md`, or remain mostly in topic-local file headers until the shared-helper packets finish?

## Next Safe Resume Point

- Use this inventory as the D4 evidence base for the repo-maintainability program, then turn the rows that are still architecture decisions into tracker packets instead of drafting `ARCHITECTURE.md` prematurely.
