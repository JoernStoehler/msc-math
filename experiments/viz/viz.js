// viz.js — Three.js scene for 4D polytope visualization (light theme)
//
// Depends on: projection.js (loaded first), Three.js r128 + OrbitControls

// ---- Constants ----
const MAX_RADIUS = 30;
const EDGE_SAMPLES = 96;
const RIDGE_SUBDIVISIONS = 8;
const TRAJ_SAMPLES = 96;

// Visual parameters (light theme, tuned for white background)
const VERTEX_RADIUS = 0.05;
const VERTEX_SEGMENTS = 14;
const EDGE_OPACITY = 0.7;
const RIDGE_FILL_OPACITY = 0.18;
const RIDGE_WIRE_OPACITY = 0.12;
const ARROW_COLOR = 0x24292f;
const ARROW_HEAD_LENGTH = 0.08;
const ARROW_HEAD_WIDTH = 0.04;
const FACET_SATURATION = 0.65;
const FACET_LIGHTNESS = 0.42;

// ---- State ----
let scene, camera, renderer, controls;
let polytopeData = null;
let northPole = normalize4([0, 0, 0, 1]);
let orthoBasis = buildOrthoBasis(northPole);

let showEdges = true;
let showRidges = false;
let showVertices = true;
let showTrajectories = true;
let selectedTrajectory = -1; // -1 = all

// Scene groups (cleared and rebuilt when polytope or projection changes)
let edgeGroup, ridgeGroup, vertexGroup, trajectoryGroup;

/** HSL color for a facet, evenly spaced around the hue wheel. */
function facetColor(facetIndex, totalFacets) {
    const hue = facetIndex / totalFacets;
    return new THREE.Color().setHSL(hue, FACET_SATURATION, FACET_LIGHTNESS);
}

// ---- Initialization ----

function init() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0xffffff);

    camera = new THREE.PerspectiveCamera(60, window.innerWidth / window.innerHeight, 0.01, 500);
    camera.position.set(4, 3, 5);

    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    document.getElementById('canvas-container').appendChild(renderer.domElement);

    controls = new THREE.OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;

    // Three-point lighting: key + fill + rim
    scene.add(new THREE.AmbientLight(0xffffff, 0.6));
    const key = new THREE.DirectionalLight(0xffffff, 0.8);
    key.position.set(5, 8, 5);
    scene.add(key);
    const fill = new THREE.DirectionalLight(0x8899bb, 0.4);
    fill.position.set(-3, -2, -5);
    scene.add(fill);
    const rim = new THREE.DirectionalLight(0xbb9988, 0.2);
    rim.position.set(0, -5, 3);
    scene.add(rim);

    edgeGroup = new THREE.Group();
    ridgeGroup = new THREE.Group();
    vertexGroup = new THREE.Group();
    trajectoryGroup = new THREE.Group();
    scene.add(edgeGroup, ridgeGroup, vertexGroup, trajectoryGroup);

    window.addEventListener('resize', onResize);
    animate();
}

function onResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
}

function animate() {
    requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
}

// ---- Data loading ----

function loadPolytope(name) {
    const path = `data/${name}.json`;
    fetch(path)
        .then(r => {
            if (!r.ok) throw new Error(`Failed to load ${path}: ${r.status}`);
            return r.json();
        })
        .then(data => {
            polytopeData = data;
            selectedTrajectory = -1;
            updateTrajectorySlider();
            rebuildScene();
            updateInfoPanel();
        })
        .catch(err => {
            document.getElementById('info-text').textContent = `Error: ${err.message}`;
        });
}

// ---- Scene rebuild ----

/** Dispose geometry and materials, then remove all children from a group. */
function clearGroup(group) {
    while (group.children.length > 0) {
        const child = group.children[0];
        if (child.geometry) child.geometry.dispose();
        if (child.material) {
            if (Array.isArray(child.material)) {
                child.material.forEach(m => m.dispose());
            } else {
                child.material.dispose();
            }
        }
        group.remove(child);
    }
}

