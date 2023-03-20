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
    println!("ein Textadventure von Nils Wrenger\n");
    thread::sleep(Duration::from_secs_f64(1.0));
    loop {
        let num = loop {
            if let Some(num) = question("Du befindest dich auf deinem Raumschiff. Du bist auf dem Weg zu deinem Heimat Planeten, die Erde.\nDu hattest gerade deine letze Mission des Monats beendet und willst dich endlich mal ein bisschen entspannen.\nDu legst dich in dein Bett und lässt den Autopilot das Raumschiff steuern. Aber dann hörst einen Schiffsalarm.\nDu stehst auf. Da du immer die Tür zu deinem Quartier mit einem altmodischen Schlüssel abschließt, brauchst du diesen. Wo liegt der Schlüssel?\n(Hosen(1), Medizin Schrank(2), unterm Bett(3), wieder sich ins Bett legen(4))", 4) {
                break num;
            }
        };
        if num == 1 || num == 2 {
            println!("\nDa findest du leider nichts\n");
        }
        if num == 3 {
            loop {
                let num2: usize = loop {
                    if let Some(num2) = question(r#"
Da ist der Schlüssel.
Du schließt die Tür auf.
     ┌───┐     
    ┌┘   └┐    
    │  1  │    
┌───┼─────┼───┐
│ Y │     │ 2 │
└───┴─────┴───┘
Du kannst zur Brücke, um zu gucken was los ist oder zum Reaktor, um den Alarm abzustellen.
(Deine Position(Y), Brücke(1), Reaktor(2))"#, 2){
                        break num2;
                        }
                    };
                if num2 == 1{
                    println!("\nDu gehst zur Brücke.\n");
                    loop{
                        let num3: usize = loop {
                            if let Some(num3) = question("In der Brücke kannst du dir die Steuer Konsole ansehen oder eine nervige Fliege angreifen.\n(Steuer Konsole(1), nervige Fliege(2))\n", 2) {
                                break num3;
                            }
                        };     
                if num3 == 1 {
                    println!("Auf der Steuerkonsole siehst du, wie in den nächsetn 5 Minuten ein Supernova das Schiff wegfegen wird.");
                    loop {
                        let num4: usize = loop {
                            if let Some(num4) = question("Willst du hinter ein Mond in der nähe Fliegen oder nichts machen und auf dein Tod warten?\n(Hinter Mond fliegen(1), Auf Tod warten(2))\n", 2) {
                                break num4; 
                            }
                        };
                        if num4 == 1 {
                            println!("Hat der Mond dich beschützt? Ja, da der Mond einen Zitanium Anteil hat, der jede Art von Energie, Schockwellen, etc. reflektiert.\nDu legst dich wieder hin, schaltest den Autopilot ein und genießt den Schlaf und die Erholung. Du hast überlebt!\n");
                            return;
                        }
                        if num4 == 2 {
                            println!("Das hast du dann auch geatan...Du bist gestorben!\n");
                            return;
                        }
                    }
                }
                if num3 == 2 {
                    println!("Du gehst zu nerfigen Fliege\n");
                    loop {
                        let num5:usize = loop {
                            if let Some(num5) = question("Willst du die Fliege Töten oder mit ihr Verhandel?\n(Töten(1), Verhandeln(2))\n", 2){
                                break num5;
                            }
                    };
                    if num5 == 1 {
                        println!("Du benutzt deinen Laser und machst der Fliege 2d4 Schaden.\nDabei Sprengst du leider ein Loch in die Hülle des Schiffes und alles Fliegt ins Vakuum des Weltalls...Auch du. Du bist Gestorben!\n");
                        return;
                    }
                    if num5 == 2 {
                        println!("Du verhandelst mit der Fliege aus, dass sie weniger rum nervt und gehst zur Steuer Konsole.\nLeider war es schon zu Spät. Du konntest nur noch einen Timer sehen, der anzeigt wann die Supernova dich erreicht.\nDieser war gerade auf 00:00 gefallen...Du bist gestorben!\n");
                        return;
                    }
                }
            }
                
        }
    }
                if num2 == 2{
                    println!("\nDu gehst zum Reaktor.\n");
                    loop {
                        let num6: usize = loop{
                            if let Some(num6) = question("Du kannst den Alarm Deaktivieren und wieder ins Bett gehen oder wieder zurück in den vorherigen Raum gehen.\n(Alarm Deaktivieren und wieder schlafen gehen(1), wieder zurückgehen(2))\n", 2) {
                                break num6;
                            }
                        };
                    if num6 == 1 {
                        println!("Du gehst wieder Schlafen. Aber da war doch was? Ja eine Supernova...Du bist gestorben!\n");
                        return;
                    }     
                    
                    if num6 == 2 {
                        println!("Du gehst zum vorherigen Raum zurück.\n");
                        break;
                    }
                }
            }
        }
    }

        if num == 4 {
            println!("\nZum Glück überlebst du. Aber von was oder wie? Der Schiffsalarm kam von einer anstehenden Supernova.\nAber wegen einer Fliege, die ausversehen nen Knopf auf der Brücke gedrückt hatte, hat sich das Schiff in Bewegung gesetzt und hinter einem Mond angehalten. Du hast überlebt! Und nicht mal gewusst in was für einer Gefahr du warst!\n");
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
