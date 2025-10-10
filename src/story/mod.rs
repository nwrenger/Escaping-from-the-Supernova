use console_utils::input::select;

use crate::story::intro::print_reveal;

pub mod intro;
pub mod part1;
pub mod part2;
pub mod part3;
pub mod part4;

pub fn select_num(qst: &str, num_answers: &[&str]) -> u8 {
    (select(qst, num_answers) + 1).try_into().unwrap()
}

pub fn end(str: &str, seed: u8) {
    print_reveal(&format!("{str}\n"));
    print_reveal(&format!("Your current Seed: {seed}\n"));
}
