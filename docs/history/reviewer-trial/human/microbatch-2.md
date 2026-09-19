# Human responses: microbatch 2

Responses received directly from Jörn in this trial conversation on 2026-09-18. Predictions in classification-transfer.md and the open-review outputs were frozen before these requests. This is a purposive set mixing shared concerns, disagreements and an endorsed passage.

## F: seed placement

Exact response:

> the fac thtat the seed 42 is mentioned tells me this is not written witha good model of the reader in mind. mathematicians don't want results sensitive to the seed? and the code documents the seed already for those who want to run the code?

Interpretation: confirms a writing/relevance problem with mentioning the seed in this mathematical account. Does not independently reject the support-height range or all generation-condition detail. The question about seed sensitivity is a relevance objection, not evidence that the result actually depends on this seed.

Frozen comparison: candidate-conditioned classifier predicted PROBLEM (0.78), explicitly identifying the seed as a reproduction detail interrupting the mathematical population explanation. None of the three initial open reviews flagged this location. The relevance review explicitly judged the sampling paragraph's detail broadly justified, though that does not separately endorse every detail. This is evidence for one useful candidate-conditioned detection, not demonstrated autonomous detection or general accuracy.

## J: control-selection mechanics

Exact response:

> this is super detailed -- why is it necessary to say how the controls were chosen? the code says it for non-mathematicians who need to reproduce this, a "randomly chosen" also would be useful perhaps for undersanding / rulilng out standard mistakes that could've been made here. I am a bit worried that this much i sbeing talked about "contorls" and "significance" -- do we have a problem with significance?? or is this just wasting the reader's time to talk about irrelevant digits of signficiance?

Interpretation: a confirmed concern about detail and reader relevance, with a possible concise explanation if it rules out a standard methodological mistake. This is not approval to relabel fixed hash ordering as random sampling, not an established significance defect, and not an instruction to erase consequential restrictions on the comparison.

Frozen comparison: all three open reviews and the classifier flagged this location. Their emphasis was mainly an unexplained second selector/control population, often proposing further description. The classifier additionally identified experimental machinery burdening the reader. Thus location overlap is strong, mechanism overlap partial, and remedy agreement not established. The human questions why this detail is needed; a reviewer proposing more detail might worsen that cost. Do not count a matching highlighted span as a fully matched diagnosis.

The parent answered the human's significance question narrowly: this trial has not established a significance problem; finite-pool comparisons do not establish recurrence beyond those pools, and that limitation alone does not justify the amount of narrative detail. No new scientific audit was claimed.

## K: opposing paths

Exact response:

> "designed product paths" is weird - it's clear that they are "deisgned" / it is not clear what that word adds. but "two product families with edge count 3x6 and 4x4 respectively exhibit the opposite behavior numerically: both R and the systolic ratio decrease at every retained step" -- not sure what step is tbc -- sounds good already / plainly states that we have observed sth. a sentence "this rules out that the relation between R and sys holds universally in a strict manner, and that it would hold strictly for natural families of interest." would further spell out why we bring this up

Interpretation: the substantive observed counter-direction is useful; “designed” adds little, and “step” is not clear. A sentence explaining the role of the counterexamples would help. The original surrounding paragraph already says the paths warn against extrapolating the pentagon direction of improvement; the human saw a summary of that context, not its exact wording. Do not score the proposed consequence sentence as an established omission. The illustrative rewrite does not strengthen numerical observations into a proven universal counterexample or certify what counts as a natural family.

Frozen comparison: relevance reviewer requests deformation description, ranges and numerical resolution; the human's “step” uncertainty provides partial mechanism overlap, but he does not endorse the full requested expansion. Candidate classifier KEEP (0.26 problem probability) matches the useful substantive observation but misses the local wording concerns. Neither should be scored as a simple full hit. The nonessential modifier is a fresh local miss by the tested reviewers.

## L: absolute values and symplectic area

Exact response:

> Keep

Interpretation: accepts the explanatory sentence in the supplied definition context. This is a local positive judgment; it does not approve other routine explanations.

Frozen comparison: baseline explicitly endorsed this passage; classifier KEEP with problem probability 0.05 agrees. This is an adjudicated retained passage, useful against a rule that indiscriminately deletes apparent restatements of formulas.

## Interaction cost

Estimated 0.5–1 minute for F and 1–2 minutes for J, uninstrumented; original display estimates were 15 and 30 seconds. Allow a further 1–2 minutes for K. Other questions remain pending. These estimates are budget allowances, not measured active time.
