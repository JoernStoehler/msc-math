# Repo Promises Are Truthful

## Use When

Use this packet when the task is to check whether the repo really matches what
the thesis or repo-level deliverable surfaces promise about:

- available components and paths;
- build, test, smoke, or rerun commands;
- reproducible versus preserved computational artifacts;
- repo-facing statements such as "Rust library", "reproducible pipeline", or
  "future research can resume from local artifacts".

This packet is useful before submission. Run it whenever thesis-facing repo
claims, top-level deliverable wording, or experiment rerun promises are being
edited or relied on.

## Authority And Scope

Use these surfaces in this order:

1. `research/INDEX.md`, topic research notes, and `tasks/*.md` for
   thesis-facing deliverable and infrastructure obligations.
2. `tasks/verify-thesis-done.md`, especially the references/provenance and
   repo-promises final gates.
3. `AGENTS.md`, `crates/MAP.md`, and `experiments/MAP.md` for current repo maps
   and intended durable component boundaries.
4. The concrete code paths, scripts, datasets, and commands that the promise
   points to.
5. `ROADMAP.md` and `tasks/*.md` for known stale-data, rerun, or packaging
   caveats.

If the current repo state and the promise disagree, do not silently interpret
the promise more weakly. Report the mismatch and either weaken the wording or
name the concrete fix.

## Procedure

1. Name the exact promise or deliverable surface under review.
2. Classify the promise type:
   - component/path existence;
   - command succeeds;
   - rerunnable artifact;
   - preserved artifact;
   - semantic truthfulness of a repo-facing description.
3. Inspect the actual target surface and classify the promise as:
   - `supported`
   - `supported only with explicit prerequisites or caveat`
   - `stale wording`
   - `missing artifact or command path`
   - `reproducibility gap`
   - `Jörn decision needed on whether to weaken or keep the promise`
4. When the promise is command-shaped, record:
   - exact command;
   - scope actually checked;
   - prerequisites not encoded in the promise;
   - whether the repo should promise rerun or preserved-artifact wording.
5. Report findings first with file paths and the concrete mismatch.

## Ask Jörn Only For

- whether a repo-facing promise should be weakened, cut, or kept despite an
  identified cost or prerequisite;
- whether a thesis-facing artifact should be treated as rerunnable output or as
  preserved record;
- taste/framing decisions about how prominently to advertise repo reuse.

Do not ask Jörn to locate the concrete command path, inspect the generated
artifact, or compare the promise wording against the actual repo surface.

## Output Shape

Prefer findings first in severity order. For each finding, say:

- the promise or deliverable statement;
- status;
- authority surfaces checked;
- concrete repo path / command / artifact checked;
- missing prerequisite, stale wording, or reproducibility gap;
- whether the fix is agent-doable or Jörn-only.
