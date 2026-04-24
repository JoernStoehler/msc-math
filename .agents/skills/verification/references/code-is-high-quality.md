# Code Is High Quality

Property:

- explorable
- predictable
- maintainable
- clear/simple
- mathematically honest

Starter read set:

Use these surfaces in this order:

1. `AGENTS.md` for repo-wide code placement and verification boundary rules.
2. The relevant convention skills, especially `rust-conventions`,
   `python-conventions`, `experiment-conventions`, and
   `formal-math-conventions` when applicable.
3. `ARCHITECTURE.md` for intended component boundaries and API tiers.
4. The concrete code files, tests, and nearby research/formal notes for the
   surface under review.
5. `ROADMAP.md` and `tasks/*.md` for open design decisions or explicitly
   deferred cleanup.

Checks:

1. Name the exact surface under review.

2. Entry and read path:
   - `rg -n "pub fn|pub struct|pub enum|fn main" <paths>`
   - open the main entry file(s) and trace one semantic stage
   - flag when one stage requires unreasonable jumping across files just to
     recover local context

3. Helper / indirection smell:
   - `rg -n "^fn |^pub fn " <paths>`
   - flag single-use helpers, wrappers, or generic layers that do not hide a
     real stage or remove real duplication

4. Duplicate policy / threshold / path logic:
   - `rg -n "<symbol|threshold|path fragment>" crates experiments`
   - flag repeated policy that can drift, especially when comments and code no
     longer agree

5. Boundary honesty:
   - compare public entrypoints, deep imports, and comments against
     `ARCHITECTURE.md`
   - flag experiment-only code that reads as stable API, or stable code that
     still reads like a temporary spike

6. Mathematical honesty:
   - `rg -n "\\[lem:|\\[thm:|\\[def:" crates experiments`
   - check that claimed labels, invariants, exact-vs-empirical boundaries, and
     assumptions are truthful

7. Fix shape to suggest when obvious:
   - inline / simplify
   - extract to topic-local helper
   - promote to stable surface
   - relabel as experimental/internal
   - defer because the boundary is unsettled
