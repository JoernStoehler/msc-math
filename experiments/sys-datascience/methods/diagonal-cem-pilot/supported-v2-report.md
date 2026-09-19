# Supported-v2 diagonal CEM pilot

Completed in 215.797 s under the 240 s scientific wall cap: 1344 unique requests all succeeded, charged as 256 per arm per seed (1536 total) with each seed’s 64 shared initial requests charged to both. All generations contain 64 accepted constructions; all CEM updates use completed populations and 16 elites. The unchanged current-body evaluator used 5 s/input and two-thread limits under the common evaluator flock.

This is a small descriptive three-seed result: CEM improves both paired maxima and distinct-top-eight medians in every seed. It supports further investigation in this admitted product source; it establishes neither universal optimizer superiority nor a threshold violation. No sys value exceeded 1.

V1 was exposed and stopped procedurally after one PrimalNormOutOfRange target failure. V2 mechanically conditions both proposal arms on the same evaluator facet, finite, dual-norm, exact-geometry and primal-norm admission policy before acceptance. Seeds, random stream material and optimizer constants remain unchanged. The acceptance source changed, so this is explicitly supported-v2, not an untouched preregistration. Original frozen matching source and binary bytes are in source-v1/; v1 outputs remain in artifacts/.

| Seed | IID max | CEM max | IID top8 median | CEM top8 median |
|---|---:|---:|---:|---:|
| 202609140301 | 0.759574455 | 0.798512700 | 0.728815347 | 0.782273903 |
| 202609140302 | 0.810904406 | 0.831348119 | 0.753788417 | 0.776334044 |
| 202609140303 | 0.760067722 | 0.855654742 | 0.722309634 | 0.813741738 |

Per-arm seconds include shared initialization charged to each arm. Update costs belong only to CEM. Evaluator process time includes Python supervisor/process overhead; request time is the sum of recorded individual requests. These clocks are measured serial process costs, not an equal-wall-budget comparison.

| Seed | Arm | Generation s | Update s | Evaluator process s | Evaluator requests s | Attempts / rejections |
|---|---|---:|---:|---:|---:|---:|
| 202609140301 | iid | 44.166 | 0.0000 | 6.457 | 5.256 | 1132 / 876 |
| 202609140301 | cem | 28.946 | 0.0182 | 6.324 | 4.886 | 617 / 361 |
| 202609140302 | iid | 41.440 | 0.0000 | 5.974 | 4.933 | 1185 / 929 |
| 202609140302 | cem | 29.271 | 0.0171 | 6.161 | 5.156 | 599 / 343 |
| 202609140303 | iid | 39.832 | 0.0000 | 6.373 | 5.243 | 1170 / 914 |
| 202609140303 | cem | 30.255 | 0.0181 | 6.180 | 5.165 | 607 / 351 |

Actual unique-process totals: generation 182.245 s, updates 0.0534 s, evaluator processes 32.915 s; individual evaluator requests 26.805 s. All 4407 construction attempts and 3063 rejections are retained, including 2 evaluator-admission rejections. Generation timings include construction and admission work. No target errors or timeouts occurred.

Validation: analyze.py verified the pre-target frozen source/binary hashes, all 21 input/result batches, 1344 unique IDs, one evaluator identity, product route, capacity bounds, and sys=capacity²/(2 volume). analysis.json contains every arm’s eight top values and costs; best-so-far.jsonl contains 1536 charged rows. No library/adapter changes, commits, or merges.

Files: src/main.rs, run.py, analyze.py; commands-supported-v2.sh; artifacts-supported-v2/frozen-config.json, run-status.json, analysis.json, best-so-far.jsonl, run.time, and batch construction/request/result/update files.
