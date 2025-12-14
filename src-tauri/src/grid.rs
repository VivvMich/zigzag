use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};
use rand::{rng, Rng};
use rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;
use crate::letters_prob::LetterProbs;

const MAX_NUMBERS_LETTERS: u32 = 12;
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
        let limit_budget = max_letters - MAX_NUMBERS_LETTERS;
        let mut budget_letter : u32 = 0;
        let letter_probability = LetterProbs::new();
        let accumilated = letter_probability.list_accumulated;


        self.choice_words_with_random_size(&accumilated, &words_map, limit_budget) ;

        // Il reste un mot à mettre
        let last_len_word: u8 = (max_letters - budget_letter) as u8;
        self.choice_one_word(&accumilated, &words_map, last_len_word) ;
    }

    fn get_random_size_word(&mut self) -> u8 {
        let rand_nbr= self.rng.random_range(0..100);
        print!("Nbr aléa : {} \n",  rand_nbr);
        match rand_nbr {
            0..25 => self.rng.random_range(4..=5),
            25..80 => self.rng.random_range(6..=8),
            80..99 => self.rng.random_range(9..=12),
            _=> 0
        }
    }

    fn choice_words_with_random_size(&mut self, probability_accumulated:  &Vec<(char, f32)>, words_map : &HashMap<(u8, char), Vec<String>>, limit_budget : u32) {
        let mut budget_letter = 0;


        while budget_letter <= limit_budget {
            let length = self.get_random_size_word();
            for (ch, threshold) in probability_accumulated {
                let prob_letter: f32 = self.rng.random_range(0.00_f32..=100.00_f32);
                if prob_letter <= *threshold {
                    let wm = words_map.get(&(length, ch.to_lowercase().next().unwrap()));
                    if let Some(w) = wm {
                        let len_vec = w.len();
                        let rand_word = self.rng.random_range(0..len_vec);
                        let word = &w[rand_word].clone();
                        self.words.push(word.to_string());
                        budget_letter += length as u32;
                    }
                }
            }
        }
    }

    fn choice_one_word(&mut self, probability_accumulated:  &Vec<(char, f32)>, words_map : &HashMap<(u8, char), Vec<String>>, length : u8) {

        for (ch, threshold) in probability_accumulated {
            let prob_letter: f32 = self.rng.random_range(0.00_f32..=100.00_f32);
            if prob_letter <= *threshold {
                let wm = words_map.get(&(length, ch.to_lowercase().next().unwrap()));
                if let Some(w) = wm {
                    let len_vec = w.len();
                    let rand_word = self.rng.random_range(0..len_vec);
                    let word = &w[rand_word].clone();
                    self.words.push(word.to_string());
                }
            }
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

