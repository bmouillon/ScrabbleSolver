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

    pub fn handle_draw(&mut self, rack: &mut HashMap<char, usize>, lim: usize) {
        loop {
            self.valid_draw(rack, lim);
            println!("Tirage actuel : {}", rack_to_string(rack));
            println!("Voulez-vous garder ce tirage ? (y/n)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "y" {
                break;
            } else {
                self.discard(rack);
            }
        }
    }

    pub fn handle_manual_draw(
        &mut self,
        rack: &mut HashMap<char, usize>,
        lim: usize,
        input: String,
    ) -> bool {
        let current_count: usize = rack.values().sum();
        let letters_needed = lim - current_count;
        let input_letters: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();

        if input == "-" {
            self.discard(rack);
            println!("Le tirage a été rejeté");
            return false;
        }

        if input_letters.len() != letters_needed {
            println!("Vous devez rentrer {} lettres", letters_needed);
            return false;
        }

        // Vérification préalable des lettres disponibles dans le sac
        let mut bag_copy = self.bag.clone();
        for &letter in &input_letters {
            if let Some(pos) = bag_copy.iter().position(|&c| c == letter) {
                bag_copy.remove(pos);
            } else {
                println!("Lettre épuisée : '{}'", letter);
                return false;
            }
        }

        // Toutes les lettres sont disponibles, on procède aux modifications
        for &letter in &input_letters {
            if let Some(pos) = self.bag.iter().position(|&c| c == letter) {
                self.bag.remove(pos);
                *rack.entry(letter).or_insert(0) += 1;
            }
        }

        true
    }
}
