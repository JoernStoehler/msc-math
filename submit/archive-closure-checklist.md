# Archive Closure Checklist

Use this at final repository/Zenodo closure. It records accepted direction and
remaining work; it does not authorize publication or expensive recomputation.

## Final repository state

- [ ] Preserve the useful closure-time repository state as the continuation
      surface, including agent infrastructure and ordinary project context.
- [ ] Remove raw session logs, credentials/authentication state, and genuinely
      private correspondence.
- [ ] Remove downloaded papers, publisher figures, university forms after they
      are no longer needed for submission, copied vendor text, and other
      third-party material without clear redistribution permission. Retain or
      relocate project-authored notes, bibliography metadata, source maps, and
      original assets from mixed directories.
- [ ] Retain applicable third-party license notices.
- [ ] Do not rewrite Git history merely for the Zenodo snapshot. Do not submit
      inherited history to Software Heritage while unclear third-party material
      remains reachable; omit Software Heritage unless a clean origin has a
      concrete preservation benefit.

## Final data state

- [ ] Beside each experiment producer, classify current artifacts as
      final/useful or disposable. Preserve data that lets a future agent inspect,
      interpret, validate, or continue a result without first repeating
      expensive computation.
- [ ] Delete or ignore smoke outputs, superseded intermediates, duplicate
      encodings without a consumer, disposable caches, and cheaply regenerated
      data with no immediate interpretive value.
- [ ] Commit every final/useful artifact. Use ordinary Git for small,
      inspectable, diff-friendly data and Git LFS for large or poorly diffing
      data. Size determines storage mechanism; regeneration cost and downstream
      value determine retention.
- [ ] Hydrate every LFS object in the final tracked tree and verify that no LFS
      pointer enters the Zenodo ZIP. Historical LFS objects absent from the final
      tree need not be copied into Zenodo.
- [ ] Recheck the final thesis claims against retained artifacts after the
      data-science and first-order wording closes. Smoke success is plumbing,
      not thesis evidence.

## Final release

- [ ] Run the clean documented environment and record material version drift.
- [ ] Build and check `thesis/build/main.pdf` from the exact release commit. The
      PDF remains generated/Git-ignored and is added to the release ZIP.
- [ ] Independently review and externally record the literal full release commit
      SHA. The packaging command must compare this value with `HEAD`.
- [ ] Build the ZIP, reopen it, extract it without `.git`, and verify its exact
      file inventory and hashes.
- [ ] Complete the final figure/font/third-party-rights review.
- [ ] Prepare `CITATION.cff` and Zenodo metadata when the final title, authorship,
      identifiers, commit, and payload are stable.
- [ ] Jörn reviews the prepared Zenodo draft and performs the exterior publish
      action. Record the DOI and final URLs afterward.
