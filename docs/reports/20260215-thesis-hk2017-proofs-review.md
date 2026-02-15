# Review: HK2017 algorithm chapter — proofs and definitions

**Branch:** `thesis-hk2017-proofs` at `/workspaces/worktrees/thesis-hk2017-proofs`
**Base:** local `main` at `ed58245`
**Date:** 2026-02-15
**Scope:** `thesis/` only (per Jörn's instruction)

---

## 1. Build Verification

- **Rust:** `cargo test` passes (all crates, zero failures).
- **LaTeX:** `latexmk` produces a 451KB PDF. Warnings are all pre-existing on `main` (undefined citations `AAO2014`, `Rudolf2022`, `BezdekBezdek2009` and multiply-defined label `def:lagrangian-product` — all in `chapter-billiard.tex`, untouched by this branch).
- **No new warnings** introduced by the branch.

## 2. Deletion Verification

| Deleted File | LOC | Purpose | Replacement | Verdict |
|---|---|---|---|---|
| `thesis/dictation.md` | 138 | Workflow file: Jörn dictates, Claude translates to LaTeX | Content now in `.tex` files (all marked `[done]` before deletion) | ✓ Consumed, no longer needed |

The `correspondence.tex` change removes one row from the notation table (`$Q$-function`), which is no longer used in the thesis (the branch uses `$\beta^\top H \beta$` directly). ✓

## 3. New Files

- `thesis/bibliography.bib` (9 lines): Single entry for HK2017. Matches the `\cite{HK2017}` used in the chapter. ✓
- `thesis/label-map.py` (123 lines): Utility script parsing `.aux` file to generate label-to-rendered-number mapping. Clean, useful for agents. Not part of the thesis output. ✓

## 4. Structural Overview

The branch adds ~1350 lines across two main files:

- **`chapter-algorithm.tex`** (~1230 lines): Sections 3 (Main Result), 4 (Definitions), 5 (Simple Minimizer Existence), 6 (Algorithm Correctness), 7 (Optimizations).
- **`simple-minimizer-proof.tex`** (~766 lines): Subsections 5.1 (Primal-Dual Equivalence) and 5.2 (Simplification Algorithm), `\input`'d from `chapter-algorithm.tex`.

## 5. Mathematical Findings

### Finding 1 (HIGH): Possible circularity in Theorem 5.9, Part 3(d)

**Location:** `simple-minimizer-proof.tex`, lines 370–387 (Theorem 5.9, proof Part 3, step (d) and "Combining").

**The claim:** From the integrated Fenchel equality $c^2 T + I_K(z) = 2\nu T$, the text deduces "hence $c^2 = 2\nu - 1$" (line 381), which requires $I_K(z) = T$.

**The issue:** This step is the crucial one that, combined with $c^2 = \nu$ from (b)+(c), determines $\nu = 1$. But $I_K(z) = T$ is not independently established at this point. By applying Euler's identity for 2-homogeneous functions to $\frac{1}{4}h_K^2$ at the subgradient relation $\nu z + b \in \partial(\frac{1}{4}h_K^2)(-J_0\dot z)$:

$$\langle \nu z + b, -J_0\dot z\rangle = \frac{1}{2}h_K^2(-J_0\dot z)$$

Integrating: $2\nu T = 2I_K(z)$, giving $I_K(z) = \nu T$.

Substituting back into $c^2 T + I_K(z) = 2\nu T$ gives $c^2 = \nu$ — the **same** equation as (b)+(c). So (d) provides no new information, and $\nu$ is undetermined from these three identities alone.

**Impact on downstream results:** The theorem's consequent ($\min A = \min I_K = c_{\text{EHZ}}(K)$) is a classical result (Clarke's Dual Action Principle) and is almost certainly correct. The gap is in this specific proof, not the theorem itself. The primal→dual direction (Part 2) is clean.

