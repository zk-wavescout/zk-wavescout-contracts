use soroban_sdk::Env;

pub fn challenge_key(challenge_id: u32) -> u32 {
    challenge_id
}

pub fn challenge_exists(env: &Env, challenge_id: u32) -> bool {
    env.storage().persistent().has(&challenge_id)
}
