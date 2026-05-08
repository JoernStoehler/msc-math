---
name: project-quality
description: Use when Codex writes, edits, reviews, or delegates work where repo-wide quality objectives, navigation, clarity, verification, tracking, or coordinate conventions matter.
---

# Project Quality

## Long-Term Quality Objectives

The project is worked on by many agents over a long time. To avoid accumulation of technical debt and errors, we optimize more strongly for the following instrumental objectives than we do already for publication quality, and for short-term success at tasks.

- **Verifiability**: We stick to true claims, and distinguish strength of evidence, empirical versus theoretical support, observation from inference under potential overlooked hypotheses, aspirations from historical from current state, and so on. We also make it easy to check claims by linking them to their source of truth, and to evidence that previous checks were done. The main guidance here is to notice when a check was annoyingly hard, and to then add more signposting, cached reasoning results, full reasoning traces, references to all evidence, timestamped comments/markers that something was checked and by whom, and so on. The opposite pressure comes from avoiding staleness and reducing complexity, so we don't bother to record the full reasoning, but instead aim for the sweet spot where future agents can reproduce the steps between source of truth and the final claim they are checking.
- **Reproducibility**: Relatedly, everything should be reproducible from source truth, albeit we of course supply structure to speed up the process. This includes data, interpretation, writeup, but also code features, test cases, planned tasks and conventions.
- **Navigability**: We minimize the risk of future agents not finding relevant information, or being drowned in irrelevant material. This includes longer than usual speaking filenames, use of predictable standard terminology that can be grepped for, and cross-referencing between files. Navigation mostly works on a file-level, so we keep files single-concern.
- **Clarity**: Most code, math and text is read many times, so we optimize both code we created and we inherited from previous agents for readability and maintainability. This mainly includes using plain, specific, unambiguous descriptions, standard terminology, examples instead of analogies, and iteration to remove sources of complexity once a better alternative is found. Sentences should be broken down when they entangle multiple concerns, to be easier to edit.
- **Tracking**: We track tasks, progress, todos in the repo instead of external tools. Git tracks the history for us, to keep the current state of the repo more simple and focused on active and future work.

## Conventions

All conventions serve the long-term quality objectives, the final publication objectives, and short-term task success within a single agent session. We don't document in `AGENTS.md` what serves what, often multiple benefits apply.

### General

- The coordinate convention is `(q1, q2, p1, p2)`.

**Navigation and Exploration**
- use long descriptive names for files and folders
- use predictable code symbols, keywords, LaTeX labels; grep to quickly find definitions and uses
- cross-reference other files, avoid unstable line numbers

**Clarity**
- write plainly, don't use metaphors or analogies
- focus on information transfer to future agents
- use standard terminology
- be specific, neither over- nor under-inclusive
- break down sentences that entangle multiple concerns
- avoid vague terms
- don't abstract prematurely

**Verification and Tracking**
- link claims to their source of truth, except where obvious
- record enough arguments and intermediate steps to enable agents to easily check whether some reasoning result is true and detect when the underlying source of truth has changed
- note that often the supporting arguments are straightforward, and the real work was elevating the hypothesis
- explicitly track epistemic status of claims, such as empirical versus theoretical evidence, strong versus weak support, potential unknown unknowns i.e. overlooked hypotheses, diverse versus correlated arguments
- track task states, external decisions from Jörn, and results of expensive tests/checks to allow future agents to deem checks unnecessary/unchanged
- move unnecessary claims into the git history i.e. delete them, since they are expensive to verify
