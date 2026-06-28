# Error Model Smoke Fixture

This directory is for fast analyzer development checks, not evidence. The
fixture is a three-row slice from the schema-upgraded high-degeneracy smoke:

- one smooth fixed-window row;
- one sigma-window-dominated row;
- one target-polytope construction failure.

Run:

```bash
python3 experiments/dev-sys-prediction/analyze_prediction_error_model.py \
  --prediction-cloud experiments/dev-sys-prediction/error-model-smoke/prediction-cloud-smoke.jsonl \
  --out-dir /tmp/dev-sys-prediction-error-model-smoke
```

