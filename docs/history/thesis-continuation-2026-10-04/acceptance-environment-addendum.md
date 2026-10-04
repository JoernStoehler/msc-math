# Independent acceptance addendum: repaired second TeX environment

Owner: `/root/acceptance`. Observed 4 October 2026, 01:48–01:49 UTC.
This addendum concerns only the final dependency-repair experiment. It does
not reopen the thesis claims or replace the accepted PDF.

The original [failed build](cross-environment-build.json) remains retained:
the existing image lacked `lmodern.sty`, a dependency already named in
`INSTALL.md`. The [separate retry](cross-environment-repaired.json) used the
same image digest and a new disposable container. The package log reports
exactly four newly installed packages, no upgrades and no removals:
`lmodern` 2.005-1build1, `libfontenc1` 1:1.1.8-1build2,
`xfonts-encodings` 1:1.0.5-0ubuntu3 and `xfonts-utils` 1:7.7+7build1.
Networking was disconnected before the non-root thesis build. The recorded
limits were two CPUs, 2 GiB and 180 seconds for the complete operation.

The setup and build completed in 22.64 seconds; the build itself took 10.84
seconds. It produced 101 pages, with no recorded overfull boxes or undefined
references/citations. The log's intermediate 98-page first pass is followed
by converged 101-page passes and a successful `latexmk` completion. The
container was removed, and the frozen 61-source identity and canonical
candidate were preserved. This establishes a successful build in a second
**existing** TeX environment after one declared dependency repair; it does
not establish a complete fresh-machine installation or repeat any empirical
producer.

The reproduction PDF is [cross-environment-repaired.pdf](cross-environment-repaired.pdf),
SHA-256 `f09acee885c27f07375ff7d11ee0acc3241aa570c90e940e303d1120446fcfff`.
It differs from the accepted candidate, whose hash remains
`d27e2f5330c61b25df23939a3da8118da98dd95aa6f336ac9d09d52302724a81`.
The [integration comparison](cross-environment-comparison.json) reports only
10 of 101 page rasters identical at 96 dpi, so no pixel-equivalence claim is
made. The selected reader-facing artifact remains the independently reviewed
candidate, not this reproduction PDF.

Acceptance independently extracted layout text from both PDFs. Only pages
9, 10, 14, 27, 31, 43, 48 and 51 differ. A whitespace-token comparison has
six changed spans, all involving mathematical glyph ordering or spacing:
the numerator/denominator extraction order for `1/4` and `1/2`, placement of
the slash in a not-equal sign, and a matrix-bracket space. All eight pages
were independently rendered at 1600 pixels and viewed individually. The
fractions, inequalities and matrix render correctly; no new clipping or
changed mathematical content was observed. This is targeted visual review
of the text differences, not a new all-page acceptance of the alternate
PDF. The other raster differences are not asserted to be identical or
exhaustively classified.

The maintained reproduction documentation accurately states the initial
failure, precise repair, successful second-environment build and comparison
limits. No additional source fix is requested. The new receipt, logs, PDF,
comparison and this addendum are bound in the final acceptance manifest.
Full installation portability, cross-version byte identity, raw-data
reproduction and human PASS remain outside this check.
