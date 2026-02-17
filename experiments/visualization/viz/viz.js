// viz.js — Three.js scene for 4D polytope visualization (light theme)
//
// Depends on: projection.js (loaded first), Three.js r128 + OrbitControls

// ---- Constants ----
var MAX_RADIUS = 30; // var (not const) so screenshot scripts can override via page.evaluate
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
let visibleTrajectories = new Set(); // Set of trajectory indices to show

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

    // Sync checkbox states with JS variables
    document.getElementById('show-vertices').checked = showVertices;
    document.getElementById('show-edges').checked = showEdges;
    document.getElementById('show-ridges').checked = showRidges;

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
    // Load from embedded data (window.POLYTOPE_DATA from data.js)
    if (!window.POLYTOPE_DATA || !window.POLYTOPE_DATA[name]) {
        document.getElementById('info-text').textContent = `Error: Polytope "${name}" not found`;
        return;
    }

    polytopeData = window.POLYTOPE_DATA[name];

    // Initialize all trajectories as visible
    visibleTrajectories.clear();
    for (let i = 0; i < polytopeData.trajectories.length; i++) {
        visibleTrajectories.add(i);
    }

    generateTrajectoryCheckboxes();
    rebuildScene();
    updateInfoPanel();
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

/**
 * Collect ridge geometry (triangles) grouped by color.
 * Mutates ridgesByColor map to add triangles for this ridge.
 */
function collectRidgeGeometry(ridge, poly, ridgesByColor) {
    const verts = ridge.vertices;
    const nv = verts.length;
    if (nv < 3) return;

    const N = RIDGE_SUBDIVISIONS;
    const color = facetColor(ridge.facets[0], poly.facet_count);
    const colorKey = color.getHex();

    if (!ridgesByColor.has(colorKey)) {
        ridgesByColor.set(colorKey, { color, solidTriangles: [], wireTriangles: [] });
    }
    const entry = ridgesByColor.get(colorKey);

    if (nv === 3) {
        collectTriangleRidgeGeometry(
            poly.vertices[verts[0]], poly.vertices[verts[1]], poly.vertices[verts[2]],
            N, entry
        );
    } else if (nv === 4) {
        collectQuadRidgeGeometry(
            poly.vertices[verts[0]], poly.vertices[verts[1]],
            poly.vertices[verts[2]], poly.vertices[verts[3]],
            N, entry
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
            collectTriangleRidgeGeometry(centroid4, poly.vertices[vi], poly.vertices[vj], N, entry);
        }
    }
}

function collectTriangleRidgeGeometry(a4, b4, c4, N, entry) {
    const gridPoints = projectTriangleGrid(a4, b4, c4, N, northPole, orthoBasis, MAX_RADIUS);

    const gridIndex = (i, j) => {
        let idx = 0;
        for (let row = 0; row < j; row++) idx += (N + 1 - row);
        return idx + i;
    };

    for (let j = 0; j < N; j++) {
        for (let i = 0; i < N - j; i++) {
            const a = gridIndex(i, j);
            const b = gridIndex(i + 1, j);
            const c = gridIndex(i, j + 1);
            const pa = gridPoints[a], pb = gridPoints[b], pc = gridPoints[c];
            if (pa && pb && pc) {
                entry.solidTriangles.push([pa[0], pa[1], pa[2], pb[0], pb[1], pb[2], pc[0], pc[1], pc[2]]);
                entry.wireTriangles.push([pa[0], pa[1], pa[2], pb[0], pb[1], pb[2], pc[0], pc[1], pc[2]]);
            }

            if (i < N - j - 1) {
                const d = gridIndex(i + 1, j + 1);
                const pd = gridPoints[d];
                if (pb && pc && pd) {
                    entry.solidTriangles.push([pb[0], pb[1], pb[2], pd[0], pd[1], pd[2], pc[0], pc[1], pc[2]]);
                    entry.wireTriangles.push([pb[0], pb[1], pb[2], pd[0], pd[1], pd[2], pc[0], pc[1], pc[2]]);
                }
            }
        }
    }
}

