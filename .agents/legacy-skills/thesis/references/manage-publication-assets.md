# Manage Publication Assets

Use this reference whenever creating, regenerating, selecting, copying, or
integrating a generated thesis-facing or experiment-support asset.

Keep an experiment- or code-derived authoritative asset attributable to its
producer, with the question/contract needed to interpret it. Use conventional
local placement after inspecting the producer, inputs, consumers, and related
evidence; this rule does not require a particular experiment-directory
boundary. Keep a genuinely thesis-native asset in `thesis/`. Let producer code
define data transformation, fonts, sizes, colors, labels, and layout.
Regenerate outputs rather than patching them by hand.

Deliberately copy selected publication outputs into the self-contained
`thesis/` tree. Record the relation between producer output and thesis copy
when future work could otherwise review or regenerate the wrong artifact.

Track whether the asset is draft, candidate, rejected, or thesis-ready and
whether it is source truth, proof input, empirical evidence, diagnostic, or
explanation only. A successful producer run or invariant check establishes
only what it checks.

Before reporting completion, rerun the producer when freshness is part of the
task, inspect the integrated rendered result, and verify source attribution,
paths, commands, status, emphasis, and epistemic claims.
