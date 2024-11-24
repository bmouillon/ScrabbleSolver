use phf::phf_set;
use rand::Rng;
use std::cmp::min;
use std::collections::HashMap;

use crate::constants::LETTERS_OCCURRENCE;

pub static VOWELS: phf::Set<char> = phf_set! {
    'A', 'E', 'I', 'O', 'U', 'Y', '?'
};

pub static CONSONANTS: phf::Set<char> = phf_set! {
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N',
    'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Z', 'Y', '?'
};

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

pub struct Bag {
    pub bag: Vec<char>,
    vowels_nb: usize,
    consonants_nb: usize,
}

impl Bag {
    pub fn new() -> Self {
        // Crée un nouveau sac de lettres
        let mut bag = Vec::new();
        let mut vowels_nb = 0;
        let mut consonants_nb = 0;
        for (&letter, &count) in LETTERS_OCCURRENCE.entries() {
            bag.extend(std::iter::repeat(letter).take(count));
            if VOWELS.contains(&letter) {
                vowels_nb += count;
            };
            if CONSONANTS.contains(&letter) {
                consonants_nb += count;
            }
        }
        Bag {
            bag,
            vowels_nb,
            consonants_nb,
        }
    }

    fn draw(&mut self, nb_letters: usize) -> (Vec<char>, usize, usize) {
        let mut rng = rand::thread_rng();
        let mut drawn_letters = Vec::new();
        let mut nb_v = 0;
        let mut nb_c = 0;
        for _ in 0..nb_letters {
            let index = rng.gen_range(0..self.bag.len());
            let letter = self.bag.remove(index);
            if VOWELS.contains(&letter) {
                nb_v += 1;
                self.vowels_nb -= 1;
            };
            if CONSONANTS.contains(&letter) {
                nb_c += 1;
                self.consonants_nb -= 1;
            }
            drawn_letters.push(letter);
        }
        (drawn_letters, nb_v, nb_c)
    }

    fn discard(&mut self, rack: &mut HashMap<char, usize>, drawn_letters: Vec<char>) {
        for letter in drawn_letters {
            self.bag.push(letter);
            if VOWELS.contains(&letter) {
                self.vowels_nb += 1;
            };
            if CONSONANTS.contains(&letter) {
                self.consonants_nb += 1;
            }
        }
        for (letter, count) in rack.drain() {
            self.bag.push(letter);
            if VOWELS.contains(&letter) {
                self.vowels_nb += count;
            };
            if CONSONANTS.contains(&letter) {
                self.consonants_nb += count;
            }
        }
    }

    pub fn valid_draw(
        &mut self,
        rack: &mut HashMap<char, usize>,
        lim: usize,
        min_vc: usize,
    ) -> bool {
        // Pioche pour remplir rack avec lim lettres, et au moins min_vc voyelles et consonnes
        // On compte le nombre de lettres, de voyelles et de consonnes du rack
        let mut rack_len = 0;
        let mut rack_vowels = 0;
        let mut rack_consonants = 0;
        for (letter, count) in rack.iter() {
            if VOWELS.contains(letter) {
                rack_vowels += count;
            }
            if CONSONANTS.contains(letter) {
                rack_consonants += count;
            }
            rack_len += count;
        }
        // Si il n'y a plus de voyelles ou plus de consonnes on s'arrête
        if self.vowels_nb + rack_vowels == 0 || self.consonants_nb + rack_consonants == 0 {
            return false;
        }
        // Si il ne reste plus assez de lettres dans le sac, on les prend toutes
        if self.bag.len() <= lim - rack_len {
            let (drawn_letters, _, _) = self.draw(lim - rack_len);
            augment_rack(rack, drawn_letters);
            return true;
        }
        // Sinon on essaie d'effectuer un tirage valide
        let min_vowels = min(min_vc, rack_vowels + self.vowels_nb);
        let min_consonants = min(min_vc, rack_consonants + self.consonants_nb);
        loop {
            let (drawn_letters, nb_v, nb_c) = self.draw(lim - rack_len);
            if nb_v + rack_vowels >= min_vowels && nb_c + rack_consonants >= min_consonants {
                augment_rack(rack, drawn_letters);
                return true;
            } else {
                self.discard(rack, drawn_letters);
                rack_len = 0;
                rack_vowels = 0;
                rack_consonants = 0;
            }
        }
    }
}
