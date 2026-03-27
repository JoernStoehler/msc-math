---
paths:
  - "**/*.rs"
---

# Rust Conventions

## Coordinate convention

(q₁, q₂, p₁, p₂) — components [0,1] = q-space, [2,3] = p-space, [0,2] and [1,3] = symplectic planes. Defined in `geom/symplectic_form.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂).

## Math-code correspondence

Types, function signatures, and function bodies have 1:1 structural correspondence to mathematical definitions. Not "inspired by" — literal correspondence.

- Doc comment formulas must match the code's actual computation
- Invariants stated in doc comments are enforced by types/constructors/assert!
- Properties stated in doc comments have corresponding tests
- Types encode mathematical invariants, validated in `::new()`

## Cross-references to math.tex

Format: `[lem:label]`, `[thm:label]`, `[def:label]` — matching `\label{}` in the module's math.tex.

- Include a one-line English description of the referenced result
- Never duplicate proofs — math.tex is the single maintained source of truth
- Never invent labels — use `// TODO: add [lem:...] to math.tex` if the lemma isn't written
- In source code, never use rendered numbers like "Lemma 3.2" — always use the label
- Every non-trivial code block must map to a math.tex lemma

Read the module's math.tex before editing .rs files in that module.

## Algorithms

Three capacity algorithms: `hk2017` (general, exponential), `billiard` (Lagrangian products, fast), `tube` (no Lagrangian 2-faces). Where domains overlap, algorithms must agree on computed capacity.

No rayon inside algorithms — parallelism is at the dataset level (each polytope independently).

## Magic numbers

Empirically chosen constants: document rationale, motivating data point, limitations, and what to re-validate if changed. All in a comment on the constant definition.

## Performance claims

Never state performance without an inline benchmark citation. "~1ms" is a claim. "1.5-2.0ms for F=5-16 (criterion bench 2026-03-23)" is measured.

## Error handling

Follow standard Rust error handling. Types have semantic meaning — use them to distinguish outcomes.

**Return types for mathematical outcomes.** When a computation has multiple valid outcomes (e.g. feasible, infeasible, ill-conditioned), use an enum — not `Option<T>`. `None` discards the reason and is an anti-pattern for semantically meaningful results. Example: a KKT solver should return an enum like `KktOutcome::Feasible(result) | Infeasible { min_beta } | IllConditioned { lambda_min }`, not `Option<KktResult>`.

**`Result<T, E>` for operational errors.** I/O failures, invalid input formats, resource exhaustion. Use domain-specific error enums (`thiserror`), not strings. Propagate with `?`.

**`panic!` / `assert!` for bugs only.** Panics mean programmer error or violated invariants or falsified math theorems — conditions that should be unreachable in correct code and correct math. Standard Rust semantics.
- Never catch panics (`catch_unwind`). Never convert panics to `None` or `Result::Err`. Escalate them to the developer agent.
- When a panic fires during a run: read the source comment at the panic location, then investigate to find the root causes that fully explain why the panic was possible. Report preliminary findings to Jörn if the root cause is unclear, or has no straightforward fix that is in-scope. Don't hide the bug, not even to defer its resolution to later.
- Intentional panics for deferred work (edge case the developer knows CAN fire but hasn't handled yet) must have a comment explaining: why the work was deferred and what to do when the panic fires. These are temporary — they should be converted to proper return types when the deferred work is done.
- Standard idiomatic safety-net asserts (conditions the developer believes CANNOT fire in correct code with correct math) need no special comment beyond the assert message.

## Experiment binaries

For `experiments/*.rs`: copy library code into the binary rather than modifying `crates/` for experiment-specific behavior. Only stable, validated code lives in `crates/`.