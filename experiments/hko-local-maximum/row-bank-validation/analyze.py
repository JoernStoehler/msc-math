"""
Goal: independently recompute the selected exact-bank rows in SageMath and
compare them against the Rust exact export.
Input Artifacts: experiments/hko-local-maximum/row-bank-validation/row-bank-validation-input.jsonl
Output Artifacts: experiments/hko-local-maximum/row-bank-validation/row-bank-validation-report.jsonl
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

try:
    from sage.all import QQ, RR, Matrix, NumberField, PolynomialRing, vector, tan, pi
except ModuleNotFoundError as exc:
    raise SystemExit("run this script with `sage -python analyze.py`") from exc


EXPERIMENT_DIR = Path(__file__).resolve().parent
INPUT_FILENAME = "row-bank-validation-input.jsonl"
SMOKE_INPUT_FILENAME = "smoke-row-bank-validation-input.jsonl"
OUTPUT_FILENAME = "row-bank-validation-report.jsonl"
SMOKE_OUTPUT_FILENAME = "smoke-row-bank-validation-report.jsonl"


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


def read_jsonl(path: Path) -> list[dict]:
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def write_jsonl(path: Path, rows: list[dict]) -> None:
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row))
            handle.write("\n")


class FieldContext:
    def __init__(self, field_tag: str):
        self.field_tag = field_tag
        self.phi = None
        if field_tag == "rational":
            self.field = QQ
            self.gen = None
            self.phi = RR
        elif field_tag == "q_tan_pi_fifth":
            poly_ring = PolynomialRing(QQ, names="x")
            x = poly_ring.gen()
            poly = x**4 - 10 * x**2 + 5
            self.field = NumberField(poly, names="t", embedding=RR(tan(pi / 5)))
            self.gen = self.field.gen()
            self.phi = next(
                embedding
                for embedding in self.field.embeddings(RR)
                if 0 < embedding(self.gen) < 1
            )
        else:
            raise ValueError(f"unsupported field tag: {field_tag}")

    def scalar(self, element_json: dict):
        coeffs = [decode_rational(coeff_json) for coeff_json in element_json["coeffs"]]
        if self.field_tag == "rational":
            if len(coeffs) != 1:
                raise ValueError("rational element must have one coefficient")
            return self.field(coeffs[0])
        return self.field(coeffs)

    def as_float(self, value) -> float:
        if self.field_tag == "rational":
            return float(RR(value))
        return float(self.phi(value))

    @staticmethod
    def is_positive(value) -> bool:
        return value > 0

    @staticmethod
    def is_nonpositive(value) -> bool:
        return value <= 0


def decode_rational(coeff_json: dict):
    return QQ(coeff_json["numer"]) / QQ(coeff_json["denom"])


def omega0(left, right):
    return left[0] * right[2] - right[0] * left[2] + left[1] * right[3] - right[1] * left[3]


def apply_j0(value):
    return vector(value.base_ring(), [-value[2], -value[3], value[0], value[1]])


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


def choose_positive_solution(context: FieldContext, particular, kernel_basis, beta_len: int):
    if not kernel_basis:
        if all(context.is_positive(entry) for entry in particular[:beta_len]):
            return particular, 0
        return None, 0
    if len(kernel_basis) != 1:
        raise ValueError(
            f"unsupported kernel dimension {len(kernel_basis)}; "
            "current packet supports 0 or 1"
        )

    direction = kernel_basis[0]
    lower = None
    upper = None
    field = particular.base_ring()

    for base, delta in zip(particular[:beta_len], direction[:beta_len]):
        if delta == 0:
            if context.is_nonpositive(base):
                return None, 1
            continue

        bound = -base / delta
        if context.is_positive(delta):
            if lower is None or bound > lower:
                lower = bound
        else:
            if upper is None or bound < upper:
                upper = bound

    if lower is not None and upper is not None and upper <= lower:
        return None, 1

    if lower is None and upper is None:
        alpha = field.zero()
    elif lower is None:
        alpha = upper - field.one()
    elif upper is None:
        alpha = lower + field.one()
    else:
        alpha = (lower + upper) / field(2)

    solution = particular + alpha * direction
    if not all(context.is_positive(entry) for entry in solution[:beta_len]):
        return None, 1
    return solution, 1


def solve_sigma(context: FieldContext, dual_vertices, sigma):
    matrix, rhs = build_kkt_matrix(dual_vertices, sigma)
    try:
        particular = matrix.solve_right(rhs)
    except ValueError:
        return None

    kernel_basis = list(matrix.right_kernel().basis())
    solution, kernel_dim = choose_positive_solution(context, particular, kernel_basis, len(sigma))
    if solution is None:
        return None

    beta = list(solution[: len(sigma)])
    mu = vector(solution.base_ring(), list(solution[len(sigma) : len(sigma) + 4]))
    q = solution.base_ring().zero()
    for i in range(1, len(beta)):
        for j in range(i):
            q += beta[i] * beta[j] * omega0(dual_vertices[sigma[j]], dual_vertices[sigma[i]])

    return {
        "beta": beta,
        "mu": mu,
        "q": q,
        "xi": solution[len(sigma) + 4],
        "kernel_dim": kernel_dim,
    }


def capacity_gradient(dual_vertices, sigma, orbit):
    field = dual_vertices[0].base_ring()
    q_sq = orbit["q"] * orbit["q"]
    scale = -(field.one() / (field(2) * q_sq))
    zero = vector(field, [field.zero(), field.zero(), field.zero(), field.zero()])
    sigma_positions = {facet: idx for idx, facet in enumerate(sigma)}
    out = []

    for facet in range(len(dual_vertices)):
        if facet not in sigma_positions:
            out.append(zero)
            continue

        i0 = sigma_positions[facet]
        p = zero
        for i in range(i0):
            p += orbit["beta"][i] * dual_vertices[sigma[i]]
        inner = field(2) * p + orbit["beta"][i0] * dual_vertices[facet]
        dq_da = orbit["beta"][i0] * (apply_j0(inner) + orbit["mu"])
        out.append(scale * dq_da)
    return out

def max_abs_scalar_diff(context: FieldContext, left, right) -> float:
    return abs(context.as_float(left - right))


def require_same_length(name: str, left, right) -> None:
    if len(left) != len(right):
        raise ValueError(f"{name} length mismatch: {len(left)} != {len(right)}")


def require_same_matrix_shape(name: str, left, right) -> None:
    require_same_length(f"{name} row count", left, right)
    for index, (left_row, right_row) in enumerate(zip(left, right)):
        if len(left_row) != len(right_row):
            raise ValueError(
                f"{name} column count mismatch at row {index}: "
                f"{len(left_row)} != {len(right_row)}"
            )


def max_abs_vector_diff(context: FieldContext, left, right) -> float:
    require_same_matrix_shape("capacity_gradient", left, right)
    values = [
        abs(context.as_float(l_entry - r_entry))
        for l_row, r_row in zip(left, right)
        for l_entry, r_entry in zip(l_row, r_row)
    ]
    return max(values, default=0.0)


def reconstruct_row(row: dict):
    context = FieldContext(row["exact_field"])
    dual_vertices = [
        vector(context.field, [context.scalar(coord_json) for coord_json in vertex_json])
        for vertex_json in row["dual_vertices"]
    ]
    rust_q = context.scalar(row["rust_q"])
    rust_action = context.scalar(row["rust_action"])
    rust_beta = [context.scalar(item) for item in row["rust_beta"]]
    rust_gradient = [
        [context.scalar(coord_json) for coord_json in grad_row]
        for grad_row in row["rust_capacity_gradient_a"]
    ]
    return context, dual_vertices, rust_q, rust_action, rust_beta, rust_gradient


def validate_row(row: dict) -> dict:
    context, dual_vertices, rust_q, rust_action, rust_beta, rust_gradient = reconstruct_row(row)
    sigma = row["sigma"]

    orbit = solve_sigma(context, dual_vertices, sigma)
    if orbit is None:
        return {
            "row_name": row["row_name"],
            "polytope": row["polytope"],
            "exact_field": row["exact_field"],
            "sigma_label": row["sigma_label"],
            "sigma": sigma,
            "sage_status": "unsolved",
        }

    gradient = capacity_gradient(dual_vertices, sigma, orbit)
    sage_action = orbit["q"].parent()(1) / (orbit["q"].parent()(2) * orbit["q"])

    sage_beta = orbit["beta"]
    sage_gradient = [list(grad) for grad in gradient]
    require_same_length("beta", sage_beta, rust_beta)
    require_same_matrix_shape("capacity_gradient", sage_gradient, rust_gradient)

    q_match = orbit["q"] == rust_q
    action_match = sage_action == rust_action
    beta_match = all(left == right for left, right in zip(sage_beta, rust_beta))
    gradient_match = all(
        left == right
        for left_row, right_row in zip(sage_gradient, rust_gradient)
        for left, right in zip(left_row, right_row)
    )

    return {
        "row_name": row["row_name"],
        "polytope": row["polytope"],
        "exact_field": row["exact_field"],
        "sigma_label": row["sigma_label"],
        "sigma": sigma,
        "sage_status": "solved",
        "sage_kernel_dim": orbit["kernel_dim"],
        "exact_q_match": q_match,
        "exact_action_match": action_match,
        "exact_beta_match": beta_match,
        "exact_capacity_gradient_match": gradient_match,
        "max_abs_q_diff": max_abs_scalar_diff(context, orbit["q"], rust_q),
        "max_abs_action_diff": max_abs_scalar_diff(context, sage_action, rust_action),
        "max_abs_beta_diff": max(
            (
                abs(context.as_float(left - right))
                for left, right in zip(sage_beta, rust_beta)
            ),
            default=0.0,
        ),
        "max_abs_capacity_gradient_diff": max_abs_vector_diff(
            context, sage_gradient, rust_gradient
        ),
    }


def main() -> None:
    args = parse_args()
    canonical = args.canonical and not args.smoke
    rows = read_jsonl(input_path(canonical))
    report_rows = [validate_row(row) for row in rows]
    write_jsonl(output_path(canonical), report_rows)

    solved = sum(row.get("sage_status") == "solved" for row in report_rows)
    print(
        f"wrote {len(report_rows)} Sage validation rows to {output_path(canonical)} "
        f"({solved} solved)"
    )


if __name__ == "__main__":
    main()
