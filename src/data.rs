use std::fs;
use std::hash::Hash;
use std::io::Write;
use serde::__private::de;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

use std::cmp::min;

use serde_with::serde_as;

const MAX_COMBO_SIZE: usize = 20;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Sentence {
    letters: Vec<char>,
    delays: Vec<u32>,
}

impl Sentence {
    pub fn new(letters: Vec<char>, delays: Vec<u32>) -> Self {

        assert_eq!(letters.len(), delays.len() + 1);

        Sentence {
            letters,
            delays,
        }
    }

    pub fn letters(&self) -> &Vec<char> {
        &self.letters
    }

    pub fn delays(&self) -> &Vec<u32> {
        &self.delays
    }


}

#[derive(Serialize, Deserialize)]
pub struct Data{
    sentences: Vec<Sentence>,
    analyzed_data: Option<AnalyzedData>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            sentences: Vec::new(),
            analyzed_data: Some(AnalyzedData::new()),
        }
    }

    pub fn load() -> std::io::Result<Self> {
        let path = "data.json";
        let file = fs::File::open(path)?;
        let mut data: Data = serde_json::from_reader(file)?;

        if data.analyzed_data.is_none() {
            data.analyzed_data = Some(AnalyzedData::new());
        } 

        for sentence in data.sentences.iter_mut() {
            data.analyzed_data.as_mut().unwrap().add_sentence(sentence)
        }

        Ok(data)
    }

    pub fn sentences(&self) -> &Vec<Sentence> {
        &self.sentences
    }

    pub fn add_sentence(&mut self, sentence: Sentence) {
        
        self.analyzed_data.as_mut().unwrap().add_sentence(&sentence);

        self.sentences.push(sentence);
    }

    pub fn save(&mut self) -> std::io::Result<()> {

        self.analyzed_data = None;

        // let data = serde_json::to_string_pretty(self)?;
        let data = serde_json::to_string(self)?;

        fs::write("data.json", data)?;

        Ok(())
    } 

    pub fn analyzed_data(&self, key: &[char]) -> Option<&Vec<u32>> {
        self.analyzed_data.as_ref().unwrap().combinations.get(key)
    }


}

impl Data {
    pub fn estimate_word_time_fast(&self, word: Vec<char>) -> u32 {

        // println!("starting on chunk {}", word.iter().collect::<String>());

        let max_num_chars = min(word.len(), MAX_COMBO_SIZE);

        let mut keysets = Vec::with_capacity(max_num_chars);
        
        for i in 1..=max_num_chars { //todo maybe start at 2, but change other loop too
            let keys: Vec<Vec<char>>;

            if i == 1{ 
                keys = word[0..].iter().map(|c| vec![*c]).collect();
            } else {
                keys = word.windows(i).map(|w| w.iter().cloned().collect()).collect(); //todo rewrite this
            }

            keysets.push(keys);
        }

        // dbg!(&keysets);

        for (j, keyset) in keysets.iter().enumerate().rev(){ //j is length of keyset
            for (i, key) in keyset.iter().enumerate(){ //i is the starting position of the keyset in the word

                // dbg!(key);
                // dbg!(&self.analyzed_data()[j].iter().next().unwrap());

                if let Some(delay) = self.analyzed_data(key) {

                    let mut last_delay = Self::recent_data(delay); //todo maybe average some amomunt of the last delays

                    // #[cfg(debug_assertions)]
                    // {
                    //     println!("availible delays {:?}", delay);
                    //     println!("computed chunk [{}] with delay {}", key.iter().collect::<String>(), last_delay);
                    // }

                    // println!("found key [{}] with delay {}", key.iter().collect::<String>(), last_delay);
                    

                    // dbg!(i);
                    // dbg!(&word);

                    if i > 0{

                        if i == 1 {
                            last_delay += self.estimate_word_time_fast(word[0..i + 1].to_vec());
                        } else {
                            last_delay += self.estimate_word_time_fast(word[0..i + 1].to_vec());
                        }
                        
                    }

                    //todo see if both of these should allow overlap to children

                    // dbg!(i + j + 1);
                    // dbg!(&word);

                    if i + j + 1< word.len(){
                        if i + j == 0{
                            last_delay += self.estimate_word_time_fast(word[i + j + 1..].to_vec());
                        } else {
                            last_delay += self.estimate_word_time_fast(word[i + j..].to_vec());
                        }

                        // last_delay += self.estimate_word_time_fast(word[i + j..].to_vec());
                    }


                    return last_delay;

                }
            }
        }

        panic!("was unable to detirmine delay of the word: {}", word.iter().collect::<String>());

    }

