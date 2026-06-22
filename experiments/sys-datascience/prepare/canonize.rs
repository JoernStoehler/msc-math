//! Translation canonization helpers for normalized dual-vertex polytopes.
//!
//! A normalized dual list represents `K = {x : <a_i,x> <= 1}`. If `c` is an
//! interior point and `y = x - c`, then the translated body is represented by
//! normalized dual vectors `a_i / (1 - <a_i,c>)`.

use nalgebra::DMatrix;
use nalgebra::Vector4;

use euclidean_polytopes::volume_and_centroid_from_incidence_f64;

const MIN_TRANSLATION_DENOMINATOR: f64 = 1e-12;

#[derive(Clone, Debug)]
pub struct TranslationCanonization {
    pub center: Vector4<f64>,
    pub min_denominator: f64,
    pub dual_vertices: Vec<Vector4<f64>>,
}

fn average_primal_vertex_center(vertices: &[Vector4<f64>]) -> Option<Vector4<f64>> {
    if vertices.is_empty() {
        return None;
    }
    let center = vertices
        .iter()
        .copied()
        .fold(Vector4::zeros(), |sum, vertex| sum + vertex)
        / vertices.len() as f64;
    center
        .iter()
        .all(|value| value.is_finite())
        .then_some(center)
}

pub fn body_centroid_from_incidence(
    primal_vertices: &[Vector4<f64>],
    vertex_facet_incidence: &DMatrix<bool>,
) -> Option<Vector4<f64>> {
    let (_, centroid) =
        volume_and_centroid_from_incidence_f64(primal_vertices, vertex_facet_incidence).ok()?;
    centroid
        .iter()
        .all(|value| value.is_finite())
        .then_some(centroid)
}

pub fn translate_normalized_dual_vertices(
    dual_vertices: &[Vector4<f64>],
    center: Vector4<f64>,
) -> Option<TranslationCanonization> {
    if !center.iter().all(|value| value.is_finite()) {
        return None;
    }

    let mut min_denominator = f64::INFINITY;
    let mut translated = Vec::with_capacity(dual_vertices.len());
    for dual in dual_vertices {
        if !dual.iter().all(|value| value.is_finite()) {
            return None;
        }
        let denominator = 1.0 - dual.dot(&center);
        if !denominator.is_finite() || denominator <= MIN_TRANSLATION_DENOMINATOR {
            return None;
        }
        min_denominator = min_denominator.min(denominator);
        translated.push(dual / denominator);
    }

    Some(TranslationCanonization {
        center,
        min_denominator,
        dual_vertices: translated,
    })
}

pub fn volume_one_then_translate_by_body_centroid(
    dual_vertices: &[Vector4<f64>],
    primal_vertices: &[Vector4<f64>],
    vertex_facet_incidence: &DMatrix<bool>,
    volume: f64,
) -> Option<TranslationCanonization> {
    if !volume.is_finite() || volume <= 0.0 {
        return None;
    }

    let dual_scale = volume.powf(0.25);
    let primal_scale = dual_scale.recip();
    let scaled_duals = dual_vertices
        .iter()
        .map(|dual| dual * dual_scale)
        .collect::<Vec<_>>();
    let body_centroid = body_centroid_from_incidence(primal_vertices, vertex_facet_incidence)?;
    let center = body_centroid * primal_scale;
    translate_normalized_dual_vertices(&scaled_duals, center)
}

