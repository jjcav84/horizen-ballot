//! Voting session — evaluates an anonymous vote through the ZenKinetic gate.

use crate::types::VoteResult;
use zenkinetic::{PrivacyGate, TransactionProfile};
use zenkinetic::staking::StakeTier;

/// Configuration for a Horizen anonymous voting session.
///
/// Adapts zk-ballot's `BallotPotential` to the Horizen ecosystem:
/// - ZEN staking replaces on-chain verify costs
/// - ZenKinetic gate scores privacy alignment
/// - Negentropy: N = constraints × tree_depth (same as zk-ballot)
#[derive(Debug, Clone)]
pub struct VotingSession {
    /// Merkle tree depth (determines anonymity set: 2^depth)
    pub tree_depth: usize,
    /// Circuit constraint count (zk-ballot default: 20)
    pub constraint_count: u64,
    /// Registry trust score (0..1)
    pub registry_trust: f64,
    /// ZEN tokens staked by the voter
    pub zen_staked: f64,
    /// Vote age in seconds (recency decay)
    pub vote_age_secs: f64,
    /// Proof generation latency in ms
    pub proof_latency_ms: u64,
}

impl Default for VotingSession {
    fn default() -> Self {
        Self {
            tree_depth: 4,           // VOTE_TREE_DEPTH = 4 → 16 voters
            constraint_count: 20,    // zk-ballot: ~20 constraints
            registry_trust: 0.95,
            zen_staked: 1_000.0,     // Pro tier required for voting
            vote_age_secs: 0.0,
            proof_latency_ms: 1000,
        }
    }
}

impl VotingSession {
    pub fn new(tree_depth: usize, constraint_count: u64, registry_trust: f64, zen_staked: f64) -> Self {
        Self {
            tree_depth,
            constraint_count,
            registry_trust,
            zen_staked,
            ..Default::default()
        }
    }

    /// Evaluate the vote through the ZenKinetic privacy gate.
    ///
    /// This replaces zk-ballot's `BallotPotential::energy()` + on-chain
    /// verify with a ZenKinetic gate evaluation.
    pub fn evaluate(&self) -> VoteResult {
        // Anonymity set: 2^tree_depth
        let anonymity_set = 1u64 << self.tree_depth;

        // Build a ZenKinetic transaction profile for the vote
        let profile = TransactionProfile {
            has_zk_proof: true,
            constraint_count: self.constraint_count,
            anonymity_set_bits: self.tree_depth as u64,
            proof_age_secs: self.vote_age_secs,
            proof_latency_ms: self.proof_latency_ms,
            verify_latency_ms: 27, // Horizen L3 verify is fast
            zen_staked: self.zen_staked,
        };

        let gate = PrivacyGate::evaluate(&profile);

        // Check ZEN staking access — Pro tier required for anonymous voting
        let stake_tier = StakeTier::from_staked(self.zen_staked);
        let access_granted = stake_tier.grants_anonymous_voting();

        // Negentropy: N = constraints × tree_depth
        // Expressed as from_constraints(n, 2^depth) since log₂(2^depth) = depth
        let negentropy_bits =
            negentropy::Negentropy::from_constraints(self.constraint_count, anonymity_set).bits();

        VoteResult {
            gate_decision: format!("{:?}", gate.decision),
            fee_bps: gate.discounted_fee_bps,
            negentropy_bits,
            alignment: gate.alignment,
            committor: gate.committor,
            stake_tier: stake_tier.label().to_string(),
            anonymity_set,
            access_granted,
            tree_depth: self.tree_depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vote_aligned() {
        let session = VotingSession::default();
        let result = session.evaluate();

        assert_eq!(result.gate_decision, "Allow");
        assert!(result.negentropy_bits > 0.0);
        assert!(result.access_granted);
        assert_eq!(result.anonymity_set, 16);
    }

    #[test]
    fn test_insufficient_stake_denied() {
        let session = VotingSession {
            zen_staked: 500.0, // Basic tier — not enough for voting
            ..Default::default()
        };
        let result = session.evaluate();

        assert!(!result.access_granted);
    }

    #[test]
    fn test_deeper_tree_more_negentropy() {
        let shallow = VotingSession::new(4, 20, 0.9, 1_000.0).evaluate();
        let deep = VotingSession::new(8, 20, 0.9, 1_000.0).evaluate();

        assert!(deep.negentropy_bits > shallow.negentropy_bits);
        assert_eq!(deep.anonymity_set, 256);
    }

    #[test]
    fn test_stale_vote_decays() {
        let fresh = VotingSession::default().evaluate();
        let stale = VotingSession {
            vote_age_secs: 7200.0,
            ..Default::default()
        }
        .evaluate();

        assert!(stale.alignment < fresh.alignment);
    }

    #[test]
    fn test_negentropy_formula() {
        // 20 constraints, depth 4: N = 20 * 4 = 80 bits
        let session = VotingSession::default();
        let result = session.evaluate();
        let expected = 20.0 * 4.0;
        assert!((result.negentropy_bits - expected).abs() < 0.01);
    }
}
