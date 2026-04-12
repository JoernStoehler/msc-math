# Incident: proposed worked-example artifact to fix a reflex failure

2026-04-12, co-project-owner session v2. Session context: this session is a second attempt at a task-graph coordinator role, after the first attempt burned out at ~250k tokens. Sibling postmortem: `feedback/2026-04-12-co-ownership-v2-postmortem.md`.

## What happened (verbatim where possible)

During session wrap I proposed "R2": a request for Jörn to produce a worked 5-8 option brainstorm for the L94/L153 handoff-to-pool decision, as a training artifact future coordinator sessions could read to learn what diverse brainstorming looks like. I had motivated R2 as addressing the binary-option collapse failure mode that cost the most time this session.

Jörn asked me to walk through whether that value chain actually works:

> Jörn: i am a bit confused - why would that be useful ?

On reflection I concluded it doesn't:

> Me: The failure mode is a reflex (binary-collapse under time pressure), not a knowledge gap. Seeing one worked example doesn't fix a reflex. Future coordinator sessions will have the same reflex. Reading a static example won't flip it. The verification mechanism (ask Jörn) works regardless of whether a worked example exists. The example would sit in the postmortem file hoping to be read, compared, and internalized — three unreliable steps.

I then dropped R2 and marked its task deleted. Jörn then pushed further:

> Jörn: i am not sure why you need sth from me in order to write down an incident?

Pointing out that I was conflating "worked example from Jörn" (which needs his input) with "recording the incident as an observation" (which doesn't). No blocking on Jörn was required to document what happened.

## Why it went wrong

Two separate errors:

### Error 1: Proposed a teaching artifact for a reflex failure without checking the mechanism

I proposed R2 motivated by "binary-option collapse cost the most this session, so I need a fix." I did not check whether the proposed mitigation (worked example → read → absorb → apply) actually addresses the mechanism of the failure.

Reflexes do not update from reading static examples. They need real-time forcing functions: being interrupted mid-brainstorm, being required to justify option count, being blocked from proceeding until N options are listed. Writing down "here's what good looks like" is a knowledge transfer, and the failure is not knowledge-shaped.

I should have checked: "is this failure mode a knowledge gap or a reflex?" before proposing any educational artifact. Had I done that check, R2 would never have been posed.

### Error 2: Blocked on Jörn for work that was agent-side

I framed the task as "waiting on Jörn to produce the worked example," making the session unable to make progress without his time. But the work of *observing and recording what happened this session* does not require Jörn's judgment or context — it requires only my own session context, which I have.

I conflated "this work would benefit from Jörn's input" with "this work is blocked until Jörn speaks." Not all work benefits mean blocking is required. Incident reports, failure-mode analyses, and observational notes are agent-side work.

## Generalization

1. **Reflex failures need structural mitigation, not educational artifacts.** Ask-first rules, external checks, forcing functions in the execution loop. Writing down what good looks like does not change a reflex.
2. **Before proposing a mitigation, check if it addresses the failure mechanism.** A mitigation that works on knowledge gaps does not automatically work on reflexes, motor patterns, or attention shortcuts. Match the mitigation to the mechanism.
3. **Before blocking on Jörn, check if the work actually needs his input.** Observations, analyses, and records can usually be produced from the agent's own context. Ask only when his judgment, context, or decision is load-bearing.
