# Thesis Development Notes

Status: maintainer-facing notes for agents working on `thesis/`.

Purpose:
- preserve process knowledge from the scaffold-design session;
- transfer what was discussed to agents that do not have the session log;
- avoid turning the discussion into a rigid process or reusable prompt.

Do not treat this file as a process contract. Use it as context.

## Current Scaffold State

`thesis/planned-toc.md` was converted into active `.tex` scaffold files.
`thesis/main.tex` inputs the scaffold files. Old thesis prose and thesis-local
notes were moved to `thesis/legacy/` as source material only.

Scaffold files may intentionally contain only headings, labels, and local
TODO/context comments. They are meant to let Jörn start writing, spawn
thesis-local tasks, ask agents where text needs fleshing out, and decide ad hoc
whether Jörn writes, an agent attempts work, or an agent only gathers
information.

## Section-Local Content Companions

Some thesis sections may have a nearby `*-content.md` companion file. These
files gather section-local result packets, source pointers, evidence status,
missing non-writing work, fallback branches, and review gates before final prose
is written.

Treat these files as writeup-gathering notes, not source truth. They should
state their own status, purpose, overruled-by sources, lifecycle, and update
rule. Prefer source pointers and explicit `needs source` markers over dangling
claims. Keeping the header local to each companion is intentional; it avoids
indirection and lets each file have its own lifecycle.

Use a companion file when it reduces source scatter or prevents a section from
losing thesis-facing content. Do not create one merely to record transient
reasoning that is cheaper to rederive.

## Questionnaire And Answers From The Design Session

### QJ1. Minimum thesis story

What is the smallest thesis story that would still feel acceptable to submit?

Candidate answer shape:
- main story;
- optional side stories;
- results that may be cut or moved to future work.

Why this matters:
- It determines which claims must be supported and which can be weakened or
  cut.

Jörn answer:

see planned-toc iiuc. i think it's okay if the datascience story gets
invalidated by a sudden positive result - and yet we don't follow up on it just
note it for future work. not sure what else to say here, the question is quite
high and it's hard to articulate how content "sums" up its utility / what each
component / what levels of quality there are everywhere. it's somewhat
heterogenous so i fail to come up rn with flat statements. e.g. numerics is
very unimportant in the thesis and turned out in hindisght to also not matter
much for many of the experiments (despite the sheer amount of work put into
it). instead it's more relevant that the few proof-by-computation use sagemath
(trusted/known) for verification of a witness and rust (fast, high control over
what happens) for generation of the witness. e.g. rust spits out algebraic
numbers in Q[tan(pi/5)] or whatever, and we don't even need correctness proofs
of that - because all we care is that *this one run* produces a witness that
sagemath can check (e.g. it checks linear equations, inequalities, etc etc). I
am tbh not sure rn whether planned-toc is complete - i need to check (i.e. you
need to list its contents quickly so i can tell you whether any aspect isn't
named/mentioned yet, then I can also check whethe rnuances are correctly
captured)

Follow-up from session:

The planned TOC was listed back to Jörn. Jörn said: "The TOC looks right to me!
Thx."

### QJ2. Biggest current failure risk

What is most likely to kill submission if not handled soon?

Candidate answer shape:
- missing prose;
- unsupported or overstrong claims;
- missing experiments story;
- missing final verification;
- external submission/admin issue;
- dunno.

Why this matters:
- It chooses whether agents should first draft sections, audit claims, write
  the experiments story, or run final-gate preparation.

Jörn answer:

timing :3 - missing prose seems like the highest uncertainty rn with how to
write quickly and "good enough". the experiments are all such that they can be
kicked off imo - but yes, they can unexpectedly take 2 days longer in the
background -- important is that i parallelize workloads such as "jörn types
thesis prose" "codex codes and runs and debugs in bg" "jörn chats with codex"
"codex reviews prose" "codex offers multiple figures" "codex accelerates
writing by providing drafts/lists/claim-maps/reviews/manages todos/reminds jörn
to move fast and polish later/etc"

### QJ3. Current included thesis text

Is the current included thesis text mostly salvageable, mostly stale, or mixed
by section?

Candidate answer shape:
- mostly salvageable;
- mostly stale;
- mixed, with examples;
- dunno.

Why this matters:
- It chooses between patching current text, cleaned active structure, or fresh
  drafting.

Jörn answer:

mostly stale, but highly mixed even on a paragraph-by-paragraph level. worth
keeping around for quickly absorbing old knowledge which helps absord the (more
spottily documented) new knowledge

### QJ4. Experiment story readiness

