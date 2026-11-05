```
Act as a skeptical thesis editor. In one consultation of about 1500–2200 words, diagnose why the current introduction fails an MSc mathematics reader who has no project context. Focus on whether the problem and contribution narrative is significant and coherent, citing specific passages and separating what I observe from what I think caused it. Then propose one introduction structure and a substantial prose exemplar. The exemplar may use only contents the thesis supports and must disclose any premise it needs that the thesis does not support. End with remaining uncertainties.
```
Assumptions: no tools or skill loading (the brief forbids tools). The exemplar is in LaTeX, so the next writer can adapt it directly.

---

## 1. Diagnosis

**Observation: the reader meets the claims before the question.** The first paragraph says the thesis "contributes to computational symplectic geometry by studying how the shape of a four-dimensional convex body affects its symplectic capacity relative to its volume". At that point the reader has no idea what a capacity is, why its ratio to volume is interesting, or what is open about it. Paragraphs 2 and 3 then describe methods ("The empirical approach lets us examine populations…") and AI use ("This makes the project an occasion to examine their use…"). Both are written for a problem the reader has not yet been given.

Viterbo's conjecture arrives only in paragraph 4. The actual research question comes at the end of paragraph 5: "which bodies have large systolic ratio, and which changes of shape increase it?" That is the best sentence in the introduction, and it is buried.

**Plausible cause.** Jörn's first comment was that the earlier candidate opened with previous work instead of motivating the contribution. The revision appears to have answered this by *moving* contribution sentences to the front, not by *motivating* them. Motivation means the reader understands why this field's question needs empirical methods and why AI is worth discussing. A declaration that the thesis does these things is not motivation. I do not read Jörn's two short comments as his full critique. They are consistent with a deeper problem: the text has no argument running through it.

**Observation: the two method paragraphs are defensive and abstract.** Paragraph 2 gives its caveats ("A numerical association need not be a universal law, and a search that finds no improvement does not establish a local maximum") before reporting any finding. The caveats are correct, but here they read as hedging against an unstated claim. "Proofs for restricted families test how far a pattern can extend" is a methodology slogan with no pattern attached to it.

Paragraph 3 justifies the AI theme only by the fact that agents were used. It gives no hint of what was learned. The reflection chapter has real content that could be previewed:
- a model developed essentially the whole local-maximality argument from an incomplete suggestion;
- readable exposition was much harder to delegate;
- the author offers a hypothesis about the criterion of success that agents pursue.

**Observation: the promised themes are not delivered in Contributions.** The opening names three themes: geometric results, empirical methods, and AI. The Contributions subsection substantiates only the first.

The empirical results get one compressed sentence, appended to a paragraph about algorithms ("In random samples, there is a strong inverse rank association…"). The most interesting empirical results are missing entirely:
- the balancing intervention, which changes the shapes and moves the mean ratio from 0.342 to 0.654 (`16-conclusion.tex`);
- the opposite exact laws for pentagon and triangle rotations;
- the search comparison.

AI does not appear in Contributions at all. So the opening claims a focus that the body of the introduction drops. A reader would rightly feel the narrative is incoherent.

**Observation: results are stated without their consequences.** Each theorem is reported at its technical resolution but not at its meaning. Some examples:
- The second local maximum is "non-strict, with a two-parameter family of equality cases". The conclusion supplies what it means: the local-maximality phenomenon "is not confined to the HKO orbit". The introduction omits that.
- The symmetric-partner bound is given as a number. Its consequence is that symmetry of the partner rules out an HKO-sized ratio, while asymmetric partners stay open. That is also omitted.
- The rotation theorem's consequence is that neither the symplectic nor the Lagrangian placement need be optimal. Omitted.

The reader gets a catalogue and must reconstruct why each item matters for the question.

**Observation: the motivating background is filed as "literature".** Three facts tell an MSc reader why the inequality matters:
- the Gromov width satisfies it trivially, so the EHZ case is the substantive one;
- on L×L° it becomes the Mahler inequality;
- EHZ equals cylindrical capacity in dimension 4.

