mod cube;
use cube::{Cube, Keys};
use dirs::home_dir;
use getch::Getch;
use std::fs;
use std::io::stdin;
use std::str;

fn parse_settings(settings: &str) -> Keys {
    let mut section = "";
    let mut keys = Keys::new();
    for line in settings.lines() {
        if line.contains("[clockwise]") {
            section = "cw";
        } else if line.contains("[counterclockwise]") {
            section = "ccw";
        } else if line.contains("[ui]") {
            section = "ui";
        } else {
            let s: Vec<&str> = line.split(" = ").collect();
            if s.len() < 2 {
            } else {
                let (input, key, sec) = (s[0], s[1].chars().next().unwrap(), section);
                match (input, key, sec) {
                    ("quit", _, "ui") => keys.quit = key,
                    ("up", _, "cw") => keys.up = key,
                    ("up", _, "ccw") => keys.up_prime = key,
                    ("down", _, "cw") => keys.down = key,
                    ("down", _, "ccw") => keys.down_prime = key,
                    ("left", _, "cw") => keys.left = key,
                    ("left", _, "ccw") => keys.left_prime = key,
                    ("right", _, "cw") => keys.right = key,
                    ("right", _, "ccw") => keys.right_prime = key,
                    ("front", _, "cw") => keys.front = key,
                    ("front", _, "ccw") => keys.front_prime = key,
                    ("back", _, "cw") => keys.back = key,
                    ("back", _, "ccw") => keys.back_prime = key,
                    _ => (),
                }
            }
        }
    }
    keys
}

enum GameMode {
    Instant,
    Algorithm,
    Help,
    Null,
}

fn main() {
    let home = home_dir().unwrap();
    let dotfile = format!("{}/.cuberc", home.to_str().unwrap());
    let settings = fs::read_to_string(dotfile).unwrap();
    let keys = parse_settings(&settings);
    let mut cube = Cube::new(&keys);
    let getch = Getch::new();
    println!(
        "     ___       ___       ___       ___   
    /\\  \\     /\\__\\     /\\  \\     /\\  \\  
   /::\\  \\   /:/ _/_   /::\\  \\   /::\\  \\ 
  /:/\\:\\__\\ /:/_/\\__\\ /::\\:\\__\\ /::\\:\\__\\
  \\:\\ \\/__/ \\:\\/:/  / \\:\\::/  / \\:\\:\\/  /
   \\:\\__\\    \\::/  /   \\::/  /   \\:\\/  / 
    \\/__/     \\/__/     \\/__/     \\/__/  
    
    
    
    "
    );
    let mut game_mode = GameMode::Null;

    while match game_mode {
        GameMode::Null => true,
        GameMode::Help => {
            println!("Game Modes");
            println!("Regular - Each key you press with instantly make a move on the cube");
            println!("Algorithm - Enter a string of move and execute them all immediately");
            println!("");
            true
        }
        _ => false,
    } {
        println!("Select game mode [1 = Regular, 2 = Algorithm, 3 = Help]");
        let get = &[Getch::new().getch().unwrap()];
        let input = str::from_utf8(get).unwrap();
        game_mode = match input {
            "1" => GameMode::Instant,
            "2" => GameMode::Algorithm,
            "3" => GameMode::Help,
            _ => GameMode::Null,
        };
    }

    match game_mode {
        GameMode::Instant => loop {
            cube.print();
            let input = getch.getch().unwrap();
            if str::from_utf8(&[input]).unwrap().chars().next().unwrap() == keys.quit {
                return;
            }
            cube.input(str::from_utf8(&[input]).unwrap());
        },
        GameMode::Algorithm => loop {
            let mut input = String::from("");
            println!("Enter your moves (or enter 'q' to quit): ");
            stdin().read_line(&mut input).unwrap();
            if input.contains("q") {
                return;
            }
            cube.string_input(input.trim());
            cube.print();
        },
        _ => panic!("Not a valid game mode!"),
    }
}
