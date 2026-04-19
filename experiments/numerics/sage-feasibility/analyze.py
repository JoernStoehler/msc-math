"""
Goal: run a Sage end-to-end HK2017-style unpruned search on a deterministic
benchmark bank and compare it against the Rust `f64` unpruned baseline.
Input Artifacts: experiments/numerics/sage-feasibility/{sage-feasibility-input.jsonl,smoke-sage-feasibility-input.jsonl}
Output Artifacts: experiments/numerics/sage-feasibility/{sage-feasibility.jsonl,smoke-sage-feasibility.jsonl}
"""

from __future__ import annotations

import argparse
import itertools
import json
import time
from pathlib import Path

try:
    from sage.all import QQ, RDF, Matrix, NumberField, PolynomialRing, RR, vector, tan, pi
except ModuleNotFoundError as exc:
    raise SystemExit("run this script with `sage -python analyze.py`") from exc


EXPERIMENT_DIR = Path(__file__).resolve().parent
INPUT_FILENAME = "sage-feasibility-input.jsonl"
SMOKE_INPUT_FILENAME = "smoke-sage-feasibility-input.jsonl"
OUTPUT_FILENAME = "sage-feasibility.jsonl"
SMOKE_OUTPUT_FILENAME = "smoke-sage-feasibility.jsonl"

RDF_MODE = "rdf"
RATIONAL_EXACT_MODE = "rational_exact"
ALGEBRAIC_EXACT_MODE = "algebraic_exact"
EPS_BETA_POSITIVE = 1.0e-12
EPS_Q_POSITIVE = 1.0e-15

CANONICAL_TIMEOUT_SECONDS = {
    RDF_MODE: 30.0,
    RATIONAL_EXACT_MODE: 30.0,
    ALGEBRAIC_EXACT_MODE: 60.0,
}

SMOKE_TIMEOUT_SECONDS = {
    RDF_MODE: 5.0,
    RATIONAL_EXACT_MODE: 5.0,
    ALGEBRAIC_EXACT_MODE: 10.0,
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--canonical", action="store_true")
    parser.add_argument("--smoke", action="store_true")
    args = parser.parse_args()
    if args.canonical and args.smoke:
        raise SystemExit("`--canonical` and `--smoke` are separate output modes")
    return args


def input_path(canonical: bool) -> Path:
    return EXPERIMENT_DIR / (INPUT_FILENAME if canonical else SMOKE_INPUT_FILENAME)


def output_path(canonical: bool) -> Path:
    return EXPERIMENT_DIR / (OUTPUT_FILENAME if canonical else SMOKE_OUTPUT_FILENAME)


def timeout_for(canonical: bool, scalar_mode: str) -> float:
    if canonical:
        return CANONICAL_TIMEOUT_SECONDS[scalar_mode]
    return SMOKE_TIMEOUT_SECONDS[scalar_mode]


def read_jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, rows: list[dict]) -> None:
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row))
            handle.write("\n")


def decode_rational(coeff_json: dict):
    return QQ(coeff_json["numer"]) / QQ(coeff_json["denom"])


class FieldContext:
    def __init__(self, exact_field: str, scalar_mode: str):
        self.exact_field = exact_field
        self.scalar_mode = scalar_mode

        if scalar_mode == RDF_MODE:
            self.field = RDF
            self.gen = RDF(tan(pi / 5)) if exact_field == "q_tan_pi_fifth" else None
            return

        if scalar_mode == RATIONAL_EXACT_MODE:
            if exact_field != "rational":
                raise ValueError(f"{scalar_mode} is unsupported for {exact_field}")
            self.field = QQ
            self.gen = None
            return

        if scalar_mode == ALGEBRAIC_EXACT_MODE:
            if exact_field != "q_tan_pi_fifth":
                raise ValueError(f"{scalar_mode} is unsupported for {exact_field}")
            poly_ring = PolynomialRing(QQ, names="x")
            x = poly_ring.gen()
            poly = x**4 - 10 * x**2 + 5
            self.field = NumberField(poly, names="t", embedding=RR(tan(pi / 5)))
            self.gen = self.field.gen()
            return

        raise ValueError(f"unsupported scalar mode: {scalar_mode}")

    def scalar(self, element_json: dict):
        coeffs = [decode_rational(coeff_json) for coeff_json in element_json["coeffs"]]
        if self.scalar_mode == RDF_MODE:
            if self.exact_field == "rational":
                if len(coeffs) != 1:
                    raise ValueError("rational element must have one coefficient")
                return self.field(coeffs[0])
            value = self.field.zero()
            power = self.field.one()
            for coeff in coeffs:
                value += self.field(coeff) * power
                power *= self.gen
            return value

        if self.exact_field == "rational":
            if len(coeffs) != 1:
                raise ValueError("rational element must have one coefficient")
            return self.field(coeffs[0])
        return self.field(coeffs)

    def as_float(self, value) -> float:
        if self.scalar_mode == RDF_MODE:
            return float(value)
        if self.scalar_mode == RATIONAL_EXACT_MODE:
            return float(RR(value))
        return float(value.n())

    @staticmethod
    def zero():
        return 0

    @staticmethod
    def is_positive(value) -> bool:
        return value > 0

    @staticmethod
    def is_negative(value) -> bool:
        return value < 0

    @staticmethod
    def is_nonpositive(value) -> bool:
        return value <= 0

    def beta_is_admissible(self, value) -> bool:
        if self.scalar_mode == RDF_MODE:
            return float(value) > EPS_BETA_POSITIVE
        return value > 0

    def q_is_admissible(self, value) -> bool:
        if self.scalar_mode == RDF_MODE:
            return float(value) > EPS_Q_POSITIVE
        return value > 0


