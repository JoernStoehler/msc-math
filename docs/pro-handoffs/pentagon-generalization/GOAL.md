# Beyond the rotated regular pentagon

Find a mathematically valuable generalization or structural explanation of the
capacity of planar polygon Lagrangian products, starting from the exact
rotated-pentagon result below. This is a mathematical research request, not a
request to rewrite a thesis chapter or build software.

For the unit-circumradius regular pentagon P and Kθ = P × RθP, with
ω((q,p),(q′,p′)) = q·p′ − p·q′, the established profile is

    c_EHZ(Kθ) = (1 + cos(π/5))² / cos d(θ),
    d(θ) = min_{k ∈ Z} |θ − kπ/5|.

The accompanying proof is short: the feasible QP weights do not change with
rotation; every fixed candidate has value A cos θ + B sin θ; known endpoint
capacities bound every candidate by positive trigonometric interpolation; an
explicit candidate attains the bound. This raises the question of which part
is special to pentagons and what useful mathematics survives beyond them.

## Starting material

Read `math/rotated-pentagon-proof.tex` for the full proof and explicit witness.
`CONTEXT.md` fixes the conventions and distinguishes general ingredients from
pentagon-specific inputs. An optional tool is the accepted product-specific
support reduction in `math/product-six-facet-reduction.tex`: some global
maximizer uses at most three facets from each polygon. `SOURCES.md` gives
primary literature pointers, including neighboring results that could make an
apparent extension already known. `PROVENANCE.md` records the status of the
project material; it need not be read as part of the mathematical argument.

## Freedom to choose a productive question

Possible directions include unequal polygon factors, other regular polygons,
nonregular families, structural descriptions of angular capacity profiles,
or geometric conditions making an endpoint bound sharp. These are prompts
for exploration, not a required list or claims that these questions are open.
A different representation or question may be more valuable than extending
the displayed formula. A simple consequence of the provided ingredients is a
useful stepping stone, but not automatically a substantial new result.

Please pursue the most promising direction you find. A theorem with a useful
restricted scope, a counterexample to a tempting extension, or a precise
obstruction explaining why the pentagon proof cannot extend would all help.
The broader motivation is understanding capacity and systolic ratios of
Lagrangian products in relation to Viterbo's inequality. There is no demand to
prove that inequality generally (it is false) or to beat the known HKO ratio.

## Useful return

Give exact definitions, hypotheses, statements and proofs for any results you
obtain. Explain their relation to the supplied result and what geometric
insight they add. Keep proposed conjectures, numerical evidence, imported
results and proved conclusions distinct. For piecewise profiles, include
endpoint/degenerate cases and justify that no competing support has been
missed. Distinguish existence of a sparse optimizer from a description of all
optimizers. If a promising argument stops, identify its exact unproved step;
a well-explained obstruction is preferable to a fabricated completion.

The packet is deliberately small: no repository, dataset, implementation or
computational certificate is needed to read it. You may use computation or
consult sources when useful, but no computing campaign is prescribed. If
specific additional data would settle a consequential remaining question,
describe the smallest useful request. This is not an exhaustive literature
survey; qualify novelty claims accordingly.
