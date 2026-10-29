# Research report delivery — 29 October 2026

Final report revision: **2026-10-29-r6**, 56 pages. The PDF, LaTeX sources,
finite certificates, figures and scoped reviews are retained in
[the paper packet](convex-systolic-optimization/README.md). The final source
archive is [source-and-evidence.zip](convex-systolic-optimization/source-and-evidence.zip).
The archive's internal `MANIFEST.sha256` covers the same-revision snapshot;
this post-export delivery receipt is deliberately outside that snapshot.

The exact PDF has assembled mathematical and reader PASS reports and root's
inspection of all 56 final pages. The separate extracted-copy verification
below was performed by the native `affine_hko_exact_replay` agent; its report
is retained verbatim. These are agent checks, not external refereeing, formal
verification or human thesis acceptance. The selected thesis was not changed.

Root also checked both actual HTTP responses through the private temporary
[review index](http://100.70.188.20:35027) at about 01:17 UTC:

| Artifact | Response | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| [PDF](http://100.70.188.20:35027/files/19191ee79a11/paper.pdf) | 200, `application/pdf` | 809440 | `a87d422c1ee43ca10454b5c7fa25eeb3f35f1b9ea424734aa501fe6c7e13f0b3` |
| [Source ZIP](http://100.70.188.20:35027/files/aa55115e7b5a/source-and-evidence.zip) | 200, `application/zip` | 5008918 | `09c3b2c255c22510847b0fa1d69ea255892932d6da2c09d31628e5c0651d0050` |

Both hashes match the retained files. The review index expires about
08:43 UTC on 29 October; retained checkout/Git files do not depend on it.
No remote publication or university submission was performed.

---

# Independent final archive replay

29 October 2026 UTC. **PASS** for delivery integrity, portability of the selected replays, and extracted source build. The archive was read from the canonical packet and all writes stayed in `/tmp/msc-math/restart/final-archive-replay/`. No canonical packet file was changed.

## Archive and hashes

- Source archive: `/home/joern/.codex/worktrees/research-restart-20261028/msc-math/topics/capacity-and-qp/convex-systolic-optimization/source-and-evidence.zip`.
- Exact archive SHA-256: `09c3b2c255c22510847b0fa1d69ea255892932d6da2c09d31628e5c0651d0050` (5008918 bytes).
- Safe extraction checked paths, duplicate entries, symlinks, ZIP CRC and the single root directory. There are 166 archive members. The internal `MANIFEST.sha256` covers all other 165 files, with no missing or extra files; every hash passes. `BUNDLE.json` reports revision `2026-10-29-r6` and matches the extracted `main.tex` and `paper.pdf` hashes.
- Archived PDF SHA-256: `a87d422c1ee43ca10454b5c7fa25eeb3f35f1b9ea424734aa501fe6c7e13f0b3`, matching the required final PDF. It has 56 A4 pages.
- Nested certificate manifests pass: `hko-t03/MANIFEST.sha256` (11 entries), `hko-two-hinge/MANIFEST.sha256` (5), and `hko-ascent/MANIFEST.json` (30). The retained historical F11 proof ZIP passes CRC. The archive contains no `build`, `render`, `.git`, `__pycache__`, `.pyc`, `.o`, or `.so` artifacts. Its only executable member is `build.sh`. The selected replay/build entrypoints do not require an absolute workstation executable path.
- The ten moderate-HKO proof/code/input files in the archive, including `COVERAGE-PROOF.md`, are byte-identical to the packet independently replayed through all 77,927 curvature exclusions and 2,326 leaf bounds earlier. The six-minute moderate scan was therefore not repeated.

## Fresh runs from extraction

- `certificates/hko-ascent/replay.py` passed in a fresh output directory in 12.98 s. All five detailed expected JSON outputs matched, with 190 complete charts and response maximum `1/125`.
- `certificates/hko-two-hinge/certify_amplification.py` produced a byte-identical match to `expected/amplification-PASS-portable.json`.
- `reviews/independent-two-hinge/check-two-hinge-literals.py` produced a byte-identical match to `two-hinge-literals.out`.
- `sh build.sh` from the extracted packet succeeded and produced a 56-page A4 PDF. The final `build/main.log` has no unresolved warning or error (the combined latexmk log contains expected first-pass reference/citation warnings that resolved on rerun). The rebuilt PDF binary SHA-256 differs from the archived PDF, but `pdftotext -layout` output matches byte for byte, SHA-256 `e45b131fc61ae1c111166ed6f16270da5382a8dc23721440817047a9eb89a58d`. The archived PDF was restored inside the extraction after that comparison, and all 165 internal manifest entries passed again.

The run receipt is `archive-integrity.json`; logs and generated outputs are beside this report. These checks establish that the delivered bytes contain the reviewed PDF and reproducible companion inputs. Mathematical correctness, reader judgment, and root's all-page visual inspection are separate evidence.
