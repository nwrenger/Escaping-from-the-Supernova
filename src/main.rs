use std::thread;
use std::time::Duration;
mod generation;
mod questions;
use colored::Colorize;

fn main() {
    println!(
        "{}",
        r#"
 _____                           _                  __                         
|  ___|                         (_)                / _|                        
| |__   ___   ___   __ _  _ __   _  _ __    __ _  | |_  _ __   ___   _ __ ___  
|  __| / __| / __| / _` || '_ \ | || '_ \  / _` | |  _|| '__| / _ \ | '_ ` _ \ 
| |___ \__ \| (__ | (_| || |_) || || | | || (_| | | |  | |   | (_) || | | | | |
\____/ |___/ \___| \__,_|| .__/ |_||_| |_| \__, | |_|  |_|    \___/ |_| |_| |_|
                         | |                __/ |                              
                         |_|               |___/                               "#
            .yellow()
    );

    thread::sleep(Duration::from_secs_f64(1.5));
    println!(
        "{}",
        r#"
 _    _             _____                                                       
| |  | |           /  ___|                                                      
| |_ | |__    ___  \ `--.  _   _  _ __    ___  _ __  _ __    ___  __   __  __ _ 
| __|| '_ \  / _ \  `--. \| | | || '_ \  / _ \| '__|| '_ \  / _ \ \ \ / / / _` |  TM
| |_ | | | ||  __/ /\__/ /| |_| || |_) ||  __/| |   | | | || (_) | \ V / | (_| |
 \__||_| |_| \___| \____/  \__,_|| .__/  \___||_|   |_| |_| \___/   \_/   \__,_|
                                 | |                                            
                                 |_|                                            "#
            .yellow()
    );
    thread::sleep(Duration::from_secs_f64(1.5));
    println!("a Textadventure from {}\n", "Nils Wrenger".red());

    thread::sleep(Duration::from_secs_f64(1.0));
    let seed = questions::question_advanced(
        "Give your current seed or press enter for a random seed:",
        255,
        true,
    );

    let seed = generation::seeder(seed);
    println!("Your current Seed: {seed:?}");

    let data = generation::generator(seed);
    // println!("Data:{data:?}");

    thread::sleep(Duration::from_secs_f64(1.0));
    loop {
        let input = questions::questions(data[0], 1);
        if input == data[0] {
            loop {
                let input = questions::questions(data[1], 2);
                if input == 1 {
                    questions::answers(data[1], 2, input);
                    questions::questions(data[2], 3);
                    println!("Your current Seed: {seed:?}");
                    return;
                }
                if input == 2 {
                    questions::answers(data[2], 2, input);
                    let input = questions::questions(data[3], 4);
                    if input == 137 {
                        println!("Your current Seed: {seed:?}");
                        return;
                    }
                }
            }
        }
        if input == 4 {
            questions::answers(data[0], 1, input);
            println!("Your current Seed: {seed:?}");
            return;
        } else {
            questions::answers(data[0], 1, input);
        }
    }
}
