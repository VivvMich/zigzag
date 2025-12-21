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
    pub letter_probability: HashMap<char, f32>,
    pub words_map: HashMap<(u32, char), Vec<String>>,
    pub max_letters: u32,
    pub max_budget: u32,
    pub quota_words: Vec<u32>
}


impl Grid {
    pub fn new() -> Grid {
        let lexicon = get_lexicon();
        let mut prob = HashMap::new();
        prob.insert('x', 0.01);
        let seed = rng().next_u64();
        let rng = Xoroshiro128PlusPlus::seed_from_u64(seed);
        Self {
            rng,
            height: 29,
            width: 31,
            letter_cases: vec![],
            words: vec![],
            lexicon: lexicon.unwrap(),
            letter_probability: prob,
            words_map: HashMap::new(),
            max_letters: 0,
            max_budget: 0,
            quota_words: vec![],

        }
    }

    pub fn init(&mut self) {
        self.words_map = self.separate_words_by_size_and_letter();
        self.max_letters = (self.height * self.width);
        print!("Reste {}", self.max_letters);
        self.max_budget = self.max_letters - (MAX_NUMBERS_LETTERS + MIN_LETTERS);
        self.load_words();
    }

    pub fn separate_words_by_size_and_letter(&mut self) -> HashMap<(u32, char), Vec<String>> {
        let mut separate_words: HashMap<(u32, char), Vec<String>> = HashMap::new();
        for word in &self.lexicon {
            let len = word.len();
            let c = remove_accent_from_str(word.chars().next().unwrap());
            separate_words.entry((len as u32, c)).or_default().push(word.clone());
        }

        separate_words
    }

    pub fn load_words(&mut self) {
        print!("Limite de budget : {} \n", &self.max_budget);
        let letter_probability = LetterProbs::new();
        let accumulated = letter_probability.list_accumulated;
        self.choice_words_with_random_size(&accumulated);
        get_all_characters_number_of_word(&self.words);
    }

    fn get_random_size_word(&mut self) -> u32 {
        let rand_nbr = self.rng.random_range(0..100);
        match rand_nbr {
            0..25 => self.rng.random_range(4..=5),
            25..80 => self.rng.random_range(6..=8),
            80..99 => self.rng.random_range(9..=12),
            _ => 0
        }
    }


    fn set_words_quota(&mut self) {
        let mut budget_limit: u32 = 0;
        loop {
            let length = self.get_random_size_word();
            self.quota_words.push(length);
            budget_limit += length as u32;
            print!("Budget >= Limite du budget -------> {} >= {} \n", budget_limit, self.max_budget);
            if budget_limit >= self.max_budget {
                let rest = (self.max_letters - budget_limit);
                let split_rest = split_number_two(rest);
                print!("Reste scinder : {}, {} \n", split_rest.0, split_rest.1);
                self.quota_words.push(split_rest.0);
                self.quota_words.push(split_rest.1);
                return;
            }
        }
    }

    fn choice_words_with_random_size(&mut self, probability_accumulated: &Vec<(char, f32)>) {}


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

fn get_all_characters_number_of_word(words: &Vec<String>) {
    let total: usize = words.iter()
        .map(|s| s.len())
        .sum();

    println!("Nombre total de caractères (sans espaces) : {}", total);
}

fn split_number_two(nbr: u32) -> (u32, u32) {
    let original = nbr as f32;
    let div: f32 = (nbr as f32 / 2.0).floor();
    let other = (original - div) as u32;
    (other, div as u32)
}