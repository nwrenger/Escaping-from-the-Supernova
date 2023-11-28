use console_utils::select;

pub fn questions(data: u8, qst: u8) -> u8 {
    match (qst, data) {
        (1, 1) => question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it. Where is the key?\n", &["Pants".to_owned(), "Medicine Carbinet".to_owned(), "Under the Bed".to_owned(), "go back to sleep".to_owned()]),
        (1, 2) => question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with a keypad, you need a code. Where is the code?\n", &["under the bed".to_owned(), "on the pinboard".to_owned(), "on your smartphone".to_owned(), "go back to sleep".to_owned()]),
        (1, 3) => question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it. Where is the key?\n", &["keyhole".to_owned(), "wardrobe".to_owned(), "pants".to_owned(), "go back to sleep".to_owned()]),
        (2, 1) => question(
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
(Your position(Y))"#,
            &["Bridge(1)".to_owned(), "Reactor(2)".to_owned()],
        ),
        (2, 2) => question(
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
(Your position(Y))"#,
            &["Bridge(1)".to_owned(), "Reactor(2)".to_owned()],
        ),
        (2, 3) => question(
            r#"
There is it!
You unlock the door.
    ┌───┐
┌───┤ 1 ├───┐
│ 2 ├───┤ Y │
└┬┬─┼┬─┬┼─┬┬┘
 └┘ └┘ └┘ └┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y))"#,
            &["Bridge(1)".to_owned(), "Reactor(2)".to_owned()],
        ),
        (3, 1) => {
            let input = select("In the bridge, you can take a look at the control console or attack an annoying fly.\n", &["control console".to_owned(), "annoying Fly".to_owned()], false, false).unwrap().iter().position(|&x| x).unwrap() as u8 + 1;
            answers(1, 3, input);
            input
        }
        (3, 2) => {
            let input = select("As the door is opening, you see a hole in the hull of the ship and you see the control panel, which is on fire. What do you fix first?.\n", &["hull".to_owned(), "control console".to_owned()], false, false).unwrap().iter().position(|&x| x).unwrap() as u8 + 1;
            answers(2, 3, input);
            input
        }
        (3, 3) => {
            let input = select("As you enter the room, you can either go to a chessboard on a table or to a flux capacitor, which can reverse the time. What do want todo?\n", &["chessboard".to_owned(), "flux capacitor".to_owned()], false, false).unwrap().iter().position(|&x| x).unwrap() as u8 + 1;
            answers(3, 3, input);
            input
        }
        (4, 1) => {
            let input = select("You can deactivate the alarm and go back to sleep, or go back to the previous room.\n", &["Deactivate alarm and go back to sleep".to_owned(), "Go back".to_owned()], false, false).unwrap().iter().position(|&x| x).unwrap() as u8 + 1;
            answers(1, 4, input);
            if input == 1 {
                137
            } else {
                input
            }
        }
        (4, 2) => {
            let input = select("You can go back to the previous room, or initiate a reactor meltdown and...die I guess(why the f would you do that).\n", &["Go back".to_owned(), "Initiate Reactor meltdown".to_owned()], false, false).unwrap().iter().position(|&x| x).unwrap() as u8 + 1;
            answers(2, 4, input);
            if input == 2 {
                137
            } else {
                input
            }
        }
        (4, 3) => {
            let input = select("You can go back to the previous room, or @#($(#J$QIJ#RES)).\n", &["Go back".to_owned(), "_)@#$#IR)@#RK".to_owned()], false, false).unwrap().iter().position(|&x| x).unwrap() as u8 + 1;
            answers(3, 4, input);
            input
        }
        (_, _) => 0,
    }
}

