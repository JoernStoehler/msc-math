use nalgebra::Vector4;

pub const PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductBlock {
    Q,
    P,
}

#[derive(Clone, Debug)]
pub struct ProductRoundingReport {
    pub status: ProductRoundingStatus,
    pub rounded_dual_vertices: Vec<Vector4<f64>>,
    pub q_facet_count: usize,
    pub p_facet_count: usize,
    pub max_minor_over_major: Option<f64>,
    pub max_abs_change: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductRoundingStatus {
    NotAttempted,
    Rounded,
    NotBlockStructured,
    InsufficientBlocks,
}

impl ProductRoundingStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotAttempted => "not_attempted",
            Self::Rounded => "rounded",
            Self::NotBlockStructured => "not_block_structured",
            Self::InsufficientBlocks => "insufficient_blocks",
        }
    }
}

impl ProductRoundingReport {
    pub fn not_attempted(dual_vertices: &[Vector4<f64>]) -> Self {
        Self {
            status: ProductRoundingStatus::NotAttempted,
            rounded_dual_vertices: dual_vertices.to_vec(),
            q_facet_count: 0,
            p_facet_count: 0,
            max_minor_over_major: None,
            max_abs_change: None,
        }
    }

    pub fn should_use_rounded_vertices(&self) -> bool {
        self.status == ProductRoundingStatus::Rounded
    }
}

pub fn round_product_blocks(dual_vertices: &[Vector4<f64>]) -> ProductRoundingReport {
    let mut q_count = 0usize;
    let mut p_count = 0usize;
    let mut rounded = Vec::with_capacity(dual_vertices.len());
    let mut max_minor_over_major = 0.0f64;
    let mut max_abs_change = 0.0f64;
    for (idx, vertex) in dual_vertices.iter().enumerate() {
        let Some(vertex_report) = round_product_block(idx, vertex) else {
            return ProductRoundingReport {
                status: ProductRoundingStatus::NotBlockStructured,
                rounded_dual_vertices: dual_vertices.to_vec(),
                q_facet_count: q_count,
                p_facet_count: p_count,
                max_minor_over_major: Some(max_minor_over_major),
                max_abs_change: Some(max_abs_change),
            };
        };
        match vertex_report.block {
            ProductBlock::Q => q_count += 1,
            ProductBlock::P => p_count += 1,
        }
        max_minor_over_major = max_minor_over_major.max(vertex_report.minor_over_major);
        max_abs_change = max_abs_change.max((vertex - vertex_report.rounded).abs().max());
        rounded.push(vertex_report.rounded);
    }
    let status = if q_count >= 3 && p_count >= 3 {
        ProductRoundingStatus::Rounded
    } else {
        ProductRoundingStatus::InsufficientBlocks
    };
    ProductRoundingReport {
        status,
        rounded_dual_vertices: if status == ProductRoundingStatus::Rounded {
            rounded
        } else {
            dual_vertices.to_vec()
        },
        q_facet_count: q_count,
        p_facet_count: p_count,
        max_minor_over_major: Some(max_minor_over_major),
        max_abs_change: Some(max_abs_change),
    }
}

pub(crate) fn round_known_product_dual_vertices(
    dual_vertices: &[Vector4<f64>],
) -> Vec<Vector4<f64>> {
    assert_known_product_dual_vertices(dual_vertices);
    let report = round_product_blocks(dual_vertices);
    assert_eq!(
        report.status,
        ProductRoundingStatus::Rounded,
        "known product dual vertices should have at least three q-facets and three p-facets"
    );
    report.rounded_dual_vertices
}

struct ProductVertexRounding {
    block: ProductBlock,
    rounded: Vector4<f64>,
    minor_over_major: f64,
}

fn round_product_block(_idx: usize, vertex: &Vector4<f64>) -> Option<ProductVertexRounding> {
    let q_norm = vertex.fixed_rows::<2>(0).norm();
    let p_norm = vertex.fixed_rows::<2>(2).norm();
    let major = q_norm.max(p_norm);
    let minor = q_norm.min(p_norm);
    if !(major.is_finite() && major > 0.0) {
        return None;
    }
    if minor > PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR * major {
        return None;
    }
    if q_norm >= p_norm {
        Some(ProductVertexRounding {
            block: ProductBlock::Q,
            rounded: Vector4::new(vertex[0], vertex[1], 0.0, 0.0),
            minor_over_major: minor / major,
        })
    } else {
        Some(ProductVertexRounding {
            block: ProductBlock::P,
            rounded: Vector4::new(0.0, 0.0, vertex[2], vertex[3]),
            minor_over_major: minor / major,
        })
    }
}

fn assert_known_product_dual_vertices(dual_vertices: &[Vector4<f64>]) {
    dual_vertices
        .iter()
        .enumerate()
        .for_each(|(idx, vertex)| assert_known_product_dual_vertex(idx, vertex));
}

fn assert_known_product_dual_vertex(idx: usize, vertex: &Vector4<f64>) {
    let q_norm = vertex.fixed_rows::<2>(0).norm();
    let p_norm = vertex.fixed_rows::<2>(2).norm();
    let major = q_norm.max(p_norm);
    let minor = q_norm.min(p_norm);
    assert!(
        major.is_finite() && major > 0.0,
        "product facet {idx} has no dominant finite block: {vertex:?}"
    );
    assert!(
        minor <= PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR * major,
        "product facet {idx} is not block-structured enough to round: q_norm={q_norm}, p_norm={p_norm}, vertex={vertex:?}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_product_rounding_removes_tiny_off_block_drift() {
        let rounded = round_known_product_dual_vertices(&[
            Vector4::new(1.0, 2.0, 1e-14, -2e-14),
            Vector4::new(-2.0, 1.0, -1e-14, 1e-14),
            Vector4::new(1.0, -2.0, 2e-14, -1e-14),
            Vector4::new(1e-14, -2e-14, 1.0, 2.0),
            Vector4::new(-1e-14, 1e-14, -2.0, 1.0),
            Vector4::new(2e-14, -1e-14, 1.0, -2.0),
        ]);
        assert_eq!(rounded[0], Vector4::new(1.0, 2.0, 0.0, 0.0));
        assert_eq!(rounded[3], Vector4::new(0.0, 0.0, 1.0, 2.0));
    }

    #[test]
    #[should_panic(expected = "not block-structured enough")]
    fn known_product_rounding_rejects_mixed_facets() {
        let _ = round_known_product_dual_vertices(&[Vector4::new(1.0, 0.0, 0.1, 0.0)]);
    }

    #[test]
    fn tolerant_rounding_reports_mixed_facets_without_changing_input() {
        let input = [Vector4::new(1.0, 0.0, 0.1, 0.0)];
        let report = round_product_blocks(&input);
        assert_eq!(report.status, ProductRoundingStatus::NotBlockStructured);
        assert_eq!(report.rounded_dual_vertices, input);
    }
}
