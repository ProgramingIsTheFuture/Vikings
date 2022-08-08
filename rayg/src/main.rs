mod constants;
mod game;
mod loading;
mod menu;
mod network;
mod state;

use state::GameState;

fn main() {
    let mut game = GameState::new(true);

    game.run();
}
