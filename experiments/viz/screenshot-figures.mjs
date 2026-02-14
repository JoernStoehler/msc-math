#!/usr/bin/env node
// screenshot-figures.mjs — Generate thesis figures from the 4D polytope viewer.
//
// Reproducible: deterministic camera position, north pole, trajectory selection.
// Requires: serve.sh running on localhost:8080, Playwright installed.
//
// Usage:
//   cd experiments/viz && node screenshot-figures.mjs
//
// Output: experiments/figures/viz-*.png

import { chromium } from 'playwright';
import { mkdirSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const FIGURES_DIR = resolve(__dirname, '..', 'figures');

// Viewport matches a thesis-friendly aspect ratio (4:3, 800x600)
const VIEWPORT = { width: 800, height: 600 };
const BASE_URL = 'http://localhost:8080';

// Figures to generate. Each entry produces one PNG.
//
// Camera and north pole are chosen per-polytope for balanced framing.
// The diagonal pole (1,1,1,1)/2 avoids aligning with any axis, giving
// a more balanced stereographic projection where no edges go to infinity.
const FIGURES = [
  // ---- Polytope structure (edges only, no trajectories) ----
  //
  // maxRadius=6 clips far arcs for tighter framing.
  // Separate edge/trajectory figures so trajectories are clearly visible.
  {
    name: 'viz-hypercube-edges',
    polytope: 'hypercube',
    trajectory: -2,          // -2 = hide trajectories entirely
    showEdges: true,
    showRidges: false,
    showVertices: true,
    showTrajectories: false,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-hko-pentagon-edges',
    polytope: 'hko_pentagon',
    trajectory: -2,
    showEdges: true,
    showRidges: false,
    showVertices: true,
    showTrajectories: false,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  // ---- Single Reeb trajectories (no edges, trajectory clearly visible) ----
  {
    name: 'viz-hypercube-traj',
    polytope: 'hypercube',
    trajectory: 0,
    showEdges: false,
    showRidges: false,
    showVertices: true,
    showTrajectories: true,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-simplex-traj',
    polytope: 'simplex',
    trajectory: 0,
    showEdges: false,
    showRidges: false,
    showVertices: true,
    showTrajectories: true,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-hko-pentagon-traj',
    polytope: 'hko_pentagon',
    trajectory: 0,
    showEdges: false,
    showRidges: false,
    showVertices: true,
    showTrajectories: true,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  {
    name: 'viz-lagrangian-tri-product-traj',
    polytope: 'lagrangian_triangle_product',
    trajectory: 0,
    showEdges: false,
    showRidges: false,
    showVertices: true,
    showTrajectories: true,
    northPole: 'e4',
    maxRadius: 6,
    camera: { x: 4, y: 3, z: 5 },
  },
  // ---- Overview: hypercube with ridges ----
  {
    name: 'viz-hypercube-ridges',
    polytope: 'hypercube',
    trajectory: -2,
    showEdges: true,
    showRidges: true,
    showVertices: false,
    showTrajectories: false,
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

    // Load polytope and wait for data
    await page.evaluate((name) => {
      return new Promise((resolve, reject) => {
        const origLoad = loadPolytope;
        // Patch to detect completion
        window._figReady = false;
        const origRebuild = rebuildScene;
        rebuildScene = function() {
          origRebuild();
          window._figReady = true;
        };
        origLoad(name);
        // Poll for completion
        const check = setInterval(() => {
          if (window._figReady) {
            clearInterval(check);
            rebuildScene = origRebuild;
            resolve();
          }
        }, 50);
        setTimeout(() => { clearInterval(check); reject(new Error('timeout')); }, 10000);
      });
    }, fig.polytope);

    // Override MAX_RADIUS for tighter framing (default 30 is too large for screenshots)
    if (fig.maxRadius) {
      await page.evaluate((r) => { MAX_RADIUS = r; }, fig.maxRadius);
    }

    // Set north pole preset (also calls rebuildScene, picking up the new MAX_RADIUS)
    await page.evaluate((preset) => setNorthPolePreset(preset), fig.northPole);

    // Set display toggles
    const showTraj = fig.showTrajectories !== false;
    await page.evaluate(({ showEdges, showRidges, showVertices, showTraj }) => {
      document.getElementById('show-edges').checked = showEdges;
      document.getElementById('show-ridges').checked = showRidges;
      document.getElementById('show-vertices').checked = showVertices;
      document.getElementById('show-traj').checked = showTraj;
      onToggle('edges', showEdges);
      onToggle('ridges', showRidges);
      onToggle('vertices', showVertices);
      onToggle('trajectories', showTraj);
    }, { showEdges: fig.showEdges, showRidges: fig.showRidges, showVertices: fig.showVertices, showTraj });

    // Select trajectory
    if (showTraj && fig.trajectory >= -1) {
      await page.evaluate((trajIdx) => onTrajectoryChange(trajIdx), fig.trajectory);
    }

    // Set camera position
    await page.evaluate(({ x, y, z }) => {
      camera.position.set(x, y, z);
      camera.lookAt(0, 0, 0);
      controls.update();
    }, fig.camera);

    // Wait a frame for render
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
