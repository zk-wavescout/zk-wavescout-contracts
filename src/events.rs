use soroban_sdk::{Env, Symbol, Address};

pub fn emit_bounty_claimed(env: &Env, challenge_id: u32, contributor: &Address) {
    env.events().publish(
        (Symbol::new(env, "bounty_claimed"), challenge_id),
        contributor,
    );
}

pub fn emit_challenge_created(env: &Env, challenge_id: u32, creator: &Address) {
    env.events().publish(
        (Symbol::new(env, "challenge_created"), challenge_id),
        creator,
    );
}
