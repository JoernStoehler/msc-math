<!--
Purpose: submission, preservation, dissemination, and final archive roadmap.
Context: external-clock task bundle for thesis closeout.
-->

# Submit Thesis Roadmap

## Status

- State: external clock.
- Last updated: 2026-05-08.
- Source surfaces: `tasks/verify-thesis-done.md`,
  old harness extraction: verification packet candidate `submission-artifacts-are-complete.md`,
  `tasks/submit-thesis/`, `tasks/MAP.md`.
- Refresh when: Prüfungsamt, Kai, Elizabeth, Zenodo, arXiv, or final archive
  facts change.

## Steering Cache

- [accepted 2026-04-24] Final closure means no further direct repo-related
  master-thesis action remains; the final GitHub archive/read-only action is
  the last direct repo action.
  Source: Jorn, `tasks/verify-thesis-done.md`.
  Why it matters: keeps later publication/research/admin observations out of
  the thesis-project done definition unless explicitly promoted.
- [external 2026-04-24] The Bachelor-/Masterarbeit registration form is already
  filled and signed by Kai.
  Source: Jorn.
  Why it matters: next action waits on Elizabeth agreement/signature, not form
  preparation.
- [external 2026-04-24] University handin details must be verified from the
  current Ausgabebescheid/checklist: printed-copy count, form names, USB/CD
  contents, and upload mechanics.
  Source: MNTF forms copied under `tasks/submit-thesis/`.
  Why it matters: agents can index forms, but Jorn must verify the current
  official handin facts.
- [accepted 2026-04-24] Zenodo is the leading non-GitHub preservation candidate
  because Kai named it.
  Source: Kai email:
  "Herr Pietschmann hat mich auf folgendes Repository aufmerksam gemacht:
  <https://zenodo.org/>
  Das waere vielleicht ein Ort, um Deine Arbeit (wenn sie dann fertig ist)
  inclusive Programmcode hochzuladen und allgemein zugaenglich zu machen."
  Why it matters: repository preservation is part of final archive prep.
- [accepted 2026-04-24] arXiv upload and outreach mails to Haim-Kislev,
  Ostrover, and similar researchers are post-Kai-review dissemination
  candidates unless Jorn/Kai explicitly promote one into thesis closure.
  Source: Jorn/Kai discussion from the old submission admin note.
  Why it matters: prevents public-dissemination work from silently blocking
  thesis handin.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Registration handin | `[external]` | external clock | Jorn / Elizabeth | Hand in already-filled form after Elizabeth agrees/signs; earliest known date was Monday 2026-04-27. | Steering Cache |
| Final university checklist | `[Jorn]` | external clock | Jorn | Verify exact Prüfungsamt copy count, forms, USB/CD contents, and upload mechanics from current letter/checklist. | `tasks/submit-thesis/`, `submission-artifacts-are-complete.md` |
| Non-GitHub preservation | `[Jorn]` | external clock | Jorn/Kai | Choose Zenodo/GitHub integration, manual upload, OPUS Augsburg, Figshare, or another target before final archive. | Steering Cache |
| Final archive | `[blocked]` | mainline thesis | thesis-done gate | Identify final commit/tag, preserve required copies, then archive/read-only GitHub as last direct repo action. | `tasks/verify-thesis-done.md` |
| arXiv/outreach | `[future]` | future/follow-up | Jorn/Kai after review | Decide after Kai reviews thesis; keep out of closure unless promoted. | Steering Cache |

## Submission Admin Sources

Official source page:
<https://www.uni-augsburg.de/de/studium/organisation-beratung/pruefungen/infos-und-antrage/mathematisch-naturwissenschaftlich-technische-fakultat-pruefungen/>

Downloaded on 2026-04-24 so final handin requirements can be checked without
rediscovering the form links.

| Form | Local PDF | Local Markdown | Source URL |
| --- | --- | --- | --- |
| Hinweisseite zur Anmeldung der Bachelor-/Masterarbeit | `submit-thesis/hinweis-bachelor-masterarbeit-mntf.pdf` | `submit-thesis/hinweis-bachelor-masterarbeit-mntf.md` | <https://assets.uni-augsburg.de/media/filer_public/f0/b9/f0b923c7-1698-4136-af89-4810469d26e8/hinweis_bachelor-_masterarbeit-_mntf.pdf> |
| Anmeldung Bachelor- und Masterarbeit für die Studiengänge der MNTF | `submit-thesis/anmeldung-bachelor-masterarbeit-mntf.pdf` | `submit-thesis/anmeldung-bachelor-masterarbeit-mntf.md` | <https://assets.uni-augsburg.de/media/filer_public/bc/7f/bc7fa7ba-6648-4f37-9530-206a4bfb7904/protokoll_mntf_2026.pdf> |
| Erklärung zur Abgabe der Abschlussarbeit | `submit-thesis/erklaerung-abgabe-abschlussarbeit-mntf.pdf` | `submit-thesis/erklaerung-abgabe-abschlussarbeit-mntf.md` | <https://assets.uni-augsburg.de/media/filer_public/1f/5d/1f5d67ea-6121-488d-8f0e-a1bdcef414a8/1erklarung_abgabe_abschlussarbeit_mntf_mit_freiwilliger_cd-1.pdf> |
| Erklärung zur Einsichtnahme Dritter | `submit-thesis/erklaerung-einsichtnahme-dritter.pdf` | `submit-thesis/erklaerung-einsichtnahme-dritter.md` | <https://assets.uni-augsburg.de/media/filer_public/43/24/4324dafc-253b-4741-9d62-10398bbc4599/erklarung_zur_einsichtnahme_dritter-prufungsamt.pdf> |

Reference links checked on 2026-04-24:

- GitHub Docs, "Referencing and citing content":
  <https://docs.github.com/en/repositories/archiving-a-github-repository/referencing-and-citing-content>
- Zenodo Docs, "GitHub and Software":
  <https://help.zenodo.org/docs/github/>
- OpenDOAR, "OPUS Augsburg":
  <https://opendoar.ac.uk/repository/3341>

## Agent Cache

- [fresh 2026-04-24] Downloaded MNTF PDFs and cleaned Markdown conversions live
  under `tasks/submit-thesis/`.
  Refresh by: rechecking the MNTF Prüfungsamt page linked from
  the Submission Admin Sources section.
- [fresh 2026-04-24] Shallow preservation alternatives pass found OPUS Augsburg
  and Figshare, but no reason to broaden search before Kai review.
  Refresh by: checking the Submission Admin Sources links and asking
  Jorn/Kai whether advisor-facing value changed.

## Pruned / Stale

- [moved 2026-04-24] Finish-mode admin facts moved from the old tracker and
  deleted `FINISH.md` into this bundle.
