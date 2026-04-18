#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""
Goal: verify the exact symbolic simplifications used for the active 2-bounce
branch and the first two competitive 3-bounce branches in the pentagon-rotation
proof draft.
Input Artifacts:
  - None
Output Artifacts:
  - None
"""

from dataclasses import dataclass
from typing import Callable

from sympy import Matrix, cos, pi, simplify, sin, sqrt, symbols, tan


def two_bounce(theta):
    amplitude = (5 + sqrt(5)) / 4
    edge_height = sin(pi / 5)
    lam_theta = 1 / 2 - tan(theta) / (2 * tan(pi / 10))
    area = 5 * sin(2 * pi / 5) / 2

    switch_residual = simplify(
        sin(theta) * (amplitude - edge_height / tan(pi / 10))
    )
    left_slope = simplify(
        2 * edge_height * (sin(theta + 6 * pi / 5) - sin(theta))
    )
    right_slope = simplify(
        2 * edge_height * (sin(theta + 4 * pi / 5) - sin(theta))
    )
    left_factor = -sqrt(5) * sin(theta + pi / 10)
    right_factor = sqrt(5) * cos(theta + 2 * pi / 5)

    d_theta = Matrix([-amplitude, -amplitude * tan(theta)])
    rotation_w0 = Matrix([cos(theta), sin(theta)])
    rotation_w2 = Matrix(
        [
            cos(theta + 4 * pi / 5),
            sin(theta + 4 * pi / 5),
        ]
    )
    h_minus = simplify((-d_theta).dot(rotation_w0))
    h_plus = simplify(d_theta.dot(rotation_w2))
    capacity = simplify(h_minus + h_plus)
    sys_prefactor = simplify(capacity.subs(theta, 0) ** 2 / (2 * area**2))

    assert simplify(amplitude - edge_height / tan(pi / 10)) == 0
    assert switch_residual == 0
    assert simplify(
        simplify((left_slope - left_factor) / sin(theta))
    ) == 0
    assert simplify(
        simplify((right_slope - right_factor) / sin(theta))
    ) == 0
    assert simplify(h_minus - amplitude / cos(theta)) == 0
    assert simplify(h_plus - amplitude * cos(pi / 5) / cos(theta)) == 0
    assert simplify(capacity - amplitude**2 / cos(theta)) == 0
    assert simplify(sys_prefactor - (5 + 2 * sqrt(5)) / 10) == 0

    return {
        "lambda(theta)": 1 / 2 - tan(theta) / (2 * tan(pi / 10)),
        "left_slope(theta)": left_factor,
        "right_slope(theta)": right_factor,
        "support_minus(theta)": amplitude / cos(theta),
        "support_plus(theta)": amplitude * cos(pi / 5) / cos(theta),
        "capacity(theta)": amplitude**2 / cos(theta),
        "sys_prefactor": (5 + 2 * sqrt(5)) / 10,
    }


def first_family(theta):
    amplitude = (5 + sqrt(5)) / 4
    a_theta = sqrt(5) / (2 * sin(theta + 3 * pi / 10))
    b_theta = (amplitude - a_theta * cos(theta + pi / 5)) / sin(
        theta + 3 * pi / 10
    )
    action = simplify(amplitude * a_theta + sqrt(5) * b_theta / 2)
    gap = (
        5
        * sin(theta + pi / 10)
        * sin(pi / 10 - theta)
        / (4 * sin(theta + 3 * pi / 10) ** 2 * cos(theta))
    )
    return action, gap


def second_family(theta):
    amplitude = (5 + sqrt(5)) / 4
    a_theta = sqrt(5) / (2 * sin(3 * pi / 10 - theta))
    b_theta = (amplitude - a_theta * sin(theta + 3 * pi / 10)) / cos(
        theta + pi / 5
    )
    action = simplify(amplitude * a_theta + sqrt(5) * b_theta / 2)
    gap = (
        5
        * sin(theta + pi / 10)
        * sin(pi / 10 - theta)
        / (4 * cos(theta) * cos(theta + pi / 5) ** 2)
    )
    return action, gap


@dataclass(frozen=True)
class ThreeBounceWitness:
    name: str
    template: str
    signature: str
    builder: Callable


THREE_BOUNCE_WITNESSES = [
    ThreeBounceWitness(
        name="first_family",
        template="EEV/EEV",
        signature="Q:0-1-23|P:2-3-01",
        builder=first_family,
    ),
    ThreeBounceWitness(
        name="second_family",
        template="EEV/EEV",
        signature="Q:0-1-34|P:3-4-01",
        builder=second_family,
    ),
]


def main():
    theta = symbols("theta", real=True)
    baseline = ((5 + sqrt(5)) / 4) ** 2 / cos(theta)

    two_bounce_data = two_bounce(theta)
    three_bounce_outputs = []
    for witness in THREE_BOUNCE_WITNESSES:
        action, gap = witness.builder(theta)
        assert simplify(action - (baseline + gap)) == 0
        three_bounce_outputs.append((witness, action, gap))

    print("Verified symbolic simplifications for the active 2-bounce branch and")
    print("the descriptor-listed competitive 3-bounce branches.")
    print("This script checks exact symbolic identities; interval positivity is")
    print("a separate claim unless encoded explicitly in the asserted formula.")
    print()
    print(f"lambda(theta)      = {two_bounce_data['lambda(theta)']}")
    print(f"left_slope(theta)  = {two_bounce_data['left_slope(theta)']}")
    print(f"right_slope(theta) = {two_bounce_data['right_slope(theta)']}")
    print(f"support_minus(theta) = {two_bounce_data['support_minus(theta)']}")
    print(f"support_plus(theta)  = {two_bounce_data['support_plus(theta)']}")
    print(f"capacity(theta)    = {two_bounce_data['capacity(theta)']}")
    print(f"sys_prefactor      = {two_bounce_data['sys_prefactor']}")
    print()
    for witness, action, gap in three_bounce_outputs:
        print(f"{witness.name} [{witness.template}] {witness.signature}")
        print(f"  action(theta) = {action}")
        print(f"  gap(theta)    = {gap}")
        print()


if __name__ == "__main__":
    main()
