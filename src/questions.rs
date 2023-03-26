use std::io;

pub fn questions(data: u8, qst: u8) -> u8 {
    let mut input: u8 = 0;

    if qst == 1 {
        if data == 1 {
            input = question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it. Where is the key?\n(Pants(1), Medicine Carbinet(2), under the bed(3), go back to sleep(4))", 4);
        }
        if data == 2 {
            input = question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with a keypad, you need a code. Where is the code?\n(under the bed(1), on your smartphone(2), on the pinboard(3), go back to sleep(4))", 4);
        }
        if data == 3 {
            input = question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it. Where is the key?\n(keyhole(1), wardrope(2), pants(3), go back to sleep(4))", 4);
        }
    } else if qst == 2 {
        if data == 1 {
            input = question(
                r#"
There is it!
You unlock the door.
     ┌───┐     
    ┌┘   └┐    
    │  1  │    
┌───┼─────┼───┐
│ Y │     │ 2 │
└───┴─────┴───┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y), Bridge(1), Reactor(2))"#,
                2,
            );
        } else if data == 2 {
            input = question(
                r#"
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
(Your position(Y), Bridge(1), Reactor(2))"#,
                2,
            );
        } else if data == 3 {
            input = question(
                r#"
There is it!
You unlock the door.
     ┌───┐
 ┌───┤ 1 ├───┐
 │ 2 ├───┤ Y │
 └┬┬─┼┬─┬┼─┬┬┘
  └┘ └┘ └┘ └┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y), Bridge(1), Reactor(2))"#,
                2,
            );
        }
    } else if qst == 3 {
        if data == 1 {
            input = question("In the bridge, you can take a look at the control console or attack an annoying fly.\n(control console(1), annoying Fly(2))\n", 2);
            if input == 1 {
                answers(1, 3, input);
                return input;
            } else if input == 2 {
                answers(1, 3, input);
                return input;
            }
        } else if data == 2 {
            todo!("question 3 variant 2,3 and answer 2,3")
        } else if data == 3 {
            todo!("question 3 variant 2,3 and answer 2,3")
        }
    } else if qst == 4 {
        if data == 1 {
            input = question("You can deactivate the alarm and go back to sleep, or go back to the previous room.\n(Deactivate alarm and go back to sleep(1), Go back(2))\n", 2);
            if input == 1 {
                answers(1, 4, input);
                return 137;
            } else if input == 2 {
                answers(1, 4, input);
                return input;
            }
        }
        if data == 2 {
            input = question("You can go back to the previous room, or initiate a reactor meltdown and...die I guess(why the f would you do that).\n(Go back(1), Initiate Reactor meltdown(2))\n", 2);
            if input == 1 {
                answers(2, 4, input);
                return input;
            } else if input == 2 {
                answers(2, 4, input);
                return 137;
            }
        }
        if data == 3 {
            input = question("You can go back to the previous room, or @#($(#J$QIJ#RES)).\n(Go back(1), _)@#$#IR)@#RK(2))\n", 2);

            if input == 1 {
                answers(3, 4, input);
                return input;
            } else if input == 2 {
                answers(3, 4, input);
                return input;
            }
        }
    }
    return input;
}
pub fn answers(data: u8, qst: u8, input: u8) {
    if qst == 1 {
        if data == 1 || data == 2 || data == 3 {
            if input == 1 || input == 2 {
                println!("\nUnfortunately, you don't find anything there.\n");
            }
            if input == 3 {
                return;
            }
            if input == 4 {
                println!("\nLuckily, you survived. But how and from what? The ship's alarm was triggered by an impending supernova.\nHowever, because of *redacted* you have survived! And you didn't even know what danger you were in!\n");
                return;
            }
        }
    }
    if qst == 2 {
        if data == 1 || data == 2 || data == 3 {
            //todo: add randomness
            if input == 1 {
                println!("\nYou go to the bridge.\n");
            }
            if input == 2 {
                println!("\nYou go to the reactor.\n");
            }
        }
    }
    if qst == 3 {
        if data == 1 {
            if input == 1 {
                println!("On the control console, you see that a supernova will obliterate the ship and you in the next 5 minutes.");
                loop {
                    let num = question("Do you want to fly behind a moon nearby or do nothing and wait for your death?\n(Fly behind moon(1), Wait for death(2))\n", 2);
                    if num == 1 {
                        println!("Did the moon protect you? Yes, as the moon has a zitanium component that reflects any kind of energy, shockwaves, etc.\nYou lie down again, turn on the autopilot, and enjoy the sleep and relaxation. You have survived!\n");
                        return;
                    }
                    if num == 2 {
                        println!("You did that too...You died!\n");
                        return;
                    }
                }
            }
            if input == 2 {
                println!("You go to the annoying fly.\n");
                loop {
                    let num = question("Do you want to kill the fly or negotiate with it?\n(kill(1), negotiate(2))\n", 2);
                    if num == 1 {
                        println!("You use your laser and deal 2d4 damage to the fly.\nUnfortunately, you also blast a hole in the hull of the ship, and everything gets sucked out into the vacuum of space, including you. You have died!\n");
                        return;
                    }
                    if num == 2 {
                        println!("You negotiate with the fly and convince it to be less annoying before heading to the control console.\nUnfortunately, it was already too late. You could only see a timer indicating when the supernova would reach you,\nand it had just reached 00:00. You have died!\n");
                        return;
                    }
                }
            }
        }
        if data == 2 {
            todo!("question 3 variant 2,3 and answer 2,3")
        }
        if data == 3 {
            todo!("question 3 variant 2,3 and answer 2,3")
        }
    }
    if qst == 4 {
        if data == 1 {
            if input == 1 {
                println!("You go back to sleep. But wait, there was something you forgot? Oh, right, a Supernova... You have died!\n");
            }
            if input == 2 {
                println!("You go back to the previous room.\n");
            }
        }
        if data == 2 {
            if input == 1 {
                println!("You go back to the previous room.\n");
            }
            if input == 2 {
                println!("As you try to blow yourself up, the ship mailfunctions and activates it shields which are barely strong enough to protect you from the Supernova.\nYou didn't die and had to live, even when you tried to kill yourself!")
            }
        }
        if data == 3 {
            if input == 1 {
                println!("You go back to the previous room.\n");
            }
            if input == 2 {
                println!("*#$D#*UD#@()$@H#R()@#)(RJH#DHUFH*(W(#$&@#$@#GD*(!@Q#&*$Y@#$YRH&HCJ@()#)_DHUJHFUDHUISHF*DFUHS*()#EYRH@WQ#(*RYU@Q#*(R(ED(FISHUDFY(*@#&QY&*RT@&#*R*(YY*(EYR(*@#&YG@#TYUDGF^SG^TFASTEFD@&*#$*U#*(@%YU*$@Y*YRU*FU()$WE")
            }
        }
    }
}

pub fn question(qst: &str, num_answers: u8) -> u8 {
    question_advanced(qst, num_answers, false).unwrap()
}

pub fn question_advanced(qst: &str, num_answers: u8, allow_empty: bool) -> Option<u8> {
    loop {
        println!("{qst}");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        if allow_empty && guess.trim().is_empty() {
            return None;
        }

        if let Ok(num) = guess.trim().parse() {
            if 0 < num && num <= num_answers {
                return Some(num);
            } else {
                println!("\nNumber out of Range\n");
            }
        } else {
            println!("\nWrong Input, give Numbers\n");
        };
    }
}
