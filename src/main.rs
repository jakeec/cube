mod cube;
use cube::Cube;

fn main() {
    let mut cube = Cube::new();
    // cube.up_cw();
    // cube.up_cw();
    cube.right_cw();
    cube.down_cw();
    println!("");
    cube.print();
}