These appear only after the technical contributions, under "Relation to the literature". Placed earlier, they would motivate the question. The introduction also never says informally what EHZ capacity is, even though one clause would do: "least action of a closed orbit of the characteristic flow on the boundary" (`02-preliminaries`).

**Observation: there is no "why is this hard / why compute" step.** The thesis's own logic for empirical methods is available but never stated as an argument:
- explicit capacity formulas exist only for special families;
- for polytopes, the Haim–Kislev formula makes capacity a finite computation;
- so one can study populations and run searches.

That chain is exactly the motivation Jörn asked for. The current text has only its fragments.

**On the reviewer approval.** The approval checked factual alignment and fit, which a stack of individually correct sentences can pass. The human judged whether a reader is led through a problem to its answers. These are different criteria, and only the second makes an introduction.

## 2. Recommended structure

1. **Opening statement (one short paragraph).** State the question, why it is newly open, and the thesis's two distinctive angles: computation in an experimental role, and AI agents as both instrument and subject. Keep it concrete and free of undefined terms.
2. **The problem.** Explain capacity informally, Viterbo's inequality, the systolic ratio, and why it matters (Gromov width is trivial; the Mahler connection; EHZ equals cylindrical capacity).
3. **What changed.** Present HKO and the questions it leaves open.
4. **Why compute, and the limits of computation.** Explain the finite formula, what populations and searches let us do, and why proofs are still needed. Put the caveats *here*, attached to the method.
5. **AI in the research.** Say what was delegated, what was hard, and point to the reflection and the separate disclosure.
6. **Results, grouped by question, each ending in its consequence:**
   - local structure near HKO;
   - families;
   - placement of factors;
   - the empirical picture;
   - computational foundations, briefly.
7. **Reading guide.** Short, including which chapters can be skipped.

The literature subsection disappears as a separate block. Its content goes into items 2–3, and the Haim–Kislev cut paper goes in item 6, with a pointer to `subsec:hko-related-local-results`.

## 3. Prose exemplar

