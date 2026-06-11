use std::io::stdin;

use rand::RngExt;

use crate::{board::Board, model::cell::Cell};

const PLAYER_CELL_TYPE: Cell = Cell::X;
const COMPUTER_CELL_TYPE: Cell = Cell::O;

#[derive(PartialEq)]
pub enum Winner {
    Player,
    Computer,
}

pub struct Game {
    board: Board,
}

#[allow(dead_code)]
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

            if !self.board.is_empty_on(row, col) {
                println!("You can only play your turn on fields that are empty!");
                continue;
            }
            self.board.set(row, col, PLAYER_CELL_TYPE);

            if self.has_winner() == Some(Winner::Player) {
                println!("You won!!");
                break;
            }

            let (row, col) = self.ask_computer_for_turn();
            self.board.set(row, col, COMPUTER_CELL_TYPE);

            if self.has_winner() == Some(Winner::Computer) {
                println!("You loose! The computer is better!");
                break;
            }
        }

        self.board.render();
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

    fn ask_computer_for_turn(&self) -> (usize, usize) {
        let empty: Vec<(usize, usize)> = self.board.get_empty_cells();
        let rnd_cell_loc = empty[rand::rng().random_range(0..empty.len())];

        rnd_cell_loc
    }

    fn has_winner(&self) -> Option<Winner> {
        let lines = [
            //3 Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            //3 Column
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            //2 Diagonal
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for [p1, p2, p3] in lines {
            let a = self.board.get(p1.0, p1.1);
            let b = self.board.get(p2.0, p2.1);
            let c = self.board.get(p3.0, p3.1);

            if a != Cell::Empty && a == b && b == c {
                return match a {
                    PLAYER_CELL_TYPE => Some(Winner::Player),
                    Cell::O => Some(Winner::Computer),
                    _ => None,
                };
            }
        }

        None
    }
}
