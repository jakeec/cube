mod cube;
use cube::Cube;
use getch::Getch;
use std::io;
use std::io::{stdin, Read, Stdin, StdinLock, Stdout, Write};
use std::str;

fn main() {
    println!("CUBE");
    let mut cube = Cube::new();
    let getch = Getch::new();
    loop {
        let input = getch.getch().unwrap();
        cube.input(str::from_utf8(&[input]).unwrap());
        cube.print();
    }
    // loop {
    //     let mut input = String::from("");
    //     println!("Enter your moves: ");
    //     stdin().read_line(&mut input).unwrap();
    //     cube.input(input.trim());
    //     cube.print();
    // }
}
