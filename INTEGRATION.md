# Integration Guide

## Repository Linkage

This contract integrates with two key repositories:

### 1. zk-wavescout-circuits
**Repository:** https://github.com/zk-wavescout/zk-wavescout-circuits  
**Role:** Noir ZK circuit definitions  
**Integration Point:**
- Circuits compile to proving keys used by `verify_zk_proof()`
- Public inputs from circuit output match `Challenge.target_hash`

**Setup:**
```bash
git submodule add https://github.com/zk-wavescout/zk-wavescout-circuits.git circuits
```

### 2. zk-wavescout-coordinator
**Repository:** https://github.com/zk-wavescout/zk-wavescout-coordinator  
**Role:** Event listener & relayer for GitHub automation  
**Integration Point:**
- Listens for `bounty_claimed` events from this contract
- Decrypts contributor's code submission
- Automatically merges PR to repository

**Event Flow:**
```
Contract: emit bounty_claimed(challenge_id, contributor)
    ↓
Coordinator: Receives event via webhook
    ↓
Coordinator: Query challenge metadata
    ↓
Coordinator: Decrypt code payload
    ↓
Coordinator: GitHub API → Merge PR
```

## Initialization

Clone with submodules:
```bash
git clone --recurse-submodules https://github.com/zk-wavescout/zk-wavescout-contracts.git
cd zk-wavescout-contracts
```

## Cross-Repo Dependencies

| Component | Depends On | Purpose |
|-----------|-----------|---------|
| Circuits | - | Generate proving keys for verification |
| Contract | Circuits | Use public inputs + proving keys for verification |
| Coordinator | Contract | Listen for events and trigger automations |

## Development Workflow

1. **Modify circuits** → Rebuild proving keys → Update contract verification
2. **Modify contract** → Test locally → Deploy to testnet
3. **Coordinator listens** → Automatically picks up new events → Processes claims

See the respective repositories' README files for detailed setup instructions.
