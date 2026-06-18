# zk-wavescout-contracts

Soroban smart contract framework for ZK-verified bounty puzzles. Holds bounty funds in escrow and executes proof verification.

## 📋 Quick Start

```bash
cargo check
soroban contract build
```

## 🏗️ Project Structure

```
src/
├── lib.rs          # Core contract (40% complete)
├── verification.rs # ISSUE: ZK proof verification module
├── events.rs       # ISSUE: Event emission utilities
└── storage.rs      # ISSUE: Persistent storage helpers
```

## ✅ Implemented (40%)

- `create_challenge()` — Lock bounty in escrow
- `claim_bounty()` — Submit proof and claim reward
- `get_challenge()` — Query challenge data
- `is_challenge_solved()` — Check claim status

## ❌ ISSUE (60%) - Create Issues

| Issue | Task | Priority |
|-------|------|----------|
| #1 | Implement BN254 pairing ZK verifier | Critical |
| #2 | Add proof replay protection (nonce) | High |
| #3 | Extract storage logic to module | Medium |
| #4 | Comprehensive public input validation | High |
| #5 | Implement challenge cancellation | Medium |
| #6 | Add batch claim support | Low |
| #7 | Implement challenge enumeration | Low |
| #8 | Add comprehensive integration tests | High |

## 🔗 Linkage

- **Circuits**: [zk-wavescout-circuits](https://github.com/zk-wavescout/zk-wavescout-circuits) — Noir ZK circuit definitions
- **Coordinator**: [zk-wavescout-coordinator](https://github.com/zk-wavescout/zk-wavescout-coordinator) — Event listener & relayer
- **Integration**: Emits `bounty_claimed` events → Coordinator processes claims → Merges to GitHub

## Deploy

```bash
soroban network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/zk_wavescout_contracts.wasm \
  --source deployer \
  --network testnet
```

See ISSUE items for remaining implementation.
