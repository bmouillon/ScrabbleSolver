use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

type GaddagNode = Rc<RefCell<Tree>>;

#[derive(Debug)]
struct Tree {
    is_word: bool,
    children: HashMap<char, GaddagNode>,
}

impl Tree {
    fn new(is_word: bool) -> GaddagNode {
        Rc::new(RefCell::new(Tree {
            is_word,
            children: HashMap::new(),
        }))
    }

    fn insert_into_tree(word: &[char], tree: GaddagNode) {
        let mut tree = tree.borrow_mut();
        if word.is_empty() {
            tree.is_word = true;
        } else {
            let first_char = word[0];
            let rest_word = &word[1..];
            let child = tree
                .children
                .entry(first_char)
                .or_insert_with(|| Tree::new(false));
            Tree::insert_into_tree(rest_word, Rc::clone(child));
        }
    }

    fn generate_permutations(word: &str, tree: GaddagNode) {
        for i in 0..word.len() {
            let mut w: Vec<char> = Vec::new();
            for j in (0..=i).rev() {
                w.push(word.chars().nth(j).unwrap());
            }
            w.push('!');
            for j in (i + 1)..word.len() {
                w.push(word.chars().nth(j).unwrap());
            }
            Tree::insert_into_tree(&w, Rc::clone(&tree));
        }
    }

    fn contains_word(word: &str, tree: GaddagNode) -> bool {
        let mut current_node = Rc::clone(&tree);

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
}

fn read_words_from_file(filename: &str, tree: GaddagNode) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let word = line?;
        Tree::generate_permutations(&word, Rc::clone(&tree));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = Tree::new(false);
    read_words_from_file("ODS9.txt", Rc::clone(&root))?;

    // Vérifier si la string "UOF!DRE" est dans le GADDAG
    let word_to_check = "!FOUDRE";
    if Tree::contains_word(word_to_check, Rc::clone(&root)) {
        println!("The word '{}' is in the GADDAG!", word_to_check);
    } else {
        println!("The word '{}' is NOT in the GADDAG.", word_to_check);
    }

    Ok(())
}
