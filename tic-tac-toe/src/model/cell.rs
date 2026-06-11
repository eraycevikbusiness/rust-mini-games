#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    pub fn symbol(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::O => 'O',
            Cell::X => 'X',
        }
    }
}