def omega0(left, right):
    return left[0] * right[2] - right[0] * left[2] + left[1] * right[3] - right[1] * left[3]


def build_kkt_matrix(dual_vertices, sigma):
    field = dual_vertices[0].base_ring()
    m = len(sigma)
    size = m + 5
    matrix_rows = [[field.zero() for _ in range(size)] for _ in range(size)]
    rhs = [field.zero() for _ in range(size)]

    for i in range(m):
        for j in range(i + 1, m):
            value = omega0(dual_vertices[sigma[i]], dual_vertices[sigma[j]])
            matrix_rows[i][j] = value
            matrix_rows[j][i] = value

    for i in range(m):
        for dim in range(4):
            value = dual_vertices[sigma[i]][dim]
            matrix_rows[i][m + dim] = value
            matrix_rows[m + dim][i] = value

    for row in range(m):
        matrix_rows[row][m + 4] = field.one()
        matrix_rows[m + 4][row] = field.one()
    rhs[m + 4] = field.one()

    return Matrix(field, matrix_rows), vector(field, rhs)


def back_substitute(aug_rows, pivot_positions, width: int):
    solution = [aug_rows[0][0].parent().zero() for _ in range(width)]
    for pivot_row, pivot_col in reversed(pivot_positions):
        rhs = aug_rows[pivot_row][width]
        for col in range(pivot_col + 1, width):
            rhs -= aug_rows[pivot_row][col] * solution[col]
        pivot = aug_rows[pivot_row][pivot_col]
        if pivot == 0:
            return None
        solution[pivot_col] = rhs / pivot
    return solution


def gauss_solve_with_null_space(matrix, rhs):
    width = len(rhs)
    aug_rows = [list(matrix.row(i)) + [rhs[i]] for i in range(width)]
    pivot_positions = []
    free_cols = []
    current_row = 0

    for col in range(width):
        pivot_row = next(
            (row for row in range(current_row, width) if aug_rows[row][col] != 0),
            None,
        )
        if pivot_row is None:
            free_cols.append(col)
            continue

        aug_rows[current_row], aug_rows[pivot_row] = aug_rows[pivot_row], aug_rows[current_row]
        pivot = aug_rows[current_row][col]
        pivot_tail = aug_rows[current_row][col : width + 1]
        for row in range(current_row + 1, width):
            if aug_rows[row][col] == 0:
                continue
            factor = aug_rows[row][col] / pivot
            for offset, value in enumerate(pivot_tail):
                aug_rows[row][col + offset] -= value * factor
        pivot_positions.append((current_row, col))
        current_row += 1

    rank = len(pivot_positions)
    for row in range(rank, width):
        if aug_rows[row][width] != 0:
            return None

    particular = back_substitute(aug_rows, pivot_positions, width)
    if particular is None:
        return None

    if not free_cols:
        return {"particular": particular, "null_space": []}

    null_space = []
    for free_col in free_cols:
        basis = [matrix.base_ring().zero() for _ in range(width)]
        basis[free_col] = matrix.base_ring().one()
        for pivot_row, pivot_col in reversed(pivot_positions):
            value = matrix.base_ring().zero()
            for col in range(pivot_col + 1, width):
                value += aug_rows[pivot_row][col] * basis[col]
            basis[pivot_col] = -value / aug_rows[pivot_row][pivot_col]
        null_space.append(basis)

    return {"particular": particular, "null_space": null_space}