```latex
\section{Introduction}\label{sec:introduction}

This thesis asks how the shape of a convex body in $\mathbb R^4$ controls
the ratio of its symplectic capacity to its volume. The question was
reopened by the recent counterexample to Viterbo's conjecture, which shows
that the ball is not the extremal shape but says little about which shapes
are. Alongside proofs, the thesis uses computation in an experimental role:
capacities of many polytopes are computed and compared with their geometry,
and search procedures look for bodies with large ratio. The work was carried
out with extensive use of AI agents, and how such agents can be used in
mathematical research is one of its subjects.

\subsection*{The capacity--volume problem}

Symplectic capacities measure constraints on embeddings that volume alone
cannot detect. We use the Ekeland--Hofer--Zehnder capacity
$c_{\rm EHZ}$, which for a convex body $K$ is the least action of a
closed orbit of the characteristic flow on $\partial K$
(Section~\ref{subsec:preliminaries-smooth-symplectic-geometry}). In
dimension four, Viterbo's conjecture~\cite{Viterbo2000} asserted that
\[
 \operatorname{sys}(K):=\frac{c_{\rm EHZ}(K)^2}{2\operatorname{vol}_4(K)}\leq1
\]
for every convex body $K$, with equality for the ball. The \emph{systolic
ratio} $\operatorname{sys}$ is unchanged by translations, positive
dilations and linear symplectic maps. For the Gromov width the analogous
bound follows from volume preservation; for $c_{\rm EHZ}$, which on convex
bodies in $\mathbb R^4$ coincides with the cylindrical
capacity~\cite[Corollary~1]{AbbondandoloEdtmairKang2024}, it is a genuine
statement about closed orbits. It also contains a classical problem of
convex geometry: for a centrally symmetric planar body $L$ and its polar,
$c_{\rm EHZ}(L\times L^\circ)=4$~\cite{AKO2014}, so on these products the
inequality is the symmetric Mahler inequality
$\operatorname{area}(L)\operatorname{area}(L^\circ)\geq8$.

Haim-Kislev and Ostrover showed that the inequality fails: the product
$K_{\rm HKO}$ of a regular pentagon in the $q$-plane with its rotation by
$-\pi/2$ in the $p$-plane has
$\operatorname{sys}(K_{\rm HKO})=(3+\sqrt5)/5>1$
\cite{HaimKislevOstrover2024}. The counterexample turns a conjecture into a
landscape problem. Which bodies have large systolic ratio, and which changes
of shape increase it? Can $K_{\rm HKO}$ be improved by a small deformation,
or by changing the pentagons within a natural family?

\subsection*{Why compute, and what computation cannot settle}

Capacity is defined by a minimum over closed orbits, and explicit formulas
are known only for special families. For convex polytopes, however,
Haim-Kislev's formula~\cite{HK2017} reduces the capacity to a finite
quadratic program. This makes capacity a quantity one can evaluate on
thousands of bodies: one can sample populations of polytopes beyond the
solved families, ask which geometric measurements predict a large ratio,
and compare procedures that try to increase the ratio at fixed volume.
Evidence of this kind proposes questions but does not answer them. A strong
association across a population need not be a law, and a search that finds
no improvement does not show that a body is a local maximum. The thesis
therefore pairs experiments with proofs on restricted classes, where a
suggested behaviour can be established or bounded.

\subsection*{AI agents in the research}

AI agents took part in developing the mathematical arguments, software and
experiments of this thesis. Section~\ref{sec:use-of-ai} reflects on that
experience. Its central contrast is that a model developed essentially the
entire local maximality argument for $K_{\rm HKO}$ from an incomplete
suggestion, whereas directing agents to complete ordinary tasks and to
produce readable exposition was much harder. The section proposes, as a
hypothesis, that the difficulty concerns which criterion of success an
agent pursues. The separate disclosure records the extent and provenance of
AI use.

\subsection{Results}

\paragraph{Local structure near HKO.}
Among polytopes with ten facets, $K_{\rm HKO}$ is a strict local maximum of
the systolic ratio up to the symmetries above
(Theorem~\ref{thm:hko-ten-facet-local-maximum}): no sufficiently small
ten-facet deformation increases its ratio. The proof bounds the capacity of
nearby bodies by finitely many explicit feasible values of the finite
capacity problem, whose first derivatives force a strict decrease in every
direction transverse to the symmetries
(Sections~\ref{sec:first-order-perturbations}
and~\ref{sec:hko-local-maximum}). Since a small hyperplane cut adds a facet,
the theorem does not cover the cuts studied in~\cite{HaimKislev2025Zoll};
whether $K_{\rm HKO}$ stays locally maximal when the facet number changes is
open. Local maximality in this space is not special to HKO: a rational
hexagon--quadrilateral product with ratio one is also a (non-strict) local
maximum (Theorem~\ref{thm:non-hko-ten-facet-local-maximum}). It is not a
counterexample, but it shows that the ten-facet landscape has more than one
peak.

\paragraph{Families of products.}
Large deformations within natural families do not beat HKO either. For
products of independently linearly deformed regular pentagons, the sharp
upper bound is $(3+\sqrt5)/5$, attained only at images of $K_{\rm HKO}$
(Theorem~\ref{thm:pentagon-affine-capacity}). If one factor is a regular
pentagon and the other is any centrally symmetric planar convex body, the
ratio is at most $(10+6\sqrt5)/25<1$
(Theorem~\ref{thm:pentagon-symmetric-partner}), so a symmetric partner
cannot produce a counterexample; asymmetric partners remain open. Rotated
regular-pentagon products have an explicit capacity formula
(Theorem~\ref{thm:rotated-pentagon-product-formula}).

\paragraph{Placement of the factors.}
Even with the planar shapes fixed, their position matters: a small rotation
of any product of two planar convex bodies out of symplectic position
increases capacity at fixed volume
(Theorem~\ref{thm:product-initial-rotation}). Neither the symplectic nor
the Lagrangian arrangement need be the best position of a given product.

\paragraph{The empirical picture.}
In random samples, the total symplectic area of the two-dimensional faces,
normalized by $\sqrt{\operatorname{vol}_4}$, strongly predicts the systolic
ratio in rank: smaller area goes with larger ratio. The association weakens
substantially among low-area or high-ratio bodies, so it selects good
populations better than it guides improvement within them. Changing the
shapes tests the association directly. Balancing each factor of 320
products by a determinant-one map raised the mean ratio from $0.342$ to
$0.654$ and weakened the rank correlation from $-0.929$ to $-0.249$; factor
shape thus contributes materially to the association, without being
identified as its sole cause. Exact formulas on two rotation families show
that even the direction of the relationship depends on the family: regular
pentagons obey an inverse-square law, equilateral triangles the opposite
ordering. Among local searches, retaining previously active branches
outperformed literal gradient ascent on a common set of starts, but high
terminal values did not certify local maxima. No sample or search produced
a numerical ratio above one (Section~\ref{sec:black-box-datascience}).

\paragraph{Computational foundations.}
We give an expanded derivation of the Haim-Kislev formula and show that for
Lagrangian products some maximizer uses at most three facet directions per
factor (Sections~\ref{sec:preliminaries}--%
\ref{sec:quadratic-program-algorithm-hk2019}). A second algorithm, based on
the passage geometry of Chaidez and Hutchings~\cite{CH2021}, is proved
correct in exact real arithmetic under explicit regularity hypotheses
(Section~\ref{sec:flow-graph-algorithm-ch2021}).

\subsection{Reading guide}

[Keep the current outline, shortened; retain the note that
Section~\ref{sec:hko-local-maximum} needs neither the flow-graph nor the
data chapter.]
```

