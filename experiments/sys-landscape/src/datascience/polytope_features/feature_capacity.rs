use super::PolytopeFeatureInputRow;

pub struct CapacityFields {
    pub capacity: f64,
    pub capacity_iterations: Option<u64>,
    pub capacity_source: String,
}

pub fn compute(row: &PolytopeFeatureInputRow) -> CapacityFields {
    CapacityFields {
        capacity: row.capacity,
        capacity_iterations: row.capacity_iterations,
        capacity_source: row.capacity_source.clone(),
    }
}
