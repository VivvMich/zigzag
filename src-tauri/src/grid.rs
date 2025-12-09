use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use csv::Reader;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

#[derive(Clone)]
pub struct Grid {
    pub height: u8,
    pub width: u8,
    pub letter_cases: Vec<char>,
    pub words: Vec<Option<String>>,
    pub lexicon: Vec<String>,
    pub letter_probability: HashMap<char, f32>
}


impl Grid {
    pub fn new() -> Grid {
        let lexicon = get_lexicon() ;
        let mut prob = HashMap::new() ;
        prob.insert('x', 0.01) ;

        Self {
            height: 29,
            width: 31,
            letter_cases: vec![],
            words: vec![],
            lexicon: lexicon.unwrap(),
            letter_probability: prob
        }
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

    // pub fn load_words(&mut self, words_map : HashMap<u8, Vec<String>> ) -> Vec<String> {
    //     let max_letters = self.height * self.width;
    //     let budget_letter = 0;
    //
    //
    //
    // }



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