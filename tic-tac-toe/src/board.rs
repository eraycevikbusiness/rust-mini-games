use crate::model::cell::Cell;

const RENDER_LINE: &str = "---+------+------+---";

pub struct Board {
    fields: [[Cell; 3]; 3],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            fields: [[Cell::Empty; 3]; 3],
        }
    }
}

impl Board {
    pub fn render(&self) {
        for row in self.fields.iter() {
            for (j, column) in row.iter().enumerate() {
                print!("{:?}", column);
                if j < 2 {
                    print!(" | ");
                }
            }
            println!();
            println!("{RENDER_LINE}");
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Cell {
        self.fields[row][column]
    }

    pub fn set(&mut self, row: usize, column: usize, to: Cell) {
        self.fields[row][column] = to;
    }
}
