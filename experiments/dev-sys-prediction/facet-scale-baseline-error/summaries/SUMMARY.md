# Facet-Count Scale And Baseline Prediction Summary

Generated summary. See `README.md` for interpretation and caveats.

## Global scale by facet count

| facet_count | source_rows | median_flat_norm | median_coord_rms | median_inter_polytope_dist | unit_direction_coord_rms |
| --- | --- | --- | --- | --- | --- |
| 6 | 1536 | 2.49948 | 0.510204 | 3.23558 | 0.204124 |
| 10 | 20754 | 3.23665 | 0.51176 | 4.40908 | 0.158114 |
| 12 | 1536 | 3.53424 | 0.510124 | 3.86589 | 0.144338 |

## Branch window at threshold 0.01

| facet_count | rows | failures | median_near_active_count | max_near_active_count | labels |
| --- | --- | --- | --- | --- | --- |
| 6 | 2 | 0 | 1 | 1 | large_gap:2 |
| 10 | 2 | 0 | 8 | 10 | high_degeneracy:2 |
| 12 | 2 | 0 | 3.5 | 4 | narrow_gap:2 |

## Prediction error by facet count and radius

| facet_count | step | model_source | rows | ok_rows | failure_rows | mean_abs_total_error | median_abs_total_error | normal_se_mean_abs_total_error | p90_abs_total_error | max_abs_total_error | median_abs_active_model_error | median_abs_linearization_error | median_abs_sigma_set_error | target_best_not_in_base_window | median_target_best_base_sys_gap | p90_target_best_base_sys_gap | catch_probability_for_5pct_tail |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 6 | 0.0001 | local-geometry-probe | 8 | 8 | 0 | 1.73942e-07 | 5.61181e-08 | 7.05889e-08 | 4.09811e-07 | 4.5249e-07 | 5.61181e-08 | 5.61181e-08 | 0 | 0 | 0 | 0 | 0.33658 |
| 6 | 0.001 | local-geometry-probe | 8 | 8 | 0 | 1.74346e-05 | 5.69301e-06 | 7.06708e-06 | 4.10939e-05 | 4.54103e-05 | 5.69301e-06 | 5.69301e-06 | 0 | 0 | 0 | 0 | 0.33658 |
| 6 | 0.01 | local-geometry-probe | 8 | 8 | 0 | 0.0339155 | 0.00352436 | 0.0207107 | 0.128336 | 0.129915 | 0.00352436 | 0.00066024 | 0.00273404 | 5 | 0 | 0 | 0.33658 |
| 6 | 0.03 | local-geometry-probe | 8 | 8 | 0 | 0.13744 | 0.0674927 | 0.0611699 | 0.41081 | 0.414481 | 0.0674927 | 0.00854525 | 0.0690961 | 5 | 0 | 0 | 0.33658 |
| 10 | 0.0001 | local-geometry-probe | 10 | 10 | 0 | 3.63634e-08 | 5.26338e-08 | 8.58066e-09 | 6.25222e-08 | 6.25497e-08 | 3.95528e-05 | 5.26338e-08 | 0 | 0 | 0 | 0 | 0.401263 |
| 10 | 0.001 | local-geometry-probe | 10 | 10 | 0 | 2.43794e-06 | 7.2898e-07 | 8.48464e-07 | 5.43329e-06 | 6.23823e-06 | 0.000396093 | 7.2898e-07 | 0 | 0 | 0 | 0.00239648 | 0.401263 |
| 10 | 0.01 | local-geometry-probe | 10 | 10 | 0 | 0.000262933 | 3.13295e-05 | 0.000138849 | 0.000682584 | 0.00133185 | 0.0018758 | 3.13295e-05 | 0 | 1 | 0.00239648 | 0.0109837 | 0.401263 |
| 10 | 0.03 | local-geometry-probe | 10 | 9 | 1 | 0.0041549 | 0.000272242 | 0.00292374 | 0.0095716 | 0.0268842 | 0.00451851 | 0.000272242 | 0 | 2 | 0.00239648 | 0.0109837 | 0.369751 |
| 12 | 0.0001 | local-geometry-probe | 10 | 10 | 0 | 8.07199e-09 | 8.93451e-09 | 1.72394e-09 | 1.23429e-08 | 1.77146e-08 | 8.93451e-09 | 8.93451e-09 | 1.11022e-16 | 5 | 0 | 0 | 0.401263 |
| 12 | 0.001 | local-geometry-probe | 10 | 10 | 0 | 8.07274e-07 | 8.94042e-07 | 1.7232e-07 | 1.23494e-06 | 1.76989e-06 | 8.94042e-07 | 8.94042e-07 | 0 | 5 | 0 | 0 | 0.401263 |
| 12 | 0.01 | local-geometry-probe | 10 | 10 | 0 | 6.6192e-05 | 5.30811e-05 | 1.65459e-05 | 0.000118853 | 0.000175426 | 9.00008e-05 | 5.30811e-05 | 0 | 5 | 0 | 0.00396842 | 0.401263 |
| 12 | 0.03 | local-geometry-probe | 10 | 10 | 0 | 0.00859266 | 0.000694388 | 0.0042453 | 0.029845 | 0.0333681 | 0.0055963 | 0.000477797 | 1.11022e-16 | 5 | 0 | 0.00396842 | 0.401263 |
