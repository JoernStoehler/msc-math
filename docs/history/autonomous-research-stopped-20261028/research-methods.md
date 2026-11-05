# AI-assisted mathematics: bounded methods scan

Research snapshot: 2026-10-28 UTC. Prepared for the authorized local autonomous
Viterbo/convex-geometry loop. Temporary research input; root owns integration,
retention, commits and cleanup. No maintained instructions changed.

## What the primary sources actually support

**Aletheia, March 2026.** Feng et al., [*Towards Autonomous Mathematics
Research*, v3](https://arxiv.org/html/2602.10177v3), §§2.2–2.3, 3, 6.
The system separates generation, verification and revision in natural language,
with bounded retries and an option to return failure. Authors report mathematical
content generated autonomously for an eigenweights paper and useful strategic
ideas for independence-polynomial bounds; humans wrote final papers. They report
four resolved questions in a 700-problem Erdős sweep. These are author-reported
research contributions, not claims to a landmark advance. A natural-language
verifier provides no formal guarantee. Search reduced fictitious references but
left misquotation of real results. Their explanations of why separated
verification helps are explicitly hypotheses. [Released prompts and
outputs](https://github.com/google-deepmind/superhuman/tree/main/aletheia)
are available; this scan did not audit those transcripts.

**AlphaEvolve, December 2025 revision.** Georgiev, Gómez-Serrano, Tao and Wagner,
[*Mathematical exploration and discovery at scale*, v3](https://arxiv.org/html/2511.02864v3),
§§1.3–1.6 and finite-field Kakeya case.
Rather than ask an LLM to enumerate objects, they evolve programs that search
for or improve constructions under a fixed evaluation budget. A generalizer
mode evaluates across multiple parameter values. In one Kakeya pipeline,
AlphaEvolve found a construction, Deep Think derived a formula and proof, and
AlphaProof formalized it in Lean. The paper covers failures as well as successes.
Its arithmetic Kakeya example illustrates reward mismatch: evaluation on square
inputs encouraged solutions for squares despite asking for all inputs. This
supports careful evaluator design, not a theorem that finite testing generalizes.

**Station, September 2026 revision.** Chung, Du and Wesley,
[*Autonomous Mathematical Discovery in an Open-World Multi-Agent Environment*,
v2](https://arxiv.org/html/2608.23691v2), §§2–4 and Appendix A.4.
Authors report new results on five of twelve construction problems, including
finite-field Kakeya families and exact 604-point kissing configurations in
dimension 11. Agents select directions, experiment, exchange reviewed papers and
reuse mathematical explanations. The sign-uncertainty case escaped an evaluator's
restricted function family after analyzing its limitations. Periodic reflection,
stagnation lanes and multistart were implemented; that does not establish each
mechanism's individual causal value. Typical six-agent runs lasted one to two
weeks, with substantially more resources than this local loop. Novelty is
reported relative to checked prior literature; this scan did not independently
verify their proofs or priority. [Dialogs and proofs](https://dualverse-ai.github.io/station_data_v2/)
are published.

**First Proof, October 14, 2026: independent capability evidence.** Abouzaid,
Srivastava, Ward and Williams, [*First Proof Third Batch*, v1](https://hexagonmath.org/pdf/2610.00244v1),
§§2, 4–5, 7. Ten human-solved, unpublished research problems were tested with
roughly thirty expert referees. Correctness, exposition and attribution were
graded separately. ProofCouncil solved 9/10 and single-query GPT-6 Astra 7/10;
reported total costs were $1,888 and $36 respectively. These API costs are
historical benchmark measurements, not our quota or authorization. Strong
correctness did not eliminate weak exposition or citation gaps. Systems had
different models, tools and budgets, so the comparison does not isolate a
benefit from orchestration. This tests proving specified statements, not choosing
valuable questions. Public [results, reports and logs](https://1stproof.org/third-batch.html)
allow follow-up. No live deployment or API purchase is proposed.

**Concrete prompting record, September 2026 revision.** Li et al.,
[*A Human-AI Collaborative Workflow for Mathematical Discovery*, v3](https://arxiv.org/html/2512.09443v3),
§§3–5 and Appendix A.3. This smaller case study documents hypothesis control,
prove-or-disprove queries, invariant-subspace and retraction discovery, numerical
checks and expert proof reconstruction in Grover-compatible Riemannian
optimization. Prompt records and failed routes are included. Its authors
explicitly state that generalizability beyond the documented cases remains
untested. Treat its templates as concrete experiments to adapt, not established
best practice for our domain.

## Proposed protocol for this loop — local hypotheses, not established results

The following is my synthesis for this repository. It changes no project policy
and adds no spending or approval gate.

1. **Choose one mathematical bottleneck per cycle.** Write the exact claim,
   quantifiers, representation, assumptions and closest established result.
   Name what would count as a useful negative result. Avoid a broad request to
   "solve Viterbo": the repository already studies a counterexample and several
   distinct related questions.
2. **Test truth before polishing proofs.** Check normalization, symplectic sign,
   scale invariance, product type, redundant facets, zero weights and degeneracy
   where relevant. Request both a proof route and the smallest plausible
   counterexample. Keep a failing example even if the proposed theorem is revised.
3. **Separate cheap discovery from certification.** Reuse existing producers and
   a fixed short budget for numeric reconnaissance. Save the actual polytope,
   word, weights, seed, parameters and evaluator output. A favorable capacity
   candidate is not automatically a certified capacity value: a feasible Q
   value gives an upper bound on c, which cannot by itself prove a lower bound
   on the systolic ratio. Verify the required bound direction before promotion.
4. **Ask for structure after an improvement.** What symmetry, active constraint,
   factorization or exact identity explains it? Can it be parameterized? Derive
   a statement valid for a specified family, then prove that statement separately.
   Rational reconstruction and exact finite checks establish only what their
   explicit certificate covers.
5. **Verify the candidate independently.** Give a reviewer the final candidate,
   definitions, cited theorem statements and computational artifacts. Ask it
   to locate the earliest unsupported implication, missing case, or wrong
   inequality direction. Return complete / local repair / structural failure /
   unresolved with the precise obligation. Agreement between models is evidence
   of review, not proof.
6. **Distill before selecting the next cycle.** Preserve the claim, new idea,
   proof status, actual checked cases, failed route, source pinpoints and next
   discriminating question. After repeated nonprogress, explicitly reconsider
   assumptions or representation, or change to a different promising bottleneck;
   another random seed alone does not explain stagnation.

Suggested investigator prompt:

> Under these definitions and hypotheses [paste], prove or disprove [exact
> statement]. Start with the weakest link and small/degenerate cases. Propose
> at most two materially different routes and one cheap discriminating check.
> Separate known theorems, new lemmas, numerical observations and conjectures.
> Preserve any counterexample. If unresolved, identify the smallest remaining
> obligation and what would settle it.

Suggested reviewer prompt:

> Independently check this candidate against the stated definitions and sources.
> Test the direction of every bound used to support the target conclusion.
> Check quantifiers, zero/degenerate cases, sign/scale conventions and the scope
> of certificates. For each cited result, verify its statement and hypotheses.
> Report the first consequential gap or give a complete dependency argument;
> label anything you could not check.

## Read depth and remaining uncertainty

Inspected repository AGENTS.md, README.md, collaboration rationale, literature
README and research guide. Read primary abstracts and selected methods/results
sections above; this is a bounded scan, not a systematic literature review.
Did not reproduce external constructions, run external agents, audit all logs,
or establish source-paper correctness/novelty. GitHub fetching of Anthropic's
math-proof plugin failed through the web tool; the First Proof report describes
that workflow, but I make no claims about its current implementation. Existing
repository conventions and root's selected mathematical programme govern use.
