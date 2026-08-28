#!/usr/bin/env node
// screenshot-figures.mjs — Generate thesis figures from the 4D polytope viewer.
//
// Reproducible: deterministic camera position, north pole, trajectory selection.
// Requires: a local HTTP server on localhost:8080 serving this directory,
//           plus the pinned Playwright and Three.js npm packages documented in
//           ../README.md.
//
// Usage:
//   cd experiments/visualization/viewer
//   python3 -m http.server 8080 &
//   node screenshot-figures.mjs
//   kill %1
//
// Output: experiments/visualization/figures/viz-*.png

import { chromium } from 'playwright';
import { mkdirSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const FIGURES_DIR = resolve(__dirname, '../figures');
const THREE_JS_PATH = resolve(__dirname, 'node_modules/three/build/three.min.js');
const ORBIT_CONTROLS_PATH = resolve(
  __dirname, 'node_modules/three/examples/js/controls/OrbitControls.js'
);

// The browser viewport is fixed for deterministic cameras. Each publication
// figure may then crop to its plotted content so whitespace is not enlarged by
// the thesis include width.
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
    structureColor: 0x374151,
    edgeOpacity: 0.9,
    ridgeFillOpacity: 0.12,
    ridgeWireOpacity: 0.08,
    camera: { x: 2.6, y: 1.95, z: 3.25 },
    clip: { x: 70, y: 15, width: 660, height: 560 },
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
    structureColor: 0x6b7280,
    trajectoryColor: 0xc2410c,
    trajectoryTubeRadius: 0.014,
    trajectoryOutlineRadius: 0.025,
    trajectoryOutlineColor: 0x111827,
    edgeOpacity: 0.5,
    camera: { x: 3.2, y: 2.4, z: 4.0 },
    clip: { x: 180, y: 75, width: 450, height: 510 },
  },
];

async function main() {
  mkdirSync(FIGURES_DIR, { recursive: true });

  const browser = await chromium.launch();
  const context = await browser.newContext({ viewport: VIEWPORT });
  const page = await context.newPage();

  // The interactive viewer uses public CDNs. Publication regeneration instead
  // serves the same pinned Three.js files from node_modules so a CDN outage or
  // network policy cannot change or block the rendered asset.
  await page.route(
    'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js',
    route => route.fulfill({ path: THREE_JS_PATH, contentType: 'text/javascript' })
  );
  await page.route(
    'https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js',
    route => route.fulfill({ path: ORBIT_CONTROLS_PATH, contentType: 'text/javascript' })
  );

  for (const fig of FIGURES) {
    console.log(`Generating ${fig.name}...`);

    await page.goto(BASE_URL);
    // Wait for Three.js and its scene groups, not only for the script-level
    // function declaration. A fast browser can expose loadPolytope before
    // init() has created the groups that rebuildScene clears.
    await page.waitForFunction(() => {
      try {
        return typeof loadPolytope === 'function'
          && document.querySelector('#canvas-container canvas') !== null
          && Boolean(edgeGroup && ridgeGroup && vertexGroup && trajectoryGroup);
      } catch {
        return false;
      }
    });

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

    await page.evaluate(({
      structureColor,
      trajectoryColor,
      edgeOpacity,
      ridgeFillOpacity,
      ridgeWireOpacity,
      trajectoryTubeRadius,
      trajectoryOutlineRadius,
      trajectoryOutlineColor,
    }) => {
      STRUCTURE_COLOR_OVERRIDE = structureColor ?? null;
      TRAJECTORY_COLOR_OVERRIDE = trajectoryColor ?? null;
      EDGE_OPACITY = edgeOpacity ?? 0.7;
      RIDGE_FILL_OPACITY = ridgeFillOpacity ?? 0.18;
      RIDGE_WIRE_OPACITY = ridgeWireOpacity ?? 0.12;
      TRAJECTORY_TUBE_RADIUS = trajectoryTubeRadius ?? 0;
      TRAJECTORY_OUTLINE_RADIUS = trajectoryOutlineRadius ?? 0;
      TRAJECTORY_OUTLINE_COLOR = trajectoryOutlineColor ?? null;
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
    await page.screenshot({ path, clip: fig.clip });
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
