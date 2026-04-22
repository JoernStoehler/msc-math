use super::PolytopeFeatureInputRow;

pub struct VolumeFields {
    pub volume: f64,
}

pub fn compute(row: &PolytopeFeatureInputRow) -> VolumeFields {
    VolumeFields { volume: row.volume }
}
