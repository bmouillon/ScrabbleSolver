use phf::phf_map;

pub const GRID_SIZE: usize = 15; // Size of the grid

pub const ALPHABET: [char; 26] = [
    // Alphabet
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const LETTERS_VALUE: phf::Map<char, usize> = phf::phf_map! {
    // Letters value
    'A' => 1, 'B' => 3, 'C' => 3, 'D' => 2, 'E' => 1, 'F' => 4,
    'G' => 2, 'H' => 4, 'I' => 1, 'J' => 8, 'K' => 10, 'L' => 1,
    'M' => 2, 'N' => 1, 'O' => 1, 'P' => 3, 'Q' => 8, 'R' => 1,
    'S' => 1, 'T' => 1, 'U' => 1, 'V' => 4, 'W' => 10, 'X' => 10,
    'Y' => 10, 'Z' => 10,
};

pub static BONUS_CELLS: phf::Map<&'static str, &'static [(usize, usize)]> = phf_map! {
    "LCD" => &[(0, 3), (2, 6), (3, 0), (3, 7), (6, 2), (6, 6), (7, 3)],
    "LCT" => &[(1, 5), (5, 1), (5, 5)],
    "LCQ" => &[],
    "MCD" => &[(1, 1), (2, 2), (3, 3), (4, 4)],
    "MCT" => &[(0, 0), (0, 7), (7, 0)],
    "MCQ" => &[],
};
