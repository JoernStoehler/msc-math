use euclidean_polytopes::{
    edges_from_incidence, two_faces_from_incidence, vertex_facets_from_incidence, TwoFace,
};
use nalgebra::DMatrix;

#[test]
fn simplex_incidence_has_expected_edges_and_triangular_2faces() {
    let incidence = simplex_4d_incidence();

    let vertex_facets = vertex_facets_from_incidence(&incidence);
    assert_eq!(
        vertex_facets,
        vec![
            vec![1, 2, 3, 4],
            vec![0, 2, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 4],
            vec![0, 1, 2, 3]
        ]
    );

    let edges = edges_from_incidence(&incidence);
    assert_eq!(edges.len(), 10);
    assert_sorted_edges(&edges);

    let two_faces = two_faces_from_incidence(&incidence);
    assert_eq!(two_faces.len(), 10);
    assert_sorted_2faces(&two_faces);
    assert!(two_faces
        .iter()
        .all(|two_face| two_face.vertices.len() == 3));
}

#[test]
fn hypercube_incidence_has_expected_edges_and_square_2faces() {
    let incidence = hypercube_4d_incidence();

    let vertex_facets = vertex_facets_from_incidence(&incidence);
    assert!(vertex_facets
        .iter()
        .all(|facets| facets.len() == 4 && facets.windows(2).all(|pair| pair[0] < pair[1])));

    let edges = edges_from_incidence(&incidence);
    assert_eq!(edges.len(), 32);
    assert_sorted_edges(&edges);

    let two_faces = two_faces_from_incidence(&incidence);
    assert_eq!(two_faces.len(), 24);
    assert_sorted_2faces(&two_faces);
    assert!(two_faces
        .iter()
        .all(|two_face| two_face.vertices.len() == 4));
}

#[test]
fn faces_output_is_deterministically_sorted() {
    let incidence = DMatrix::from_row_slice(
        5,
        4,
        &[
            true, true, true, false, //
            true, true, false, true, //
            true, true, true, true, //
            true, false, true, true, //
            false, true, true, true,
        ],
    );

    let edges = edges_from_incidence(&incidence);
    assert_sorted_edges(&edges);

    let two_faces = two_faces_from_incidence(&incidence);
    assert_sorted_2faces(&two_faces);
}

#[test]
fn incidence_with_no_three_vertex_facet_intersection_has_no_2faces() {
    let incidence = DMatrix::from_row_slice(
        4,
        4,
        &[
            true, true, false, false, //
            true, false, true, false, //
            false, true, false, true, //
            false, false, true, true,
        ],
    );

    assert!(two_faces_from_incidence(&incidence).is_empty());
}

#[test]
fn empty_faces_incidence_returns_empty_outputs() {
    let incidence = DMatrix::from_element(0, 0, false);

    assert!(vertex_facets_from_incidence(&incidence).is_empty());
    assert!(edges_from_incidence(&incidence).is_empty());
    assert!(two_faces_from_incidence(&incidence).is_empty());
}

fn simplex_4d_incidence() -> DMatrix<bool> {
    DMatrix::from_row_slice(
        5,
        5,
        &[
            false, true, true, true, true, //
            true, false, true, true, true, //
            true, true, false, true, true, //
            true, true, true, false, true, //
            true, true, true, true, false,
        ],
    )
}

fn hypercube_4d_incidence() -> DMatrix<bool> {
    DMatrix::from_fn(16, 8, |vertex_index, facet_index| {
        let coordinate_index = facet_index / 2;
        let positive_facet = facet_index % 2 == 0;
        let positive_vertex = ((vertex_index >> coordinate_index) & 1) == 1;
        positive_vertex == positive_facet
    })
}

fn assert_sorted_edges(edges: &[[usize; 2]]) {
    for edge in edges {
        assert!(edge[0] < edge[1], "edge is not sorted: {edge:?}");
    }
    assert!(
        edges.windows(2).all(|pair| pair[0] < pair[1]),
        "edges are not deterministic increasing pairs: {edges:?}"
    );
}

fn assert_sorted_2faces(two_faces: &[TwoFace]) {
    for two_face in two_faces {
        assert!(
            two_face.facets[0] < two_face.facets[1],
            "2-face facets are not sorted: {two_face:?}"
        );
        assert!(
            two_face.vertices.windows(2).all(|pair| pair[0] < pair[1]),
            "2-face vertices are not increasing: {two_face:?}"
        );
    }
    assert!(
        two_faces
            .windows(2)
            .all(|pair| pair[0].facets < pair[1].facets),
        "2-faces are not deterministic increasing facet pairs: {two_faces:?}"
    );
}
