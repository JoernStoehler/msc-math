"""
Goal: Check all propositions and bounds from the Q accuracy experiment.
      Report observed ranges for EHZ-like vs abstract datasets.
      Identify violations, tightest cases, and filtering effects.
Input: experiments/verify-numerics/results.jsonl
Output: stdout (tables), experiments/verify-numerics/checks.txt
"""

import json
import math
import sys
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "results.jsonl"
OUTPUT_PATH = EXPERIMENT_DIR / "checks.txt"

# ── EHZ-like families (feasible by construction, well-conditioned) ──
# vs abstract stress-test families (deliberately extreme)
# Natural (polytope-derived) families are classified as EHZ-like.
EHZ_LIKE_FAMILIES = {
    "identity", "random_dense", "ehz_like", "small_m6", "large_m16",
    "feasible_constructed", "near_singular_h", "singular_h", "indefinite_h",
    "tiny_lam_min",
    "polytope_sigma_node",  # natural dataset: actual polytope inputs
}
STRESS_FAMILIES = {
    "ill_cond_c", "large_h_ill_c", "double_singular",
    "clustered_h_eig", "clustered_m_eig",
}


@dataclass
class CheckResult:
    name: str
    type: str          # "assumption", "conjecture", "bug_detection", "proven_bound", "proven_identity"
    assumes: str       # which propositions this depends on
    used_for: str      # "correctness", "recorded", "sanity", "runtime_cert", "structural"
    threshold: str     # human-readable threshold
    n_ehz: int         # cases checked (EHZ-like)
    n_stress: int      # cases checked (stress)
    range_ehz: str     # observed range in EHZ-like
    range_stress: str  # observed range in stress
    tightest_ehz: str  # tightest case in EHZ-like
    tightest_stress: str
    violations_ehz: int
    violations_stress: int


def load_data():
    rows = []
    with open(DATA_PATH) as f:
        for line in f:
            rows.append(json.loads(line))
    return rows


def feasible(r):
    return r["verdict_exact"] == "feasible" and r["verdict_saddle"] == "feasible"


def is_ehz(r):
    return r["family"] in EHZ_LIKE_FAMILIES


def fmt_range(vals):
    if not vals:
        return "—"
    vs = sorted(vals)
    return f"{vs[0]:.2e} – {vs[-1]:.2e}"


def fmt_tight(vals, threshold, direction):
    """direction: 'upper' (val/threshold, want ≤1) or 'lower' (threshold/val, want ≤1) or 'equal' (|val-target|)"""
    if not vals:
        return "—"
    vs = sorted(vals)
    if direction == "upper":
        return f"{vs[-1]/threshold:.3f}" if threshold != 0 else "—"
    elif direction == "lower":
        return f"{threshold/vs[0]:.1e}" if vs[0] != 0 else "inf"
    elif direction == "equal":
        worst = max(abs(v - threshold) for v in vs)
        return f"±{worst:.2e}"
    return "—"


def safe_get(r, key, default=float('nan')):
    v = r.get(key, default)
    if v is None:
        return default
    return v


