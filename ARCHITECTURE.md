# ZK-WaveScout Contracts Architecture

## Module Structure

### Core Modules
- **lib.rs** - Main contract implementation (Challenge, ZKWaveScout)
- **verification.rs** - ZK proof verification logic
- **storage.rs** - Persistent state management
- **events.rs** - Event emission utilities

## Contract Functions

### Public Functions
1. `create_challenge()` - Initialize bounty with token escrow
2. `claim_bounty()` - Submit ZK proof and claim reward
3. `get_challenge()` - Query challenge state
4. `is_challenge_solved()` - Check if challenge claimed

## Data Flow

1. Creator calls `create_challenge` → tokens escrowed in contract
2. Contributor calls `claim_bounty` with ZK proof
3. Proof verified on-chain via UltraPLONK verifier
4. Tokens transferred to contributor
5. Event emitted for off-chain coordinator

## Security Considerations

- All functions require auth via `require_auth()`
- Double-claim prevention via `is_solved` flag
- Token escrow in contract address (no direct user custody)
- Event-driven state synchronization with backend
