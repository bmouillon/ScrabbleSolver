use std::collections::HashMap;
use std::io;

mod gaddag;
use gaddag::Gaddag;

mod grid;
use grid::Grid;
use grid::Square;

mod solver;
use crate::solver::filter_left_parts;
use crate::solver::generate_left_parts;
use solver::WordInfo;

mod constants;

fn main() -> io::Result<()> {
    let gaddag = Gaddag::read_words_from_file("ODS9.txt");
    let mut grid = Grid::new();
    Grid::generate_grid(&mut grid);

    grid.squares[7][3] = Square::Letter('G');
    grid.squares[7][4] = Square::Letter('U');
    grid.squares[7][5] = Square::Letter('E');
    grid.squares[7][6] = Square::Letter('E');
    grid.squares[7][7] = Square::Letter('E');
    grid.squares[7][8] = Square::Letter('S');
    grid.squares[1][6] = Square::Letter('C');
    grid.squares[2][6] = Square::Letter('I');
    grid.squares[3][6] = Square::Letter('B');
    grid.squares[4][6] = Square::Letter('L');
    grid.squares[5][6] = Square::Letter('E');
    grid.squares[6][6] = Square::Letter('R');
    grid.squares[8][6] = Square::Letter('Z');
    grid.squares[4][4] = Square::Letter('W');
    grid.squares[4][5] = Square::Letter('A');
    grid.squares[4][7] = Square::Letter('I');
    grid.squares[5][0] = Square::Letter('T');
    grid.squares[5][1] = Square::Letter('A');
    grid.squares[5][2] = Square::Letter('R');
    grid.squares[5][3] = Square::Letter('G');
    grid.squares[5][4] = Square::Letter('U');
    grid.squares[5][5] = Square::Letter('I');

    grid.update_anchors();
    grid.update_crosswords(&gaddag);
    println!("{}", grid);

    let rack: HashMap<char, usize> = [('A', 2), ('F', 1), ('I', 1), ('L', 2), ('X', 1)]
        .iter()
        .cloned()
        .collect();
    let results = generate_left_parts(2, 5, &grid, &rack, &gaddag);
    let filtered_results = filter_left_parts(results);
    for wordinfo in filtered_results {
        println!(
            "Prefix: {}, Rack: {:?}, Flat Score: {}, Multiplier: {}, CW Score: {}",
            wordinfo.prefix, wordinfo.rack, wordinfo.score.0, wordinfo.score.1, wordinfo.score.2
        );
    }

    // // Vérifier si la string "UOF!DRE" est dans le GADDAG
    // let word_to_check = "!FOUDRE";
    // if Tree::contains_word(word_to_check, Rc::clone(&root)) {
    //     println!("The word '{}' is in the GADDAG!", word_to_check);
    // } else {
    //     println!("The word '{}' is NOT in the GADDAG.", word_to_check);
    // }

    // // Initialize the grid
    // let mut grid = Grid::new();

    // // Generate the grid
    // grid.generate_grid();

    // // Update anchors based on the current grid state
    // grid.update_anchors();

    // // Print the grid and anchors
    // println!("{}", grid);

    Ok(())
}
