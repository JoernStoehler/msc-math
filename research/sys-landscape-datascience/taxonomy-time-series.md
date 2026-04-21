# Taxonomy: Time Series And Sequential Analysis

Source intent:

- Frozen taxonomy snapshot for time-series and sequential-analysis methods adapted from Hyndman--Athanasopoulos, *Forecasting: Principles and Practice*, and standard state-space / sequence-analysis chapter organization.
- This file is the external home for trajectory-log summaries, ordered step-event analysis, and sequential regime-change methods.

## Classical Time-Series Models

- `TS-ARIMA-AR` Autoregressive models
- `TS-ARIMA-MA` Moving-average models
- `TS-ARIMA-ARIMA` ARIMA-family models
- `TS-ETS` Exponential smoothing / ETS models

## State-Space And Filtering

- `TS-SSM-LINEAR` Linear state-space models
- `TS-SSM-KALMAN` Kalman filtering / smoothing
- `TS-SSM-SWITCH` Switching or regime-change state-space models

## Change-Point And Regime Detection

- `TS-CHANGEPOINT` Change-point detection
- `TS-REGIME-SWITCH` Regime-switch models

## Sequence Summaries And Event Logs

- `TS-SEQ-AGG` Ordered-sequence summary statistics
- `TS-SEQ-MARKOV` Markov-chain style transition models
- `TS-SEQ-HMM` Hidden Markov models

## Similarity And Shape-Based Sequence Methods

- `TS-SHAPE-DTW` Dynamic time warping and shape-based sequence comparison
- `TS-SHAPE-FEAT` Feature-based sequence comparison

## Likely Use In This Project

- Strongest overlap: step-event summaries, regime changes in ascent traces, transition analysis, and light sequence models on search logs.
- Weak overlap: long-horizon forecasting or full classical time-series modeling unless the trajectory packets become much richer.
