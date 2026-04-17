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
REPO_ROOT := $(abspath $(CURDIR)/../../..)
CARGO := cd $(REPO_ROOT) && cargo run -p dev-numerical-analysis --release --bin

# ── Stage 1: collect (one-time, slow) ──

collected_poly.jsonl: $(POLYTOPES)
	$(CARGO) num-collect-poly -- --polytopes $(POLYTOPES) --max-facets 8

$(POLYTOPES):
	cat $(REPO_ROOT)/experiments/verification/correctness/correctness.jsonl \
	    $(REPO_ROOT)/experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl \
	    $(REPO_ROOT)/experiments/verification/algorithm-comparison/benchmark/benchmark.jsonl \
	    $(REPO_ROOT)/experiments/verification/algorithm-comparison/ablation/ablation.jsonl > $(POLYTOPES)

# ── Ad-hoc exploration (manual: edit filter, then run) ──
# Example:
#   jq -c 'select(.m >= 6 and .margin > 0.01)' collected_poly.jsonl | head -100 > filtered.jsonl
#   make run INPUT=filtered.jsonl OUTPUT=results.jsonl
#   make analyze RESULTS=results.jsonl

INPUT ?= filtered.jsonl
OUTPUT ?= results.jsonl
RESULTS ?= results.jsonl

run: $(INPUT)
	$(CARGO) num-error-bounds -- $(INPUT) $(OUTPUT)

analyze: $(RESULTS)
	uv run analyze.py $(RESULTS)

# ── Shortcuts ──

.PHONY: collect run analyze clean test

collect: collected_poly.jsonl

test:
	cd $(REPO_ROOT) && cargo test -p dev-numerical-analysis --release --test verify_numerics_tests

clean:
	rm -f filtered_*.jsonl results_*.jsonl
