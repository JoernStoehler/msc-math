# Writing-quality evidence investigation

Prepared during the September 17–18, 2026 discussion with Jörn. This is a
bounded research record, not a validated detector, authoring workflow, or
authorization to restart the paused thesis-production agents.

The question is whether existing human feedback and primary research can
shorten the route to detecting and repairing defects in AI-written
mathematical exposition. Distinguish detecting a true defect, detecting the
most consequential defect, predicting reader acceptance, and generating a
successful repair. Evidence for one does not automatically establish another.

Companion reports record local datasets, external detector research, and
external authoring-workflow research. Their claims remain limited to the
specified tasks, models, selection procedures, and outcome measurements.

## Additional reader-research sources

- Gopen and Swan, *The Science of Scientific Writing* (1990):
  <https://www.gatsby.ucl.ac.uk/~pel/misc/gopen_swan.pdf>.
  The article demonstrates revisions of scientific passages using reader
  expectations. It supplies candidate mechanisms involving context, emphasis,
  and sentence structure; it is not a randomized evaluation of an AI detector.
  Its authors explicitly caution against turning these principles into rigid
  rules. Use it to form hypotheses, not as evidence that a checklist guarantees
  acceptable prose.
- Inglis and Alcock, *Expert and Novice Approaches to Reading Mathematical
  Proofs* (2012): <https://mjinglis.github.io/files/JRME_EyeMovements.pdf>.
  Eighteen undergraduate students and twelve research-active mathematicians
  read six purported proofs. Expert readers made more transitions between
  adjacent lines, interpreted as greater attention to implicit warrants.
  Some validity judgments differed even among experts. The task was proof
  validation, not thesis-writing acceptance; backtracking cannot simply be
  labelled a prose defect. The paper also discusses how required verbalization
  can change the reading process being studied.
- Jakobi et al., *PoTeC: A German Naturalistic Eye-tracking-while-reading
  Corpus*: <https://arxiv.org/abs/2403.00506> (2024 preprint; 2025 journal
  reference). Seventy-five participants read twelve scientific texts, with
  expertise, comprehension and background-knowledge measurements. This offers
  richer reader-behavior observations than a prose score, but is neither an
  AI-writing defect dataset nor a collection of Jörn's acceptance judgments.

## Open comparisons

These are candidate investigations, not new claims established by the reports:

1. Minimal review request versus compact defect reminders versus contextualized
   examples. Hold reviewed passages fixed and inspect both misses and harmful
   criticisms. Do not presume longer prompts improve recognition.
2. Defect localization versus critique explanation versus repair utility.
   A plausible critique is not evidence that following it improves the text.
3. Accepted passages with annotations versus failing passages. A detector that
   rejects all imperfect writing does not reproduce the human threshold.
4. Full-context natural failures versus isolated synthetic corruptions.
   Keep provenance and selection mechanisms visible rather than pooling them
   as exchangeable test examples.

The local corpus is small and feedback is selective. An unmentioned span is
not a confirmed negative example, and several comments on one reading are
not independent trials. Fresh agents must not receive the human labels before
making a held-out prediction.
