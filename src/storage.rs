use soroban_sdk::{Env, Symbol};
use crate::lib::Challenge;

pub fn store_challenge(env: &Env, challenge_id: u32, challenge: &Challenge) {
    env.storage().persistent().set(&challenge_id, challenge);
}

pub fn retrieve_challenge(env: &Env, challenge_id: u32) -> Option<Challenge> {
    env.storage().persistent().get(&challenge_id)
}

pub fn mark_challenge_solved(env: &Env, challenge_id: u32, solver: &soroban_sdk::Address) {
    if let Some(mut challenge) = retrieve_challenge(env, challenge_id) {
        challenge.is_solved = true;
        challenge.solver = Some(solver.clone());
        store_challenge(env, challenge_id, &challenge);
    }
}
