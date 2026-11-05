# HKO extremality under a Minkowski asymmetry bound

The [research note](asymmetry.md) proves, for the fixed regular pentagon Q and
any convex planar partner K,

    sys(K×_LQ) ≤ [(10+6√5)/25] [1+s(K)]/2,

where s(K) is the translation-minimized Minkowski asymmetry. Consequently every
partner with s(K)≤sec(π/5) satisfies the sharp HKO ratio bound. Equality occurs
exactly for regular pentagons at the HKO relative angles. In particular, a
partner beating HKO must have strictly greater Minkowski asymmetry than a
regular pentagon. A violation of sys≤1 requires s(K)>(15√5−29)/4.

This is an analytic deduction from the existing sharp symmetric-partner theorem
and a mixed-area estimate. The equality proof uses positivity of polygon edge
contributions and the cyclotomic relation Φ10. It also supplies a shorter proof
of the ratio bound/equality for independent affine regular factors; their full
capacity formula remains a separate result. Global optimality among partners
of larger asymmetry is still open.

[verify_asymmetry.py](verify_asymmetry.py) uses the existing Python capacity
oracle and a separate asymmetry LP. All twenty retained controls in
[controls.json](controls.json) pass. These are floating-point controls, with no
exact-capacity or global-search interpretation. Reproduction and output effects
are specified in the research note. [review.md](review.md) records independent
agent mathematical and computational review, without human acceptance or a
novelty determination.
