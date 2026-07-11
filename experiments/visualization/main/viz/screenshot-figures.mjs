#!/usr/bin/env node
// screenshot-figures.mjs — Generate thesis figures from the 4D polytope viewer.
//
// Reproducible: deterministic camera position, north pole, trajectory selection.
// Requires: a local HTTP server on localhost:8080 serving this directory,
//           and Playwright installed (npm install playwright).
//
// Usage:
//   cd experiments/visualization/main/viz
//   python3 -m http.server 8080 &
//   node screenshot-figures.mjs
//   kill %1
//
// Output: experiments/visualization/main/viz-*.png

import { chromium } from 'playwright';
import { mkdirSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const FIGURES_DIR = resolve(__dirname, '..');

// Viewport matches a thesis-friendly aspect ratio (4:3, 800x600)
const VIEWPORT = { width: 800, height: 600 };
const BASE_URL = process.env.VIZ_BASE_URL || 'http://localhost:8080';

// Thesis figures to generate. Each entry produces one PNG. The first gives a
// structural view of projected two-faces; the second keeps the projected HKO
// one-skeleton visible while emphasizing one recovered minimum-action orbit.
const FIGURES = [
  {
    name: 'viz-hypercube-ridges',
    polytope: 'hypercube',
    showEdges: true,
    showRidges: true,
    showVertices: false,
    trajectoryIndex: null,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 2.6, y: 1.95, z: 3.25 },
  },
  {
    name: 'viz-hko-pentagon-min-orbit',
    polytope: 'hko_pentagon',
    showEdges: true,
    showRidges: false,
    showVertices: true,
    trajectoryIndex: 0,
    northPole: 'e4',
    maxRadius: 6,
    structureColor: 0x4b5563,
    trajectoryColor: 0x6d28d9,
    edgeOpacity: 0.65,
    camera: { x: 3.2, y: 2.4, z: 4.0 },
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

    await page.evaluate(({ structureColor, trajectoryColor, edgeOpacity }) => {
      STRUCTURE_COLOR_OVERRIDE = structureColor ?? null;
      TRAJECTORY_COLOR_OVERRIDE = trajectoryColor ?? null;
      EDGE_OPACITY = edgeOpacity ?? 0.7;
    }, fig);

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
