# Core reader A: printed pages 5–28

Reviewed on 18 September 2026 against the frozen `thesis-v1-5458356d.pdf` in this directory. Coverage: all prose, definitions, displayed arguments and figures on printed pages 5–28, read in order from the Introduction through Section 4.5; every page was also inspected as a rendered image. Page references below are printed page numbers, which agree with PDF page numbers in this snapshot.

The review used the PDF and its preceding definitions as the reader's mathematical context. It followed the presented reasoning and checked whether the needed objects, transitions and qualifications were supplied. It did not independently audit the cited papers, theorem certificates, implementation, later chapters or bibliography. No chapter source was changed.

Two minor findings are recorded below. No substantial exposition or proof-legibility defect was established in this assigned portion. That is bounded reader evidence, not a whole-thesis acceptance judgment or an independent theorem verification.

## A1. Specify the cylinder's bounded coordinate plane

- **Page and anchor:** p. 10, Section 2.3: “`Z^4(r) = B^2(r) × R^2` is the symplectic cylinder.”
- **Severity:** Minor notation ambiguity with mathematical consequences if read literally.
- **Why it matters:** Section 2.1 fixes coordinate order `(q1,q2,p1,p2)`. A Cartesian product in that order places the disk in the Lagrangian `q`-plane. The intended normalized symplectic cylinder bounds a symplectic plane, such as `(q1,p1)`. The distinction is subsequently emphasized in Section 2.5, so the cylinder should follow the same explicit convention.
- **Minimal repair:** Define the cylinder directly as `Z^4(r) = {(q1,q2,p1,p2) : q1^2 + p1^2 ≤ r^2}`, with `(q2,p2)` unrestricted, or explicitly label the two factors by their coordinate planes. Use the thesis's chosen open/closed ball convention consistently.
- **Confidence and scope:** High confidence that the expression is ambiguous in the declared coordinates; the intended capacity normalization is clear from the words “symplectic cylinder.” No later proof change is indicated.

## A2. The simple-minimizer overview describes the wrong rescaling direction

- **Page and anchor:** p. 18, overview after Theorem 3.4: “Shortening all durations then restores the constraint `A(z) = T` and lowers the dual value.”
- **Severity:** Minor proof-exposition contradiction; the detailed argument supplies the correct calculation.
- **Why it matters:** On p. 20 the rescaling factor is `β_n = T/A(z_{3,n})`. Dual minimality then proves `A(z_{3,n}) ≤ T`, hence `β_n ≥ 1`. In the actual approximating sequence, rescaling lengthens durations or leaves them unchanged. The overview prepares the reader for an operation with the opposite direction and an improvement the constructed sequence does not make.
- **Minimal repair:** Replace the two sentences beginning “Shortening” and “Starting near the minimum” with: “Rescaling all durations restores the action–period constraint. Dual minimality controls the possible action change, and a convergent subsequence then supplies a minimizer with the desired finite form.” The displayed proof can remain as written.
- **Confidence and scope:** High confidence: the mismatch is internal to pp. 18 and 20 and follows directly from the displayed inequalities.

## What this review supports

The inspected portion explains the distinction between closed dual loops and boundary-realized orbits; provides the dual reconstruction and its normalization; tracks approximation, splitting, merging and the limiting simple orbit; explains how all active words and singular stationary systems enter the global comparison; and distinguishes the twelve-facet billiard-derived family from the six-facet product maximizer. These connections were traceable from preceding material during this reading.

Both recorded repairs are local and should take less than 30 minutes together, including rebuilding and checking the affected pages. The review provides no evidence of a revision exceeding the two-hour minor-fix allowance within pp. 5–28. It does not estimate the revision burden of the remaining manuscript or establish that all substantive defects have been found.
