use std::u8;

use rand::{prelude::*};
use rand::rngs::adapter::{ReseedingRng};
use rand_chacha::ChaCha20Core;
use rand::rngs::OsRng;

pub fn seeder(seed: u8) -> u8 {
    let s: u8 = if seed != 0 {
        seed
    } else {
        let mut rng = rand::thread_rng();
        rng.gen::<u8>() // generates a float between 0 and 1
    };
    s
}

pub fn generator(seed: u8) -> Vec<u8> {
    let mut rng = ReseedingRng::new(ChaCha20Core::seed_from_u64(seed.into()), 0, OsRng);
    let nums: Vec<u8>= vec![rng.gen_range(1..4), rng.gen_range(1..4), rng.gen_range(1..4), rng.gen_range(1..4)];
    nums
}