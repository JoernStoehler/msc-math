#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""
Build an interactive HTML viewer for the sampled minimizing pentagon-rotation
orbits.

Input artifact:
  - theta-sweep.jsonl

Output artifacts:
  - minimum_orbit_projection_dataset.jsonl
  - minimum_orbit_projection_viewer.html

This is a thesis-illustration artifact. It is not proof input; the exact proof
is `executable_proof.sage.py`.
"""

from __future__ import annotations

import html
import json
import math
from pathlib import Path

import numpy as np


EXPERIMENT_DIR = Path(__file__).resolve().parent
INPUT_PATH = EXPERIMENT_DIR / "theta-sweep.jsonl"
DATASET_PATH = EXPERIMENT_DIR / "minimum_orbit_projection_dataset.jsonl"
HTML_PATH = EXPERIMENT_DIR / "minimum_orbit_projection_viewer.html"


def load_jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def regular_pentagon_normals_and_heights() -> tuple[np.ndarray, np.ndarray]:
    n = 5
    inradius = math.cos(math.pi / n)
    normals = np.array(
        [
            [
                math.cos(math.pi / 2.0 + 2.0 * math.pi * k / n),
                math.sin(math.pi / 2.0 + 2.0 * math.pi * k / n),
            ]
            for k in range(n)
        ],
        dtype=float,
    )
    heights = np.full(n, inradius, dtype=float)
    return normals, heights


def polygon_vertices_from_hrep(normals: np.ndarray, heights: np.ndarray) -> np.ndarray:
    vertices = []
    n = len(normals)
    for i in range(n):
        j = (i + 1) % n
        mat = np.array([normals[i], normals[j]], dtype=float)
        rhs = np.array([heights[i], heights[j]], dtype=float)
        vertices.append(np.linalg.solve(mat, rhs))
    return np.array(vertices, dtype=float)


def rotate_points(points: np.ndarray, theta: float) -> np.ndarray:
    c, s = math.cos(theta), math.sin(theta)
    rot = np.array([[c, -s], [s, c]], dtype=float)
    return points @ rot.T


def dual_vertices_for_theta(theta_deg: float) -> np.ndarray:
    normals_q, heights_q = regular_pentagon_normals_and_heights()
    theta = math.radians(theta_deg)
    normals_p = rotate_points(normals_q, theta)
    dual_q = np.column_stack(
        [
            normals_q[:, 0] / heights_q,
            normals_q[:, 1] / heights_q,
            np.zeros(5),
            np.zeros(5),
        ]
    )
    dual_p = np.column_stack(
        [
            np.zeros(5),
            np.zeros(5),
            normals_p[:, 0] / heights_q,
            normals_p[:, 1] / heights_q,
        ]
    )
    return np.vstack([dual_q, dual_p])


def recover_breakpoints(orbit: dict, theta_deg: float) -> np.ndarray:
    duals = dual_vertices_for_theta(theta_deg)
    sigma = orbit["sigma"]
    beta = np.array(orbit["beta"], dtype=float)
    capacity = float(orbit["action"])
    dwell_times = capacity * beta
    reeb_vectors = np.array(
        [
            2.0
            * np.array(
                [
                    -duals[facet, 2],
                    -duals[facet, 3],
                    duals[facet, 0],
                    duals[facet, 1],
                ],
                dtype=float,
            )
            for facet in sigma
        ]
    )

    displacements = [np.zeros(4, dtype=float)]
    for tau, reeb in zip(dwell_times, reeb_vectors):
        displacements.append(displacements[-1] + tau * reeb)
    displacements = np.array(displacements, dtype=float)

    mat = np.array([duals[facet] for facet in sigma], dtype=float)
    rhs = np.array(
        [1.0 - duals[facet].dot(displacements[k]) for k, facet in enumerate(sigma)],
        dtype=float,
    )
    base_point, *_ = np.linalg.lstsq(mat, rhs, rcond=None)
    breakpoints = base_point + displacements

    max_violation = max(
        float(dual.dot(point) - 1.0) for point in breakpoints for dual in duals
    )
    closure_error = float(np.linalg.norm(breakpoints[-1] - breakpoints[0]))
    if max_violation > 1e-7 or closure_error > 1e-7:
        raise RuntimeError(
            f"recovered orbit failed validation: max_violation={max_violation:.2e}, "
            f"closure_error={closure_error:.2e}"
        )
    return breakpoints


def segment_motion_type(start: np.ndarray, end: np.ndarray, tol: float = 1e-9) -> str:
    dq = np.linalg.norm(end[:2] - start[:2])
    dp = np.linalg.norm(end[2:] - start[2:])
    if dq <= tol and dp > tol:
        return "Q"
    if dp <= tol and dq > tol:
        return "P"
    raise RuntimeError(f"unexpected segment type: dq={dq:.3e}, dp={dp:.3e}")


def compress_affine_cycle(breakpoints: np.ndarray) -> np.ndarray:
    segment_types = [
        segment_motion_type(start, end)
        for start, end in zip(breakpoints[:-1], breakpoints[1:])
    ]
    affine_points = [breakpoints[0]]
    current_type = segment_types[0]
    for index, segment_type in enumerate(segment_types):
        if segment_type != current_type:
            affine_points.append(breakpoints[index])
            current_type = segment_type
    affine_points.append(breakpoints[-1])
    return np.array(affine_points, dtype=float)


def normalize_block(block: dict) -> tuple:
    if block["kind"] == "edge":
        return ("E", block["facets_rel"][0])
    return ("V", tuple(block["facets_rel"]))


def format_block_token(label: tuple) -> str:
    kind, value = label
    if kind == "E":
        return str(value)
    a, b = value
    return f"{a}{b}"


def format_signature(orbit: dict) -> str:
    q_labels = [normalize_block(block) for block in orbit["q_blocks"]]
    p_labels = [normalize_block(block) for block in orbit["p_blocks"]]
    q = "-".join(format_block_token(label) for label in q_labels)
    p = "-".join(format_block_token(label) for label in p_labels)
    return f"Q:{q}  P:{p}"


def point_list(points: np.ndarray) -> list[list[float]]:
    return [[round(float(x), 10), round(float(y), 10)] for x, y in points]


def projected_path(points: np.ndarray, coord_slice: slice, tol: float = 1e-9) -> np.ndarray:
    projection = points[:, coord_slice]
    compressed = [projection[0]]
    for point in projection[1:]:
        if np.linalg.norm(point - compressed[-1]) > tol:
            compressed.append(point)
    if np.linalg.norm(compressed[0] - compressed[-1]) > tol:
        compressed.append(compressed[0])
    return np.array(compressed, dtype=float)


def orbit_entry(orbit: dict, theta_deg: float) -> dict:
    breakpoints = recover_breakpoints(orbit, theta_deg)
    affine = compress_affine_cycle(breakpoints)
    return {
        "signature": format_signature(orbit),
        "sigma": orbit["sigma"],
        "bounces": orbit["bounces"],
        "q_path": point_list(projected_path(affine, slice(0, 2))),
        "p_path": point_list(projected_path(affine, slice(2, 4))),
        "raw_q_path": point_list(projected_path(breakpoints, slice(0, 2))),
        "raw_p_path": point_list(projected_path(breakpoints, slice(2, 4))),
    }


def polygon_data(theta_deg: float) -> dict:
    normals, heights = regular_pentagon_normals_and_heights()
    q_vertices = polygon_vertices_from_hrep(normals, heights)
    p_vertices = rotate_points(q_vertices, math.radians(theta_deg))
    return {
        "q_polygon": point_list(q_vertices),
        "p_polygon": point_list(p_vertices),
        "p_reference": point_list(q_vertices),
    }


def output_rows() -> list[dict]:
    rows = []
    for row in load_jsonl(INPUT_PATH):
        theta_deg = float(row["angle_deg"])
        polygons = polygon_data(theta_deg)
        rows.append(
            {
                "angle_deg": theta_deg,
                "angle_rad": row["angle_rad"],
                "capacity": row["capacity"],
                "sys": row["sys"],
                "n_tied_orbits": row["n_tied_orbits"],
                **polygons,
                "orbits": [
                    orbit_entry(orbit, theta_deg) for orbit in row["tied_orbits"]
                ],
            }
        )
    return rows


def write_dataset(rows: list[dict]) -> None:
    with DATASET_PATH.open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row, separators=(",", ":")) + "\n")


def write_html(rows: list[dict]) -> None:
    embedded = json.dumps(rows, separators=(",", ":"))
    HTML_PATH.write_text(
        HTML_TEMPLATE.replace("__DATA__", embedded),
        encoding="utf-8",
    )


HTML_TEMPLATE = r"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Pentagon Rotation Minimum Orbit Viewer</title>
  <style>
    :root {
      color-scheme: light;
      --ink: #222;
      --muted: #666;
      --axis: #d6d6d6;
      --poly: #777;
      --path: #1457a8;
      --path-soft: rgba(20, 87, 168, 0.28);
      --panel: #f7f7f7;
    }
    body {
      margin: 0;
      font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      color: var(--ink);
      background: white;
    }
    main {
      max-width: 1120px;
      margin: 0 auto;
      padding: 24px;
    }
    h1 {
      margin: 0 0 18px;
      font-size: 24px;
      font-weight: 650;
      letter-spacing: 0;
    }
    .controls {
      display: grid;
      grid-template-columns: 1fr minmax(190px, 280px) minmax(130px, 160px);
      gap: 16px;
      align-items: end;
      padding: 14px;
      background: var(--panel);
      border: 1px solid #ddd;
      border-radius: 6px;
    }
    label {
      display: block;
      font-size: 13px;
      color: var(--muted);
      margin-bottom: 6px;
    }
    input[type="range"] {
      width: 100%;
    }
    select {
      width: 100%;
      padding: 7px 8px;
      border: 1px solid #bbb;
      border-radius: 4px;
      background: white;
      color: var(--ink);
      font: inherit;
    }
    .toggle {
      display: flex;
      align-items: center;
      min-height: 37px;
      gap: 8px;
      font-size: 14px;
      color: var(--ink);
    }
    .metrics {
      display: flex;
      flex-wrap: wrap;
      gap: 10px 20px;
      margin: 14px 0 18px;
      font-size: 14px;
      color: var(--muted);
    }
    .metrics strong {
      color: var(--ink);
      font-weight: 650;
    }
    .plots {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 18px;
    }
    .plot {
      border: 1px solid #ddd;
      border-radius: 6px;
      padding: 10px;
    }
    .plot h2 {
      margin: 0 0 6px;
      font-size: 18px;
      font-weight: 600;
      text-align: center;
    }
    svg {
      width: 100%;
      height: auto;
      display: block;
      aspect-ratio: 1 / 1;
    }
    .note {
      margin-top: 14px;
      color: var(--muted);
      font-size: 13px;
      line-height: 1.45;
    }
    @media (max-width: 780px) {
      main {
        padding: 16px;
      }
      .controls,
      .plots {
        grid-template-columns: 1fr;
      }
    }
  </style>
</head>
<body>
<main>
  <h1>Pentagon Rotation Minimum Orbit Viewer</h1>
  <section class="controls">
    <div>
      <label for="theta">Rotation angle θ</label>
      <input id="theta" type="range" min="0" max="0" value="0" step="1">
    </div>
    <div>
      <label for="orbit">Tied minimizing orbit</label>
      <select id="orbit"></select>
    </div>
    <div>
      <label>Projection mode</label>
      <label class="toggle"><input id="rawMode" type="checkbox"> raw recovered</label>
    </div>
  </section>
  <section class="metrics">
    <div>θ = <strong id="angle"></strong></div>
    <div>capacity = <strong id="capacity"></strong></div>
    <div>sys = <strong id="sys"></strong></div>
    <div>tied minima = <strong id="ties"></strong></div>
    <div>signature = <strong id="signature"></strong></div>
  </section>
  <section class="plots">
    <div class="plot">
      <h2>Q projection</h2>
      <svg id="qPlot" viewBox="-1.45 -1.45 2.9 2.9" aria-label="Q projection"></svg>
    </div>
    <div class="plot">
      <h2>P projection</h2>
      <svg id="pPlot" viewBox="-1.45 -1.45 2.9 2.9" aria-label="P projection"></svg>
    </div>
  </section>
  <p class="note">
    This viewer uses the sampled minima sweep. It is for intuition and thesis
    illustration only. The exact proof is the Sage certificate
    <code>executable_proof.sage.py</code>.
  </p>
</main>
<script>
const rows = __DATA__;
const theta = document.getElementById("theta");
const orbitSelect = document.getElementById("orbit");
const rawMode = document.getElementById("rawMode");
theta.max = rows.length - 1;

function fmt(value, digits = 6) {
  return Number(value).toFixed(digits);
}

function pts(points) {
  return points.map(([x, y]) => `${x},${-y}`).join(" ");
}

function polyline(points, attrs) {
  const node = document.createElementNS("http://www.w3.org/2000/svg", "polyline");
  node.setAttribute("points", pts(points));
  for (const [key, value] of Object.entries(attrs)) node.setAttribute(key, value);
  return node;
}

function circle([x, y], attrs) {
  const node = document.createElementNS("http://www.w3.org/2000/svg", "circle");
  node.setAttribute("cx", x);
  node.setAttribute("cy", -y);
  for (const [key, value] of Object.entries(attrs)) node.setAttribute(key, value);
  return node;
}

function text([x, y], value, attrs) {
  const node = document.createElementNS("http://www.w3.org/2000/svg", "text");
  node.setAttribute("x", x);
  node.setAttribute("y", -y);
  node.textContent = value;
  for (const [key, val] of Object.entries(attrs)) node.setAttribute(key, val);
  return node;
}

function closed(points) {
  return [...points, points[0]];
}

function drawPlot(svg, polygon, path, reference = null) {
  svg.replaceChildren();
  svg.appendChild(polyline([[-1.25, 0], [1.25, 0]], {
    stroke: "var(--axis)", "stroke-width": "0.006", fill: "none"
  }));
  svg.appendChild(polyline([[0, -1.25], [0, 1.25]], {
    stroke: "var(--axis)", "stroke-width": "0.006", fill: "none"
  }));
  if (reference) {
    svg.appendChild(polyline(closed(reference), {
      stroke: "#cfcfcf", "stroke-width": "0.018", "stroke-dasharray": "0.04 0.035", fill: "none"
    }));
  }
  svg.appendChild(polyline(closed(polygon), {
    stroke: "var(--poly)", "stroke-width": "0.022", fill: "none", "stroke-linejoin": "round"
  }));
  svg.appendChild(polyline(path, {
    stroke: "var(--path)", "stroke-width": "0.034", fill: "none",
    "stroke-linecap": "round", "stroke-linejoin": "round"
  }));
  path.forEach((point, index) => {
    svg.appendChild(circle(point, { r: "0.035", fill: "white", stroke: "var(--path)", "stroke-width": "0.012" }));
    svg.appendChild(text([point[0] + 0.045, point[1] + 0.045], String.fromCharCode(65 + index), {
      "font-size": "0.105", fill: "var(--path)", "font-family": "monospace"
    }));
  });
}

function updateOrbitOptions(row, selectedIndex) {
  orbitSelect.replaceChildren();
  row.orbits.forEach((orbit, index) => {
    const option = document.createElement("option");
    option.value = index;
    option.textContent = `${index + 1}: ${orbit.signature}`;
    orbitSelect.appendChild(option);
  });
  orbitSelect.value = Math.min(selectedIndex, row.orbits.length - 1);
}

function render() {
  const row = rows[Number(theta.value)];
  const selected = Number(orbitSelect.value || 0);
  updateOrbitOptions(row, selected);
  const orbit = row.orbits[Number(orbitSelect.value)];
  document.getElementById("angle").textContent = `${fmt(row.angle_deg, 2)}°`;
  document.getElementById("capacity").textContent = fmt(row.capacity, 6);
  document.getElementById("sys").textContent = fmt(row.sys, 6);
  document.getElementById("ties").textContent = row.n_tied_orbits;
  document.getElementById("signature").textContent = orbit.signature;
  const qPath = rawMode.checked ? orbit.raw_q_path : orbit.q_path;
  const pPath = rawMode.checked ? orbit.raw_p_path : orbit.p_path;
  drawPlot(document.getElementById("qPlot"), row.q_polygon, qPath);
  drawPlot(document.getElementById("pPlot"), row.p_polygon, pPath, row.p_reference);
}

theta.addEventListener("input", render);
orbitSelect.addEventListener("change", render);
rawMode.addEventListener("change", render);
render();
</script>
</body>
</html>
"""


def main() -> None:
    rows = output_rows()
    write_dataset(rows)
    write_html(rows)
    print(f"wrote {DATASET_PATH}")
    print(f"wrote {HTML_PATH}")


if __name__ == "__main__":
    main()
