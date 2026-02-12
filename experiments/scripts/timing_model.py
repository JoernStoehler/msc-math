#!/usr/bin/env python3
"""
Fit timing model T(F) = a·b^F for EHZ capacity computation.

Reads timing data from experiments/profiling/timing_data.csv
Fits exponential model via log-linear regression
Outputs model parameters to experiments/profiling/timing_model.json
"""

import json
import numpy as np
import pandas as pd
from pathlib import Path
from scipy.optimize import curve_fit
from typing import Tuple

REPO_ROOT = Path(__file__).resolve().parent.parent.parent


def exponential_model(F: np.ndarray, a: float, b: float) -> np.ndarray:
    """T(F) = a·b^F"""
    return a * np.power(b, F)


def fit_timing_model(csv_path: Path) -> Tuple[float, float, dict]:
    """
    Fit exponential model to timing data.

    Returns:
        (a, b, metrics) where metrics contains R², RMSE, etc.
    """
    df = pd.read_csv(csv_path)

    # Extract facets and time per run (in seconds)
    F = df['facets'].values
    T_ms = df['ms_per_run'].values
    T_sec = T_ms / 1000.0

    # Fit via curve_fit (nonlinear least squares)
    # Initial guess: a=0.001, b=2 (doubling per facet)
    (a, b), _ = curve_fit(exponential_model, F, T_sec, p0=[0.001, 2.0])

    # Compute fit quality metrics
    T_pred = exponential_model(F, a, b)
    residuals = T_sec - T_pred
    ss_res = np.sum(residuals**2)
    ss_tot = np.sum((T_sec - np.mean(T_sec))**2)
    r_squared = 1 - (ss_res / ss_tot)
    rmse = np.sqrt(np.mean(residuals**2))

    metrics = {
        'r_squared': float(r_squared),
        'rmse_seconds': float(rmse),
        'data_points': int(len(F)),
        'facet_range': [int(F.min()), int(F.max())],
    }

    return a, b, metrics


def project_dataset_size(a: float, b: float, total_hours: float, cores: int,
                         facet_distribution: dict) -> dict:
    """Project how many polytopes can be generated in total_hours on cores CPUs.

    Args:
        a, b: Model parameters for T(F) = a·b^F (seconds)
        total_hours: Total compute time available
        cores: Number of CPU cores
        facet_distribution: {facets: count} for target dataset

    Returns:
        {
            'total_polytopes': int,
            'compute_hours_per_core': float,
            'breakdown': {facets: {'count': int, 'hours': float}}
        }
    """
    total_seconds = total_hours * 3600
    effective_seconds = total_seconds * cores  # Parallelizable work

    breakdown = {}
    total_polytopes = 0
    total_compute_seconds = 0.0

    for facets, count in sorted(facet_distribution.items()):
        time_per_polytope = exponential_model(np.array([facets]), a, b)[0]
        total_time = time_per_polytope * count
        total_compute_seconds += total_time
        total_polytopes += count

        breakdown[str(facets)] = {
            'count': count,
            'time_per_polytope_sec': float(time_per_polytope),
            'total_hours': float(total_time / 3600),
        }

    return {
        'total_polytopes': total_polytopes,
        'total_compute_hours': float(total_compute_seconds / 3600),
        'wallclock_hours': float(total_compute_seconds / (cores * 3600)),
        'breakdown': breakdown,
        'feasible': bool(total_compute_seconds <= effective_seconds),
    }


def main():
    """Fit timing model and project dataset sizes."""
    # Paths
    csv_path = REPO_ROOT / 'experiments' / 'profiling' / 'timing_data.csv'
    output_path = REPO_ROOT / 'experiments' / 'profiling' / 'timing_model.json'
    # Model lives with raw timing data, not in experiments/data/ (which is for datasets)

    print(f"Reading timing data from {csv_path}")
    a, b, metrics = fit_timing_model(csv_path)

    print(f"\nFitted model: T(F) = {a:.6f} · {b:.4f}^F")
    print(f"R² = {metrics['r_squared']:.4f}")
    print(f"RMSE = {metrics['rmse_seconds']:.4f} seconds")

    # Example dataset: 1000 polytopes with facet distribution
    # Realistic mix: mostly small polytopes, few large ones
    facet_distribution = {
        5: 200,
        6: 200,
        7: 200,
        8: 200,
        9: 100,
        10: 50,
        12: 30,
        14: 15,
        16: 5,
    }

    print("\n=== Dataset Size Projection (24h on 8 cores) ===")
    projection_24h = project_dataset_size(a, b, 24, 8, facet_distribution)

    print(f"Total polytopes: {projection_24h['total_polytopes']}")
    print(f"Total compute hours: {projection_24h['total_compute_hours']:.2f}")
    print(f"Wallclock hours: {projection_24h['wallclock_hours']:.2f}")
    print(f"Feasible in 24h on 8 cores: {projection_24h['feasible']}")

    print("\nBreakdown by facet count:")
    for facets, info in projection_24h['breakdown'].items():
        print(f"  F={facets}: {info['count']} polytopes, "
              f"{info['time_per_polytope_sec']:.3f}s each, "
              f"total {info['total_hours']:.2f}h")

    # Save model
    output = {
        'model': {
            'type': 'exponential',
            'formula': 'T(F) = a * b^F',
            'parameters': {'a': a, 'b': b},
            'units': 'seconds',
        },
        'fit_quality': metrics,
        'projections': {
            '24h_8cores': projection_24h,
        },
    }

    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w') as f:
        json.dump(output, f, indent=2)

    print(f"\nModel saved to {output_path}")


if __name__ == '__main__':
    main()
