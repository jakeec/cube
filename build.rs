extern crate dirs;
use dirs::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("Checking for .cuberc file...");
    let home = home_dir().unwrap();

    let dotfile = format!("{}/.cuberc", home.to_str().unwrap());
    if Path::new(&dotfile).exists() {
    } else {
        println!(".cuberc not found in home directory! Creating .cuberc...");
        let mut file = File::create(dotfile).unwrap();
        let default_config = fs::read_to_string(".cuberc").unwrap();
        file.write_all(default_config.as_ref()).unwrap();
    }
}
