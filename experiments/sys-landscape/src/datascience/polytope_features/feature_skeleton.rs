use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;

pub struct SkeletonFields {
    pub vertex_count: usize,
    pub edge_count: usize,
    pub ridge_count: usize,
    pub is_simple: bool,
    pub simple_vertex_fraction: f64,
    pub edge_density: f64,
    pub vertex_incident_facets_mean: f64,
    pub vertex_incident_facets_std: f64,
    pub vertex_incident_facets_min: f64,
    pub vertex_incident_facets_max: f64,
    pub vertex_degree_mean: f64,
    pub vertex_degree_std: f64,
    pub vertex_degree_min: f64,
    pub vertex_degree_max: f64,
    pub ridge_size_mean: f64,
    pub ridge_size_std: f64,
    pub ridge_size_min: f64,
    pub ridge_size_max: f64,
    pub facet_vertex_count_mean: f64,
    pub facet_vertex_count_std: f64,
    pub facet_vertex_count_min: f64,
    pub facet_vertex_count_max: f64,
    pub facet_neighbor_count_mean: f64,
    pub facet_neighbor_count_std: f64,
    pub facet_neighbor_count_min: f64,
    pub facet_neighbor_count_max: f64,
}

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

pub fn compute(polytope: &Polytope4D, skeleton: &Skeleton, facet_count: usize) -> SkeletonFields {
    let vertex_count = polytope.vertices().len();
    let edge_count = skeleton.edges.len();
    let ridge_count = skeleton.ridges.len();
    let vertex_incident_facets = skeleton
        .vertex_facets
        .iter()
        .map(|facets| facets.len() as f64)
        .collect::<Vec<_>>();
    let simple_vertices = vertex_incident_facets
        .iter()
        .filter(|&&count| (count - 4.0).abs() < f64::EPSILON)
        .count();
    let simple_vertex_fraction = if vertex_count > 0 {
        simple_vertices as f64 / vertex_count as f64
    } else {
        0.0
    };
    let is_simple = simple_vertices == vertex_count;
    let mut vertex_degrees = vec![0usize; vertex_count];
    for edge in &skeleton.edges {
        vertex_degrees[edge[0]] += 1;
        vertex_degrees[edge[1]] += 1;
    }
    let vertex_degrees = vertex_degrees
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();
    let ridge_sizes = skeleton
        .ridges
        .iter()
        .map(|ridge| ridge.vertices.len() as f64)
        .collect::<Vec<_>>();
    let mut facet_vertex_counts = vec![0usize; facet_count];
    for facets in &skeleton.vertex_facets {
        for &facet in facets {
            facet_vertex_counts[facet] += 1;
        }
    }
    let facet_vertex_counts = facet_vertex_counts
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();
    let facet_neighbor_counts = (0..facet_count)
        .map(|facet| {
            (0..facet_count)
                .filter(|&other| polytope.vertex_adjacency()[(facet, other)])
                .count() as f64
        })
        .collect::<Vec<_>>();
    let edge_density = if vertex_count >= 2 {
        (2.0 * edge_count as f64) / ((vertex_count * (vertex_count - 1)) as f64)
    } else {
        0.0
    };

    let (vertex_incident_facets_mean, vertex_incident_facets_std, vertex_incident_facets_min, vertex_incident_facets_max) =
        stats_or_zero(&vertex_incident_facets);
    let (vertex_degree_mean, vertex_degree_std, vertex_degree_min, vertex_degree_max) =
        stats_or_zero(&vertex_degrees);
    let (ridge_size_mean, ridge_size_std, ridge_size_min, ridge_size_max) =
        stats_or_zero(&ridge_sizes);
    let (facet_vertex_count_mean, facet_vertex_count_std, facet_vertex_count_min, facet_vertex_count_max) =
        stats_or_zero(&facet_vertex_counts);
    let (facet_neighbor_count_mean, facet_neighbor_count_std, facet_neighbor_count_min, facet_neighbor_count_max) =
        stats_or_zero(&facet_neighbor_counts);

    SkeletonFields {
        vertex_count,
        edge_count,
        ridge_count,
        is_simple,
        simple_vertex_fraction,
        edge_density,
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
        vertex_degree_mean,
        vertex_degree_std,
        vertex_degree_min,
        vertex_degree_max,
        ridge_size_mean,
        ridge_size_std,
        ridge_size_min,
        ridge_size_max,
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
    }
}