    pub fn estimate_word_time_fast_shallow(&self, word: Vec<char>) -> u32 {

        unreachable!();

        let max_num_chars = min(min(word.len(), MAX_COMBO_SIZE), 2);

        let mut keysets = Vec::with_capacity(max_num_chars);
        
        for i in 1..=max_num_chars { //todo maybe start at 2, but change other loop too
            let keys: Vec<Vec<char>>;

            if i == 1{ 
                keys = word[0..].iter().map(|c| vec![*c]).collect();
            } else {
                keys = word.windows(i).map(|w| w.iter().cloned().collect()).collect(); //todo rewrite this
            }

            keysets.push(keys);
        }

        // dbg!(&keysets);

        for (j, keyset) in keysets.iter().enumerate().rev(){ //j is length of keyset
            for (i, key) in keyset.iter().enumerate(){ //i is the starting position of the keyset in the word

                // dbg!(key);
                // dbg!(&self.analyzed_data()[j].iter().next().unwrap());

                if let Some(delay) = self.analyzed_data(key) {

                    let mut last_delay = Self::recent_data(delay); //todo maybe average some amomunt of the last delays



                    // #[cfg(debug_assertions)]
                    // {
                    //     println!("availible delays {:?}", delay);
                    //     println!("computed chunk [{}] with delay {}", key.iter().collect::<String>(), last_delay);
                    // }

                    // dbg!(i);
                    // dbg!(&word);

                    if i > 0{

                        if i == 1 {
                            last_delay += self.estimate_word_time_fast_shallow(word[1..i + 1].to_vec());
                        } else {
                            last_delay += self.estimate_word_time_fast_shallow(word[0..i + 1].to_vec());
                        }
                        
                    }

                    //todo see if both of these should allow overlap to children

                    // dbg!(i + j + 1);
                    // dbg!(&word);

                    if i + j + 1< word.len(){
                        if i + j == 0{
                            last_delay += self.estimate_word_time_fast_shallow(word[i + j + 1..].to_vec());
                        } else {
                            last_delay += self.estimate_word_time_fast_shallow(word[i + j..].to_vec());
                        }

                        // last_delay += self.estimate_word_time_fast(word[i + j..].to_vec());
                    }


                    return last_delay;

                }
            }
        }

        panic!("was unable to detirmine delay of the word: {}", word.iter().collect::<String>());

    }

    pub fn estimate_sum_letters(&self, word: Vec<char>) -> u32 {
        let mut sum = 0;
        for c in word {
            sum += Self::recent_data(self.analyzed_data(&[c]).expect("missing single letter delay"));
        }
        sum
    }

    pub fn estimate_word_time(&self, word: Vec<char>){
        todo!()
    }

    pub fn recent_data(delays: &[u32]) -> u32{
        delays.iter().cloned().reduce(|prev, cur| (prev + cur) / 2).unwrap()

    }
}


type Combinations = HashMap<Vec<char>, Vec<u32>>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzedData {
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    combinations: Combinations,
}

impl AnalyzedData {
    pub fn new() -> Self {
        AnalyzedData {
            combinations: HashMap::new(),
        }
    }

    pub fn add_sentence(&mut self, sentence: &Sentence) {

        let max_num_chars = min(sentence.letters.len(), MAX_COMBO_SIZE);

        // let mut combinations = Vec::with_capacity(max_num_chars);

        for i in 1..=max_num_chars {

            let cum_delays;
            if i <= 2{
                cum_delays = sentence.delays.clone();
            } else {
                cum_delays = sentence.delays.windows(i-1).map(|w| w.iter().sum::<u32>()).collect();
            }

            let keys: Vec<Vec<char>>;

            if i == 1{ 
                keys = sentence.letters[1..].iter().map(|c| vec![*c]).collect();
            } else {
                keys = sentence.letters.windows(i).map(|w| w.iter().cloned().collect()).collect(); //todo rewrite this
            }

            assert_eq!(keys.len(), cum_delays.len());

            for (key, delay) in keys.into_iter().zip(cum_delays.into_iter()) {
                self.combinations.entry(key).or_insert(vec![]).push(delay);
                // let entry = self.combinations[i-1].entry(key).or_insert(vec![]);

            }

        }
    }

}


pub struct WordList {
    pub words: Vec<String>,
    pub scored_words: Vec<(u32, String)>,
}

impl WordList {
    pub fn load() -> Self {
        let path = "words.json";
        let file = fs::File::open(path).unwrap();
        let mut words: Vec<String> = serde_json::from_reader(file).unwrap();

        WordList {
            words,
            scored_words: vec![],
        }
    }

    pub fn sort_words(&mut self, data: &Data) {

        let mut scored_words = Vec::with_capacity(self.words.len());

        for word in self.words.iter(){

            let mut buffered_word: Vec<char> = word.chars().collect();
            // buffered_word.push(' ');
            // buffered_word.insert(0, ' ');
            

            let full_estimate = data.estimate_word_time_fast(buffered_word.clone()) * 100;
            // let letters_estimate = data.estimate_word_time_fast(buffered_word);

            let score = full_estimate.checked_div(word.len() as u32 ).unwrap_or(0);
            scored_words.push((score, word.clone()));
            // println!("{} {}", word.chars().collect::<String>(), score)
        }

        scored_words.sort_by_key(|&(score, _)| std::cmp::Reverse(score));
        // scored_words.sort_by_key(|&(score, _)| score);

        self.scored_words = scored_words;

        #[cfg(debug_assertions)]
        {
            println!("Easiest Words: {:?}", &self.scored_words[self.scored_words.len()-5..]);
            println!("Hardest Words: {:?}", &self.scored_words[0..5])
        }

        // println!("{:?}", self.scored_words);
    }

    pub fn get_sentence(&self, num_words: usize) -> String {
        let mut sentence = String::new();
        for i in 0..num_words {
            
            sentence.push_str(&self.scored_words[i].1);
            sentence.push(' ');
        }
        sentence.pop();
        sentence
    }
}