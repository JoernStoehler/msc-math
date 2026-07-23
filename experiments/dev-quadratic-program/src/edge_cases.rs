use nalgebra::Vector4;
use symplectic::known_polytopes;

use crate::ScanCase;

pub fn edge_fixture_cases() -> Vec<ScanCase> {
    vec![
        duplicate_dual_vertices_case(),
        missing_origin_interior_case(),
        dual_vertex_norm_out_of_range_case(),
        primal_vertex_norm_out_of_range_case(),
        drifted_product_case(),
        near_redundant_product_case(),
    ]
}

fn dual_vertex_norm_out_of_range_case() -> ScanCase {
    let scale = 1e-4;
    ScanCase {
        family: "edge_invalid".to_string(),
        source_id: "edge:dual_vertex_norm_out_of_range".to_string(),
        input_source: "edge_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(5),
        dual_vertices: vec![
            Vector4::new(scale, 0.0, 0.0, 0.0),
            Vector4::new(0.0, scale, 0.0, 0.0),
            Vector4::new(0.0, 0.0, scale, 0.0),
            Vector4::new(0.0, 0.0, 0.0, scale),
            Vector4::repeat(-scale),
        ],
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

fn primal_vertex_norm_out_of_range_case() -> ScanCase {
    let scale = 1e-3;
    ScanCase {
        family: "edge_invalid".to_string(),
        source_id: "edge:primal_vertex_norm_out_of_range".to_string(),
        input_source: "edge_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(5),
        dual_vertices: vec![
            Vector4::new(scale, 0.0, 0.0, 0.0),
            Vector4::new(0.0, scale, 0.0, 0.0),
            Vector4::new(0.0, 0.0, scale, 0.0),
            Vector4::new(0.0, 0.0, 0.0, scale),
            Vector4::repeat(-scale),
        ],
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

fn duplicate_dual_vertices_case() -> ScanCase {
    ScanCase {
        family: "edge_invalid".to_string(),
        source_id: "edge:duplicate_dual_vertices".to_string(),
        input_source: "edge_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(5),
        dual_vertices: vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ],
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

fn missing_origin_interior_case() -> ScanCase {
    ScanCase {
        family: "edge_invalid".to_string(),
        source_id: "edge:missing_origin_interior".to_string(),
        input_source: "edge_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(5),
        dual_vertices: vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(1.0, 1.0, 1.0, 1.0),
        ],
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

fn drifted_product_case() -> ScanCase {
    let fixture = known_polytopes::lagrangian_triangle_product();
    let mut dual_vertices = fixture.dual_vertices_f64.clone();
    for vertex in &mut dual_vertices {
        if vertex[2] == 0.0 && vertex[3] == 0.0 {
            vertex[2] = 1e-14;
        } else {
            vertex[0] = -1e-14;
        }
    }
    ScanCase {
        family: "edge_product".to_string(),
        source_id: "edge:drifted_product_rounding".to_string(),
        input_source: "edge_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(dual_vertices.len()),
        dual_vertices,
        audit_capacity_label: Some(fixture.capacity),
        artifact_capacity_label: Some(fixture.capacity),
        audit_sigma_label: None,
    }
}

fn near_redundant_product_case() -> ScanCase {
    let eps = 1e-8;
    ScanCase {
        family: "edge_product".to_string(),
        source_id: "edge:near_redundant_product".to_string(),
        input_source: "edge_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(8),
        dual_vertices: vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(1.0, eps, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -1.0, -1.0),
        ],
        audit_capacity_label: Some(1.0),
        artifact_capacity_label: Some(1.0),
        audit_sigma_label: None,
    }
}
