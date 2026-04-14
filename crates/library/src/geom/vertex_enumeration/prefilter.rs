//! Floating-point prefilter used before exact candidate checks.

pub(super) fn f64_prefilter_rejects(dv_f64: &[[f64; 4]], subset: &[usize; 4], f: usize) -> bool {
    use nalgebra::{Matrix4, Vector4};

    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const C: f64 = 1e4;

    let a = Matrix4::new(
        dv_f64[subset[0]][0],
        dv_f64[subset[0]][1],
        dv_f64[subset[0]][2],
        dv_f64[subset[0]][3],
        dv_f64[subset[1]][0],
        dv_f64[subset[1]][1],
        dv_f64[subset[1]][2],
        dv_f64[subset[1]][3],
        dv_f64[subset[2]][0],
        dv_f64[subset[2]][1],
        dv_f64[subset[2]][2],
        dv_f64[subset[2]][3],
        dv_f64[subset[3]][0],
        dv_f64[subset[3]][1],
        dv_f64[subset[3]][2],
        dv_f64[subset[3]][3],
    );
    let svd = a.svd(true, true);
    let svals = &svd.singular_values;
    let sigma_min = svals[0].min(svals[1]).min(svals[2]).min(svals[3]);
    let sigma_max = svals[0].max(svals[1]).max(svals[2]).max(svals[3]);
    if sigma_min == 0.0 {
        return false;
    }
    let kappa_hat = sigma_max / sigma_min;
    if EPS_MACH * kappa_hat > 0.25 {
        return false;
    }
    let ones = Vector4::new(1.0, 1.0, 1.0, 1.0);
    let Ok(v_hat) = svd.solve(&ones, 0.0) else {
        return false;
    };
    if v_hat.iter().any(|&x| !x.is_finite()) {
        return false;
    }

    let v_norm = v_hat.norm();
    for (i, y_i) in dv_f64[..f].iter().enumerate() {
        if subset.contains(&i) {
            continue;
        }
        let s_hat = y_i[0] * v_hat[0] + y_i[1] * v_hat[1] + y_i[2] * v_hat[2] + y_i[3] * v_hat[3];
        let y_norm = (y_i[0] * y_i[0] + y_i[1] * y_i[1] + y_i[2] * y_i[2] + y_i[3] * y_i[3]).sqrt();
        let delta = C * kappa_hat * EPS_MACH * v_norm * y_norm;
        if !s_hat.is_finite() || !delta.is_finite() {
            return false;
        }
        if s_hat > 1.0 + delta {
            return true;
        }
    }
    false
}
