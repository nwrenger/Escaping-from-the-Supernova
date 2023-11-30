mod generation;
mod questions;
use colored::Colorize;
use console_utils::input::{input, reveal, spinner, SpinnerType};

fn main() {
    reveal(
        &format!(
            "{}",
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
                .yellow()
        ),
        0.005,
    );

    spinner(1.5, SpinnerType::Dots);
    reveal(
        &format!(
            "{}",
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
                .yellow()
        ),
        0.005,
    );
    spinner(1.5, SpinnerType::Dots);
    reveal(&format!("a Textadventure from {}\n", "Nils Wrenger".red()), 0.05);

    let seed = input(
        "Give your current seed or press enter for a random seed (0-255): ",
        true,
    );

    let seed = generation::seeder(seed);
    reveal(&format!("Your current Seed: {seed}\n"), 0.05);

    let data = generation::generator(seed);

    spinner(
        1.5,
        SpinnerType::Custom(vec!["You.", "Are.", "Gonna.", "Die."]),
    );

    loop {
        let input = questions::questions(data[0], 1);
        if input == data[0] {
            loop {
                let input = questions::questions(data[1], 2);
                if input == 1 {
                    questions::answers(data[1], 2, input);
                    questions::questions(data[2], 3);
                    reveal(&format!("Your current Seed: {seed}\n"), 0.05);
                    return;
                }
                if input == 2 {
                    questions::answers(data[2], 2, input);
                    let input = questions::questions(data[3], 4);
                    if input == 137 {
                        reveal(&format!("Your current Seed: {seed}\n"), 0.05);
                        return;
                    }
                }
            }
        }
        if input == 4 {
            questions::answers(data[0], 1, input);
            reveal(&format!("Your current Seed: {seed}\n"), 0.05);
            return;
        } else {
            questions::answers(data[0], 1, input);
        }
    }
}
