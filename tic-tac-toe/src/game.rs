use std::io::stdin;

use crate::board::Board;
pub enum Winner {
    Player,
    Computer,
}

pub struct Game {
    board: Board,
}

#[derive(Debug)]
enum GameError {
    InvalidUserInput(&'static str),
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
        }
    }
}

impl Game {
    pub fn run(&mut self) {
        loop {
            self.board.render();
            let (row, col) = match self.ask_user_for_turn() {
                Ok((row, col)) => (row, col),
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
            //User turn
            self.board.set(row, col, Cell::X);
        }
    }

    fn ask_user_for_turn(&self) -> Result<(usize, usize), GameError> {
        println!("Enter the position where you want to play (Row;Column)");
        print!(">");
        let mut i_buffer: String = String::new();
        let row: usize;
        let column: usize;
        stdin().read_line(&mut i_buffer).unwrap();

        let parts: Vec<&str> = i_buffer.trim().split(';').collect();

        if parts.len() > 2 {
            println!("Bitte das richtige Format (Row;Column) - Beispiel: 1;1");
            return Err(GameError::InvalidUserInput(
                "Invalid Format - please (Row;Column)",
            ));
        }

        row = parts[0].parse().unwrap();
        column = parts[1].parse().unwrap();

        if (row <= 2) && (column <= 2) {
            Ok((row, column))
        } else {
            Err(GameError::InvalidUserInput(
                "Numbers have to be in the field!",
            ))
        }
    }
}
