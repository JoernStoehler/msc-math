use super::PolytopeFeatureInputRow;

pub struct SysFields {
    pub sys: f64,
}

pub fn compute(row: &PolytopeFeatureInputRow) -> SysFields {
    SysFields { sys: row.sys }
}
