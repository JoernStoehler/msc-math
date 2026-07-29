# Predictor replay analysis

The direct target reevaluation control reproduced `sys` on all 420 usable targets (maximum absolute discrepancy 0).

The table reports pooled development data. Rows remain paired by saved state in `atoms.jsonl`; this summary is not a complete-optimizer comparison.

| distance scale | candidate set | branch values | median candidates | usable | winner coverage | median error | 90% error | within 0.01 | sign accuracy | median ms |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 0.5 | action window 0.010000 | affine anchor model | 207 | 1 | 0.7465 | 7.689e-05 | 0.07561 | 0.7254 | 0.831 | 2.144 |
| 0.5 | action window 0.010000 | branches reevaluated at target | 207 | 0.9859 | 0.7465 | 0 | 0.003048 | 0.9155 | 0.8786 | 2.223 |
| 0.5 | action window 0.100000 | affine anchor model | 303 | 1 | 0.7887 | 7.689e-05 | 0.04835 | 0.7254 | 0.838 | 3.059 |
| 0.5 | action window 0.100000 | branches reevaluated at target | 303 | 0.993 | 0.7887 | 0 | 0.0006822 | 0.9507 | 0.8794 | 3.068 |
| 0.5 | action window 0.300000 | affine anchor model | 429 | 1 | 0.8169 | 7.689e-05 | 0.03393 | 0.7394 | 0.8451 | 4.258 |
| 0.5 | action window 0.300000 | branches reevaluated at target | 429 | 1 | 0.8169 | 0 | 0.0001611 | 0.9789 | 0.8803 | 4.135 |
| 0.5 | action window 1.000000 | affine anchor model | 607.5 | 1 | 0.8169 | 7.689e-05 | 0.04476 | 0.7394 | 0.8451 | 6.01 |
| 0.5 | action window 1.000000 | branches reevaluated at target | 607.5 | 1 | 0.8169 | 0 | 0.0001611 | 0.9789 | 0.8803 | 5.705 |
| 0.5 | all anchor-feasible | affine anchor model | 807.5 | 1 | 0.8169 | 7.689e-05 | 0.1104 | 0.7254 | 0.7958 | 8.008 |
| 0.5 | all anchor-feasible | branches reevaluated at target | 807.5 | 1 | 0.8169 | 0 | 0.0001611 | 0.9789 | 0.8803 | 7.387 |
| 0.5 | anchor winner | affine anchor model | 1 | 1 | 0.4155 | 0.0004331 | 0.0875 | 0.7183 | 0.7606 | 0.09373 |
| 0.5 | anchor winner | branches reevaluated at target | 1 | 0.7817 | 0.4155 | 0 | 0.006802 | 0.7183 | 0.8468 | 0.2796 |
| 0.5 | target winner control | affine anchor model | 1 | 0.669 | 1 | 9.697e-06 | 0.028 | 0.5282 | 0.9789 | 0.07759 |
| 0.5 | target winner control | branches reevaluated at target | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 0.2815 |
| 1 | action window 0.010000 | affine anchor model | 207 | 1 | 0.5248 | 0.0004689 | 0.3082 | 0.617 | 0.7589 | 2.15 |
| 1 | action window 0.010000 | branches reevaluated at target | 207 | 0.9716 | 0.5248 | 0 | 0.1796 | 0.773 | 0.8175 | 2.082 |
| 1 | action window 0.100000 | affine anchor model | 302 | 1 | 0.6028 | 0.0004689 | 0.2032 | 0.6312 | 0.766 | 3.046 |
| 1 | action window 0.100000 | branches reevaluated at target | 302 | 0.9858 | 0.6028 | 0 | 0.04621 | 0.844 | 0.8201 | 2.891 |
| 1 | action window 0.300000 | affine anchor model | 429 | 1 | 0.6738 | 0.0004689 | 0.1812 | 0.6454 | 0.7872 | 4.255 |
| 1 | action window 0.300000 | branches reevaluated at target | 429 | 1 | 0.6738 | 0 | 0.009558 | 0.9007 | 0.844 | 4.042 |
| 1 | action window 1.000000 | affine anchor model | 607 | 1 | 0.6879 | 0.0004395 | 0.2783 | 0.6667 | 0.7305 | 5.988 |
| 1 | action window 1.000000 | branches reevaluated at target | 607 | 1 | 0.6879 | 0 | 0.007941 | 0.9149 | 0.844 | 5.345 |
| 1 | all anchor-feasible | affine anchor model | 808 | 1 | 0.7021 | 0.0004395 | 3.791 | 0.6454 | 0.6454 | 8.143 |
| 1 | all anchor-feasible | branches reevaluated at target | 808 | 1 | 0.7021 | 0 | 0.005337 | 0.9291 | 0.844 | 6.579 |
| 1 | anchor winner | affine anchor model | 1 | 1 | 0.1206 | 0.003149 | 0.3082 | 0.5887 | 0.617 | 0.09388 |
| 1 | anchor winner | branches reevaluated at target | 1 | 0.5816 | 0.1206 | 0.0004149 | 0.05386 | 0.4397 | 0.6463 | 0.2757 |
| 1 | target winner control | affine anchor model | 1 | 0.5177 | 1 | 3.748e-05 | 0.1483 | 0.3972 | 0.9041 | 0.07066 |
| 1 | target winner control | branches reevaluated at target | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 0.2814 |
| 2 | action window 0.010000 | affine anchor model | 204 | 1 | 0.4234 | 0.001644 | 0.8475 | 0.5693 | 0.6934 | 2.146 |
| 2 | action window 0.010000 | branches reevaluated at target | 204 | 0.9489 | 0.4234 | 1.694e-06 | 0.2886 | 0.6569 | 0.8231 | 1.996 |
| 2 | action window 0.100000 | affine anchor model | 302 | 1 | 0.4599 | 0.001395 | 0.7177 | 0.5912 | 0.7226 | 3.05 |
| 2 | action window 0.100000 | branches reevaluated at target | 302 | 0.9708 | 0.4599 | 4.962e-07 | 0.2296 | 0.708 | 0.8421 | 2.525 |
| 2 | action window 0.300000 | affine anchor model | 429 | 1 | 0.4964 | 0.001395 | 0.6837 | 0.5839 | 0.7591 | 4.25 |
| 2 | action window 0.300000 | branches reevaluated at target | 429 | 0.9854 | 0.4964 | 0 | 0.07491 | 0.7372 | 0.8519 | 3.609 |
| 2 | action window 1.000000 | affine anchor model | 607 | 1 | 0.562 | 0.001644 | 1.071 | 0.5766 | 0.7518 | 6.013 |
| 2 | action window 1.000000 | branches reevaluated at target | 607 | 0.9927 | 0.562 | 0 | 0.04016 | 0.8029 | 0.8529 | 4.418 |
| 2 | all anchor-feasible | affine anchor model | 813 | 1 | 0.5912 | 0.001644 | 11.35 | 0.5766 | 0.6788 | 8.144 |
| 2 | all anchor-feasible | branches reevaluated at target | 813 | 1 | 0.5912 | 0 | 0.03137 | 0.8321 | 0.854 | 5.468 |
| 2 | anchor winner | affine anchor model | 1 | 1 | 0.05109 | 0.01012 | 0.8475 | 0.4964 | 0.3577 | 0.09504 |
| 2 | anchor winner | branches reevaluated at target | 1 | 0.4526 | 0.05109 | 0.001123 | 0.0415 | 0.3212 | 0.2097 | 0.2739 |
| 2 | target winner control | affine anchor model | 1 | 0.4818 | 1 | 2.499e-05 | 0.3124 | 0.3577 | 0.7879 | 0.01623 |
| 2 | target winner control | branches reevaluated at target | 1 | 1 | 1 | 0 | 0 | 1 | 1 | 0.2809 |

Interpret the controls in order:

- Direct target reevaluation with an anchor-selected set isolates candidate-set staleness.
- Replacing those target reevaluations by anchor affine models adds branch-value and constant-domain approximation.
- The target-winner control removes candidate selection error for one branch.
- Errors are reported in `sys` units; optimizer value still depends on realized improvement per measured compute.

See `error-vs-distance.png`, `window-tradeoff.png`, and `cost-vs-error.png`.
