use std::collections::HashMap;
use std::io;

mod gaddag;
use gaddag::Gaddag;

mod grid;
use grid::Grid;
use grid::Square;

mod solver;
use crate::solver::generate_solutions;

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

    let rack: HashMap<char, usize> = [
        ('A', 1),
        ('F', 1),
        ('I', 1),
        ('M', 1),
        ('N', 1),
        ('U', 1),
        ('?', 1),
    ]
    .iter()
    .cloned()
    .collect();
    let mut valid_words = generate_solutions(&grid, &rack, &gaddag);
    valid_words.sort_by(|a, b| b.score.cmp(&a.score));
    println!("Number of solutions: {}", valid_words.len());
    for validword in valid_words.iter().take(10) {
        println!(
            "Position: {:?}, Word: {}, Rack: {:?}, Score: {}",
            validword.position, validword.word, validword.rack, validword.score
        );
    }

    Ok(())
}