def check_all(rows):
    sp = [r for r in rows if feasible(r)]
    sp_ehz = [r for r in sp if is_ehz(r)]
    sp_stress = [r for r in sp if not is_ehz(r)]

    results = []
    out_lines = []

    def log(s=""):
        out_lines.append(s)
        print(s)

    log(f"Q Accuracy Checks — {len(rows)} problems, {len(sp)} feasible "
        f"({len(sp_ehz)} EHZ-like, {len(sp_stress)} stress-test)")
    log()

    # ══════════════════════════════════════════════════════════════
    log("=" * 120)
    log("PROPOSITIONS (assumptions, conjectures, bug detection)")
    log("=" * 120)

    def check_prop(name, typ, assumes, used_for, threshold_str,
                   extract_fn, check_fn, direction, threshold_val):
        vals_ehz = [v for r in sp_ehz if (v := extract_fn(r)) is not None and math.isfinite(v)]
        vals_stress = [v for r in sp_stress if (v := extract_fn(r)) is not None and math.isfinite(v)]
        viol_ehz = sum(1 for v in vals_ehz if not check_fn(v))
        viol_stress = sum(1 for v in vals_stress if not check_fn(v))

        log(f"\n  {name}")
        log(f"    Type: {typ}  |  Assumes: {assumes}  |  Used for: {used_for}")
        log(f"    Threshold: {threshold_str}")
        log(f"    EHZ-like:    n={len(vals_ehz):4d}  range={fmt_range(vals_ehz):30s}  "
            f"tightest={fmt_tight(vals_ehz, threshold_val, direction):>12s}  violations={viol_ehz}")
        log(f"    Stress-test: n={len(vals_stress):4d}  range={fmt_range(vals_stress):30s}  "
            f"tightest={fmt_tight(vals_stress, threshold_val, direction):>12s}  violations={viol_stress}")

    # P4: sigma_min(C) > 1e-12
    check_prop("P4: σ_min(C) > 1e-12",
               "assumption", "—", "B2, B3 (E₁ finite)",
               "> 1e-12",
               lambda r: safe_get(r, "sigma_min_c"),
               lambda v: v > 1e-12, "lower", 1e-12)

    # P5: REMOVED (2026-03-31). ‖H‖/σ_min(C) ≤ 100 falsified on natural polytope data.
    # σ_min(C) → 0 for m ≤ 5 (C rank-deficient), ratio unbounded.
    # Still recorded as a diagnostic (not a conjecture):
    check_prop("P5: ‖H‖/σ_min(C) (diagnostic, no threshold)",
               "diagnostic", "—", "recorded (conjecture falsified 2026-03-31)",
               "none",
               lambda r: safe_get(r, "norm_h") / safe_get(r, "sigma_min_c")
                         if safe_get(r, "sigma_min_c") > 0 else None,
               lambda v: True, "upper", float('inf'))

    # P6: ||r_beta|| < 1e-3 (full-rank M only)
    # Rank-deficient cases have ||r_beta|| up to 0.63 by construction (discarded eigenspace).
    check_prop("P6: ‖r_β‖ < 1e-3 (full-rank M only)",
               "bug_detection", "—", "sanity",
               "< 1e-3",
               lambda r: safe_get(r, "norm_r_beta") if r["sp_rank"] == r["m"] + 5 else None,
               lambda v: v < 1e-3, "upper", 1e-3)

    # P7: ||beta|| <= 2
    check_prop("P7: ‖β‖ ≤ 2",
               "bug_detection", "—", "sanity (EHZ: ‖β‖₁=1)",
               "≤ 2",
               lambda r: safe_get(r, "norm_beta_sp"),
               lambda v: v <= 2.0, "upper", 2.0)

    # P8: ||r_lambda|| < 1e-6
    check_prop("P8: ‖r_λ‖ < 1e-6",
               "bug_detection", "—", "sanity",
               "< 1e-6",
               lambda r: safe_get(r, "norm_r_lambda"),
               lambda v: v < 1e-6, "upper", 1e-6)

    # ══════════════════════════════════════════════════════════════
    log()
    log("=" * 120)
    log("PROVEN BOUNDS (from math.tex, checked against exact rational ground truth)")
    log("=" * 120)

    def check_bound(name, typ, assumes, used_for, threshold_str,
                    extract_ratio_fn, check_fn, direction, threshold_val):
        vals_ehz = [v for r in sp_ehz if (v := extract_ratio_fn(r)) is not None and math.isfinite(v)]
        vals_stress = [v for r in sp_stress if (v := extract_ratio_fn(r)) is not None and math.isfinite(v)]
        viol_ehz = sum(1 for v in vals_ehz if not check_fn(v))
        viol_stress = sum(1 for v in vals_stress if not check_fn(v))

        log(f"\n  {name}")
        log(f"    Type: {typ}  |  Assumes: {assumes}  |  Used for: {used_for}")
        log(f"    Threshold: {threshold_str}")
        log(f"    EHZ-like:    n={len(vals_ehz):4d}  range={fmt_range(vals_ehz):30s}  "
            f"tightest={fmt_tight(vals_ehz, threshold_val, direction):>12s}  violations={viol_ehz}")
        log(f"    Stress-test: n={len(vals_stress):4d}  range={fmt_range(vals_stress):30s}  "
            f"tightest={fmt_tight(vals_stress, threshold_val, direction):>12s}  violations={viol_stress}")

        if viol_ehz > 0 or viol_stress > 0:
            # Show worst violations
            all_v = [(v, r) for r in sp for v in [extract_ratio_fn(r)]
                     if v is not None and math.isfinite(v) and not check_fn(v)]
            all_v.sort(key=lambda x: -abs(x[0]))
            for v, r in all_v[:3]:
                log(f"    VIOLATION: ratio={v:.4e} family={r['family']} inst={r['instance']} m={r['m']}")

    # B2: ||lambda*|| <= ||H||*||beta*||/sigma_min(C)
    check_bound("B2: ‖λ*‖ / (‖H‖·‖β*‖/σ_min(C)) ≤ 1",
                "proven_bound", "P3, P4", "B3 proof",
                "ratio ≤ 1",
                lambda r: safe_get(r, "lambda_bound_ratio"),
                lambda v: v <= 1.0 + 1e-10, "upper", 1.0)

    # B3: |Q - Q*| <= ||H||*||beta||*||r||/sigma_min(C)
    def b3_ratio(r):
        err = abs(safe_get(r, "err_saddle"))
        e1 = safe_get(r, "e1_bound")
        if err > 0 and e1 > 0 and math.isfinite(err) and math.isfinite(e1):
            return err / e1
        return None

    check_bound("B3: |Q−Q*| / (‖H‖·‖β‖·‖r‖/σ_min(C)) ≤ 1",
                "proven_bound", "P3, P4, β*>0", "runtime certification",
                "ratio ≤ 1",
                b3_ratio,
                lambda v: v <= 1.0 + 1e-10, "upper", 1.0)

    # B4: |Q_raw - Q*| <= same
    def b4_ratio(r):
        err = abs(safe_get(r, "err_raw_saddle"))
        e1 = safe_get(r, "e1_bound")
        if err > 0 and e1 > 0 and math.isfinite(err) and math.isfinite(e1):
            return err / e1
        return None

    check_bound("B4: |Q_raw−Q*| / (‖H‖·‖β‖·‖r‖/σ_min(C)) ≤ 1",
                "proven_bound", "P3, P4, β*>0", "correction helps",
                "ratio ≤ 1",
                b4_ratio,
                lambda v: v <= 1.0 + 1e-10, "upper", 1.0)

    # B5: 1st/2nd = 2 (identity, full-rank M only)
    def b5_ratio(r):
        fo = safe_get(r, "first_order_beta0")
        so = safe_get(r, "second_order_beta0")
        rank = r.get("sp_rank", 0)
        size = r.get("m", 0) + 5
        if rank < size:  # rank-deficient: identity doesn't hold
            return None
        if fo > 1e-10 and so > 1e-10 and math.isfinite(fo) and math.isfinite(so):
            return fo / so
        return None

    check_bound("B5: 1st/2nd = 2  (identity, full-rank M only)",
                "proven_identity", "P3, x*∈col(M)", "structural",
                "ratio = 2 ± 1e-3",
                b5_ratio,
                lambda v: abs(v - 2.0) < 1e-3, "equal", 2.0)

    # B6: correction doesn't 2x worsen Q (when raw err > noise)
    def b6_ratio(r):
        ec = abs(safe_get(r, "err_saddle"))
        er = abs(safe_get(r, "err_raw_saddle"))
        if er > 1e-14 and ec > 0 and math.isfinite(ec) and math.isfinite(er):
            return ec / er
        return None

    check_bound("B6: |Q−Q*| / |Q_raw−Q*| ≤ 2  (correction doesn't worsen)",
                "observation", "—", "sanity",
                "ratio ≤ 2",
                b6_ratio,
                lambda v: v <= 2.0, "upper", 2.0)

    # ══════════════════════════════════════════════════════════════
    log()
    log("=" * 120)
    log("CORRECTION EFFECTIVENESS")
    log("=" * 120)

    for label, group in [("EHZ-like", sp_ehz), ("Stress-test", sp_stress)]:
        raw = sorted([abs(safe_get(r, "err_raw_saddle"))
                      for r in group if math.isfinite(safe_get(r, "err_raw_saddle"))
                      and abs(safe_get(r, "err_raw_saddle")) > 0])
        corr = sorted([abs(safe_get(r, "err_saddle"))
                       for r in group if math.isfinite(safe_get(r, "err_saddle"))
                       and abs(safe_get(r, "err_saddle")) > 0])
        helps_10x = sum(1 for r in group
                        if abs(safe_get(r, "err_raw_saddle")) > 0
                        and abs(safe_get(r, "err_saddle")) > 0
                        and abs(safe_get(r, "err_raw_saddle")) / abs(safe_get(r, "err_saddle")) > 10)
        if raw:
            log(f"\n  {label} ({len(group)} cases):")
            log(f"    |Q_raw−Q*|:  median {raw[len(raw)//2]:.1e},  max {raw[-1]:.1e}")
            log(f"    |Q−Q*|:      median {corr[len(corr)//2]:.1e},  max {corr[-1]:.1e}")
            log(f"    Correction helps >10x: {helps_10x}/{len(group)}")

    # ══════════════════════════════════════════════════════════════
    log()
    log("=" * 120)
    log("β > 0 CLASSIFICATION")
    log("=" * 120)

    # Compare f64 solver β > 0 classification against exact rational solver.
    # verdict_saddle: "feasible" means f64 solver found β > 0.
    # verdict_exact: "feasible" means exact solver found β > 0.
    # margin_saddle: min(β) from f64 solver.
    # margin_exact: min(β*) from exact solver (None if exact infeasible).

    for label, group in [("EHZ-like", sp_ehz), ("Stress-test", sp_stress), ("Natural polytope",
            [r for r in sp if r["family"] == "polytope_sigma_node"])]:
        if not group:
            continue

        # Classification matrix
        tp = fp = fn_ = tn = skip = 0
        margin_errs = []
        beta_errs = []
        for r in group:
            me = safe_get(r, "margin_exact")
            ms = safe_get(r, "margin_saddle")
            vs = r.get("verdict_saddle", "")
            ve = r.get("verdict_exact", "")
            if not math.isfinite(me) or not math.isfinite(ms):
                skip += 1
                continue
            exact_pos = me > 1e-12
            solver_pos = ms > 1e-9
            if exact_pos and solver_pos: tp += 1
            elif not exact_pos and not solver_pos: tn += 1
            elif solver_pos and not exact_pos: fp += 1
            elif not solver_pos and exact_pos: fn_ += 1
            margin_errs.append(abs(ms - me))

        beta_errs_all = sorted([safe_get(r, "beta_err_saddle")
                                for r in group
                                if math.isfinite(safe_get(r, "beta_err_saddle"))])

        log(f"\n  {label} ({len(group)} problems, {skip} skipped):")
        log(f"    True positive  (both β>0):  {tp}")
        log(f"    True negative  (both β≤0):  {tn}")
        log(f"    False positive (solver β>0, exact β≤0): {fp}")
        log(f"    False negative (solver β≤0, exact β>0): {fn_}")

        if margin_errs:
            margin_errs.sort()
            log(f"    |margin_f64 - margin_exact|: med={margin_errs[len(margin_errs)//2]:.2e}, max={margin_errs[-1]:.2e}")

        if beta_errs_all:
            log(f"    ‖β_f64 - β*‖₂: med={beta_errs_all[len(beta_errs_all)//2]:.2e}, max={beta_errs_all[-1]:.2e}")

        # Margin distribution for true positives
        tp_margins = sorted([safe_get(r, "margin_saddle")
                            for r in group
                            if safe_get(r, "margin_exact") > 1e-12 and safe_get(r, "margin_saddle") > 1e-9])
        if tp_margins:
            log(f"    True-positive margin_f64: min={tp_margins[0]:.2e}, med={tp_margins[len(tp_margins)//2]:.2e}, max={tp_margins[-1]:.2e}")

        # Q sign classification
        q_pos = sum(1 for r in group if safe_get(r, "q_exact") > 0 and safe_get(r, "q_saddle") > 0)
        q_neg_exact = sum(1 for r in group if safe_get(r, "q_exact") <= 0)
        q_neg_f64 = sum(1 for r in group if safe_get(r, "q_saddle") <= 0)
        log(f"    Q sign: both Q>0: {q_pos}, Q*≤0: {q_neg_exact}, Q_f64≤0: {q_neg_f64}")

    # ══════════════════════════════════════════════════════════════
    log()
    # ══════════════════════════════════════════════════════════════
    log("\n" + "=" * 120)
    log("PERTURBATION CHAIN — β > 0 CERTIFICATION BOUND [lem:link-beta]")
    log("=" * 120)

    # Filter: projection solver returned true/indeterminate AND exact is feasible
    proj_feasible = [r for r in rows
                     if r["verdict_exact"] == "feasible"
                     and r.get("verdict_projection") in ("true", "indeterminate")
                     and math.isfinite(safe_get(r, "proj_eta_ratio"))]

    if proj_feasible:
        eta_ratios = sorted([safe_get(r, "proj_eta_ratio") for r in proj_feasible
                             if math.isfinite(safe_get(r, "proj_eta_ratio"))])
        eta_maxes = sorted([safe_get(r, "proj_eta_max") for r in proj_feasible
                            if math.isfinite(safe_get(r, "proj_eta_max"))])
        beta_errs = sorted([safe_get(r, "proj_beta_err_inf") for r in proj_feasible
                            if math.isfinite(safe_get(r, "proj_beta_err_inf"))])
        cert_margins = sorted([safe_get(r, "proj_certified_margin") for r in proj_feasible
                               if math.isfinite(safe_get(r, "proj_certified_margin"))])
        eps_gammas = sorted([safe_get(r, "proj_eps_gamma") for r in proj_feasible
                             if math.isfinite(safe_get(r, "proj_eps_gamma"))])

        log(f"\n  Dataset: {len(proj_feasible)} problems (projection feasible + exact feasible)")
        log(f"")
        log(f"  η_k bound validity (max_k |β̃_k - β*_k| / η_k):")
        log(f"    Should be ≤ 1 for the bound to be valid.")
        if eta_ratios:
            log(f"    median: {eta_ratios[len(eta_ratios)//2]:.4e}")
            log(f"    p99:    {eta_ratios[int(len(eta_ratios)*0.99)]:.4e}")
            log(f"    max:    {eta_ratios[-1]:.4e}")
            n_invalid = sum(1 for r in eta_ratios if r > 1.0)
            log(f"    violations (ratio > 1): {n_invalid}/{len(eta_ratios)}")

        log(f"")
        log(f"  Actual β error (max_k |β̃_k - β*_k|):")
        if beta_errs:
            log(f"    median: {beta_errs[len(beta_errs)//2]:.4e}")
            log(f"    p99:    {beta_errs[int(len(beta_errs)*0.99)]:.4e}")
            log(f"    max:    {beta_errs[-1]:.4e}")

        log(f"")
        log(f"  η_k bound magnitude (max_k η_k):")
        if eta_maxes:
            log(f"    median: {eta_maxes[len(eta_maxes)//2]:.4e}")
            log(f"    p99:    {eta_maxes[int(len(eta_maxes)*0.99)]:.4e}")
            log(f"    max:    {eta_maxes[-1]:.4e}")
            n_inf = sum(1 for e in eta_maxes if e > 1e100)
            log(f"    infinite (η = ∞): {n_inf}/{len(eta_maxes)}")

        log(f"")
        log(f"  Certified margin min_k(β̃_k - η_k):")
        if cert_margins:
            n_certified = sum(1 for m in cert_margins if m > 0)
            n_finite = sum(1 for m in cert_margins if math.isfinite(m))
            log(f"    β > 0 certified: {n_certified}/{n_finite} ({100*n_certified/max(n_finite,1):.1f}%)")
            positive_margins = sorted([m for m in cert_margins if m > 0])
            if positive_margins:
                log(f"    certified margin min: {positive_margins[0]:.4e}")
                log(f"    certified margin max: {positive_margins[-1]:.4e}")

        log(f"")
        log(f"  ε_γ (eigenvalue perturbation threshold):")
        if eps_gammas:
            log(f"    median: {eps_gammas[len(eps_gammas)//2]:.4e}")
            log(f"    max:    {eps_gammas[-1]:.4e}")

        # Breakdown by family
        log(f"\n  Per-family breakdown:")
        log(f"  {'Family':25s} {'n':>5s} {'max η_ratio':>12s} {'med η_max':>12s} "
            f"{'certified':>10s} {'max|δβ|':>10s}")
        log("  " + "-" * 80)
        by_fam_proj = defaultdict(list)
        for r in proj_feasible:
            by_fam_proj[r["family"]].append(r)
        for fam in sorted(by_fam_proj.keys()):
            group = by_fam_proj[fam]
            ratios = sorted([safe_get(r, "proj_eta_ratio") for r in group
                             if math.isfinite(safe_get(r, "proj_eta_ratio"))])
            etas = sorted([safe_get(r, "proj_eta_max") for r in group
                           if math.isfinite(safe_get(r, "proj_eta_max"))])
            errs = sorted([safe_get(r, "proj_beta_err_inf") for r in group
                           if math.isfinite(safe_get(r, "proj_beta_err_inf"))])
            n_cert = sum(1 for r in group
                         if math.isfinite(safe_get(r, "proj_certified_margin"))
                         and safe_get(r, "proj_certified_margin") > 0)
            log(f"  {fam:25s} {len(group):5d} "
                f"{ratios[-1] if ratios else float('nan'):12.4e} "
                f"{etas[len(etas)//2] if etas else float('nan'):12.4e} "
                f"{n_cert:>4d}/{len(group):<5d} "
                f"{errs[-1] if errs else float('nan'):10.4e}")
    else:
        log("\n  No projection-feasible + exact-feasible problems found.")

    log("")
    log("=" * 120)
    log("FAMILY SUMMARY")
    log("=" * 120)

    by_fam = defaultdict(list)
    for r in sp:
        by_fam[r["family"]].append(r)

    log(f"\n  {'Family':25s} {'n':>5s} {'max|Q−Q*|':>10s} {'med|Q−Q*|':>10s} "
        f"{'σ_min(C)':>10s} {'‖H‖/σ_min':>10s} {'rank':>8s}")
    log("  " + "-" * 85)

    for fam in sorted(by_fam.keys()):
        group = by_fam[fam]
        errs = sorted([abs(safe_get(r, "err_saddle"))
                       for r in group if math.isfinite(safe_get(r, "err_saddle"))])
        smins = [safe_get(r, "sigma_min_c") for r in group
                 if math.isfinite(safe_get(r, "sigma_min_c"))]
        ratios = [safe_get(r, "norm_h") / safe_get(r, "sigma_min_c")
                  for r in group if safe_get(r, "sigma_min_c") > 0]
        ranks = [f"{r['sp_rank']}/{r['m']+5}" for r in group]
        full_rank = sum(1 for r in group if r["sp_rank"] == r["m"] + 5)

        marker = "  " if fam in EHZ_LIKE_FAMILIES else "* "
        log(f"  {marker}{fam:23s} {len(group):5d} {errs[-1] if errs else 0:10.2e} "
            f"{errs[len(errs)//2] if errs else 0:10.2e} "
            f"{min(smins) if smins else 0:10.2e} "
            f"{max(ratios) if ratios else 0:10.2e} "
            f"{full_rank}/{len(group)}")

    log(f"\n  (* = stress-test family)")

    # Write output
    with open(OUTPUT_PATH, "w") as f:
        f.write("\n".join(out_lines) + "\n")
    print(f"\nWritten to {OUTPUT_PATH}")


if __name__ == "__main__":
    rows = load_data()
    check_all(rows)
