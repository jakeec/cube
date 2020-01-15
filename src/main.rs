mod cube;
use cube::Cube;
use std::io::{stdin, Read, Stdout, Write};

fn main() {
    println!("CUBE");
    let mut cube = Cube::new();
    loop {
        let mut input = String::from("");
        println!("Enter your moves: ");
        stdin().read_line(&mut input).unwrap();
        cube.input(input.trim());
        cube.print();
    }
}