pub fn answers(data: u8, qst: u8, input: u8) {
    match (qst, data) {
        (1, 1..=3) => {
            if input == data {
                return;
            }
            if input == 4 {
                println!("\nLuckily, you survived. But how and from what? The ship's alarm was triggered by an impending supernova.\nHowever, because of *redacted* you have survived! And you didn't even know what danger you were in!\n")
            } else {
                println!("\nUnfortunately, you don't find anything there.\n");
            }
        }
        (2, 1..=3) => {
            if data == 1 || data == 2 || data == 3 {
                if input == 1 {
                    println!("\nYou go to the bridge.\n");
                }
                if input == 2 {
                    println!("\nYou go to the reactor.\n");
                }
            }
        }
        (3, 1) => {
            if input == 1 {
                println!("On the control console, you see that a supernova will obliterate the ship and you in the next 5 minutes.");
                loop {
                    let num = question("Do you want to fly behind a moon nearby or do nothing and wait for your death?\n", &["Fly behind moon".to_owned(), "Wait for death".to_owned()]);
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
                println!("You go to the annoying Fly.\n");
                loop {
                    let num = question(
                        "Do you want to kill the fly or negotiate with it?\n",
                        &["kill".to_owned(), "negotiate".to_owned()],
                    );
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
        (3, 2) => {
            if input == 1 {
                println!("The hole in the hull gets bigger and bigger and the pull into space gets stronger and stronger. And you begin to see better through it.\nSuddenly Jesus appears and you can see him through that hole.");
                loop {
                    let num = question(
                        "Do you want to jump into Jesus arms or do you want to fix up the hole?\n",
                        &["Jesus".to_owned(), "Fix up the hole".to_owned()],
                    );
                    if num == 1 {
                        println!("As you epicly dive into space, you can see Jesus smile and getting brighter and brighter.\nYou died from Suffocation! Jesus welcomed you in his arms.\n");
                        return;
                    }
                    if num == 2 {
                        println!("You fixed up the hole and the Extinguishing system of the ship turns on. The fire at the controle panel exinguishs and the alarm suddenly stop.\nIt seems that the alarm was caused by a malefunktion of the main system and the ship thought it was in near proximity to a Supernova. You have Survived!\n");
                        return;
                    }
                }
            }
            if input == 2 {
                println!("The fire around the control panel grows.\n");
                loop {
                    let num = question(
                        "Do you want to extinguish it or run away?\n)",
                        &["extinguish".to_owned(), "RUN!?!?".to_owned()],
                    );
                    if num == 1 {
                        println!("You extinguished the fire in seconds. Luckily, the fire extinguisher was loacated nearby. The hole in the hull got bigger and pulled you out into space. You have died!\n");
                        return;
                    }
                    if num == 2 {
                        println!(
                            "You run. Through time and space. Back to earth. You have survived!\n"
                        );
                        return;
                    }
                }
            }
        }
        (3, 3) => {
            if input == 1 {
                println!("You start to play chess with the Supernova\n");
                loop {
                    let num = question("You know to play the Damen Gambit Opening and you can also try to throw the game.\n", &["Damen Gambit".to_owned(), "Throw".to_owned()]);
                    if num == 1 {
                        println!("You open your game with the Damen Gambit Opening. The Supernova is no real opponent for you and you win in the 5 next turns.\nUnfortunately, the Supernova gets angry from the loose and explodes. You have died!\n");
                        return;
                    }
                    if num == 2 {
                        println!("You throw, a lot. The Supernova wins and is pleased with it. Afterwards, the Supernova thanks you and flies away. You have Survived!\n");
                        return;
                    }
                }
            }
            if input == 2 {
                println!("You go to the Flux Capacitor.\n");
                loop {
                    let num = question("You can either reverse time or throw the precious Flux Capacitor at the Supernova.\n", &["reverse time".to_owned(), "throw the precious".to_owned()]);
                    if num == 1 {
                        println!("You accidently reversed your time completly, now you live your life backwards from this point in time. So technecally, you have died!\n");
                        return;
                    }
                    if num == 2 {
                        println!("As quickly as you had that thought, you executed it. The Flux Capacitor holds negative energy. Because of that, the Supernova implodes. You have Survived\n");
                        return;
                    }
                }
            }
        }
        (4, 1) => {
            if input == 1 {
                println!("You go back to sleep. But wait, there was something you forgot? Oh, right, a Supernova... You have died!\n");
            }
            if input == 2 {
                println!("You go back to the previous room.\n");
            }
        }
        (4, 2) => {
            if input == 1 {
                println!("You go back to the previous room.\n");
            }
            if input == 2 {
                println!("As you try to blow yourself up, the ship mailfunctions and activates it shields which are barely strong enough to protect you from the Supernova.\nYou didn't die and had to live, even when you tried to kill yourself!")
            }
        }
        (4, 3) => {
            if input == 1 {
                println!("You go back to the previous room.\n");
            }
            if input == 2 {
                println!("*#$D#*UD#@()$@H#R()@#)(RJH#DHUFH*(W(#$&@#$@#GD*(!@Q#&*$Y@#$YRH&HCJ@()#)_DHUJHFUDHUISHF*DFUHS*()#EYRH@WQ#(*RYU@Q#*(R(ED(FISHUDFY(*@#&QY&*RT@&#*R*(YY*(EYR(*@#&YG@#TYUDGF^SG^TFASTEFD@&*#$*U#*(@%YU*$@Y*YRU*FU()$WE")
            }
        }
        (_, _) => {}
    }
}

pub fn question(qst: &str, num_answers: &[String]) -> u8 {
    select(qst, num_answers, false, false)
        .unwrap()
        .iter()
        .position(|&x| x)
        .unwrap() as u8
        + 1
}
