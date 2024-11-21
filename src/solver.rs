use std::collections::HashMap;

use crate::constants::LETTERS_VALUE;
use crate::gaddag::{Gaddag, GaddagNode};
use crate::grid::{Grid, Square};

fn reduce_rack(rack: &HashMap<char, usize>, letter: char) -> HashMap<char, usize> {
    // Effectue une copie du rack avec une occurence de letter en moins
    let mut new_rack = rack.clone();
    if let Some(count) = new_rack.get_mut(&letter) {
        if *count > 1 {
            *count -= 1;
        } else {
            new_rack.remove(&letter);
        }
    }
    new_rack
}

fn process_letter(
    i: usize,
    j: usize,
    grid: &Grid,
    rack: &HashMap<char, usize>,
    prefix: &str,
    flat_score: usize,
    multiplier: usize,
    cw_score: usize,
    node: &GaddagNode,
    letter: char,
    replacement: char,
) -> Option<(
    HashMap<char, usize>,
    String,
    usize,
    usize,
    usize,
    GaddagNode,
)> {
    if let Some(next_node) = node.borrow().children.get(&replacement) {
        // Vérifie si on ne forme pas un crossword invalide
        if grid.crosswords[i][j]
            .as_ref()
            .map_or(true, |cw| cw.contains_key(&replacement))
        {
            // Génère la nouvelle rack et le nouveau prefix
            let new_rack = reduce_rack(rack, letter);
            let mut new_prefix = prefix.to_owned();
            new_prefix.push(if letter == '?' {
                replacement.to_ascii_lowercase()
            } else {
                replacement
            });
            // Mise à jour des scores
            let (square_flat, square_mult) = grid.get_square_multiplier(i, j);
            let new_flat_score =
                flat_score + LETTERS_VALUE.get(&replacement).unwrap_or(&0) * square_flat;
            let new_multiplier = multiplier * square_mult;
            // Calcul du nouveau score des crosswords
            let new_cw_score = if let Some(cw) = &grid.crosswords[i][j] {
                if let Some(cw_value) = cw.get(&replacement) {
                    cw_score + cw_value
                } else {
                    cw_score
                }
            } else {
                cw_score
            };
            return Some((
                new_rack,
                new_prefix,
                new_flat_score,
                new_multiplier,
                new_cw_score,
                next_node.clone(),
            ));
        }
    }
    None
}

fn step(
    i: usize,
    j: usize,
    grid: Grid,
    rack: &HashMap<char, usize>,
    prefix: &str,
    flat_score: usize,
    multiplier: usize,
    cw_score: usize,
    node: &GaddagNode,
) -> Vec<(
    HashMap<char, usize>,
    String,
    usize,
    usize,
    usize,
    GaddagNode,
)> {
    let mut results = Vec::new();
    for (&letter, _) in rack.iter() {
        if letter == '?' {
            for replacement in 'A'..='Z' {
                if let Some(result) = process_letter(
                    i,
                    j,
                    &grid,
                    rack,
                    prefix,
                    flat_score,
                    multiplier,
                    cw_score,
                    node,
                    letter,
                    replacement,
                ) {
                    results.push(result);
                }
            }
        } else if let Some(result) = process_letter(
            i, j, &grid, rack, prefix, flat_score, multiplier, cw_score, node, letter, letter,
        ) {
            results.push(result);
        }
    }
    results
}