/** Rebuild all scene geometry from polytopeData + current north pole. */
function rebuildScene() {
    clearGroup(edgeGroup);
    clearGroup(ridgeGroup);
    clearGroup(vertexGroup);
    clearGroup(trajectoryGroup);

    if (!polytopeData) return;

    const poly = polytopeData;
    const dotThreshold = poleCullingThreshold(MAX_RADIUS);

    // ---- Vertices ----
    if (showVertices) {
        const sphereGeom = new THREE.SphereGeometry(VERTEX_RADIUS, VERTEX_SEGMENTS, VERTEX_SEGMENTS);
        for (let i = 0; i < poly.vertices.length; i++) {
            const onSphere = radialProject(poly.vertices[i]);
            if (dot4(onSphere, northPole) >= dotThreshold) continue; // near pole — skip
            const p = stereographicProject(onSphere, northPole, orthoBasis, MAX_RADIUS);
            const fc = poly.vertex_facets[i][0] || 0;
            const mat = new THREE.MeshPhongMaterial({
                color: facetColor(fc, poly.facet_count),
                shininess: 40,
            });
            const mesh = new THREE.Mesh(sphereGeom, mat);
            mesh.position.set(p[0], p[1], p[2]);
            vertexGroup.add(mesh);
        }
    }

    // ---- Edges ----
    if (showEdges) {
        for (const [vi, vj] of poly.edges) {
            const subSegments = projectSegment(
                poly.vertices[vi], poly.vertices[vj],
                northPole, orthoBasis, MAX_RADIUS, EDGE_SAMPLES
            );

            const sharedFacets = poly.vertex_facets[vi].filter(f => poly.vertex_facets[vj].includes(f));
            const color = facetColor(sharedFacets[0] || 0, poly.facet_count);

            for (const pts of subSegments) {
                if (pts.length < 2) continue;
                const positions = [];
                for (const p of pts) positions.push(p[0], p[1], p[2]);

                const geom = new THREE.BufferGeometry();
                geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
                const mat = new THREE.LineBasicMaterial({
                    color: color,
                    linewidth: 1,
                    transparent: true,
                    opacity: EDGE_OPACITY,
                });
                edgeGroup.add(new THREE.Line(geom, mat));
            }
        }
    }

    // ---- Ridges (2-faces) ----
    if (showRidges) {
        for (const ridge of poly.ridges) {
            renderRidge(ridge, poly);
        }
    }

    // ---- Trajectories ----
    if (showTrajectories && poly.trajectories.length > 0) {
        const trajIndices = selectedTrajectory === -1
            ? poly.trajectories.map((_, i) => i)
            : [selectedTrajectory];

        for (const ti of trajIndices) {
            if (ti >= poly.trajectories.length) continue;
            renderTrajectory(poly.trajectories[ti]);
        }
    }
}

// ---- Ridge rendering ----

/**
 * Render a single ridge (2-face) as a curved surface.
 * Sampling on S³ via slerp for uniform angular spacing.
 */
function renderRidge(ridge, poly) {
    const verts = ridge.vertices;
    const nv = verts.length;
    if (nv < 3) return;

    const N = RIDGE_SUBDIVISIONS;
    const color = facetColor(ridge.facets[0], poly.facet_count);

    if (nv === 3) {
        renderTriangleRidge(poly.vertices[verts[0]], poly.vertices[verts[1]], poly.vertices[verts[2]], color, N);
    } else if (nv === 4) {
        renderQuadRidge(
            poly.vertices[verts[0]], poly.vertices[verts[1]],
            poly.vertices[verts[2]], poly.vertices[verts[3]],
            color, N
        );
    } else {
        // General polygon: fan-triangulate from centroid
        const centroid4 = [0, 0, 0, 0];
        for (const vi of verts) {
            for (let k = 0; k < 4; k++) centroid4[k] += poly.vertices[vi][k];
        }
        for (let k = 0; k < 4; k++) centroid4[k] /= nv;
        for (let i = 0; i < nv; i++) {
            const vi = verts[i];
            const vj = verts[(i + 1) % nv];
            renderTriangleRidge(centroid4, poly.vertices[vi], poly.vertices[vj], color, N);
        }
    }
}

function renderTriangleRidge(a4, b4, c4, color, N) {
    const gridPoints = projectTriangleGrid(a4, b4, c4, N, northPole, orthoBasis, MAX_RADIUS);

    const gridIndex = (i, j) => {
        let idx = 0;
        for (let row = 0; row < j; row++) idx += (N + 1 - row);
        return idx + i;
    };

    const positions = [];
    for (let j = 0; j < N; j++) {
        for (let i = 0; i < N - j; i++) {
            const a = gridIndex(i, j);
            const b = gridIndex(i + 1, j);
            const c = gridIndex(i, j + 1);
            const pa = gridPoints[a], pb = gridPoints[b], pc = gridPoints[c];
            positions.push(pa[0], pa[1], pa[2]);
            positions.push(pb[0], pb[1], pb[2]);
            positions.push(pc[0], pc[1], pc[2]);

            if (i + 1 <= N - j - 1) {
                const idx = gridIndex(i + 1, j + 1);
                const pd = gridPoints[idx];
                positions.push(pb[0], pb[1], pb[2]);
                positions.push(pd[0], pd[1], pd[2]);
                positions.push(pc[0], pc[1], pc[2]);
            }
        }
    }

    if (positions.length === 0) return;
    addRidgeMesh(positions, color);
}

