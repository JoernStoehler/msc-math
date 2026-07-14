# Design Investigation Displays

Use this for plots, tables, compact reports, or interactive views whose primary
consumer is a researcher deciding what the data might mean or what to do next.
The goal is epistemic usefulness and fast error detection, not publication
polish.

## Investigation Task

State what the display should help inspect: distribution, ordering, dependence,
tail behavior, subgroup mixture, residual structure, trajectory, disagreement,
missingness, instability, an individual witness, or another live relation.
Name the decision it can change and the likely false inference it could invite.

Use prose or a shell summary for a short linear fact, a table for exact repeated
comparisons or selected witnesses, and a plot when shape or multiscale structure
is materially easier to perceive. Multiple cheap views are useful when they
test different explanations; extra panels that do not change interpretation are
not.

## Honest Encodings

Make population, denominator, selection rule, transforms, normalization,
aggregation, censoring, and missing values recoverable. Preserve distinctions
between direct observations, fitted or extrapolated values, thresholds,
controls, and highlighted examples. Show uncertainty or variation when it
changes the research conclusion.

For plots, choose scales and stratification around the live comparison. Inspect
whether pooled structure disappears or reverses by generator, bucket, or other
known source. Use alternate transforms or residual views when they distinguish
a real hypothesis rather than merely making a pattern look stronger.

For tables, say why rows are included and how they are ordered. Keep enough
precision to audit the comparison without implying unsupported accuracy. Do not
turn strongest, cleanest, representative, and merely diagnostic examples into
interchangeable evidence.

## Ownership And Review

Keep the producer and authoritative output beside the experiment or data owner.
Regenerate rather than hand-edit. A scratch display may stay in `/tmp` when its
only purpose is current reasoning; promote its producer, source contract, and
interpretation if later research depends on it.

Review proportionately by checking that the display answers its investigation
task, agrees with source data, exposes rather than hides relevant failures, and
does not convert exploratory selection into validation or mechanism evidence.
Do not require final typography, print constraints, or integrated-PDF review.

If the display becomes a candidate for reader-facing explanation, treat that as
a new consumer: use `$thesis` to choose the publication medium, model reader
effects, develop the asset, and review the rendered thesis context.
