<!--
Purpose: submission/admin source index for the master-thesis handin.
Context: downloaded from the University of Augsburg MNTF Prüfungsamt page on
2026-04-24 so final handin requirements can be checked without rediscovering
the form links.
-->

# Thesis Submission Admin

Official source page:
<https://www.uni-augsburg.de/de/studium/organisation-beratung/pruefungen/infos-und-antrage/mathematisch-naturwissenschaftlich-technische-fakultat-pruefungen/>

## Downloaded Forms

Original PDFs are stored under `forms/pdf/`; cleaned Markdown conversions are
stored under `forms/md/`.

| Form | Local PDF | Local Markdown | Source URL |
| --- | --- | --- | --- |
| Hinweisseite zur Anmeldung der Bachelor-/Masterarbeit | `forms/pdf/hinweis-bachelor-masterarbeit-mntf.pdf` | `forms/md/hinweis-bachelor-masterarbeit-mntf.md` | <https://assets.uni-augsburg.de/media/filer_public/f0/b9/f0b923c7-1698-4136-af89-4810469d26e8/hinweis_bachelor-_masterarbeit-_mntf.pdf> |
| Anmeldung Bachelor- und Masterarbeit für die Studiengänge der MNTF | `forms/pdf/anmeldung-bachelor-masterarbeit-mntf.pdf` | `forms/md/anmeldung-bachelor-masterarbeit-mntf.md` | <https://assets.uni-augsburg.de/media/filer_public/bc/7f/bc7fa7ba-6648-4f37-9530-206a4bfb7904/protokoll_mntf_2026.pdf> |
| Erklärung zur Abgabe der Abschlussarbeit | `forms/pdf/erklaerung-abgabe-abschlussarbeit-mntf.pdf` | `forms/md/erklaerung-abgabe-abschlussarbeit-mntf.md` | <https://assets.uni-augsburg.de/media/filer_public/1f/5d/1f5d67ea-6121-488d-8f0e-a1bdcef414a8/1erklarung_abgabe_abschlussarbeit_mntf_mit_freiwilliger_cd-1.pdf> |
| Erklärung zur Einsichtnahme Dritter | `forms/pdf/erklaerung-einsichtnahme-dritter.pdf` | `forms/md/erklaerung-einsichtnahme-dritter.md` | <https://assets.uni-augsburg.de/media/filer_public/43/24/4324dafc-253b-4741-9d62-10398bbc4599/erklarung_zur_einsichtnahme_dritter-prufungsamt.pdf> |

## Current Handin State

- The Bachelor-/Masterarbeit registration form is already filled and signed by
  Kai.
- TODO(Jörn): hand in the registration form after Elizabeth agrees/signs. Jörn
  expects Monday 2026-04-27 as the earliest date because Elizabeth is currently
  away.
- TODO(Jörn): before final submission, verify the exact current Prüfungsamt
  requirements for printed-copy count, required forms, USB-stick contents, and
  any required portal/upload step against the latest letter or checklist.
- TODO(Jörn/Kai): name the non-GitHub repo backup destinations before the final
  GitHub archive step. Current candidate from Kai: Zenodo.

## Repository Preservation And Dissemination

Kai suggested Zenodo as a place to upload the finished work, including program
code, and make it generally accessible:

> Herr Pietschmann hat mich auf folgendes Repository aufmerksam gemacht:
> <https://zenodo.org/>
>
> Das waere vielleicht ein Ort, um Deine Arbeit (wenn sie dann fertig ist)
> inclusive Programmcode hochzuladen und allgemein zugaenglich zu machen.

Current interpretation:

- Zenodo is the leading non-GitHub preservation candidate because Kai named it.
- GitHub's documentation describes Zenodo as a way to archive a GitHub
  repository and issue a DOI for the archive.
- Zenodo's documentation covers GitHub/software archiving and software metadata.
- A shallow alternatives pass found two other classes, but no reason yet to
  spend broad search time before Kai's review:
  - OPUS Augsburg is the university's institutional repository and OpenDOAR
    lists "Theses and Dissertations" among its content types. It may be useful
    for thesis-only publication if the university route wants that.
  - Figshare is named by GitHub's repository-citation documentation as another
    place to publicize and cite research material. It is a generic fallback, not
    currently advisor-preferred.
- TODO(Jörn/Kai): decide whether the final repo closure includes a Zenodo
  deposit, a manual upload, GitHub release integration, or another preservation
  destination.
- TODO(Jörn/Kai): decide whether to look for additional institutional or
  subject-specific repositories; do this only if it changes preservation or
  advisor-facing value enough to justify the search time.

Reference links checked on 2026-04-24:

- GitHub Docs, "Referencing and citing content":
  <https://docs.github.com/en/repositories/archiving-a-github-repository/referencing-and-citing-content>
- Zenodo Docs, "GitHub and Software":
  <https://help.zenodo.org/docs/github/>
- OpenDOAR, "OPUS Augsburg":
  <https://opendoar.ac.uk/repository/3341>

Post-review dissemination candidates:

- arXiv upload of the thesis or a thesis-derived preprint. This is to be
  discussed with Kai after he has reviewed the thesis; do not decide it before
  that review. Expected work shape: autonomous formatting/submission packaging
  after the decision.
- Outreach mail to Haim-Kislev, Ostrover, and similar researchers. Decide this
  together with the arXiv decision, after Kai's thesis review.
- These dissemination actions are follow-up/publication work unless Jörn and
  Kai explicitly make one of them part of master-thesis closure.
