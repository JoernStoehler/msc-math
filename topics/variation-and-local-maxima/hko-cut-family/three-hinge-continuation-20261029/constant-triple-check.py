"""Exact replay of a zero-slope triple of constant characteristic hinges."""
import itertools
import json
import sys
from fractions import Fraction
from pathlib import Path

CERT = Path(__file__).resolve().parents[4] / "topics/capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent"
sys.path.insert(0, str(CERT))
import exact_core as E

primitive = json.loads((CERT / "expected/primitives.json").read_text())
catalogue = json.loads((CERT / "expected/catalogue.json").read_text())


def decode(coefficients):
    return E.K([Fraction(c) for c in coefficients])


vertices = []
for key in ("triangles_q", "triangles_p"):
    rows = []
    for labels, weights in primitive[key]:
        beta = [E.ZERO] * 10
        for label, weight in zip(labels, weights):
            beta[label] = decode(weight) / 2
        rows.append(beta)
    vertices.append(rows)

labels = (0, 1, 2)
constants = ((5 - E.SQRT5) / 10, E.SQRT5 / 5, (5 - E.SQRT5) / 10)
best = E.ZERO
witness = None
for chart_index, chart in enumerate(catalogue["charts"]):
    qvertices = [i for i in range(5) if chart["q_vertices_mask"] & (1 << i)]
    pvertices = [i for i in range(5) if chart["p_vertices_mask"] & (1 << i)]
    word = chart["sigma"]
    for qi, pi in itertools.product(qvertices, pvertices):
        beta = [vertices[0][qi][i] + vertices[1][pi][i] for i in word]
        response = sum((beta[j] * constants[labels.index(label)]
                        for j, label in enumerate(word) if label in labels), E.ZERO)
        if E.sign(response - best) > 0:
            best = response
            witness = {"chart_index": chart_index, "word": word,
                       "q_vertex_index": qi, "p_vertex_index": pi}

volume = sum(constants, E.ZERO) / 10
gap = best - volume
assert best == E.K(Fraction(1, 10))
assert volume == E.K(Fraction(1, 10))
assert gap == E.ZERO
print(json.dumps({
    "field_coefficients_ascending": [c.coefficients() for c in constants],
    "constant_profiles_numeric": [E.nominal(c) for c in constants],
    "maximum_response_coefficients_ascending": best.coefficients(),
    "maximum_response_numeric": E.nominal(best),
    "volume_response_coefficients_ascending": volume.coefficients(),
    "volume_response_numeric": E.nominal(volume),
    "exact_gap_coefficients_ascending": gap.coefficients(),
    "attaining_chart_witness": witness,
}, indent=2))
