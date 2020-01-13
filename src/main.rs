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
                vec![Color::Black; 3],
                vec![Color::Blue; 3],
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
            Color::White => print!("\u{25A0} "),
            Color::Yellow => print!("\x1b[0;35m\u{25A0}\x1b[0m "),
            Color::Orange => print!("\x1b[0;33m\u{25A0}\x1b[0m "),
            Color::Red => print!("\x1b[38;5;160m\u{25A0}\x1b[0m "),
            Color::Green => print!("\x1b[0;32m\u{25A0}\x1b[0m "),
            Color::Blue => print!("\x1b[34;5m\u{25A0}\x1b[0m "),
            Color::Black => print!("\x1b[0;30m\u{25A0}\x1b[0m "),
        }
    }

    fn face_cw(face: &Face) -> Face {
        let face = match face {
            Face(face) => face,
        };
        let mut new = face.clone();
        for i in 0..face.len() {
            for j in 0..face[i].len() {
                new[j][face.len() - 1 - i] = face[i][j];
            }
        }
        Face(new)
    }

    fn face_ccw(face: &Face) -> Face {
        let face = match face {
            Face(face) => face,
        };
        let mut new = face.clone();
        for i in 0..face.len() {
            for j in 0..face[i].len() {
                new[i][j] = face[j][face[i].len() - 1 - i];
            }
        }
        Face(new)
    }

    fn rotate_slice(
        faces: (&Face, &Face, &Face, &Face),
        row: usize,
    ) -> (Vec<Color>, Vec<Color>, Vec<Color>, Vec<Color>) {
        let (first, second, third, fourth) = match (faces.0, faces.1, faces.2, faces.3) {
            (Face(first), Face(second), Face(third), Face(fourth)) => (
                first.to_owned(),
                second.to_owned(),
                third.to_owned(),
                fourth.to_owned(),
            ),
        };

        (
            first[row].to_owned(),
            second[row].to_owned(),
            third[row].to_owned(),
            fourth[row].to_owned(),
        )
    }

    fn up_cw(&mut self) {
        self.Up = Self::face_cw(&self.Up);

        let (left, front, right, back) =
            Self::rotate_slice((&self.Left, &self.Front, &self.Right, &self.Back), 0);

        self.Back.0[0] = left;
        self.Right.0[0] = back;
        self.Front.0[0] = right;
        self.Left.0[0] = front;
    }

    fn up_ccw(&mut self) {
        self.Up = Self::face_ccw(&self.Up);

        let (left, front, right, back) =
            Self::rotate_slice((&self.Left, &self.Front, &self.Right, &self.Back), 0);

        self.Back.0[0] = right;
        self.Right.0[0] = front;
        self.Front.0[0] = left;
        self.Left.0[0] = back;
    }
}

fn main() {
    let mut cube = Cube::new();
    cube.print();
    cube.up_cw();
    println!("");
    cube.print();
    cube.up_ccw();
    println!("");
    cube.print();
}