def find_positive_alpha(beta0, null_vecs):
    beta_len = len(beta0)
    null_dim = len(null_vecs)
    constraints = [
        ([null_vecs[col][row] for col in range(null_dim)], -beta0[row])
        for row in range(beta_len)
    ]
    stages = []

    for elim_idx in range(null_dim - 1, -1, -1):
        bounds = []
        positive = []
        negative = []
        new_constraints = []

        for coeffs, rhs in constraints:
            coeff = coeffs[elim_idx]
            if coeff > 0:
                positive.append((coeffs, rhs))
            elif coeff < 0:
                negative.append((coeffs, rhs))
            else:
                reduced = coeffs[:elim_idx] + coeffs[elim_idx + 1 :]
                new_constraints.append((reduced, rhs))

        for coeffs, rhs in positive + negative:
            divisor = coeffs[elim_idx]
            remaining = coeffs[:elim_idx] + coeffs[elim_idx + 1 :]
            bounds.append(
                {
                    "remaining_coeffs": remaining,
                    "rhs": rhs,
                    "divisor": divisor,
                }
            )
        stages.append(bounds)

        for lower_coeffs, lower_rhs in positive:
            a_lower = lower_coeffs[elim_idx]
            for upper_coeffs, upper_rhs in negative:
                a_upper = upper_coeffs[elim_idx]
                reduced = []
                for idx in range(len(lower_coeffs)):
                    if idx == elim_idx:
                        continue
                    reduced.append(
                        a_lower * upper_coeffs[idx] - a_upper * lower_coeffs[idx]
                    )
                rhs = a_lower * upper_rhs - a_upper * lower_rhs
                new_constraints.append((reduced, rhs))

        constraints = new_constraints

    alpha = [beta0[0].parent().zero() for _ in range(null_dim)]
    one = beta0[0].parent().one()

    for stage_idx, bounds in enumerate(stages):
        alpha_idx = null_dim - 1 - stage_idx
        lower = None
        upper = None

        for bound in bounds:
            residual = bound["rhs"]
            for coeff, assigned in zip(bound["remaining_coeffs"], alpha[:alpha_idx]):
                residual -= coeff * assigned
            candidate = residual / bound["divisor"]
            if bound["divisor"] > 0:
                lower = candidate if lower is None else max(lower, candidate)
            else:
                upper = candidate if upper is None else min(upper, candidate)

        if lower is not None and upper is not None and lower > upper:
            return None

        if lower is not None and upper is not None:
            alpha[alpha_idx] = (lower + upper) / 2
        elif lower is not None:
            alpha[alpha_idx] = lower + one
        elif upper is not None:
            alpha[alpha_idx] = upper - one
        else:
            alpha[alpha_idx] = beta0[0].parent().zero()

    beta = []
    for row in range(beta_len):
        value = beta0[row]
        for col in range(null_dim):
            value += alpha[col] * null_vecs[col][row]
        beta.append(value)

    if all(entry > 0 for entry in beta):
        return alpha
    return None


def choose_positive_solution(particular, null_space, beta_len):
    if not null_space:
        if all(entry > 0 for entry in particular[:beta_len]):
            return particular
        return None

    beta0 = particular[:beta_len]
    null_beta = [basis[:beta_len] for basis in null_space]
    alpha = find_positive_alpha(beta0, null_beta)
    if alpha is None:
        return None

    solution = list(particular)
    for basis_idx, basis in enumerate(null_space):
        for row in range(len(solution)):
            solution[row] += alpha[basis_idx] * basis[row]
    return solution


def solve_sigma(context: FieldContext, dual_vertices, sigma):
    matrix, rhs = build_kkt_matrix(dual_vertices, sigma)
    linear_data = gauss_solve_with_null_space(matrix, rhs)
    if linear_data is None:
        return None

    solution = choose_positive_solution(
        linear_data["particular"], linear_data["null_space"], len(sigma)
    )
    if solution is None:
        return None

    beta = list(solution[: len(sigma)])
    if any(not context.beta_is_admissible(entry) for entry in beta):
        return None

    q = matrix.base_ring().zero()
    for i in range(1, len(beta)):
        for j in range(i):
            q += beta[i] * beta[j] * omega0(dual_vertices[sigma[j]], dual_vertices[sigma[i]])
    if not context.q_is_admissible(q):
        return None

    return {
        "beta": beta,
        "q": q,
        "mu": solution[len(sigma) : len(sigma) + 4],
        "xi": solution[len(sigma) + 4],
    }


