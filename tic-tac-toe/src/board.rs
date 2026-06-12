use crate::model::cell::Cell;

const RENDER_LINE: &str = "---+------+------+---";

pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell::Empty; 3]; 3],
        }
    }
}

impl Board {
    pub fn render(&self) {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                print!(" {} ", cell.symbol());
                if j < 2 {
                    print!("|");
                }
            }
            println!();
            if i < 2 {
                println!("---+---+---");
            }
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Cell {
        self.cells[row][column]
    }

    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut v: Vec<(usize, usize)> = Vec::new();
        for row in 0..=2 {
            for col in 0..=2 {
                if self.cells[row][col] == Cell::Empty {
                    v.push((row, col));
                }
            }
        }

        v
    }

    pub fn has_empty_cells(&self) -> bool {
        self.cells.iter().flatten().any(|c| *c == Cell::Empty)
    }

    pub fn set(&mut self, row: usize, column: usize, to: Cell) {
        self.cells[row][column] = to;
    }

    pub fn is_empty_on(&self, row: usize, column: usize) -> bool {
        self.cells[row][column] == Cell::Empty
    }
}
