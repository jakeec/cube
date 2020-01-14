mod cube;
use cube::Cube;

fn main() {
    let mut cube = Cube::new();
    cube.input("ULDR");
    println!("");
    cube.print();
}
