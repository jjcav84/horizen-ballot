//! Types for Horizen anonymous voting.

use serde::{Deserialize, Serialize};

/// Result of a Horizen vote evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResult {
    /// ZenKinetic gate decision
    pub gate_decision: String,
    /// Fee in basis points (after ZEN stake discount)
    pub fee_bps: u32,
    /// Negentropy extracted by the vote proof (bits)
    pub negentropy_bits: f64,
    /// Privacy alignment score (0..1)
    pub alignment: f64,
    /// Committor probability (vote validity confidence)
    pub committor: f64,
    /// ZEN stake tier
    pub stake_tier: String,
    /// Anonymity set size (2^tree_depth)
    pub anonymity_set: u64,
    /// Whether ZEN staking grants voting access
    pub access_granted: bool,
    /// Merkle tree depth
    pub tree_depth: usize,
}
