use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

pub type GaddagNode = Rc<RefCell<Gaddag>>;

#[derive(Debug)]
pub struct Gaddag {
    is_word: bool,
    children: HashMap<char, GaddagNode>,
}

impl Gaddag {
    fn new() -> GaddagNode {
        Rc::new(RefCell::new(Gaddag {
            is_word: false,
            children: HashMap::new(),
        }))
    }

    fn insert_into_gaddag(word: &[char], gaddag: GaddagNode) {
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

    pub fn contains_word(word: &str, gaddag: GaddagNode) -> bool {
        let mut current_node = Rc::clone(&gaddag);

        for c in word.chars() {
            let next_node_opt = {
                let node = current_node.borrow();
                node.children.get(&c).map(Rc::clone)
            };

            match next_node_opt {
                Some(next_node) => {
                    current_node = next_node;
                }
                None => return false,
            }
        }

        let is_word = current_node.borrow().is_word;
        is_word
    }

    pub fn read_words_from_file(filename: &str) -> Rc<RefCell<Gaddag>> {
        let path = Path::new(filename);
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                println!("Error opening file: {}", filename);
                return Gaddag::new(); // Return an empty GADDAG if file can't be opened
            }
        };
        let reader = io::BufReader::new(file);
        let gaddag = Gaddag::new();

        for line in reader.lines() {
            match line {
                Ok(word) => {
                    Gaddag::generate_permutations(&word, Rc::clone(&gaddag));
                }
                Err(_) => {
                    println!("Error reading a line from file: {}", filename);
                    return Gaddag::new(); // Return an empty GADDAG in case of read error
                }
            }
        }
        gaddag
    }
}
