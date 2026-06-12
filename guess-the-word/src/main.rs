use crate::game::Game;

mod game;
mod word;

fn main() {
    Game::new().run();
}
