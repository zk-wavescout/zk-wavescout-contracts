use soroban_sdk::{Env, Symbol};

pub fn emit_challenge_created(env: &Env, challenge_id: u32) {
    env.events().publish(
        (Symbol::new(env, "challenge_created"),),
        challenge_id,
    );
}

pub fn emit_bounty_claimed(env: &Env, challenge_id: u32, solver: String) {
    env.events().publish(
        (Symbol::new(env, "bounty_claimed"),),
        (challenge_id, solver),
    );
}
