use crate::story::{end, select_num};

use super::part2;

pub fn init(proc: [u8; 4], seed: u8, intro: bool) {
    match proc[0] {
        1 => {
            if intro {
                println!("\nYou are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it.\n");
            }
            match select_num(
                "Where is the key?\n",
                &[
                    "Pants",
                    "Medicine Carbinet",
                    "Under the Bed",
                    "go back to sleep",
                ],
            ) {
                1 => {
                    part2::init(proc, seed, true);
                }
                4 => {
                    end("Luckily, you survived. But how and from what? The ship's alarm was triggered by an impending Supernova.\nHowever, because of *redacted* you have survived! And you didn't even know what danger you were in!", seed);
                }
                _ => {
                    println!("Unfortunately, you don't find anything there.\n");
                    init(proc, seed, false);
                }
            }
        }
        2 => {
            if intro {
                println!("\nYou are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with a keypad, you need a code.\n");
            }
            match select_num(
                "Where is the code?\n",
                &[
                    "under the bed",
                    "on the pinboard",
                    "on your smartphone",
                    "go back to sleep",
                ],
            ) {
                2 => {
                    part2::init(proc, seed, true);
                }
                4 => {
                    end("Luckily, you survived. But how and from what? The ship's alarm was triggered by an impending Supernova.\nHowever, because of *redacted* you have survived! And you didn't even know what danger you were in!", seed);
                }
                _ => {
                    println!("Unfortunately, you don't find anything there.\n");
                    init(proc, seed, false);
                }
            }
        }
        3 => {
            if intro {
                println!("\nYou are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it.\n");
            }
            match select_num(
                "Where is the key?\n",
                &["keyhole", "wardrobe", "pants", "go back to sleep"],
            ) {
                3 => {
                    part2::init(proc, seed, true);
                }
                4 => {
                    end("Luckily, you survived. But how and from what? The ship's alarm was triggered by an impending Supernova.\nHowever, because of *redacted* you have survived! And you didn't even know what danger you were in!", seed);
                }
                _ => {
                    println!("Unfortunately, you don't find anything there.\n");
                    init(proc, seed, false);
                }
            }
        }
        _ => {}
    }
}
