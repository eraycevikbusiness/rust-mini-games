use crate::game::Game;

mod board;
mod game;
mod model;
mod rules;

fn main() {
    Game::default().run();
}
