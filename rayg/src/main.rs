mod constants;
mod game;
mod menu;
mod network;
mod state;

use state::GameState;

fn main() {
    let mut game = GameState::new();

    game.run();
}
