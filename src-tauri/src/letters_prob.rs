

pub struct LetterProbs {
    pub list_accumulated : Vec<(char, f32)>
}


impl LetterProbs {
    pub fn new() -> LetterProbs {
        let letter_probability: Vec<(char, f32)> = vec![
            ('E', 13.25),
            ('A', 7.40),
            ('I', 6.85),
            ('S', 6.60),
            ('N', 6.55),
            ('R', 6.10),
            ('T', 5.80),
            ('O', 5.55),
            ('L', 5.30),
            ('U', 4.55),
            ('D', 3.35),
            ('C', 3.20),
            ('M', 3.00),
            ('P', 2.45),
            ('G', 1.35),
            ('B', 1.20),
            ('F', 1.15),
            ('H', 1.10),
            ('V', 1.05),
            ('Q', 0.95),
            ('J', 0.30),
            ('K', 0.20),
            ('W', 0.20),
            ('X', 0.45),
            ('Y', 0.45),
            ('Z', 0.45),
        ];

        let mut cumulative = 0.00_f32;
        let list_accumulated = letter_probability.iter().map(
            |(ch, prob)| {
                cumulative += prob;
                (*ch,cumulative)
            }).collect();

        Self {
                list_accumulated
            }
    }
}