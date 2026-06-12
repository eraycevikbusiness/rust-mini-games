pub struct Word {
    origin: String,
    curr_input: Vec<Option<char>>,
}

impl Word {
    pub fn new(value: &str) -> Self {
        let vec: Vec<Option<char>> = Vec::new();
        Self {
            origin: value.to_owned(),
            curr_input: vec![None; value.chars().count()],
        }
    }

    pub fn contains(&self, c: char) -> bool {
        self.origin.contains(c)
    }

    pub fn fill_empty_fields_with(&self, c: char) {}

    pub fn render(&self) {
        for c in self.curr_input.iter() {
            match c {
                Some(v) => {
                    print!(" {v} ");
                }
                None => {
                    print!(" _ ");
                }
            }
        }
        println!();
    }
}