#[cfg(test)]
mod tests {
    use super::*;
    use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};

    fn simplex_cache() -> SysLandscapePolytopeCache {
        SysLandscapePolytopeCache::from_f64_dual_vertices(vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(-1.0, -1.0, -1.0, -1.0),
        ])
        .expect("simplex cache")
    }

    fn assert_translated_facets_are_normalized(
        polytope: &SysLandscapePolytopeCache,
        canonization: &TranslationCanonization,
        primal_scale: f64,
    ) {
        for (vertex_index, vertex) in polytope.vertices_f64.iter().enumerate() {
            let shifted = vertex * primal_scale - canonization.center;
            for facet_index in 0..polytope.facet_count() {
                let value = canonization.dual_vertices[facet_index].dot(&shifted);
                if polytope.vertex_facet_incidence[(vertex_index, facet_index)] {
                    assert!(
                        (value - 1.0).abs() < 1e-9,
                        "incident facet value {value} at vertex {vertex_index}, facet {facet_index}"
                    );
                } else {
                    assert!(
                        value <= 1.0 + 1e-9,
                        "nonincident facet value {value} at vertex {vertex_index}, facet {facet_index}"
                    );
                }
            }
        }
    }

    #[test]
    fn translation_formula_keeps_shifted_facets_normalized() {
        let polytope = simplex_cache();
        let center = average_primal_vertex_center(&polytope.vertices_f64).unwrap();
        let canonization =
            translate_normalized_dual_vertices(&polytope.dual_vertices_f64, center).unwrap();

        assert!(canonization.min_denominator > 0.0);
        assert_translated_facets_are_normalized(&polytope, &canonization, 1.0);
    }

    fn shifted_box_cache() -> SysLandscapePolytopeCache {
        SysLandscapePolytopeCache::from_f64_dual_vertices(vec![
            Vector4::new(1.0 / 3.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0 / 4.0, 0.0, 0.0),
            Vector4::new(0.0, -0.5, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 2.0 / 3.0, 0.0),
            Vector4::new(0.0, 0.0, -2.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0 / 3.0),
        ])
        .expect("shifted box cache")
    }

    #[test]
    fn body_centroid_matches_simplex_symmetry() {
        let polytope = simplex_cache();
        let centroid =
            body_centroid_from_incidence(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                .unwrap();

        assert!(centroid.norm() < 1e-12, "centroid {centroid:?}");
    }

    #[test]
    fn body_centroid_matches_shifted_box_center() {
        let polytope = shifted_box_cache();
        let centroid =
            body_centroid_from_incidence(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                .unwrap();

        assert!(
            (centroid - Vector4::new(1.0, 1.0, 0.5, -1.0)).norm() < 1e-10,
            "centroid {centroid:?}"
        );
    }

    #[test]
    fn volume_one_then_body_centroid_translation_keeps_shifted_facets_normalized() {
        let polytope = simplex_cache();
        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        let canonization = volume_one_then_translate_by_body_centroid(
            &polytope.dual_vertices_f64,
            &polytope.vertices_f64,
            &polytope.vertex_facet_incidence,
            volume,
        )
        .unwrap();

        assert!(canonization.min_denominator > 0.0);
        assert_translated_facets_are_normalized(&polytope, &canonization, volume.powf(-0.25));
    }

    #[test]
    fn volume_one_then_body_centroid_translation_handles_nonzero_center() {
        let polytope = shifted_box_cache();
        let volume = exact_volume_from_incidence_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        let primal_scale = volume.powf(-0.25);
        let canonization = volume_one_then_translate_by_body_centroid(
            &polytope.dual_vertices_f64,
            &polytope.vertices_f64,
            &polytope.vertex_facet_incidence,
            volume,
        )
        .unwrap();

        assert!(
            (canonization.center - Vector4::new(1.0, 1.0, 0.5, -1.0) * primal_scale).norm() < 1e-10
        );
        assert!(canonization.min_denominator > 0.0);
        assert_translated_facets_are_normalized(&polytope, &canonization, primal_scale);
    }

    #[test]
    fn boundary_or_exterior_center_is_rejected() {
        let polytope = simplex_cache();
        let boundary_center = Vector4::new(1.0, 0.0, 0.0, 0.0);
        assert!(
            translate_normalized_dual_vertices(&polytope.dual_vertices_f64, boundary_center)
                .is_none()
        );
    }
}
