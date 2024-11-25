use std::collections::HashMap;
use std::io;

mod constants;

mod gaddag;
use gaddag::Gaddag;

mod grid;
use grid::Grid;
use grid::Square;

mod bag;

mod solver;
use crate::solver::generate_solutions;

mod generate;
use crate::generate::generate_game;

fn main() -> io::Result<()> {
    let gaddag = Gaddag::read_words_from_file("ODS9.txt");
    // let mut grid = Grid::new();
    // Grid::generate_grid(&mut grid);

    // grid.play("KALIS", 7, 3, true, &gaddag);
    // grid.play("HERON", 6, 6, true, &gaddag);
    // grid.play("WONS", 5, 9, false, &gaddag);
    // grid.play("EBOUTEES", 0, 7, false, &gaddag);
    // grid.play("SUDATION", 2, 1, true, &gaddag);
    // grid.play("AmALGAmER", 2, 4, false, &gaddag);
    // grid.play("BALBUTIE", 1, 7, true, &gaddag);
    // grid.play("VENT", 0, 11, true, &gaddag);
    // grid.play("FJELL", 5, 0, true, &gaddag);
    // grid.play("FUMEZ", 5, 0, false, &gaddag);

    // grid.squares[7][3] = Square::Letter('G');
    // grid.squares[7][4] = Square::Letter('U');
    // grid.squares[7][5] = Square::Letter('E');
    // grid.squares[7][6] = Square::Letter('E');
    // grid.squares[7][7] = Square::Letter('E');
    // grid.squares[7][8] = Square::Letter('S');
    // grid.squares[1][6] = Square::Letter('C');
    // grid.squares[2][6] = Square::Letter('I');
    // grid.squares[3][6] = Square::Letter('B');
    // grid.squares[4][6] = Square::Letter('L');
    // grid.squares[5][6] = Square::Letter('E');
    // grid.squares[6][6] = Square::Letter('R');
    // grid.squares[8][6] = Square::Letter('Z');
    // grid.squares[4][4] = Square::Letter('W');
    // grid.squares[4][5] = Square::Letter('A');
    // grid.squares[4][7] = Square::Letter('I');
    // grid.squares[5][0] = Square::Letter('T');
    // grid.squares[5][1] = Square::Letter('A');
    // grid.squares[5][2] = Square::Letter('R');
    // grid.squares[5][3] = Square::Letter('G');
    // grid.squares[5][4] = Square::Letter('U');
    // grid.squares[5][5] = Square::Letter('I');

    // grid.update_anchors();
    // grid.update_crosswords(&gaddag);
    // println!("{}", grid);

    // let rack: HashMap<char, usize> = [('E', 2), ('G', 1), ('T', 1), ('X', 1), ('Y', 1), ('A', 1)]
    //     .iter()
    //     .cloned()
    //     .collect();
    // let mut valid_words = generate_solutions(&grid, &rack, &gaddag);
    // valid_words.sort_by(|a, b| b.score.cmp(&a.score));
    // println!("Number of solutions: {}", valid_words.len());
    // for validword in valid_words.iter().take(10) {
    //     println!(
    //         "Position: {:?}, Word: {}, Rack: {:?}, Score: {}",
    //         validword.position, validword.word, validword.rack, validword.score
    //     );
    // }

    generate_game(&gaddag);

    Ok(())
}
