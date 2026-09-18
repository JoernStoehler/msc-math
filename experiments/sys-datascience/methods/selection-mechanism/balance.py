"""Freeze target-free matched products before covariance balancing evaluation."""
import argparse
from collections import defaultdict
from datetime import datetime, timezone
from fractions import Fraction
import hashlib
import itertools
import json
from pathlib import Path
import subprocess
import time

import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SOURCE = Path.home() / '.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl'
SELECTION_SALT = 'covariance-balance-v1|'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exact_vertices(dual):
    """Exact 2D halfplane intersection for the actual binary64 coefficients."""
    rows = [tuple(Fraction.from_float(float(x)) for x in row) for row in dual]
    vertices = set()
    for a, b in itertools.combinations(rows, 2):
        det = a[0] * b[1] - a[1] * b[0]
        if not det:
            continue
        v = ((b[1] - a[1]) / det, (a[0] - b[0]) / det)
        if all(d[0] * v[0] + d[1] * v[1] <= 1 for d in rows):
            vertices.add(v)
    vertices = list(vertices)
    center = np.mean([[float(x) for x in v] for v in vertices], axis=0)
    vertices.sort(key=lambda v: np.arctan2(float(v[1]) - center[1], float(v[0]) - center[0]))
    area = sum(a[0] * b[1] - a[1] * b[0] for a, b in zip(vertices, vertices[1:] + vertices[:1])) / 2
    assert area > 0
    return np.asarray([[float(x) for x in v] for v in vertices]), float(area)


def covariance(vertices):
    centered = vertices - np.mean(vertices, axis=0)
    return centered.T @ centered / len(vertices)


def whitening(vertices):
    cov = covariance(vertices)
    eig, vectors = np.linalg.eigh(cov)
    assert eig[0] > 0
    transform = (vectors * (np.linalg.det(cov) ** .25 / np.sqrt(eig))) @ vectors.T
    assert abs(np.linalg.det(transform) - 1) < 1e-9
    return transform


def features(q, p, aq, ap):
    a = np.roll(q, -1, axis=0) - q
    b = np.roll(p, -1, axis=0) - p
    ridge = float(np.abs(a @ b.T).sum() / np.sqrt(aq * ap))
    A, B = covariance(q), covariance(p)
    ae, av = np.linalg.eigh(A)
    sqrtA = (av * np.sqrt(ae)) @ av.T
    eigen = np.linalg.eigvalsh(sqrtA @ B @ sqrtA)
    rho = float(np.sqrt(eigen[1] / eigen[0]))
    return dict(R=ridge, rho=rho, q_area=aq, p_area=ap, volume=aq * ap,
                q_covariance=A.tolist(), p_covariance=B.tolist())


def write_jsonl(path, rows):
    with path.open('x') as output:
        for row in rows:
            output.write(json.dumps(row, allow_nan=False) + '\n')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path, default=SOURCE)
    parser.add_argument('--out', type=Path, default=HERE / 'balance-artifacts')
    args = parser.parse_args()
    start = time.monotonic()
    args.out.mkdir(parents=True, exist_ok=True)
    if (args.out / 'freeze.json').exists():
        raise SystemExit('frozen experiment already exists; use a new output directory')
    groups = defaultdict(list)
    for line in args.source.open():
        row = json.loads(line)
        # Selection uses names/group only, never historical target or features.
        key = hashlib.sha256((SELECTION_SALT + row['name']).encode()).hexdigest()
        groups[(row['k'], row['m'])].append((key, row))
    assert len(groups) == 10 and all(len(rows) == 1024 for rows in groups.values())
    selected = {group: sorted(rows, key=lambda item: item[0])[:32] for group, rows in groups.items()}
    requests, geometry = [], []
    for index in range(32):
        for (k, m), selected_rows in sorted(selected.items()):
            selection_hash, source = selected_rows[index]
            dual = np.asarray(source['dual_vertices'])
            assert dual.shape == (k + m, 4)
            assert np.all(dual[:k, 2:] == 0) and np.all(dual[k:, :2] == 0)
            q, aq = exact_vertices(dual[:k, :2])
            p, ap = exact_vertices(dual[k:, 2:])
            assert len(q) == k and len(p) == m
            L, M = whitening(q), whitening(p)
            balanced = dual.copy()
            balanced[:k, :2] = dual[:k, :2] @ np.linalg.inv(L)
            balanced[k:, 2:] = dual[k:, 2:] @ np.linalg.inv(M)
            bq, baq = exact_vertices(balanced[:k, :2])
            bp, bap = exact_vertices(balanced[k:, 2:])
            assert len(bq) == k and len(bp) == m
            area_error = max(abs(baq / aq - 1), abs(bap / ap - 1))
            assert area_error < 1e-9
            old_features = features(q, p, aq, ap)
            new_features = features(bq, bp, baq, bap)
            assert abs(new_features['rho'] - 1) < 1e-8
            for arm, actual_dual, feat in [('baseline', dual, old_features), ('balanced', balanced, new_features)]:
                ident = f"{source['name']}:{arm}"
                requests.append(dict(id=ident, source_name=source['name'], group=f'{k}x{m}', pair_index=index,
                    arm=arm, selection_hash=selection_hash, dual_vertices=actual_dual.tolist()))
                geometry.append(dict(id=ident, source_name=source['name'], group=f'{k}x{m}', pair_index=index,
                    arm=arm, **feat, q_transform=L.tolist(), p_transform=M.tolist(),
                    max_relative_area_change=area_error))
    assert len(requests) == 640
    write_jsonl(args.out / 'inputs.jsonl', requests)
    # First matched pair in every bucket: 10 original bodies, 20 target requests.
    write_jsonl(args.out / 'pilot-inputs.jsonl', requests[:20])
    write_jsonl(args.out / 'remaining-inputs.jsonl', requests[20:])
    write_jsonl(args.out / 'geometry.jsonl', geometry)
    freeze = dict(frozen_utc=datetime.now(timezone.utc).isoformat(), head=subprocess.check_output(['git','rev-parse','HEAD'], cwd=ROOT, text=True).strip(),
        source_sha256=digest(args.source), script_sha256=digest(Path(__file__)),
        input_sha256=digest(args.out / 'inputs.jsonl'), pilot_input_sha256=digest(args.out / 'pilot-inputs.jsonl'),
        remaining_input_sha256=digest(args.out / 'remaining-inputs.jsonl'), geometry_sha256=digest(args.out / 'geometry.jsonl'),
        selection_salt=SELECTION_SALT, per_group=32, pairs=320, requests=640,
        construction_wall_seconds=time.monotonic() - start,
        primary_question='Does removing vertex-covariance anisotropy of both planar factors weaken the within-group R/sys correlation and change paired sys?',
        intervention='For each factor with centered uniform-vertex covariance A, apply the SPD determinant-one map det(A)^(1/4) A^(-1/2). Reconstruct actual rounded binary64 dual geometry exactly.',
        primary_statistics=['within-group rank correlation before/after balancing', 'paired sys and R effects with sign counts, means and ranges'],
        limits='This operation changes shapes and capacity; it is not generally symplectic. It tests a specified intervention, not universal mediation, local optimality or all alternative geometric mechanisms.',
        evaluation_contract='Current scalar product evaluator, one serial worker, 5 seconds per input, 300-second total cap; pilot 20 requests then remaining620 if runtime fits. Retain all errors/timeouts. No replacements.')
    (args.out / 'freeze.json').write_text(json.dumps(freeze, indent=2) + '\n')
    print(json.dumps(freeze, indent=2))


if __name__ == '__main__':
    main()
