# opportunity-official-status — bounded read-only review

Reviewed 2026-10-22 UTC in `/workspaces/msc-math`, HEAD `9667e351514faf1f00e2d86796c20e9d9b6f2f10`. Timebox: 40 minutes, starting approximately 20:00 UTC.

**Current conclusion:** administrative registration, actual submission and any operative individual deadline remain **unknown**. The 3 October answer “none” concerns the planning target, not proof of administrative status. Four currently linked MNTF forms were fetched and matched byte-for-byte to the repository PDFs. Official Mathematics regulations contradict the local “no defense talk” assumption, subject to confirming Jörn's applicable programme/regulations. University AI rules supply transparency obligations, but this review does not establish acceptance of the thesis's particular AI use or disclosure.

This is a review, not a finding of misconduct, lateness or non-submission. No repository edits, Git writes, institutional messages, submissions or publication were performed. All fixes below are proposals. “Solved” means a review question was resolved by evidence, not that repository content was edited; “withdrawn” means an unsupported inference was rejected.

## Findings

### F1 — Surviving · High — Entry points overstate current submission and registration status

**Exact locations:** `README.md:4`–`5`; `meta-topics/thesis-submission/anmeldung-bachelor-masterarbeit-mntf.md:104`–`106`. Contradicting authority: `docs/project-facts.md:34`–`50`. Relevant continuation limit: `meta-topics/project-management/handoff.md:339`–`343`.

The README asserts that the thesis has not been submitted for final grading. The registration conversion calls delivery of the approved note a pending action. The fact sheet expressly leaves both current statuses unknown and labels the registration note historical. Neither the historical deadline nor continued manuscript work establishes whether administrative submission occurred.

**Concrete proposed fix:** replace the README's submission assertion with “Current administrative submission status is unverified; see docs/project-facts.md.” Qualify the registration note as historical and replace “Pending action” with “Delivery was pending when recorded; present registration status is unverified.” Ask Jörn for current status and the latest official notice/receipt before assigning any hand-in task. Preserve the dated deadline as provenance without treating it as a live deadline.

### F2 — Surviving · High — The “no defense talk” assumption conflicts with located official Mathematics rules

**Exact location:** `meta-topics/thesis-submission/hinweis-bachelor-masterarbeit-mntf.md:85`–`86`.

