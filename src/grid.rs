use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::gaddag::{Gaddag, GaddagNode};

pub const GRID_SIZE: usize = 15;
pub const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Debug, Clone, Copy)]
pub enum Square {
    Blank,
    LCD,
    LCT,
    LCQ,
    MCD,
    MCT,
    MCQ,
    Letter(char),
}

pub struct Grid {
    pub squares: [[Square; GRID_SIZE]; GRID_SIZE],
    anchors: [[bool; GRID_SIZE]; GRID_SIZE],
    crosswords: [[Option<HashMap<char, i32>>; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            squares: [[Square::Blank; GRID_SIZE]; GRID_SIZE],
            anchors: [[false; GRID_SIZE]; GRID_SIZE],
            crosswords: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }

    pub fn generate_grid(&mut self) {
        let lcd_list = [(0, 3), (2, 6), (3, 0), (3, 7), (6, 2), (6, 6), (7, 3)];
        let lct_list = [(1, 5), (5, 1), (5, 5)];
        let lcq_list: [(usize, usize); 0] = [];
        let mcd_list = [(1, 1), (2, 2), (3, 3), (4, 4)];
        let mct_list = [(0, 0), (0, 7), (7, 0)];
        let mcq_list: [(usize, usize); 0] = [];

        for &(x, y) in &lcd_list {
            self.set_bonus(Square::LCD, x, y);
        }
        for &(x, y) in &lct_list {
            self.set_bonus(Square::LCT, x, y);
        }
        for &(x, y) in &lcq_list {
            self.set_bonus(Square::LCQ, x, y);
        }
        for &(x, y) in &mcd_list {
            self.set_bonus(Square::MCD, x, y);
        }
        for &(x, y) in &mct_list {
            self.set_bonus(Square::MCT, x, y);
        }
        for &(x, y) in &mcq_list {
            self.set_bonus(Square::MCQ, x, y);
        }
    }

    fn set_bonus(&mut self, case: Square, x: usize, y: usize) {
        let n = GRID_SIZE - 1;
        self.squares[x][y] = case;
        self.squares[x][n - y] = case;
        self.squares[n - x][y] = case;
        self.squares[n - x][n - y] = case;
    }

    pub fn update_anchors(&mut self) {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                self.anchors[i][j] = false;
                if self.is_empty(i, j) {
                    let b1 = self.is_empty(i.wrapping_sub(1), j);
                    let b2 = self.is_empty(i + 1, j);
                    let b3 = self.is_empty(i, j.wrapping_sub(1));
                    let b4 = self.is_empty(i, j + 1);
                    self.anchors[i][j] = !(b1 && b2 && b3 && b4);
                }
            }
        }
        if self.anchors.iter().all(|row| row.iter().all(|&a| !a)) {
            self.anchors[7][7] = true; // Activer l'ancre au centre si la grille est vide
        }
    }

    pub fn is_empty(&self, i: usize, j: usize) -> bool {
        if i >= GRID_SIZE || j >= GRID_SIZE {
            return true;
        }
        match self.squares[i][j] {
            Square::Letter(_) => false,
            _ => true,
        }
    }

    pub fn update_crosswords(&mut self, gaddag: GaddagNode) {
        let adj = |x: usize, y: usize| {
            let mut up_letters = String::new();
            let mut down_letters = String::new();

            let mut i = x;
            while i > 0 {
                i -= 1;
                let c = self.squares[i][y];
                if let Square::Letter(letter) = c {
                    up_letters.push(letter);
                } else {
                    break;
                }
            }

            i = x;
            while i < GRID_SIZE - 1 {
                i += 1;
                let c = self.squares[i][y];
                if let Square::Letter(letter) = c {
                    down_letters.push(letter);
                } else {
                    break;
                }
            }

            (up_letters, down_letters)
        };

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                self.crosswords[x][y] = None;
                if self.anchors[x][y] {
                    let (up_letters, down_letters) = adj(x, y);
                    for &c in &ALPHABET {
                        let word = format!("{}{}!{}", c, up_letters, down_letters);
                        if Gaddag::contains_word(&word, Rc::clone(&gaddag)) {
                            let entry = self.crosswords[x][y].get_or_insert_with(HashMap::new);
                            entry.insert(c, 0);
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print the grid
        writeln!(f, "Grid:")?;
        for i in 0..self.squares.len() {
            for j in 0..self.squares[i].len() {
                let square = &self.squares[i][j];
                write!(
                    f,
                    "{} ",
                    match square {
                        Square::Letter(c) => c.to_string(),
                        _ => ".".to_string(),
                    }
                )?;
            }
            writeln!(f)?;
        }

        // Print the anchors
        writeln!(f, "\nAnchors:")?;
        for i in 0..self.anchors.len() {
            for j in 0..self.anchors[i].len() {
                let anchor = self.anchors[i][j];
                write!(f, "{} ", if anchor { "x" } else { "." })?;
            }
            writeln!(f)?;
        }

        // Print the crosswords
        writeln!(f, "\nCrosswords:")?;
        for row in &self.crosswords {
            for cell in row {
                match cell {
                    None => write!(f, ". ")?,
                    Some(map) => {
                        if map.is_empty() {
                            write!(f, "/ ")?;
                        } else {
                            let random_char = map.keys().next().unwrap();
                            write!(f, "{} ", random_char)?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
