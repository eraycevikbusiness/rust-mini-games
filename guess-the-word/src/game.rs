use std::{fs, io::stdin};

use rand::{RngExt, rng};

use crate::word::Word;

const CSV_PATH: &str = "words.csv";
const MAX_FAILUES: u8 = 6;

enum GameError {
    InvalidInput(&'static str),
}

pub struct Game {
    data: Vec<String>,
    word: Word,
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

        fn get_rnd_word(from: &Vec<String>) -> String {
            from[rng().random_range(0..from.len())]
                .to_ascii_lowercase()
                .clone()
        }

        let data = load_data_from_csv();

        Self {
            word: Word::new(get_rnd_word(&data).as_str()),
            data: data,
            failures: 0,
            failures_words: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("\x1B[2J\x1B[1;1H");
            self.word.render();

            let c = match self.ask_for_char() {
                Ok(v) => v,
                Err(GameError::InvalidInput(msg)) => {
                    println!("{msg}");
                    println!("Press enter to continue...");
                    let mut t_buffer = String::new();
                    stdin().read_line(&mut t_buffer).ok();
                    continue;
                }
            };

            if self.word.contains(c) {
                self.word.fill_empty_fields_with(c);
            } else {
                self.failures += 1;
                self.failures_words.push(c);
            }

            if self.is_game_over() {
                println!("You lost the game! The word was '{}'", self.word.origin);
                break;
            }

            if self.word.is_complete() {
                println!("You won!");
                break;
            }
        }

        self.display_failures();
    }

    fn ask_for_char(&self) -> Result<char, GameError> {
        let mut i_buffer: String = String::new();
        self.display_failures();
        println!("Please enter a character: ");
        stdin().read_line(&mut i_buffer).ok();
        let as_c: char = i_buffer
            .trim()
            .parse()
            .map_err(|_| GameError::InvalidInput("Only letters!"))?;

        if as_c.is_ascii_digit() {
            return Err(GameError::InvalidInput("Only letters!"));
        }
        Ok(as_c.to_ascii_lowercase())
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
