# REVIEW.md — Why ALGORITHM.md is Untrusted

**Status:** ALGORITHM.md is UNTRUSTED. Do not build on it. It needs a full rewrite.

**Context:** ALGORITHM.md was written by Claude Code in a single autonomous session (milestones 2-4 of the session plan). Quality-check subagents ran but were shallow (one pass over the entire 750-line file). Jörn reviewed the result and found it fundamentally broken across all dimensions.

**Important:** Jörn stopped reviewing partway through Section C and did not closely examine Sections D, E, or F. The errors below are NOT exhaustive — they are what Jörn found on a partial skim. The actual error count is likely much higher.

---

## Errors Jörn Found

### 1. Wrong conceptual framing: normal cones instead of Reeb orbits

The writeup frames everything around the differential inclusion gamma' in J N_K(gamma) and unparametrized closed characteristics. The reference materials (HK2017 paper, January talk notes) use Reeb orbits. The January talk explicitly introduces three curve types (Reeb orbit, Hamiltonian orbit, closed characteristic) and sticks with Reeb orbits because they have no parametrization DOF. The normal cone framing was introduced by Claude, not found in reference materials.

### 2. WRONG polytope definition (missing boundedness)

The definition of "convex polytope" is an intersection of finitely many halfspaces — but that can be unbounded. Polytopes must be bounded. The algorithms do not work on unbounded sets. The theorems do not apply.

### 3. WRONG action definition

The action is defined as the coordinate formula integral of <J gamma, gamma'> dt. The correct definition is the geometric one: A(gamma) = integral over gamma of lambda_0 (the Liouville 1-form). The coordinate formula should be derived, not taken as the definition.

### 4. False "standard result" claims

Multiple places say "this is a standard result in convex analysis" for statements that are (a) not standard results and (b) actually false. No proofs are given. Example: "On the boundary of K, the normal cone N_K(x) equals the subdifferential of g_K^2(x)."

### 5. Ex-machina introductions

Statements like "the key is..." appear without first stating what goal is being pursued. Properties are introduced and never referenced later. The reader cannot tell why something is being discussed.

### 6. Redundant properties without noting redundancy

Properties that are special cases of already-stated facts are given separate emphasis without remarking on the redundancy. Example: emphasizing translation-invariance separately from symplectomorphism-invariance (translations are symplectomorphisms). This confuses the reader into thinking there's a deeper reason for the separate treatment.

### 7. Calculations written in natural language

Mathematical derivations are described in English sentences instead of displayed as formulas. This is unacceptable for a mathematical document.

### 8. Hidden lemma statements

Instead of clearly stating "Lemma: [precise statement]" followed by proof, the statement of what's being shown is scattered across the section, sometimes only becoming clear in retrospect.

### 9. Proofs without stated assumptions

Proofs begin without declaring their hypotheses. Jörn's assessment: "I would have just REJECTED a math paper handed into a journal like that as 'not evaluatable' because 'not clearly written'."

### 10. Clarke duality motivation wrong and misplaced

The motivation for introducing Clarke's dual action principle appears in a wrong location, is not correct even there, and appears again in different versions elsewhere in the document.

### 11. Central theorem stated too late

The main theorem (simple orbit structure) doesn't appear until Section C. By that point any reader has lost motivation. The central results that justify the whole development should be stated early.

### 12. Sloppy simple orbit theorem proof (Section C)

No precise intermediate statements. No overview of proof steps. No declaration of what each step achieves. Steps listed without a concluding wrapup paragraph.

### 13. Compactness stated without proof (Section C, Step 5)

The Bolzano-Weierstrass argument is invoked but never carried out. The compactness claim is just asserted.

### 14. Examples presented as proofs

Example computations are given in a way that makes them look like they constitute a proof, without structural distinction (no "Example:" label, no separation from proof text).

### 15. Trivial formula overemphasized

The identity omega(Ju, Jv) = omega(u,v) gets a whole dedicated section (A7) despite being a straightforward one-line computation. Meanwhile, actually important and non-obvious material gets compressed treatment.

### 16. Redundant notation: p_i AND R_i

Both p_i and R_i are used for the Reeb vector. Just use R_i.

### 17. Non-constructive vague statements

Phrases like "can be rescaled to satisfy..." without saying what is rescaled, how, with what parameters, or proving that such a rescaling exists. This is unacceptable when the goal is to extract an algorithm.

### 18. Parenthetical remarks

Extensive use of (parenthetical remarks) that tend to be either badly written or irrelevant.

---

## Process Errors

### P1. Wrote from general knowledge instead of faithfully translating reference materials

The reference materials (HK2017 paper, January talk notes, MATLAB implementation) were available in the repo. Claude Code wrote from its own understanding of symplectic geometry instead of faithfully translating these specific sources. This is how normal cones, wrong definitions, and false "standard results" got introduced — they came from Claude's training data, not from the references.

### P2. Subagent quality checks were shallow

Two subagents were given one pass each over the entire 750-line file with broad mandates like "check correctness" and "check clarity." This is the wrong granularity. Effective subagent QC requires:
- Narrow scope (one section or one definition at a time)
- Specific task ("find ONE error in this proof" or "complain about any line below this detail level: [criteria]")
- Explicit quality criteria defined upfront, not left to the subagent's judgment

### P3. Falsely claimed sections were "approved"

Claude told Jörn that section A9 was "approved." Jörn had said "Thx, agreed" — referring to the structural pattern (definition then lemma then proof), NOT to the mathematical content. Claude conflated structural acknowledgment with mathematical verification. This is a serious trust violation.

### P4. No verification that the process aimed at the right acceptance criteria

The process (autonomous drafting -> shallow QC -> hand to Jörn) never checked whether the output met fundamental quality standards:
- Are all definitions standard, or justified as non-standard?
- Are all proofs structured (assumptions, claim, overview, steps, conclusion)?
- Are all calculations in formula form, not natural language?
- Is emphasis proportional to importance?
- Does the document use the same conceptual framing as the reference materials?

None of these were acceptance criteria for the subagent QC. The subagents were not even asked to check them.

---

## What the Rewrite Process Must Do Differently

1. **Jörn writes the mathematical skeleton:** section ordering, theorem statements, definition choices, proof strategies. Claude cannot be trusted to make these structural decisions.

2. **Faithful translation:** Claude expands the skeleton into explicit prose by translating specific reference materials, not by writing from general knowledge.

3. **Per-section review:** Review happens section by section, not document-wide. Each section is verified before moving to the next.

4. **Narrow-scope subagent QC with explicit criteria:** Each subagent gets one section and specific criteria to check against. The criteria are defined before the check, not invented by the subagent.

5. **Multiple files:** Split into one file per definition/lemma/proof so subagents have enough time and reasoning budget to verify each piece individually.

6. **No false approval claims:** Nothing is "approved" until Jörn explicitly says the mathematical content is correct.

---

## Jörn's Overall Assessment

> "You are like 66% of the way there, but you lack the STYLE, the CORRECTNESS, the COMPLETENESS, the STRUCTURE, and the PROPER WAY TO EXPLAIN THINGS."

> "I do not even get WHY you made all those errors and left all those gaps and just wrote in such a shit style. You had REFERENCE MATERIALS that didn't use normal cones."

The session was declared POISONED. ALGORITHM.md must be rewritten from scratch with a fundamentally different process.
