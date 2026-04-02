#!/usr/bin/env node
// screenshot-figures.mjs — Generate thesis figures from the 4D polytope viewer.
//
// Reproducible: deterministic camera position, north pole, trajectory selection.
// Requires: a local HTTP server on localhost:8080 serving this directory,
//           and Playwright installed (npm install playwright).
//
// Usage:
//   cd experiments/visualization/viz
//   python3 -m http.server 8080 &
//   node screenshot-figures.mjs
//   kill %1
//
// Output: experiments/visualization/viz-*.png

import { chromium } from 'playwright';
import { mkdirSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const FIGURES_DIR = resolve(__dirname, '..');

// Viewport matches a thesis-friendly aspect ratio (4:3, 800x600)
const VIEWPORT = { width: 800, height: 600 };
const BASE_URL = 'http://localhost:8080';

// Figures to generate. Each entry produces one PNG.
//
// Camera and north pole are chosen per-polytope for balanced framing.
// The diagonal pole (1,1,1,1)/2 avoids aligning with any axis, giving
// a more balanced stereographic projection where no edges go to infinity.
//
// trajectoryIndex: which single trajectory to show (null = hide all)
const FIGURES = [
  // ---- Polytope structure (edges only, no trajectories) ----
  //
  // maxRadius=6 clips far arcs for tighter framing.
  // Separate edge/trajectory figures so trajectories are clearly visible.
  {
    name: 'viz-hypercube-edges',
    polytope: 'hypercube',
    showEdges: true,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: null,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-hko-pentagon-edges',
    polytope: 'hko_pentagon',
    showEdges: true,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: null,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  // ---- Single Reeb trajectories (no edges, trajectory clearly visible) ----
  {
    name: 'viz-hypercube-traj',
    polytope: 'hypercube',
    showEdges: false,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: 0,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-simplex-traj',
    polytope: 'simplex',
    showEdges: false,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: 0,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-hko-pentagon-traj',
    polytope: 'hko_pentagon',
    showEdges: false,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: 0,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-lagrangian-tri-product-traj',
    polytope: 'lagrangian_triangle_product',
    showEdges: false,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: 0,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  // ---- Overview: hypercube with ridges ----
  {
    name: 'viz-hypercube-ridges',
    polytope: 'hypercube',
    showEdges: true,
    showRidges: true,
    showVertices: false,
    trajectoryIndex: null,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
];

async function main() {
  mkdirSync(FIGURES_DIR, { recursive: true });

  const browser = await chromium.launch();
  const context = await browser.newContext({ viewport: VIEWPORT });
  const page = await context.newPage();

  for (const fig of FIGURES) {
    console.log(`Generating ${fig.name}...`);

    await page.goto(BASE_URL);
    // Wait for Three.js to initialize
    await page.waitForFunction(() => typeof loadPolytope === 'function');

    // Load polytope (this also initializes all trajectories as visible)
    await page.evaluate((name) => {
      loadPolytope(name);
    }, fig.polytope);

    // Wait for scene to build
    await page.waitForTimeout(300);

    // Override MAX_RADIUS for tighter framing (default 30 is too large for screenshots)
    if (fig.maxRadius) {
      await page.evaluate((r) => { MAX_RADIUS = r; }, fig.maxRadius);
    }

    // Set north pole preset (calls rebuildScene with new MAX_RADIUS)
    await page.evaluate((preset) => setNorthPolePreset(preset), fig.northPole);

    // Set display toggles via the global variables + onToggle (which calls rebuildScene)
    await page.evaluate(({ edges, ridges, vertices }) => {
      onToggle('edges', edges);
      onToggle('ridges', ridges);
      onToggle('vertices', vertices);
    }, { edges: fig.showEdges, ridges: fig.showRidges, vertices: fig.showVertices });

    // Set trajectory visibility: show only the specified trajectory, or none
    await page.evaluate((trajIdx) => {
      // Hide all trajectories first
      const indices = Array.from(visibleTrajectories);
      for (const i of indices) {
        onTrajectoryToggle(i, false);
      }
      // Show the single requested trajectory (if any)
      if (trajIdx !== null && polytopeData && trajIdx < polytopeData.trajectories.length) {
        onTrajectoryToggle(trajIdx, true);
      }
    }, fig.trajectoryIndex);

    // Set camera position
    await page.evaluate(({ x, y, z }) => {
      camera.position.set(x, y, z);
      camera.lookAt(0, 0, 0);
      controls.update();
    }, fig.camera);

    // Wait for render
    await page.waitForTimeout(200);

    // Hide the control panels for clean screenshot
    await page.evaluate(() => {
      document.getElementById('controls').style.display = 'none';
      document.getElementById('info').style.display = 'none';
    });

    await page.waitForTimeout(100);

    const path = resolve(FIGURES_DIR, `${fig.name}.png`);
    await page.screenshot({ path });
    console.log(`  → ${path}`);

    // Restore panels
    await page.evaluate(() => {
      document.getElementById('controls').style.display = '';
      document.getElementById('info').style.display = '';
    });
  }

  await browser.close();
  console.log(`Done. ${FIGURES.length} figures saved to ${FIGURES_DIR}/`);
}

main().catch(e => { console.error(e); process.exit(1); });
