use console_utils::input::Empty;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub fn seeder(seed: Empty<u8>) -> u8 {
    if let Empty::Some(seed) = seed {
        seed
    } else {
        let mut rng = rand::rng();
        rng.random::<u8>() // generates an usize between 0 and 255
    }
}

pub fn generator(seed: u8) -> [u8; 4] {
    let mut rng = SmallRng::seed_from_u64(seed.into());
    [
        rng.random_range(1..4),
        rng.random_range(1..4),
        rng.random_range(1..4),
        rng.random_range(1..4),
    ]
}
