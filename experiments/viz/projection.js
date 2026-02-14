// projection.js — 4D to 3D projection functions
//
// Pipeline: point in R⁴ → radial projection to S³ → stereographic to R³
//
// Sampling is done on S³ (via slerp) to get uniform angular spacing,
// which stereographic projection maps to smooth R³ spacing.

/**
 * Radial projection: x ↦ x / |x|, mapping ∂K → S³.
 * @param {number[]} p - Point in R⁴ (4-element array)
 * @returns {number[]} Unit vector on S³
 */
function radialProject(p) {
    const norm = Math.sqrt(p[0]*p[0] + p[1]*p[1] + p[2]*p[2] + p[3]*p[3]);
    if (norm < 1e-15) return [0, 0, 0, 0];
    return [p[0]/norm, p[1]/norm, p[2]/norm, p[3]/norm];
}

/**
 * Spherical linear interpolation on S³.
 * slerp(a, b, t) traces a great-circle arc from a to b.
 * @param {number[]} a - Unit vector on S³
 * @param {number[]} b - Unit vector on S³
 * @param {number} t - Parameter in [0, 1]
 * @returns {number[]} Unit vector on S³
 */
function slerp4(a, b, t) {
    let d = dot4(a, b);
    // Clamp for numerical safety
    d = Math.max(-1, Math.min(1, d));
    const omega = Math.acos(d);
    if (omega < 1e-10) {
        // Nearly identical: use lerp
        return normalize4([
            a[0] + t * (b[0] - a[0]),
            a[1] + t * (b[1] - a[1]),
            a[2] + t * (b[2] - a[2]),
            a[3] + t * (b[3] - a[3]),
        ]);
    }
    const sinOmega = Math.sin(omega);
    const sa = Math.sin((1 - t) * omega) / sinOmega;
    const sb = Math.sin(t * omega) / sinOmega;
    return [
        sa * a[0] + sb * b[0],
        sa * a[1] + sb * b[1],
        sa * a[2] + sb * b[2],
        sa * a[3] + sb * b[3],
    ];
}

/**
 * Build an orthonormal basis for the 3D hyperplane perpendicular to `pole` in R⁴.
 * Returns 3 vectors, each of length 4.
 * @param {number[]} pole - Unit vector in R⁴
 * @returns {number[][]} Three orthonormal basis vectors [e1, e2, e3]
 */
function buildOrthoBasis(pole) {
    // Start with standard basis vectors and Gram-Schmidt
    const candidates = [
        [1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]
    ];

    const basis = [];
    for (const c of candidates) {
        if (basis.length >= 3) break;

        // Subtract projections onto pole and existing basis vectors
        let v = [...c];
        // Remove pole component
        const dp = dot4(v, pole);
        for (let i = 0; i < 4; i++) v[i] -= dp * pole[i];
        // Remove existing basis components
        for (const b of basis) {
            const db = dot4(v, b);
            for (let i = 0; i < 4; i++) v[i] -= db * b[i];
        }

        const n = norm4(v);
        if (n > 1e-8) {
            for (let i = 0; i < 4; i++) v[i] /= n;
            basis.push(v);
        }
    }

    return basis;
}

/**
 * Stereographic projection from north pole on S³ to R³.
 *
 * For y on S³ and north pole n:
 *   The projection maps y to a point in the 3D hyperplane ⊥ n, via the ray from n through y.
 *   Formula: π(y) = (y - n(y·n)) / (1 - y·n), then project to 3D coords via orthobasis.
 *
 * @param {number[]} y - Point on S³ (4D unit vector)
 * @param {number[]} northPole - North pole (4D unit vector)
 * @param {number[][]} basis - Orthonormal basis for hyperplane ⊥ northPole (from buildOrthoBasis)
 * @param {number} maxRadius - Clamp projected points to this radius (prevents infinity near pole)
 * @returns {number[]} Point in R³
 */
