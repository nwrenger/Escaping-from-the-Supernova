use crate::story::intro::print_reveal;

use super::{end, select_num};

pub fn init(proc: [u8; 4], seed: u8) {
    match proc[2] {
        1 => {
            print_reveal("\nAfter you squeezed through the doors to the bridge, which are apparently broken, you can either take a look at the control console or attack an annoying fly.\n");
            match select_num("What will you do?", &["control console", "annoying Fly"]) {
                1 => {
                    print_reveal("\nOn the control console, you see that a supernova will obliterate the ship and you in the next 5 minutes.\n");
                    match select_num("Do you want to fly behind a moon nearby or do nothing and wait for your death?", &["Fly behind moon", "Wait for death"]) {
                        1 => end("Did the moon protect you? Yes, as the moon has a Zitanium component that reflects any kind of energy, shockwaves, etc.\nYou lie down again, turn on the autopilot, and enjoy the sleep and relaxation. You have survived!", seed),
                        2 => end("You did that...You died but didn't feel any pain because the Supernova obliterated you in nanoseconds!", seed),
                        _ => {}
                    }
                }
                2 => {
                    print_reveal("\nYou go to the annoying Fly.");
                    match select_num("Do you want to kill the fly or negotiate with it?", &["kill", "negotiate"]) {
                        1 => end("You use your laser and deal 2d4 damage to the fly.\nUnfortunately, you also blast a hole in the hull of the ship, and everything gets sucked out into the vacuum of space, including you. You have died!", seed),
                        2 => end("You negotiate with the fly and convince it to be less annoying before heading to the control console.\nUnfortunately, it was already too late. You could only see a timer indicating when the supernova would reach you,\nand it had just reached 00:00. You have died!", seed),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        2 => {
            print_reveal("\nAs the door is opening, you see a hole in the hull of the ship and you see the control panel, which is on fire.\n");
            match select_num("What do you fix first?", &["hull", "control console"]) {
                1 => {
                    print_reveal("\nThe hole in the hull gets bigger and bigger and the pull into space gets stronger and stronger. And you begin to see better through it.\nSuddenly Jesus appears and you can see him through that hole.\n");
                    match select_num("Do you want to jump into Jesus arms or do you want to fix up the hole?", &["Jesus", "Fix up the hole"]) {
                        1 => end("As you epicly dive into space, you can see Jesus smile and getting brighter and brighter.\nYou died from Suffocation! Jesus welcomed you in his arms!", seed),
                        2 => end("You fixed up the hole and the Extinguishing system of the ship turns on. The fire at the controle panel exinguishs and the alarm suddenly stop.\nIt seems that the alarm was caused by a malefunktion of the main system and the ship thought it was in near proximity to a Supernova. You have Survived!", seed),
                        _ => {}
                    }
                }
                2 => {
                    print_reveal("\nThe fire around the control panel grows.\n");
                    match select_num("Do you want to extinguish it or run away?", &["extinguish", "RUN!?!?"]) {
                        1 => end("You extinguished the fire in seconds. Luckily, the fire extinguisher was located nearby. The hole in the hull got bigger and pulled you out into space. You have died!", seed),
                        2 => end("You run. Through time and space. Back to earth. You have survived!", seed),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        3 => {
            print_reveal("\nAs you enter the room, you can either go to a chessboard on a table or to a flux capacitor, which can reverse the time.\n");
            match select_num("What do you want to do?", &["chessboard", "flux capacitor"]) {
                1 => {
                    print_reveal("\nYou start to play chess with the Supernova.\n");
                    match select_num("You know to play the Damen Gambit Opening and win or you can also try to throw the game.", &["Damen Gambit", "Throw"]) {
                        1 => end("You open your game with the Damen Gambit Opening. The Supernova is no real opponent for you and you win in the 5 next turns.\nUnfortunately, the Supernova gets angry from the loose and explodes. You have died!", seed),
                        2 => end("You throw, a lot. The Supernova wins and is pleased with it. Afterwards, the Supernova thanks you and flies away. You have Survived!", seed),
                        _ => {}
                    }
                }
                2 => {
                    print_reveal("\nYou go to the Flux Capacitor.\n");
                    match select_num("You can either reverse time or throw the precious Flux Capacitor at the Supernova.", &["reverse time", "throw the precious"]) {
                        1 => end("You accidentally reversed your time completely, now you live your life backwards from this point in time. So technically, you have died!", seed),
                        2 => end("As quickly as you had that thought, you executed it. The Flux Capacitor holds negative energy. Because of that, the Supernova implodes. You have survived!", seed),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}
