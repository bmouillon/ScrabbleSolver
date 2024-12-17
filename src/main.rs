use std::io;

mod constants;
use constants::FILENAME;

mod gaddag;
use gaddag::Gaddag;

mod grid;

mod bag;

mod solver;

mod generate;
use crate::generate::generate_game;

fn main() -> io::Result<()> {
    let gaddag = Gaddag::read_words_from_file(FILENAME);
    generate_game(&gaddag);
    Ok(())
}
