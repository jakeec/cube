mod cube;
use cube::Cube;
use std::io::{stdin, Stdout, Write};

fn main() {
    let mut input: String = String::from("");
    println!("CUBE");
    loop {
        input = String::from("");
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
