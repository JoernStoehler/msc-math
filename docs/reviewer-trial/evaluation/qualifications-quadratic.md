# Qualifications review: quadratic-program chapter

Scope: MSc mathematical reader with the preceding capacity and basic symplectic theory, including the prior chapter’s pure-facet minimizer result. PDF extraction artifacts are disregarded. This review concerns qualifications only.

## Concerns, ranked by reader cost

1. **Likely: repeated defense against an all-orbits interpretation.**

   > “It neither classifies all minimizing orbits nor says that every maximizer has support at most six.”

   In Remark 4.8, the preceding sentence already says that the theorem establishes the existence of one sparse maximizer, and Theorem 4.6 states that same existential conclusion explicitly. The proof has constructed such a maximizer. No intervening argument claims a universal support bound. This additional denial asks the reader to revisit a quantifier that the positive statements have already settled. Delete this sentence; retain the positive existence statement and the following explanation of why the twelve-facet family remains useful. The denial would earn its place if a subsequent argument tried to apply the six-facet bound to every minimizing orbit, or if the text were correcting a previously stated universal claim.

2. **Likely: a second, unraised consequence of singularity.**

   > “A singular matrix by itself means neither infeasibility nor failure of the capacity formula.”

   Section 4.2 has just proved that the stationary affine solution set has a constant objective value and has identified the remaining task: testing whether it contains positive weights. That already addresses what singularity changes in the computation. Introducing “failure of the capacity formula” raises a substantially different doubt about an established theorem without any local reason for it. Delete this sentence, leaving the positive-feasibility requirement and the reference to the pentagon classification. It would be useful beside an actual singular example whose solver output had been mistaken for infeasibility or a contradiction to the theorem.

3. **Uncertain: advertising an avoided solver step after the alternative algorithm is already specified.**

   > “without solving KKT systems”

   Section 4.5 has already identified the product route as Algorithm 4.7 and described its support and objective enumeration. The additional phrase does not qualify the accuracy of the returned capacity; it emphasizes a computational task avoided by the implementation. A minimal edit is to end the sentence at “its sparse maximizing words.” The phrase would be useful if the section were explaining a measured runtime difference, solver selection, or a concrete limitation of the KKT route. Since the surrounding paragraph explicitly compares two computational routes, retaining this short distinction is also defensible; this is a lower-confidence concern.

## Qualifications worth retaining

- > “A stationary point can be a minimum or a saddle, so this calculation does not yet identify the maximum.”

  This directly limits the inference licensed by solving the displayed linear KKT system. The section promises a global optimum, so distinguishing stationarity from maximality is locally necessary.

- > “They need not hold for every feasible dual curve; what makes them valid for computing capacity is the existence of a boundary-realized global minimizer.”

  The transition restrictions come from boundary geometry, while the quadratic program includes feasible dual curves. This qualification explains why pruning can preserve the optimum despite discarding feasible candidates; deleting it would obscure the justification for the reduced search.

- > “The following surgery is performed on z as a competitor in the dual variational problem, not as an orbit already constrained to ∂K.”

  The proof starts with a boundary characteristic and then splits its velocities. This sentence establishes the admissible class during surgery, preventing the unsupported inference that each intermediate broken path must remain on the boundary. It is relevant even with the prior pure-facet minimizer result because the present proof must preserve the additional block bound before reconstructing a boundary orbit.
