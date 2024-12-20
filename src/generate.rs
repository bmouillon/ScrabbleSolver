use std::collections::HashMap;
use std::time::Instant;

use crate::bag::Bag;
use crate::gaddag::GaddagNode;
use crate::solver::generate_solutions;
use crate::Grid;

pub fn generate_game(gaddag: &GaddagNode) {
    // Initialisation
    let start = Instant::now();
    let mut grid = Grid::new();
    Grid::generate_grid(&mut grid);
    let mut bag = Bag::new();
    let mut rack = HashMap::new();
    let mut moves_nb = 0;
    let mut min_vc = 2;
    // Génération
    while bag.valid_draw(&mut rack, 7, min_vc) {
        println!("Move: {}, Rack: {:?}", moves_nb, rack);
        let mut valid_words = generate_solutions(&grid, &rack, &gaddag);
        valid_words.sort_by(|a, b| b.score.cmp(&a.score));
        let top = &valid_words[0];
        println!(
            "Position: {}, Word: {}, Score: {}, Remaining Rack: {:?}",
            top.position, top.word, top.score, top.rack
        );
        let ((i, j), direction) = Grid::ref_to_pos(&top.position);
        grid.play(&top.word, i, j, direction, &gaddag);
        rack = top.rack.clone();
        moves_nb += 1;
        if moves_nb > 15 {
            min_vc = 1;
        }
    }
    let duration = start.elapsed();
    println!("Le temps d'exécution est de: {:?}", duration);
}
