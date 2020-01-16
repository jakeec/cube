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

#[derive(Debug, Clone)]
pub struct Keys {
    pub quit: char,
    pub up: char,
    pub down: char,
    pub left: char,
    pub right: char,
    pub front: char,
    pub back: char,
    pub up_prime: char,
    pub down_prime: char,
    pub left_prime: char,
    pub right_prime: char,
    pub front_prime: char,
    pub back_prime: char,
}

impl Keys {
    pub fn new() -> Self {
        Keys {
            quit: 'p',
            up: 'w',
            down: 'e',
            left: 'a',
            right: 'f',
            front: 's',
            back: 'd',
            up_prime: 'W',
            down_prime: 'E',
            left_prime: 'A',
            right_prime: 'F',
            front_prime: 'S',
            back_prime: 'D',
        }
    }
}

// impl Clone for Keys {
//     fn clone(&self) -> Self {
//         Keys {
//             quit: self.quit,
//             up: self.up,
//             down: self.down,
//             left: self.left,
//             right: self.right,
//             front: self.front,
//             back: self.back,
//             up_prime: self.up_prime,
//             down_prime: self.down_prime,
//             left_prime: self.left_prime,
//             right_prime: self.right_prime,
//             front_prime: self.front_prime,
//             back_prime: self.back_prime,
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Cube {
    Up: Face,
    Down: Face,
    Left: Face,
    Right: Face,
    Front: Face,
    Back: Face,
    Void: Face,
    keys: Keys,
}

impl Cube {
    pub fn new(keys: &Keys) -> Self {
        Cube {
            Up: Face::new(Color::White),
            Down: Face::new(Color::Yellow),
            Left: Face::new(Color::Orange),
            Right: Face::new(Color::Red),
            Front: Face::new(Color::Green),
            Back: Face::new(Color::Blue),
            Void: Face::new(Color::Black),
            keys: keys.clone(),
        }
    }

    pub fn input(&mut self, instructions: &str) {
        let instructions: Vec<char> = instructions.chars().collect();
        for i in 0..instructions.len() {
            let mut prime = false;
            if i + 1 > instructions.len() - 1 {
            } else {
                prime = instructions[i + 1] == '\'';
            }
            if instructions[i] == self.keys.up {
                self.up(false);
            } else if instructions[i] == self.keys.down {
                self.down(false);
            } else if instructions[i] == self.keys.left {
                self.left(false);
            } else if instructions[i] == self.keys.right {
                self.right(false);
            } else if instructions[i] == self.keys.front {
                self.front(false);
            } else if instructions[i] == self.keys.back {
                // self.back(false);
            } else if instructions[i] == self.keys.up_prime {
                self.up(true);
            } else if instructions[i] == self.keys.down_prime {
                self.down(true);
            } else if instructions[i] == self.keys.left_prime {
                self.left(true);
            } else if instructions[i] == self.keys.right_prime {
                self.right(true);
            } else if instructions[i] == self.keys.front_prime {
                self.front(true);
            } else if instructions[i] == self.keys.back_prime {
                // self.back(false);
            }
            // match instructions[i] {
            //     'w' => self.up(prime),
            //     'e' => self.down(prime),
            //     'f' => self.right(prime),
            //     'a' => self.left(prime),
            //     's' => self.front(prime),
            //     _ => (),
            // }
        }
    }

    pub fn print(&self) {
        print!("{}[2J", 27 as char);
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
        Self::print_row(&self.Down, 0);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Down, 1);
        print!("\n");
        Self::print_row(&self.Void, 0);
        Self::print_row(&self.Down, 2);
        print!("\n");
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

    fn rotate_layer(f1: &mut Face, f2: &mut Face, f3: &mut Face, f4: &mut Face, layer: usize) {
        let temp = vec![f1[layer][0], f1[layer][1], f1[layer][2]];
        for i in 0..3 {
            f1[layer][i] = f2[layer][i];
            f2[layer][i] = f3[layer][i];
            f3[layer][i] = f4[layer][i];
            f4[layer][i] = temp[i];
        }
    }

