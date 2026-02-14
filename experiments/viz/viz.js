// viz.js — Three.js scene for 4D polytope visualization (light theme)

// ---- State ----
let scene, camera, renderer, controls;
let polytopeData = null;
let northPole = normalize4([0, 0, 0, 1]);
let orthoBasis = buildOrthoBasis(northPole);
const MAX_RADIUS = 30;
const EDGE_SAMPLES = 96;
const RIDGE_INTERIOR_SUBDIVISIONS = 8;
const TRAJ_SAMPLES = 96;

// Display toggles
let showEdges = true;
let showRidges = false;
let showVertices = true;
let showTrajectories = true;
let selectedTrajectory = -1; // -1 = all

// Scene groups
let edgeGroup, ridgeGroup, vertexGroup, trajectoryGroup;

// Color palette for facets — tuned for white background.
// Medium saturation, moderate lightness for good contrast without harshness.
function facetColor(facetIndex, totalFacets) {
    const hue = facetIndex / totalFacets;
    return new THREE.Color().setHSL(hue, 0.65, 0.42);
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

    // Lighting — tuned for white background, softer ambient
    scene.add(new THREE.AmbientLight(0xffffff, 0.6));
    const dir1 = new THREE.DirectionalLight(0xffffff, 0.8);
    dir1.position.set(5, 8, 5);
    scene.add(dir1);
    const dir2 = new THREE.DirectionalLight(0x8899bb, 0.4);
    dir2.position.set(-3, -2, -5);
    scene.add(dir2);
    const dir3 = new THREE.DirectionalLight(0xbb9988, 0.2);
    dir3.position.set(0, -5, 3);
    scene.add(dir3);

    // Groups
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

function rebuildScene() {
    clearGroup(edgeGroup);
    clearGroup(ridgeGroup);
    clearGroup(vertexGroup);
    clearGroup(trajectoryGroup);

    if (!polytopeData) return;

    const d = polytopeData;

    // Project all vertices to R³
    const projVerts = d.vertices.map(v => fullProject(v, northPole, orthoBasis, MAX_RADIUS));

    // ---- Vertices ----
    if (showVertices) {
        const sphereGeom = new THREE.SphereGeometry(0.05, 14, 14);
        for (let i = 0; i < projVerts.length; i++) {
            const p = projVerts[i];
            const fc = d.vertex_facets[i][0] || 0;
            const mat = new THREE.MeshPhongMaterial({
                color: facetColor(fc, d.facet_count),
                shininess: 40,
            });
            const mesh = new THREE.Mesh(sphereGeom, mat);
            mesh.position.set(p[0], p[1], p[2]);
            vertexGroup.add(mesh);
        }
    }

    // ---- Edges ----
    if (showEdges) {
        for (const [vi, vj] of d.edges) {
            const pts = projectSegment(
                d.vertices[vi], d.vertices[vj],
                northPole, orthoBasis, MAX_RADIUS, EDGE_SAMPLES
            );
            const positions = [];
            for (const p of pts) positions.push(p[0], p[1], p[2]);

            const geom = new THREE.BufferGeometry();
            geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));

            const sharedFacets = d.vertex_facets[vi].filter(f => d.vertex_facets[vj].includes(f));
            const color = facetColor(sharedFacets[0] || 0, d.facet_count);

            const mat = new THREE.LineBasicMaterial({
                color: color,
                linewidth: 1,
                transparent: true,
                opacity: 0.7,
            });
            edgeGroup.add(new THREE.Line(geom, mat));
        }
    }

    // ---- Ridges (2-faces) ----
    if (showRidges) {
        for (const ridge of d.ridges) {
            renderRidge(ridge, d);
        }
    }

    // ---- Trajectories ----
    if (showTrajectories && d.trajectories.length > 0) {
        const trajIndices = selectedTrajectory === -1
            ? d.trajectories.map((_, i) => i)
            : [selectedTrajectory];

        for (const ti of trajIndices) {
            if (ti >= d.trajectories.length) continue;
            renderTrajectory(d.trajectories[ti], ti, d.trajectories.length);
        }
    }
}

