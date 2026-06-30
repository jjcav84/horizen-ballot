//! # horizen-ballot — Anonymous On-Chain Voting on Horizen Base L3
//!
//! Horizen adaptation of [zk-ballot](https://github.com/jjcav84/zk-ballot) —
//! replaces on-chain verify costs with ZEN token staking and ZenKinetic
//! privacy gate integration.
//!
//! ## How it works
//!
//! 1. **Voter** generates a Halo2 ZK proof of Merkle membership + vote (same as zk-ballot)
//! 2. **ZenKinetic gate** scores the proof's negentropy and determines fees
//! 3. **ZEN staking** grants voting access — Pro tier (1,000 ZEN) required
//! 4. **Vote settles** on Horizen Base L3 with privacy-preserving fee = 0%
//!
//! ## Quick Start
//!
//! ```rust
//! use horizen_ballot::VotingSession;
//!
//! let session = VotingSession::new(4, 20, 0.95, 1_000.0);
//! let result = session.evaluate();
//! println!("Gate: {:?}", result.gate_decision);
//! println!("Negentropy: {:.1} bits", result.negentropy_bits);
//! println!("Anonymity set: {}", result.anonymity_set);
//! ```

pub mod session;
pub mod types;

pub use session::VotingSession;
pub use types::VoteResult;

/// Re-export zenkinetic gate for direct access.
pub use zenkinetic::{PrivacyGate, TransactionProfile, GateDecision};
/// Re-export negentropy for scoring.
pub use negentropy::{Negentropy, RouteEnergy, Committor};
