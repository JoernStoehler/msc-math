# Retrospective capacity-interval adapter

Fork of `../current-body-evaluator` for historical revalidation. Same input
geometry and numerical-size policy; same production capacity and numerical
volume code. It preserves the computed certified capacity interval even when
`capacity_value(..., 1e-10)` refuses scalar conversion. No tolerance relaxation.

Successful capacity evaluation has status `capacity_interval`. `capacity_scalar`
and `numerical_sys_scalar` are null when the scalar tolerance fails;
`scalar_error` retains the reason. `volume` is numerical, not certified. No
certified sys interval is advertised. All ordinary errors retain their original
stages. Exact product capacity is retained when supplied by the production API.

Build and supervisor scripts have the same interfaces as the original adapter.
Use a separate target directory: both packages intentionally retain the executable
name `current-body-evaluator`, and build receipts identify actual source/binary
bytes. Do not overwrite an old frozen binary whose receipt is still needed.

The retrospective repair packet under `docs/ds-retrospective-revalidation/repair`
records the old refusal IDs, exact transformed inputs, build/source receipt,
raw responses and transfer calculations. An exact rescaling is performed by
input preparation, not hidden inside this adapter; it still reconstructs every
scaled body and enforces all input policies afresh.
