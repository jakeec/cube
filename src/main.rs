mod cube;
use cube::{Cube, Keys};
use dirs::home_dir;
use getch::Getch;
use std::fs;
use std::io::stdin;
use std::io::{self, Write};
use std::str;
use termcolor::{Color as TermColor, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

fn print_selected(message: &str, selected: bool) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    print!("\t");
    if selected {
        stdout
            .set_color(
                ColorSpec::new()
                    .set_bg(Some(TermColor::Blue))
                    .set_fg(Some(TermColor::Black)),
            )
            .unwrap();
    }
    print!("{}", message);
    stdout.set_color(ColorSpec::new().set_bg(None)).unwrap();
    print!("\n");
}

fn main() {
    let home = home_dir().unwrap();
    let dotfile = format!("{}/.cuberc", home.to_str().unwrap());
    let settings = fs::read_to_string(dotfile).unwrap();
    let keys = parse_settings(&settings);
    let mut cube = Cube::new(&keys);
    let mut game_mode = GameMode::Null;

    let mut selected = false;
    let mut selected_game_mode = 1;

    // j = 106
    // k = 107

    while !selected {
        print!("{}[2J", 27 as char);
        game_mode = match selected_game_mode {
            1 => GameMode::Instant,
            2 => GameMode::Algorithm,
            3 => GameMode::Help,
            _ => GameMode::Null,
        };
        println!(
            "         ___       ___       ___       ___   
        /\\  \\     /\\__\\     /\\  \\     /\\  \\  
       /::\\  \\   /:/ _/_   /::\\  \\   /::\\  \\ 
      /:/\\:\\__\\ /:/_/\\__\\ /::\\:\\__\\ /::\\:\\__\\
      \\:\\ \\/__/ \\:\\/:/  / \\:\\::/  / \\:\\:\\/  /
       \\:\\__\\    \\::/  /   \\::/  /   \\:\\/  / 
        \\/__/     \\/__/     \\/__/     \\/__/  
        
        "
        );
        match game_mode {
            GameMode::Null => {}
            GameMode::Help => {
                println!("");
                println!("Game Modes");
                println!("Regular - Each key you press will instantly make a move on the cube");
                println!("Algorithm - Enter a string of moves and execute them all immediately");
                println!("");
            }
            _ => {}
        }
        println!("Select game mode:\n");
        print_selected(
            "Normal",
            match game_mode {
                GameMode::Instant => true,
                _ => false,
            },
        );
        print_selected(
            "Algorithm",
            match game_mode {
                GameMode::Algorithm => true,
                _ => false,
            },
        );
        print_selected(
            "Help",
            match game_mode {
                GameMode::Help => true,
                _ => false,
            },
        );
        let get = &[Getch::new().getch().unwrap()];
        match get {
            [10] => match game_mode {
                GameMode::Algorithm => {
                    selected = true;
                    break;
                }
                GameMode::Instant => {
                    selected = true;
                    break;
                }
                _ => (),
            },
            [106] => {
                selected_game_mode += 1;
                if selected_game_mode > 3 {
                    selected_game_mode = 3;
                }
            }
            [107] => {
                selected_game_mode -= 1;
                if selected_game_mode == 0 {
                    selected_game_mode = 1;
                }
            }
            _ => println!("{:?}", get),
        }
        let input = str::from_utf8(get).unwrap();
    }

    match game_mode {
        GameMode::Instant => loop {
            cube.print();
            let input = Getch::new().getch().unwrap();
            if str::from_utf8(&[input]).unwrap().chars().next().unwrap() == keys.quit {
                return;
            }
            cube.input(str::from_utf8(&[input]).unwrap());
        },
        GameMode::Algorithm => loop {
            cube.print();
            let mut input = String::from("");
            println!("Enter your moves (or enter 'q' to quit): ");
            stdin().read_line(&mut input).unwrap();
            if input.contains("\u{1b}[A") {
                return;
            }
            if input.contains("q") {
                return;
            }
            cube.string_input(input.trim());
        },
        _ => panic!("Not a valid game mode!"),
    }
}
