# verify-numerics — simplified pipeline
#
# Conjecture validation is now in Rust tests:
#   cargo test --test verify_numerics_tests
#
# This Makefile is for ad-hoc exploration only:
#   make collect   — enumerate polytope sigma-nodes (~2 min)
#   make run       — run projection + exact solver on an ad-hoc filter
#   make analyze   — freeform Python analysis of results

POLYTOPES := /tmp/all_polytopes.jsonl
CARGO := cd .. && cargo run --release --bin

# ── Stage 1: collect (one-time, slow) ──

collected_poly.jsonl: $(POLYTOPES)
	$(CARGO) collect_poly -- --polytopes $(POLYTOPES) --max-facets 8

$(POLYTOPES):
	cat ../correctness/correctness.jsonl \
	    ../random-product-sweep/random-product-sweep.jsonl \
	    ../benchmark/benchmark.jsonl \
	    ../ablation/ablation.jsonl > $(POLYTOPES)

# ── Ad-hoc exploration (manual: edit filter, then run) ──
# Example:
#   jq -c 'select(.m >= 6 and .margin > 0.01)' collected_poly.jsonl | head -100 > filtered.jsonl
#   make run INPUT=filtered.jsonl OUTPUT=results.jsonl
#   make analyze RESULTS=results.jsonl

INPUT ?= filtered.jsonl
OUTPUT ?= results.jsonl
RESULTS ?= results.jsonl

run: $(INPUT)
	$(CARGO) verify_numerics -- verify-numerics/$(INPUT) verify-numerics/$(OUTPUT)

analyze: $(RESULTS)
	python3 analyze.py $(RESULTS)

# ── Shortcuts ──

.PHONY: collect run analyze clean test

collect: collected_poly.jsonl

test:
	cd .. && cargo test --release --test verify_numerics_tests

clean:
	rm -f filtered_*.jsonl results_*.jsonl
