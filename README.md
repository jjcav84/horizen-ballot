<p align="center">
  <a href="https://www.orkidlabs.com"><img src="assets/logo.png" alt="Orkid Labs" width="220" /></a>
</p>

# horizen-ballot — Anonymous On-Chain Voting on Horizen Base L3

> *Vote without revealing who you are or how you voted. Pay zero fees for
> preserving privacy.*
>
> **By [Orkid Labs](https://www.orkidlabs.com)** — privacy-first crypto engineering

Horizen adaptation of [zk-ballot](https://github.com/jjcav84/zk-ballot) —
replaces on-chain verify costs with ZEN token staking and
[ZenKinetic](https://github.com/jjcav84/zenkinetic) privacy gate
integration.

[![License: MIT](https://img.shields.io/badge/License-MIT-a78bfa.svg)](LICENSE)
[![Horizen](https://img.shields.io/badge/Horizen-Base%20L3-ff6b35.svg)](https://horizen.org)
[![negentropy](https://img.shields.io/badge/powered%20by-negentropy-a78bfa.svg)](https://github.com/jjcav84/negentropy)

## How it works

1. **Voter** generates a Halo2 ZK proof of Merkle membership + vote (same as zk-ballot)
2. **ZenKinetic gate** scores the proof's negentropy — privacy-preserving = 0% fee
3. **ZEN staking** grants voting access — Pro tier (1,000 ZEN) required
4. **Vote settles** on Horizen Base L3 with full privacy

The ZK circuit proves, without revealing identity or vote:
- **Merkle membership** — voter is in the registry
- **Nullifier** — prevents double-voting
- **Boolean vote** — vote is valid (0 or 1)
- **Vote commitment** — binds proof to specific ballot

## Quick Start

```rust
use horizen_ballot::VotingSession;

let session = VotingSession::new(4, 20, 0.95, 1_000.0);
let result = session.evaluate();

println!("Gate: {:?}", result.gate_decision);    // Allow
println!("Fee: {} bps", result.fee_bps);          // 0
println!("Negentropy: {:.1} bits", result.negentropy_bits);  // 80.0
println!("Anonymity set: {}", result.anonymity_set);  // 16
```

## ZEN Token Utility

| Stake Tier | Min ZEN | Fee Discount | Voting Access |
|-----------|---------|-------------|--------------|
| Basic | 100 | 25% off | — |
| Pro | 1,000 | 50% off | ✓ |
| Max | 10,000 | 75% off | ✓ |

## Architecture

```
horizen-ballot
├── depends on → negentropy (physics scoring)
├── depends on → zenkinetic (privacy gate + ZEN staking)
├── adapts → zk-ballot (Halo2 voting circuit)
└── deploys on → Horizen Base L3
```

## Origin

This is the Horizen-native adaptation of [zk-ballot](https://github.com/jjcav84/zk-ballot).
The Halo2 circuit and proof generation are the same; the chain integration
changes from direct on-chain verify to Horizen Base L3 with ZEN staking
and ZenKinetic privacy gating.

## Thrive Horizen Genesis Program (#38) — Grant Plan

### Ecosystem value proposition

horizen-ballot brings anonymous on-chain voting to Horizen Base L3. Each vote generates a Halo2 ZK proof scored by the negentropy engine and gated by ZenKinetic. Privacy-preserving votes settle at 0% fee; ZEN staking (Pro tier, 1,000 ZEN) grants voting access.

### Milestone roadmap

Progressive achievement over 150 days, following Thrive's Horizen Genesis Program milestone structure.

**Application Requirements (10% unlocked at approval)**:
- ✅ Detailed technical architecture showcasing privacy technology implementation
- ✅ Clear demonstration of ZK integration (Halo2 PLONK + IPA commitment)
- ✅ Privacy-focused user experience design and privacy preservation methodology
- ✅ ZEN token utility and ecosystem value proposition (via ZenKinetic integration)
- ✅ Team background with relevant cryptographic and privacy expertise

**Milestone 1 (20% unlocked) — 45 days post approval**:
- Smart contract(s) deployed on Horizen testnet with privacy features functional
- Core privacy mechanisms implemented and audited
- Technical documentation published with privacy proofs
- Beta testing with privacy verification and user feedback

**Milestone 2 (30% unlocked) — 90 days post approval**:
- Mainnet deployment with full privacy feature set operational
- Privacy compliance documentation and audit reports
- Integration with Horizen ecosystem tools and infrastructure
- Early traction metrics (choose one):
  - TVL: $50K+ in ZEN locked in smart contracts, staking, or liquidity pools
  - Volume: 10,000+ transactions demonstrating privacy preservation
  - Unique wallets: 250+ verified users utilizing privacy features

**Milestone 3 (40% unlocked) — 150 days post approval**:
- Scale metrics (choose one):
  - TVL: $100K+ in ZEN locked in smart contracts, staking, or liquidity pools
  - Volume: 20,000+ transactions demonstrating privacy preservation
  - Unique Wallets: 500+ verified users utilizing privacy features

## Ecosystem

Part of the negentropy-powered privacy stack for Horizen:

- [negentropy](https://github.com/jjcav84/negentropy) — shared physics engine
- [zenkinetic](https://github.com/jjcav84/zenkinetic) — thermodynamic privacy gate
- [horizen-age](https://github.com/jjcav84/horizen-age) — age verification
- [horizen-attest](https://github.com/jjcav84/horizen-attest) — attestations
- [horizen-ballot](https://github.com/jjcav84/horizen-ballot) — **this repo**

## About

Built by [Orkid Labs](https://www.orkidlabs.com) — a privacy-first crypto
engineering lab building thermodynamic infrastructure for decentralized
systems.

## License

MIT — see [LICENSE](LICENSE).
