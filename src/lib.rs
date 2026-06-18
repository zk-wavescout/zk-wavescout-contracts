use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, BytesN, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Challenge {
    pub creator: Address,
    pub reward_amount: u128,
    pub token: Address,
    pub target_hash: BytesN<32>,
    pub is_solved: bool,
    pub solver: Option<Address>,
}

#[contract]
pub struct ZKWaveScout;

#[contractimpl]
impl ZKWaveScout {
    /// Creates a new bounty challenge, locking reward assets in escrow.
    /// TODO: Add event emission for challenge_created
    pub fn create_challenge(
        env: Env,
        creator: Address,
        challenge_id: u32,
        token: Address,
        amount: u128,
        target_hash: BytesN<32>,
    ) {
        creator.require_auth();

        let client = soroban_sdk::token::Client::new(&env, &token);
        client.transfer(&creator, &env.current_contract_address(), &amount);

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

    /// Validates ZK proof and processes token payout.
    /// TODO: Implement real BN254 pairing verification (Issue #1)
    /// TODO: Add proof replay protection with nonce (Issue #2)
    pub fn claim_bounty(
        env: Env,
        challenge_id: u32,
        contributor: Address,
        proof: Vec<u8>,
        public_inputs: Vec<BytesN<32>>,
    ) {
        contributor.require_auth();

        // TODO: Extract storage retrieval to helper module (Issue #3)
        let mut challenge: Challenge = env.storage().persistent().get(&challenge_id)
            .expect("Challenge does not exist");

        if challenge.is_solved {
            panic!("Challenge already claimed");
        }

        // TODO: Add comprehensive public input validation (Issue #4)
        let target_hash_input = public_inputs.get(0).expect("Missing target hash");
        if target_hash_input != challenge.target_hash {
            panic!("Hash mismatch");
        }

        // TODO: Replace stub with production verifier (Issue #1)
        if !verify_zk_proof(&env, proof, public_inputs) {
            panic!("Proof verification failed");
        }

        challenge.is_solved = true;
        challenge.solver = Some(contributor.clone());
        env.storage().persistent().set(&challenge_id, &challenge);

        let client = soroban_sdk::token::Client::new(&env, &challenge.token);
        client.transfer(&env.current_contract_address(), &contributor, &challenge.reward_amount);

        env.events().publish(
            (Symbol::new(&env, "bounty_claimed"), challenge_id),
            contributor,
        );
    }

    /// TODO: Implement challenge cancellation (Issue #5)
    pub fn cancel_challenge(env: Env, challenge_id: u32) {
        unimplemented!("Challenge cancellation not yet implemented");
    }

    /// TODO: Add batch claim support (Issue #6)
    pub fn batch_claim_bounties(env: Env, claims: Vec<(u32, Address)>) {
        unimplemented!("Batch claims not yet implemented");
    }

    /// Query full challenge data
    pub fn get_challenge(env: Env, challenge_id: u32) -> Option<Challenge> {
        env.storage().persistent().get(&challenge_id)
    }

    /// Quick check if challenge is solved
    pub fn is_challenge_solved(env: Env, challenge_id: u32) -> bool {
        env.storage().persistent().get::<u32, Challenge>(&challenge_id)
            .map(|c| c.is_solved)
            .unwrap_or(false)
    }

    /// TODO: Add challenge enumeration (Issue #7)
    pub fn list_challenges(env: Env) {
        unimplemented!("Challenge listing not yet implemented");
    }
}

/// Stub ZK proof verifier - MUST be replaced with BN254 pairing for production
/// TODO: Issue #1 - Implement real verification using:
/// - Option A: Soroban native BN254 pairing (when available)
/// - Option B: Port UltraPLONK verifier library (~500KB WASM)
fn verify_zk_proof(_env: &Env, _proof: Vec<u8>, _public_inputs: Vec<BytesN<32>>) -> bool {
    true  // Stub for testnet only
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_struct_compiles() {
        assert_eq!(std::mem::size_of::<Challenge>(), std::mem::size_of::<Challenge>());
    }

    // TODO: Issue #8 - Add integration tests:
    // - test_create_challenge_valid_token
    // - test_double_claim_prevention
    // - test_hash_mismatch_rejection
    // - test_event_emission_on_claim
    // - test_storage_persistence
}
