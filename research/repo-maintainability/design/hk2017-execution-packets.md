<!--
Purpose: execution-kickoff note for the shared capacity/orbit result refactor.
Context: this lives on the integration worktree branch so later subagent
sessions can branch from the same parent state and report back into one packet
queue. It is intentionally incremental: only the first packets are defined.
-->

# Capacity/Orbit Execution Packets

## Status

- Parent branch: `capacity-result-api-exec`
- Parent worktree: `/workspaces/msc-math/.codex/worktrees/capacity-result-api-exec`
- Source design note:
  `research/repo-maintainability/design/hk2017-result-api-plan.md`
- Planning rule: do not freeze the whole DAG up front. Add packets as earlier
  packets land and expose the next dependency surface.

## Integration Model

- This worktree is the integration trunk for the refactor.
- Later subagent worktrees should branch from `capacity-result-api-exec`, not
  from root `main`.
- Each packet should end with:
  - code integrated back into this worktree
  - packet-local verification run here
  - tracker/doc update here if the landed code changes the approved shape

## First Packets

1. **Core types and naming scaffold**
   - Scope:
     - add the shared enums/types for guarantee mode, backend, admissibility,
       orbit payload, and search result
     - add/co-locate the shared error enums
     - no broad caller migration yet
   - Why first:
     - every later packet depends on the names and field shapes existing in
       code
   - Verification:
     - `cargo build -p symplectic --release`
     - targeted library tests for touched modules if any compile-time glue needs
       updates
   - Stop condition:
     - if `mu`/`xi` optionality or public-module placement becomes unclear

2. **Shared search frontend surface**
   - Scope:
     - introduce shared collector entrypoints for `hk2017`,
       `hk2017_unpruned`, and `billiard`
     - wire guarantee/backend parameters through the search layer
     - keep old wrappers only as staging aids if needed
   - Why second:
     - this establishes the real architectural seam before consumer migration
   - Verification:
     - `cargo build -p symplectic --release`
     - `cargo test -p symplectic --release --lib`
   - Stop condition:
     - if backend plumbing forces a solver-semantics change instead of an API
       refactor

3. **Derivative/subdifferential library helpers**
   - Scope:
     - add `OrbitGradientA`
     - add orbit-level derivative helper(s)
     - add primitive Clarke-subdifferential helpers on orbit lists
   - Why third:
     - this gives immediate leverage for the duplicated experiment logic
   - Verification:
     - derivative-related library tests
     - build at least one derivative-heavy experiment package
   - Stop condition:
     - if helper shape depends on unresolved consumer ergonomics not covered by
       the design note

4. **First consumer migrations**
   - Scope:
     - migrate one geometry/orbit consumer
     - migrate one derivative/subdifferential consumer
     - prefer consumers with clear existing verification surfaces
   - Why fourth:
     - proves the API is actually usable before broader migration
   - Verification:
     - targeted experiment build(s)
     - packet-specific smoke checks
   - Stop condition:
     - if migration reveals a missing core type/field rather than a local bug

## Immediate Next Action

- Start Packet 1 in this worktree.
- Once Packet 1 is scoped precisely, branch subagent worktrees from this branch
  only if the packet splits into disjoint write scopes.
