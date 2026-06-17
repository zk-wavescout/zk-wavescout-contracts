# zk-wavescout-contracts

A Soroban smart contract framework for ZK-verified bounty puzzles. Holds bounty pool funds in escrow, executes on-chain ZK-proof verification, and handles automated asset disbursement.

## Architecture

```
zk-wavescout-contracts/
├── Cargo.toml              # Dependencies
├── soroban.toml            # Soroban network config
├── src/
│   ├── lib.rs              # Core contract (Challenge, ZKWaveScout)
│   ├── verification.rs     # ZK proof verification
│   ├── events.rs           # Event emission
│   └── storage.rs          # Persistent state
├── ARCHITECTURE.md         # Detailed module docs
└── SETUP.md               # Build & deployment
```

## Core Contract

### Challenge Data Structure
```rust
pub struct Challenge {
    pub creator: Address,           // Bounty creator
    pub reward_amount: u128,        // Token payout amount
    pub token: Address,             // Stellar Asset Contract
    pub target_hash: BytesN<32>,    // Poseidon hash criteria
    pub is_solved: bool,            // Claim prevention flag
    pub solver: Option<Address>,    // Who solved it
}
```

### Primary Functions

#### `create_challenge()`
Initialize a bounty with token escrow.
```rust
pub fn create_challenge(
    env: Env,
    creator: Address,
    challenge_id: u32,
    token: Address,
    amount: u128,
    target_hash: BytesN<32>,
) {
    creator.require_auth();
    
    // Transfer tokens from creator to contract
    let client = soroban_sdk::token::Client::new(&env, &token);
    client.transfer(&creator, &env.current_contract_address(), &amount);
    
    // Store challenge in persistent storage
    let challenge = Challenge {
        creator,
        reward_amount: amount,
        token,
        target_hash,
        is_solved: false,
        solver: None,
    };
    env.storage().persistent().set(&challenge_id, &challenge);
}
```

#### `claim_bounty()`
Submit ZK proof, verify, and claim reward.
```rust
pub fn claim_bounty(
    env: Env,
    challenge_id: u32,
    contributor: Address,
    proof: Vec<u8>,
    public_inputs: Vec<BytesN<32>>,
) {
    contributor.require_auth();
    
    let mut challenge: Challenge = env.storage().persistent().get(&challenge_id)
        .expect("Challenge does not exist");
    
    if challenge.is_solved {
        panic!("Challenge already claimed");
    }
    
    // Verify public inputs match challenge target
    let target_hash_input = public_inputs.get(0).expect("Missing target hash");
    if target_hash_input != challenge.target_hash {
        panic!("Hash mismatch");
    }
    
    // Execute ZK verification
    if !verify_zk_proof(&env, proof, public_inputs) {
        panic!("Proof verification failed");
    }
    
    // Mark solved & transfer reward
    challenge.is_solved = true;
    challenge.solver = Some(contributor.clone());
    env.storage().persistent().set(&challenge_id, &challenge);
    
    let client = soroban_sdk::token::Client::new(&env, &challenge.token);
    client.transfer(&env.current_contract_address(), &contributor, &challenge.reward_amount);
    
    // Emit event for off-chain coordinator
    env.events().publish(
        (Symbol::new(&env, "bounty_claimed"), challenge_id),
        contributor,
    );
}
```

#### `get_challenge()` / `is_challenge_solved()`
Query challenge state.
```rust
pub fn get_challenge(env: Env, challenge_id: u32) -> Option<Challenge>
pub fn is_challenge_solved(env: Env, challenge_id: u32) -> bool
```

## Data Flow

1. **Create** → Creator calls `create_challenge()` → tokens escrowed in contract
2. **Submit** → Contributor calls `claim_bounty()` with ZK proof
3. **Verify** → Proof validated on-chain via UltraPLONK verifier
4. **Payout** → Tokens transferred to contributor
5. **Notify** → `bounty_claimed` event emitted for backend coordinator

## Security

- ✅ All functions require authorization via `require_auth()`
- ✅ Double-claim prevention via `is_solved` flag
- ✅ Token escrow in contract (no user custody risk)
- ✅ Event-driven state sync with backend
- ⚠️ ZK verifier currently stub (requires BN254 pairing implementation)

## Build & Deploy

```bash
# Check compilation
cargo check

# Build WASM
soroban contract build

# Deploy to testnet
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/zk_wavescout_contracts.wasm
```

See [SETUP.md](./SETUP.md) for detailed instructions.

## Modules

- **lib.rs** - Contract core + Challenge struct
- **verification.rs** - ZK proof validation interface
- **events.rs** - Event emission utilities
- **storage.rs** - Persistent storage helpers
