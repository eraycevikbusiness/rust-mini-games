pub struct Word {
    pub origin: String,
    curr_input: Vec<Option<char>>,
}

impl Word {
    pub fn new(value: &str) -> Self {
        Self {
            origin: value.to_owned(),
            curr_input: vec![None; value.chars().count()],
        }
    }

    pub fn contains(&self, c: char) -> bool {
        self.origin.contains(c)
    }

    pub fn fill_empty_fields_with(&mut self, c: char) {
        for (i, curr_c) in self.origin.chars().enumerate() {
            if curr_c == c {
                self.curr_input[i] = Some(c);
            }
        }
    }

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

    pub fn is_complete(&self) -> bool {
        !self.curr_input.contains(&None)
    }
}
