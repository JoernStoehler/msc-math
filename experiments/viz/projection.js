// projection.js — 4D to 3D projection functions
//
// Pipeline: point in R⁴ → radial projection to S³ → stereographic to R³

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
 * Project a line segment in R⁴ to R³, sampling intermediate points for smooth curves.
 * Under radial+stereographic, straight lines become circular arcs.
 * @param {number[]} start - Start point in R⁴
 * @param {number[]} end - End point in R⁴
 * @param {number[]} northPole
 * @param {number[][]} basis
 * @param {number} maxRadius
 * @param {number} nSamples - Number of sample points (default 20)
 * @returns {number[][]} Array of R³ points tracing the projected curve
 */
function projectSegment(start, end, northPole, basis, maxRadius, nSamples) {
    nSamples = nSamples || 20;
    const points = [];
    for (let i = 0; i <= nSamples; i++) {
        const t = i / nSamples;
        const interp = [
            start[0] + t * (end[0] - start[0]),
            start[1] + t * (end[1] - start[1]),
            start[2] + t * (end[2] - start[2]),
            start[3] + t * (end[3] - start[3]),
        ];
        points.push(fullProject(interp, northPole, basis, maxRadius));
    }
    return points;
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
