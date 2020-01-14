use std::ops::{Index, IndexMut};

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

impl Clone for Face {
    fn clone(&self) -> Self {
        Face(self.0.clone())
    }
}

impl Index<usize> for Face {
    type Output = Vec<Color>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.0[row]
    }
}

impl IndexMut<usize> for Face {
    fn index_mut(&mut self, row: usize) -> &mut Vec<Color> {
        &mut self.0[row]
    }
}

#[derive(Debug)]
pub struct Cube {
    Up: Face,
    Down: Face,
    Left: Face,
    Right: Face,
    Front: Face,
    Back: Face,
    Void: Face,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            Up: Face::new(Color::White),
            Down: Face::new(Color::Yellow),
            Left: Face::new(Color::Orange),
            Right: Face::new(Color::Red),
            Front: Face::new(Color::Green),
            Back: Face::new(Color::Blue),
            Void: Face::new(Color::Black),
        }
    }

    pub fn input(&mut self, instructions: &str) {
        let instructions: Vec<char> = instructions.chars().collect();
        for i in 0..instructions.len() {
            println!("{:?}\n", self);
            let mut prime = false;
            if i + 1 > instructions.len() - 1 {
            } else {
                prime = instructions[i + 1] == '\'';
            }
            match instructions[i] {
                'U' => self.up(prime),
                'D' => self.down(prime),
                'R' => self.right(prime),
                'L' => self.left(prime),
                _ => (),
            }
        }

        println!("{:?}\n", self);
    }

    fn up(&mut self, prime: bool) {
        self.Up = Self::face_cw(&self.Up);
        let temp = vec![self.Front[0][0], self.Front[0][1], self.Front[0][2]];
        for i in 0..3 {
            self.Front[0][i] = self.Right[0][i];
            self.Right[0][i] = self.Back[0][i];
            self.Back[0][i] = self.Left[0][i];
            self.Left[0][i] = temp[i];
        }
    }

    fn down(&mut self, prime: bool) {
        self.Down = Self::face_cw(&self.Down);
        let temp = vec![self.Front[2][0], self.Front[2][1], self.Front[2][2]];
        for i in 0..3 {
            self.Front[2][i] = self.Left[2][i];
            self.Left[2][i] = self.Back[2][i];
            self.Back[2][i] = self.Right[2][i];
            self.Right[2][i] = temp[i];
        }
    }

    fn left(&mut self, prime: bool) {
        self.Right = Self::face_cw(&self.Right);
        let temp = vec![self.Up[0][2], self.Up[1][2], self.Up[2][2]];
        for i in 0..3 {
            self.Up[i][0] = self.Back[i][2];
            self.Back[i][2] = self.Down[i][0];
            self.Down[i][0] = self.Front[i][2];
            self.Front[i][0] = temp[i];
        }
    }

    fn right(&mut self, prime: bool) {
        self.Right = Self::face_cw(&self.Right);
        let temp = vec![self.Up[0][2], self.Up[1][2], self.Up[2][2]];
        for i in 0..3 {
            self.Up[i][2] = self.Front[i][2];
            self.Front[i][2] = self.Down[i][2];
            self.Down[i][2] = self.Back[i][0];
            self.Back[i][0] = temp[i];
        }
    }

    pub fn print(&self) {
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Up, 0);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Up, 1);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Up, 2);
        print!("\n");
        Self::print_row(&self.Left, 0);
        Self::print_row(&self.Front, 0);
        Self::print_row(&self.Right, 0);
        Self::print_row(&self.Back, 0);
        print!("\n");
        Self::print_row(&self.Left, 1);
        Self::print_row(&self.Front, 1);
        Self::print_row(&self.Right, 1);
        Self::print_row(&self.Back, 1);
        print!("\n");
        Self::print_row(&self.Left, 2);
        Self::print_row(&self.Front, 2);
        Self::print_row(&self.Right, 2);
        Self::print_row(&self.Back, 2);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Down, 2);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Down, 1);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Down, 0);
    }

    fn print_row(face: &Face, row: usize) {
        match face {
            Face(squares) => {
                for square in &squares[row] {
                    Self::draw_square(*square)
                }
            }
        }
    }

    fn print_row_reverse(face: &Face, row: usize) {
        match face {
            Face(squares) => {
                for i in 0..squares.len() {
                    Self::draw_square(squares[row][squares.len() - 1 - i])
                }
            }
        }
    }

    fn draw_square(color: Color) {
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

    fn rotate_row_cw(faces: (&mut Face, &mut Face, &mut Face, &mut Face), row: usize) {
        let (first, second, third, fourth) = match (
            (faces.0).0.to_owned(),
            (faces.1).0.to_owned(),
            (faces.2).0.to_owned(),
            (faces.3).0.to_owned(),
        ) {
            (first, second, third, fourth) => (
                first.to_owned(),
                second.to_owned(),
                third.to_owned(),
                fourth.to_owned(),
            ),
        };

        (faces.0).0[row] = second[row].to_owned();
        (faces.1).0[row] = third[row].to_owned();
        (faces.2).0[row] = fourth[row].to_owned();
        (faces.3).0[row] = first[row].to_owned();
    }

    fn rotate_row_ccw(faces: (&mut Face, &mut Face, &mut Face, &mut Face), row: usize) {
        let (first, second, third, fourth) = match (
            (faces.0).0.to_owned(),
            (faces.1).0.to_owned(),
            (faces.2).0.to_owned(),
            (faces.3).0.to_owned(),
        ) {
            (first, second, third, fourth) => (
                first.to_owned(),
                second.to_owned(),
                third.to_owned(),
                fourth.to_owned(),
            ),
        };

        (faces.0).0[row] = fourth[row].to_owned();
        (faces.1).0[row] = first[row].to_owned();
        (faces.2).0[row] = second[row].to_owned();
        (faces.3).0[row] = third[row].to_owned();
    }

    fn rotate_col_cw(faces: (&mut Face, &mut Face, &mut Face, &mut Face), col: usize) {
        let (first, second, third, fourth) = match (
            (faces.0).0.to_owned(),
            (faces.1).0.to_owned(),
            (faces.2).0.to_owned(),
            (faces.3).0.to_owned(),
        ) {
            (first, second, third, fourth) => (
                first.to_owned(),
                second.to_owned(),
                third.to_owned(),
                fourth.to_owned(),
            ),
        };

        for i in 0..3 {
            (faces.0).0[i][col] = fourth[i].to_owned()[col];
            (faces.1).0[i][col] = first[i].to_owned()[col];
            (faces.2).0[i][col] = second[i].to_owned()[col];
            (faces.3).0[i][col] = third[i].to_owned()[col];
        }
    }

    pub fn up_cw(&mut self) {
        self.Up = Self::face_cw(&self.Up);

        Self::rotate_row_cw(
            (
                &mut self.Left,
                &mut self.Front,
                &mut self.Right,
                &mut self.Back,
            ),
            0,
        );
    }

    pub fn up_ccw(&mut self) {
        self.Up = Self::face_ccw(&self.Up);
        Self::rotate_row_ccw(
            (
                &mut self.Left,
                &mut self.Front,
                &mut self.Right,
                &mut self.Back,
            ),
            0,
        );
    }

    pub fn down_cw(&mut self) {
        self.Down = Self::face_cw(&self.Down);

        Self::rotate_row_cw(
            (
                &mut self.Left,
                &mut self.Front,
                &mut self.Right,
                &mut self.Back,
            ),
            2,
        );
    }

    pub fn down_ccw(&mut self) {
        self.Up = Self::face_ccw(&self.Up);
        Self::rotate_row_ccw(
            (
                &mut self.Left,
                &mut self.Front,
                &mut self.Right,
                &mut self.Back,
            ),
            0,
        );
    }

    pub fn right_cw(&mut self) {
        self.Right = Self::face_cw(&self.Right);
        Self::rotate_col_cw(
            (
                &mut self.Up,
                &mut self.Back,
                &mut self.Down,
                &mut self.Front,
            ),
            2,
        );
    }

    pub fn right_ccw(&mut self) {
        self.Right = Self::face_cw(&self.Right);
        Self::rotate_col_cw(
            (
                &mut self.Up,
                &mut self.Front,
                &mut self.Down,
                &mut self.Back,
            ),
            2,
        );
    }

    pub fn left_cw(&mut self) {
        self.Left = Self::face_cw(&self.Left);
        Self::rotate_col_cw(
            (
                &mut self.Up,
                &mut self.Front,
                &mut self.Down,
                &mut self.Back,
            ),
            0,
        );
    }

    pub fn left_ccw(&mut self) {
        self.Left = Self::face_cw(&self.Left);
        Self::rotate_col_cw(
            (
                &mut self.Up,
                &mut self.Back,
                &mut self.Down,
                &mut self.Front,
            ),
            0,
        );
    }
}
