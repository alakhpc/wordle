use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashSet;

static ANSWERS: &str = include_str!("../data/answers.txt");
static WORDS: &str = include_str!("../data/words.txt");

pub fn get_random_answer() -> String {
    let words = ANSWERS.lines().collect::<Vec<_>>();
    let word = words.choose(&mut thread_rng()).expect("No words");

    word.to_string()
}

pub fn get_allowed_words() -> HashSet<String> {
    let answers = ANSWERS.lines();
    let words = WORDS.lines();

    answers.chain(words).map(|l| l.to_string()).collect()
}
