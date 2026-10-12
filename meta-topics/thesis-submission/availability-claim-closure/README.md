# Availability, reflection provenance and HKO listing closure

Bounded review on 18 September 2026, against the candidate-assembly-integration
worktree. Exact input hashes are in `input-hashes.json`. Proposed edits are in
`availability.patch`; this packet does not edit the manuscript or publish anything.

## Findings and proposed disposition

1. **Public repository exists, but is older than the candidate.** Unauthenticated
   GitHub API inspection gives public `main` commit
   `1bec30637070effe7ab8b0a022ae62e83f6bbcab` (repository push time 31 August).
   Its complete public tree does not include `thesis/candidate/main.tex`.
   Both named theorem verifiers, HKO witness/summary and full pentagon stdout are
   publicly retrievable and byte-identical to the assembly's files; see
   `public-check.json`. Replace “tracked at submission time” with the checked
   public commit and explicitly distinguish local candidate additions. This
   prevents local Git tracking from being mistaken for public availability.
   Do not publish to repair the mismatch; the current task is PDF review only.

2. **The frozen registry counts are stale.** Public and local registries both
   contain 13 snapshots and 39 consumer links, not 12 and 21. These counts carry
   little explanatory value; the patch replaces them with the materialization
   distinction. The ongoing DS restoration can recover inputs into a local
   cache without changing what a plain clone contains or proving that all
   public data downloads work. No blanket claim of remote artifact availability
   was verified here. Do not add one.

3. **HKO displayed source matches the recorded executed core.** Current SHA-256
   `475196efdf33d1f281a724b90069ec08a2335552ddc4362332e2b975866f10f5`
   matches `thesis/candidate/support/hko-cas-appendix/README.md`, which records
   the successful rerun after explicit guards were added. The original hidden
   README at `/workspaces/msc-math/.git/codex/hko-cas-appendix/README.md` agrees.
   The public original verifier, witness and verification summary also match
   the three hashes recorded there. The four TeX ranges 1–11, 12–39, 40–68 and
   69–EOF partition all 93 core lines without omissions or overlap.
   This closes source identity to an existing execution record; it is not a
   new execution or independent mathematical audit. The core uses Python
   `assert`, so the proposed patch names its file and requires optimization
   disabled. A plain PDF build does not run these assertions.

4. **No new first-person assertions are justified by this review.** The selected
   reflection's HKO attribution matches Jörn's direct account as recorded in
   `docs/review-evidence/calibration/whole-thesis-reading-1700.md` under the
   section 13.1 correction: broad strategy from Jörn, essentially complete
   argument and Sage work from an uncertain GPT-5-series version. Do not
   strengthen the model identification. The sign-defect anecdote is supported
   by `experiments/ai-use/sign-replay/README.md` and
   `experiments/ai-use/reports/ai-research-workflow-case-matrix.md`: the minimal
   Sol test passed after restoring the known defect. The reflection does not
   infer a model ranking or causal productivity gain from that observation.
   Subjective experience (“increasingly useful”) remains Jörn's reported
   experience; source-code inspection cannot establish it. No replacement
   autobiography is proposed. The existing disclosure preserves the dated
   July review and explicitly distinguishes agent checking of the new affine
   proof from personal checking. This review does not establish a completed
   personal final review or institutional acceptance.

## Remaining assembly actions

- Apply or adapt the two-file patch; `git apply --check` passed against the
  inspected assembly worktree. This is a factual repair, not prose acceptance.
- On final selection, ensure any new empirical/formal packet described as
  available has an actual delivery path; local commit alone is insufficient.
- Rebuild/layout-check the small availability and appendix changes together
  with the rest of the assembly. No expensive certificate was rerun.

## Read-only public evidence

The API and raw-file checks were unauthenticated, with no credential access.
The public page also identifies the repository as public:
<https://github.com/JoernStoehler/msc-math>.
Pinned source scope:
<https://github.com/JoernStoehler/msc-math/tree/1bec30637070effe7ab8b0a022ae62e83f6bbcab>.
`public-check.json` records the exact publicly fetched file hashes.