def cyclic_permutations_of_subset(subset):
    first = subset[0]
    tail = subset[1:]
    for perm in itertools.permutations(tail):
        yield (first,) + perm


def enumerate_sigmas(facet_count):
    indices = range(facet_count)
    for orbit_len in range(2, facet_count + 1):
        for subset in itertools.combinations(indices, orbit_len):
            yield from cyclic_permutations_of_subset(subset)


def minimizer_equal(scalar_mode: str, left, right) -> bool:
    if scalar_mode == RDF_MODE:
        return abs(float(left - right)) <= 1.0e-12
    return left == right


def reconstruct_dual_vertices(context: FieldContext, row: dict):
    return [
        vector(context.field, [context.scalar(coord_json) for coord_json in vertex_json])
        for vertex_json in row["dual_vertices"]
    ]


def scalar_modes_for_row(row: dict) -> list[str]:
    if row["exact_field"] == "rational":
        return [RDF_MODE, RATIONAL_EXACT_MODE]
    return [RDF_MODE, ALGEBRAIC_EXACT_MODE]


def run_search(row: dict, scalar_mode: str, canonical: bool) -> dict:
    context = FieldContext(row["exact_field"], scalar_mode)
    dual_vertices = reconstruct_dual_vertices(context, row)
    timeout_s = timeout_for(canonical, scalar_mode)
    start_time = time.perf_counter()
    deadline = start_time + timeout_s
    field = dual_vertices[0].base_ring()

    sigma_count_total = 0
    sigma_count_admissible = 0
    best_action = None
    best_sigma = None
    minimizers = []
    timeout = False

    for sigma in enumerate_sigmas(len(dual_vertices)):
        if time.perf_counter() > deadline:
            timeout = True
            break
        sigma_count_total += 1

        orbit = solve_sigma(context, dual_vertices, sigma)
        if orbit is None:
            continue

        sigma_count_admissible += 1
        action = field.one() / (field(2) * orbit["q"])
        if best_action is None or action < best_action:
            best_action = action
            best_sigma = list(sigma)
            minimizers = [list(sigma)]
        elif minimizer_equal(scalar_mode, action, best_action):
            minimizers.append(list(sigma))

    wall_time_ms = (time.perf_counter() - start_time) * 1000.0
    status = "timed_out" if timeout else "completed"
    result = {
        "polytope": row["polytope"],
        "family": row["family"],
        "facet_count": row["facet_count"],
        "exact_field": row["exact_field"],
        "scalar_mode": scalar_mode,
        "status": status,
        "timeout_s": timeout_s,
        "sigma_count_total": sigma_count_total,
        "sigma_count_admissible": sigma_count_admissible,
        "sage_representative_sigma": best_sigma,
        "sage_minimizer_representative_count": len(minimizers),
        "representative_note": (
            "Sage counts cyclic-permutation representatives in its own search order; "
            "these fields are diagnostic and not normalized to Rust collector semantics."
        ),
        "best_action_so_far": None if best_action is None else context.as_float(best_action),
        "capacity": None if timeout or best_action is None else context.as_float(best_action),
        "wall_time_ms": wall_time_ms,
        "rust_f64_capacity": row["rust_f64_capacity"],
        "rust_f64_iterations": row["rust_f64_iterations"],
        "rust_f64_returned_orbit_count": row["rust_f64_returned_orbit_count"],
        "rust_f64_representative_sigma": row["rust_f64_best_sigma"],
        "rust_f64_wall_time_ms": row["rust_f64_wall_time_ms"],
        "capacity_abs_diff_vs_rust": None,
    }
    if result["capacity"] is not None:
        result["capacity_abs_diff_vs_rust"] = abs(
            result["capacity"] - row["rust_f64_capacity"]
        )
    return result


def analyze_rows(rows: list[dict], canonical: bool) -> list[dict]:
    out = []
    for row in rows:
        for scalar_mode in scalar_modes_for_row(row):
            out.append(run_search(row, scalar_mode, canonical))
    return out


def main() -> None:
    args = parse_args()
    canonical = args.canonical and not args.smoke
    rows = read_jsonl(input_path(canonical))
    report_rows = analyze_rows(rows, canonical)
    write_jsonl(output_path(canonical), report_rows)
    print(
        f"wrote {len(report_rows)} Sage-feasibility rows to {output_path(canonical)}"
    )


if __name__ == "__main__":
    main()
