# Evidence and checks for the DS passage trial

This proposed passage covers the fresh-selection, transfer, support-height,
ambient-rotation and adaptive-sampling material from sections 1.6–1.9 of
`ds-complete-draft/chapter.tex` at `43ad6a90`. It is not integrated into the
candidate thesis. No experiment or capacity producer was run for this edit.

All results below are numerical systolic ratios. Capacity evaluation alone
does not certify the floating-point volume normalization. The source audit
checked the retained experiment reports and relevant producer semantics;
the new prose received a separate bounded scientific review. These checks
establish support for the stated finite observations, not human acceptance
or submission readiness of the whole thesis.

The independent reviewer also checked the final four-row contrast table
and its collective inference after the opening was reorganized. No required
scientific correction remained. The rotation material was moved to the trial
appendix following Jörn's editorial direction, without changing its claims.

## Claim support

Artifact paths below are relative to
`experiments/sys-datascience/methods/`.

| Passage claim | Retained source and scope |
|---|---|
| Low-R mean 0.626 versus 0.316 | `extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selection-summary.tsv`, row `per_bucket_low_ridge_symp_area_sum_over_volume_sqrt_top_10`: 0.6261166821 versus 0.3155039434. Ten selected bodies and ten comparison bodies per side-count group. Controls use rule-specific hash ordering, not asserted random sampling. |
| Low-C increment 0.034 in the low-R tail; positive in eight groups | `extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/validation-verdict.json`. C measures the largest ridge share, not every notion of area evenness. |
| Covariance motivation and fresh comparison | `canonical-vertex-covariance/artifacts/current/report.json` supports the exploratory inverse association. `extreme-scalar-rejection-proposer/artifacts/covariance-rho-frozen-validation/evaluation/covariance-rho-validation-verdict.json` supports differences 0.3331231475 and 0.0144879956, with the displayed rounded intervals. Student-t intervals use twenty pool-by-group differences from two pools. |
| Generic mean-ridge selector and stronger filter | `generic-ridge-tail-stage1/artifacts/stage1/analysis.json`: means 0.6257067667 and 0.3385699646; extreme-tail contrast −0.0317607412. Its percentile interval resamples the realized 10/90 panels, without generating or selecting new pools. |
| Tangential-source transfer | `alternative-source-transfer/artifacts/transfer-v1/analysis.json`: equal-group covariance/control effect 0.2587281074 and ridge/control effect 0.2253551372. One generated pool, two side-count groups; this is a conditioned tangential source. |
| Paired support-height change | `paired-tangentialization/artifacts/analysis.json`: both-factor mean effect 0.0152684962; eleven increases and five decreases. Admission requires the original and all paired constructions to be valid. Separate factor normalization is a symmetry of the ratio. |
| Ambient rotation | `orientation-allocation/artifacts/summary.json` and `details.json`: fresh-sampling maxima exceed prescribed-rotation maxima in all four comparisons; three starts improve under rotation. The quadrilateral example is 0.3276937313 → 0.5292849220, versus a fresh maximum of 0.7735357995. Matrices lie in SO(4), with a symplectic control; this is not a test of every ambient rotation. |
| Adaptive sampling | `diagonal-cem-pilot/artifacts-supported-v2/analysis.json` supports the three maximum/top-eight pairs. Ordinary coordinates are log gap ratios and centered log dual-normal lengths; the relative angle uses a wrapped Gaussian and circular mean update. The differing representations make evaluator admission act differently on the two arms. |

## Mathematical additions

For a symplectic S, the centered vertex covariance transforms as
Γ(SK) = SΓ(K)Sᵀ. The identity JS = S⁻ᵀJ makes JΓ(SK) similar to JΓ(K),
so the Williamson eigenvalues are unchanged. Dilation scales both by the
same square factor. For a product, uniform counting measure on its vertices
is the product of uniform counting measures on the factor vertices. Hence
Γ(P×Q) = diag(A,B), and (JΓ)² = diag(−BA,−AB). The two squared Williamson
eigenvalues are therefore the eigenvalues of AB. The scientific reviewer
checked these additions; they give no capacity formula for general polytopes.

## Build and visual check

- `sh thesis/candidate/ds-result-trial/build.sh` produces the three-page
  `build/preview.pdf` using the existing TeX installation.
- The resolved build has no undefined references or citations and no
  overfull/underfull-box warnings.
- All three pages were rendered and inspected for equations, tables,
  line wrapping and page breaks. The preview context is not proposed as
  new thesis prose.

## Integration choices

Jörn judged the finite rotation-allocation comparison insufficiently
interesting for its own main subsection. It is retained in `appendix.tex`,
with one sentence and a pointer in the main passage. The remaining strands
remain available for judging their place in the thesis. Full integration must
preserve the existing detailed methods and provenance, reconnect the
bibliography and section references, and review the surrounding transition.
Neither the bounded scientific review nor the clean build supplies a whole-
thesis acceptance judgment.
