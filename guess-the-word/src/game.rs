use std::{fs, io::stdin};

use rand::{RngExt, rng};

use crate::word::Word;

const CSV_PATH: &str = "words.csv";
const MAX_FAILUES: u8 = 6;

pub struct Game {
    data: Vec<String>,
    rnd_word: String,
    failures: u8,
    failures_words: Vec<char>,
}

impl Game {
    pub fn new() -> Self {
        fn load_data_from_csv() -> Vec<String> {
            fs::read_to_string(CSV_PATH)
                .unwrap()
                .split(',')
                .map(|s| s.to_string())
                .collect()
        }

        Self {
            data: load_data_from_csv(),
            rnd_word: String::new(),
            failures: 0,
            failures_words: vec![],
        }
    }

    fn get_rnd_word(&self) -> String {
        self.data[rng().random_range(0..self.data.len())]
            .to_ascii_lowercase()
            .clone()
    }

    pub fn run(&mut self) {
        self.rnd_word = self.get_rnd_word();
        let mut word = Word::new(&self.rnd_word);
        loop {
            print!("\x1B[2J\x1B[1;1H");
            word.render();

            let c = self.ask_for_char();

            if word.contains(c) {
                word.fill_empty_fields_with(c);
            } else {
                self.failures += 1;
                self.failures_words.push(c);
            }

            if self.is_game_over() {
                println!("You lost the game! The word was '{}'", self.rnd_word);
                break;
            }

            if word.is_complete() {
                println!("You won!");
                break;
            }
        }

        self.display_failures();
    }

    fn ask_for_char(&self) -> char {
        let mut i_buffer: String = String::new();
        self.display_failures();
        println!("Please enter a character: ");
        stdin().read_line(&mut i_buffer);
        let as_c: char = i_buffer.trim().parse().unwrap();

        as_c.to_ascii_lowercase()
    }

    fn is_game_over(&self) -> bool {
        self.failures >= MAX_FAILUES
    }

    fn display_failures(&self) {
        println!(
            "[FAILURES: {} / {} | {:?}]",
            self.failures, MAX_FAILUES, self.failures_words
        );
    }
}