/**
 * Render a single ridge (2-face) as a curved surface.
 * Sampling on S³ via slerp for uniform angular spacing.
 */
function renderRidge(ridge, d) {
    const verts = ridge.vertices;
    const nv = verts.length;
    if (nv < 3) return;

    const N = RIDGE_INTERIOR_SUBDIVISIONS;
    const color = facetColor(ridge.facets[0], d.facet_count);

    if (nv === 3) {
        renderTriangleRidge(d.vertices[verts[0]], d.vertices[verts[1]], d.vertices[verts[2]], color, N);
    } else if (nv === 4) {
        renderQuadRidge(
            d.vertices[verts[0]], d.vertices[verts[1]],
            d.vertices[verts[2]], d.vertices[verts[3]],
            color, N
        );
    } else {
        const centroid4 = [0, 0, 0, 0];
        for (const vi of verts) {
            for (let k = 0; k < 4; k++) centroid4[k] += d.vertices[vi][k];
        }
        for (let k = 0; k < 4; k++) centroid4[k] /= nv;
        for (let i = 0; i < nv; i++) {
            const vi = verts[i];
            const vj = verts[(i + 1) % nv];
            renderTriangleRidge(centroid4, d.vertices[vi], d.vertices[vj], color, N);
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
                const d = gridIndex(i + 1, j + 1);
                const pd = gridPoints[d];
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

function addRidgeMesh(positions, color) {
    const geom = new THREE.BufferGeometry();
    geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
    geom.computeVertexNormals();

    // Tinted fill
    const mat = new THREE.MeshPhongMaterial({
        color: color,
        transparent: true,
        opacity: 0.18,
        side: THREE.DoubleSide,
        depthWrite: false,
        shininess: 20,
    });
    ridgeGroup.add(new THREE.Mesh(geom, mat));

    // Wireframe overlay — subtle on white
    const wireMat = new THREE.MeshBasicMaterial({
        color: color,
        transparent: true,
        opacity: 0.12,
        wireframe: true,
    });
    ridgeGroup.add(new THREE.Mesh(geom.clone(), wireMat));
}

function renderTrajectory(traj, trajIndex, totalTrajectories) {
    const d = polytopeData;

    for (const seg of traj.segments) {
        const pts = projectSegment(
            seg.start, seg.end,
            northPole, orthoBasis, MAX_RADIUS, TRAJ_SAMPLES
        );
        const positions = [];
        for (const p of pts) positions.push(p[0], p[1], p[2]);

        const geom = new THREE.BufferGeometry();
        geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));

        const color = facetColor(seg.facet, d.facet_count);
        const mat = new THREE.LineBasicMaterial({
            color: color,
            linewidth: 2,
        });
        trajectoryGroup.add(new THREE.Line(geom, mat));
    }

    // Direction arrow — dark on white background
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
                0x24292f,
                0.08,
                0.04
            );
            trajectoryGroup.add(arrow);
        }
    }
}

// ---- UI ----

function updateInfoPanel() {
    if (!polytopeData) return;
    const d = polytopeData;
    let info = `${d.name}\n`;
    info += `source: ${d.source}\n`;
    info += `capacity: ${d.capacity.toFixed(6)}\n\n`;
    info += `${d.facet_count} facets\n`;
    info += `${d.vertex_count} vertices\n`;
    info += `${d.edge_count} edges\n`;
    info += `${d.ridge_count} ridges\n\n`;
    info += `${d.trajectories.length} trajectories\n`;

    const closedCount = d.trajectories.filter(t => t.closed).length;
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
    const d = polytopeData;

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
            if (d.vertices.length > 0) {
                northPole = normalize4(d.vertices[0]);
            }
            break;
        case 'normal0':
            if (d.normals.length > 0) {
                northPole = normalize4(d.normals[0]);
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
            renderTrajectory(polytopeData.trajectories[ti], ti, polytopeData.trajectories.length);
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
