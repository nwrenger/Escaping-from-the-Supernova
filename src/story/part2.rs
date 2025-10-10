use crate::story::intro::print_reveal;

use super::{part3, part4, select_num};

pub fn init(proc: [u8; 4], seed: u8, intro: bool) {
    match proc[1] {
        1 => {
            if intro {
                print_reveal(
                    r"
There is it!
You unlock the door.
     ┌───┐
    ┌┘   └┐
    │  1  │
┌───┼─────┼───┐
│ Y │     │ 2 │
└───┴─────┴───┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y))",
                );
            }
            match select_num("Where do you want to go?", &["Bridge(1)", "Reactor(2)"]) {
                1 => {
                    part3::init(proc, seed);
                }
                2 => {
                    part4::init(proc, seed);
                }
                _ => {}
            }
        }
        2 => {
            if intro {
                print_reveal(
                    r"
There is it!
You unlock the door.
    ┌┬───┬┐
    ││ 2 ││
 ┌──┴┴───┴┴──┐
 │           │
 └──┬─────┬──┘
    │  Y  │
    └┬───┬┘
     │ 1 │
     └───┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y))",
                );
            }
            match select_num("Where do you want to go?", &["Bridge(1)", "Reactor(2)"]) {
                1 => {
                    part3::init(proc, seed);
                }
                2 => {
                    part4::init(proc, seed);
                }
                _ => {}
            }
        }
        3 => {
            if intro {
                print_reveal(
                    r"
There is it!
You unlock the door.
    ┌───┐
┌───┤ 1 ├───┐
│ 2 ├───┤ Y │
└┬┬─┼┬─┬┼─┬┬┘
 └┘ └┘ └┘ └┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y))",
                );
            }
            match select_num("Where do you want to go?", &["Bridge(1)", "Reactor(2)"]) {
                1 => {
                    part3::init(proc, seed);
                }
                2 => {
                    part4::init(proc, seed);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
