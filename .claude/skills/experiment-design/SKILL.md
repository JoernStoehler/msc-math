---
name: experiment-design
description: Turn a research question into an actionable experiment. Use when starting a new experiment or tackling a new research question — not for bug fixes, data regeneration, or narrow implementation tasks.
user-invocable: true
---

# Experiment Design Workflow

Experiments have open research questions. Answering them requires choosing what to measure, how to measure it, and what the observations would mean — this is the methodology. Different methodologies test different things, assume different things, and can miss different failure modes. The wrong choice wastes the implementation effort.

## 1. Formalize the question

State the claim being tested as a mathematical property. "The gradient is correct" is not a property — it's vague about what correctness means. "The output of `capacity_derivatives_a` predicts capacity values to first order under perturbation" is a testable property.

List the assumptions the claim depends on. Check whether math.tex or papers say these assumptions hold, and whether they are verified or conjectured.

State what observation would confirm the property and what would falsify it. If you can't answer this, the property isn't precise enough.

## 2. Generate candidate methods

Come up with 2-4 approaches to produce evidence for or against the property. For each method, state: what it measures, what it assumes, and what failure modes it would miss.

Common method families in this project:
- **First-order prediction:** Perturb inputs by td, check that f(x+td) − f(x) − t·g·d = o(t). Tests the defining property of a (sub)gradient. Does not assume differentiability.
- **Known closed-form:** Verify on polytopes where the answer is analytically known. Strong evidence but limited to available closed forms.
- **Convergence rate:** Show error decreases at a theoretically predicted rate as a parameter varies. Tests asymptotic behavior but can miss constant-factor errors.
- **Cross-implementation:** Compare two independent computations of the same quantity. Catches coding bugs but not shared conceptual errors, and assumes both implementations are truly independent.
- **Property-based:** Test mathematical properties the result must satisfy (symmetry, invariance, monotonicity). Catches structural bugs but doesn't verify values directly.

## 3. Present to Jörn

Present the formalized property (Step 1) and candidate methods (Step 2). One line per method: what it tests, what it assumes, what it misses. Jörn picks the methodology. Do not begin implementation before this step.

## 4. Track epistemic status

During implementation and when writing results, label each claim:
- **Observation:** raw data the experiment produced.
- **Assumption:** something the method requires to be true for the observations to be meaningful.
- **Inference:** conclusion drawn from observations + assumptions.
- **Hypothesis:** speculation beyond what the data shows.