mod cube;
use cube::Cube;
use std::io::{stdin, Stdout, Write};

fn main() {
    println!("CUBE");
    loop {
        let mut input = String::from("");
        let mut cube = Cube::new();
        println!("Enter your moves: ");
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            return;
        }
        cube.input(input.trim());
        cube.print();
    }
}
