use console_utils::input::{reveal, select};

pub mod intro;
pub mod part1;
pub mod part2;
pub mod part3;
pub mod part4;

pub fn select_num(qst: &str, num_answers: &[&str]) -> u8 {
    select(qst, num_answers, false, false)
        .unwrap()
        .iter()
        .position(|&x| x)
        .unwrap() as u8
        + 1
}

pub fn end(str: &str, seed: u8) {
    println!("{}", str);
    reveal(&format!("Your current Seed: {seed}\n"), 0.05);
}
