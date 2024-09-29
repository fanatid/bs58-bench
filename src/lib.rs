use {rand::SeedableRng, rand_chacha::ChaCha20Rng, rand_core::RngCore, solana_sdk::pubkey::Pubkey};

pub fn generate(state: u64, count: usize) -> Vec<Pubkey> {
    let mut rng = ChaCha20Rng::seed_from_u64(state);

    let mut pubkeys = Vec::with_capacity(count);
    for _ in 0..count {
        loop {
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);

            let pubkey = Pubkey::new_from_array(bytes);
            if pubkey.is_on_curve() {
                pubkeys.push(pubkey);
                break;
            }
        }
    }
    pubkeys
}
