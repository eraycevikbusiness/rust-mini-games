use crate::game::Game;

mod board;
mod game;
mod model;

fn main() {
    Game::default().run();
}
