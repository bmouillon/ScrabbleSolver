use phf::phf_map;

pub const GRID_SIZE: usize = 15; // Size of the grid

pub const ALPHABET: [char; 26] = [
    // Alphabet
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const LETTERS_VALUE: phf::Map<char, usize> = phf::phf_map! {
    // Valeurs des lettres
    'A' => 1, 'B' => 3, 'C' => 3, 'D' => 2, 'E' => 1, 'F' => 4,
    'G' => 2, 'H' => 4, 'I' => 1, 'J' => 8, 'K' => 10, 'L' => 1,
    'M' => 2, 'N' => 1, 'O' => 1, 'P' => 3, 'Q' => 8, 'R' => 1,
    'S' => 1, 'T' => 1, 'U' => 1, 'V' => 4, 'W' => 10, 'X' => 10,
    'Y' => 10, 'Z' => 10,
};

pub static BONUS_CELLS: phf::Map<&'static str, &'static [(usize, usize)]> = phf_map! {
    // Emplacement des cases bonus
    "LCD" => &[(0, 3), (2, 6), (3, 0), (3, 7), (6, 2), (6, 6), (7, 3)],
    "LCT" => &[(1, 5), (5, 1), (5, 5)],
    "LCQ" => &[],
    "MCD" => &[(1, 1), (2, 2), (3, 3), (4, 4)],
    "MCT" => &[(0, 0), (0, 7), (7, 0)],
    "MCQ" => &[],
};

pub static LETTERS_OCCURRENCE: phf::Map<char, usize> = phf_map! {
    // Occurrence de chaque lettre
    'A' => 9, 'B' => 2, 'C' => 2, 'D' => 3, 'E' => 15, 'F' => 2, 'G' => 2,
    'H' => 2, 'I' => 8, 'J' => 1, 'K' => 1, 'L' => 5, 'M' => 3, 'N' => 6,
    'O' => 6, 'P' => 2, 'Q' => 1, 'R' => 6, 'S' => 6, 'T' => 6, 'U' => 6,
    'V' => 2, 'W' => 1, 'X' => 1, 'Y' => 1, 'Z' => 1, '?' => 2,
};

pub static BINGOS_BONUS: phf::Map<u8, usize> = phf_map! {
    // Primes de scrabble
    1u8 => 0, 2u8 => 0, 3u8 => 0, 4u8 => 0, 5u8 => 0, 6u8 => 0,
    7u8 => 50, 8u8 => 75, 9u8 => 100, 10u8 => 125, 11u8 => 150,
    12u8 => 175, 13u8 => 200, 14u8 => 225, 15u8 => 250,
};
