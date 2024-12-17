use rand::Rng;
use std::collections::HashMap;

use crate::constants::LETTERS_OCCURRENCE;

fn augment_rack(rack: &mut HashMap<char, usize>, letters: Vec<char>) {
    // Ajoute letters au rack
    for letter in letters {
        if let Some(count) = rack.get_mut(&letter) {
            *count += 1;
        } else {
            rack.insert(letter, 1);
        }
    }
}

pub fn rack_to_string(rack: &HashMap<char, usize>) -> String {
    let mut rack_string = String::new();
    for (letter, &count) in rack.iter() {
        rack_string.push_str(&letter.to_string().repeat(count));
    }
    rack_string
}

pub struct Bag {
    pub bag: Vec<char>,
}

impl Bag {
    pub fn new() -> Self {
        // Crée un nouveau sac de lettres
        let mut bag = Vec::new();
        for (&letter, &count) in LETTERS_OCCURRENCE.entries() {
            bag.extend(std::iter::repeat(letter).take(count));
        }
        Bag { bag }
    }

    fn draw(&mut self, nb_letters: usize) -> Vec<char> {
        let mut rng = rand::thread_rng();
        let mut drawn_letters = Vec::new();
        for _ in 0..nb_letters {
            let index = rng.gen_range(0..self.bag.len());
            let letter = self.bag.remove(index);
            drawn_letters.push(letter);
        }
        drawn_letters
    }

    fn discard(&mut self, rack: &mut HashMap<char, usize>) {
        for (letter, count) in rack.drain() {
            self.bag.extend(std::iter::repeat(letter).take(count));
        }
    }

    fn valid_draw(&mut self, rack: &mut HashMap<char, usize>, lim: usize) {
        let mut rack_len = 0;
        for (_, count) in rack.iter() {
            rack_len += count;
        }

        let drawn_letters = if self.bag.len() <= lim - rack_len {
            self.draw(self.bag.len())
        } else {
            self.draw(lim - rack_len)
        };

        augment_rack(rack, drawn_letters);
    }

    pub fn handle_draw(bag: &mut Bag, rack: &mut HashMap<char, usize>, lim: usize) {
        loop {
            bag.valid_draw(rack, lim);
            println!("Tirage actuel : {}", rack_to_string(rack));
            println!("Voulez-vous garder ce tirage ? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "y" {
                break;
            } else {
                bag.discard(rack);
            }
        }
    }
}
