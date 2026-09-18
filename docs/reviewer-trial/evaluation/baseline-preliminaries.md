# Baseline review: preliminaries

Frozen predictions, before scoring. Audience: an MSc student with graduate linear algebra, real analysis and basic differential geometry, but no assumed knowledge of symplectic capacities or contact geometry. This review uses only the baseline prompt and supplied preliminaries excerpt. It excludes extraction artifacts; figure captions were available, but images were not.

## Concerns

1. **Some foundational geometric names arrive without their meaning.** Exact quotations: “The restriction α = λ₀|∂K is a contact form.” and “Thus Kq occupies the Lagrangian q-plane and Kp occupies the Lagrangian p-plane.” The characteristic line and Reeb normalization are explained well, but the reader is never told what makes a form contact or a plane Lagrangian. The displayed Reeb equations and coordinate definition make subsequent calculations usable; they do not explain these two new classifications. For this preliminaries audience, the omissions leave a gap in the promised common language, especially when the distinction between Lagrangian and symplectic planes becomes central. **Confidence: medium.**

2. **The curve space is used as a prerequisite without an introductory bridge.** Exact quotation: “A contact-normalized generalized characteristic on ∂K is a curve γ ∈ W¹,²([0, T], R⁴), with T > 0 and γ(0) = γ(T)”. The earlier action definition uses absolutely continuous curves, whereas the generalized characteristic definition introduces Sobolev notation without explaining its relationship to that familiar class. Graduate real analysis does not necessarily include Sobolev spaces. A reader without that background cannot immediately tell what extra regularity is required, or why endpoint values and the almost-everywhere velocity are meaningful. This same space then defines the dual optimization problem. **Confidence: medium.**

3. **The capacity's stated domain and cylinder normalization do not match on the page.** Exact quotations: “All convex bodies below are compact convex subsets of R⁴ with non-empty interior.” and “It is normalized by cEHZ(B⁴(r)) = cEHZ(Z⁴(r)) = πr², where B⁴(r) is the radius-r ball and Z⁴(r) = B²(r) × R² is the symplectic cylinder.” The capacity has been introduced by a boundary least-action description for convex bodies, and the axioms are explicitly introduced as those of a capacity “on convex bodies.” The cylinder is unbounded. An experienced symplectic reader can supply the broader capacity framework, but the intended newcomer is left to infer which extension makes this displayed value meaningful. The note that the axioms are only background reduces the downstream impact, but does not resolve the domain mismatch. **Confidence: medium.**

## Effective passages worth retaining

1. “Thus action is the signed symplectic area enclosed by the curve.” Following the explicit primitive and Stokes calculation, this gives the action a geometric interpretation before introducing the boundary dynamics.

2. “In block notation J₀(u, v) = (−v, u). Consequently, the Reeb direction associated later with a supporting row in one Lagrangian factor moves in the other factor.” This ties the coordinate convention to a concrete dynamical consequence and explains why the product decomposition matters.

3. “Feasibility allows us to compare functional values, but does not place a curve on the boundary. Once a modified curve is shown to minimize IK, the variational equation and reconstruction justify placing it there.” This identifies precisely which conclusion needs optimality and makes the purpose of the dual construction clear for the next section.

## Missing context

An earlier definition of contact forms, Lagrangian subspaces or Sobolev curve spaces would materially weaken concerns 1 and 2. An earlier definition of capacities on unbounded domains would weaken concern 3. None is supplied here. The actual figures were unavailable, so this review makes no judgment about their visual effectiveness. The cited variational results were not inspected; their correctness and exact hypotheses are outside this writing review.
