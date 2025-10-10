use crate::story::{end, intro::print_reveal, part2, select_num};

pub fn init(proc: [u8; 4], seed: u8) {
    match proc[3] {
        1 => {
            print_reveal("\nYou go to the Reactor.\n");
            match select_num("You can deactivate the alarm and go back to sleep, or go back to the previous room.", &["Deactivate alarm and go back to sleep", "Go back"]) {
                1 => end("You go back to sleep. But wait, there was something you forgot? Oh, right, a Supernova... You have died!", seed),
                2 => {
                    print_reveal("\nYou go back to the previous room.\n");
                    part2::init(proc, seed, false);
                }
                _ => {}
            }
        }
        2 => {
            print_reveal("\nYou go to the Reactor.\n");
            match select_num("You can go back to the previous room, or initiate a Reactor meltdown and...die I guess(why the f would you do that).", &["Go back", "Initiate Reactor meltdown"]) {
                1 => {
                    print_reveal("\nYou go back to the previous room.\n");
                    part2::init(proc, seed, false);
                }
                2 => end("As you try to blow yourself up, the ship malfunctions and activates it shields which are barely strong enough to protect you from the Supernova.\nYou didn't die and had to live, even when you tried to kill yourself!", seed),
                _ => {}
            }
        }
        3 => {
            print_reveal("\nYou go to the Reactor.\n");
            match select_num(
                "You can go back to the previous room, or @#($(#J$QIJ#RES)).",
                &["Go back", "_)@#$#IR)@#RK"],
            ) {
                1 => {
                    print_reveal("\nYou go back to the previous room.\n");
                    part2::init(proc, seed, false);
                }
                2 => end("*#$D#*UD#@()$@H#R()@#)(RJH#DHUFH*(W(#$&@#$@#GD*(!@Q#&*$Y@#$YRH&HCJ@()#)_DHUJHFUDHUISHF*DFUHS*()#EYRH@WQ#(*RYU@Q#*(R(ED(FISHUDFY(*@#&QY&*RT@&#*R*(YY*(EYR(*@#&YG@#TYUDGF^SG^TFASTEFD@&*#$*U#*(@%YU*$@Y*YRU*FU()$WE", seed),
                _ => {}
            }
        }
        _ => {}
    }
}
