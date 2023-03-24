use std::thread;
use std::{io, time::Duration};



fn main() {
    println!(
        r#"
 _____                           _                  __                         
|  ___|                         (_)                / _|                        
| |__   ___   ___   __ _  _ __   _  _ __    __ _  | |_  _ __   ___   _ __ ___  
|  __| / __| / __| / _` || '_ \ | || '_ \  / _` | |  _|| '__| / _ \ | '_ ` _ \ 
| |___ \__ \| (__ | (_| || |_) || || | | || (_| | | |  | |   | (_) || | | | | |
\____/ |___/ \___| \__,_|| .__/ |_||_| |_| \__, | |_|  |_|    \___/ |_| |_| |_|
                        | |                __/ |                              
                        |_|               |___/                               "#);

    thread::sleep(Duration::from_secs_f64(1.5));
    println!(
        r#"
 _    _             _____                                                       
| |  | |           /  ___|                                                      
| |_ | |__    ___  \ `--.  _   _  _ __    ___  _ __  _ __    ___  __   __  __ _ 
| __|| '_ \  / _ \  `--. \| | | || '_ \  / _ \| '__|| '_ \  / _ \ \ \ / / / _` |  TM
| |_ | | | ||  __/ /\__/ /| |_| || |_) ||  __/| |   | | | || (_) | \ V / | (_| |
 \__||_| |_| \___| \____/  \__,_|| .__/  \___||_|   |_| |_| \___/   \_/   \__,_|
                                 | |                                            
                                 |_|                                            "#);
    thread::sleep(Duration::from_secs_f64(1.5));
    println!("a Textadventure from Nils Wrenger\n");
    thread::sleep(Duration::from_secs_f64(1.0));
    loop {
        let num = loop {
            if let Some(num) = question("You are on a spaceship. You are on your way to your home planet, Earth.\nYou just finished your last mission of the month and want to relax a bit.\nYou lie down in your bed and let the autopilot control the spacecraft. But then you hear a ship alarm.\nYou get up. Since you always lock the door to your quarters with an old-fashioned key, you need it. Where is the key?\n(Pants(1), Medicine Carbinet(2), under the bed(3), go back to sleep(4))", 4) {
                break num;
            }
        };
        if num == 1 || num == 2 {
            println!("\nUnfortunately, you don't find anything there.\n");
        }
        if num == 3 {
            loop {
                let num2: usize = loop {
                    if let Some(num2) = question(r#"
There's the key.
You unlock the door.
     ┌───┐     
    ┌┘   └┐    
    │  1  │    
┌───┼─────┼───┐
│ Y │     │ 2 │
└───┴─────┴───┘
You can go to the bridge to see what's going on or to the reactor to turn off the alarm.
(Your position(Y), Bridge(1), Reactor(2))"#, 2){
                        break num2;
                        }
                    };
                if num2 == 1{
                    println!("\nYou go to the bridge.\n");
                    loop{
                        let num3: usize = loop {
                            if let Some(num3) = question("In the bridge, you can take a look at the control console or attack an annoying fly.\n(control console(1), annoying Fly(2))\n", 2) {
                                break num3;
                            }
                        };     
                if num3 == 1 {
                    println!("On the control console, you see that a supernova will obliterate the ship and you in the next 5 minutes.");
                    loop {
                        let num4: usize = loop {
                            if let Some(num4) = question("Do you want to fly behind a moon nearby or do nothing and wait for your death?\n(Fly behind moon(1), Wait for death(2))\n", 2) {
                                break num4; 
                            }
                        };
                        if num4 == 1 {
                            println!("Did the moon protect you? Yes, as the moon has a zitanium component that reflects any kind of energy, shockwaves, etc.\nYou lie down again, turn on the autopilot, and enjoy the sleep and relaxation. You have survived!\n");
                            return;
                        }
                        if num4 == 2 {
                            println!("You did that too...You died!\n");
                            return;
                        }
                    }
                }
                if num3 == 2 {
                    println!("You go to the annoying fly.\n");
                    loop {
                        let num5:usize = loop {
                            if let Some(num5) = question("Do you want to kill the fly or negotiate with it?\n(kill(1), negotiate(2))\n", 2){
                                break num5;
                            }
                    };
                    if num5 == 1 {
                        println!("ou use your laser and deal 2d4 damage to the fly.\nUnfortunately, you also blast a hole in the hull of the ship, and everything gets sucked out into the vacuum of space, including you. You have died!\n");
                        return;
                    }
                    if num5 == 2 {
                        println!("You negotiate with the fly and convince it to be less annoying before heading to the control console.\nUnfortunately, it was already too late. You could only see a timer indicating when the supernova would reach you,\nand it had just reached 00:00. You have died!\n");
                        return;
                    }
                }
            }
                
        }
    }
                if num2 == 2{
                    println!("\nYou go to the reactor.\n");
                    loop {
                        let num6: usize = loop{
                            if let Some(num6) = question("You can deactivate the alarm and go back to sleep, or go back to the previous room.\n(Deactivate alarm and go back to sleep(1), Go back(2))\n", 2) {
                                break num6;
                            }
                        };
                    if num6 == 1 {
                        println!("You go back to sleep. But wait, there was something you forgot? Oh, right, a supernova... You have died!\n");
                        return;
                    }     
                    
                    if num6 == 2 {
                        println!("You go back to the previous room.\n");
                        break;
                    }
                }
            }
        }
    }

        if num == 4 {
            println!("\nLuckily, you survived. But how? The ship's alarm was triggered by an impending supernova.\nHowever, because of a fly that accidentally pressed a button on the bridge, the ship had started moving and had come to a stop behind a moon. You have survived! And you didn't even know what danger you were in!\n");
            return;
        }
    }
}

fn question(q: &str, a: usize) -> Option<usize> {
    println!("{q}");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();

    let Ok(num) = guess.trim().parse() else {
        println!("Wrong Input, give Numbers\n");
        return None;
    };

    if num <= a {
        Some(num)
    } else {
        println!("Number out of Range\n");
        None
    }
}