Are the underlying experiment results and interpretation ready enough for
agents to write a thesis experiments section now?

Candidate answer shape:
- yes, draft from existing research/tasks/artifacts;
- partly, but one named result/interpretation is missing;
- no, experiments story is still undecided;
- dunno.

Why this matters:
- The active thesis has an experiments stub, and the thesis topic depends on
  computational evidence and interpretation.

Jörn answer:

no (as usual: for some experiments yes, for some not; overall: we are like 90%
done but the last 20% round up the entire thing; i.e. labor: 90% done, target
conjecture: known, evidence-for-thesis: 50% done [the last 10% labor cross 50%
of readers' view distance], writing: ~10% done (concepts, but not even fleshed
out / narrated well / lots of context isn't in the repo such as the 'why'
behind picking some approaches or the motivation / the discussions with Kai and
Jörn).

### QJ5. Jörn-hours

How many high-quality Jörn-hours are realistic between 2026-05-18 and the
deadline?

Candidate answer shape:
- 4-8;
- 8-16;
- 16-25;
- more;
- dunno.

Why this matters:
- It determines how aggressively agents must decide, weaken, or cut without
  asking Jörn.

Jörn answer:

high quality: i can do 8h per day just fine sustainable (7d per week) -- but
the probelm is that my time management isn't great and i run into unimportant
side tasks / don't parallelize with codex' labor well enough as to not ever end
up waiting on codex (waiting 3600s is quite bad use of my time after all)

### QJ6. Old material discoverability

Do you expect agents to find old relevant material section-locally, or are
important dependencies often hidden?

Candidate answer shape:
- section-local search should usually work;
- hidden dependencies are common;
- depends on topic, with examples;
- dunno.

Why this matters:
- It chooses between local source checks and bounded migration spikes.

Jörn answer:

hidden/scattered I think - that's why auxiliary maps help and a rewrite/moving
material around would help.

### QJ7. Claim weakening policy

When a proof, experiment, or exact validation route is expensive, should agents
default to weakening/caveating/cutting the claim unless it supports the main
story?

Candidate answer shape:
- yes by default;
- only for side claims;
- ask Jörn first for theorem-strength claims;
- no, preserve stronger claims if possible;
- dunno.

Why this matters:
- It controls proof/code rabbit-hole risk.

Jörn answer:

no they should just distinguish between the "predicted" target at thesis
submission (we will have a proof that ... because we already have the proof
strategy ... and know it's feasible to flesh out) and the is-state (the proof
strategy is written up in ... and next up is testing whether agents know how to
complete the proof given the strategy or whether jörn has to provide more
expert-reasoning help / more prompt-engineering to utilize gpt 5.5's
capabilities / prevent corner-cutting and other failure modes that agents can
have)

### QJ8. Sections most worth first attention

Which active thesis section should agents touch first for maximum thesis
success?

Candidate answer shape:
- introduction;
- experiments;
- sys first-order section;
- proofs;
- numerical appendix;
- other;
- dunno.

Why this matters:
- It selects the first concrete agent task.

Jörn answer:

unsure what you mean with "touch" - if you mean "write polished prose" then
none bc that takes a large amount of review time and baserate experimetns in
the past showed gpt 5.5 has bad writing habits for polished pubication ready
text (despite having read more papers than any human -.- -- probably RL
reinforces bad habits that cut cornrers or bedazzle bad readers, and
counteracting habits is hard for gpt 5.5 bc again RLHF/RLVR does not reward
such behavior all that muhc (at least i think it reinforces >0 to error-correct
written code+comments, and that generalizes to writing prose, but ugh,
speculation). so knowledge-transfer / structuring text / identifying the most
difficult spots is a better type of labor gpt 5.5 can do -- e.g. it can fill a
whole thesis with nothing but "TODO: define polytope, and polytope with zero in
interior; TODO: next, define polar dual polytope; TODO: next define algorithm
to check whether zero lies in the interior; TODO: non-rigorous remark about
numerics; TODO: algorithm to check whether a set of vertices equals the extreme
points of their convex hull; SUBSECTION: Sample-able random polytope families
we use; TODO: explain rejection sampling as our approach; TODO: define the
first family as F = given parameter, a_k ~ Uniform([0.8, 1.2]) *
Uniform(S^3), accept if extremal & contains zero; ...; TODO: table with
acceptance & rejection rates  for different values of F and different
families; ...

### QJ9. Advisor risk

What would Kai or Elizabeth most likely object to in the current thesis state?

Candidate answer shape:
- missing story;
- weak mathematical proof support;
- unclear computational evidence;
- presentation/readability;
- scope mismatch;
- dunno.

Why this matters:
- Advisor objections should affect process weighting.

Jörn answer:

eh, it's an empty thesis / incomplete. i don't think going into more details
for the rejection would be informative rn (?)

## Follow-Up Questions And Answers

### Q1. What is the best first agent task?

Candidate options:
- Q1a make a thesis-wide scaffold/TODO map from `planned-toc.md`;
- Q1b make a section-by-section claim/support/current-state/target-state map;
- Q1c focus on experiments story structure first;
- Q1d build a parallel work queue for Jörn-writing plus Codex-background tasks.

Jörn answer:

cleanup, scaffolding, high-level todos that are quick to review (such as
splitting hte thesis into .tex files that are all empty except for section +
label, a copied-over TODO paragraph that describes the section [if available!]
and ofc the \input in main and the basic helpers/config in preamble.tex
bibliography.bib etc.) So the target state i think of the next session (for
which we have to write a prompt) is that i can start just asking an agent to
tell me where to flesh out text more, and then i can decide whether *I* collect
claims and/or write prose, or whether I let an agent/subagent do an attempt [in
a worktree] and then i look at whether to merge / follow up / or
discard-and-do-myself the work.

Importantly that means everything is documented etc so that agents know what
state to expect for the thesis:
- mostly empty
- a mix of polished print-ready, somewhat polished, unpolished prose that's
  content-focused, not-really-prose that's just about inventory / about
  exploring structure and deciding what to kick out / keep in / what to say in
  what order with what interconnections / where to place what figures + tables,
  and promises (todos) that document what remains to be done (including ofc
  naming the current stage which implies what remains to do wrt what other
  stages to pass through). There is no formal process here, Jörn can just jump
  from nothing to typed-up polished print-ready prose in one commit, or maybe
  ten commits iterate on the mere inventory bc it's so difficult.
- yeah I'd really stress the heterogenity and how our process is not known to
  be the best process in any situation (or rather, i'd bet against it), muss
  less having the same parameters (difficulty, amount of labor,
  passes/iterations needed, length/verbosity, ...) for every
  word/sentence/paragraph/section.
- similarly preamble and toc can change, but those are at least more
  stable/simpler than writing publicationr-eady text!

So i think this maps to Q1a followed by Q1b in the sense of copying/moving
knowledge we already have but not trying to combine moves with novel reasoning
(the mixup will just get confusing, and probably I need to again be interviewed
a lot for novel reasoning about thesis inventoriy items os that's not even sth
the agent can do rn in one go)

Q1c is not in the next first session -- it's something *implied* by todos once
those todos are broken down. like, writing the section on how data science
failed has a a todo about making a big table with all experiments / data
science methods -- and then it's obvious that there are still methods that are
not even run yet -- at which point i can just define them and ask a codex agent
to go through them one by one and run them and record the result and update the
inventory with the result. Which is nice, bc it is bg work and bc gpt 55 is so
familiar with standard data science methods -- so it should know what to do
with little upfront explanation / during-task help / post-task review needed
from me.

Q1d this is implicit imo - better to just grep for all TODO items and
prioritize ad-hoc / leave breadcrumbs where ad-hoc reasoning isn't fast enough
to correctly estimate importance. adding a map layer just grows stale way too
qucikly / adds friction (bc one can no longer just edit a .tex file). Unsure -
maybe a high-level map file + the source-of-truth arbitrary-level TODO items is
better. feels premature to decide AND feels like a process compontent that will
switch as we work bc say suddenly complexity emerges or vanishes and needs
change for what i need agents to do wrt process/project/jörn-time
management/prioritization help.

### Q2. Should `planned-toc.md` become the controlling structure now?

Candidate answers:
- yes, use it as current intended thesis structure;
- mostly, but first compare it to active `thesis/main.tex`;
- no, it is only a source.

Jörn answer:

i mean, it should be turned/converted into the .tex files. so that it becomes
redundant bc the sections then exist and the todos copied the descriptions of
what goes where content-wise / thesis-success-wise. Does that make sense? Like,
ideally we should delete it at the end of next session bc it has become
obsolete / 100% redundant.

### Q3. What should agents produce for each planned section?

Candidate answers:
- TODO scaffold only;
- claim/support/current-state/target-state map;
- rough prose draft;
- source extraction notes;
- a mix, but no polished prose.

Jörn answer:

confused question. I will decide ad-hoc where to use agents vs do things myself
- agents have sadly demonstrated they cannot estimate in-advance the difficulty
of say writing prose or making a educationally valuable inventory of
things-to-say, and they even are bad in review here. i will try to see if i can
get gpt55 to stop behaving badly and use its latent writing capabilities (it
has as said read and next-token-predicted a vast amount of high quality
scientific writing, so the capabilities are there somewhere, just somehow
blocked by bad habits trained on top :/). So agents do one or more of those
things, or ofc neither bc they don't touch the thesis itself. The *next* agent
will only do scaffolding, including copying the TODO items we already have so
that task management becomes more local / in-place rather than scattered
throughout tasks/ and planned-toc.md and research/ etc. But copying is just one
type of TODO work, other future agents will for example break down TODOs, or
interview me to resolve ambiguoties in the TODO formulation, or to gather
data/expertise that is needed to work through the TODO.

### Q4. What is the right unit for parallel work?

Candidate answers:
- one agent per planned TOC section;
- one agent per story area: algorithms, HKO, data science, polygon products,
  appendices;
- one integrator plus background workers;
- no parallel thesis editing, only parallel support work.

Jörn answer:

again, there won't be one pattern. The main way i think about this is:
- do we have tasks that are formulated in a closed objective way (i.e.: are
  verifiable s.t. jörn doesn't have to spend a lot of time reviewing / only has
  to review once / only has to review certain aspects once that gpt 5.5 cannot
  do on his own / s.t. there's minimal Jörn-time needed before/during/after the
  task?)
- how much jörn-attention is rn consumed => usually this is sth like
  - one writing task that's active for Jörn
  - a tab with all active agents so Jörn can respond once an agent pings Jörn
    (final turn message or AskQuestionTool ) -- Jörn answers with delay ofc
    instead of dropping whatever he's doing once the ping happens
  - so basically the fomrual is "minutes per hour spent writing = 60 minus
    minutes per hour spent talking with agents"
- thn there's just the normal VoI concept: how valuable is 10s context
  switching overhead + 10s to 10min of synchronous chatting-with-agent ? how
  valuable is 10s context switching overhead + 1min to 60min of uninterrupted
  writing done by Jörn [with mere light agent-chatting on the sight such as a
  writing assistant that does live-commentary which is sth that the
  agent-stream-managing agent needn't consider explicitly since it's just a
  tool that Jörn uses himself]
- so sometimes it's worthwhile to have like say 10 data science experiment
  sessions open in the background bc they require 1min of babysitting per hour
  [assuming their tasks are already well defined in the .tex TODOs + prompt
  snippets]
- and sometimes there's just not anything worthwile to run, so like, mayb
  ehtere's 2 sessions that need 2min of wrapup each (e.g. merge review + merge
  + chores about TODO updates once merged) but that lie in Jörn's inbox for
  20min doing nothing bc he's busy writing and context switching isn't worth
  it, and no active agent at all is running at the time.
- also worth noting: writing up TODOs such that an agent becomes start-able
  autonomoulsy is work that needs to be done, usually by Jörn even bc gpt 5.5
  is okay at prompting gpt 5.3 models but hasn't quite been trained well to
  deal wiht gpt 5.5 yet for complex tasks. so that's another 2-10min cost
  usually before an autonomous session *for an uncommon / custom / complex
  task* can be even started in the background. For repeated tasks such as
  "review this prose block" that 2-10min cost amoritizes bc the same task can
  be spawned like several times, so effectively it's a 12s-1min overhead only.
  For standard tasks such as "fix latex errors" or "explore repo and gather
  relevant information into a qucik report" the prompt cost is tiny as well
  since it's just literally typing the standard request , plus adding reusable
  boilerplate around output-format / conventions / review-gates to catch
  errors/catch corner-cutting/catch misinterpretations.

### Q5. For current-state vs target-state, what labels should agents use?

Candidate answer shape:
- target: expected thesis submission state.
- current: what exists now.
- gap: what must happen.
- route: agent/Jörn/background experiment/SageMath/future-work.

Jörn answer:

i generally am not a fan of fixing labels *before* writing a lot of items.
Categories were made for labeling text, not for writing text. So like, use
prose, and only if prose isn't working (e.g. if grep really doesn't work
anymore to locate items, or if there's a need to programmatically extract
semantics) should we switch to standardized syntax (such as tags / yaml
metadata / extra companion files / etc). So it's more important to define
ad-hoc the epistemic standard / the labor that was already done, without
prescriping a process for what comes after. Basically: there's no need rn to
reason about what comes after until thesi spublicaiton, any agent can do that
ad-hoc easily, but future agents cannot just guess the history of a todo item /
cannot just take a statement "X is true" and know whether that's repeatedly
checked by multiple agents / a singel agent's takeaway / a mere guess /
aspirational target-state description / a pure made-up hallucination that isn't
even rooted in any source-of-turth / something Jörn said with confidence /
something Jörn guessed.

### Q6. Which content is allowed to be marked future work even if a positive result appears?

Current known answer before asking:
- data-science sudden positive result may be noted for future work.

Unknown before asking:
- whether this also applies to polygon products, visualizations, extra
  numerics, or side experiments.

Jörn answer:

i am not quite sure what you mean? what'd be a positive result for polygon
products? Afaik the only place where i am not confident in the conlcusion we
marged as target result is the datascience experiment bc while most attempts so
far were negative (no pattern recognized beyond gradient ascent results) there
is the potential that suddenly we have some pattern (like, imagine suddenly
there's a quadratic regression function in the incidence matrix matrix
invariants that predicts the systolic capacity well) and then we can no longer
say "well, ntohgin worked" btu have to say "lots of things didn't work, but
here's somet things somebody should follow up on"

### Q7. What is the minimum useful experiments story?

Candidate answer shape:
- Q7a witness generation plus SageMath verification;
- Q7b HKO local maximum evidence;
- Q7c data-science negative result;
- Q7d random/gradient search story;
- Q7e polygon product side story;
- Q7f all of the above but some can be appendix/future-work.

Jörn answer:

- Q7a is a method for how to get Q7b proven, in addition to mere experimental
  evidence such as sampling (using different samplers)
- Q7b proven is a central result
- Q7c it should be complete, whether the end is negative or positive is open,
  i'd bet it's negative with like 70% confidence; note that compelte means "we
  hammered at the problem with every data-science method known to the standard
  data science handbooks" but "we skipped a method bc our implementation was a
  mess" is a fine non-result that still completes the hammer; we jus tshouldn't
  say "oh, yeah, we never got around to even trying that one, no reason we just
  were bad at task management and forgot"
- Q7d yep, we need that for the data science experiments and it's a major novel
  method
- Q7e: yep, it's another nice result we can prove computationally and have
  detailed empirics about
- Q8f: no, i don't really see anything there going into the appendix/future
  work
- basically, Q7 is answered by what Kai and I agreed on has to go into the
  thesis - it would be odd to now reduce thesis scope when we are so close to
  finish line (measured in what results we have and what little is missing -
  mainly, we know it's feasible to finish within a few days the resutl gaps)

### Q8. What kind of Codex output actually helps you write fastest?

Candidate answers:
- Q8a bullet scaffolds;
- Q8b claim maps;
- Q8c source summaries;
- Q8d local TODO lists;
- Q8e prose drafts for you to rewrite;
- Q8f review comments on your prose.

Jörn answer:

UNSURE! This has to be tried + measured! Great question! So part of the next 2
weeks of work will be to figure out where codex helped/did nothing/slowed down!
And importantly, we cannot directly measure gpt 5.5 capabilities bc for
writing the issue isn't how much gpt55 knows about scientific writing (A LOT)
or about how-to-write (A LOT of guide books memorized) -- but the issue is
whether bad habits override gpt55's capabilities and lead to corner-cutting /
slop / hallucinations / under- and overconfident claims / bad prioritization /
etc

## Follow-Up Process Answer On Writing Task Surfaces

### Q1. Should task files keep tracking chapter work at all, or should thesis/*.tex TODOs become the only routing surface for writing tasks unless complexity forces a map layer back in?

Jörn answer:

i will probably use .tex files as the root surface, but tasks/*.md files can
exist in addition and e.g. point at .tex or be pointed at by .tex ; reason: i
don't want to squeeze everything into one format that may not be suitable.
tasks/*.md for example is way nicer wrt colocating similar items in a tigher
document, and wrt deviating from thesis prose style (agents soemtimes struggle
to basically write 3 different styles in one file, switching between on a per
paragraph / sentence basis). But the ultimate flow of motivation is thesis
success => thesis state + thesis TODOs => work that needs to be done in the
thesis, and in the rest of the repo, and externally (e.g. some TODO in some
.tex file should mention that the thesis has to be submitted eventually).
[$project-quality](/workspaces/msc-math/.agents/skills/project-quality/SKILL.md)
is another thing that sort of tracks process knowledge, as does DEVELOPMENT.md,
which isn't like a once-done task but augments many of the tasks by providing
context and learnings and a default background with which to interpret the raw
task descriptions
