use std::collections::HashMap;

use crate::bag::rack_to_string;
use crate::bag::Bag;
use crate::gaddag::GaddagNode;
use crate::grid::Grid;
use crate::solver::generate_solutions;
use crate::solver::ValidWord;

pub fn choose_best_solution(valid_words: Vec<ValidWord>) -> ValidWord {
    let max_score = valid_words.iter().map(|word| word.score).max().unwrap();
    let best_words: Vec<&ValidWord> = valid_words
        .iter()
        .filter(|&word| word.score == max_score)
        .collect();

    println!("Score maximal: {}", max_score);
    for (i, word) in best_words.iter().enumerate() {
        println!(
            "{}: {}, {}, Reliquat: {}",
            i + 1,
            word.word,
            word.position,
            rack_to_string(&word.rack),
        );
    }

    println!("Choisissez une solution:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choice: usize = input.trim().parse().unwrap_or(1);

    if choice > 0 && choice <= best_words.len() {
        let solution = best_words[choice - 1].clone();
        println!(
            "Solution retenue: {} en {}\n",
            solution.word, solution.position
        );
        return solution;
    } else {
        let solution = best_words[0].clone();
        println!(
            "Solution retenue: {} en {}\n",
            solution.word, solution.position
        );
        return solution;
    }
}

pub fn generate_game(gaddag: &GaddagNode) {
    // Initialisation
    let mut grid = Grid::new();
    Grid::generate_grid(&mut grid);
    let mut bag = Bag::new();
    let mut rack = HashMap::new();
    // Génération
    while bag.bag.len() > 0 || !rack.is_empty() {
        Bag::handle_draw(&mut bag, &mut rack, 7);
        let valid_words = generate_solutions(&grid, &rack, &gaddag);
        let top = choose_best_solution(valid_words);
        let ((i, j), direction) = Grid::ref_to_pos(&top.position);
        grid.play(&top.word, i, j, direction, &gaddag);
        rack = top.rack.clone();
    }
    println!("Partie terminée !");
}
