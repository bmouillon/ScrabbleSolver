use std::collections::HashMap;
use std::fmt;

use crate::constants::{ALPHABET, BONUS_CELLS, GRID_SIZE, LETTERS_VALUE};
use crate::gaddag::{Gaddag, GaddagNode};

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
    pub anchors: [[bool; GRID_SIZE]; GRID_SIZE],
    pub crosswords: [[Option<HashMap<char, usize>>; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            squares: [[Square::Blank; GRID_SIZE]; GRID_SIZE],
            anchors: [[false; GRID_SIZE]; GRID_SIZE],
            crosswords: std::array::from_fn(|_| std::array::from_fn(|_| None)),
        }
    }

    pub fn transpose_grid(&self, gaddag: &GaddagNode) -> Grid {
        // Renvoie une copie transposée de la grille
        let mut transposed_grid = Self::new();
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                transposed_grid.squares[j][i] = self.squares[i][j];
            }
        }
        transposed_grid.update_anchors();
        transposed_grid.update_crosswords(gaddag);
        transposed_grid
    }

    fn set_bonus(&mut self, bonus: Square, idx_list: &[(usize, usize)]) {
        let n = GRID_SIZE - 1;
        for (x, y) in idx_list {
            self.squares[*x][*y] = bonus;
            self.squares[*x][n - *y] = bonus;
            self.squares[n - *x][*y] = bonus;
            self.squares[n - *x][n - *y] = bonus;
        }
    }

    pub fn generate_grid(&mut self) {
        // Place les cases bonus sur la grille
        let bonuses = [
            ("LCD", Square::LCD),
            ("LCT", Square::LCT),
            ("LCQ", Square::LCQ),
            ("MCD", Square::MCD),
            ("MCT", Square::MCT),
            ("MCQ", Square::MCQ),
        ];
        for (str_bonus, bonus) in bonuses {
            if let Some(idx_list) = BONUS_CELLS.get(str_bonus) {
                self.set_bonus(bonus, idx_list);
            }
        }
    }

    pub fn get_square_multiplier(&self, x: usize, y: usize) -> (usize, usize) {
        // Retourne le bonus sur la case (i, j)
        match self.squares[x][y] {
            Square::Blank => (1, 1),
            Square::LCD => (2, 1),
            Square::LCT => (3, 1),
            Square::LCQ => (4, 1),
            Square::MCD => (1, 2),
            Square::MCT => (1, 3),
            Square::MCQ => (1, 4),
            _ => (1, 1),
        }
    }

    pub fn play(&mut self, word: &str, i: usize, j: usize, direction: usize, gaddag: &GaddagNode) {
        if direction == 0 {
            // Mot horizontal
            for (k, c) in word.chars().enumerate() {
                self.squares[i][j + k] = Square::Letter(c);
            }
        } else {
            // Mot vertical
            for (k, c) in word.chars().enumerate() {
                self.squares[i + k][j] = Square::Letter(c);
            }
        }
        self.update_anchors();
        self.update_crosswords(&gaddag);
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

    pub fn update_anchors(&mut self) {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
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
            self.anchors[7][7] = true; // Activation de l'ancre centrale si la grille est vide
        }
    }

    fn adj(&self, x: usize, y: usize) -> (String, String, usize) {
        // Récupère les mots au dessus et en dessous de la case actuelle
        let mut up_letters = String::new();
        let mut down_letters = String::new();
        let mut score = 0;
        // Parcours vers le haut
        let mut i = x;
        while i > 0 {
            i -= 1;
            if let Square::Letter(c) = self.squares[i][y] {
                score += *LETTERS_VALUE.get(&c).unwrap_or(&0);
                up_letters.push(c);
            } else {
                break;
            }
        }
        // Parcours vers le bas
        i = x;
        while i < 14 {
            i += 1;
            if let Square::Letter(c) = self.squares[i][y] {
                score += *LETTERS_VALUE.get(&c).unwrap_or(&0);
                down_letters.push(c);
            } else {
                break;
            }
        }
        (up_letters, down_letters, score)
    }

    pub fn update_crosswords(&mut self, gaddag: &GaddagNode) {
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                self.crosswords[x][y] = None;
                if self.anchors[x][y] {
                    let (up_letters, down_letters, score) = self.adj(x, y);
                    // On regarde s'il y a des lettres en haut ou en bas de la case
                    if up_letters != "" || down_letters != "" {
                        let (flat, mult) = self.get_square_multiplier(x, y);
                        for &c in &ALPHABET {
                            let word = format!("{}{}!{}", c, up_letters, down_letters);
                            if Gaddag::contains_word(&word, gaddag) {
                                // Calcul du score du crossword
                                let letter_score = *LETTERS_VALUE.get(&c).unwrap_or(&0);
                                let cw_score = (letter_score * flat + score) * mult;
                                // Insertion de la lettre et du score dans la table
                                let entry = self.crosswords[x][y].get_or_insert_with(HashMap::new);
                                entry.insert(c, cw_score);
                            }
                        }
                        if let Some(entry) = &mut self.crosswords[x][y] {
                            // Insertion du joker s'il existe au moins un crossword possible
                            let jok_cw_score = score * mult;
                            entry.insert('?', jok_cw_score);
                        } else {
                            // Sinon on initialise avec une HashMap vide
                            self.crosswords[x][y] = Some(HashMap::new());
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
