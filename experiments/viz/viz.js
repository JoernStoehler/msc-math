// viz.js — Three.js scene for 4D polytope visualization

// ---- State ----
let scene, camera, renderer, controls;
let polytopeData = null;
let northPole = normalize4([0, 0, 0, 1]);
let orthoBasis = buildOrthoBasis(northPole);
const MAX_RADIUS = 30;
const EDGE_SAMPLES = 24;

// Display toggles
let showEdges = true;
let showRidges = false;
let showVertices = true;
let showTrajectories = true;
let selectedTrajectory = -1; // -1 = all

// Scene groups
let edgeGroup, ridgeGroup, vertexGroup, trajectoryGroup;

// Color palette for facets
function facetColor(facetIndex, totalFacets) {
    const hue = facetIndex / totalFacets;
    return new THREE.Color().setHSL(hue, 0.7, 0.55);
}

// ---- Initialization ----

function init() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0d1117);

    camera = new THREE.PerspectiveCamera(60, window.innerWidth / window.innerHeight, 0.01, 500);
    camera.position.set(4, 3, 5);

    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    document.getElementById('canvas-container').appendChild(renderer.domElement);

    controls = new THREE.OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;

    // Lighting
    scene.add(new THREE.AmbientLight(0x404060, 1.5));
    const dir1 = new THREE.DirectionalLight(0xffffff, 1.0);
    dir1.position.set(5, 8, 5);
    scene.add(dir1);
    const dir2 = new THREE.DirectionalLight(0x6688cc, 0.4);
    dir2.position.set(-3, -2, -5);
    scene.add(dir2);

    // Axes
    scene.add(new THREE.AxesHelper(1.5));

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
            // Reset trajectory selector
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
        const sphereGeom = new THREE.SphereGeometry(0.04, 12, 12);
        for (let i = 0; i < projVerts.length; i++) {
            const p = projVerts[i];
            // Color by first incident facet
            const fc = d.vertex_facets[i][0] || 0;
            const mat = new THREE.MeshPhongMaterial({ color: facetColor(fc, d.facet_count) });
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

            // Color by shared facets
            const sharedFacets = d.vertex_facets[vi].filter(f => d.vertex_facets[vj].includes(f));
            const color = facetColor(sharedFacets[0] || 0, d.facet_count);

            const mat = new THREE.LineBasicMaterial({
                color: color,
                linewidth: 1,
                transparent: true,
                opacity: 0.6,
            });
            edgeGroup.add(new THREE.Line(geom, mat));
        }
    }

    // ---- Ridges (2-faces as translucent surfaces) ----
    if (showRidges) {
        for (const ridge of d.ridges) {
            const verts = ridge.vertices;
            if (verts.length < 3) continue;

            // Sample each edge of the polygon densely
            const polygonPoints = [];
            for (let i = 0; i < verts.length; i++) {
                const vi = verts[i];
                const vj = verts[(i + 1) % verts.length];
                const pts = projectSegment(
                    d.vertices[vi], d.vertices[vj],
                    northPole, orthoBasis, MAX_RADIUS, 12
                );
                // Don't duplicate the endpoint (next segment starts there)
                for (let k = 0; k < pts.length - 1; k++) {
                    polygonPoints.push(new THREE.Vector3(pts[k][0], pts[k][1], pts[k][2]));
                }
            }

            // Fan triangulation from centroid
            const centroid = new THREE.Vector3(0, 0, 0);
            for (const p of polygonPoints) centroid.add(p);
            centroid.divideScalar(polygonPoints.length);

            const geom = new THREE.BufferGeometry();
            const positions = [];
            for (let i = 0; i < polygonPoints.length; i++) {
                const a = polygonPoints[i];
                const b = polygonPoints[(i + 1) % polygonPoints.length];
                positions.push(centroid.x, centroid.y, centroid.z);
                positions.push(a.x, a.y, a.z);
                positions.push(b.x, b.y, b.z);
            }
            geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
            geom.computeVertexNormals();

            const color = facetColor(ridge.facets[0], d.facet_count);
            const mat = new THREE.MeshPhongMaterial({
                color: color,
                transparent: true,
                opacity: 0.12,
                side: THREE.DoubleSide,
                depthWrite: false,
            });
            ridgeGroup.add(new THREE.Mesh(geom, mat));
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

function renderTrajectory(traj, trajIndex, totalTrajectories) {
    const d = polytopeData;

    // Give each trajectory a distinctive overall hue offset
    const hueOffset = trajIndex / Math.max(totalTrajectories, 1);

    for (const seg of traj.segments) {
        const pts = projectSegment(
            seg.start, seg.end,
            northPole, orthoBasis, MAX_RADIUS, EDGE_SAMPLES
        );
        const positions = [];
        for (const p of pts) positions.push(p[0], p[1], p[2]);

        const geom = new THREE.BufferGeometry();
        geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));

        // Color by facet
        const color = facetColor(seg.facet, d.facet_count);
        const mat = new THREE.LineBasicMaterial({
            color: color,
            linewidth: 2,
        });
        trajectoryGroup.add(new THREE.Line(geom, mat));
    }

    // Add small arrow at the start to show direction
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
                0xffffff,
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

function setProjectionMode(mode) {
    // Currently only stereographic is supported
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

    // Update display
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
    // Only rebuild trajectory group (not the whole scene)
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
