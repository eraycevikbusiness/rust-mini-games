use std::{cmp::Ordering, io::stdin};

use rand::{RngExt, rng};

fn main() {
    let rnd_number = rng().random_range(1..=100);

    loop {
        let mut i_buffer = String::new();
        println!("Guess the number between 1 and 100: ");

        stdin().read_line(&mut i_buffer).unwrap();
        let as_i32: i32 = match i_buffer.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Please just numbers!");
                continue;
            }
        };

        match as_i32.cmp(&rnd_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("TO HIGH!"),
            Ordering::Equal => {
                println!("YOU'VE WON!");
                break;
            }
        }
    }
}
