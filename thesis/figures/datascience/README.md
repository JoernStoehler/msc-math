# Data-science appendix figures

These PDFs are deliberate thesis-local copies of producer-owned outputs.
Regenerate and copy them rather than editing them directly.

| thesis copy | authoritative output | producer command |
| --- | --- | --- |
| `hko-recovery-by-source-distance.pdf` | `experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/analysis/recovery-by-source-distance.pdf` | `uv run --script experiments/dev-gradient-ascent/ascent-continuation/analyze_hko_calibration.py experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/raw experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/analysis` |
| `derivative-and-kkt-scale.pdf` | `experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/analysis/derivative-and-kkt-scale.pdf` | `uv run --script experiments/dev-gradient-ascent/endpoint-model-audit/analyze.py experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/raw/audit.json experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/analysis` |

After regeneration, verify each authoritative PDF and thesis copy are
byte-identical.
