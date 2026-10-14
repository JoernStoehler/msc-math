# /// script
# dependencies = ["numpy"]
# ///
"""Fit one conversation's metadata-only request-cycle timing export."""
import argparse
import json
import numpy as np

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("events", help="single-conversation OpenObserve JSON export")
args = parser.parse_args()
events = sorted(json.load(open(args.events))["hits"], key=lambda x: x["_timestamp"])
pending = None
reconnects = 0
rows = []
times = []
for event in events:
    name = event["event_name"]
    if name == "codex.websocket_request" and pending is None:
        pending = event["_timestamp"]
        reconnects = 0
    elif name == "codex.websocket_connect" and pending is not None:
        reconnects += 1
    elif name == "codex.sse_event" and event.get("event_kind") == "response.completed":
        if pending is None:
            raise ValueError("Completion without request: export interval is incomplete")
        reason = int(event["reasoning_token_count"])
        other = int(event["output_token_count"]) - reason
        if other < 0:
            raise ValueError("Invalid token counters")
        rows.append([1, reason, other, reconnects])
        times.append((event["_timestamp"] - pending) / 1e6)
        pending = None
if pending is not None:
    raise ValueError("Uncompleted request: export interval is incomplete")
X = np.array(rows, dtype=float)
y = np.array(times)
if len(y) <= 4 or np.linalg.matrix_rank(X) < 4:
    raise ValueError("Insufficient independent observations for this four-parameter fit")
coeff = np.linalg.lstsq(X, y, rcond=None)[0]
loo_errors = []
loo_reasoning = []
for i in range(len(y)):
    mask = np.arange(len(y)) != i
    if np.linalg.matrix_rank(X[mask]) < 4:
        continue
    fit = np.linalg.lstsq(X[mask], y[mask], rcond=None)[0]
    loo_errors.append(float(X[i] @ fit - y[i]))
    loo_reasoning.append(float(fit[1] * X[:, 1].sum()))
print(json.dumps({
    "n": len(y),
    "formula": "seconds = base + reasoning_seconds_per_token * R + other_seconds_per_token * N + reconnect_seconds * reconnects",
    "coefficients": dict(zip(["base_seconds", "reasoning_seconds_per_token", "other_seconds_per_token", "reconnect_seconds"], coeff.tolist())),
    "contributions_seconds": dict(zip(["base", "reasoning", "other_output", "reconnect"], (X.sum(0) * coeff).tolist())),
    "observed_cycle_seconds": float(y.sum()),
    "fit_rmse_seconds": float(np.sqrt(np.mean((X @ coeff - y) ** 2))),
    "leave_one_out_rmse_seconds": float(np.sqrt(np.mean(np.array(loo_errors) ** 2))) if loo_errors else None,
    "reasoning_leave_one_out_range_seconds": [min(loo_reasoning), max(loo_reasoning)] if loo_reasoning else None,
    "negative_coefficients": bool(np.any(coeff < 0)),
    "limitations": "Model-based allocation assumes approximately constant class-specific generation rates and additive overhead. Sensitivity omits systematic model error. Not direct hidden-reasoning or backend-inference measurement. Single conversation only; retry attempts grouped into logical cycles.",
}, indent=2))