function renderQuadRidge(a4, b4, c4, d4, color, N) {
    const gridPoints = projectQuadGrid(a4, b4, c4, d4, N, northPole, orthoBasis, MAX_RADIUS);

    const positions = [];
    for (let j = 0; j < N; j++) {
        for (let i = 0; i < N; i++) {
            const a = j * (N + 1) + i;
            const b = a + 1;
            const c = a + (N + 1);
            const dd = c + 1;
            const pa = gridPoints[a], pb = gridPoints[b], pc = gridPoints[c], pd = gridPoints[dd];
            positions.push(pa[0], pa[1], pa[2]);
            positions.push(pb[0], pb[1], pb[2]);
            positions.push(pc[0], pc[1], pc[2]);

            positions.push(pb[0], pb[1], pb[2]);
            positions.push(pd[0], pd[1], pd[2]);
            positions.push(pc[0], pc[1], pc[2]);
        }
    }

    addRidgeMesh(positions, color);
}

/** Add a translucent mesh + wireframe overlay for a ridge surface. */
function addRidgeMesh(positions, color) {
    const geom = new THREE.BufferGeometry();
    geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
    geom.computeVertexNormals();

    const mat = new THREE.MeshPhongMaterial({
        color: color,
        transparent: true,
        opacity: RIDGE_FILL_OPACITY,
        side: THREE.DoubleSide,
        depthWrite: false,
        shininess: 20,
    });
    ridgeGroup.add(new THREE.Mesh(geom, mat));

    const wireMat = new THREE.MeshBasicMaterial({
        color: color,
        transparent: true,
        opacity: RIDGE_WIRE_OPACITY,
        wireframe: true,
    });
    ridgeGroup.add(new THREE.Mesh(geom.clone(), wireMat));
}

// ---- Trajectory rendering ----

/** Render a single Reeb trajectory as colored polyline segments with a direction arrow. */
function renderTrajectory(traj) {
    for (const seg of traj.segments) {
        const subSegments = projectSegment(
            seg.start, seg.end,
            northPole, orthoBasis, MAX_RADIUS, TRAJ_SAMPLES
        );

        const color = facetColor(seg.facet, polytopeData.facet_count);
        for (const pts of subSegments) {
            if (pts.length < 2) continue;
            const positions = [];
            for (const p of pts) positions.push(p[0], p[1], p[2]);

            const geom = new THREE.BufferGeometry();
            geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
            const mat = new THREE.LineBasicMaterial({
                color: color,
                linewidth: 2,
            });
            trajectoryGroup.add(new THREE.Line(geom, mat));
        }
    }

    // Direction arrow at trajectory start
    if (traj.segments.length > 0) {
        const seg = traj.segments[0];
        const startPt = fullProject(seg.start, northPole, orthoBasis, MAX_RADIUS);
        const midPt = fullProject(
            [seg.start[0]*0.8 + seg.end[0]*0.2,
             seg.start[1]*0.8 + seg.end[1]*0.2,
             seg.start[2]*0.8 + seg.end[2]*0.2,
             seg.start[3]*0.8 + seg.end[3]*0.2],
            northPole, orthoBasis, MAX_RADIUS
        );
        const dir = new THREE.Vector3(
            midPt[0] - startPt[0],
            midPt[1] - startPt[1],
            midPt[2] - startPt[2]
        );
        const len = dir.length();
        if (len > 0.01) {
            dir.normalize();
            const arrow = new THREE.ArrowHelper(
                dir,
                new THREE.Vector3(startPt[0], startPt[1], startPt[2]),
                Math.min(len * 2, 0.3),
                ARROW_COLOR,
                ARROW_HEAD_LENGTH,
                ARROW_HEAD_WIDTH
            );
            trajectoryGroup.add(arrow);
        }
    }
}

// ---- UI handlers ----

