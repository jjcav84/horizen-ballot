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