**Suggested resolution:** The fix likely involves either (a) restricting Part 3 to dual minimizers (where $I_K(z) = T$ could follow from Part 2's direction + a Fenchel-inequality lower bound), or (b) using the transversality condition for the free-period optimization. Jörn should verify which approach works.

**Confidence:** High that this is a real gap. The three identities ($c^2 = \nu$ from Euler on $g_K^2$, $c^2 = \nu$ from (b)+(c), $I_K(z) = \nu T$ from Euler on $h_K^2/4$) are algebraically consistent but underdetermined. I was unable to find a fourth independent identity from the E-L conditions alone.

### Finding 2 (LOW): Centering not verified in splitting and merging proofs

**Location:** Proofs of Lemma 5.12 (splitting) and Lemma 5.13 (merging).

Both lemma statements claim the output curve is centered ($\int z' = 0$). The proofs verify closure, period, and action, but neither verifies centering.

- **Splitting** changes the path within segments (same endpoints, different intermediate points), which changes $\int z'$.
- **Merging** rearranges segments, which also changes $\int z'$.

**Fix:** Both are easily fixed by noting that centering can be restored by translating $z'$ by $-\frac{1}{T}\int_0^T z'$, which preserves $\dot{z}'$, $A(z')$, $I_K(z')$, and closure.

The merging proof's line "Closure and centering are preserved" (line 724) is incorrect as stated — centering is NOT automatically preserved by rearrangement. It should say "centering is restored by translation."

## 6. Readability Findings ("not obviously correct on first read")

### Finding 3 (MEDIUM): $\tilde\beta$ notation undefined

**Location:** `chapter-algorithm.tex`, lines 868–910 (Theorem 3.1 proof, Steps 5–6).

The notation $\tilde\beta$ appears first at line 868 ("Substitute $\tau_k = T h_{\sigma(k)} \tilde\beta_k$") without formal definition. From context, $\tilde\beta_k = \beta_{\sigma(k)}$ (the restricted, reindexed vector), but this is never stated.

The notation is then used throughout the Algorithm Correctness section (Lemmas 6.1, 6.2, proof of Theorem 6.3) without clarification. Meanwhile, the linear system and well-definedness proof use $\beta$ (the |S|-dimensional vector).

**Impact:** Reader must infer $\tilde\beta_k = \beta_{\sigma(k)}$ from the substitution. Not a mathematical error, but adds friction to an already dense proof.

**Suggested fix:** Add a one-line definition: "Write $\tilde\beta_k := \beta_{\sigma(k)}$ for $k = 1, \ldots, |S|$ for the restricted vector indexed by position in the ordering."

### Finding 4 (LOW): Euler identity used without naming or proving it

**Location:** `simple-minimizer-proof.tex`, lines 345–349 (Part 3, step (b)).

The text uses: "For $y \in \partial g_K^2(x)$, ... yields $\langle y, x\rangle = 2 g_K^2(x)$." This is Euler's identity for 2-homogeneous convex functions, but it's stated as an inline derivation without naming it. It's used again in Part 4 (line 410, implicitly) and in the Fenchel equality computations.

Since this identity appears repeatedly and is crucial, a named remark (e.g., "Remark: Euler's identity for $g_K^2$") after the gauge function definition would improve readability.

### Finding 5 (LOW): Primal-dual proof Part 3, step (c) — intermediate step unclear

**Location:** `simple-minimizer-proof.tex`, lines 364–366.

The computation:
$$A(\gamma) = \nu^2 A(z) + \frac{\nu}{2}\langle b, \underbrace{-J_0[z(T)-z(0)]}_{=0}\rangle$$

On first read, the term $\frac{\nu}{2}\langle b, -J_0[z(T)-z(0)]\rangle$ is not obviously the correct result of expanding $\langle -\nu J_0\dot z, b\rangle$. The algebra is:
- $\frac{1}{2}\int \langle -\nu J_0\dot z, b\rangle dt = -\frac{\nu}{2}\langle J_0\int \dot z, b\rangle = -\frac{\nu}{2}\langle J_0(z(T)-z(0)), b\rangle$

But the text writes $\frac{\nu}{2}\langle b, -J_0[z(T)-z(0)]\rangle$, which is the same thing (using $\langle u, v\rangle = \langle v, u\rangle$). Correct, but the sign manipulation is easy to misread.

### Finding 6 (LOW): Definition 4.8 (Closed characteristic) — "a.e." ambiguity

**Location:** `chapter-algorithm.tex`, line 276.

The definition says $\dot\gamma(t) \in \ell_{\gamma(t)}$ and $\lambda_0(\dot\gamma(t)) > 0$ "for a.e. $t$." This "a.e." is appropriate for the $W^{1,2}$ generalized Reeb orbit setting (Definition 4.20), but Definition 4.8 is specifically for smooth boundaries ("Let $K$ be a convex body with smooth boundary"). For smooth boundaries, Reeb orbits are smooth, so "a.e." could be replaced by "for all $t$."

Not wrong (a.e. is weaker), but a reader might wonder why "a.e." appears in the smooth setting.

### Finding 7 (LOW): Theorem 5.5 (Simple minimizer) — proof referenced but not provided

**Location:** `chapter-algorithm.tex`, line 741.

Theorem 5.5 is followed by `\input{simple-minimizer-proof}` which provides the proof. This is fine structurally, but the theorem statement doesn't say "see below" or "proof in Section 5.1–5.2" — the reader just sees a theorem with no proof, then a subsection with its own structure. The `\input` resolves this, but it's mildly disorienting.

## 7. Missing Citations (TODO items found in source)

- Line 693 (`chapter-algorithm.tex`): "TODO: add \cite when bib entry exists" — for Rabinowitz 1978 (Theorem 5.3, existence of closed characteristics, smooth case).
- Line 708 (`chapter-algorithm.tex`): "TODO: add \cite when bib entry exists" — for Artstein-Avidan & Ostrover 2014 (Theorem 5.4, existence for polytopes).

These should be added to `bibliography.bib` before the thesis is finalized.

## 8. Strengths

1. **Structure:** The chapter has an exceptionally clear logical flow: Main Result → Definitions → Search Space Reduction → Algorithm Correctness → Optimizations. Each piece builds on the previous one.

2. **QC comments:** Every definition and lemma has detailed QC comments explaining: what the mathematical content means, what properties follow, what downstream uses exist, and what tests correspond to it. These are invaluable for agents and reviewers.

3. **Jörn approval markers:** Nearly every environment has a `% Jörn: text approved` marker with commit hash. This makes trust levels immediately visible.

4. **Self-contained proofs:** The algorithm correctness proof (Section 6) is remarkably explicit — every step cites the exact lemma/definition used, every variable substitution is shown, and the well-definedness proof (Lemma 3.4/6.3) is presented at both the lemma statement and again with full proof in the correctness section.

5. **The shoelace lemma proof** (Lemma 4.22) is a model of clarity: three terms identified, each dispatched in one line.

## Executive Summary

**Summary of findings:**
1. **Theorem 5.9 Part 3(d): possible circular reasoning** — The step deducing $c^2 = 2\nu - 1$ implicitly uses $I_K(z) = T$, but Euler's identity gives $I_K(z) = \nu T$, making the deduction equivalent to the already-known $c^2 = \nu$. Jörn should verify whether an independent argument for $\nu = 1$ is needed.
2. **Centering preservation claims in splitting/merging proofs** — Both proofs claim centering is preserved, but rearranging/splitting segments changes $\int z'$. Fix: note that centering is restored by translation (preserving all other properties).
3. **$\tilde\beta$ notation undefined** — First used in Step 5 of the main proof without definition. Add a one-line definition.
4. **Two missing citations** (Rabinowitz 1978, AAO 2014) marked as TODO in source.

**Recommendation:** Jörn should verify Finding 1 (the $\nu = 1$ derivation in the primal-dual proof). If the gap is real, it may require restructuring Part 3 of the proof (e.g., restricting to minimizers, or adding a transversality argument). The other findings are minor and easily addressable. The overall writing quality is high.

**Time investment:** ~90min review.
