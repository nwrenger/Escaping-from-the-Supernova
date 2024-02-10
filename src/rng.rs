use std::u8;

use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn seeder(seed: Option<u8>) -> u8 {
    if let Some(seed) = seed {
        seed
    } else {
        let mut rng = rand::thread_rng();
        rng.gen::<u8>() // generates an usize between 0 and 255
    }
}

pub fn generator(seed: u8) -> [u8; 4] {
    let mut rng = SmallRng::seed_from_u64(seed.into());
    [
        rng.gen_range(1..4),
        rng.gen_range(1..4),
        rng.gen_range(1..4),
        rng.gen_range(1..4),
    ]
}
