use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

pub type GaddagNode = Rc<RefCell<Gaddag>>;

#[derive(Debug)]
pub struct Gaddag {
    pub is_word: bool,
    pub children: HashMap<char, GaddagNode>,
}

impl Gaddag {
    fn new() -> GaddagNode {
        Rc::new(RefCell::new(Gaddag {
            is_word: false,
            children: HashMap::new(),
        }))
    }

    fn insert_into_gaddag(word: &[char], gaddag: GaddagNode) {
        // Insère la séquence word dans le gaddag
        let mut gaddag = gaddag.borrow_mut();
        if word.is_empty() {
            gaddag.is_word = true;
        } else {
            let first_char = word[0];
            let rest_word = &word[1..];
            let child = gaddag
                .children
                .entry(first_char)
                .or_insert_with(|| Gaddag::new());
            Gaddag::insert_into_gaddag(rest_word, Rc::clone(child));
        }
    }

    fn generate_permutations(word: &str, gaddag: GaddagNode) {
        // Génère toutes les permutations de word à insérer dans le gaddag
        for i in 0..word.len() {
            let mut w: Vec<char> = Vec::new();
            for j in (0..=i).rev() {
                w.push(word.chars().nth(j).unwrap());
            }
            w.push('!');
            for j in (i + 1)..word.len() {
                w.push(word.chars().nth(j).unwrap());
            }
            Gaddag::insert_into_gaddag(&w, Rc::clone(&gaddag));
        }
    }

    pub fn follow_path(node: &GaddagNode, path: &str) -> Option<GaddagNode> {
        // Retourne le noeud en partant de node et en suivant path
        let mut current_node = Rc::clone(node);
        for c in path.chars() {
            let next_node = current_node.borrow().children.get(&c).cloned();
            match next_node {
                Some(next_node) => current_node = next_node,
                None => return None,
            }
        }
        Some(current_node)
    }

    pub fn contains_word(word: &str, gaddag: &GaddagNode) -> bool {
        // Vérifie si word est un mot valide du gaddag
        if let Some(final_node) = Gaddag::follow_path(gaddag, word) {
            final_node.borrow().is_word
        } else {
            false
        }
    }

    pub fn read_words_from_file(filename: &str) -> Rc<RefCell<Gaddag>> {
        // Crée un nouveau gaddag qui contient tous les mots présents dans filename
        let path = Path::new(filename);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                println!("Error opening file: {}", filename);
                return Gaddag::new();
            }
        };
        let reader = io::BufReader::new(file);
        let gaddag = Gaddag::new();
        // Chaque ligne correspond à un mot
        for line in reader.lines() {
            match line {
                Ok(word) => {
                    Gaddag::generate_permutations(&word, Rc::clone(&gaddag));
                }
                Err(_) => {
                    println!("Error reading a line from file: {}", filename);
                    return Gaddag::new();
                }
            }
        }
        gaddag
    }
}
