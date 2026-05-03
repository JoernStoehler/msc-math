# formal/symplectic-polytope-geometry.tex review feedback

**Source:** Jörn's top-to-bottom review of the former library math PDF (session 2026-03-24).
Current source: `formal/symplectic-polytope-geometry.tex`.
**Branch:** `remove-unit-normals` (worktree `.claude/worktrees/remove-unit-normals/`)
**Status:** Jörn reviewed Defs 1–13, then stopped. Review incomplete — covers only `formal/symplectic-polytope-geometry.tex`, not `formal/ehz-kkt-system.tex` or `formal/capacity-algorithms.tex`.

## Jörn's verbatim feedback

### Defs 1–2 (symplectic form, J₀)

Def 2 overlaps with Def 1. I think it'd be better to just have:
- Def 1: we use the standard symplectic setting in R^4: items: coordinates, inner product, omega_0, J_0, lambda_0, various algebraic identities to use (e.g. omega_0(u,v) = <J_0 u, v> = <coordinate expression> etc.
- Remark 2: in the code we use nalgebra::Vector4<f64/BigRational> etc; we offer a simple Matrix4<> J0 and inlineable fewer-flops operations that use sparsity for e.g. v ↦ J_0 v or u,v ↦ omega_0(u,v)

### Def 4 (EHZ capacity / systolic ratio)

Correct, worth mentioning.

Insert after Def 4: Def: HKO2024 counterexample, Thm: HKO2024 has vol(K), cap(K), sys(K) = ..., and so Viterbo's Conjecture is false.

### Def 5 (symplectic product)

Correct.

### Def 6 (Lagrangian product)

Worth to talk about symplectic subspaces (standard terminology) S_1, S_2 maybe for clarity / analogy to Def 5 (highlighting the differences). But correct already.

### Prop 7 (capacity of symplectic product)

Rename Thm 7. No need to require "origin in interior" here — that is an unused assumption. Proof is well written, starts with Proof idea, then executes step by step. Unsure whether the proof can be made shorter without making it harder to verify — since we are however in developer-facing formal notes and not thesis/, clarity & correctness & verifiability are more important than educational value/readability/style. So this is probably the right balance!

### Def 8 (polytope dual / H-representation)

A bit weird. I'd like to make it clearer to the reader how polytopes K ⊂ R^4 with 0 ∈ int K correspond to H-representations { x : <a_i, x> ≤ 1 f.a. i=1,...,F }; namely one needs to check on the rhs boundedness of the resulting set.

Insert a definition of when a H-representation is irredundant (no a_i can be removed without changing the set K).

### Def 9 (face lattice)

This is a natural language definition, which is fine and clear! No need to define the incidence relations here yet in detail, since there are different choices depending on context that are useful. Face lattice is a well known term, while "skeleton" is maybe a bit ambiguous. I should start calling it face lattice as well.

In the "face lattice" highlight that to us faces are closed, and we write "interior of a face" for the interior that has a unique signature of equality and strict inequality cases in the H-representation.

### Def 10 (cross product in R⁴)

Ok.

### Def 11 (2D polygon H-representation)

I think we actually also want to get rid of heights here, and just use the same definition as before (is it worth just doing arbitrary m-dim polytopes in Def 8+9?). But yes we need a Definition of when a H-representation is irredundant and counter-clockwise for 2D polytopes.

### Def 12 (shoelace / polygon area)

That's not a definition but an algorithm (that isn't even specified bc it is so well known). Or rather two algorithms.

### Def 13 (volume via pyramid decomposition)

Same issue as Def 12.

### Structural feedback

Maybe it's worth splitting `formal/symplectic-polytope-geometry.tex` into topic files already here, since we have so much ground to cover. Probably worth to have:
1. Euclidean geometry of polytopes
2. Symplectic geometry standard setting
3. Reeb orbits starting with the Reeb vectors on a polytope (not a vector field since there's no unique vector for points on the interior of 0,1,2-faces)

## Goal

Go through `formal/symplectic-polytope-geometry.tex` top to bottom, fix everything, and get our math in order. The reviewed PDF had red/orange approval bars — red = unapproved, orange = notation-updated. The goal is to eliminate all red bars by the end.

Follow-up task (separate session): get the Rust code in order as well.
