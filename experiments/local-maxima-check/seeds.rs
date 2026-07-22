use euclidean_polytopes::polar_vertices_exact_rational;
use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::Vector4;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use symplectic::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

#[derive(Clone, Debug)]
pub(crate) struct ProductSpec {
    pub(crate) q_sides: usize,
    pub(crate) p_sides: usize,
    pub(crate) theta_rad: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct Seed {
    pub(crate) id: &'static str,
    pub(crate) role: &'static str,
    pub(crate) source: &'static str,
    pub(crate) expected_sys: f64,
    pub(crate) dual_vertices: Vec<Vector4<f64>>,
    pub(crate) product: Option<ProductSpec>,
}

pub(crate) fn known_equality_seeds() -> Vec<Seed> {
    let pentagon_c0 = (5.0 + 2.0 * 5.0_f64.sqrt()) / 10.0;
    let pentagon_threshold = pentagon_c0.sqrt().acos();
    vec![
        product_seed(
            "pentagon_threshold_control",
            "expected_positive_control",
            "thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex",
            5,
            5,
            pentagon_threshold,
        ),
        product_seed(
            "triangle_hexagon_theta0",
            "target",
            "experiments/regular-products/rotated-regular-products/lagrangian-products-3x6-6deg.jsonl",
            3,
            6,
            0.0,
        ),
        product_seed(
            "square_square_pi_over_4",
            "target",
            "experiments/regular-products/rotated-regular-products/lagrangian-products-4x4-6deg.jsonl",
            4,
            4,
            std::f64::consts::FRAC_PI_4,
        ),
        ch2021_seed(),
    ]
}

pub(crate) fn product_dual_vertices(spec: &ProductSpec) -> Vec<Vector4<f64>> {
    let (q_normals, q_heights) = regular_polygon_2d(spec.q_sides, 1.0);
    let (p_normals_0, p_heights_0) = regular_polygon_2d(spec.p_sides, 1.0);
    let (p_normals, p_heights) = rotate_polygon_2d(&p_normals_0, &p_heights_0, spec.theta_rad);
    SysLandscapePolytopeCache::from_lagrangian_product(
        &q_normals, &q_heights, &p_normals, &p_heights,
    )
    .expect("regular product seed must construct")
    .dual_vertices_f64
}

fn product_seed(
    id: &'static str,
    role: &'static str,
    source: &'static str,
    q_sides: usize,
    p_sides: usize,
    theta_rad: f64,
) -> Seed {
    let product = ProductSpec {
        q_sides,
        p_sides,
        theta_rad,
    };
    Seed {
        id,
        role,
        source,
        expected_sys: 1.0,
        dual_vertices: product_dual_vertices(&product),
        product: Some(product),
    }
}

fn ch2021_seed() -> Seed {
    // CH2021 displays these six primal vertices in (q1,q2,p1,p2) order.
    // Translation by their exact arithmetic centroid places the origin in the
    // interior; the polar vertices are the normalized halfspace rows.
    let source_vertices = [
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 1],
        [0, -1, 1, 0],
        [-1, -1, 0, 1],
    ]
    .into_iter()
    .map(|row| Vector4::from_fn(|i, _| integer_ratio(row[i])))
    .collect::<Vec<_>>();
    let mut centroid = Vector4::from_element(BigRational::zero());
    for vertex in &source_vertices {
        centroid += vertex;
    }
    centroid /= integer_ratio(source_vertices.len() as i64);
    let translated = source_vertices
        .iter()
        .map(|vertex| vertex - &centroid)
        .collect::<Vec<_>>();
    let polar = polar_vertices_exact_rational(&translated);
    let dual_vertices = polar
        .vertices
        .iter()
        .map(|row| Vector4::from_fn(|i, _| row[i].to_f64().expect("small CH rational")))
        .collect::<Vec<_>>();
    assert_eq!(dual_vertices.len(), 9, "CH seed must have nine facets");
    Seed {
        id: "ch2021_six_vertex",
        role: "target",
        source: "experiments/verification/ch2021-six-vertex/report.json",
        expected_sys: 1.0,
        dual_vertices,
        product: None,
    }
}

fn integer_ratio(value: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(value))
}
