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

fn step(
    i: usize,
    j: usize,
    grid: Grid,
    rack: &HashMap<char, usize>,
    prefix: &str,
    flat_score: usize,
    multiplier: usize,
    node: &GaddagNode,
) -> Vec<(HashMap<char, usize>, String, usize, usize, GaddagNode)> {
    // Génère les préfixes d'une lettre plus long avec le préfixe actuel
    let mut results = Vec::new();
    for (&letter, _) in rack.iter() {
        if letter == '?' {
            // Génération des préfixes avec le joker
            for replacement in 'A'..='Z' {
                if let Some(next_node) = node.borrow().children.get(&replacement) {
                    if grid.crosswords[i][j]
                        .as_ref()
                        .map_or(true, |cw| cw.contains_key(&letter))
                    {
                        let new_rack = reduce_rack(rack, letter);
                        let mut new_prefix = prefix.to_owned();
                        new_prefix.push(replacement.to_ascii_lowercase());
                        // Mise à jour de flat_score et multiplier
                        let (_, square_mult) = grid.get_square_multiplier(i, j);
                        let new_multiplier = multiplier * square_mult;
                        // Ajoute le résultat à la liste
                        results.push((
                            new_rack,
                            new_prefix,
                            flat_score,
                            new_multiplier,
                            next_node.clone(),
                        ));
                    }
                }
            }
        } else if let Some(next_node) = node.borrow().children.get(&letter) {
            // Vérifie que letter ne forme pas un crossword invalide
            if grid.crosswords[i][j]
                .as_ref()
                .map_or(true, |cw| cw.contains_key(&letter))
            {
                // Copie et mise à jour de prefix et rack
                let new_rack = reduce_rack(rack, letter);
                let mut new_prefix = prefix.to_owned();
                new_prefix.push(letter);
                // Mise à jour de flat_score et multiplier
                let (square_flat, square_mult) = grid.get_square_multiplier(i, j);
                let (new_flat_score, new_multiplier) = (
                    flat_score + LETTERS_VALUE.get(&letter).unwrap_or(&0) * square_flat,
                    multiplier * square_mult,
                );
                // Ajoute le résultat à la liste
                results.push((
                    new_rack,
                    new_prefix,
                    new_flat_score,
                    new_multiplier,
                    next_node.clone(),
                ));
            }
        }
    }
    results
}
