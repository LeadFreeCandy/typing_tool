use std::fs;

const SENTENCE: &'static str = "the quick brown fox jumps over the lazy dog";

mod tui;
use tui::*;

mod data;
use data::*;



fn main() {



    // let mut data = Data::new();

    // let s1 = tui::test_sentence(SENTENCE.to_string()).unwrap();
    // data.add_sentence(s1);
    let mut words = WordList::load();
    tui::setup().unwrap();


    //,{"letters":["b","o","s","s"],"delays":[91,160,143]}

    let mut data = Data::load().unwrap_or_else(|_| {Data::new()});
    if data.sentences().is_empty(){
        let s1 = tui::test_sentence(SENTENCE.to_string()).unwrap();
        data.add_sentence(s1);
    }

    //"czech"), (30366, "broadcast"), (29560, "brook"), (28660, "broke"), (28460, "broad"
    
    let word = "attorney";

    let word_delay = data.estimate_word_time_fast(word.chars().collect());
    // let word_delay_sum = data.estimate_word_time_fast_shallow(word.chars().collect());

    println!("delay: {}", word_delay);

    for i in 0..1 {
        words.sort_words(&data);
        let s1 = tui::test_sentence(words.get_sentence(17)).unwrap();
        data.add_sentence(s1);
    }
    



    
    
    // data.add_sentence(s2);

    // let word_delay = data.estimate_word_time_fast("abczabc".chars().collect()); //4795

    // println!("{:?}", word_delay);


    data.save().unwrap();

}