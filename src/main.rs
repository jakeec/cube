mod cube;
use cube::Cube;

fn main() {
    let mut cube = Cube::new();
    cube.input("URU");
    println!("");
    cube.print();
}