The [current official Mathematics legal collection](https://www.uni-augsburg.de/de/services/rechtssammlung/mntf-master-mathematik/) links the [2015 consolidated Mathematics examination regulations, M-420-2-3-002](https://assets.uni-augsburg.de/media/filer_public/c7/ca/c7ca192b-ebc8-4200-9906-cfa736063088/m-420-2-3-002.pdf). Under §20(1), (5), printed p.20, the final assessment includes a colloquium, normally 2–6 weeks after thesis submission, comprising a 30–60 minute talk and 15–30 minutes of questions. §21(2)–(3), p.21, requires both components to pass; the colloquium is assessed pass/fail. Jörn's applicable regulations and whether this component has already occurred are unverified.

**Concrete proposed fix:** remove the unsupported default “no defense talk”; record “The currently listed Mathematics PO requires a colloquium; confirm applicable PO/cohort and any individual disposition with Jörn/Prüfungsamt.” After confirmation, record whether it is completed, outstanding, or inapplicable, with the notice/source date. Do not schedule a new colloquium from this finding alone.

### F3 — Surviving · High — AI disclosure exists, but compatibility with official declaration remains unresolved

**Exact locations:** `thesis/ai-use-disclosure.tex:6`–`23`, `thesis/ai-use-disclosure.tex:33`–`44`; `meta-topics/thesis-submission/erklaerung-abgabe-abschlussarbeit-mntf.md:38`–`83`; source-refresh obligation at `docs/project-facts.md:44`–`45`.

The disclosure describes substantial agent contributions and advisor approval. It also explicitly distinguishes agent checks from Jörn's completed personal verification. The currently linked [MNTF submission declaration](https://assets.uni-augsburg.de/media/filer_public/1f/5d/1f5d67ea-6121-488d-8f0e-a1bdcef414a8/1erklarung_abgabe_abschlussarbeit_mntf_mit_freiwilliger_cd-1.pdf), pp.1–2, requires independent work, declared aids, and compliance with the chair's scientific-writing guide; its explanation of permissible assistance includes software proofreading limited to spelling, punctuation and grammar. It contains no explicit generative-AI exception. That creates a clarification need, not proof that approved AI use is prohibited.

The [university AI policy E-048-1-000](https://assets.uni-augsburg.de/media/filer_public/7c/5e/7c5e26c5-231d-4199-a234-4bd5be18b8d3/e-048-1-000.pdf), effective 1 August 2025, §1.1 and §§5–6.2, applies to study/research use and calls for transparent documentation of relevant origin and use of AI content, good scientific practice, and careful validation of prompts/outputs before use or onward transmission. It offers logging, citations and footnotes as examples; it does not establish a universal requirement to publish every prompt. The [teaching portal](https://www.uni-augsburg.de/de/services/portal-lehre/ki-in-der-hochschullehre/) recommends communicating whether AI is permitted and how it must be documented. These public sources were accessible; no authenticated intranet access was needed for the policy PDF.

**Concrete proposed fix:** obtain the applicable chair guide and a written clarification from the responsible examiner/Prüfungsamt or examination committee reconciling the disclosed breadth of AI use with the exact declaration to be signed. Confirm the required location and granularity of disclosure and whether any separate signed statement is required. Then compare that requirement with the final candidate's disclosure and actual verification state. Preserve truthful provenance and review caveats; do not silently soften them, substitute another faculty's template, or treat the reflection chapter as formal approval. No institutional message has been sent.

### F4 — Surviving · Medium — Individual notice still needed for copies, medium, deadline and language consent

**Exact locations:** `meta-topics/thesis-submission/hinweis-bachelor-masterarbeit-mntf.md:23`–`28`, `:62`–`63`, `:83`–`84`; `meta-topics/thesis-submission/erklaerung-abgabe-abschlussarbeit-mntf.md:89`–`96`, `:113`–`119`; the English manuscript is selected at `thesis/main.tex:42` and `:57`–`81`.

The [current MNTF information sheet](https://assets.uni-augsburg.de/media/filer_public/f0/b9/f0b923c7-1698-4136-af89-4810469d26e8/hinweis_bachelor-_masterarbeit-_mntf.pdf), p.1, identifies the individual Bescheid as the source for topic, start and latest submission date, and the Bescheid/Merkblatt for copy count. The submission form still offers the specified printed copies or one fewer printed copy plus an additional PDF CD. This does not verify an email-only or USB substitution for Jörn. The located Mathematics PO §20(1) allows English with the examiners' consent; §§20(2)–(3) provide a six-month standard maximum and possible individual extensions/exclusions. None of this computes Jörn's current deadline or proves missing consent.

**Concrete proposed fix:** reconcile the latest Bescheid/Merkblatt and any later official modification into one dated case-specific checklist: applicable programme/PO, registration state, topic/title, operative deadline if any, copy/binding count, electronic medium/channel, required signed forms, English-language consent and colloquium disposition. Where a requirement differs from the public form, cite the specific official instruction. Keep unknown fields explicitly unknown; do not prepare a submission package as if hand-in were confirmed outstanding.

### F5 — Solved · Informational — Current linked form identity has been checked

**Exact locations:** `meta-topics/thesis-submission/README.md:8`–`24`; `docs/project-facts.md:44`–`45`.

The [official MNTF submission page](https://www.uni-augsburg.de/de/studium/organisation-beratung/pruefungen/infos-und-antrage/mathematisch-naturwissenschaftlich-technische-fakultat-pruefungen/) currently links the same four URLs recorded in the submission README. Direct HTTPS retrieval during this review found exact byte equality with each local PDF:

| Local PDF under `meta-topics/thesis-submission/` | Bytes | SHA-256 |
| --- | ---: | --- |
| `hinweis-bachelor-masterarbeit-mntf.pdf` | 98000 | `c2e1194c9e1802ff332419fc75029cba23b6f34c501750e28a1b96edb6eb00e9` |
| `anmeldung-bachelor-masterarbeit-mntf.pdf` | 189710 | `d5f0f0e13133f3b3d950aed7606e3c621e222777c367f57ad057543f57e97528` |
| `erklaerung-abgabe-abschlussarbeit-mntf.pdf` | 206028 | `63d9cd05642cea8b8e595b9c1ce688007e9dc8a31154fc9e0fd2603303f526cb` |
| `erklaerung-einsichtnahme-dritter.pdf` | 9086 | `07a47dc9e350240d4c745f72a02fbdc37ae806508234eec9d1f5b79799581780` |

**Concrete proposed fix/disposition:** in a later authorized documentation update, add the recheck date and hashes while preserving the original 24 April retrieval date. Keep current-form identity separate from individual applicability and administrative status. Do not replace unchanged PDFs or call all submission requirements resolved.

### F6 — Solved · Informational — Disclosure and research reflection are already separate artifacts

**Exact locations:** `docs/project-facts.md:406`–`419`; `thesis/main.tex:57`, `:81`; `thesis/ai-use-disclosure.tex:2`; `thesis/chapters/15-ai-reflection.tex:2`–`11`.

The selected manuscript includes an unnumbered disclosure and a numbered research-process reflection, matching the recorded project design. “Missing AI disclosure” is not supported. This source check does not certify the final PDF or official sufficiency.

**Concrete proposed fix/disposition:** preserve the two-artifact structure; reconcile official requirements through F3 rather than adding a duplicate generic AI chapter. Final length/detail remain Jörn's decision within applicable requirements.

### F7 — Withdrawn · Informational — Historical deadlines do not prove a missed or remaining hand-in

**Exact locations:** `docs/project-facts.md:24`–`42`; `meta-topics/project-management/handoff.md:339`–`343`.

Any conclusion that Jörn missed the August deadline, is currently late, or still must submit is withdrawn as unsupported. The recorded “none” planning answer likewise does not prove that registration was withdrawn or no institutional deadline exists.

**Concrete proposed fix/disposition:** use “unknown pending current confirmation” in planning; ask for current status and the latest official disposition. Do not carry the historical date into a new execution deadline.

## Bounded next inquiry — prepared, not sent

Ask Jörn: **Has the thesis been administratively registered and/or submitted, and what does the latest Prüfungsamt notice say about your programme/PO, current deadline (if any), and any completed colloquium? What written AI-use/disclosure instructions or chair guide apply?**

If details require official clarification, prepare one consolidated inquiry for Jörn to approve/send to the responsible examiner or MNTF Prüfungsamt covering F2–F4. This review authorizes neither external messaging nor hand-in. The report is complete as a bounded review; the underlying administrative facts remain open until answered.

## Scope and evidence limits

Inspected README/AGENTS guidance, the requested project facts and handoff, submission README and four Markdown conversions, the four form PDFs through text extraction and byte comparison, targeted coordination/backlog references, selected manuscript entry point and both AI artifacts. Followed the current public MNTF page, Mathematics legal collection, general examination regulations (§13(6), p.8), AI legal collection/policy and teaching portal. Searches were targeted, not an exhaustive repository/history audit. Public page retrieval establishes displayed guidance as of this check, not Jörn's private case record. No mail, Studis, signed forms, individual Bescheid, authenticated private correspondence, or final-PDF review was available/performed. No build or tests were needed for this source review. Repository coordination files were deliberately left unchanged under the read-only instruction.

[review: message_reviewer]
