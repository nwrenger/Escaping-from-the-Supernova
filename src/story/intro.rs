use console_utils::{
    input::{input, reveal, spinner, Empty, SpinnerType},
    styled::{Color, StyledText},
};

use crate::rng::{generator, seeder};

pub fn init() {
    // title
    reveal(
        &format!(
            "{}",
            StyledText::new(
                r"
 _____                           _                  __                         
|  ___|                         (_)                / _|                        
| |__   ___   ___   __ _  _ __   _  _ __    __ _  | |_  _ __   ___   _ __ ___  
|  __| / __| / __| / _` || '_ \ | || '_ \  / _` | |  _|| '__| / _ \ | '_ ` _ \ 
| |___ \__ \| (__ | (_| || |_) || || | | || (_| | | |  | |   | (_) || | | | | |
\____/ |___/ \___| \__,_|| .__/ |_||_| |_| \__, | |_|  |_|    \___/ |_| |_| |_|
                         | |                __/ |                              
                         |_|               |___/                               
"
            )
            .fg(Color::Yellow)
        ),
        0.0005,
    );

    reveal(
        &format!(
            "{}",
            StyledText::new(
                r"
 _    _             _____                                                       
| |  | |           /  ___|                                                      
| |_ | |__    ___  \ `--.  _   _  _ __    ___  _ __  _ __    ___  __   __  __ _ 
| __|| '_ \  / _ \  `--. \| | | || '_ \  / _ \| '__|| '_ \  / _ \ \ \ / / / _` |  TM
| |_ | | | ||  __/ /\__/ /| |_| || |_) ||  __/| |   | | | || (_) | \ V / | (_| |
 \__||_| |_| \___| \____/  \__,_|| .__/  \___||_|   |_| |_| \___/   \_/   \__,_|
                                 | |                                            
                                 |_|                                            

"
            )
            .fg(Color::Yellow)
        ),
        0.0005,
    );

    // author
    spinner(1.5, SpinnerType::Dots);
    reveal(
        &format!(
            "a Textadventure from {}\n",
            StyledText::new("Nils Wrenger").fg(Color::Red)
        ),
        0.05,
    );
    // seed stuff
    let seed: Empty<u8> = input("Give your current seed or press enter for a random seed (0-255)");

    let seed = seeder(seed);
    reveal(&format!("Your current Seed: {seed}\n"), 0.05);

    spinner(
        1.0,
        SpinnerType::Custom(&["You.", "Are.", "Gonna.", "Die."]),
    );

    super::part1::init(generator(seed), seed, true)
}