## 4. Remaining uncertainties and unsupported premises

**Premises the exemplar relies on:**
- **"Thousands of bodies."** I have not seen the sample sizes. Replace this with the actual scale, or with "many".
- **"Reopened by the recent counterexample."** This is an interpretive framing, not a historical claim, but check that it matches Jörn's view of the field.
- **The informal capacity gloss** ("least action of a closed orbit of the characteristic flow") compresses the preliminaries. The nonsmooth case uses generalized characteristics, and the definition is deferred.
- **The empirical results paragraph** reports conclusion-chapter numbers whose sampling-group details I cannot check. The "64 starts / median 0.985 vs 0.881" figures are omitted deliberately; include them only if Jörn wants that level of detail.
- **"Selects good populations better than it guides improvement"** is my paraphrase of the chapter's "two tasks" distinction in `08-data-science.tex`. It is supported in substance; the wording is mine.

**Status questions the introduction cannot resolve on its own:**
- The disclosure says the affine-pentagon argument has agent review, not completed personal verification. The exemplar states that theorem without qualification, as the current thesis does. Jörn should decide whether the introduction mirrors that status.
- The `08-data-science.tex` excerpt contains a `% CHECK` comment about how the balancing result is pooled. Resolve it before the introduction quotes those numbers.
- The current outline says "Sections …–… treat the product families", but I cannot see the section order, so the reading guide is left as a placeholder.

**Interpretive questions:**
- I cannot tell whether Jörn wants the AI theme as prominent as the opening paragraph makes it. His first comment suggests yes, but his second comment does not confirm that it is enough.
- Project-facts item 9 warns against forcing a single tight narrative. The exemplar uses the HKO landscape question as the frame and groups the results under it without claiming one result builds on another. If that frame feels forced to Jörn, the "Results" paragraphs can stand as parallel directions under the same opening.
