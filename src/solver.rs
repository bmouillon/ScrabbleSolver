use std::collections::HashMap;
use std::rc::Rc;

use crate::constants::LETTERS_VALUE;
use crate::gaddag::{Gaddag, GaddagNode};
use crate::grid::{Grid, Square};

pub struct WordInfo {
    pub position: (usize, usize),
    pub rack: HashMap<char, usize>,
    pub prefix: String,
    pub score: (usize, usize, usize),
    pub node: GaddagNode,
}

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
    wordinfo: &WordInfo,
    letter: char,
    replacement: char,
) -> Option<WordInfo> {
    if let Some(next_node) = wordinfo.node.borrow().children.get(&replacement) {
        // Vérifie si on ne forme pas un crossword invalide
        if grid.crosswords[i][j]
            .as_ref()
            .map_or(true, |cw| cw.contains_key(&replacement))
        {
            // Génère la nouvelle rack et le nouveau prefix
            let new_rack = reduce_rack(&wordinfo.rack, letter);
            let mut new_prefix = wordinfo.prefix.clone();
            new_prefix.push(if letter == '?' {
                replacement.to_ascii_lowercase()
            } else {
                replacement
            });
            // Mise à jour des scores
            let (square_flat, square_mult) = grid.get_square_multiplier(i, j);
            let new_flat_score =
                wordinfo.score.0 + LETTERS_VALUE.get(&replacement).unwrap_or(&0) * square_flat;
            let new_multiplier = wordinfo.score.1 * square_mult;
            // Calcul du nouveau score des crosswords
            let new_cw_score = if let Some(cw) = &grid.crosswords[i][j] {
                if let Some(cw_value) = cw.get(&replacement) {
                    wordinfo.score.2 + cw_value
                } else {
                    wordinfo.score.2
                }
            } else {
                wordinfo.score.2
            };
            return Some(WordInfo {
                position: wordinfo.position,
                rack: new_rack,
                prefix: new_prefix,
                score: (new_flat_score, new_multiplier, new_cw_score),
                node: Rc::clone(&next_node),
            });
        }
    }
    None
}

fn step(i: usize, j: usize, grid: &Grid, wordinfo: &WordInfo) -> Vec<WordInfo> {
    // Prend un WordInfo et effectue un pas
    let mut results = Vec::new();
    for (&letter, _) in wordinfo.rack.iter() {
        if letter == '?' {
            for replacement in 'A'..='Z' {
                if let Some(result) = process_letter(i, j, grid, wordinfo, letter, replacement) {
                    results.push(result);
                }
            }
        } else if let Some(result) = process_letter(i, j, grid, wordinfo, letter, letter) {
            results.push(result);
        }
    }
    results
}

pub fn handle_left_part(
    i: usize,
    j: usize,
    grid: &Grid,
    rack: &HashMap<char, usize>,
    gaddag: &GaddagNode,
) -> Vec<WordInfo> {
    // Récupère le préfixe à gauche de l'ancre
    let mut left_prefix = String::new();
    let mut left_score = 0;
    let mut k = j;
    while k > 0 {
        if let Square::Letter(letter) = grid.squares[i][k - 1] {
            left_prefix.push(letter);
            left_score += *LETTERS_VALUE.get(&letter).unwrap_or(&0);
            k -= 1;
        } else {
            break;
        }
    }
    let (square_flat, square_mult) = grid.get_square_multiplier(i, j);
    // Construit les préfixes valides
    let mut results = Vec::new();
    for (&c, _) in rack.iter() {
        let mut cw_score = 0;
        if let Some(crossword) = &grid.crosswords[i][j] {
            if let Some(&cw_value) = crossword.get(&c) {
                cw_score = cw_value;
            } else {
                continue;
            }
        }
        let path = format!("{}{}", c, left_prefix);
        if let Some(node) = Gaddag::follow_path(gaddag, &path) {
            let flat_score = left_score + *LETTERS_VALUE.get(&c).unwrap_or(&0) * square_flat;
            results.push(WordInfo {
                position: (i, j),
                rack: reduce_rack(rack, c),
                prefix: path,
                score: (flat_score, square_mult, cw_score),
                node,
            });
        }
    }
    results
}

pub fn generate_left_parts(
    i: usize,
    j: usize,
    grid: &Grid,
    rack: &HashMap<char, usize>,
    gaddag: &GaddagNode,
) -> Vec<WordInfo> {
    // Retourne l'ensemble des préfixes gauches à partir de (i, j)
    // Vérification de la présence d'une lettre à gauche de l'ancre
    if let Square::Letter(_) = grid.squares[i][j - 1] {
        return handle_left_part(i, j, grid, rack, gaddag);
    }
    // Recherche de la place disponible à gauche de l'ancre
    let mut left_limit = j;
    while left_limit > 0 && grid.anchors[i][left_limit - 1] == false {
        left_limit -= 1;
    }
    // Génération des préfixes
    let mut all_results = Vec::new();
    let mut current_prefixes = vec![WordInfo {
        position: (i, j),
        rack: rack.clone(),
        prefix: String::new(),
        score: (0, 1, 0),
        node: Rc::clone(gaddag),
    }];
    let mut next_prefixes = Vec::new();
    for k in (left_limit..=j).rev() {
        for wordinfo in current_prefixes.drain(..) {
            let results = step(i, k, grid, &wordinfo);
            for result in results {
                next_prefixes.push(result);
            }
            all_results.push(wordinfo);
        }
        current_prefixes = std::mem::take(&mut next_prefixes);
    }
    all_results.extend(current_prefixes);
    all_results
}

pub fn filter_left_parts(results: Vec<WordInfo>) -> Vec<WordInfo> {
    // Filtre les préfixes gauches obtenus en ne gardant que ceux qui peuvent être le début d'un mot
    let mut filtered_results = Vec::new();
    for wordinfo in results {
        if let Some(bang_node) = wordinfo.node.borrow().children.get(&'!') {
            // Renversement du préfixe
            let reversed_prefix = wordinfo.prefix.chars().rev().collect();
            // Ajout du résultat modifié à la liste filtrée
            filtered_results.push(WordInfo {
                position: wordinfo.position,
                rack: wordinfo.rack,
                prefix: reversed_prefix,
                score: wordinfo.score,
                node: Rc::clone(bang_node),
            });
        }
    }
    filtered_results
}