function collectQuadRidgeGeometry(a4, b4, c4, d4, N, entry) {
    const grid = projectQuadGrid(a4, b4, c4, d4, N, northPole, orthoBasis, MAX_RADIUS);

    for (let j = 0; j < N; j++) {
        for (let i = 0; i < N; i++) {
            const p00 = grid[j * (N + 1) + i];
            const p10 = grid[j * (N + 1) + (i + 1)];
            const p01 = grid[(j + 1) * (N + 1) + i];
            const p11 = grid[(j + 1) * (N + 1) + (i + 1)];

            if (p00 && p10 && p11) {
                entry.solidTriangles.push([p00[0], p00[1], p00[2], p10[0], p10[1], p10[2], p11[0], p11[1], p11[2]]);
                entry.wireTriangles.push([p00[0], p00[1], p00[2], p10[0], p10[1], p10[2], p11[0], p11[1], p11[2]]);
            }
            if (p00 && p11 && p01) {
                entry.solidTriangles.push([p00[0], p00[1], p00[2], p11[0], p11[1], p11[2], p01[0], p01[1], p01[2]]);
                entry.wireTriangles.push([p00[0], p00[1], p00[2], p11[0], p11[1], p11[2], p01[0], p01[1], p01[2]]);
            }
        }
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

    // ---- Vertices (batched by color) ----
    if (showVertices) {
        const verticesByColor = new Map(); // color_key -> [{pos, matrix}, ...]

        for (let i = 0; i < poly.vertices.length; i++) {
            const onSphere = radialProject(poly.vertices[i]);
            if (dot4(onSphere, northPole) >= dotThreshold) continue;
            const p = stereographicProject(onSphere, northPole, orthoBasis, MAX_RADIUS);
            const fc = poly.vertex_facets[i][0] || 0;
            const color = facetColor(fc, poly.facet_count);
            const colorKey = color.getHex();

            if (!verticesByColor.has(colorKey)) {
                verticesByColor.set(colorKey, { color, positions: [] });
            }
            verticesByColor.get(colorKey).positions.push(p);
        }

        // Create instanced meshes for each color
        const sphereGeom = new THREE.SphereGeometry(VERTEX_RADIUS, VERTEX_SEGMENTS, VERTEX_SEGMENTS);
        for (const [colorKey, { color, positions }] of verticesByColor) {
            const instancedMesh = new THREE.InstancedMesh(
                sphereGeom,
                new THREE.MeshPhongMaterial({ color: color, shininess: 40 }),
                positions.length
            );
            for (let i = 0; i < positions.length; i++) {
                const matrix = new THREE.Matrix4();
                matrix.setPosition(positions[i][0], positions[i][1], positions[i][2]);
                instancedMesh.setMatrixAt(i, matrix);
            }
            instancedMesh.instanceMatrix.needsUpdate = true;
            vertexGroup.add(instancedMesh);
        }
    }

    // ---- Edges (batched by color) ----
    if (showEdges) {
        const edgesByColor = new Map(); // color_key -> [positions...]

        for (const [vi, vj] of poly.edges) {
            const subSegments = projectSegment(
                poly.vertices[vi], poly.vertices[vj],
                northPole, orthoBasis, MAX_RADIUS, EDGE_SAMPLES
            );

            const sharedFacets = poly.vertex_facets[vi].filter(f => poly.vertex_facets[vj].includes(f));
            const color = facetColor(sharedFacets[0] || 0, poly.facet_count);
            const colorKey = color.getHex();

            if (!edgesByColor.has(colorKey)) {
                edgesByColor.set(colorKey, { color, segments: [] });
            }

            for (const pts of subSegments) {
                if (pts.length < 2) continue;
                edgesByColor.get(colorKey).segments.push(pts);
            }
        }

        // Create one LineSegments per color
        for (const [colorKey, { color, segments }] of edgesByColor) {
            const positions = [];
            for (const pts of segments) {
                // Convert continuous line to line segments (pairs of points)
                for (let i = 0; i < pts.length - 1; i++) {
                    positions.push(pts[i][0], pts[i][1], pts[i][2]);
                    positions.push(pts[i + 1][0], pts[i + 1][1], pts[i + 1][2]);
                }
            }

            if (positions.length > 0) {
                const geom = new THREE.BufferGeometry();
                geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
                const mat = new THREE.LineBasicMaterial({
                    color: color,
                    linewidth: 1,
                    transparent: true,
                    opacity: EDGE_OPACITY,
                });
                edgeGroup.add(new THREE.LineSegments(geom, mat));
            }
        }
    }

    // ---- Ridges (2-faces) (batched by color) ----
    if (showRidges) {
        const ridgesByColor = new Map(); // color_key -> {solid: triangles, wire: triangles}

        for (const ridge of poly.ridges) {
            collectRidgeGeometry(ridge, poly, ridgesByColor);
        }

        // Create merged meshes per color
        for (const [colorKey, { color, solidTriangles, wireTriangles }] of ridgesByColor) {
            // Solid fill
            if (solidTriangles.length > 0) {
                const positions = [];
                for (const tri of solidTriangles) {
                    positions.push(...tri);
                }
                const geom = new THREE.BufferGeometry();
                geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
                geom.computeVertexNormals();
                const mat = new THREE.MeshPhongMaterial({
                    color: color,
                    side: THREE.DoubleSide,
                    transparent: true,
                    opacity: RIDGE_FILL_OPACITY,
                    depthWrite: false,
                    shininess: 20,
                });
                ridgeGroup.add(new THREE.Mesh(geom, mat));
            }

            // Wireframe
            if (wireTriangles.length > 0) {
                const positions = [];
                for (const tri of wireTriangles) {
                    // Convert triangles to line segments (edges)
                    positions.push(tri[0], tri[1], tri[2], tri[3], tri[4], tri[5]); // edge 0-1
                    positions.push(tri[3], tri[4], tri[5], tri[6], tri[7], tri[8]); // edge 1-2
                    positions.push(tri[6], tri[7], tri[8], tri[0], tri[1], tri[2]); // edge 2-0
                }
                const geom = new THREE.BufferGeometry();
                geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
                const mat = new THREE.LineBasicMaterial({
                    color: color,
                    transparent: true,
                    opacity: RIDGE_WIRE_OPACITY,
                });
                ridgeGroup.add(new THREE.LineSegments(geom, mat));
            }
        }
    }

    // ---- Trajectories (batched by facet color) ----
    if (visibleTrajectories.size > 0 && poly.trajectories.length > 0) {
        const trajIndices = Array.from(visibleTrajectories);

        const trajsByColor = new Map(); // color_key -> [segments...]
        const arrows = []; // arrows can't be batched easily, collect them separately

        for (const ti of trajIndices) {
            if (ti >= poly.trajectories.length) continue;
            const traj = poly.trajectories[ti];

            // Collect line segments by color
            for (const seg of traj.segments) {
                const subSegments = projectSegment(
                    seg.start, seg.end,
                    northPole, orthoBasis, MAX_RADIUS, TRAJ_SAMPLES
                );

                const color = facetColor(seg.facet, poly.facet_count);
                const colorKey = color.getHex();

                if (!trajsByColor.has(colorKey)) {
                    trajsByColor.set(colorKey, { color, segments: [] });
                }

                for (const pts of subSegments) {
                    if (pts.length < 2) continue;
                    trajsByColor.get(colorKey).segments.push(pts);
                }
            }

            // Collect direction arrow
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
                arrows.push({ startPt, midPt });
            }
        }

        // Create one LineSegments per color
        for (const [colorKey, { color, segments }] of trajsByColor) {
            const positions = [];
            for (const pts of segments) {
                for (let i = 0; i < pts.length - 1; i++) {
                    positions.push(pts[i][0], pts[i][1], pts[i][2]);
                    positions.push(pts[i + 1][0], pts[i + 1][1], pts[i + 1][2]);
                }
            }

            if (positions.length > 0) {
                const geom = new THREE.BufferGeometry();
                geom.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
                const mat = new THREE.LineBasicMaterial({
                    color: color,
                    linewidth: 2,
                });
                trajectoryGroup.add(new THREE.LineSegments(geom, mat));
            }
        }

        // Add arrows (not batched)
        for (const { startPt, midPt } of arrows) {
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
    info += `${poly.ridge_count} ridges\n`;
    if (poly.volume !== undefined) {
        info += `volume: ${poly.volume.toFixed(6)}\n`;
    }
    if (poly.systolic_ratio !== undefined) {
        info += `systolic ratio: ${poly.systolic_ratio.toFixed(6)}\n`;
    }
    info += `\n${poly.trajectories.length} trajectories\n`;

    const closedCount = poly.trajectories.filter(t => t.closed).length;
    if (closedCount > 0) {
        info += `(${closedCount} closed)\n`;
    }

    document.getElementById('info-text').textContent = info;
}

function generateTrajectoryCheckboxes() {
    const container = document.getElementById('trajectory-checkboxes');
    if (!container || !polytopeData) return;

    container.innerHTML = '';

    for (let i = 0; i < polytopeData.trajectories.length; i++) {
        const traj = polytopeData.trajectories[i];
        const row = document.createElement('div');
        row.className = 'toggle-row';

        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.id = `traj-${i}`;
        checkbox.checked = visibleTrajectories.has(i);
        checkbox.onchange = () => onTrajectoryToggle(i, checkbox.checked);

        const label = document.createElement('span');
        label.textContent = `Traj ${i} (facet ${traj.start_facet}, ${traj.segments.length} seg${traj.closed ? ', closed' : ''})`;

        row.appendChild(checkbox);
        row.appendChild(label);
        container.appendChild(row);
    }
}

function setNorthPoleFromText(text) {
    try {
        const parts = text.split(',').map(s => parseFloat(s.trim()));
        if (parts.length !== 4 || parts.some(isNaN)) {
            alert('Invalid input. Please enter four numbers separated by commas, e.g., "0, 0, 0, 1"');
            return;
        }

        const [x, y, z, w] = parts;
        northPole = normalize4([x, y, z, w]);
        orthoBasis = buildOrthoBasis(northPole);

        updateNorthPoleDisplay();
        rebuildScene();
    } catch (e) {
        alert('Error parsing north pole coordinates: ' + e.message);
    }
}

function updateNorthPoleDisplay() {
    const poleStr = `(${northPole[0].toFixed(2)}, ${northPole[1].toFixed(2)}, ${northPole[2].toFixed(2)}, ${northPole[3].toFixed(2)})`;
    document.getElementById('pole-display').textContent = poleStr;
    document.getElementById('north-pole-input').value = `${northPole[0].toFixed(4)}, ${northPole[1].toFixed(4)}, ${northPole[2].toFixed(4)}, ${northPole[3].toFixed(4)}`;
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
    updateNorthPoleDisplay();
    rebuildScene();
}

function onPolytopeChange(name) {
    loadPolytope(name);
}

function onTrajectoryToggle(index, checked) {
    if (checked) {
        visibleTrajectories.add(index);
    } else {
        visibleTrajectories.delete(index);
    }
    rebuildScene();
}

function onToggle(which, checked) {
    switch (which) {
        case 'edges': showEdges = checked; break;
        case 'ridges': showRidges = checked; break;
        case 'vertices': showVertices = checked; break;
    }
    rebuildScene();
}

function resetCamera() {
    camera.position.set(4, 3, 5);
    camera.lookAt(0, 0, 0);
    controls.reset();
}