function stereographicProject(y, northPole, basis, maxRadius) {
    const dn = dot4(y, northPole);
    const denom = 1.0 - dn;

    if (Math.abs(denom) < 1e-10) {
        // At the north pole — project to "infinity" (clamped)
        return [maxRadius, 0, 0];
    }

    // 4D point in the tangent plane
    const scale = 1.0 / denom;
    const proj4 = [
        (y[0] - northPole[0] * dn) * scale,
        (y[1] - northPole[1] * dn) * scale,
        (y[2] - northPole[2] * dn) * scale,
        (y[3] - northPole[3] * dn) * scale,
    ];

    // Project to 3D using orthonormal basis
    const x = dot4(proj4, basis[0]);
    const yCoord = dot4(proj4, basis[1]);
    const z = dot4(proj4, basis[2]);

    // Clamp
    const r = Math.sqrt(x*x + yCoord*yCoord + z*z);
    if (r > maxRadius) {
        const s = maxRadius / r;
        return [x*s, yCoord*s, z*s];
    }

    return [x, yCoord, z];
}

/**
 * Full projection: R⁴ boundary point → R³ via radial + stereographic.
 * @param {number[]} p - Point in R⁴ (on ∂K)
 * @param {number[]} northPole - North pole for stereographic projection
 * @param {number[][]} basis - Orthonormal basis (from buildOrthoBasis)
 * @param {number} maxRadius - Clamp radius
 * @returns {number[]} Point in R³
 */
function fullProject(p, northPole, basis, maxRadius) {
    const onSphere = radialProject(p);
    return stereographicProject(onSphere, northPole, basis, maxRadius);
}

/**
 * Project a line segment in R⁴ to R³, sampling along the great-circle arc on S³.
 * Clips near the stereographic north pole: splits the arc at the 0–2 points
 * where it enters/exits the ε-ball around the pole, omitting the inside.
 *
 * @param {number[]} start - Start point in R⁴
 * @param {number[]} end - End point in R⁴
 * @param {number[]} northPole
 * @param {number[][]} basis
 * @param {number} maxRadius
 * @param {number} nSamples - Number of sample points (default 20)
 * @returns {number[][][]} Array of polyline sub-segments, each an array of R³ points.
 *   Typically 1 sub-segment; 0 if entirely inside the pole ball; 2 if it crosses through.
 */
function projectSegment(start, end, northPole, basis, maxRadius, nSamples) {
    nSamples = nSamples || 20;
    const a = radialProject(start);
    const b = radialProject(end);

    // Dot product threshold: points with dot(y, pole) >= this project beyond maxRadius.
    // From stereographic radius: r = sqrt((1+d)/(1-d)), setting r = R gives d = (R²-1)/(R²+1).
    const R2 = maxRadius * maxRadius;
    const dotThreshold = (R2 - 1) / (R2 + 1);

    const segments = [];
    let current = [];

    for (let i = 0; i <= nSamples; i++) {
        const t = i / nSamples;
        const onSphere = slerp4(a, b, t);
        const d = dot4(onSphere, northPole);

        if (d < dotThreshold) {
            // Outside pole ball — include this point
            if (current.length === 0 && i > 0) {
                // Just exited pole ball — find boundary point
                const tPrev = (i - 1) / nSamples;
                const boundary = findPoleBoundary(a, b, tPrev, t, northPole, dotThreshold);
                current.push(stereographicProject(boundary, northPole, basis, maxRadius));
            }
            current.push(stereographicProject(onSphere, northPole, basis, maxRadius));
        } else {
            // Inside pole ball — clip
            if (current.length > 0) {
                // Just entered pole ball — find boundary point
                const tPrev = (i - 1) / nSamples;
                const boundary = findPoleBoundary(a, b, tPrev, t, northPole, dotThreshold);
                current.push(stereographicProject(boundary, northPole, basis, maxRadius));
                segments.push(current);
                current = [];
            }
        }
    }
    if (current.length > 0) {
        segments.push(current);
    }

    return segments;
}

/**
 * Binary search for the slerp parameter where dot(slerp(a,b,t), pole) = threshold.
 * Assumes dot changes monotonically between tLow and tHigh (valid for nearby samples).
 */
