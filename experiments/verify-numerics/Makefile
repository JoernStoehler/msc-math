# verify-numerics pipeline — data flow only (no binary builds).
#
# Stage 1: collect (one-time, slow)
#   make collected_poly.jsonl    — enumerate polytope σ-nodes (~2 min)
#   make collected_synth.jsonl   — generate synthetic families (~5 sec)
#
# Stage 2: filter (fast, edit filter_*.rs to change criteria)
#   make filtered_poly_smoke.jsonl     — ~10 rows, pipeline smoke test
#   make filtered_poly_diverse.jsonl   — ~1500 rows, diverse coverage
#   make filtered_synth_all.jsonl      — all synthetic rows
#
# Stage 3: diagnose (moderate, exact rational solver)
#   make results_poly_smoke.jsonl      — seconds
#   make results_poly_diverse.jsonl    — ~2.5 min
#   make results_synth_all.jsonl       — ~1 min
#
# Stage 4: analyze
#   make checks.txt
#
# Shortcuts:
#   make smoke   — pipeline smoke test (~10 rows, seconds)
#   make full    — diverse poly + all synth (~3.5 min)
#   make all     — everything from stage 1

POLYTOPES := /tmp/all_polytopes.jsonl
CARGO := cd .. && cargo run --release --bin

# ── Stage 1: collect ──

collected_poly.jsonl: $(POLYTOPES)
	$(CARGO) collect_poly -- --polytopes $(POLYTOPES) --max-facets 8

collected_synth.jsonl:
	$(CARGO) collect_synth

# Helper: concatenate polytope sources into /tmp/all_polytopes.jsonl
$(POLYTOPES):
	cat ../correctness/correctness.jsonl \
	    ../random-product-sweep/random-product-sweep.jsonl \
	    ../benchmark/benchmark.jsonl \
	    ../ablation/ablation.jsonl > $(POLYTOPES)

# ── Stage 2: filter ──

filtered_poly_smoke.jsonl: collected_poly.jsonl
	$(CARGO) filter_poly_smoke

filtered_poly_diverse.jsonl: collected_poly.jsonl
	$(CARGO) filter_poly_diverse

filtered_synth_all.jsonl: collected_synth.jsonl
	$(CARGO) filter_synth_all

# ── Stage 3: diagnose ──

results_poly_smoke.jsonl: filtered_poly_smoke.jsonl
	$(CARGO) verify_numerics -- verify-numerics/filtered_poly_smoke.jsonl verify-numerics/results_poly_smoke.jsonl

results_poly_diverse.jsonl: filtered_poly_diverse.jsonl
	$(CARGO) verify_numerics -- verify-numerics/filtered_poly_diverse.jsonl verify-numerics/results_poly_diverse.jsonl

results_synth_all.jsonl: filtered_synth_all.jsonl
	$(CARGO) verify_numerics -- verify-numerics/filtered_synth_all.jsonl verify-numerics/results_synth_all.jsonl

# ── Stage 4: analyze ──

checks_smoke.txt: results_poly_smoke.jsonl
	python3 analyze.py results_poly_smoke.jsonl > checks_smoke.txt

checks.txt: results_poly_diverse.jsonl results_synth_all.jsonl
	python3 analyze.py results_poly_diverse.jsonl results_synth_all.jsonl

# ── Shortcuts ──

.PHONY: smoke full all clean

smoke: checks_smoke.txt

full: checks.txt

all: collected_poly.jsonl collected_synth.jsonl full

clean:
	rm -f filtered_*.jsonl results_*.jsonl checks*.txt
