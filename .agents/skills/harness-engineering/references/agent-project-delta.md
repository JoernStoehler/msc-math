# Agent Project Delta

This note is about the delta between design heuristics for ordinary human-team
projects and for projects that will be read and edited mainly by AI agents.

It is not an ab-initio theory of teamwork. Agents already have a lot of
implicit knowledge about human-team defaults. The useful extra content is the
delta: what changes once future readers are cheap, fast, usually do not carry
reliable memory from one session to the next, and are bad at some kinds of
independent judgment.

## Core Shift

Human-team heuristics often optimize for:

- expensive repeated reading
- expensive repeated edits at many call sites
- durable human memory of the project surface
- putting shared insights in one place so the team does not need to rediscover
  them

Agent-team heuristics often optimize for:

- it is often cheap for an agent to re-derive a local fact when it needs it
- agents often start without reliable memory of earlier sessions
- it is often cheap to copy an insight into several files
- writing down the few implications that would otherwise need real reasoning
  time
- files where it is easy to predict what to read next and where to edit

So line count, diff size and duplication are weaker proxies than they are in ordinary
human-team advice.

## Use Observed Friction, Not Counterfactual Theater

Because agents have weak introspection about unrecorded reasoning, they are bad
at telling whether some explanation or abstraction was truly needed, or whether
they would have derived it quickly on their own.

So agent-heavy projects should usually avoid premature optimization of the code
surface. Wait for real friction to appear, then address that friction directly.

Prefer observable evidence such as:

- a task needed deep planning before edits could begin
- a task needed several trial-error iterations
- the agent had to open many files before it could act safely
- the agent had to ask Jörn because the local surface was too hard to reason
  about
- the same implication had to be re-derived in several sessions
- review found confusion that should have been cheap to avoid locally

Be skeptical of claims like:

- "future agents will probably be confused by this"
- "this abstraction might save reasoning later"
- "the code feels like it should be cleaner"

unless they can be tied to observed costs.

In practice, ask:

- did I actually have trouble understanding this surface?
- what concrete cost showed up: planning time, search breadth, retries,
  escalation, or review friction?
- am I describing my own observed cost, or speculating about a hypothetical
  weaker reader?

The practical rule is: optimize first for observed friction on real tasks, not
for imagined weakness in some future reader.

## Why Human Defaults Transfer Poorly

Humans naturally build durable internal documentation from repeated contact
with the same code. This makes centralized abstractions attractive: the team
gradually learns the shared helper and reuses that memory later.

Agents do not work like that. They often either:

- read, reason, act, and then drop the reasoning, or
- externalize the reasoning into code, comments, names, or docs

This changes the tradeoff:

- centralizing an insight into one abstraction is less valuable than for humans
- repeating an insight where it is needed is cheaper than for humans
- a local explanation copied and specialized in three files can be better than
  one abstract helper plus two extra jumps
- local ownership often beats clever centralization when future edits are
  likely to be local and variant-specific

Human teams often optimize for their weakest member because the members differ
in background, taste, and project memory. Agent teams are different: most
agents share roughly the same training prior and differ much more in local
context than in baseline expertise.

This is why "complexity that's bad for ephemeral agents" is not quite the same
as "complexity that's bad for stable human teams".

## What Usually Costs Agent Time

The things that usually cost agent time are not reading, typing, or copying.
They are:

- deep reasoning and long trial-error loops
- nested indirection that interrupts search, editing, or reasoning flow
- unstated implications that must be repeatedly re-derived before safe edits
- situations where the agent must open and understand more files before it can
  safely start the main work
- code that is hard to explain with a short local summary that stays correct
  while the agent edits
- edit surfaces that are hard to split into disjoint ownership
- self-reflection about unrecorded reasoning, especially counterfactuals about
  what information was actually necessary

One useful distinction here is:

- hard dependency:
  file or abstraction B must be opened and understood before doing the main
  work in A
- soft dependency:
  work on A can proceed locally, and an insight from nearby code can later be
  copied, adapted, or checked without forcing a larger active context now

For agent-heavy projects, hard dependencies are often much more expensive than
repetition or later transfer. Broad but loosely coupled work is often cheap.
Broad coupled context is not.

## What Is Usually Cheap For Agents

This list mixes execution costs, editing costs, and session-level costs. These
things are often cheap enough that human-style pressure to centralize them
should be weaker:

- rereading a full file, a diff, or command output
- running standard commands
- writing routine glue, boilerplate, or straightforward code
- making both surgical and large edits
- iterating on a file until the local quality improves
- scanning long output and picking out the relevant part
- standard reasoning patterns once the pattern is visible
- working through long checklists or broad task surfaces when the pieces are
  loosely coupled
