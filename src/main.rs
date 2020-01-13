use std::string::String;

#[derive(Clone, Copy, Debug)]
enum Color {
    Red,
    Green,
    Blue,
    White,
    Yellow,
    Orange,
    Black,
}

#[derive(Debug)]
struct Face(Vec<Vec<Color>>);

impl Face {
    fn new(color: Color) -> Self {
        Face(vec![vec![color; 3], vec![color; 3], vec![color; 3]])
    }
}

#[derive(Debug)]
struct Cube {
    Up: Face,
    Down: Face,
    Left: Face,
    Right: Face,
    Front: Face,
    Back: Face,
    Void: Face,
}

impl Cube {
    fn new() -> Self {
        Cube {
            Up: Face(vec![
                vec![Color::Red; 3],
                vec![Color::White; 3],
                vec![Color::Green; 3],
            ]),
            Down: Face::new(Color::Yellow),
            Left: Face::new(Color::Orange),
            Right: Face::new(Color::Red),
            Front: Face::new(Color::Green),
            Back: Face::new(Color::Blue),
            Void: Face::new(Color::Black),
        }
    }

    fn print(&self) {
        self.print_row(&self.Void, 0);
        self.print_row(&self.Up, 0);
        print!("\n");
        self.print_row(&self.Void, 0);
        self.print_row(&self.Up, 1);
        print!("\n");
        self.print_row(&self.Void, 0);
        self.print_row(&self.Up, 2);
        print!("\n");
        self.print_row(&self.Left, 0);
        self.print_row(&self.Front, 0);
        self.print_row(&self.Right, 0);
        self.print_row(&self.Back, 0);
        print!("\n");
        self.print_row(&self.Left, 1);
        self.print_row(&self.Front, 1);
        self.print_row(&self.Right, 1);
        self.print_row(&self.Back, 1);
        print!("\n");
        self.print_row(&self.Left, 2);
        self.print_row(&self.Front, 2);
        self.print_row(&self.Right, 2);
        self.print_row(&self.Back, 2);
        print!("\n");
        self.print_row(&self.Void, 0);
        self.print_row(&self.Down, 0);
        print!("\n");
        self.print_row(&self.Void, 0);
        self.print_row(&self.Down, 1);
        print!("\n");
        self.print_row(&self.Void, 0);
        self.print_row(&self.Down, 2);
    }

    fn print_row(&self, face: &Face, row: usize) {
        match face {
            Face(squares) => {
                for square in &squares[row] {
                    self.draw_square(*square)
                }
            }
        }
    }

    fn draw_square(&self, color: Color) {
        match color {
            Color::White => print!("\u{2589} "),
            Color::Yellow => print!("\x1b[0;35m\u{2589}\x1b[0m "),
            Color::Orange => print!("\x1b[0;33m\u{2589}\x1b[0m "),
            Color::Red => print!("\x1b[38;5;160m\u{2589}\x1b[0m "),
            Color::Green => print!("\x1b[0;32m\u{2589}\x1b[0m "),
            Color::Blue => print!("\x1b[34;5m\u{2589}\x1b[0m "),
            Color::Black => print!("\x1b[0;30m\u{2589}\x1b[0m "),
        }
    }

    fn up_clockwise(&mut self) {
        // rotate Up face 90deg
        let face = match &self.Up {
            Face(face) => face,
        };
        let mut new = face.clone();
        for i in 0..face.len() {
            for j in 0..face[i].len() {
                new[i][j] = face[j][i];
            }
        }
        self.Up = Face(new);

        // shift top slice of each horizontal face across three spaces
    }
}

fn main() {
    let mut cube = Cube::new();
    cube.print();
    cube.up_clockwise();
    println!("");
    cube.print();
}
