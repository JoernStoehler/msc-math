mod remove_near_redundant_facets;
mod round_blocks;

pub use remove_near_redundant_facets::{
    remove_near_redundant_facets, ProductFacetRemoval, ProductFacetRemovalReport,
    ProductFacetRemovalStatus,
};
pub(crate) use round_blocks::round_known_product_dual_vertices;
pub use round_blocks::{
    round_blocks, ProductBlock, ProductRoundingReport, ProductRoundingStatus,
    PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR,
};
