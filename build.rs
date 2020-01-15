use std::fs::File;
use std::io::Write;

fn main() {
    println!("Running build scripts...");
    let mut file = File::create("jake.txt").unwrap();
    file.write_all(b"Jake").unwrap();
}
