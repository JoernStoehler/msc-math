| File | Type | Origin | Description |
|------|------|--------|-------------|
| action.rs | implementation | billiard-deleted (46095acd) | Action computation using support functions for 2-bounce and 3-bounce trajectories. |
| algorithm.rs | implementation | billiard-deleted (46095acd) | Main billiard algorithm entry point enumerating 2-bounce and 3-bounce edge combinations. |
| archive__tube.rs | implementation | algorithm-archive (f613c166) | Tube algorithm with flow maps, action functions, and branch-and-bound search. |
| billiard.rs | implementation | algorithm-archive (f613c166) | Billiard algorithm infrastructure including Lagrangian factor extraction and polygon types. |
| billiard_lp.rs | implementation | algorithm-archive (f613c166) | LP-based billiard algorithm using linear programming for edge parameter optimization. |
| geom.rs | implementation | tube-reverted (2b71e367) | 2D geometry primitives including symplectic form, affine maps, and polygon intersection. |
| hk2019.rs | implementation | algorithm-archive (f613c166) | HK2019 quadratic programming algorithm marked as broken with incomplete QP solver. |
| polytope.rs | implementation | tube-reverted (2b71e367) | Polytope data structures with vertex enumeration, 2-face enumeration, and enrichment. |
| reverted__tube.rs | implementation | tube-reverted (2b71e367) | Branch-and-bound tube algorithm using affine flow maps and priority queue. |
| solve.rs | implementation | billiard-deleted (46095acd) | Constrained optimization solver validating achievable billiard trajectories. |
| trivialization.rs | implementation | tube-reverted (2b71e367) | 2-face trivialization using quaternion matrices for coordinate transformation. |
| types.rs | implementation | billiard-deleted (46095acd) | Core data structures including Polygon2D, LagrangianProduct, and BilliardTrajectory. |
