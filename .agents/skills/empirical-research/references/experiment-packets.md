# Structure Experiment Packets

Use this reference when creating, moving, splitting, joining, or documenting an
experiment packet, or when deciding whether code or methodology should be
shared between packets.

## Physical owner

The default physical unit is `experiments/<durable-question>/`. A packet keeps
together what a future agent needs to reproduce and interpret one empirical
question:

- the original question and current decision served;
- systems or mathematical objects under study and comparison alternatives;
- producer code and input/selection contract;
- retained outputs and their identity/provenance;
- analysis, interpretation, limitations, and reproduction commands.

Use a direct child of `experiments/` by default. Do not choose a parent
directory merely because the packet involves numerics, performance,
verification, one algorithm, one thesis topic, or active development. Those are
non-exclusive relations. Do not move a packet merely because its status or
mainline consumer changes.

Name the directory after a durable question or measured object. Avoid
lifecycle prefixes and names that become false when a candidate algorithm is
promoted or retired.

## README contract

The beginning of the packet README should make these facts cheap to recover,
using ordinary prose or labeled fields rather than a mandatory schema:

- status;
- original question;
- current decision or consumer;
- what the packet owns and does not establish;
- systems under study and comparisons actually supported;
- authoritative producer, evidence, and interpretation;
- relations that make another change likely to require reassessment;
- safe checks versus commands that refresh evidence.

Use exact paths, algorithm names, and stable research terms so ordinary `rg`
search finds both ends of a relationship. Preserve the original question even
when the current consumer or comparison set changes. Retained evidence names
the actual algorithm/configuration/commit it measured; the mutable label
`mainline` is not sufficient provenance.

`experiments/README.md` may expose several views—for example by measured
system, method, thesis decision, or status. Views are derived navigation and
must state their scope. The packet remains authoritative.

## Split and join

Split a packet when each result can be reproduced, interpreted, reviewed, and
maintained independently without importing the other packet's hidden context.
A distinct question, comparison contract, evidence lifecycle, or consumer can
justify a split.

Do not split merely because one packet has several runs, figures, objects,
algorithms, or methodological labels. Keep those together when their meaning
depends on a shared selection rule, control, comparison, or interpretation.

Join packets when neither has a stable interpretation without the other and
separate maintenance mostly creates duplicated provenance or synchronized
edits. Links are enough when the packets merely teach one another or share a
method family.

## Dependencies and copying

Distinguish these relationships:

- The implementation being tested is a real dependency. Name its exact API,
  path, and relevant version or source identity.
- A methodological or interpretive relationship is a README link, not an
  import.
- A small helper may be copied when independent evolution is desirable. Record
  its source path and copied-at commit near the copy; later divergence is then
  visible and allowed.
- Shared code is justified when synchronized semantics are part of the
  experiment contract. Give it an explicit maintained owner rather than an
  incidental import from another experiment packet.

Changing a mainline or candidate algorithm does not automatically move the
packet. Reassess the packet when its declared system-under-study or comparison
relation matches the change; update the supported comparison set while keeping
historical evidence identifiable.