- explicit audits of tool-call history and recorded artifacts
- starting fresh with no prior session memory
- copying an architectural hint into several files when that reduces later
  re-derivation

## Heuristics

Prefer these when optimizing for agent readers:

- Local pipelines where the next step is obvious from the code.
- Semantic primitives such as `A`, `B`, `C`.
  Example: expose `load_polytope`, `classify_facets`, and `search_orbits`
  instead of a partial wrapper like `load_and_classify` when later variants are
  likely to do `load -> classify -> search` in one file and
  `load -> classify -> sample -> search` in another.
- A full wrapper `A -> B -> C` when many callers want exactly that.
- Repeated local comments when they save real re-derivation.
- Comments that say what pattern the reader is looking at when the pattern is
  uncommon.
- Comments that hand the reader the important implication when deriving it is
  nontrivial.
- Files where it is clear who owns what and what happens next.
- Nearby helper layers that can be read later rather than before the first safe
  edit.

Be skeptical of these:

- partial wrappers like `AB` whose main job is to reduce repeated simple code
- context objects or state wrappers that add nouns without protecting a strong
  invariant
- shared glue that centralizes text but hides real differences between callers
- abstractions that make `A -> B' -> C` harder than writing the pipeline
  locally
- centralization done mainly because copying an insight to several files feels
  untidy

## Pipeline Rule Of Thumb

Suppose many experiments use variants of the same flow:

- `A -> B -> C`
- `A -> B' -> C`
- `A -> B -> D -> C`

Then the likely good shape is:

- expose `A`, `B`, `C`, maybe `D` if semantically real
- maybe expose a full `A -> B -> C` wrapper for callers who truly want the
  whole thing
- avoid filling the codebase with `AB`, `BC`, `ABD`, `ABCContext`, and similar
  partial abstractions

Reason:

- future variants usually need to think about `A -> B` at least once
- `A`, `B`, and `C` are often clean units of thought
- `AB` is often a weaker unit that saves text but not reasoning

## Comments And Names

Comments are valuable when they reduce reasoning that would otherwise have to
be redone.

Good uses:

- name the pattern:
  `// Straight pipeline: load -> classify -> search -> report`
- warn about an uncommon pattern:
  `// Unusual pattern: retry in f64 first, then fall back to algebraic numbers
  // only when error bounds cannot decide the predicate.`
- spell out the implication the next reader will need:
  `// Zero coordinates stay zero under wiggle because the perturbation is
  // multiplicative.`

Bad uses:

- comments that restate obvious syntax
- comments that are longer than the reasoning they save
- comments for patterns the typical agent already recognizes immediately

## Duplication

Duplication is not automatically bad.

Treat duplication as a problem only when it causes observable friction such as:

- future edits must be mirrored in several files and are easy to miss
- verification burden is materially higher because the same behavior lives in
  several places
- the repeated code hides a truly semantic shared unit that would make the
  local pipelines easier to read
- the duplication creates hard dependencies anyway because every meaningful
  change requires reopening all copies together

Otherwise, duplicated local glue can be the simpler system.

## A Better Question Than "Should This Be DRY?"

Ask:

- What would a future agent have to re-derive before making a safe edit here?
- Can that be made local and explicit?
- Does this abstraction remove reasoning, or only move text?
- If a future caller wants `A -> B' -> C`, is that easier or harder after this
  refactor?
- Did this change reduce dependency edges and surprises, or add them?

## Short Examples

Better for agents:

- three experiment files each own their own short pipeline, but all call the
  same library primitive for the mathematically meaningful step
- two files repeat the same architectural hint in comments because each file is
  commonly read alone
- a wrapper object is removed because every caller immediately unpacks it

Worse for agents:

- a large shared runner hides the real differences between experiments just to
  reduce line count
- a helper is extracted even though later edits will have to open that helper
  and both callers anyway
- a comment is deleted as "redundant" even though it carried the non-obvious
  implication future readers actually need

## Practical Editing Bias

When simplifying for agent-heavy work, prefer:

- removing fake abstractions
- reducing indirection
- replacing hard dependencies with soft ones when the local owner can carry the
  relevant insight directly
- making local pipelines explicit
- repeating small insights where they are consumed
- extracting only semantically real primitives

Do not optimize for:

- minimal line count
- maximal deduplication
- centralized ownership of every idea
- human-style aesthetic neatness when it increases future agent reasoning
