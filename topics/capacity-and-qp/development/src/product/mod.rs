mod remove_near_redundant_facets;
mod round_blocks;

pub use crate::selected_route::product::{
    audit_product_closure_capacity_binary64, solve_product_closure_capacity_exact_binary64,
    solve_product_closure_capacity_hybrid, ProductClosureAuditReport, ProductClosureCapacityReport,
    ProductClosureError, ProductClosureNumerics, ProductClosureStats, ProductClosureWinner,
};
pub use remove_near_redundant_facets::{
    remove_near_redundant_facets, ProductFacetRemoval, ProductFacetRemovalReport,
    ProductFacetRemovalStatus,
};
pub(crate) use round_blocks::round_known_product_dual_vertices;
pub use round_blocks::{
    round_blocks, ProductBlock, ProductRoundingReport, ProductRoundingStatus,
    PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR,
};