    fn up(&mut self, prime: bool) {
        match prime {
            true => {
                self.Up = Self::face_ccw(&self.Up);
                Self::rotate_layer(
                    &mut self.Front,
                    &mut self.Left,
                    &mut self.Back,
                    &mut self.Right,
                    0,
                );
            }
            false => {
                self.Up = Self::face_cw(&self.Up);
                Self::rotate_layer(
                    &mut self.Front,
                    &mut self.Right,
                    &mut self.Back,
                    &mut self.Left,
                    0,
                );
            }
        }
    }

    fn down(&mut self, prime: bool) {
        match prime {
            true => {
                self.Down = Self::face_ccw(&self.Down);
                Self::rotate_layer(
                    &mut self.Front,
                    &mut self.Right,
                    &mut self.Back,
                    &mut self.Left,
                    2,
                );
            }
            false => {
                self.Down = Self::face_cw(&self.Down);
                Self::rotate_layer(
                    &mut self.Front,
                    &mut self.Left,
                    &mut self.Back,
                    &mut self.Right,
                    2,
                );
            }
        }
    }

    fn left(&mut self, prime: bool) {
        match prime {
            true => {
                self.Left = Self::face_ccw(&self.Left);
                let temp = vec![self.Up[0][0], self.Up[1][0], self.Up[2][0]];
                for i in 0..3 {
                    self.Up[i][0] = self.Front[i][0];
                    self.Front[i][0] = self.Down[i][0];
                    self.Down[i][0] = self.Back[2 - i][2];
                    self.Back[2 - i][2] = temp[i];
                }
            }
            false => {
                self.Left = Self::face_cw(&self.Left);
                let temp = vec![self.Up[0][0], self.Up[1][0], self.Up[2][0]];
                for i in 0..3 {
                    self.Up[i][0] = self.Back[2 - i][2];
                    self.Back[2 - i][2] = self.Down[i][0];
                    self.Down[i][0] = self.Front[i][0];
                    self.Front[i][0] = temp[i];
                }
            }
        }
    }

    fn right(&mut self, prime: bool) {
        match prime {
            true => {
                self.Right = Self::face_ccw(&self.Right);
                let temp = vec![self.Up[0][2], self.Up[1][2], self.Up[2][2]];
                for i in 0..3 {
                    self.Up[i][2] = self.Back[2 - i][0];
                    self.Back[2 - i][0] = self.Down[i][2];
                    self.Down[i][2] = self.Front[i][2];
                    self.Front[i][2] = temp[i];
                }
            }
            false => {
                self.Right = Self::face_cw(&self.Right);
                let temp = vec![self.Up[0][2], self.Up[1][2], self.Up[2][2]];
                for i in 0..3 {
                    self.Up[i][2] = self.Front[i][2];
                    self.Front[i][2] = self.Down[i][2];
                    self.Down[i][2] = self.Back[2 - i][0];
                    self.Back[2 - i][0] = temp[i];
                }
            }
        }
    }

    fn front(&mut self, prime: bool) {
        match prime {
            true => {
                self.Front = Self::face_ccw(&self.Front);
                let temp = vec![self.Up[2][0], self.Up[2][1], self.Up[2][2]];
                for i in 0..3 {
                    self.Up[2][i] = self.Right[i][0];
                    self.Right[i][0] = self.Down[0][2 - i];
                    self.Down[0][2 - i] = self.Left[2 - i][2];
                    self.Left[2 - i][2] = temp[i];
                }
            }
            false => {
                self.Front = Self::face_cw(&self.Front);
                let temp = vec![self.Up[2][0], self.Up[2][1], self.Up[2][2]];
                for i in 0..3 {
                    self.Up[2][2 - i] = self.Left[i][2];
                    self.Left[i][2] = self.Down[0][i];
                    self.Down[0][i] = self.Right[2 - i][0];
                    self.Right[2 - i][0] = temp[2 - i];
                }
            }
        }
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
}
