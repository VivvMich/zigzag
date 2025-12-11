use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};
use rand::{rng, Rng};
use rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;
use crate::letters_prob::LetterProbs;

const MAX_LETTERS: u32 = 12;
const MIN_LETTERS: u32 = 5;



#[derive(Clone)]
pub struct Grid {
    rng: Xoroshiro128PlusPlus,
    pub height: u32,
    pub width: u32,
    pub letter_cases: Vec<char>,
    pub words: Vec<String>,
    pub lexicon: Vec<String>,
    pub letter_probability: HashMap<char, f32>
}


impl Grid {
    pub fn new() -> Grid {
        let lexicon = get_lexicon() ;
        let mut prob = HashMap::new() ;
        prob.insert('x', 0.01) ;
        let seed = rng().next_u64();
        let rng = Xoroshiro128PlusPlus::seed_from_u64(seed);

        Self {
            rng,
            height: 29,
            width: 31,
            letter_cases: vec![],
            words: vec![],
            lexicon: lexicon.unwrap(),
            letter_probability: prob
        }
    }

    pub fn create_grid(&mut self) {
        let separate_index = self.separate_words_by_size_and_letter();
        self.load_words(separate_index);
    }

    pub fn separate_words_by_size_and_letter(&mut self) -> HashMap<(u8, char) ,Vec<String>> {
        let mut separate_words : HashMap<(u8, char) ,Vec<String>> = HashMap::new();

        for word in &self.lexicon {
            let len = word.len() as u8;
            let c = remove_accent_from_str(word.chars().next().unwrap()) ;
            separate_words.entry((len, c)).or_default().push(word.clone());
        }
        separate_words
    }

    pub fn load_words(&mut self, words_map : HashMap<(u8, char), Vec<String>>){
        let max_letters: u32 = self.height * self.width;
        let budget_letter:u32 = 0;

        // PROBABILITE //

        let mut letter_probability = LetterProbs::new();

        // GENERATION //

        while budget_letter <= max_letters - MAX_LETTERS {
            let prob_letter:f32 = self.rng.random_range(0.00_f32..=100.00_f32);
            let length = self.get_random_size_word();

            for (ch, threshold) in &letter_probability.list_accumulated {
                if prob_letter <= *threshold {
                    let wm = words_map.get(&(length, *ch));
                    if let Some(w) = wm {
                        let len_vec = w.len();
                        let rand_word = self.rng.random_range(0..len_vec) ;
                        let word = &w[rand_word].clone();
                        self.words.push(word.to_string());
                    }

                }
            }
        }
    }

    fn get_random_size_word(&mut self) -> u8 {
        let rand_nbr = rng().next_u64() ;
        match rand_nbr {
            0..35 => self.rng.random_range(4..=5),
            35..80 => self.rng.random_range(6..=8),
            80..100 => self.rng.random_range(9..=12),
            _=> 0
        }
    }



}

fn get_lexicon() -> Result<Vec<String>, Box<dyn Error>> {
    let mut reader = Reader::from_path("./lexique.csv")?;
    let mut words : Vec<String> = vec![];

    for res in reader.records(){
        let record= res?;
        let word = record.get(0).unwrap().to_string();
        words.push(word);
    }
    Ok(words)
}

fn remove_accent_from_str(input: char) -> char {
    input
        .nfd()
        .find(|c| !is_combining_mark(*c))
        .unwrap_or('_')
}

