use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const PACKET_VERSION: &str = "s0-equal-budget-product-search-v1";
pub const TARGET_BUDGET: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arm {
    Iid,
    MultistartBranchLocalPhase0,
    DiagonalCem,
}

impl Arm {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iid => "iid",
            Self::MultistartBranchLocalPhase0 => "multistart_branch_local_phase0",
            Self::DiagonalCem => "diagonal_cem",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidateIdentity<'a> {
    pub packet_version: &'a str,
    pub master_seed: u64,
    pub replicate: usize,
    pub arm: Arm,
    pub generation: Option<usize>,
    pub trajectory: Option<usize>,
    pub iteration: Option<usize>,
    pub proposal_index: usize,
    pub construction_attempt: usize,
}

pub fn candidate_id(identity: &CandidateIdentity<'_>) -> String {
    let bytes = serde_json::to_vec(identity).expect("candidate identity serializes");
    let digest = format!("{:x}", Sha256::digest(bytes));
    format!("s0v1-{}", &digest[..24])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalRole {
    Iid,
    CemPopulation,
    LocalStart,
    WithinStep,
    Overshoot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheStatus {
    Miss,
    Hit,
    FailedMiss,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProposalMeta {
    pub candidate_id: String,
    pub arm: Arm,
    pub replicate: usize,
    pub generation: Option<usize>,
    pub trajectory: Option<usize>,
    pub iteration: Option<usize>,
    pub proposal_index: usize,
    pub construction_attempt: usize,
    /// Zero-based arm/replicate construction event. The union of accepted
    /// target rows and rejected-construction rows is exactly `0..N`.
    pub construction_sequence_index: usize,
    pub construction_rejections_before: usize,
    pub role: ProposalRole,
    pub parent_candidate_id: Option<String>,
    pub elite_set_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_ids_are_deterministic_and_field_sensitive() {
        let base = CandidateIdentity {
            packet_version: PACKET_VERSION,
            master_seed: 202607110001,
            replicate: 0,
            arm: Arm::Iid,
            generation: None,
            trajectory: None,
            iteration: None,
            proposal_index: 0,
            construction_attempt: 0,
        };
        assert_eq!(candidate_id(&base), candidate_id(&base));
        let mut changed = base.clone();
        changed.proposal_index = 1;
        assert_ne!(candidate_id(&base), candidate_id(&changed));
    }
}
