# Test Coverage Is High

Property:

- cheap bugs are caught during ordinary development loops
- unit, regression, smoke, and cheap end-to-end checks are in scope
- expensive validation experiments are not substitutes for cheap coverage

Starter read set:

Use these surfaces in this order:

1. `AGENTS.md` for the crate-test versus experiment-validation boundary.
2. The relevant convention skills, especially `rust-conventions`,
   `experiment-conventions`, and `dataset-conventions`.
3. The concrete code surface under review together with its local tests,
   smoke scripts, and nearby experiment verification packets.
4. `TASKS.md` for known stale smoke paths, missing tests, or deferred coverage.

Checks:

1. Name the exact code or pipeline surface under review.

2. Enumerate cheap checks that exist:
   - `rg -n "#\\[test\\]|mod tests" crates experiments`
   - `find experiments -name 'job-smoke.sh' -o -name '*smoke*'`
   - note local unit/regression tests, smoke scripts, and cheap happy-path runs

3. Ask which cheap bug classes are covered:
   - local logic / invariant mistakes
   - known tricky regressions
   - CLI / output-shape / data-format breakage
   - one representative maintained happy path

4. Run or cite the nearest cheap command when useful:
   - `cargo test -p <pkg> --release --lib`
   - packet-local smoke command
   - existing analyzer or smoke script

5. Flag gaps:
   - only expensive experiments exist
   - smoke contract is stale
   - no regression exists for a known bug-prone surface
   - one cheap bug class can still slip through with no local guard

6. Name the smallest missing test or smoke check when obvious.
