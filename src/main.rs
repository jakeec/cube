mod cube;
use cube::{Cube, Keys};
use getch::Getch;
use std::env::home_dir;
use std::fs;
use std::io;
use std::io::{stdin, Read, Stdin, StdinLock, Stdout, Write};
use std::path::Path;
use std::str;

fn parse_settings(settings: &str) -> Keys {
    let mut section = "";
    let mut keys = Keys::new();
    for line in settings.lines() {
        println!("{:?}", keys);
        if line.contains("[clockwise]") {
            section = "cw";
        } else if line.contains("[counterclockwise]") {
            section = "ccw";
        } else {
            let s: Vec<&str> = line.split(" = ").collect();
            if s.len() < 2 {
            } else {
                let (dir, key, sec) = (s[0], s[1].chars().next().unwrap(), section);
                match (dir, key, sec) {
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

fn main() {
    let home = home_dir().unwrap();
    let dotfile = format!("{}/.cuberc", home.to_str().unwrap());
    let settings = fs::read_to_string(dotfile).unwrap();
    let keys = parse_settings(&settings);
    println!("{:?}", settings);
    let mut cube = Cube::new(keys);
    let getch = Getch::new();
    loop {
        cube.print();
        let input = getch.getch().unwrap();
        cube.input(str::from_utf8(&[input]).unwrap());
    }
    // loop {
    //     let mut input = String::from("");
    //     println!("Enter your moves: ");
    //     stdin().read_line(&mut input).unwrap();
    //     cube.input(input.trim());
    //     cube.print();
    // }
}
