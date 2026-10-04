# Evidence and reproduction documentation integration

Checked 2026-10-04, completed by 00:39 UTC. Receiving operational owner:
`/root/integration`; independent acceptance owner: `/root/acceptance`.
This is a bounded packet report, not whole-thesis acceptance.

## Changes

Preserved the checkpoint's shortened `INSTALL.md` after comparing its entire
diff with base `52b1526c8ff5613a400f2f4028b4290f35841b21`. Most removed material
was retired sandbox shell/editor provisioning, historical observations or
sandbox-only Sage installation. Current language-tool separation, Cargo PATH,
LaTeX packages, certificate entry points, R2 access and historical recovery route
remain. Restored the useful unpinned-Conda caveat and private rclone-file mode;
clarified that the remaining sandbox text is historical.

Updated `docs/reproducibility.md` to match the current mathematical and empirical
inventory: HKO, non-HKO, analytic pentagon routes versus historical enumeration,
the generated-candidate experiment, current DS interval coverage and numerical
report retention limits. Explained producer write effects and the distinction
between building retained figures and regenerating their inputs. No release,
fresh-environment setup or R2 download was attempted or claimed.

Resolved the dashboard serving contract by moving `/INSTALL.md` to the HTTP
test's allowed paths; `/README.md` remains a deliberately unserved ordinary
root file. The checkpoint server already allows INSTALL and the dashboard
links it. No server edit was needed.

Repaired non-HKO historical evidence routing in `README.md`, `PROOF.md`,
`RESULT.md`, and existing `PROVENANCE.json`. Original JSON provenance values
remain, with explicit nested-archive and current-verification locations added.
No redundant provenance Markdown file was created. The README now uses a
fresh temporary output directory and disables Python optimization for the
independent audit's assertions. Its `-13/432` quantity is correctly called the
second derivative, not the quadratic coefficient.

Per the integration owner's follow-up, reconciled the original DS full-run
README with its separately retained repair (90 of 91 refusals repaired;
14,335 intervals, 14,289 accepted scalars, 46 wider intervals, one unresolved
body). Preserved the original run's results. Replaced visualization's absent
manuscript/legacy paths with current Chapter 12 and an exact Git-history route.

## Checks and evidence

- `python3 -m unittest discover -s scripts -p test_project_dashboard.py`:
  all 8 HTTP tests passed, including bounded serving and INSTALL access.
- All 34 relative Markdown links in the seven edited Markdown documents
  resolve in the checkout. The visualization provenance revision/path is
  readable with `git show`.
- Both consolidation archive hashes match their recorded identities:
  outer SHA-256 `82c40c7df813fdc0fa2c1b8e56cd4edbb5fd599108b87f37250f7c0ae91dae8b`;
  member `incoming/non_hko_f10_certified_result.zip` SHA-256
  `adff85a5f18c83ab8ed97c55a48cf91382d206626461519550902706f61c0bd0`.
  All 70 entries of its `MANIFEST.sha256` verified. All six current exact core
  source/witness hashes match the archived `evidence/exact/RUN.json`.
  Both archived exact/reproduction logs contain `ALL EXACT CHECKS PASS`.
- Ran the README's complete non-HKO verifier and separate independent audit
  with `PYTHONOPTIMIZE=0`, `PYTHONDONTWRITEBYTECODE=1`, and
  `uv run --with sympy==1.14.0 python`, with 60-second subprocess limits.
  Both passed (approximately 12.69 and 3.50 seconds internal time;
  Python 3.13.3, SymPy 1.14.0).
  The new exact summary equals `verification/20260924-original-summary.json`
  except elapsed time. The new independent result equals the retained result
  except environment/runtime fields. The checks recompute all 624 product
  cases, all 960 independent gradient entries, ranks and W-line identities.
- Fresh outputs are local scratch at
  `/tmp/non-hko-restart-20261004-o2shwno5/`: `exact/verify.log`,
  `exact/RUN.json`, `exact/SUMMARY.json`, `independent.json`, and
  `command-0.log`/`command-1.log`. These paths and results were sent directly
  to independent acceptance. They are not a new durable evidence dependency;
  maintained source and retained evidence suffice to reproduce them.
- DS status counts checked against `target-full/summary.json` and
  `repair/comparison.json`; no evaluator or dataset producer was rerun.
- `git diff --check` passed for all nine edited source/documentation files.

## Scope and remaining limitations

No retained mathematical data, producer source, archive bytes or verification
outputs were replaced. No commit was made. Other sessions' working-tree edits
were preserved. No remaining broken route or setup-semantic defect was found
within this bounded assignment.

This packet does not establish the analytic neighborhood implication, entire
mathematical correctness, PDF layout, clean-environment reproduction, public
availability of local additions, or release readiness. The HKO Sage and full
pentagon verifiers were not rerun here. Current DS coverage still has the one
documented unresolved body and lacks certified volumes for the ratios.
Integration owns scientific-dashboard reconciliation and final assembly;
acceptance owns independent scientific/PDF assessment.

The operational owner resolved the serving choice and provenance-file scope
without returning routine decisions to the user. A potential duplicate exact
rerun was identified at the integration/acceptance interface and the existing
results were handed directly to acceptance. This observation does not establish
sustained orchestration effectiveness or quantify saved effort.