function updateInfoPanel() {
    if (!polytopeData) return;
    const poly = polytopeData;
    let info = `${poly.name}\n`;
    info += `source: ${poly.source}\n`;
    info += `capacity: ${poly.capacity.toFixed(6)}\n\n`;
    info += `${poly.facet_count} facets\n`;
    info += `${poly.vertex_count} vertices\n`;
    info += `${poly.edge_count} edges\n`;
    info += `${poly.ridge_count} ridges\n\n`;
    info += `${poly.trajectories.length} trajectories\n`;

    const closedCount = poly.trajectories.filter(t => t.closed).length;
    if (closedCount > 0) {
        info += `(${closedCount} closed)\n`;
    }

    document.getElementById('info-text').textContent = info;
}

function updateTrajectorySlider() {
    const slider = document.getElementById('traj-index');
    if (!polytopeData) return;
    slider.max = polytopeData.trajectories.length - 1;
    slider.value = selectedTrajectory === -1 ? 0 : selectedTrajectory;
    updateTrajLabel();
}

function updateTrajLabel() {
    if (!polytopeData) return;
    const label = document.getElementById('traj-label');
    if (selectedTrajectory === -1) {
        label.textContent = `all (${polytopeData.trajectories.length})`;
    } else {
        const t = polytopeData.trajectories[selectedTrajectory];
        label.textContent = `${selectedTrajectory}/${polytopeData.trajectories.length - 1} (facet ${t.start_facet}, ${t.segments.length} segs${t.closed ? ', closed' : ''})`;
    }
}

function updateNorthPole() {
    const phi = parseFloat(document.getElementById('north-phi').value);
    const theta = parseFloat(document.getElementById('north-theta').value);
    const psi = parseFloat(document.getElementById('north-psi').value);

    // Hopf-like parameterization of S³
    northPole = normalize4([
        Math.sin(phi) * Math.cos(theta),
        Math.sin(phi) * Math.sin(theta),
        Math.cos(phi) * Math.sin(psi),
        Math.cos(phi) * Math.cos(psi),
    ]);

    orthoBasis = buildOrthoBasis(northPole);

    const poleStr = `(${northPole[0].toFixed(2)}, ${northPole[1].toFixed(2)}, ${northPole[2].toFixed(2)}, ${northPole[3].toFixed(2)})`;
    document.getElementById('pole-display').textContent = poleStr;

    rebuildScene();
}

function setNorthPolePreset(preset) {
    if (!polytopeData) return;
    const poly = polytopeData;

    switch (preset) {
        case 'e4':
            northPole = [0, 0, 0, 1];
            break;
        case 'e3':
            northPole = [0, 0, 1, 0];
            break;
        case 'e1':
            northPole = [1, 0, 0, 0];
            break;
        case 'vertex0':
            if (poly.vertices.length > 0) {
                northPole = normalize4(poly.vertices[0]);
            }
            break;
        case 'normal0':
            if (poly.normals.length > 0) {
                northPole = normalize4(poly.normals[0]);
            }
            break;
        case 'diagonal':
            northPole = normalize4([1, 1, 1, 1]);
            break;
    }

    orthoBasis = buildOrthoBasis(northPole);
    const poleStr = `(${northPole[0].toFixed(2)}, ${northPole[1].toFixed(2)}, ${northPole[2].toFixed(2)}, ${northPole[3].toFixed(2)})`;
    document.getElementById('pole-display').textContent = poleStr;

    rebuildScene();
}

function onPolytopeChange(name) {
    loadPolytope(name);
}

function onTrajectoryChange(value) {
    selectedTrajectory = parseInt(value);
    updateTrajLabel();
    clearGroup(trajectoryGroup);
    if (showTrajectories && polytopeData && polytopeData.trajectories.length > 0) {
        const trajIndices = selectedTrajectory === -1
            ? polytopeData.trajectories.map((_, i) => i)
            : [selectedTrajectory];
        for (const ti of trajIndices) {
            if (ti >= polytopeData.trajectories.length) continue;
            renderTrajectory(polytopeData.trajectories[ti]);
        }
    }
}

function onToggle(which, checked) {
    switch (which) {
        case 'edges': showEdges = checked; break;
        case 'ridges': showRidges = checked; break;
        case 'vertices': showVertices = checked; break;
        case 'trajectories': showTrajectories = checked; break;
    }
    rebuildScene();
}

function resetCamera() {
    camera.position.set(4, 3, 5);
    camera.lookAt(0, 0, 0);
    controls.reset();
}
