# Coordination cleanup structure · 30 September 2026

Agent design record for this migration, not a second task queue. Live assignments
are in `current.json`; the completed record is archived with the migration inputs.

| Node | Goal / constraint / unknown | State | Confidence |
| --- | --- | --- | --- |
| N1 | Jörn sees reported work and resource use cheaply in Chrome | downhill | Rendering straightforward; usefulness awaits real use |
| N2 | Agents have one canonical coordination owner and preserved constraints | downhill | Existing backlog and handoff establish content |
| N3 | Knowledge keeps sources and boundaries, separate from assignments | downhill | Existing docs/knowledge is adequate for the initial scope |
| N4 | No new thesis run, scientific claim promotion or spending authority | constraint | Explicit retained stopped-run boundary |
| N5 | Wider topic-packet cleanup | parked | Jörn selected central surfaces first |

Dependencies: N2 -> N1 because the browser renders reported state; N3 -> N2
because durable knowledge must remain discoverable without becoming task authority.
N4 constrains N1/N2; N5 controls only the optional later breadth of cleanup.

The fresh-timestamp indicator stands for observation age. A freshly timestamped
wrong assignment still fails the actual goal; it is not a liveness or truth test.

Decision log: reuse docs/knowledge; introduce docs/coordination and docs/dashboard;
keep one state file until conflicts justify splitting. Expect routine path/schema
checks to pass after migration; expect the retained scientific brief to stay visibly
unreconciled because this request does not include a mathematical/PDF audit.

Outcome: central migration completed; five HTTP checks pass, original backlog and
three historical inputs preserve bytes. Chrome connection unavailable. Frozen PDF
verification has seven pre-existing source mismatches; no evidence refresh performed.
Jörn answered the scope question before commit: central surfaces first. No broader
topic-packet review is assigned.