function findPoleBoundary(a, b, tLow, tHigh, pole, threshold) {
    // Ensure tLow is the "outside" side (dot < threshold)
    const dLow = dot4(slerp4(a, b, tLow), pole);
    if (dLow >= threshold) {
        const tmp = tLow; tLow = tHigh; tHigh = tmp;
    }
    for (let iter = 0; iter < 20; iter++) {
        const tMid = (tLow + tHigh) / 2;
        const d = dot4(slerp4(a, b, tMid), pole);
        if (d < threshold) {
            tLow = tMid;
        } else {
            tHigh = tMid;
        }
    }
    // Return the point just outside the threshold
    return slerp4(a, b, tLow);
}

/**
 * Project a triangle in R⁴ to a grid of R³ points, sampling on S³.
 *
 * Projects the three R⁴ vertices to S³, then uses spherical barycentric
 * interpolation (slerp along edges, then slerp between edge points)
 * for interior sampling.
 *
 * @param {number[]} a4 - First vertex in R⁴
 * @param {number[]} b4 - Second vertex in R⁴
 * @param {number[]} c4 - Third vertex in R⁴
 * @param {number} N - Number of subdivisions per edge
 * @param {number[]} northPole
 * @param {number[][]} basis
 * @param {number} maxRadius
 * @returns {number[][]} Grid of R³ points, indexed by (i,j) with i+j<=N
 */
function projectTriangleGrid(a4, b4, c4, N, northPole, basis, maxRadius) {
    const aS = radialProject(a4);
    const bS = radialProject(b4);
    const cS = radialProject(c4);

    const gridPoints = [];
    for (let j = 0; j <= N; j++) {
        for (let i = 0; i <= N - j; i++) {
            const u = i / N;
            const v = j / N;
            // Spherical barycentric: slerp along edges, then between
            // p_ab = slerp(a, b, u/(1-v))  for the point on edge a-b at fraction u/(1-v)
            // p    = slerp(p_ab, c, v)       interpolate toward c
            let onSphere;
            if (v > 1 - 1e-12) {
                onSphere = cS;
            } else {
                const uNorm = u / (1 - v);
                const pab = slerp4(aS, bS, uNorm);
                onSphere = slerp4(pab, cS, v);
            }
            gridPoints.push(stereographicProject(onSphere, northPole, basis, maxRadius));
        }
    }
    return gridPoints;
}

/**
 * Project a quad in R⁴ to a grid of R³ points, sampling on S³.
 *
 * Projects the four R⁴ vertices to S³, then uses spherical bilinear
 * interpolation for interior sampling.
 *
 * @param {number[]} a4 - Vertex 0 in R⁴ (polygon order: a-b-c-d)
 * @param {number[]} b4 - Vertex 1 in R⁴
 * @param {number[]} c4 - Vertex 2 in R⁴
 * @param {number[]} d4 - Vertex 3 in R⁴
 * @param {number} N - Number of subdivisions per edge
 * @param {number[]} northPole
 * @param {number[][]} basis
 * @param {number} maxRadius
 * @returns {number[][]} Grid of R³ points, row-major (N+1)*(N+1)
 */
function projectQuadGrid(a4, b4, c4, d4, N, northPole, basis, maxRadius) {
    const aS = radialProject(a4);
    const bS = radialProject(b4);
    const cS = radialProject(c4);
    const dS = radialProject(d4);

    const gridPoints = [];
    for (let j = 0; j <= N; j++) {
        for (let i = 0; i <= N; i++) {
            const u = i / N;
            const v = j / N;
            // Spherical bilinear: slerp edges, then slerp between
            const pab = slerp4(aS, bS, u);
            const pdc = slerp4(dS, cS, u);
            const onSphere = slerp4(pab, pdc, v);
            gridPoints.push(stereographicProject(onSphere, northPole, basis, maxRadius));
        }
    }
    return gridPoints;
}

// ---- Utility functions ----

function dot4(a, b) {
    return a[0]*b[0] + a[1]*b[1] + a[2]*b[2] + a[3]*b[3];
}

function norm4(v) {
    return Math.sqrt(v[0]*v[0] + v[1]*v[1] + v[2]*v[2] + v[3]*v[3]);
}

function normalize4(v) {
    const n = norm4(v);
    if (n < 1e-15) return [0, 0, 0, 0];
    return [v[0]/n, v[1]/n, v[2]/n, v[3]/n];
}
