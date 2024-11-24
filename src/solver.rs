use std::cmp::min;
use std::collections::HashMap;
use std::rc::Rc;

use crate::constants::{BINGOS_BONUS, GRID_SIZE, LETTERS_VALUE};
use crate::gaddag::{Gaddag, GaddagNode};
use crate::grid::{Grid, Square};

pub struct WordInfo {
    pub position: (usize, usize),
    pub rack: HashMap<char, usize>,
    pub prefix: String,
    pub score: (usize, usize, usize),
    pub letters_nb: u8,
    pub node: GaddagNode,
}

pub struct ValidWord {
    pub position: String,
    pub rack: HashMap<char, usize>,
    pub word: String,
    pub score: usize,
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
    // Remplacement sert pour le joker
    if let Some(next_node) = wordinfo.node.borrow().children.get(&replacement) {
        // Vérifie si on ne forme pas un crossword invalide
        let mut new_cw_score = wordinfo.score.2;
        if let Some(cw) = &grid.crosswords[i][j] {
            if let Some(cw_value) = cw.get(&replacement) {
                if letter == '?' {
                    new_cw_score += cw.get(&letter).unwrap_or(&0);
                } else {
                    new_cw_score += cw_value;
                }
            } else {
                return None;
            }
        }
        // Génère la nouvelle rack et le nouveau prefix
        let new_rack = reduce_rack(&wordinfo.rack, letter);
        let mut new_prefix = wordinfo.prefix.clone();
        new_prefix.push(if letter == '?' {
            replacement.to_ascii_lowercase()
        } else {
            letter
        });
        // Mise à jour des scores
        let (square_flat, square_mult) = grid.get_square_multiplier(i, j);
        let new_flat_score =
            wordinfo.score.0 + LETTERS_VALUE.get(&letter).unwrap_or(&0) * square_flat;
        let new_multiplier = wordinfo.score.1 * square_mult;
        // Retourne le nouveau WordInfo
        return Some(WordInfo {
            position: (i, min(wordinfo.position.1, j)),
            rack: new_rack,
            prefix: new_prefix,
            score: (new_flat_score, new_multiplier, new_cw_score),
            letters_nb: wordinfo.letters_nb + 1,
            node: Rc::clone(next_node),
        });
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
    wordinfos: Vec<WordInfo>,
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
    // Construit les préfixes valides
    let mut results = Vec::new();
    for wordinfo in wordinfos {
        if let Some(node) = Gaddag::follow_path(&wordinfo.node, &left_prefix) {
            let new_prefix = format!("{}{}", wordinfo.prefix, left_prefix);
            let new_flat_score = wordinfo.score.0 + left_score;
            results.push(WordInfo {
                position: (i, k),
                prefix: new_prefix,
                score: (new_flat_score, wordinfo.score.1, wordinfo.score.2),
                node: node,
                ..wordinfo
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
    let empty_wordinfo = WordInfo {
        position: (i, j),
        rack: rack.clone(),
        prefix: String::new(),
        score: (0, 1, 0),
        letters_nb: 0,
        node: Rc::clone(gaddag),
    };
    // Vérification de la présence d'une lettre à gauche de l'ancre
    if j > 0 {
        if let Square::Letter(_) = grid.squares[i][j - 1] {
            let current_wordinfos = step(i, j, grid, &empty_wordinfo);
            return handle_left_part(i, j, grid, current_wordinfos);
        }
    }
    // Recherche de la place disponible à gauche de l'ancre
    let mut left_limit = j;
    while left_limit > 0 && grid.anchors[i][left_limit - 1] == false {
        left_limit -= 1;
    }
    // Génération des préfixes
    let mut all_results = Vec::new();
    let mut current_prefixes = vec![empty_wordinfo];
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
    // Sauvegarde de la dernière itération
    all_results.extend(current_prefixes);
    all_results
}

pub fn filter_left_parts(wordinfos: Vec<WordInfo>) -> Vec<WordInfo> {
    // Filtre les préfixes gauches obtenus en ne gardant que ceux qui peuvent être le début d'un mot
    let mut filtered_wordinfos = Vec::new();
    for wordinfo in wordinfos {
        if let Some(bang_node) = wordinfo.node.borrow().children.get(&'!') {
            // Renversement du préfixe
            let reversed_prefix = wordinfo.prefix.chars().rev().collect();
            // Ajout du résultat modifié à la liste filtrée
            filtered_wordinfos.push(WordInfo {
                prefix: reversed_prefix,
                node: Rc::clone(bang_node),
                ..wordinfo
            });
        }
    }
    filtered_wordinfos
}

pub fn generate_right_parts(
    i: usize,
    j: usize,
    grid: &Grid,
    wordinfos: Vec<WordInfo>,
) -> Vec<WordInfo> {
    let mut current_wordinfos = wordinfos;
    let mut next_wordinfos = Vec::new();
    let mut all_results = Vec::new();
    let mut y = j + 1;
    while y < GRID_SIZE {
        if let Square::Letter(letter) = grid.squares[i][y] {
            // Si la case suivante contient une lettre on essaie de l'ajouter à chaque WordInfo
            for wordinfo in current_wordinfos {
                if let Some(next_node) = wordinfo
                    .node
                    .borrow()
                    .children
                    .get(&letter.to_ascii_uppercase())
                {
                    let mut new_prefix = wordinfo.prefix.clone();
                    new_prefix.push(letter);
                    let new_flat_score =
                        wordinfo.score.0 + *LETTERS_VALUE.get(&letter).unwrap_or(&0);
                    next_wordinfos.push(WordInfo {
                        prefix: new_prefix,
                        score: (new_flat_score, wordinfo.score.1, wordinfo.score.2),
                        node: Rc::clone(next_node),
                        ..wordinfo
                    });
                }
            }
        } else {
            // Si la case suivante ne possède pas de lettre on applique step à chaque WordInfo
            for wordinfo in current_wordinfos.drain(..) {
                let results = step(i, y, grid, &wordinfo);
                for result in results {
                    next_wordinfos.push(result);
                }
                all_results.push(wordinfo);
            }
        }
        current_wordinfos = std::mem::take(&mut next_wordinfos);
        y += 1;
    }
    all_results.extend(current_wordinfos);
    all_results
}

pub fn filter_valid_words(wordinfos: Vec<WordInfo>, direction: bool) -> Vec<ValidWord> {
    // Ne retourne que les WordInfo qui sont des mots valides, et calcule leur score
    wordinfos
        .into_iter()
        .filter(|wi| wi.node.borrow().is_word)
        .map(|wi| {
            let bonus = *BINGOS_BONUS.get(&wi.letters_nb).unwrap_or(&0);
            let final_score = wi.score.0 * wi.score.1 + wi.score.2 + bonus;
            ValidWord {
                position: Grid::pos_to_ref(wi.position, direction),
                rack: wi.rack,
                word: wi.prefix,
                score: final_score,
            }
        })
        .collect()
}

pub fn generate_solutions(
    grid: &Grid,
    rack: &HashMap<char, usize>,
    gaddag: &GaddagNode,
) -> Vec<ValidWord> {
    // Renvoie toutes les solutions jouables sur la grille
    let mut valid_words = Vec::new();
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if grid.anchors[i][j] {
                let left_parts = generate_left_parts(i, j, grid, rack, gaddag);
                let valid_left_parts = filter_left_parts(left_parts);
                let right_parts = generate_right_parts(i, j, grid, valid_left_parts);
                let valid_right_parts = filter_valid_words(right_parts, true);
                valid_words.extend(valid_right_parts);
            }
        }
    }
    let transposed_grid = Grid::transpose_grid(grid, gaddag);
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if transposed_grid.anchors[i][j] {
                let left_parts = generate_left_parts(i, j, &transposed_grid, rack, gaddag);
                let valid_left_parts = filter_left_parts(left_parts);
                let right_parts = generate_right_parts(i, j, &transposed_grid, valid_left_parts);
                let valid_right_parts = filter_valid_words(right_parts, false);
                valid_words.extend(valid_right_parts);
            }
        }
    }
    valid_words
}
